#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env
};

// Khai báo các khóa (key) để lưu trữ dữ liệu vào blockchain
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Commitment(Address), // Lưu cam kết theo địa chỉ ví của user
    Oracle,              // Địa chỉ của trọng tài (Backend App)
    Charity,             // Địa chỉ ví nhận tiền phạt
}

// Cấu trúc dữ liệu của một cam kết
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HabitInfo {
    pub token: Address,     // Loại token bị khóa (VD: USDC contract address)
    pub amount: i128,       // Số lượng bị khóa
    pub is_resolved: bool,  // Trạng thái đã giải quyết chưa
}

#[contract]
pub struct HabitContract;

#[contractimpl]
impl HabitContract {
    // 1. Khởi tạo contract với địa chỉ Trọng tài và Quỹ từ thiện
    pub fn initialize(env: Env, oracle: Address, charity: Address) {
        env.storage().instance().set(&DataKey::Oracle, &oracle);
        env.storage().instance().set(&DataKey::Charity, &charity);
    }

    // 2. Người dùng tạo cam kết và khóa Token
    pub fn commit(env: Env, user: Address, token: Address, amount: i128) {
        // Yêu cầu user phải ký xác nhận giao dịch này
        user.require_auth();

        // Kiểm tra xem user có cam kết nào đang dang dở không
        if env.storage().persistent().has(&DataKey::Commitment(user.clone())) {
            panic!("Bạn đang có một cam kết chưa hoàn thành!");
        }

        // Chuyển token từ ví user vào địa chỉ của Smart Contract
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&user, &env.current_contract_address(), &amount);

        // Lưu thông tin cam kết vào storage của blockchain
        let habit = HabitInfo {
            token,
            amount,
            is_resolved: false,
        };
        env.storage().persistent().set(&DataKey::Commitment(user), &habit);
    }

    // 3. Trọng tài phân xử kết quả (Thành công hoặc Thất bại)
    pub fn resolve(env: Env, oracle: Address, user: Address, is_success: bool) {
        // Yêu cầu Oracle phải ký giao dịch
        oracle.require_auth();

        // Kiểm tra đúng Oracle được cấp quyền hay không
        let stored_oracle: Address = env.storage().instance().get(&DataKey::Oracle).unwrap();
        if oracle != stored_oracle {
            panic!("Chỉ Oracle mới được phép xác nhận kết quả!");
        }

        // Lấy thông tin cam kết của user ra
        let mut habit: HabitInfo = env.storage().persistent().get(&DataKey::Commitment(user.clone())).unwrap();
        if habit.is_resolved {
            panic!("Cam kết này đã được phân xử!");
        }

        habit.is_resolved = true;
        let token_client = token::Client::new(&env, &habit.token);
        let contract_addr = env.current_contract_address();

        if is_success {
            // HOÀN THÀNH: Trả lại token cho user
            token_client.transfer(&contract_addr, &user, &habit.amount);
        } else {
            // THẤT BẠI: Chuyển token vào quỹ từ thiện
            let charity: Address = env.storage().instance().get(&DataKey::Charity).unwrap();
            token_client.transfer(&contract_addr, &charity, &habit.amount);
        }

        // Xóa dữ liệu cam kết để user có thể tạo mục tiêu mới
        env.storage().persistent().remove(&DataKey::Commitment(user));
    }
}