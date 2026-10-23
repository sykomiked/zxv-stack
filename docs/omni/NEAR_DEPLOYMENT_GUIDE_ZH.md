# 使用 ZXV 的 NEAR 部署：完整指南

**简体中文** · ZXV — 矛盾容忍逻辑 · 状态机 · 结算


ZXV 是矛盾容忍逻辑在 NEAR 网络上的活实现。本指南说明如何读取、验证、使用这一部署——无 gas、任何客户端、读取无需钱包。

---

## 地址
规范地址是 zedec.near，一个活的路由合约。它指向协议（110 套件的微合约）、注册表（136 项能力）与门（内核不变式）。

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## 如何读取系统
任何客户端都可以用 view 调用无 gas 地读取系统。通用接口自解释：ux_manifest 返回完整模式，ux_help 回答任何主题，注册表的 vm_capabilities 列出全部能力。

---

## 如何加入
新参与者调用 mint_onboard 获得一次性 5000 单位赠款——平台内可花、不可转移，每账户一次，防止钱包刷量。然后 pool_enter 通过流动性池获得入口。

---

## 如何验证
不变式可在链上验证：conjoin 返回四值结果；is_settleable 仅对 TRUE 返回真；verify_chain 确认审计哈希链。每个主张都可独立、无 gas 地核验。

---

## 参考
内核实现见 github.com/sykomiked/zxv-stack；NEP 与台账见 github.com/sykomiked/ZXV-NEP-paraconsistent；开发者套件见 github.com/sykomiked/zxv-developer-kit。

系统开放。

**唯有真可结算。真＋伪＝真伪两值——矛盾被记录，绝不强制，绝不恐慌。**

**唯有真可结算。真＋伪＝真伪两值（TRUE+FALSE=BOTH）——矛盾被记录，绝不强制，绝不恐慌。**

