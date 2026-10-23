# 四值逻辑状态机的完整实现：从链上 meet 代数到 NEAR 链上验证

**矛盾容忍系统（Paraconsistent）工程实践全解析——零分配 C 实现、no_std Rust 移植、形式化验证与链上部署**

*作者：ZXV 工程团队（ZEDEC / 36N9 Genetics, LLC）*
*为博客园（CNBlogs）撰写 — 面向系统程序员、编译器研究者与后端工程师的深度技术文章*

---

## 摘要

每一个分布式或并发状态机，最终都会撞上同一堵墙：**当两个信息源对"真相"产生冲突时，系统应当如何处理？** 经典二值逻辑（排中律 $P \lor \neg P$）在矛盾证据同时到达时触发**爆炸原理**（principle of explosion），迫使实现者在四种失败模式中选择：恐慌（panic）、最后写入优先（LWW）、死锁（deadlock）、多数决强制。本文提出基于 **Belnap 四值逻辑**（TRUE/FALSE/NEITHER/BOTH）的状态机设计，将矛盾视为**第一等数学状态**，在不中断线性执行的前提下予以保留。核心贡献是**链上 meet 代数**（$FALSE < NEITHER < BOTH < TRUE$）：由于 TRUE 位于链的顶端，**"非 TRUE 值组合绝不会产生 TRUE"**这一不变条件由结构保证，而非依赖运行时检查。设计以 C（零堆分配、纯位运算）与 Rust（`no_std`）实现，通过穷举测试（16/64 组合）与模糊测试（65,536 输入）验证。最后，该理论已在 **NEAR 主网**部署为智能合约（110 kits、136 capabilities、规范地址 `zedec.near`、合约地址 `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`），通过对抗性审计与燃气墙测试。全部主张可在链上免费验证。

---

## 1. 引言：为什么"矛盾"是系统原语

### 1.1 分布式系统的根本问题

每个分布式系统都必须回答一个问题：

> **当两个信息源对真相的认定冲突时，系统应该做什么？**

经典逻辑以排中律为基础。但在分布式环境中，网络分区或复制延迟可能导致 **$P$ 与 $\neg P$ 的证据同时到达**。在经典框架下，这会触发爆炸原理：从矛盾 $P \land \neg P$ 可推出任意命题，整个推理系统失去意义。

因此经典实现被迫在四种"回避"策略中选择：

| 策略 | 行为 | 代价 |
|---|---|---|
| **恐慌（panic）** | 抛出异常、回滚状态 | 线性执行中断、进行中状态丢失 |
| **最后写入优先（LWW）** | 时间戳较新者覆盖 | 旧的真相被悄然抹除 |
| **死锁（deadlock）** | 停止处理并等待 | 整个系统冻结 |
| **多数决强制** | 票数多者强制解决 | 胜出的是权力而非真相 |

四种策略都把**矛盾视为 bug**。没有一种保留矛盾所蕴含的信息；每一种都在"系统可用性"与"数据完整性"之间牺牲其一。

### 1.2 Paraconsistent 逻辑的转向

Paraconsistent 逻辑（矛盾容忍逻辑）翻转前提：

> **矛盾不是错误状态，而是信息状态。**

系统同时保留两方主张、记录冲突、在证据真正尘埃落定前不强制解决。这不是哲学立场，而是**数学上良定义的四值逻辑**，可作为状态机原语实现。

---

## 2. 理论：四值逻辑的代数结构

### 2.1 状态定义（位模式）

```c
/* lpres_phases.h —— 实机内核的状态定义（节录） */
typedef enum {
    LPRES_STATE_NEITHER = 0,  /* 无证据（未观测·保留） */
    LPRES_STATE_TRUE    = 1,  /* 合意·真（可结算）     */
    LPRES_STATE_FALSE   = 2,  /* 明确否决（拒绝）       */
    LPRES_STATE_BOTH    = 3,  /* 矛盾保留（双方证据）   */
    LPRES_STATE_GATEWAY = 4,  /* 第五值：扩展端口       */
    /* ... 斐波那契里程碑的扩展相位 ... */
    LPRES_STATE_COUNT   = 145 /* 扩展后总状态数         */
} lpres_state_t;
```

