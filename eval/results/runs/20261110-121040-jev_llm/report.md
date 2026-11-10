# 평가 결과 — 20261110-121040-jev_llm

- 실행: 2026-11-10T12:10:40Z → 2026-11-10T12:13:15Z (UTC) · git `86af620` (커밋 안 된 변경 있음)
- 데이터셋: `kb` v1.0.0 · kb `489e5fca3c7c` · cases `b2a362fcd60d`
- 설정: routers ["jev_llm"] · variants ["standard","deep"] · suites ["all"] · split all · repeat 1 · concurrency 6 · beam_width "case/default" · limit "case/default"
- 모델(jev_llm): jev `jev-latest` · LLM `meta/muse-ember-1.0`
- 메모: 0.11.0: Jev + muse-ember-1.0 (low) reviews picks below 0.8

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev_llm | standard | 444 | 97.7% | 96.8% | 0.981 | 98.7% | 0.993 | 0.988 | 0.944 | 1.5% | 1.3% | 0.5s / 2.7s | 2.4 | 0 | 100.0% | 100.0% |
| jev_llm | deep | 445 | 97.3% | 96.9% | 0.981 | 97.9% | 0.989 | 0.984 | 0.959 | 1.5% | 1.1% | 0.5s / 3.0s | 2.4 | 0 | 100.0% | 100.0% |

## jev_llm · standard

