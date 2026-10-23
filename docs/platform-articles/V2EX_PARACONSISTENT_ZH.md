# 四值逻辑状态机：让系统在矛盾中不 panic 的工程实践

**链上 meet 代数（FALSE < NEITHER < BOTH < TRUE）、零分配 C 实现、no_std Rust 移植，以及 NEAR 链上可验证的概念证明**

*作者：ZXV 工程团队（ZEDEC / 36N9 Genetics, LLC）*
*为 V2EX 撰写 — 面向低层架构、语言设计与基础设施讨论的极客社区*

---

## TL;DR

- 分布式系统最根本的问题：**两个信息源对"真相"产生冲突时，系统怎么办？**
- 经典二值逻辑遇到矛盾会触发**爆炸原理**——系统要么 panic、要么丢数据、要么死锁。
- 解决方案：**四值逻辑**（TRUE/FALSE/NEITHER/BOTH），把矛盾当作**信息状态**而非错误。
- 关键设计：**链上 meet 代数**。把状态排成全序 `FALSE < NEITHER < BOTH < TRUE`，由于 TRUE 在顶端，"非 TRUE 组合永远无法产生 TRUE"这个不变式**由结构保证**，不是运行时检查。
- 这套逻辑**已经跑在 NEAR 主网上**（110 kits、136 capabilities、canonical `zedec.near`、合约 `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`），gas 免费可验证。

---

## 1. 问题：分布式系统的"真相冲突"

任何分布式系统最终都要回答一个问题：

> **两个节点对同一个事实给出矛盾的答案，系统该信谁？**

经典逻辑假设排中律（$P \lor \neg P$）。但网络分区、复制延迟会让 **$P$ 和 $\neg P$ 的证据同时到达**。在经典框架里，这触发**爆炸原理**（principle of explosion）：从矛盾中可以推出任何命题，整个推理系统失去意义。

于是所有经典实现只能从四个"回避策略"里选：

| 策略 | 行为 | 代价 |
|---|---|---|
| **Panic** | 抛异常、回滚 | 执行中断、状态丢失 |
| **Last-writer-wins** | 新的时间戳覆盖 | 旧真相被静默抹除 |
| **Deadlock** | 停止处理等待 | 整个系统冻结 |
| **多数决** | 票多者胜 | 赢的是权力不是真相 |

四种策略都把**矛盾当 bug**。没有一种保留矛盾中蕴含的信息。

## 2. 解法：四值逻辑

**矛盾不是错误状态，是信息状态。** 系统同时持有两个主张、记录冲突、在证据真正落定前不强制解决。

### 2.1 四个状态（位模式）

```c
typedef enum {
    NEITHER = 0x00,  /* 无证据 */
    TRUE    = 0x01,  /* 合意·可结算 */
    FALSE   = 0x02,  /* 明确驳回 */
    BOTH    = 0x03,  /* 矛盾·保留 */
} lpres_state_t;
```

### 2.2 关键：链上 rank（全序）

核心不是 Belnap 的格（lattice），而是**一条全序链上的 rank**：

```
FALSE(0) < NEITHER(1) < BOTH(2) < TRUE(3)
```

**FALSE 是底，TRUE 是顶。** 这是整个结构的精髓。

### 2.3 meet = 取 rank 较小者

```c
lpres_state_t lpres_conjoin(lpres_state_t a, lpres_state_t b) {
    /* the meet: min on the chain. FALSE dominates, TRUE is identity,
     * NEITHER absorbs BOTH — all by the rank.
     * Deliberate deviation from Belnap's FOUR (the chain, not the lattice):
     * no mix of non-TRUE values can yield TRUE. */
    if ((unsigned)a >= LPRES_STATE_COUNT ||
        (unsigned)b >= LPRES_STATE_COUNT)
        return LPRES_STATE_NEITHER;   /* fail-closed */
    return lpres_rank[a] <= lpres_rank[b] ? a : b;
}
```

**为什么这是结构性的而非检查性的**：TRUE 的 rank 是 3（最大）。meet 返回最小值，所以只有当**两个输入都是 TRUE** 时结果才是 TRUE。**"非 TRUE 组合永不产生 TRUE"是 rank 结构的定理。**

| 输入 | 结果 | 含义 |
|---|---|---|
| `conjoin(FALSE, x)` | `FALSE` | FALSE 支配（底） |
| `conjoin(TRUE, TRUE)` | `TRUE` | 只有合意可结算 |
| `conjoin(BOTH, TRUE)` | `BOTH` | 矛盾被保留、绝不强制 |
| `conjoin(NEITHER, BOTH)` | `NEITHER` | NEITHER 吸收 BOTH |
| `conjoin(未知, x)` | `NEITHER` | 越界保护（fail-closed） |

