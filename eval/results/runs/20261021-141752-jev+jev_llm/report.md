# 평가 결과 — 20261021-141752-jev+jev_llm

- 실행: 2026-10-21T14:17:52Z → 2026-10-21T14:28:07Z (UTC) · git `0d26b76` (커밋 안 된 변경 있음)
- 데이터셋: `kb` v1.0.0 · kb `489e5fca3c7c` · cases `b2a362fcd60d`
- 설정: routers ["jev","jev_llm"] · variants ["standard"] · suites ["all"] · split all · repeat 1 · concurrency 4 · beam_width "case/default" · limit "case/default"
- 모델(jev): jev `jev-latest`
- 모델(jev_llm): jev `jev-latest` · LLM `meta/muse-spark-1.3-contributor`
- 메모: baseline before redesign (instrumented, 2026-10-21)

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev | standard | 444 | 96.6% | 93.7% | 0.960 | 96.8% | 0.972 | 0.963 | 0.836 | 2.9% | 0.8% | 0.4s / 0.7s | 2.4 | 0 | 95.6% | 100.0% |
| jev_llm | standard | 444 | 97.7% | 97.1% | 0.985 | 97.6% | 0.980 | 0.972 | 0.974 | 0.0% | 0.3% | 3.8s / 7.5s | 1.2 | 0 | 100.0% | 100.0% |

## jev · standard