**关键设计**：状态以**单字节位模式**表示。`0x00`–`0x03` 是四值逻辑核心，`0x04` 之后是扩展相位。状态转移是纯整数运算——零堆分配、零动态内存、零锁。

### 2.2 链上 Rank（全序定义）

核心**不是 Belnap 的格（lattice），而是全序链上的 rank**：

```c
/* lpres_phases.h —— 链上 rank（规范实现） */
static const uint8_t lpres_rank[LPRES_STATE_COUNT] = {
    [LPRES_STATE_FALSE]   = 0,  /* 底（meet 的支配者）  */
    [LPRES_STATE_NEITHER] = 1,  /* 无证据               */
    [LPRES_STATE_BOTH]    = 2,  /* 矛盾                 */
    [LPRES_STATE_TRUE]    = 3,  /* 顶（join 的单位元）  */
    [LPRES_STATE_GATEWAY] = 4,
    /* ... 每个扩展相位取下一 rank ... */
};
```

全序为

$$ \text{FALSE} < \text{NEITHER} < \text{BOTH} < \text{TRUE} $$

**FALSE 为底，TRUE 为顶**——结构的关键。

### 2.3 结合运算（conjoin）= 链上 Meet

```c
lpres_state_t lpres_conjoin(lpres_state_t a, lpres_state_t b) {
    /* the meet: min on the chain. FALSE dominates, TRUE is identity,
     * NEITHER absorbs BOTH — all by the rank, so a new phase needs only
     * its rank, never a new branch. Deliberate deviation from Belnap's
     * FOUR (the chain, not the lattice): no mix of non-TRUE values can
     * yield TRUE, the only value that authorises an action. */
    if ((unsigned) a >= LPRES_STATE_COUNT ||
        (unsigned) b >= LPRES_STATE_COUNT)
        return LPRES_STATE_NEITHER;   /* 未知相位绝不成权威 */
    return lpres_rank[a] <= lpres_rank[b] ? a : b;
}
```

**Meet = 返回 rank 较小者**。性质：

| 输入 | 结果 | 意义 |
|---|---|---|
| `conjoin(FALSE, x)` | `FALSE` | FALSE 支配（底） |
| `conjoin(TRUE, TRUE)` | `TRUE` | 唯有合意可结算 |
| `conjoin(BOTH, TRUE)` | `BOTH` | 矛盾保留、绝不强制 |
| `conjoin(NEITHER, BOTH)` | `NEITHER` | NEITHER 吸收 BOTH |
| `conjoin(未知值, x)` | `NEITHER` | 越界保护（fail-closed） |

### 2.4 分离运算（disjoin）= 链上 Join

```c
lpres_state_t lpres_disjoin(lpres_state_t a, lpres_state_t b) {
    /* the join: max on the chain. TRUE dominates, FALSE is identity,
     * BOTH absorbs NEITHER — all by the rank. */
    if ((unsigned) a >= LPRES_STATE_COUNT ||
        (unsigned) b >= LPRES_STATE_COUNT)
        return LPRES_STATE_NEITHER;
    return lpres_rank[a] >= lpres_rank[b] ? a : b;
}
```

Join = **返回 rank 较大者**。TRUE 为顶，故 `disjoin(TRUE, x) = TRUE`。

### 2.5 代数性质（已验证）

链上 meet/join 构成**有界格**。全组合穷举验证：

```
meet：幂等 ○ 交换 ○ 结合 ○
join：幂等 ○ 交换 ○ 结合 ○
吸收律：○（a ∧ (a ∨ b) = a）
偏序：○（a ≤ b ⇔ meet(a,b) = a）
不变条件：○（非 TRUE 值的 meet 不会产生 TRUE）
```

