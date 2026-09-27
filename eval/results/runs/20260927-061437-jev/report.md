# 평가 결과 — 20260927-061437-jev

- 실행: 2026-09-27T06:14:37Z → 2026-09-27T06:15:50Z (UTC) · git `3c61418`
- 데이터셋: `kb` v1.0.0 · kb `489e5fca3c7c` · cases `b2a362fcd60d`
- 설정: routers ["jev"] · variants ["standard"] · suites ["all"] · split all · repeat 1 · concurrency 4 · beam_width "case/default" · limit "case/default"
- 모델(jev): jev `jev-latest`

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev | standard | 444 | 97.1% | 93.0% | 0.955 | 97.3% | 0.975 | 0.964 | 0.813 | 4.4% | 0.5% | 0.5s / 1.0s | 2.2 | 0 | 94.1% | 100.0% |

## jev · standard

### 검색 (444건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 97.1% [95.1–98.3] (n=444) |
|   └ 답할 수 있는 질문 | 97.3% [95.2–98.5] (n=376) |
|   └ 답할 수 없는 질문(보류가 정답) | 95.6% [87.8–98.5] (n=68) |
| 확신 정답률 (top1 정답 + recommended) | 91.5% [88.2–93.9] (n=376) |
| 라우팅 정확도 (정확 일치) | 93.0% [90.3–95.0] (n=444) |
| 계층 F1 (hF1) | 0.955 [0.937–0.973] (n=444) |
| 트리 밖 판정 precision | 100.0% [87.1–100.0] (n=26) |
| 트리 밖 판정 recall | 68.4% [52.5–80.9] (n=38) |
| 트리 밖 판정 F1 | 0.813 |
| 오보류율 (답할 수 있는데 보류) | 0.5% [0.1–1.9] (n=376) |
| 오답변율 (답할 수 없는데 답함) | 4.4% [1.5–12.2] (n=68) |
| Hit@1 | 97.3% [95.2–98.5] (n=376) |
| Hit@3 | 97.6% [95.5–98.7] (n=376) |
| Hit@k (limit) | 97.6% [95.5–98.7] (n=376) |
| MRR@k | 0.975 [0.959–0.990] (n=376) |
| nDCG@k | 0.964 [0.947–0.980] (n=376) |
| Recall@k | 0.965 [0.948–0.982] (n=376) |
| recommended 정밀도 | 0.972 [0.956–0.987] (n=350) |
| 라우팅이 맞았을 때 Hit@1 | 99.7% [98.4–100.0] (n=361) |
| 라우팅이 맞았을 때 MRR | 0.999 [0.996–1.000] (n=361) |
| 지연 p50 / p90 / p95 / 평균 | 0.5s / 0.8s / 1.0s / 0.5s |
| 라우팅 구간 평균 | 0.3s |
| 랭킹 후보 수 평균 | 15.4 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 2.22 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 98.9% (n=359) | 98.0% (n=349) | 93.6% (n=330) | 96.3% (n=82) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|
| 6 | 4 | 477 | 15 | 10 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.3s |
| internal | 23 | 100.0% | 100.0% | 100.0% | 1.000 | 0.7s |
| leaf | 353 | 97.2% | 95.8% | 97.2% | 0.973 | 0.5s |
| no_answer | 27 | 100.0% | 85.2% | - | - | 0.5s |
| oos | 38 | 92.1% | 68.4% | - | - | 0.3s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 94.3% | 93.3% | 94.2% | 0.942 | 0.5s |
| single | 339 | 97.9% | 92.9% | 98.5% | 0.987 | 0.5s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 339 | 97.9% | 92.9% | 98.5% | 0.987 | 0.5s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 0.5s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 0.5s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| long | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 0.5s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 0.5s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 384 | 96.9% | 92.2% | 97.1% | 0.972 | 0.5s |
| root | 43 | 97.7% | 97.7% | 100.0% | 1.000 | 0.4s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 100.0% | 97.1% | 100.0% | 1.000 | 0.5s |
| leaf | 372 | 97.3% | 95.2% | 97.2% | 0.973 | 0.5s |
| none | 38 | 92.1% | 68.4% | - | - | 0.3s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 92.1% | 68.4% | - | - | 0.3s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| d2 | 30 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| d3 | 278 | 96.8% | 93.9% | 96.6% | 0.968 | 0.5s |
| d4 | 88 | 98.9% | 97.7% | 98.8% | 0.988 | 0.5s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 92.7% | 70.7% | - | - | 0.3s |
| 101+ | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 2.1s |
| 2-5 | 319 | 97.8% | 95.6% | 97.7% | 0.979 | 0.5s |
| 33-100 | 61 | 95.1% | 93.4% | 94.9% | 0.949 | 0.9s |
| 6-32 | 19 | 100.0% | 94.7% | 100.0% | 1.000 | 0.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| colloquial | 69 | 97.1% | 98.6% | 96.9% | 0.977 | 0.5s |
| condition | 32 | 93.8% | 90.6% | 93.5% | 0.935 | 0.5s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| keyword | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 0.5s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| multi_intent | 9 | 100.0% | 77.8% | 100.0% | 1.000 | 0.5s |
| negation | 20 | 100.0% | 95.0% | 100.0% | 1.000 | 0.5s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.3s |
| oos_near | 15 | 86.7% | 26.7% | - | - | 0.5s |
| paraphrase | 177 | 96.6% | 93.2% | 96.4% | 0.964 | 0.5s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| vague | 23 | 100.0% | 100.0% | 100.0% | 1.000 | 0.7s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 95.7% | 93.5% | 95.3% | 0.953 | 0.5s |
| health | 40 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| it | 101 | 98.0% | 98.0% | 97.9% | 0.979 | 0.5s |
| life | 41 | 100.0% | 95.1% | 100.0% | 1.000 | 0.5s |
| money | 49 | 98.0% | 93.9% | 97.8% | 0.978 | 0.5s |
| oos | 38 | 92.1% | 68.4% | - | - | 0.3s |
| shop | 41 | 92.7% | 90.2% | 92.1% | 0.921 | 0.5s |
| travel | 44 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| work | 44 | 95.5% | 88.6% | 95.1% | 0.963 | 0.5s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 99.2% | 99.2% | 99.1% | 0.995 | 0.5s |
| hard | 124 | 92.7% | 82.3% | 91.8% | 0.918 | 0.5s |
| medium | 202 | 98.5% | 96.0% | 98.9% | 0.989 | 0.5s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 303 | 97.4% | 92.4% | 97.7% | 0.977 | 0.5s |
| test | 141 | 96.5% | 94.3% | 96.6% | 0.971 | 0.5s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 95.0% | 99.7% | 94.6% | 17.6% |
| 0.25 | 96.4% | 99.5% | 96.3% | 8.8% |
| 0.30 | 97.1% | 99.5% | 97.1% | 4.4% |
| 0.35 | 97.1% | 99.2% | 97.3% | 2.9% |
| 0.40 | 97.1% | 99.2% | 97.3% | 2.9% |
| 0.50 | 96.6% | 98.4% | 97.6% | 2.9% |
| 0.60 | 94.1% | 94.9% | 98.1% | 2.9% |
| 0.65 | 92.6% | 92.8% | 98.3% | 1.5% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 94.1% [85.8–97.7] (n=68) |
|   └ 올바른 카테고리에 저장 | 98.2% [90.6–99.7] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 75.0% [46.8–91.1] (n=12) |
| 계층 F1 | 0.996 [0.989–1.000] (n=56) |
| 초안이 검색에 안 보임 | 100.0% [93.6–100.0] (n=56) |
| 게시 성공 (버전 +1) | 100.0% [93.6–100.0] (n=56) |
| 초안→게시 카테고리 유지 | 98.2% [90.6–99.7] (n=56) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [93.6–100.0] (n=56) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 100.0% [85.7–100.0] (n=23) |
| 게시 후 probe 검색 top1 답변 | 100.0% [85.7–100.0] (n=23) |
| 지연 p50 (초안+게시+probe) | 0.6s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 2 |
| null | it | 2 |
| food_store_item | food_store | 1 |
| it_office_excel_func | work | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| life_appl_kitchen | life_appl | 1 |
| money_bank_xfer_cap | money_bank | 1 |
| money_card_lost | money_card | 1 |
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | health | 1 |
| null | health_sym | 1 |
| null | it_mobile | 1 |
| null | money | 1 |
| null | money_card_lost | 1 |
| null | money_save | 1 |
| null | shop | 1 |
| null | shop_member | 1 |
| null | travel_car | 1 |