### 검색 (444건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 97.7% [95.9–98.8] (n=444) |
|   └ 답할 수 있는 질문 | 97.6% [95.5–98.7] (n=376) |
|   └ 답할 수 없는 질문(보류가 정답) | 98.5% [92.1–99.7] (n=68) |
| 확신 정답률 (top1 정답 + recommended) | 96.0% [93.5–97.6] (n=376) |
| 라우팅 정확도 (정확 일치) | 96.8% [94.8–98.1] (n=444) |
| 계층 F1 (hF1) | 0.981 [0.970–0.992] (n=444) |
| 트리 밖 판정 precision | 100.0% [89.8–100.0] (n=34) |
| 트리 밖 판정 recall | 89.5% [75.9–95.8] (n=38) |
| 트리 밖 판정 F1 | 0.944 |
| 오보류율 (답할 수 있는데 보류) | 1.3% [0.6–3.1] (n=376) |
| 오답변율 (답할 수 없는데 답함) | 1.5% [0.3–7.9] (n=68) |
| Hit@1 | 98.7% [96.9–99.4] (n=376) |
| Hit@3 | 100.0% [99.0–100.0] (n=376) |
| Hit@k (limit) | 100.0% [99.0–100.0] (n=376) |
| MRR@k | 0.993 [0.988–0.999] (n=376) |
| nDCG@k | 0.988 [0.984–0.993] (n=376) |
| Recall@k | 1.000 [1.000–1.000] (n=376) |
| recommended 정밀도 | 0.972 [0.957–0.986] (n=363) |
| 라우팅이 맞았을 때 Hit@1 | 99.2% [97.6–99.7] (n=367) |
| 라우팅이 맞았을 때 MRR | 0.996 [0.991–1.000] (n=367) |
| 지연 p50 / p90 / p95 / 평균 | 0.5s / 1.0s / 2.7s / 0.8s |
| 라우팅 구간 평균 | 0.5s |
| 랭킹 후보 수 평균 | 32.2 |
| Jev 입력 토큰 평균 (과금) | 22870 |
| LLM 토큰 평균 (입력+출력) | 1268 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 2.39 |
| LLM 라우팅 호출 수 평균 | 0.09 |
| LLM이 경로를 정한 비율 (Jev가 불확실해 리뷰) | 9.0% [6.7–12.0] (n=444) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 99.7% (n=359) | 98.3% (n=349) | 97.3% (n=330) | 97.6% (n=82) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|
| 2 | 1 | 498 | 4 | 7 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.5s |
| internal | 23 | 95.7% | 100.0% | 95.7% | 0.978 | 0.5s |
| leaf | 353 | 97.7% | 97.5% | 98.9% | 0.994 | 0.5s |
| no_answer | 27 | 100.0% | 96.3% | - | - | 0.5s |
| oos | 38 | 97.4% | 89.5% | - | - | 0.5s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 98.1% | 97.1% | 98.1% | 0.990 | 0.5s |
| single | 339 | 97.6% | 96.8% | 98.9% | 0.995 | 0.5s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 339 | 97.6% | 96.8% | 98.9% | 0.995 | 0.5s |
| coref | 19 | 94.7% | 100.0% | 94.7% | 0.974 | 0.5s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| disambig | 20 | 95.0% | 90.0% | 95.0% | 0.975 | 0.5s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 0.5s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 384 | 97.7% | 96.6% | 98.5% | 0.993 | 0.5s |
| root | 43 | 97.7% | 97.7% | 100.0% | 1.000 | 0.4s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 97.1% | 100.0% | 95.7% | 0.978 | 0.5s |
| leaf | 372 | 97.8% | 97.3% | 98.9% | 0.994 | 0.5s |
| none | 38 | 97.4% | 89.5% | - | - | 0.5s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 97.4% | 89.5% | - | - | 0.5s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.7s |
| d2 | 30 | 96.7% | 100.0% | 94.7% | 0.974 | 0.5s |
| d3 | 278 | 97.1% | 97.1% | 98.5% | 0.992 | 0.5s |
| d4 | 88 | 100.0% | 97.7% | 100.0% | 1.000 | 0.5s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 97.6% | 90.2% | - | - | 0.5s |
| 101+ | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| 2-5 | 319 | 97.5% | 96.9% | 98.7% | 0.993 | 0.5s |
| 33-100 | 61 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| 6-32 | 19 | 94.7% | 100.0% | 90.0% | 0.950 | 0.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| colloquial | 69 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| condition | 32 | 93.8% | 100.0% | 100.0% | 1.000 | 0.5s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| keyword | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| mixed | 13 | 92.3% | 100.0% | 100.0% | 1.000 | 0.5s |
| multi_intent | 9 | 77.8% | 22.2% | 88.9% | 0.944 | 2.3s |
| negation | 20 | 95.0% | 100.0% | 95.0% | 0.975 | 0.5s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.5s |
| oos_near | 15 | 100.0% | 80.0% | - | - | 2.8s |
| paraphrase | 177 | 98.3% | 97.7% | 98.6% | 0.993 | 0.5s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| vague | 23 | 95.7% | 100.0% | 95.7% | 0.978 | 0.5s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| health | 40 | 100.0% | 97.5% | 100.0% | 1.000 | 0.5s |
| it | 101 | 99.0% | 98.0% | 99.0% | 0.995 | 0.5s |
| life | 41 | 100.0% | 97.6% | 100.0% | 1.000 | 0.5s |
| money | 49 | 93.9% | 98.0% | 97.8% | 0.989 | 0.5s |
| oos | 38 | 97.4% | 89.5% | - | - | 0.5s |
| shop | 41 | 95.1% | 92.7% | 94.7% | 0.974 | 0.6s |
| travel | 44 | 97.7% | 100.0% | 100.0% | 1.000 | 0.5s |
| work | 44 | 95.5% | 95.5% | 97.6% | 0.988 | 0.5s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 98.3% | 100.0% | 99.1% | 0.995 | 0.5s |
| hard | 124 | 96.0% | 90.3% | 96.5% | 0.982 | 0.5s |
| medium | 202 | 98.5% | 99.0% | 99.4% | 0.997 | 0.5s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 303 | 98.0% | 96.7% | 98.4% | 0.992 | 0.5s |
| test | 141 | 97.2% | 97.2% | 99.2% | 0.996 | 0.5s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 95.7% | 100.0% | 95.1% | 20.6% |
| 0.25 | 97.1% | 100.0% | 96.6% | 11.8% |
| 0.30 | 97.3% | 100.0% | 96.9% | 10.3% |
| 0.35 | 97.7% | 100.0% | 97.4% | 7.4% |
| 0.40 | 98.4% | 100.0% | 98.1% | 2.9% |
| 0.50 | 98.4% | 99.5% | 98.7% | 1.5% |
| 0.60 | 97.3% | 98.1% | 98.6% | 1.5% |
| 0.65 | 96.4% | 96.3% | 99.4% | 1.5% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 100.0% [94.7–100.0] (n=68) |
|   └ 올바른 카테고리에 저장 | 100.0% [93.6–100.0] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 100.0% [75.7–100.0] (n=12) |
| 계층 F1 | 1.000 [1.000–1.000] (n=56) |
| 초안이 검색에 안 보임 | 100.0% [93.6–100.0] (n=56) |
| 게시 성공 (버전 +1) | 100.0% [93.6–100.0] (n=56) |
| 초안→게시 카테고리 유지 | 100.0% [93.6–100.0] (n=56) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [93.6–100.0] (n=56) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 100.0% [85.7–100.0] (n=23) |
| 게시 후 probe 검색 top1 답변 | 100.0% [85.7–100.0] (n=23) |
| 지연 p50 (초안+게시+probe) | 0.4s |
| LLM이 경로를 정한 비율 (Jev가 불확실해 리뷰) | 16.1% [8.7–27.8] (n=56) |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| health_sym_head | health | 1 |
| it_net_wifi_drop | it | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| money_bank_xfer_cap | money_bank | 1 |
| null | food_cook | 1 |
| null | it_mobile | 1 |
| null | money_card_lost | 1 |
| null | shop | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_ship_addr | shop | 1 |
| shop_ship_missing | shop | 1 |
| work_exp_trip | travel_air_ticket_mile | 1 |
| work_leave_annual_use | work | 1 |