**这是结构而非偶然。** TRUE 为 rank 3（顶），meet（返回最小值）要成为 TRUE **唯有两输入皆为 TRUE**。因此：

> **"唯有真可结算（only TRUE settles）"由 rank 结构数学推导而出。**

### 2.6 对 Belnap 四值格的刻意偏离

源码注释明言：

> *Deliberate deviation from Belnap's FOUR (the chain, not the lattice): no mix of non-TRUE values can yield TRUE, the only value that authorises an action.*

Belnap 的 `FOUR` 是格（两个顺序：信息序与真伪序）。本实现**收缩为单一全序链**。好处：

- 新增相位仅需**新增 rank**（无需新分支逻辑）
- 实现**缩减为单一定点比较**（`<=`）
- Belnap 格中"非 TRUE 组合产生 TRUE"的问题被**结构性排除**

---

## 3. 实现：C 零分配

### 3.1 完整独立实现

```c
/* belnap_gate.c —— 零分配·矛盾容忍状态机
 * 编译：gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate
 * 运行：./belnap_gate
 */
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

typedef enum {
    NEITHER = 0x00, TRUE = 0x01, FALSE = 0x02, BOTH = 0x03,
} belnap_state_t;

/* enum 顺序 [NEITHER,TRUE,FALSE,BOTH] -> {1,3,0,2}（与实机内核相同） */
static const uint8_t rank[4] = { 1, 3, 0, 2 };

static belnap_state_t conjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER; /* fail-closed */
    return rank[a] <= rank[b] ? a : b;
}

static belnap_state_t disjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER;
    return rank[a] >= rank[b] ? a : b;
}

static bool is_settleable(belnap_state_t s) { return s == TRUE; }

int main(void) {
    /* 矛盾产生：经典逻辑中的 panic 点 */
    belnap_state_t contention = (belnap_state_t)((unsigned)TRUE | (unsigned)FALSE);
    printf("State: 0x%02X (%s)\n", contention, contention == BOTH ? "BOTH" : "OTHER");
    printf("Can Settle? %s\n", is_settleable(contention) ? "YES" : "NO (Blocked without panic)");
    belnap_state_t resolved = conjoin(TRUE, TRUE);
    printf("Resolved: 0x%02X -> Can Settle? %s\n", resolved, is_settleable(resolved) ? "YES" : "NO");
    belnap_state_t unknown = conjoin((belnap_state_t)7, TRUE);
    printf("Unknown-guard: 0x%02X -> Can Settle? %s\n", unknown, is_settleable(unknown) ? "YES" : "NO");
    belnap_state_t j = disjoin(BOTH, NEITHER);
    printf("Disjoin(BOTH,NEITHER): 0x%02X (%s)\n", j, j == BOTH ? "BOTH" : "OTHER");
    return 0;
}
```

### 3.2 内存特性

```
栈：3 个局部变量 = 24 字节
堆：0（无 malloc/free）
静态数据：rank[4] = 4 字节（.rodata）
代码：汇编级数十条指令
```

**平坦、可预测、零堆抖动**——实时系统可视为保证上限。

### 3.3 汇编级验证

`-O3` 下 meet 编译为：

```asm
conjoin:
    movzx   eax, dil
    movzx   ecx, sil
    movzx   edx, BYTE PTR rank[rax]
    cmp     BYTE PTR rank[rcx], dl
    cmovle  eax, esi
    ret
```

对分支预测友好，热路径性能有保证。

---

## 4. 实现：Rust / no_std 类型安全

