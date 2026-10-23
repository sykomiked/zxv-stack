# Tham số kỹ thuật ZXV: kernel và kiến trúc

**Tiếng Việt** · ZXV — logic paraconsistent · máy trạng thái · giải quyết


Tài liệu này mô tả các tham số kỹ thuật: lõi bốn giá trị, kỷ luật nâng cấp tương thích, sổ kiểm toán chuỗi, kinh tế gas và hồ sơ chất lượng.

---

## Lõi kernel
Mô-đun lpres định nghĩa bốn trạng thái là mẫu bit trong một byte. conjoin và disjoin là so sánh số nguyên thuần túy trên bảng bậc — không cấp phát, vài lệnh.

---

## Nâng cấp tương thích
Cấu trúc tuần tự hóa cấp cao không bao giờ đổi; tính năng mới sống trong không gian tên dành riêng của bản đồ lưu trữ. Tính tương thích Borsh này cho phép 110 kit tích lũy mà không phá trạng thái.

---

## Chuỗi kiểm toán
Mỗi giải quyết liên kết vào chuỗi hash (biên lai). verify_chain xác nhận toàn vẹn — mẫu event sourcing on-chain, vết không đổi từ mâu thuẫn đến giải quyết.

---

## Kinh tế gas
Giải quyết TRUE nhận khớp gas 2x (Trạm Gas Venice); BOTH mất khớp (khiên thoát). Mâu thuẫn đắt, nên không thể spam. Đọc hoàn toàn không gas.

---

## Hồ sơ chất lượng
QC harness khớp mã nguồn C kernel (55/55); kiểm toán on-chain phủ mọi lớp (24/24); áp lực bền 70/70; kiểm toán đăng ký 136/136; tường gas 125/125 đọc và 10/10 chu kỳ.

---

## Kết luận
ZXV chứng minh logic paraconsistent là kỹ thuật: được triển khai, kiểm toán, xác minh trên NEAR.

Hệ thống mở.

**Chỉ TRUE mới giải quyết. TRUE+FALSE=BOTH — mâu thuẫn được ghi lại, không bao giờ bị ép buộc.**