### 틀린 케이스 (20건 중 20건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it_mobile · abstained · top it_mobile#article 0.17 reference | route false_tree |
| S-HEALTH-015 | 요즘 밤에 잠도 잘 못 자고, 오후만 되면 머리가 조이듯 지끈거려요. 둘 다 어떻게 관리하면 좋을까요? | health_sym_head (+1) | health · top health_sym_head#1 0.58 alternative | route shallow |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_ac#2 0.57 alternative | route shallow |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money_bank · abstained · top money_bank_cert#1 0.53 reference | route shallow, abstained |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | money_tax_yearend · top work_exp_claim#2 0.79 recommended | wrong top1 |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food_cook · abstained · top food_cook#article 0.17 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop#article 0.16 reference | route false_tree |
| S-SHOP-008 | 환불 말고 같은 바지를 한 사이즈 작은 걸로 받고 싶어요 | shop_ret_swap | shop_ret_swap · top shop_ret#article 0.60 alternative | wrong top1 |
| S-SHOP-015 | 배송지 주소를 잘못 넣은 것도 고쳐야 하고, 무통장 입금은 언제까지 해야 하는지도 알려 주세요 | shop_ship_addr (+1) | shop · top shop_ship_addr#1 0.63 alternative | route shallow |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.61 alternative | wrong top1 |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_pay#2 0.84 recommended | route branch |
| S-TRAVEL-009 | 캐나다 여행 가는데 eTA 꼭 받아야 돼요? official site 어디서 apply 하는지도 알려주세요 | travel_prep_visa | travel_prep_visa · abstained · top travel_prep_visa#3 0.54 reference | abstained |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work · top work#article 0.61 alternative | route shallow, wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air#article 0.26 reference | route domain |
| S-XD-009 | 회사 노트북 배터리가 부풀어 올라서 새 걸로 바꾸고 싶은데 어디에 신청해요? | work_it_laptop | work_it_laptop · abstained · top work_it_laptop#1 0.51 reference | abstained |
| S-XD-016 | 미국에 사는 동생한테 돈을 보내려는데 카드로도 보낼 수 있어요? 뭐가 필요해요? | money_bank_xfer_abroad | money_bank_xfer_abroad · abstained · top money_bank_xfer_abroad#1 0.43 reference | abstained |
| S-XD-018 | 배송 완료라는데 택배는 없고, 카드 명세서에는 결제가 두 번 찍혔어요 | shop_ship_missing (+1) | shop · top shop_ship_missing#1 0.69 recommended | route shallow |
| S-XD-019 | My wifi keeps dropping every few minutes and my phone battery also drains really | it_net_wifi_drop (+1) | it · top it_net_wifi_drop#2 0.81 recommended | route shallow |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · abstained · top it_sec_pw#3 0.50 reference | route branch, abstained |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.75 recommended | route false_tree, answered unanswerable |