### 검색 (444건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 96.6% [94.5–97.9] (n=444) |
|   └ 답할 수 있는 질문 | 96.5% [94.2–98.0] (n=376) |
|   └ 답할 수 없는 질문(보류가 정답) | 97.1% [89.9–99.2] (n=68) |
| 확신 정답률 (top1 정답 + recommended) | 92.3% [89.1–94.6] (n=376) |
| 라우팅 정확도 (정확 일치) | 93.7% [91.0–95.6] (n=444) |
| 계층 F1 (hF1) | 0.960 [0.943–0.977] (n=444) |
| 트리 밖 판정 precision | 96.6% [82.8–99.4] (n=29) |
| 트리 밖 판정 recall | 73.7% [58.0–85.0] (n=38) |
| 트리 밖 판정 F1 | 0.836 |
| 오보류율 (답할 수 있는데 보류) | 0.8% [0.3–2.3] (n=376) |
| 오답변율 (답할 수 없는데 답함) | 2.9% [0.8–10.1] (n=68) |
| Hit@1 | 96.8% [94.5–98.2] (n=376) |
| Hit@3 | 97.6% [95.5–98.7] (n=376) |
| Hit@k (limit) | 97.6% [95.5–98.7] (n=376) |
| MRR@k | 0.972 [0.956–0.988] (n=376) |
| nDCG@k | 0.963 [0.946–0.979] (n=376) |
| Recall@k | 0.965 [0.948–0.982] (n=376) |
| recommended 정밀도 | 0.969 [0.953–0.985] (n=354) |
| 라우팅이 맞았을 때 Hit@1 | 99.2% [97.6–99.7] (n=362) |
| 라우팅이 맞았을 때 MRR | 0.996 [0.991–1.000] (n=362) |
| 지연 p50 / p90 / p95 / 평균 | 0.4s / 0.6s / 0.7s / 0.5s |
| 라우팅 구간 평균 | 0.3s |
| 랭킹 후보 수 평균 | 15.3 |
| Jev 입력 토큰 평균 (과금) | 18619 |
| LLM 토큰 평균 (입력+출력) | 0 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 2.37 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 98.9% (n=359) | 98.0% (n=349) | 93.9% (n=330) | 96.3% (n=82) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 6 | 3 | 481 | 1 | 12 | 9 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.3s |
| internal | 23 | 91.3% | 100.0% | 91.3% | 0.957 | 0.5s |
| leaf | 353 | 96.9% | 96.0% | 97.2% | 0.973 | 0.4s |
| no_answer | 27 | 100.0% | 85.2% | - | - | 0.4s |
| oos | 38 | 94.7% | 73.7% | - | - | 0.3s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 94.3% | 93.3% | 94.2% | 0.942 | 0.4s |
| single | 339 | 97.3% | 93.8% | 97.8% | 0.984 | 0.4s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 339 | 97.3% | 93.8% | 97.8% | 0.984 | 0.4s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 0.4s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 0.5s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| long | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 0.4s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 0.4s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 384 | 96.4% | 93.0% | 96.5% | 0.969 | 0.4s |
| root | 43 | 97.7% | 97.7% | 100.0% | 1.000 | 0.2s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 0.3s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 94.1% | 97.1% | 91.3% | 0.957 | 0.5s |
| leaf | 372 | 97.0% | 95.4% | 97.2% | 0.973 | 0.4s |
| none | 38 | 94.7% | 73.7% | - | - | 0.3s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 94.7% | 73.7% | - | - | 0.3s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| d2 | 30 | 93.3% | 100.0% | 89.5% | 0.947 | 0.4s |
| d3 | 278 | 96.4% | 94.2% | 96.6% | 0.968 | 0.4s |
| d4 | 88 | 98.9% | 97.7% | 98.8% | 0.988 | 0.5s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 95.1% | 75.6% | - | - | 0.3s |
| 101+ | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.7s |
| 2-5 | 319 | 97.5% | 95.9% | 97.7% | 0.979 | 0.4s |
| 33-100 | 61 | 95.1% | 93.4% | 94.9% | 0.949 | 0.5s |
| 6-32 | 19 | 89.5% | 94.7% | 80.0% | 0.900 | 0.4s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| colloquial | 69 | 97.1% | 98.6% | 96.9% | 0.977 | 0.4s |
| condition | 32 | 90.6% | 90.6% | 93.5% | 0.935 | 0.4s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| keyword | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 0.5s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| multi_intent | 9 | 100.0% | 77.8% | 100.0% | 1.000 | 0.6s |
| negation | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.3s |
| oos_near | 15 | 93.3% | 40.0% | - | - | 0.6s |
| paraphrase | 177 | 96.6% | 93.2% | 96.4% | 0.964 | 0.4s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| vague | 23 | 91.3% | 100.0% | 91.3% | 0.957 | 0.5s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 95.7% | 93.5% | 95.3% | 0.953 | 0.5s |
| health | 40 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| it | 101 | 98.0% | 98.0% | 97.9% | 0.979 | 0.5s |
| life | 41 | 100.0% | 95.1% | 100.0% | 1.000 | 0.4s |
| money | 49 | 93.9% | 95.9% | 95.7% | 0.967 | 0.4s |
| oos | 38 | 94.7% | 73.7% | - | - | 0.3s |
| shop | 41 | 90.2% | 90.2% | 89.5% | 0.908 | 0.5s |
| travel | 44 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| work | 44 | 95.5% | 88.6% | 95.1% | 0.963 | 0.4s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 98.3% | 99.2% | 98.2% | 0.991 | 0.4s |
| hard | 124 | 91.9% | 83.9% | 91.8% | 0.918 | 0.5s |
| medium | 202 | 98.5% | 96.5% | 98.3% | 0.986 | 0.4s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 303 | 97.4% | 93.4% | 97.3% | 0.975 | 0.4s |
| test | 141 | 95.0% | 94.3% | 95.8% | 0.966 | 0.4s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 94.8% | 99.5% | 94.5% | 16.2% |
| 0.25 | 95.9% | 99.5% | 95.8% | 8.8% |
| 0.30 | 96.6% | 99.5% | 96.6% | 4.4% |
| 0.35 | 96.6% | 99.2% | 96.8% | 2.9% |
| 0.40 | 96.6% | 99.2% | 96.8% | 2.9% |
| 0.50 | 96.2% | 98.1% | 97.3% | 2.9% |
| 0.60 | 94.4% | 95.5% | 97.8% | 1.5% |
| 0.65 | 93.2% | 93.9% | 98.0% | 1.5% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 95.6% [87.8–98.5] (n=68) |
|   └ 올바른 카테고리에 저장 | 98.2% [90.6–99.7] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 83.3% [55.2–95.3] (n=12) |
| 계층 F1 | 0.996 [0.989–1.000] (n=56) |
| 초안이 검색에 안 보임 | 100.0% [93.6–100.0] (n=56) |
| 게시 성공 (버전 +1) | 100.0% [93.6–100.0] (n=56) |
| 초안→게시 카테고리 유지 | 100.0% [93.6–100.0] (n=56) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [93.6–100.0] (n=56) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 100.0% [85.7–100.0] (n=23) |
| 게시 후 probe 검색 top1 답변 | 100.0% [85.7–100.0] (n=23) |
| 지연 p50 (초안+게시+probe) | 0.3s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 2 |
| null | it | 2 |
| null | money | 2 |
| food_store_item | food_store | 1 |
| it_office_excel_func | null | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| life_appl_kitchen | life_appl | 1 |
| money_bank_xfer_cap | money_bank | 1 |
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | health_sym | 1 |
| null | it_mobile | 1 |
| null | money_card_lost | 1 |
| null | shop | 1 |
| null | travel_car | 1 |
| shop_member_grade | shop_member_coupon | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_ret_refund_card | money_card_install | 1 |
| shop_ship_track | shop_ship | 1 |

