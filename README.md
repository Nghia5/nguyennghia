Title
Stellar Habit-Builder (Commitment Contract)

Description
Stellar Habit-Builder là một Ứng dụng phi tập trung (DApp) hoạt động trên mạng lưới Stellar (Soroban), giúp người dùng xây dựng và duy trì tính kỷ luật thông qua sức mạnh của Hợp đồng thông minh (Smart Contract).

Mục đích: Dự án cho phép người dùng "khóa" một số lượng token (ví dụ: XLM, USDC) vào hợp đồng để làm tiền thế chân (cam kết) cho một mục tiêu cá nhân trong một khoảng thời gian nhất định. Nếu người dùng hoàn thành mục tiêu, hợp đồng sẽ hoàn trả lại số tiền. Nếu bỏ cuộc hoặc thất bại, số tiền này sẽ bị tịch thu và chuyển thẳng vào một quỹ từ thiện cộng đồng.

Tại sao làm idea này: Ý tưởng ra đời từ những trăn trở rất thực tế trong cuộc sống. Rất nhiều người đặt ra những mục tiêu tốt đẹp như đi tập gym đều đặn để cải thiện vóc dáng, tăng cân, hay quyết tâm cày cuốc học tập mỗi ngày để nâng cao điểm số GPA, nhưng lại rất dễ nản lòng và bỏ cuộc giữa chừng. Bằng cách gắn một "trách nhiệm tài chính" không thể gian lận trên blockchain vào các mục tiêu này, dự án tạo ra một động lực (và áp lực) mạnh mẽ, buộc người dùng phải bước ra khỏi vùng an toàn và kỷ luật với chính mình.

Tính năng
Hệ thống Smart Contract hiện tại bao gồm 3 tính năng cốt lõi:

Khởi tạo (Initialize): Thiết lập môi trường ban đầu cho hợp đồng, chỉ định rõ địa chỉ ví của Trọng tài (Oracle - người/hệ thống đánh giá kết quả) và địa chỉ ví của Quỹ từ thiện (Charity).

Cam kết (Commit): Cho phép người dùng kết nối ví, chỉ định loại token, số lượng muốn khóa và thực hiện ký xác nhận (Require Auth) để chuyển token từ ví cá nhân vào khóa an toàn bên trong Smart Contract. Hệ thống đảm bảo mỗi người dùng chỉ chạy một cam kết tại một thời điểm.

Phân xử (Resolve): Trọng tài (Oracle) gọi hàm này để chốt kết quả.

Nếu is_success = true (Hoàn thành mục tiêu): Token được mở khóa và chuyển trả về ví người dùng.

Nếu is_success = false (Thất bại): Token tự động bị chuyển sang ví Quỹ từ thiện. Dữ liệu cam kết sau đó được xóa bỏ để người dùng có thể bắt đầu một thử thách mới.

Contract
Mạng lưới: Stellar Testnet

Contract ID: CAMPXT63WY3DN2WTKJQY4G4FKTBW73CPJFE7B3L7WJV5DYWH3HDCUJLN

Link Stellar Expert: Xem lịch sử giao dịch của Contract tại đây

<img width="1919" height="970" alt="image" src="https://github.com/user-attachments/assets/a20c298d-3202-4543-ada1-4860f5e2c136" />

Future scopes
Tương lai của dự án sẽ được mở rộng từ một Smart Contract đơn thuần thành một hệ sinh thái hoàn chỉnh:

Giai đoạn 1 (Giao diện người dùng): Xây dựng một ứng dụng Mobile (sử dụng Flutter) hoặc Web App (Node.js/Spring Boot backend) để người dùng có thể tương tác với contract bằng giao diện trực quan thay vì dùng dòng lệnh Terminal.

Giai đoạn 2 (AI Oracle - Trọng tài Trí tuệ nhân tạo): Loại bỏ yếu tố con người trong việc phân xử. Tích hợp các mô hình Machine Learning và Computer Vision (như OpenCV, MediaPipe) để tự động phân tích hình ảnh/video của người dùng gửi lên. Ví dụ: AI tự động đếm số lần hít đất, kiểm tra form tập, hoặc theo dõi thời gian ngồi học trước màn hình để tự động gọi hàm resolve trên blockchain.

Giai đoạn 3 (SocialFi): Phát triển tính năng "Thách đấu nhóm", nơi một nhóm bạn bè cùng góp tiền vào chung một Pool (Hồ bơi thanh khoản). Ai bỏ cuộc giữa chừng sẽ mất tiền cọc, và số tiền đó sẽ được chia đều cho những người kiên trì đến cuối cùng.

Profile
Họ và tên: Nguyễn Trung Nghĩa (Nghĩa)

Vị trí: Sinh viên năm 3 chuyên ngành Công nghệ thông tin

GitHub: Nghia5

Kỹ năng & Công nghệ: * Blockchain: Stellar Soroban (Rust), Smart Contract Development.

Backend: Java (Spring Boot), Node.js.

Mobile/Frontend: Flutter.

Database: MongoDB, SQL Server.

AI/Data: Python, Computer Vision (OpenCV, MediaPipe), Machine Learning algorithms.
