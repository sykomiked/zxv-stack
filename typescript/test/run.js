// test/run.js — live verification of @zxv/developer-kit against NEAR mainnet.
const { Zxv, LogicState } = require("../dist/index.js");

(async () => {
  const zxv = new Zxv();
  console.log("=== @zxv/developer-kit live test (NEAR mainnet) ===");

  // 1. discover the 18 subsystems
  const caps = await zxv.discover();
  console.log("1. discover():", caps.length, "subsystems");
  console.log("   has rails_registry:", caps.includes("rails_registry"));
  console.log("   has rails_receipts:", caps.includes("rails_receipts"));

  // 2. resolve a subsystem
  const router = await zxv.resolve("settlement_router");
  console.log("2. resolve(settlement_router):", router ? router.slice(0, 16) + "…" : "null");

  // 3. verify a rail card (a testnet-minted number, verified on mainnet)
  const net = await zxv.verifyCard("88090016741821469134");
  console.log("3. verifyCard(88090016741821469134):", net);

  // 4. the dispute-tolerant price
  const q = await zxv.oraclePrice("NEAR");
  console.log("4. oraclePrice(NEAR):", JSON.stringify(q));

  // 5. the settlement invariant
  console.log("5. isSettleable(BOTH):", await zxv.isSettleable(LogicState.BOTH));
  console.log("   isSettleable(TRUE):", await zxv.isSettleable(LogicState.TRUE));

  // 6. the receipt chain
  const head = await zxv.chainHead();
  console.log("6. chainHead():", JSON.stringify(head));

  const pass = caps.length === 18 && caps.includes("rails_registry") && caps.includes("rails_receipts")
    && router !== null && net === "dragon"
    && !(await zxv.isSettleable(LogicState.BOTH)) && await zxv.isSettleable(LogicState.TRUE);
  console.log("\nVERDICT:", pass ? "ALL PASS — the kit works against live mainnet" : "FAIL");
  process.exit(pass ? 0 : 1);
})().catch((e) => { console.error("FAILED:", e.message); process.exit(1); });