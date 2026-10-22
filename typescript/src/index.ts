/**
 * @zxv/developer-kit — drop-in integration for the ZXV settlement layer.
 *
 * Discover the 18 subsystems, verify rail cards, read dispute-tolerant
 * prices, check the settlement invariant, and journal receipts — all
 * with zero runtime dependencies (plain fetch).
 *
 * ```ts
 * import { Zxv } from "@zxv/developer-kit";
 *
 * const zxv = new Zxv();
 * const caps = await zxv.discover();          // 18 subsystems
 * const net  = await zxv.verifyCard("88090016741821469134"); // "dragon"
 * const q    = await zxv.oraclePrice("NEAR"); // { price, settleable }
 * ```
 *
 * Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
 * SPDX-License-Identifier: Apache-2.0
 */

/** The four foundation states. */
export enum LogicState {
  NEITHER = 0x00,
  TRUE = 0x01,
  FALSE = 0x02,
  BOTH = 0x03,
}

/** The mainnet accounts (the 18-subsystem fabric). */
export const ACCOUNTS = {
  worldRegistry: "7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a",
  gate: "6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8",
  railsRegistry: "e4111bdb1e38a5c7761b5ad5322af4569389b4ac7e3964b2",
  railsReceipts: "882728f5a6275951f33a06d8d8b3d19f1ec1e2a20f93009b",
  oracle: "ea85f5381ceff17553fa3093a2d67a0e6cc4853abd7c19487ed7a33391acc2f7",
  settlementRouter: "8bf177019402d3d843b361a04d3ed7e21de637bf5d252956a15208f7950effae",
  aiVerdict: "781fc03d4dfb9b29d88df561157b11db6dc1e02368eae6deef7998d188ff966e",
  verdictGuard: "88e17c606d29c45e",
  treasury: "c07140bfefd94ab5",
  yhvRedeemer: "97db094fae2ba0dd",
  yhvWrapper: "1a051ac2465c9a04e532fe420e2efbd8cc12f9a1d9f2a350470782fd3255ecf1",
} as const;

/** The ZXV client. */
export class Zxv {
  readonly rpc: string;

  constructor(opts?: { rpc?: string }) {
    this.rpc = opts?.rpc ?? "https://rpc.mainnet.fastnear.com";
  }

  /** Discover the 18 subsystems (the registry's capability list). */
  async discover(): Promise<string[]> {
    return this.view<string[]>(ACCOUNTS.worldRegistry, "vm_capabilities", {});
  }

  /** Resolve a subsystem to its live account. */
  async resolve(name: string): Promise<string | null> {
    const cap = await this.view<{ contract?: string } | null>(ACCOUNTS.worldRegistry, "vm_capability", { name });
    return cap?.contract ?? null;
  }

  /** Verify a rail card number: which network ("dragon" | "phoenix" | "thunderbird" | null). */
  async verifyCard(number: string): Promise<string | null> {
    const net = await this.view<string | null>(ACCOUNTS.railsRegistry, "verify_number", { number });
    return net;
  }

  /** The dispute-tolerant price for an asset. */
  async oraclePrice(symbol: string): Promise<{ price: string; verdict: string; settleable: boolean }> {
    return this.view(ACCOUNTS.oracle, "price", { symbol });
  }

  /** The settlement invariant: is this state settleable? */
  async isSettleable(state: LogicState): Promise<boolean> {
    return this.view(ACCOUNTS.settlementRouter, "is_settleable", { state: this.stateName(state) });
  }

  /** The receipt chain head (the audit anchor). */
  async chainHead(): Promise<{ height: number; head: string }> {
    return this.view(ACCOUNTS.railsReceipts, "head", {});
  }

  /** Verify the receipt chain from a height (the audit proof). */
  async verifyChain(from: number): Promise<boolean> {
    return this.view(ACCOUNTS.railsReceipts, "verify_chain", { from });
  }

  /** The safe evidence combination (the verdict guard). */
  async conjoin(inputs: number[]): Promise<LogicState> {
    const v = await this.view<string>(ACCOUNTS.verdictGuard, "conjoin", { inputs });
    return this.stateValue(v);
  }

  private stateName(s: LogicState): string {
    return ["NEITHER", "TRUE", "FALSE", "BOTH"][s] ?? "NEITHER";
  }

  private stateValue(name: string): LogicState {
    return ({ NEITHER: 0, TRUE: 1, FALSE: 2, BOTH: 3 } as Record<string, LogicState>)[name] ?? 0;
  }

  /** Minimal JSON-RPC view call (no dependencies). */
  private async view<T>(accountId: string, method: string, args: Record<string, unknown>): Promise<T> {
    const argsBase64 = Buffer.from(JSON.stringify(args)).toString("base64");
    const body = {
      jsonrpc: "2.0", id: "zxv-kit", method: "query",
      params: { request_type: "call_function", finality: "final", account_id: accountId, method_name: method, args_base64: argsBase64 },
    };
    const res = await fetch(this.rpc, { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify(body) });
    const data = (await res.json()) as { result?: { result: number[] } };
    const bytes = data.result?.result ?? [];
    const text = Buffer.from(bytes).toString("utf8");
    return JSON.parse(text) as T;
  }
}