## jev_llm · deep

### 검색 (445건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 97.3% [95.3–98.5] (n=445) |
|   └ 답할 수 있는 질문 | 97.1% [94.9–98.4] (n=377) |
|   └ 답할 수 없는 질문(보류가 정답) | 98.5% [92.1–99.7] (n=68) |
| 확신 정답률 (top1 정답 + recommended) | 96.0% [93.5–97.6] (n=377) |
| 라우팅 정확도 (정확 일치) | 96.9% [94.8–98.1] (n=445) |
| 계층 F1 (hF1) | 0.981 [0.970–0.992] (n=445) |
| 트리 밖 판정 precision | 100.0% [90.1–100.0] (n=35) |
| 트리 밖 판정 recall | 92.1% [79.2–97.3] (n=38) |
| 트리 밖 판정 F1 | 0.959 |
| 오보류율 (답할 수 있는데 보류) | 1.1% [0.4–2.7] (n=377) |
| 오답변율 (답할 수 없는데 답함) | 1.5% [0.3–7.9] (n=68) |
| Hit@1 | 97.9% [95.9–98.9] (n=377) |
| Hit@3 | 100.0% [99.0–100.0] (n=377) |
| Hit@k (limit) | 100.0% [99.0–100.0] (n=377) |
| MRR@k | 0.989 [0.981–0.997] (n=377) |
| nDCG@k | 0.984 [0.979–0.990] (n=377) |
| Recall@k | 1.000 [1.000–1.000] (n=377) |
| recommended 정밀도 | 0.961 [0.945–0.977] (n=365) |
| 라우팅이 맞았을 때 Hit@1 | 98.6% [96.9–99.4] (n=367) |
| 라우팅이 맞았을 때 MRR | 0.993 [0.986–0.999] (n=367) |
| 지연 p50 / p90 / p95 / 평균 | 0.5s / 2.0s / 3.0s / 0.8s |
| 라우팅 구간 평균 | 0.6s |
| 랭킹 후보 수 평균 | 32.5 |
| Jev 입력 토큰 평균 (과금) | 24775 |
| LLM 토큰 평균 (입력+출력) | 1730 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 2.41 |
| LLM 라우팅 호출 수 평균 | 0.11 |
| LLM이 경로를 정한 비율 (Jev가 불확실해 리뷰) | 11.0% [8.4–14.3] (n=445) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 | d5 | d6 | d7 |
|---|---:|---:|---:|---:|---:|---:|---:|
| 일치율 | 99.4% (n=360) | 97.7% (n=350) | 96.7% (n=331) | 97.4% (n=151) | 100.0% (n=21) | 100.0% (n=10) | 100.0% (n=8) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|
| 2 | 2 | 499 | 3 | 7 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.5s |
| internal | 24 | 95.8% | 100.0% | 95.8% | 0.979 | 0.6s |
| leaf | 353 | 97.2% | 97.2% | 98.0% | 0.990 | 0.5s |
| no_answer | 27 | 100.0% | 96.3% | - | - | 0.5s |
| oos | 38 | 97.4% | 92.1% | - | - | 0.5s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 96.2% | 97.1% | 96.1% | 0.979 | 0.5s |
| single | 340 | 97.6% | 96.8% | 98.5% | 0.993 | 0.5s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 340 | 97.6% | 96.8% | 98.5% | 0.993 | 0.5s |
| coref | 19 | 89.5% | 100.0% | 89.5% | 0.939 | 0.5s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.950 | 0.5s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 0.5s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 385 | 97.1% | 96.6% | 97.7% | 0.988 | 0.5s |
| root | 43 | 97.7% | 97.7% | 100.0% | 1.000 | 0.4s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 35 | 97.1% | 100.0% | 95.8% | 0.979 | 0.6s |
| leaf | 372 | 97.3% | 97.0% | 98.0% | 0.990 | 0.5s |
| none | 38 | 97.4% | 92.1% | - | - | 0.5s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 97.4% | 92.1% | - | - | 0.5s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.7s |
| d2 | 30 | 96.7% | 100.0% | 94.7% | 0.974 | 0.5s |
| d3 | 202 | 96.0% | 96.5% | 96.8% | 0.983 | 0.5s |
| d4 | 144 | 98.6% | 97.2% | 99.3% | 0.996 | 0.5s |
| d5 | 11 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| d6 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| d7 | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 97.6% | 92.7% | - | - | 0.5s |
| 101+ | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.7s |
| 2-5 | 319 | 96.9% | 96.6% | 97.7% | 0.988 | 0.5s |
| 33-100 | 61 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| 6-32 | 20 | 95.0% | 100.0% | 90.9% | 0.955 | 0.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| colloquial | 69 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| condition | 32 | 93.8% | 100.0% | 100.0% | 1.000 | 0.5s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| keyword | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| mixed | 13 | 92.3% | 100.0% | 100.0% | 1.000 | 0.5s |
| multi_intent | 9 | 77.8% | 22.2% | 77.8% | 0.889 | 2.8s |
| negation | 20 | 95.0% | 100.0% | 95.0% | 0.975 | 0.5s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.5s |
| oos_near | 15 | 100.0% | 86.7% | - | - | 2.2s |
| paraphrase | 177 | 97.2% | 97.2% | 97.1% | 0.984 | 0.5s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| vague | 24 | 95.8% | 100.0% | 95.8% | 0.979 | 0.6s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| health | 40 | 97.5% | 97.5% | 97.2% | 0.986 | 0.5s |
| it | 101 | 99.0% | 97.0% | 99.0% | 0.995 | 0.5s |
| life | 42 | 100.0% | 97.6% | 100.0% | 1.000 | 0.5s |
| money | 49 | 93.9% | 98.0% | 95.7% | 0.978 | 0.5s |
| oos | 38 | 97.4% | 92.1% | - | - | 0.5s |
| shop | 41 | 95.1% | 92.7% | 94.7% | 0.974 | 0.5s |
| travel | 44 | 97.7% | 100.0% | 100.0% | 1.000 | 0.5s |
| work | 44 | 93.2% | 95.5% | 95.1% | 0.972 | 0.5s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 98.3% | 99.2% | 99.1% | 0.995 | 0.5s |
| hard | 125 | 95.2% | 91.2% | 95.3% | 0.977 | 0.5s |
| medium | 202 | 98.0% | 99.0% | 98.3% | 0.991 | 0.5s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 304 | 97.4% | 96.7% | 97.7% | 0.988 | 0.5s |
| test | 141 | 97.2% | 97.2% | 98.3% | 0.992 | 0.5s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 95.1% | 100.0% | 94.4% | 20.6% |
| 0.25 | 96.4% | 100.0% | 95.8% | 11.8% |
| 0.30 | 96.4% | 100.0% | 95.8% | 11.8% |
| 0.35 | 97.3% | 100.0% | 96.9% | 5.9% |
| 0.40 | 98.0% | 100.0% | 97.6% | 1.5% |
| 0.50 | 97.5% | 99.2% | 97.9% | 1.5% |
| 0.60 | 97.1% | 97.9% | 98.6% | 1.5% |
| 0.65 | 96.4% | 96.6% | 99.2% | 1.5% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 100.0% [94.7–100.0] (n=68) |
|   └ 올바른 카테고리에 저장 | 100.0% [93.6–100.0] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 100.0% [75.7–100.0] (n=12) |
| 계층 F1 | 1.000 [1.000–1.000] (n=56) |
| 초안이 검색에 안 보임 | 100.0% [93.6–100.0] (n=56) |
| 게시 성공 (버전 +1) | 100.0% [93.6–100.0] (n=56) |
| 초안→게시 카테고리 유지 | 100.0% [93.6–100.0] (n=56) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [93.6–100.0] (n=56) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 100.0% [85.7–100.0] (n=23) |
| 게시 후 probe 검색 top1 답변 | 100.0% [85.7–100.0] (n=23) |
| 지연 p50 (초안+게시+probe) | 0.4s |
| LLM이 경로를 정한 비율 (Jev가 불확실해 리뷰) | 21.4% [12.7–33.8] (n=56) |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| health_sym_head | health | 1 |
| it_mobile_lost | travel_local_metro | 1 |
| it_net_wifi_drop | it | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| money_bank_xfer_cap | money_bank | 1 |
| null | food_cook | 1 |
| null | it_mobile | 1 |
| null | money_card_lost | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_ship_addr | shop | 1 |
| shop_ship_missing | shop | 1 |
| work_exp_trip | travel_air_ticket_mile | 1 |
| work_leave_annual_use | work | 1 |

