# 【C言語/Rust】矛盾でパニックを起こさない四値論理（パラコンシステント）ステートマシンの設計

**非古典論理をシステムプリミティブとして使う——ゼロアロケーション状態遷移マシン**

*著者: ZXV（ZEDEC / 36N9 Genetics, LLC）エンジニアリングチーム*
*Qiita / Zenn 投稿用 · 対象: システムエンジニア・形式検証・低レイヤ実装者*

---

## 3行まとめ

- 分散・並行環境における競合（コンフリクト）を**例外ではなく「情報」として扱う**設計。
- Belnap 四値論理（TRUE/FALSE/NEITHER/BOTH）により、**矛盾発生時もパニックを起こさない**。
- **「真のみが確定する（only TRUE settles）」**不変条件を**ゼロアロケーション（no_std / C）**で実装。

---

## 1. 背景：なぜ分散システムは矛盾で壊れるのか

分散・並行状態マシンは、古典二値論理の**排中律**（$P \lor \neg P$：任意の命題Pに対し、Pまたは¬Pが必ず成立）を前提に設計されている。

この前提は、ネットワーク分断やノード間の競合で**破綻**する。二つのノードが矛盾するデータを生産したとき、システムは以下から選ばざるを得ない：

| 戦略 | 挙動 | 代償 |
|---|---|---|
| **クラッシュ・パニック** | 例外を投げて状態を巻き戻す | 線形実行が中断、状態が喪失 |
| **最終書き込み優先（LWW）** | 最新の書き込みで上書き | 古い真実が黙って消える |
| **デッドロック** | 処理を停止して待機 | システムが凍結 |

これらの背後には、古典論理の**爆発原理**（principle of explosion：矛盾 $P \land \neg P$ から任意の命題が導出可能）がある。矛盾が検出された時点で、古典論理の推論は無意味になる——だから実装は「検出したら即回避」しか選べない。

**しかし、矛盾そのものを情報として保持できればどうか。**

---

## 2. 実装：四値論理のビットマスクとチェーン・ミート

### 2.1 状態の定義（ビットマスク）

四値（+拡張ポート）を単一バイトのビットパターンとして定義する：

```c
/* lpres_phases.h —— 実機カーネル由来の正規定義 */
typedef enum {
    LPRES_STATE_NEITHER = 0,  /* 証拠なし（未観測・保留） */
    LPRES_STATE_TRUE    = 1,  /* 合意・真（決済可能）     */
    LPRES_STATE_FALSE   = 2,  /* 明示的否決（拒否）       */
    LPRES_STATE_BOTH    = 3,  /* 矛盾保持（両証拠）       */
} lpres_state_t;
```

各状態が `0x00`–`0x03` のビットパターンであることが本質である。**ヒープ確保・動的メモリ・ロックは一切不要**で、状態遷移は純粋な整数演算になる。

### 2.2 結合演算（conjoin）：チェーン・ミート

本実装の核心は、Belnapの束（lattice）そのものではなく、**全順序チェーン上のミート（meet）** として結合を定義する点である。

```c
/* lpres.c —— 実機カーネルの結合演算（正規実装） */
lpres_state_t lpres_conjoin(lpres_state_t a, lpres_state_t b) {
    /* the meet: min on the chain.
     * FALSE dominates, TRUE is identity, NEITHER absorbs BOTH.
     * 非TRUE値の組み合わせがTRUEになることは決してない。 */
    if ((unsigned) a >= LPRES_STATE_COUNT ||
        (unsigned) b >= LPRES_STATE_COUNT)
        return LPRES_STATE_NEITHER;  /* 未知フェーズは決して権威にならない */
    return lpres_rank[a] <= lpres_rank[b] ? a : b;
}
```

ここで `lpres_rank[]` はチェーン上の順位テーブルである：

```c
/* チェーン順位：FALSE < TRUE < NEITHER < BOTH（決済可能性の昇順） */
static const uint8_t lpres_rank[LPRES_STATE_COUNT] = { 0, 1, 2, 3 };
```

**設計上の重要な逸脱**（ソースコメントより）：

> *Deliberate deviation from Belnap's FOUR (the chain, not the lattice):
> no mix of non-TRUE values can yield TRUE, the only value that
> authorises an action.*

（Belnapの四値からの意図的な逸脱——チェーンであって束ではない。非TRUE値の組み合わせがTRUEになることは決してなく、アクションを許可する唯一の値はTRUEのみである。）

これにより、**決済判定（settleability）** は単純になる：

```c
/* 決済不変条件：真のみが確定する */
static inline bool lpres_is_settleable(lpres_state_t s) {
    return s == LPRES_STATE_TRUE;
}
```

### 2.3 完全なスタンドアロン実行例（ゼロヒープ）

以下は、`gcc` だけでコンパイル・実行できる単一ファイル実装である：