### 틀린 케이스 (36건 중 36건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-SHOP-001 | 예약 판매 상품은 언제 출고되나요? | shop_ship_track | shop_ship | shallow |
| I-SHOP-007 | 호텔 예약을 취소하면 수수료가 얼마나 나오나요? | null | shop_order_cancel | false_tree |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_ret | false_tree |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.82 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.86 recommended | route branch, wrong top1 |
| S-FOOD-517 | 쌀통에 둔 쌀이 자꾸 눅눅해지고 냄새가 나요. 쌀은 어디 두는 게 맞아요? | food_store_item | food_store · top food_store_item#rice 0.91 recommended | route shallow |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it_mobile · abstained · top it_mobile#article 0.16 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it · abstained · top it#article 0.25 reference | route false_tree |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · abstained · top work_pay_sev#2 0.21 reference | route domain, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_ac#2 0.64 alternative | route shallow |
| S-LIFE-024 | 식기세척기 전용 세제가 떨어졌는데 일반 주방세제 넣고 돌려도 돼요? | life_appl_kitchen · no answer | life_appl · abstained · top life_appl_wash_wm_drum_care_smell#2 0.22 reference | route shallow |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost | money_card · top money_card_lost#4 0.95 recommended | route shallow |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money_bank · top money_bank_xfer_cap#1 0.62 alternative | route shallow |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.86 recommended | route domain, wrong top1 |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.29 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money_save · top money_save#article 0.63 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health#article 0.17 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.19 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money · abstained · top money#article 0.21 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | shop_member · abstained · top shop_member_leave#1 0.23 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym · top health_sym#article 0.31 reference | route false_tree, answered unanswerable |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.21 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop#article 0.15 reference | route false_tree |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card | money_card_install · top money_card_install#3 0.41 alternative | route domain, wrong top1 |
| S-SHOP-020 | 해외 배송 상품은 통관 때문에 며칠 더 걸리나요? | shop_ship_track · no answer | shop_ship · abstained · top shop_ship_track#3 0.23 reference | route shallow |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.56 alternative | route branch, wrong top1 |
| S-SHOP-031 | 그 위 등급이 되면 매달 쿠폰은 몇 장씩 주는 거예요? | shop_member_grade | shop_member_coupon · abstained · top shop_member_coupon#2 0.08 reference | route branch, abstained |
| S-WORK-001 | 오후 반차 쓰면 몇 시에 퇴근하는 거예요? | work_leave_annual_use | work_leave · top work_leave_annual_use#2 0.89 recommended | route shallow |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.74 recommended | wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#3 0.07 reference | route domain |
| S-WORK-021 | 경조사 생기면 회사에서 화환이나 조화도 보내주나요? | work_leave_event · no answer | work · abstained · top work_leave_event#2 0.27 reference | route shallow |
| S-XD-009 | 회사 노트북 배터리가 부풀어 올라서 새 걸로 바꾸고 싶은데 어디에 신청해요? | work_it_laptop | work_it · top work_it_laptop#1 0.79 recommended | route shallow |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.58 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.81 recommended | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.82 recommended | route false_tree, answered unanswerable |