### 틀린 케이스 (35건 중 35건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-SHOP-001 | 예약 판매 상품은 언제 출고되나요? | shop_ship_track | shop_ship | shallow |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_ret | false_tree |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.81 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.90 recommended | route branch, wrong top1 |
| S-FOOD-517 | 쌀통에 둔 쌀이 자꾸 눅눅해지고 냄새가 나요. 쌀은 어디 두는 게 맞아요? | food_store_item | food_store · top food_store_item#rice 0.91 recommended | route shallow |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it_mobile · abstained · top it_mobile#article 0.15 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it · abstained · top it#article 0.25 reference | route false_tree |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | null · abstained · top no items | route false_none, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.65 recommended | route shallow |
| S-LIFE-024 | 식기세척기 전용 세제가 떨어졌는데 일반 주방세제 넣고 돌려도 돼요? | life_appl_kitchen · no answer | life_appl · abstained · top life_appl_wash_wm_drum_care_smell#2 0.27 reference | route shallow |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money_bank · top money_bank_xfer_cap#1 0.67 recommended | route shallow |
| S-MONEY-021 | 제 명의 신용카드 관련해서 이것저것 궁금한 게 있는데요 | money_card | money_card · top money_card_lost#4 0.75 recommended | wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.86 recommended | route domain, wrong top1 |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.27 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · top money_save#article 0.59 alternative | route false_tree, answered unanswerable |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.19 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money · abstained · top money#article 0.21 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym · abstained · top health_sym#article 0.31 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.22 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop#article 0.15 reference | route false_tree |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card | money_card_install · top money_card_install#3 0.43 alternative | route domain, wrong top1 |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.48 alternative | wrong top1 |
| S-SHOP-020 | 해외 배송 상품은 통관 때문에 며칠 더 걸리나요? | shop_ship_track · no answer | shop_ship · abstained · top shop_ship_track#3 0.22 reference | route shallow |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.53 alternative | route branch, wrong top1 |
| S-SHOP-031 | 그 위 등급이 되면 매달 쿠폰은 몇 장씩 주는 거예요? | shop_member_grade | shop_member_coupon · abstained · top shop_member_coupon#2 0.07 reference | route branch, abstained |
| S-WORK-001 | 오후 반차 쓰면 몇 시에 퇴근하는 거예요? | work_leave_annual_use | work_leave · top work_leave_annual_use#2 0.86 recommended | route shallow |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.76 recommended | wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#3 0.07 reference | route domain |
| S-WORK-021 | 경조사 생기면 회사에서 화환이나 조화도 보내주나요? | work_leave_event · no answer | work · abstained · top work_leave_event#2 0.28 reference | route shallow |
| S-XD-009 | 회사 노트북 배터리가 부풀어 올라서 새 걸로 바꾸고 싶은데 어디에 신청해요? | work_it_laptop | work_it · top work_it_laptop#1 0.77 recommended | route shallow |
| S-XD-016 | 미국에 사는 동생한테 돈을 보내려는데 카드로도 보낼 수 있어요? 뭐가 필요해요? | money_bank_xfer_abroad | money_bank_xfer_abroad · abstained · top money_bank_xfer_abroad#1 0.30 reference | abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.60 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.84 recommended | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.82 recommended | route false_tree, answered unanswerable |

## jev_llm · standard