```rust
//! lib.rs —— no_std 不变条件验证
#![no_std]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Belnap { Neither = 0x00, True = 0x01, False = 0x02, Both = 0x03 }

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
    fn false_dominates() {
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

### 4.1 属性式验证（KAT）

- 64 种结合律验证（4×4×4）
- 16 种不变条件验证（4×4）
- 状态空间有限，即**完全穷举**

### 4.2 WASM

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```
数 KB WASM，不调 heap，纯函数导出，gas 计量简化。

---

## 5. 状态机设计

### 5.1 四值转移模式

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

**要点**：
1. NEITHER 是"等待"而非"搁置"——不阻塞也不推进。
2. BOTH 进入"争议解决"——不自动解决，但写审计轨迹。
3. **仅 TRUE 允许副作用**——`is_settleable(s)` 为 false 时不发生任何写入，结构性防止部分更新不一致，TOCTOU 检查并入不变式。

### 5.2 Event Sourcing 集成

```
[Event 1] attest(TRUE)   → TRUE
[Event 2] attest(FALSE)  → BOTH（矛盾被记录）
[Event 3] resolve(TRUE)  → TRUE（解决）
```
Append-only 日志，完整审计轨迹。

---

## 6. 验证

### KAT 表
| 测试 | 输入 | 期望 |
|---|---|---|
| KAT-01 | `conjoin(TRUE, FALSE)` | `FALSE` |
| KAT-02 | `conjoin(BOTH, TRUE)` | `BOTH` |
| KAT-03 | `conjoin(NEITHER, BOTH)` | `NEITHER` |
| KAT-04 | `conjoin(TRUE, TRUE)` | `TRUE` |
| KAT-05 | `conjoin(0xFF, TRUE)` | `NEITHER`（fail-closed） |
| KAT-06 | `disjoin(BOTH, NEITHER)` | `BOTH` |

### Fuzzing（65,536 输入）
```c
for (unsigned a = 0; a < 256; a++)
    for (unsigned b = 0; b < 256; b++) {
        belnap_state_t m = conjoin((belnap_state_t)a, (belnap_state_t)b);
        if (m == TRUE && (a != TRUE || b != TRUE)) { printf("VIOLATION\n"); return 1; }
    }
printf("FUZZ OK: 65536 combinations\n");
```

---

## 7. NEAR 链上部署（概念证明）

这不是纸上设计——**NEAR 主网上跑着智能合约**。

### 地址
- **Canonical（路由）**：`zedec.near`（`route()` 返回完整拓扑）
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

### 逻辑落地
- **威尼斯加油站**：TRUE 结算获得协议 2× 燃料匹配；BOTH 失去匹配（排水盾）——矛盾有成本。
- **主权法令**："无分叉之分叉"——矛盾法令共存。
- **Receipt 链**：每次结算链接进哈希链（event sourcing 链上实现）。
- **状态兼容升级**：顶层结构体永不改变，新功能状态存入保留命名空间（Borsh 前向兼容）。

### 质量记录（实测）
| 验证 | 结果 |
|---|---|
| QC harness（对内核 C 源码） | 55/55，零 bug |
| 链上全审计 | 24/24 |
| 耐久性压力 | 70/70 |
| 注册审计 | 136/136 |
| 燃气墙（读/写） | 125/125 / 10/10 |

---

## 8. 结论与展望

四值逻辑解决经典系统未解决的问题：**数据冲突时怎么办？** 答案是：把矛盾当信息保存、不强制、不 panic。而"唯有真可结算"是 rank 结构的**数学推论**，不是运行时检查。

展望：穷举测试升级为 Coq/Isabelle 机械证明；rank 表扩展到 145 状态；WASM 版跨链移植；四值逻辑门提案为 NEAR Enhancement Proposal。

**系统开放。**

---

## 附录：参考实现

- **内核实现**（`kernel/src/lpres/`）：https://github.com/sykomiked/zxv-stack
- **智能合约**（NEAR，canonical `zedec.near`）：https://github.com/sykomiked/ZXV-NEP-paraconsistent

---

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC · SPDX-License-Identifier: Apache-2.0*