### 틀린 케이스 (22건 중 22건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it_mobile · abstained · top it_mobile#article 0.19 reference | route false_tree |
| S-HEALTH-015 | 요즘 밤에 잠도 잘 못 자고, 오후만 되면 머리가 조이듯 지끈거려요. 둘 다 어떻게 관리하면 좋을까요? | health_sym_head (+1) | health · top health_sym_cold#2 0.63 alternative | route shallow, wrong top1 |
| S-IT-017 | 지하철에 아이폰을 두고 내렸어요. 지금 어디 있는지 확인할 방법이 있을까요? | it_mobile_lost | travel_local_metro · top it_mobile_lost#1 0.92 recommended | route domain |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.56 alternative | route shallow |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money_bank · top money_bank#article 0.59 alternative | route shallow, wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | money_tax_yearend · top work_exp_claim#2 0.81 recommended | wrong top1 |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food_cook · abstained · top food_cook#article 0.15 reference | route false_tree |
| S-SHOP-008 | 환불 말고 같은 바지를 한 사이즈 작은 걸로 받고 싶어요 | shop_ret_swap | shop_ret_swap · top shop_ret#article 0.55 alternative | wrong top1 |
| S-SHOP-015 | 배송지 주소를 잘못 넣은 것도 고쳐야 하고, 무통장 입금은 언제까지 해야 하는지도 알려 주세요 | shop_ship_addr (+1) | shop · top shop_order_pay#3 0.61 alternative | route shallow |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.64 alternative | wrong top1 |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_pay#2 0.80 recommended | route branch |
| S-TRAVEL-009 | 캐나다 여행 가는데 eTA 꼭 받아야 돼요? official site 어디서 apply 하는지도 알려주세요 | travel_prep_visa | travel_prep_visa · abstained · top travel_prep_visa#3 0.53 reference | abstained |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work · top work_pay_slip#2 0.67 recommended | route shallow |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air#article 0.32 reference | route domain |
| S-WORK-022 | 그거 언제까지 올려야 돼요? | work_exp_card | work_exp_card · top work_exp_claim#2 0.57 alternative | wrong top1 |
| S-XD-009 | 회사 노트북 배터리가 부풀어 올라서 새 걸로 바꾸고 싶은데 어디에 신청해요? | work_it_laptop | work_it_laptop · abstained · top work_it_laptop#1 0.48 reference | abstained |
| S-XD-016 | 미국에 사는 동생한테 돈을 보내려는데 카드로도 보낼 수 있어요? 뭐가 필요해요? | money_bank_xfer_abroad | money_bank_xfer_abroad · abstained · top money_bank_xfer_abroad#1 0.42 reference | abstained |
| S-XD-018 | 배송 완료라는데 택배는 없고, 카드 명세서에는 결제가 두 번 찍혔어요 | shop_ship_missing (+1) | shop · top shop_ship_missing#1 0.72 recommended | route shallow |
| S-XD-019 | My wifi keeps dropping every few minutes and my phone battery also drains really | it_net_wifi_drop (+1) | it · top it_net_wifi_drop#2 0.85 recommended | route shallow |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · abstained · top it_sec_pw#3 0.44 reference | route branch, abstained |
| S-XD-031 | 분실 신고는 어떻게 해요? | work_exp_card | work_exp_card · top work_exp_card#3 1.06 recommended | wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.79 recommended | route false_tree, answered unanswerable |