### 검색 (444건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 97.7% [95.9–98.8] (n=444) |
|   └ 답할 수 있는 질문 | 97.3% [95.2–98.5] (n=376) |
|   └ 답할 수 없는 질문(보류가 정답) | 100.0% [94.7–100.0] (n=68) |
| 확신 정답률 (top1 정답 + recommended) | 92.0% [88.8–94.4] (n=376) |
| 라우팅 정확도 (정확 일치) | 97.1% [95.1–98.3] (n=444) |
| 계층 F1 (hF1) | 0.985 [0.975–0.994] (n=444) |
| 트리 밖 판정 precision | 97.4% [86.5–99.5] (n=38) |
| 트리 밖 판정 recall | 97.4% [86.5–99.5] (n=38) |
| 트리 밖 판정 F1 | 0.974 |
| 오보류율 (답할 수 있는데 보류) | 0.3% [0.0–1.5] (n=376) |
| 오답변율 (답할 수 없는데 답함) | 0.0% [0.0–5.3] (n=68) |
| Hit@1 | 97.6% [95.5–98.7] (n=376) |
| Hit@3 | 98.4% [96.6–99.3] (n=376) |
| Hit@k (limit) | 98.4% [96.6–99.3] (n=376) |
| MRR@k | 0.980 [0.967–0.993] (n=376) |
| nDCG@k | 0.972 [0.958–0.985] (n=376) |
| Recall@k | 0.976 [0.961–0.990] (n=376) |
| recommended 정밀도 | 0.974 [0.959–0.988] (n=351) |
| 라우팅이 맞았을 때 Hit@1 | 99.5% [98.0–99.9] (n=366) |
| 라우팅이 맞았을 때 MRR | 0.997 [0.993–1.000] (n=366) |
| 지연 p50 / p90 / p95 / 평균 | 3.8s / 5.8s / 7.5s / 4.1s |
| 라우팅 구간 평균 | 3.9s |
| 랭킹 후보 수 평균 | 14.0 |
| Jev 입력 토큰 평균 (과금) | 3544 |
| LLM 토큰 평균 (입력+출력) | 11677 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 1.18 |
| LLM 라우팅 호출 수 평균 | 0.98 |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–0.9] (n=444) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 99.2% (n=359) | 98.6% (n=349) | 96.7% (n=330) | 98.8% (n=82) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 5 | 2 | 499 | 1 | 1 | 4 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 3.4s |
| internal | 23 | 95.7% | 100.0% | 95.7% | 0.978 | 4.2s |
| leaf | 353 | 97.5% | 97.2% | 97.7% | 0.980 | 3.7s |
| no_answer | 27 | 100.0% | 92.6% | - | - | 4.9s |
| oos | 38 | 100.0% | 97.4% | - | - | 4.1s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 96.2% | 95.2% | 96.1% | 0.961 | 3.8s |
| single | 339 | 98.2% | 97.6% | 98.2% | 0.987 | 3.8s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 339 | 98.2% | 97.6% | 98.2% | 0.987 | 3.8s |
| coref | 19 | 89.5% | 89.5% | 89.5% | 0.895 | 3.8s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 3.6s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 3.7s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 4.0s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 4.1s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 3.7s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 4.2s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 3.9s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 4.6s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 384 | 97.4% | 96.6% | 97.4% | 0.978 | 3.7s |
| root | 43 | 100.0% | 100.0% | 100.0% | 1.000 | 4.1s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 4.7s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 97.1% | 100.0% | 95.7% | 0.978 | 4.5s |
| leaf | 372 | 97.6% | 96.8% | 97.7% | 0.980 | 3.7s |
| none | 38 | 100.0% | 97.4% | - | - | 4.1s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 100.0% | 97.4% | - | - | 4.1s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 4.2s |
| d2 | 30 | 96.7% | 100.0% | 94.7% | 0.974 | 4.6s |
| d3 | 278 | 96.8% | 96.0% | 96.9% | 0.973 | 3.7s |
| d4 | 88 | 100.0% | 98.9% | 100.0% | 1.000 | 3.6s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 100.0% | 97.6% | - | - | 3.9s |
| 101+ | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 4.5s |
| 2-5 | 319 | 97.5% | 96.9% | 97.7% | 0.982 | 3.7s |
| 33-100 | 61 | 96.7% | 96.7% | 96.6% | 0.966 | 3.9s |
| 6-32 | 19 | 100.0% | 100.0% | 100.0% | 1.000 | 4.7s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 3.1s |
| colloquial | 69 | 98.6% | 100.0% | 98.5% | 0.992 | 3.7s |
| condition | 32 | 93.8% | 96.9% | 96.8% | 0.968 | 4.0s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 3.2s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 5.3s |
| keyword | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 3.6s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 3.3s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 3.4s |
| multi_intent | 9 | 88.9% | 55.6% | 88.9% | 0.944 | 6.8s |
| negation | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 3.8s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 3.0s |
| oos_near | 15 | 100.0% | 93.3% | - | - | 4.4s |
| paraphrase | 177 | 97.7% | 96.6% | 97.1% | 0.971 | 4.0s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 3.3s |
| vague | 23 | 95.7% | 100.0% | 95.7% | 0.978 | 4.2s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 93.5% | 95.7% | 93.0% | 0.942 | 3.9s |
| health | 40 | 100.0% | 97.5% | 100.0% | 1.000 | 3.9s |
| it | 101 | 99.0% | 99.0% | 99.0% | 0.990 | 3.6s |
| life | 41 | 100.0% | 97.6% | 100.0% | 1.000 | 3.9s |
| money | 49 | 98.0% | 98.0% | 100.0% | 1.000 | 3.7s |
| oos | 38 | 100.0% | 97.4% | - | - | 4.1s |
| shop | 41 | 95.1% | 95.1% | 94.7% | 0.947 | 4.2s |
| travel | 44 | 100.0% | 97.7% | 100.0% | 1.000 | 3.5s |
| work | 44 | 93.2% | 93.2% | 92.7% | 0.951 | 3.9s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 99.2% | 100.0% | 99.1% | 0.995 | 3.5s |
| hard | 124 | 94.4% | 91.1% | 92.9% | 0.935 | 4.3s |
| medium | 202 | 99.0% | 99.0% | 98.9% | 0.992 | 3.8s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 303 | 98.0% | 97.4% | 97.7% | 0.981 | 3.7s |
| test | 141 | 97.2% | 96.5% | 97.5% | 0.979 | 3.8s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 97.3% | 100.0% | 96.8% | 4.4% |
| 0.25 | 98.0% | 100.0% | 97.6% | 0.0% |
| 0.30 | 98.0% | 100.0% | 97.6% | 0.0% |
| 0.35 | 97.7% | 99.7% | 97.6% | 0.0% |
| 0.40 | 97.7% | 99.5% | 97.9% | 0.0% |
| 0.50 | 97.7% | 99.2% | 98.1% | 0.0% |
| 0.60 | 95.0% | 96.0% | 98.1% | 0.0% |
| 0.65 | 93.2% | 93.4% | 98.6% | 0.0% |

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
| 지연 p50 (초안+게시+probe) | 4.3s |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–6.4] (n=56) |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 2 |
| health_sym_head | health | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| money_bank_xfer_cap | money_bank | 1 |
| null | food | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_ret_refund_card | money_card_install | 1 |
| travel_local_train | null | 1 |
| work_exp_trip | travel_air_ticket_mile | 1 |
| work_exp_trip | work_exp_claim | 1 |
| work_leave_annual_use | work | 1 |