### 2.4 代数性质（已验证）

meet/join 构成**有界格**：

```
meet: 幂等 ○ 交换 ○ 结合 ○
join: 幂等 ○ 交换 ○ 结合 ○
吸收: ○ (a ∧ (a ∨ b) = a)
偏序: ○ (a ≤ b ⇔ meet(a,b) = a)
不变式: ○ (非 TRUE 的 meet 永不产生 TRUE)
```

**"only TRUE settles"（唯有真可结算）从 rank 结构数学推导而来。**

---

## 3. C 实现：零分配

```c
/* belnap_gate.c — 零分配、矛盾容忍状态机 */
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

typedef enum { NEITHER = 0x00, TRUE = 0x01, FALSE = 0x02, BOTH = 0x03 } belnap_state_t;

/* enum 顺序 [NEITHER,TRUE,FALSE,BOTH] -> {1,3,0,2} */
static const uint8_t rank[4] = { 1, 3, 0, 2 };

static belnap_state_t conjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER;
    return rank[a] <= rank[b] ? a : b;
}

static belnap_state_t disjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER;
    return rank[a] >= rank[b] ? a : b;
}

static bool is_settleable(belnap_state_t s) { return s == TRUE; }

int main(void) {
    /* 矛盾产生：经典逻辑里这是 panic 点 */
    belnap_state_t c = (belnap_state_t)((unsigned)TRUE | (unsigned)FALSE);
    printf("State: 0x%02X (%s)\n", c, c == BOTH ? "BOTH" : "OTHER");
    printf("Can Settle? %s\n", is_settleable(c) ? "YES" : "NO (Blocked without panic)");
    /* 合流：TRUE + TRUE = TRUE */
    belnap_state_t r = conjoin(TRUE, TRUE);
    printf("Resolved: 0x%02X -> Can Settle? %s\n", r, is_settleable(r) ? "YES" : "NO");
    /* 越界保护：7 永远不会变成 TRUE */
    belnap_state_t u = conjoin((belnap_state_t)7, TRUE);
    printf("Unknown-guard: 0x%02X -> Can Settle? %s\n", u, is_settleable(u) ? "YES" : "NO");
    return 0;
}
```

```bash
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate && ./belnap_gate
```

输出：
```
State: 0x03 (BOTH)
Can Settle? NO (Blocked without panic)
Resolved: 0x01 -> Can Settle? YES
Unknown-guard: 0x00 -> Can Settle? NO
```

**内存特性**：栈 3 个局部变量，堆 0，静态数据 4 字节，汇编级几十条指令。`-O3` 下 meet 编译成 `movzx`×2 + `cmp` + `cmovle`——对分支预测友好。

---

## 4. Rust / no_std 实现

```rust
//! lib.rs — no_std 下的不变式验证
#![no_std]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Belnap {
    Neither = 0x00,
    True    = 0x01,
    False   = 0x02,
    Both    = 0x03,
}

const RANK: [u8; 4] = [1, 3, 0, 2];

impl Belnap {
    #[inline(always)]
    pub fn is_settleable(self) -> bool { matches!(self, Belnap::True) }

    #[inline(always)]
    pub fn merge(self, other: Self) -> Self {
        if RANK[self as usize] <= RANK[other as usize] { self } else { other }
    }

    #[inline(always)]
    pub fn combine(self, other: Self) -> Self {
        if RANK[self as usize] >= RANK[other as usize] { self } else { other }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contradiction_does_not_panic() {
        assert!(!Belnap::Both.is_settleable());
    }

    #[test]
    fn only_true_settles() {
        assert!(Belnap::True.is_settleable());
        assert!(!Belnap::False.is_settleable());
        assert!(!Belnap::Neither.is_settleable());
    }

    #[test]
    fn false_dominates_meet() {
        assert_eq!(Belnap::False.merge(Belnap::True), Belnap::False);
    }

    #[test]
    fn both_is_never_coerced() {
        assert_eq!(Belnap::Both.merge(Belnap::True), Belnap::Both);
        assert!(!Belnap::Both.merge(Belnap::True).is_settleable());
    }

    #[test]
    fn meet_associative() {
        let vals = [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both];
        for a in vals { for b in vals { for c in vals {
            assert_eq!(a.merge(b).merge(c), a.merge(b.merge(c)));
        }}}
    }

    #[test]
    fn no_nontrue_meet_yields_true() {
        let vals = [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both];
        for a in vals { for b in vals {
            if a != Belnap::True && b != Belnap::True {
                assert_ne!(a.merge(b), Belnap::True);
            }
        }}
    }
}
```