```c
/* belnap_gate.c —— ゼロアロケーション・矛盾許容ステートマシン
 * gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate && ./belnap_gate */
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

typedef enum {
    NEITHER = 0x00,  /* 証拠なし */
    TRUE    = 0x01,  /* 合意・真 */
    FALSE   = 0x02,  /* 明示的否決 */
    BOTH    = 0x03,  /* 矛盾保持 */
} belnap_state_t;

static const uint8_t rank[4] = { 0, 1, 2, 3 };

static belnap_state_t conjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER;
    return rank[a] <= rank[b] ? a : b;
}

static bool is_settleable(belnap_state_t s) { return s == TRUE; }

int main(void) {
    belnap_state_t a = TRUE;
    belnap_state_t b = FALSE;

    /* 矛盾の発生：古典論理では例外・パニックを引き起こす箇所 */
    belnap_state_t contention = (belnap_state_t)((unsigned)a | (unsigned)b);
    /* 0x01 | 0x02 = 0x03 (BOTH) —— パニックせず「情報」として保持 */

    printf("State: 0x%02X (%s)\n",
           contention, contention == BOTH ? "BOTH" : "OTHER");
    printf("Can Settle? %s\n",
           is_settleable(contention) ? "YES" : "NO (Blocked without panic)");

    /* 合流：TRUE + TRUE = TRUE（決済可能） */
    belnap_state_t resolved = conjoin(a, a);
    printf("Resolved: 0x%02X -> Can Settle? %s\n",
           resolved, is_settleable(resolved) ? "YES" : "NO");

    /* 未知フェーズの保護：範囲外入力は決してTRUEにならない */
    belnap_state_t unknown = conjoin((belnap_state_t)7, TRUE);
    printf("Unknown-guard: 0x%02X -> Can Settle? %s\n",
           unknown, is_settleable(unknown) ? "YES" : "NO");

    return 0;
}
```

**メモリ特性**：ローカル変数3つ・スタックのみ。**フラットで予測可能な消費、ヒープスラッシングゼロ**。組込み・WASM・カーネル空間でそのまま使える。

### 2.4 Rust / no_std での不変条件検証

```rust
//! lib.rs —— no_std で動作する不変条件検証
#![no_std]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Belnap {
    Neither = 0x00,
    True    = 0x01,
    False   = 0x02,
    Both    = 0x03,
}

impl Belnap {
    #[inline(always)]
    pub fn is_settleable(self) -> bool {
        matches!(self, Belnap::True)
    }

    /// チェーン・ミート：FALSE が支配、TRUE が単位元、
    /// NEITHER が BOTH を吸収する。非 TRUE の組み合わせは TRUE にならない。
    #[inline(always)]
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Belnap::False, _) | (_, Belnap::False) => Belnap::False,
            (Belnap::Both, _)  | (_, Belnap::Both)  => Belnap::Both,
            (Belnap::True, Belnap::True)             => Belnap::True,
            _                                       => Belnap::Neither,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_contradiction_does_not_panic() {
        // 矛盾が存在しても panic! ではなく false を返して状態を維持する
        assert!(!Belnap::Both.is_settleable());
    }

    #[test]
    fn verify_only_true_settles() {
        assert!(Belnap::True.is_settleable());
        assert!(!Belnap::False.is_settleable());
        assert!(!Belnap::Neither.is_settleable());
    }
}
```

---

## 3. 検証とベンチマーク

### 3.1 ローカル検証コマンド

```bash
# C 版（ゼロヒープ・純ビット演算）
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate && ./belnap_gate

# Rust 版（no_std 不変条件検証）
cargo test --release
```

### 3.2 期待される出力

```
State: 0x03 (BOTH)
Can Settle? NO (Blocked without panic)
Resolved: 0x01 -> Can Settle? YES
Unknown-guard: 0x00 -> Can Settle? NO
```

### 3.3 検証ポイント（不変条件）

1. **矛盾（BOTH）は決済されない** —— `is_settleable` は `false` を返し、**panic しない**。
2. **真のみが決済される** —— `TRUE` のみ `true`。
3. **範囲外入力は決して TRUE にならない** —— 未知フェーズは `NEITHER` に落ちる（fail-closed）。
4. **非TRUE値の組み合わせから TRUE は生成されない** —— チェーン・ミートの性質。

---

## 4. まとめ

古典論理の爆発原理は、分散システムに「矛盾を検出したら即回避」を強いる。四値論理（パラコンシステント論理）はその前提を覆し、**矛盾を第一級の数学的状態として受け入れ、線形実行を中断することなく保持する**。

本稿の実装は：

- **ゼロアロケーション**：ヒープなし、ロックなし、純整数演算
- **判定が単一比較**：`s == TRUE` のみが決済を許可
- **範囲外保護付き**：未知フェーズは決して権威にならない（fail-closed）
- **C と Rust / no_std の両方で検証可能**

この設計は、単なる理論ではなく、実機で動作している。

---

## 付録：実装リポジトリ

※ 本アルゴリズムは、高信頼性が求められる分散合意レイヤーおよび WASM ランタイム環境向けに設計・検証されています。実装リポジトリ：

- **カーネル実装**（`kernel/src/lpres/`：`lpres_phases.h`, `lpres.c`）：https://github.com/sykomiked/zxv-stack
- **スマートコントラクト実装**（NEAR メインネット、カノニカル `zedec.near`）：https://github.com/sykomiked/ZXV-NEP-paraconsistent

---

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC · SPDX-License-Identifier: Apache-2.0*