### 틀린 케이스 (16건 중 16건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.45 alternative | wrong top1 |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.80 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.90 recommended | route branch, wrong top1 |
| S-HEALTH-015 | 요즘 밤에 잠도 잘 못 자고, 오후만 되면 머리가 조이듯 지끈거려요. 둘 다 어떻게 관리하면 좋을까요? | health_sym_head (+1) | health · top health_sym_head#1 0.64 alternative | route shallow |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_ac#2 0.65 recommended | route shallow |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money_bank · top money_bank_xfer_cap#1 0.62 alternative | route shallow |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.22 reference | route false_tree |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card | money_card_install · top money_card_install#3 0.37 reference | route domain, wrong top1 |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.60 alternative | route branch, wrong top1 |
| S-TRAVEL-019 | KTX에 강아지 데리고 타려면 케이지가 꼭 있어야 하나요? | travel_local_train · no answer | null · abstained · top no items | route false_none |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.74 recommended | wrong top1 |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work · top work#article 0.74 recommended | route shallow, wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#2 0.08 reference | route domain |
| S-XD-016 | 미국에 사는 동생한테 돈을 보내려는데 카드로도 보낼 수 있어요? 뭐가 필요해요? | money_bank_xfer_abroad | money_bank_xfer_abroad · abstained · top money_bank_xfer_abroad#1 0.32 reference | abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.61 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.81 recommended | route branch, wrong top1 |