```bash
cargo test --release   # 6 个测试全绿
```

WASM 目标：
```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```
几 KB 的 WASM，不调 heap，纯函数导出。

---

## 5. 状态机设计

```
       无证据                    双方证据
          │                         │
          ▼                         ▼
      NEITHER ───────────────► BOTH ──────────► (争议解决)
      (等待)                   (保留·记录)          │
          │                                       │ 解决
          │ 单一证据                              ▼
          ▼                                   TRUE / FALSE
     (仅 TRUE 推进)                      (结算 / 拒绝)
```

**设计要点**：
1. NEITHER 是"等待"不是"挂起"——不阻塞但也不推进。
2. BOTH 进入"争议解决"——不自动解决，但写入审计轨迹。
3. **只有 TRUE 允许副作用**（状态变更、资金移动）。`is_settleable(s)` 为 false 时**不发生任何写入**——结构性防止部分更新不一致，也把 TOCTOU 检查本身并入不变式。

### Event sourcing 集成

```
[Event 1] attest(TRUE)   → TRUE
[Event 2] attest(FALSE)  → BOTH（矛盾被记录）
[Event 3] resolve(TRUE)  → TRUE（解决）
```
Append-only 日志，矛盾到解决的全审计轨迹。

---

## 6. 验证

### KAT（已知答案测试）
| 测试 | 输入 | 期望 |
|---|---|---|
| KAT-01 | `conjoin(TRUE, FALSE)` | `FALSE` |
| KAT-02 | `conjoin(BOTH, TRUE)` | `BOTH` |
| KAT-03 | `conjoin(NEITHER, BOTH)` | `NEITHER` |
| KAT-04 | `conjoin(TRUE, TRUE)` | `TRUE` |
| KAT-05 | `conjoin(0xFF, TRUE)` | `NEITHER` (fail-closed) |
| KAT-06 | `disjoin(BOTH, NEITHER)` | `BOTH` |

### Fuzzing（65,536 输入）
```c
for (unsigned a = 0; a < 256; a++)
    for (unsigned b = 0; b < 256; b++) {
        belnap_state_t m = conjoin((belnap_state_t)a, (belnap_state_t)b);
        if (m == TRUE && (a != TRUE || b != TRUE)) {
            printf("VIOLATION\n"); return 1;
        }
    }
printf("FUZZ OK: 65536 combinations\n");
```

---

## 7. NEAR 链上部署（概念证明）

这不是纸上设计。**NEAR 主网上跑着智能合约。**

### 地址
- **Canonical（路由）**：`zedec.near` — 调用 `route()` 得到完整拓扑
- **协议（110 kits）**：`f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`
- **注册表（136 capabilities）**：`7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a`
- **Gate（内核不变式）**：`6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8`

### 链上验证（免费，view call）
```bash
# conjoin {"inputs":[1,2]}        → "FALSE"
# is_settleable {"state":"BOTH"}  → false（矛盾不结算）
# is_settleable {"state":"TRUE"}  → true（合意结算）
# verify_chain {"from":0}         → true（审计哈希链完好）
```

### 逻辑如何落地
- **威尼斯加油站**：TRUE 结算获得协议 2× 燃料匹配；BOTH 失去匹配（排水盾）——矛盾有成本，无法 spam。
- **主权法令**："无分叉之分叉"——矛盾法令共存，遵守者可自由切换。
- **Receipt 链**：每次结算链接进哈希链（verify_chain → true），即链上 event sourcing。
- **状态兼容升级**：顶层结构体永不改变，新功能状态存入保留命名空间（Borsh 前向兼容）——110 kits 就是这么攒出来的。

### 质量记录（实测）
| 验证 | 结果 |
|---|---|
| QC harness（对内核 C 源码） | 55/55，零 bug |
| 链上全审计 | 24/24 |
| 耐久性压力 | 70/70 |
| 注册审计 | 136/136 |
| 燃气墙（读/写） | 125/125 / 10/10 |

---

## 8. 结论

四值逻辑解决了一个经典系统从未解决的问题：**数据冲突时怎么办？** 答案是：把矛盾当信息保存，不强制、不 panic。而"唯有真可结算"这条不变式，是 rank 结构的**数学推论**，不是运行时检查。

系统在 NEAR 上运行，验证免费。**系统开放。**

---

## 仓库
- **内核实现**（`kernel/src/lpres/`）：https://github.com/sykomiked/zxv-stack
- **智能合约**（NEAR，canonical `zedec.near`）：https://github.com/sykomiked/ZXV-NEP-paraconsistent

---

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC · SPDX-License-Identifier: Apache-2.0*