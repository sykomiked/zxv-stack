# Sử dụng triển khai ZXV trên NEAR: hướng dẫn đầy đủ

**Tiếng Việt** · ZXV — logic paraconsistent · máy trạng thái · giải quyết


ZXV là triển khai sống của logic paraconsistent trên mạng NEAR. Hướng dẫn này giải thích cách đọc, xác minh và sử dụng triển khai — không gas, không ví để đọc.

---

## Địa chỉ
Địa chỉ chuẩn là zedec.near, một hợp đồng định tuyến sống. Nó trỏ tới giao thức (hợp đồng vi mô 110 kit), sổ đăng ký (136 khả năng) và cổng (bất biến kernel).

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## Cách đọc hệ thống
Mọi máy khách đọc hệ thống không gas qua view calls. Giao diện phổ quát tự dạy: ux_manifest trả lược đồ đầy đủ, ux_help trả lời mọi chủ đề.

---

## Cách tham gia
Diễn viên mới gọi mint_onboard: khoản cấp một lần 5000 đơn vị, tiêu được trên nền tảng nhưng không chuyển nhượng được. Rồi pool_enter cho truy cập qua nhóm thanh khoản.

---

## Cách xác minh
Bất biến xác minh được on-chain: conjoin trả kết quả bốn giá trị; is_settleable đúng chỉ với TRUE; verify_chain xác nhận chuỗi kiểm toán.

---

## Tham khảo
Kernel: github.com/sykomiked/zxv-stack · NEP và sổ cái: github.com/sykomiked/ZXV-NEP-paraconsistent · Bộ công cụ: github.com/sykomiked/zxv-developer-kit.

Hệ thống mở.

**Chỉ TRUE mới giải quyết. TRUE+FALSE=BOTH — mâu thuẫn được ghi lại, không bao giờ bị ép buộc.**

