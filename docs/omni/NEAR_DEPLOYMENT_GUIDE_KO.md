# ZXV NEAR 배포 사용: 완전한 안내

**한국어** · ZXV — 파라컨시스턴트논리 · 상태머신 · 결제


ZXV는 NEAR 네트워크에서 파라컨시스턴트 논리의 살아있는 구현이다. 이 안내는 배포를 읽고, 검증하고, 사용하는 방법을 설명한다 — 가스 없이, 읽기에 지갑 불필요.

---

## 주소
정규 주소는 zedec.near, 살아있는 라우팅 계약이다. 프로토콜(110키트 마이크로 계약), 레지스트리(136 능력), 게이트(커널 불변식)를 가리킨다.

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## 시스템 읽는 법
어떤 클라이언트든 view 호출로 가스 없이 읽을 수 있다. 보편 인터페이스는 스스로 가르친다: ux_manifest는 전체 스키마를, ux_help는 어떤 주제든 답한다.

---

## 참여 방법
새 행위자는 mint_onboard를 호출한다: 일회성 5000 단위 보조금, 플랫폼에서 사용 가능하나 양도 불가. 그다음 pool_enter가 유동성 풀을 통해 접근을 준다.

---

## 검증 방법
불변식은 온체인에서 검증 가능하다: conjoin은 사치 결과를, is_settleable은 TRUE에만 참을, verify_chain은 감사 체인을 확인한다.

---

## 참조
커널: github.com/sykomiked/zxv-stack · NEP 및 원장: github.com/sykomiked/ZXV-NEP-paraconsistent · 개발자 키트: github.com/sykomiked/zxv-developer-kit.

시스템은 열려 있다.

**오직 TRUE만이 결제한다. TRUE+FALSE=BOTH — 모순은 기록되며, 결코 강제되지 않는다.**

