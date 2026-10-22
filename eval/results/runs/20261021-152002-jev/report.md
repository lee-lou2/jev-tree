# 평가 결과 — 20261021-152002-jev

- 실행: 2026-10-21T15:20:02Z → 2026-10-21T15:24:51Z (UTC) · git `0d26b76` (커밋 안 된 변경 있음)
- 데이터셋: `kb` v1.0.0 · kb `489e5fca3c7c` · cases `b2a362fcd60d`
- 설정: routers ["jev"] · variants ["deep","standard","shallow","flat","sparse"] · suites ["all"] · split all · repeat 1 · concurrency 4 · beam_width "case/default" · limit "case/default"
- 모델(jev): jev `jev-latest`
- 메모: 0.10.0: coarse map + BM25 recall, rescue (>=0.75)

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev | deep | 445 | 97.8% | 93.3% | 0.957 | 98.1% | 0.983 | 0.974 | 0.818 | 2.9% | 0.5% | 0.5s / 1.0s | 2.4 | 0 | 92.6% | 100.0% |
| jev | standard | 444 | 98.0% | 93.7% | 0.960 | 98.1% | 0.984 | 0.977 | 0.848 | 1.5% | 0.3% | 0.4s / 0.7s | 2.4 | 0 | 94.1% | 100.0% |
| jev | shallow | 400 | 97.8% | 94.8% | 0.957 | 98.0% | 0.981 | 0.969 | 0.825 | 1.8% | 0.6% | 0.4s / 0.7s | 2.3 | 0 | 94.9% | 100.0% |
| jev | flat | 369 | 97.6% | 96.7% | 0.967 | 97.5% | 0.977 | 0.966 | 0.962 | 0.0% | 0.6% | 0.4s / 0.6s | 2.2 | 0 | 100.0% | 100.0% |
| jev | sparse | 444 | 90.8% | 93.7% | 0.960 | 96.7% | 0.971 | 0.973 | 0.848 | 12.9% | 2.2% | 0.4s / 0.7s | 2.1 | 0 | 92.9% | 100.0% |

## jev · deep

### 검색 (445건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 97.8% [95.9–98.8] (n=445) |
|   └ 답할 수 있는 질문 | 97.9% [95.9–98.9] (n=377) |
|   └ 답할 수 없는 질문(보류가 정답) | 97.1% [89.9–99.2] (n=68) |
| 확신 정답률 (top1 정답 + recommended) | 92.6% [89.5–94.8] (n=377) |
| 라우팅 정확도 (정확 일치) | 93.3% [90.5–95.2] (n=445) |
| 계층 F1 (hF1) | 0.957 [0.940–0.974] (n=445) |
| 트리 밖 판정 precision | 96.4% [82.3–99.4] (n=28) |
| 트리 밖 판정 recall | 71.1% [55.2–83.0] (n=38) |
| 트리 밖 판정 F1 | 0.818 |
| 오보류율 (답할 수 있는데 보류) | 0.5% [0.1–1.9] (n=377) |
| 오답변율 (답할 수 없는데 답함) | 2.9% [0.8–10.1] (n=68) |
| Hit@1 | 98.1% [96.2–99.1] (n=377) |
| Hit@3 | 98.4% [96.6–99.3] (n=377) |
| Hit@k (limit) | 98.4% [96.6–99.3] (n=377) |
| MRR@k | 0.983 [0.970–0.996] (n=377) |
| nDCG@k | 0.974 [0.961–0.988] (n=377) |
| Recall@k | 0.977 [0.964–0.991] (n=377) |
| recommended 정밀도 | 0.969 [0.952–0.985] (n=354) |
| 라우팅이 맞았을 때 Hit@1 | 99.7% [98.4–100.0] (n=361) |
| 라우팅이 맞았을 때 MRR | 0.999 [0.996–1.000] (n=361) |
| 지연 p50 / p90 / p95 / 평균 | 0.5s / 0.8s / 1.0s / 0.5s |
| 라우팅 구간 평균 | 0.3s |
| 랭킹 후보 수 평균 | 19.3 |
| Jev 입력 토큰 평균 (과금) | 21368 |
| LLM 토큰 평균 (입력+출력) | 0 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 2.45 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 | d5 | d6 | d7 |
|---|---:|---:|---:|---:|---:|---:|---:|
| 일치율 | 98.9% (n=360) | 98.3% (n=350) | 94.3% (n=331) | 96.7% (n=151) | 100.0% (n=21) | 90.0% (n=10) | 87.5% (n=8) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 6 | 3 | 478 | 1 | 13 | 12 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.3s |
| internal | 24 | 100.0% | 100.0% | 100.0% | 1.000 | 0.7s |
| leaf | 353 | 97.7% | 95.5% | 98.0% | 0.982 | 0.5s |
| no_answer | 27 | 100.0% | 88.9% | - | - | 0.6s |
| oos | 38 | 94.7% | 71.1% | - | - | 0.3s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 95.2% | 92.4% | 95.1% | 0.951 | 0.5s |
| single | 340 | 98.5% | 93.5% | 99.3% | 0.995 | 0.5s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 340 | 98.5% | 93.5% | 99.3% | 0.995 | 0.5s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 0.5s |
| correct | 8 | 100.0% | 87.5% | 100.0% | 1.000 | 0.5s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 0.5s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| long | 8 | 100.0% | 87.5% | 100.0% | 1.000 | 0.4s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 0.5s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 385 | 97.7% | 92.5% | 97.9% | 0.981 | 0.5s |
| root | 43 | 97.7% | 97.7% | 100.0% | 1.000 | 0.3s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 35 | 100.0% | 97.1% | 100.0% | 1.000 | 0.6s |
| leaf | 372 | 97.8% | 95.2% | 98.0% | 0.982 | 0.5s |
| none | 38 | 94.7% | 71.1% | - | - | 0.3s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 94.7% | 71.1% | - | - | 0.3s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| d2 | 30 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| d3 | 202 | 97.5% | 93.1% | 97.3% | 0.973 | 0.5s |
| d4 | 144 | 97.9% | 97.2% | 98.6% | 0.989 | 0.5s |
| d5 | 11 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| d6 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| d7 | 8 | 100.0% | 87.5% | 100.0% | 1.000 | 0.4s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 95.1% | 73.2% | - | - | 0.3s |
| 101+ | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 1.4s |
| 2-5 | 319 | 98.1% | 95.6% | 98.3% | 0.985 | 0.5s |
| 33-100 | 61 | 96.7% | 93.4% | 96.6% | 0.966 | 0.7s |
| 6-32 | 20 | 100.0% | 95.0% | 100.0% | 1.000 | 0.6s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| colloquial | 69 | 98.6% | 97.1% | 98.5% | 0.992 | 0.5s |
| condition | 32 | 93.8% | 90.6% | 96.8% | 0.968 | 0.5s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| keyword | 20 | 100.0% | 95.0% | 100.0% | 1.000 | 0.5s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| multi_intent | 9 | 100.0% | 77.8% | 100.0% | 1.000 | 0.6s |
| negation | 20 | 100.0% | 95.0% | 100.0% | 1.000 | 0.6s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.3s |
| oos_near | 15 | 93.3% | 33.3% | - | - | 0.7s |
| paraphrase | 177 | 96.6% | 93.8% | 96.4% | 0.964 | 0.5s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| vague | 24 | 100.0% | 100.0% | 100.0% | 1.000 | 0.7s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 97.8% | 93.5% | 97.7% | 0.977 | 0.5s |
| health | 40 | 100.0% | 97.5% | 100.0% | 1.000 | 0.4s |
| it | 101 | 98.0% | 98.0% | 97.9% | 0.979 | 0.5s |
| life | 42 | 100.0% | 92.9% | 100.0% | 1.000 | 0.4s |
| money | 49 | 95.9% | 95.9% | 97.8% | 0.978 | 0.5s |
| oos | 38 | 94.7% | 71.1% | - | - | 0.3s |
| shop | 41 | 97.6% | 90.2% | 97.4% | 0.974 | 0.6s |
| travel | 44 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| work | 44 | 95.5% | 90.9% | 95.1% | 0.963 | 0.5s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 99.2% | 99.2% | 99.1% | 0.995 | 0.4s |
| hard | 125 | 93.6% | 82.4% | 94.2% | 0.942 | 0.5s |
| medium | 202 | 99.5% | 96.5% | 99.4% | 0.994 | 0.5s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 304 | 98.0% | 92.8% | 98.1% | 0.981 | 0.5s |
| test | 141 | 97.2% | 94.3% | 98.3% | 0.987 | 0.5s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 95.3% | 99.7% | 94.9% | 20.6% |
| 0.25 | 96.4% | 99.7% | 96.1% | 13.2% |
| 0.30 | 97.3% | 99.7% | 97.1% | 7.4% |
| 0.35 | 97.5% | 99.5% | 97.6% | 4.4% |
| 0.40 | 97.8% | 99.5% | 97.9% | 2.9% |
| 0.50 | 96.9% | 98.4% | 97.9% | 2.9% |
| 0.60 | 95.1% | 95.5% | 98.6% | 1.5% |
| 0.65 | 93.5% | 93.6% | 98.6% | 1.5% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 92.6% [83.9–96.8] (n=68) |
|   └ 올바른 카테고리에 저장 | 94.6% [85.4–98.2] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 83.3% [55.2–95.3] (n=12) |
| 계층 F1 | 0.989 [0.977–1.000] (n=56) |
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
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 2 |
| null | health | 2 |
| null | it | 2 |
| null | money | 2 |
| food_store_item | food_store | 1 |
| health_sym_head | health_sym | 1 |
| it_office_excel_func | null | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| life_appl_kitchen | life | 1 |
| life_appl_wash_wm_drum_water_spin | life_appl_wash_wm_drum | 1 |
| money_card_lost | money_card | 1 |
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | it_mobile | 1 |
| null | money_card_lost | 1 |
| null | shop | 1 |
| null | shop_member | 1 |
| shop_member_grade | shop_member_coupon | 1 |
| shop_order_pay | shop_order_cancel | 1 |

### 틀린 케이스 (37건 중 37건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-MONEY-001 | 체크카드를 해외에서 쓰려면 따로 설정해야 하나요? | money_card_abroad | money_card | shallow |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-SHOP-001 | 예약 판매 상품은 언제 출고되나요? | shop_ship_track | shop_ship | shallow |
| I-XD-004 | 도시락을 상온에 몇 시간까지 둬도 안전한가요? | food_safe_poison (+1) | food_safe | shallow |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_ret_refund | false_tree |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_item#garlic 0.93 recommended | route branch |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.87 recommended | route branch, wrong top1 |
| S-FOOD-517 | 쌀통에 둔 쌀이 자꾸 눅눅해지고 냄새가 나요. 쌀은 어디 두는 게 맞아요? | food_store_item | food_store · top food_store_item#rice 0.92 recommended | route shallow |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it_mobile · abstained · top it_mobile#article 0.17 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it · abstained · top it#article 0.24 reference | route false_tree |
| S-HEALTH-015 | 요즘 밤에 잠도 잘 못 자고, 오후만 되면 머리가 조이듯 지끈거려요. 둘 다 어떻게 관리하면 좋을까요? | health_sym_head (+1) | health_sym · top health_sym_head#1 0.61 alternative | route shallow |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | null · abstained · top no items | route false_none, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.72 recommended | route shallow |
| S-LIFE-024 | 식기세척기 전용 세제가 떨어졌는데 일반 주방세제 넣고 돌려도 돼요? | life_appl_kitchen · no answer | life · abstained · top life_appl_wash_wm_drum_care_smell#2 0.23 reference | route shallow |
| S-LIFE-032 | 아 죄송해요, 통돌이 아니고 앞문 여는 드럼이에요. 드럼이 아예 안 돌아요 | life_appl_wash_wm_drum_water_spin | life_appl_wash_wm_drum · top life_appl_wash_wm_drum_water_spin#1 0.85 recommended | route shallow |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost | money_card · top money_card_lost#4 0.95 recommended | route shallow |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.90 recommended | route domain, wrong top1 |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.30 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · top money_save#article 0.58 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health#article 0.16 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money · abstained · top money_card_lost#4 0.20 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | shop_member · abstained · top shop_member#article 0.17 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health · abstained · top health#article 0.38 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.22 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ret_refund_card#2 0.15 reference | route false_tree |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card | money_card_install · top shop_ret_refund_card#3 0.94 recommended | route domain |
| S-SHOP-020 | 해외 배송 상품은 통관 때문에 며칠 더 걸리나요? | shop_ship_track · no answer | shop_ship · abstained · top shop_ship_track#3 0.26 reference | route shallow |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.54 alternative | route branch, wrong top1 |
| S-SHOP-031 | 그 위 등급이 되면 매달 쿠폰은 몇 장씩 주는 거예요? | shop_member_grade | shop_member_coupon · top shop_member_grade#2 0.87 recommended | route branch |
| S-WORK-001 | 오후 반차 쓰면 몇 시에 퇴근하는 거예요? | work_leave_annual_use | work_leave · top work_leave_annual_use#2 0.88 recommended | route shallow |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.76 recommended | wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#3 0.19 reference | route domain |
| S-XD-009 | 회사 노트북 배터리가 부풀어 올라서 새 걸로 바꾸고 싶은데 어디에 신청해요? | work_it_laptop | work_it · top work_it_laptop#1 0.62 alternative | route shallow |
| S-XD-016 | 미국에 사는 동생한테 돈을 보내려는데 카드로도 보낼 수 있어요? 뭐가 필요해요? | money_bank_xfer_abroad | money_bank_xfer_abroad · abstained · top money_bank_xfer_abroad#1 0.31 reference | abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.58 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.85 recommended | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.93 recommended | route false_tree, answered unanswerable |

## jev · standard

### 검색 (444건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 98.0% [96.2–98.9] (n=444) |
|   └ 답할 수 있는 질문 | 97.9% [95.9–98.9] (n=376) |
|   └ 답할 수 없는 질문(보류가 정답) | 98.5% [92.1–99.7] (n=68) |
| 확신 정답률 (top1 정답 + recommended) | 93.4% [90.4–95.5] (n=376) |
| 라우팅 정확도 (정확 일치) | 93.7% [91.0–95.6] (n=444) |
| 계층 F1 (hF1) | 0.960 [0.943–0.977] (n=444) |
| 트리 밖 판정 precision | 100.0% [87.9–100.0] (n=28) |
| 트리 밖 판정 recall | 73.7% [58.0–85.0] (n=38) |
| 트리 밖 판정 F1 | 0.848 |
| 오보류율 (답할 수 있는데 보류) | 0.3% [0.0–1.5] (n=376) |
| 오답변율 (답할 수 없는데 답함) | 1.5% [0.3–7.9] (n=68) |
| Hit@1 | 98.1% [96.2–99.1] (n=376) |
| Hit@3 | 98.7% [96.9–99.4] (n=376) |
| Hit@k (limit) | 98.7% [96.9–99.4] (n=376) |
| MRR@k | 0.984 [0.972–0.996] (n=376) |
| nDCG@k | 0.977 [0.964–0.989] (n=376) |
| Recall@k | 0.981 [0.969–0.994] (n=376) |
| recommended 정밀도 | 0.973 [0.958–0.987] (n=356) |
| 라우팅이 맞았을 때 Hit@1 | 99.4% [98.0–99.8] (n=362) |
| 라우팅이 맞았을 때 MRR | 0.997 [0.993–1.000] (n=362) |
| 지연 p50 / p90 / p95 / 평균 | 0.4s / 0.7s / 0.7s / 0.5s |
| 라우팅 구간 평균 | 0.3s |
| 랭킹 후보 수 평균 | 18.4 |
| Jev 입력 토큰 평균 (과금) | 19337 |
| LLM 토큰 평균 (입력+출력) | 0 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 2.42 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 98.9% (n=359) | 98.0% (n=349) | 93.9% (n=330) | 96.3% (n=82) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|
| 6 | 4 | 480 | 12 | 10 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.2s |
| internal | 23 | 95.7% | 100.0% | 95.7% | 0.978 | 0.5s |
| leaf | 353 | 98.0% | 96.0% | 98.3% | 0.984 | 0.4s |
| no_answer | 27 | 100.0% | 85.2% | - | - | 0.5s |
| oos | 38 | 97.4% | 73.7% | - | - | 0.3s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 95.2% | 93.3% | 95.1% | 0.951 | 0.4s |
| single | 339 | 98.8% | 93.8% | 99.3% | 0.996 | 0.4s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 339 | 98.8% | 93.8% | 99.3% | 0.996 | 0.4s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 0.4s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 0.5s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| long | 8 | 100.0% | 87.5% | 100.0% | 1.000 | 0.5s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 0.5s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 384 | 97.9% | 93.0% | 97.9% | 0.982 | 0.5s |
| root | 43 | 97.7% | 97.7% | 100.0% | 1.000 | 0.3s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 97.1% | 97.1% | 95.7% | 0.978 | 0.5s |
| leaf | 372 | 98.1% | 95.4% | 98.3% | 0.984 | 0.4s |
| none | 38 | 97.4% | 73.7% | - | - | 0.3s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 97.4% | 73.7% | - | - | 0.3s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| d2 | 30 | 96.7% | 100.0% | 94.7% | 0.974 | 0.5s |
| d3 | 278 | 97.5% | 94.2% | 97.7% | 0.979 | 0.4s |
| d4 | 88 | 100.0% | 97.7% | 100.0% | 1.000 | 0.5s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 97.6% | 75.6% | - | - | 0.3s |
| 101+ | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.7s |
| 2-5 | 319 | 97.8% | 95.9% | 98.0% | 0.983 | 0.4s |
| 33-100 | 61 | 98.4% | 93.4% | 98.3% | 0.983 | 0.5s |
| 6-32 | 19 | 100.0% | 94.7% | 100.0% | 1.000 | 0.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| colloquial | 69 | 98.6% | 98.6% | 98.5% | 0.992 | 0.4s |
| condition | 32 | 96.9% | 90.6% | 100.0% | 1.000 | 0.5s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| keyword | 20 | 100.0% | 95.0% | 100.0% | 1.000 | 0.4s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| multi_intent | 9 | 100.0% | 77.8% | 100.0% | 1.000 | 0.6s |
| negation | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.3s |
| oos_near | 15 | 100.0% | 40.0% | - | - | 0.7s |
| paraphrase | 177 | 96.6% | 93.2% | 96.4% | 0.964 | 0.4s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| vague | 23 | 95.7% | 100.0% | 95.7% | 0.978 | 0.5s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 95.7% | 93.5% | 95.3% | 0.965 | 0.5s |
| health | 40 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| it | 101 | 99.0% | 98.0% | 99.0% | 0.990 | 0.5s |
| life | 41 | 100.0% | 95.1% | 100.0% | 1.000 | 0.4s |
| money | 49 | 95.9% | 95.9% | 97.8% | 0.978 | 0.4s |
| oos | 38 | 97.4% | 73.7% | - | - | 0.3s |
| shop | 41 | 97.6% | 90.2% | 97.4% | 0.974 | 0.4s |
| travel | 44 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| work | 44 | 95.5% | 88.6% | 95.1% | 0.963 | 0.4s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 99.2% | 99.2% | 99.1% | 0.995 | 0.4s |
| hard | 124 | 94.4% | 83.9% | 94.1% | 0.941 | 0.5s |
| medium | 202 | 99.5% | 96.5% | 99.4% | 0.997 | 0.4s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 303 | 98.0% | 93.1% | 98.1% | 0.982 | 0.4s |
| test | 141 | 97.9% | 95.0% | 98.3% | 0.987 | 0.4s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 95.7% | 100.0% | 95.1% | 17.6% |
| 0.25 | 97.3% | 100.0% | 96.9% | 7.4% |
| 0.30 | 98.2% | 100.0% | 97.9% | 1.5% |
| 0.35 | 98.0% | 99.7% | 97.9% | 1.5% |
| 0.40 | 98.0% | 99.7% | 97.9% | 1.5% |
| 0.50 | 97.3% | 98.7% | 98.1% | 1.5% |
| 0.60 | 95.5% | 96.3% | 98.3% | 1.5% |
| 0.65 | 94.1% | 94.4% | 98.6% | 1.5% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 94.1% [85.8–97.7] (n=68) |
|   └ 올바른 카테고리에 저장 | 96.4% [87.9–99.0] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 83.3% [55.2–95.3] (n=12) |
| 계층 F1 | 0.987 [0.969–1.000] (n=56) |
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
| food_store_item | food_store | 1 |
| it_office_excel_func | work | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| life_appl_kitchen | life_appl | 1 |
| money_bank_xfer_cap | money_bank | 1 |
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | health_sym | 1 |
| null | it_mobile | 1 |
| null | money | 1 |
| null | money_card_lost | 1 |
| null | shop | 1 |
| null | shop_member | 1 |
| null | travel_car | 1 |
| shop_member_grade | shop_member_coupon | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_ret_refund_card | money_card_install | 1 |

### 틀린 케이스 (35건 중 35건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-SHOP-001 | 예약 판매 상품은 언제 출고되나요? | shop_ship_track | shop_ship | shallow |
| I-WORK-001 | 사원증을 잃어버렸는데 재발급은 어떻게 받나요? | work_hr_onboard | work | shallow |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_ret | false_tree |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.46 alternative | wrong top1 |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_item#garlic 0.94 recommended | route branch |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.86 recommended | route branch, wrong top1 |
| S-FOOD-517 | 쌀통에 둔 쌀이 자꾸 눅눅해지고 냄새가 나요. 쌀은 어디 두는 게 맞아요? | food_store_item | food_store · top food_store_item#rice 0.92 recommended | route shallow |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it_mobile · abstained · top it_mobile#article 0.16 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it · abstained · top it#article 0.25 reference | route false_tree |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · top it_office_excel_func#countifs 0.82 recommended | route domain |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.78 recommended | route shallow |
| S-LIFE-024 | 식기세척기 전용 세제가 떨어졌는데 일반 주방세제 넣고 돌려도 돼요? | life_appl_kitchen · no answer | life_appl · abstained · top life_appl_wash_wm_drum_care_smell#2 0.24 reference | route shallow |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money_bank · top money_bank_xfer_cap#1 0.65 alternative | route shallow |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.89 recommended | route domain, wrong top1 |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_sec_hacked#3 0.24 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.19 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money · abstained · top money#article 0.20 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | shop_member · abstained · top shop_member_leave#1 0.18 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym · abstained · top health_sym#article 0.30 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.22 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop#article 0.14 reference | route false_tree |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card | money_card_install · top shop_ret_refund_card#3 0.93 recommended | route domain |
| S-SHOP-020 | 해외 배송 상품은 통관 때문에 며칠 더 걸리나요? | shop_ship_track · no answer | shop_ship · abstained · top shop_ship_track#3 0.26 reference | route shallow |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.56 alternative | route branch, wrong top1 |
| S-SHOP-031 | 그 위 등급이 되면 매달 쿠폰은 몇 장씩 주는 거예요? | shop_member_grade | shop_member_coupon · top shop_member_grade#2 0.90 recommended | route branch |
| S-WORK-001 | 오후 반차 쓰면 몇 시에 퇴근하는 거예요? | work_leave_annual_use | work_leave · top work_leave_annual_use#2 0.88 recommended | route shallow |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.74 recommended | wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#2 0.07 reference | route domain |
| S-WORK-021 | 경조사 생기면 회사에서 화환이나 조화도 보내주나요? | work_leave_event · no answer | work · abstained · top work_leave_event#2 0.25 reference | route shallow |
| S-XD-009 | 회사 노트북 배터리가 부풀어 올라서 새 걸로 바꾸고 싶은데 어디에 신청해요? | work_it_laptop | work_it · top work_it_laptop#1 0.70 recommended | route shallow |
| S-XD-016 | 미국에 사는 동생한테 돈을 보내려는데 카드로도 보낼 수 있어요? 뭐가 필요해요? | money_bank_xfer_abroad | money_bank_xfer_abroad · abstained · top money_bank_xfer_abroad#1 0.31 reference | abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.63 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.84 recommended | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.90 recommended | route false_tree, answered unanswerable |

## jev · shallow

### 검색 (400건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 97.8% [95.8–98.8] (n=400) |
|   └ 답할 수 있는 질문 | 97.7% [95.5–98.8] (n=345) |
|   └ 답할 수 없는 질문(보류가 정답) | 98.2% [90.4–99.7] (n=55) |
| 확신 정답률 (top1 정답 + recommended) | 93.3% [90.2–95.5] (n=345) |
| 라우팅 정확도 (정확 일치) | 94.8% [92.1–96.5] (n=400) |
| 계층 F1 (hF1) | 0.957 [0.938–0.976] (n=400) |
| 트리 밖 판정 precision | 96.3% [81.7–99.3] (n=27) |
| 트리 밖 판정 recall | 72.2% [56.0–84.2] (n=36) |
| 트리 밖 판정 F1 | 0.825 |
| 오보류율 (답할 수 있는데 보류) | 0.6% [0.2–2.1] (n=345) |
| 오답변율 (답할 수 없는데 답함) | 1.8% [0.3–9.6] (n=55) |
| Hit@1 | 98.0% [95.9–99.0] (n=345) |
| Hit@3 | 98.3% [96.3–99.2] (n=345) |
| Hit@k (limit) | 98.3% [96.3–99.2] (n=345) |
| MRR@k | 0.981 [0.967–0.995] (n=345) |
| nDCG@k | 0.969 [0.954–0.985] (n=345) |
| Recall@k | 0.971 [0.956–0.987] (n=345) |
| recommended 정밀도 | 0.982 [0.968–0.996] (n=327) |
| 라우팅이 맞았을 때 Hit@1 | 99.7% [98.3–99.9] (n=336) |
| 라우팅이 맞았을 때 MRR | 0.999 [0.996–1.000] (n=336) |
| 지연 p50 / p90 / p95 / 평균 | 0.4s / 0.6s / 0.7s / 0.4s |
| 라우팅 구간 평균 | 0.3s |
| 랭킹 후보 수 평균 | 17.1 |
| Jev 입력 토큰 평균 (과금) | 15640 |
| LLM 토큰 평균 (입력+출력) | 0 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 2.34 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 |
|---|---:|---:|
| 일치율 | 98.8% (n=344) | 96.4% (n=334) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 5 | 3 | 435 | 1 | 13 | 2 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.3s |
| internal | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| leaf | 335 | 97.6% | 97.3% | 97.9% | 0.981 | 0.4s |
| no_answer | 16 | 100.0% | 87.5% | - | - | 0.4s |
| oos | 36 | 97.2% | 72.2% | - | - | 0.2s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 95.2% | 94.3% | 95.1% | 0.951 | 0.4s |
| single | 295 | 98.6% | 94.9% | 99.2% | 0.994 | 0.4s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 295 | 98.6% | 94.9% | 99.2% | 0.994 | 0.4s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 0.4s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 0.4s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 0.4s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 369 | 97.8% | 94.9% | 97.9% | 0.980 | 0.4s |
| root | 31 | 96.8% | 93.5% | 100.0% | 1.000 | 0.2s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| leaf | 354 | 97.7% | 96.9% | 97.9% | 0.981 | 0.4s |
| none | 36 | 97.2% | 72.2% | - | - | 0.2s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 36 | 97.2% | 72.2% | - | - | 0.2s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| d2 | 354 | 97.7% | 96.9% | 97.9% | 0.981 | 0.4s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 39 | 97.4% | 74.4% | - | - | 0.2s |
| 101+ | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.7s |
| 2-5 | 301 | 98.0% | 97.3% | 98.3% | 0.984 | 0.4s |
| 33-100 | 56 | 96.4% | 94.6% | 96.4% | 0.964 | 0.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| colloquial | 66 | 98.5% | 100.0% | 98.4% | 0.992 | 0.4s |
| condition | 32 | 93.8% | 93.8% | 96.8% | 0.968 | 0.4s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| keyword | 20 | 100.0% | 95.0% | 100.0% | 1.000 | 0.4s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| multi_intent | 9 | 100.0% | 88.9% | 100.0% | 1.000 | 0.6s |
| negation | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.2s |
| oos_near | 15 | 100.0% | 46.7% | - | - | 0.5s |
| paraphrase | 149 | 96.0% | 94.0% | 95.9% | 0.959 | 0.4s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| vague | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 40 | 97.5% | 95.0% | 97.4% | 0.974 | 0.4s |
| health | 36 | 100.0% | 97.2% | 100.0% | 1.000 | 0.4s |
| it | 93 | 97.8% | 97.8% | 97.8% | 0.978 | 0.4s |
| life | 36 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| money | 44 | 95.5% | 97.7% | 97.6% | 0.976 | 0.4s |
| oos | 36 | 97.2% | 72.2% | - | - | 0.2s |
| shop | 36 | 97.2% | 94.4% | 97.1% | 0.971 | 0.4s |
| travel | 39 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| work | 40 | 95.0% | 92.5% | 94.7% | 0.961 | 0.4s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 109 | 99.1% | 100.0% | 99.0% | 0.995 | 0.4s |
| hard | 112 | 93.8% | 83.9% | 94.0% | 0.940 | 0.4s |
| medium | 179 | 99.4% | 98.3% | 99.4% | 0.994 | 0.4s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 271 | 97.8% | 93.7% | 97.9% | 0.979 | 0.4s |
| test | 129 | 97.7% | 96.9% | 98.2% | 0.986 | 0.4s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 95.8% | 99.7% | 95.5% | 18.2% |
| 0.25 | 96.0% | 99.7% | 95.8% | 16.4% |
| 0.30 | 96.5% | 99.7% | 96.3% | 12.7% |
| 0.35 | 97.0% | 99.4% | 97.1% | 7.3% |
| 0.40 | 97.8% | 99.4% | 98.0% | 1.8% |
| 0.50 | 97.5% | 99.1% | 98.0% | 1.8% |
| 0.60 | 95.2% | 96.5% | 97.9% | 1.8% |
| 0.65 | 94.0% | 94.5% | 98.5% | 1.8% |

### 저장 (ingest, 59건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 94.9% [86.1–98.3] (n=59) |
|   └ 올바른 카테고리에 저장 | 100.0% [92.7–100.0] (n=49) |
|   └ 트리/루트 밖 Q&A 거절 | 70.0% [39.7–89.2] (n=10) |
| 계층 F1 | 1.000 [1.000–1.000] (n=49) |
| 초안이 검색에 안 보임 | 100.0% [92.7–100.0] (n=49) |
| 게시 성공 (버전 +1) | 100.0% [92.7–100.0] (n=49) |
| 초안→게시 카테고리 유지 | 100.0% [92.7–100.0] (n=49) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [92.7–100.0] (n=49) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 100.0% [83.2–100.0] (n=19) |
| 게시 후 probe 검색 top1 답변 | 100.0% [83.2–100.0] (n=19) |
| 지연 p50 (초안+게시+probe) | 0.3s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| null | it | 3 |
| food_store_item | food_store_freeze | 2 |
| null | money | 2 |
| health_sym_head | health | 1 |
| it_office_excel_func | null | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | food_safe_poison | 1 |
| null | health | 1 |
| null | money_card_lost | 1 |
| null | shop | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_ret_refund_card | money_card_install | 1 |
| work_exp_trip | travel_air_ticket_mile | 1 |
| work_exp_trip | work_exp_claim | 1 |
| work_leave_event | work | 1 |

### 틀린 케이스 (26건 중 26건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_order_cancel | false_tree |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_item#garlic 0.94 recommended | route branch |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.85 recommended | route branch, wrong top1 |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it · abstained · top it#article 0.33 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it · abstained · top it#article 0.28 reference | route false_tree |
| S-HEALTH-015 | 요즘 밤에 잠도 잘 못 자고, 오후만 되면 머리가 조이듯 지끈거려요. 둘 다 어떻게 관리하면 좋을까요? | health_sym_head (+1) | health · top health_sleep#1 0.65 alternative | route shallow |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | null · abstained · top no items | route false_none, abstained |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.88 recommended | route domain, wrong top1 |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_sec_phish#2 0.35 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · abstained · top money_ins#2 0.35 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money · abstained · top money#article 0.20 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health · abstained · top health#article 0.38 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.27 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop#article 0.17 reference | route false_tree |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card | money_card_install · top shop_ret_refund_card#3 0.94 recommended | route domain |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.64 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.76 recommended | wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#3 0.08 reference | route domain |
| S-WORK-021 | 경조사 생기면 회사에서 화환이나 조화도 보내주나요? | work_leave_event · no answer | work · abstained · top work_leave_event#2 0.32 reference | route shallow |
| S-XD-016 | 미국에 사는 동생한테 돈을 보내려는데 카드로도 보낼 수 있어요? 뭐가 필요해요? | money_bank_xfer_abroad | money_bank_xfer_abroad · abstained · top money_bank_xfer_abroad#1 0.30 reference | abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.61 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.86 recommended | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.82 recommended | route false_tree, answered unanswerable |
| S-XD-052 | 상한 음식을 먹고 계속 토해요. 어떻게 해야 돼요? | null · no answer | food_safe_poison · abstained · top food_safe_poison#2 0.14 reference | route false_tree |

## jev · flat

### 검색 (369건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 97.6% [95.4–98.7] (n=369) |
|   └ 답할 수 있는 질문 | 97.2% [94.8–98.5] (n=325) |
|   └ 답할 수 없는 질문(보류가 정답) | 100.0% [92.0–100.0] (n=44) |
| 확신 정답률 (top1 정답 + recommended) | 92.3% [88.9–94.7] (n=325) |
| 라우팅 정확도 (정확 일치) | 96.7% [94.4–98.1] (n=369) |
| 계층 F1 (hF1) | 0.967 [0.949–0.986] (n=369) |
| 트리 밖 판정 precision | 92.6% [76.6–97.9] (n=27) |
| 트리 밖 판정 recall | 100.0% [86.7–100.0] (n=25) |
| 트리 밖 판정 F1 | 0.962 |
| 오보류율 (답할 수 있는데 보류) | 0.6% [0.2–2.2] (n=325) |
| 오답변율 (답할 수 없는데 답함) | 0.0% [0.0–8.0] (n=44) |
| Hit@1 | 97.5% [95.2–98.7] (n=325) |
| Hit@3 | 97.8% [95.6–99.0] (n=325) |
| Hit@k (limit) | 97.8% [95.6–99.0] (n=325) |
| MRR@k | 0.977 [0.961–0.993] (n=325) |
| nDCG@k | 0.966 [0.949–0.982] (n=325) |
| Recall@k | 0.968 [0.950–0.985] (n=325) |
| recommended 정밀도 | 0.982 [0.968–0.997] (n=305) |
| 라우팅이 맞았을 때 Hit@1 | 99.7% [98.2–99.9] (n=315) |
| 라우팅이 맞았을 때 MRR | 0.998 [0.995–1.000] (n=315) |
| 지연 p50 / p90 / p95 / 평균 | 0.4s / 0.5s / 0.6s / 0.4s |
| 라우팅 구간 평균 | 0.3s |
| 랭킹 후보 수 평균 | 12.1 |
| Jev 입력 토큰 평균 (과금) | 13414 |
| LLM 토큰 평균 (입력+출력) | 0 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 2.15 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 |
|---|---:|
| 일치율 | 95.8% (n=334) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_none |
|---:|---:|---:|---:|
| 5 | 5 | 407 | 2 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.3s |
| leaf | 325 | 97.2% | 96.9% | 97.5% | 0.977 | 0.4s |
| no_answer | 16 | 100.0% | 87.5% | - | - | 0.4s |
| oos | 25 | 100.0% | 100.0% | - | - | 0.2s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 94.3% | 94.3% | 94.2% | 0.942 | 0.4s |
| single | 264 | 98.9% | 97.7% | 99.1% | 0.993 | 0.4s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 264 | 98.9% | 97.7% | 99.1% | 0.993 | 0.4s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 0.4s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| disambig | 20 | 85.0% | 85.0% | 85.0% | 0.850 | 0.4s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| shift | 14 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 359 | 97.5% | 96.7% | 97.5% | 0.976 | 0.4s |
| root | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.2s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| leaf | 344 | 97.4% | 96.5% | 97.5% | 0.977 | 0.4s |
| none | 25 | 100.0% | 100.0% | - | - | 0.2s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 25 | 100.0% | 100.0% | - | - | 0.2s |
| d1 | 344 | 97.4% | 96.5% | 97.5% | 0.977 | 0.4s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 28 | 100.0% | 100.0% | - | - | 0.2s |
| 2-5 | 291 | 97.6% | 96.9% | 97.8% | 0.980 | 0.4s |
| 33-100 | 50 | 96.0% | 94.0% | 95.9% | 0.959 | 0.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| colloquial | 66 | 98.5% | 100.0% | 98.4% | 0.992 | 0.4s |
| condition | 31 | 93.5% | 93.5% | 96.8% | 0.968 | 0.4s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| keyword | 20 | 100.0% | 95.0% | 100.0% | 1.000 | 0.4s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| multi_intent | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| negation | 20 | 100.0% | 95.0% | 100.0% | 1.000 | 0.4s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.2s |
| oos_near | 15 | 100.0% | 100.0% | - | - | 0.2s |
| paraphrase | 129 | 95.3% | 93.8% | 94.7% | 0.947 | 0.4s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 37 | 97.3% | 94.6% | 97.2% | 0.972 | 0.4s |
| health | 33 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| it | 90 | 97.8% | 97.8% | 97.7% | 0.977 | 0.4s |
| life | 34 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| money | 42 | 95.2% | 95.2% | 97.5% | 0.975 | 0.4s |
| oos | 25 | 100.0% | 100.0% | - | - | 0.2s |
| shop | 33 | 97.0% | 93.9% | 96.8% | 0.968 | 0.4s |
| travel | 37 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| work | 38 | 92.1% | 89.5% | 91.7% | 0.931 | 0.4s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 99 | 99.0% | 100.0% | 98.9% | 0.995 | 0.4s |
| hard | 107 | 93.5% | 90.7% | 92.9% | 0.929 | 0.4s |
| medium | 163 | 99.4% | 98.8% | 99.3% | 0.993 | 0.4s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 248 | 98.0% | 96.8% | 97.7% | 0.977 | 0.4s |
| test | 121 | 96.7% | 96.7% | 97.1% | 0.976 | 0.4s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 97.8% | 99.7% | 97.8% | 0.0% |
| 0.25 | 97.8% | 99.7% | 97.8% | 0.0% |
| 0.30 | 97.8% | 99.7% | 97.8% | 0.0% |
| 0.35 | 97.6% | 99.4% | 97.8% | 0.0% |
| 0.40 | 97.6% | 99.4% | 97.8% | 0.0% |
| 0.50 | 97.3% | 99.1% | 97.8% | 0.0% |
| 0.60 | 94.9% | 96.0% | 98.1% | 0.0% |
| 0.65 | 93.2% | 93.8% | 98.4% | 0.0% |

### 저장 (ingest, 50건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 100.0% [92.9–100.0] (n=50) |
|   └ 올바른 카테고리에 저장 | 100.0% [92.4–100.0] (n=47) |
|   └ 트리/루트 밖 Q&A 거절 | 100.0% [43.8–100.0] (n=3) |
| 계층 F1 | 1.000 [1.000–1.000] (n=47) |
| 초안이 검색에 안 보임 | 100.0% [92.4–100.0] (n=47) |
| 게시 성공 (버전 +1) | 100.0% [92.4–100.0] (n=47) |
| 초안→게시 카테고리 유지 | 100.0% [92.4–100.0] (n=47) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [92.4–100.0] (n=47) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 94.7% [75.4–99.1] (n=19) |
| 게시 후 probe 검색 top1 답변 | 94.7% [75.4–99.1] (n=19) |
| 지연 p50 (초안+게시+probe) | 0.3s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (11건)

통과 100.0% [74.1–100.0] (n=11)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 2 |
| it_office_excel_func | null | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| money_card_lost | it_sec_hacked | 1 |
| money_tax_yearend | work_exp_claim | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_ret_refund_card | money_card_install | 1 |
| work_exp_card | money_card_limit | 1 |
| work_exp_trip | travel_air_ticket_mile | 1 |
| work_exp_trip | work_exp_claim | 1 |
| work_leave_event | null | 1 |

### 틀린 케이스 (14건 중 14건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_item#garlic 0.94 recommended | route branch |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.86 recommended | route branch, wrong top1 |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | null · abstained · top no items | route false_none, abstained |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost | it_sec_hacked · top money_card_lost#4 0.95 recommended | route domain |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.89 recommended | route domain, wrong top1 |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card | money_card_install · top shop_ret_refund_card#3 0.94 recommended | route domain |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.56 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.75 recommended | wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#2 0.07 reference | route domain |
| S-WORK-021 | 경조사 생기면 회사에서 화환이나 조화도 보내주나요? | work_leave_event · no answer | null · abstained · top no items | route false_none |
| S-XD-016 | 미국에 사는 동생한테 돈을 보내려는데 카드로도 보낼 수 있어요? 뭐가 필요해요? | money_bank_xfer_abroad | money_bank_xfer_abroad · abstained · top money_bank_xfer_abroad#1 0.31 reference | abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.65 alternative | route branch, wrong top1 |
| S-XD-024 | 한도 좀 올리고 싶은데 어떻게 해요? | work_exp_card | money_card_limit · top money_card_limit#1 0.79 recommended | route domain, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.81 recommended | route branch, wrong top1 |

## jev · sparse

### 검색 (444건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 90.8% [87.7–93.1] (n=444) |
|   └ 답할 수 있는 질문 | 96.1% [92.2–98.1] (n=180) |
|   └ 답할 수 없는 질문(보류가 정답) | 87.1% [82.5–90.6] (n=264) |
| 확신 정답률 (top1 정답 + recommended) | 87.2% [81.6–91.3] (n=180) |
| 라우팅 정확도 (정확 일치) | 93.7% [91.0–95.6] (n=444) |
| 계층 F1 (hF1) | 0.960 [0.943–0.977] (n=444) |
| 트리 밖 판정 precision | 100.0% [87.9–100.0] (n=28) |
| 트리 밖 판정 recall | 73.7% [58.0–85.0] (n=38) |
| 트리 밖 판정 F1 | 0.848 |
| 오보류율 (답할 수 있는데 보류) | 2.2% [0.9–5.6] (n=180) |
| 오답변율 (답할 수 없는데 답함) | 12.9% [9.4–17.5] (n=264) |
| Hit@1 | 96.7% [92.9–98.5] (n=180) |
| Hit@3 | 97.8% [94.4–99.1] (n=180) |
| Hit@k (limit) | 97.8% [94.4–99.1] (n=180) |
| MRR@k | 0.971 [0.948–0.995] (n=180) |
| nDCG@k | 0.973 [0.951–0.996] (n=180) |
| Recall@k | 0.972 [0.949–0.995] (n=180) |
| recommended 정밀도 | 0.909 [0.868–0.950] (n=170) |
| 라우팅이 맞았을 때 Hit@1 | 97.7% [94.3–99.1] (n=176) |
| 라우팅이 맞았을 때 MRR | 0.979 [0.959–1.000] (n=176) |
| 지연 p50 / p90 / p95 / 평균 | 0.4s / 0.6s / 0.7s / 0.5s |
| 라우팅 구간 평균 | 0.3s |
| 랭킹 후보 수 평균 | 5.2 |
| Jev 입력 토큰 평균 (과금) | 16299 |
| LLM 토큰 평균 (입력+출력) | 0 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 2.11 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 98.9% (n=359) | 98.0% (n=349) | 94.2% (n=330) | 97.6% (n=82) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|
| 6 | 4 | 468 | 12 | 10 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dropped | 196 | 83.2% | 94.9% | - | - | 0.4s |
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.3s |
| internal | 23 | 95.7% | 100.0% | 95.7% | 0.971 | 0.5s |
| leaf | 157 | 96.2% | 97.5% | 96.8% | 0.971 | 0.4s |
| no_answer | 27 | 100.0% | 85.2% | - | - | 0.4s |
| oos | 38 | 97.4% | 73.7% | - | - | 0.3s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 87.6% | 93.3% | 97.6% | 0.976 | 0.4s |
| single | 339 | 91.7% | 93.8% | 96.4% | 0.970 | 0.4s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 339 | 91.7% | 93.8% | 96.4% | 0.970 | 0.4s |
| coref | 19 | 89.5% | 84.2% | 100.0% | 1.000 | 0.4s |
| correct | 8 | 87.5% | 100.0% | 100.0% | 1.000 | 0.5s |
| disambig | 20 | 80.0% | 90.0% | 90.0% | 0.900 | 0.4s |
| distract | 9 | 88.9% | 100.0% | 100.0% | 1.000 | 0.5s |
| ellipsis | 9 | 88.9% | 100.0% | 100.0% | 1.000 | 0.5s |
| long | 8 | 75.0% | 87.5% | 100.0% | 1.000 | 0.5s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 0.4s |
| slot | 9 | 77.8% | 100.0% | 100.0% | 1.000 | 0.5s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 384 | 90.6% | 93.0% | 96.5% | 0.970 | 0.4s |
| root | 43 | 90.7% | 97.7% | 100.0% | 1.000 | 0.3s |
| start | 17 | 94.1% | 100.0% | 100.0% | 1.000 | 0.4s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 97.1% | 97.1% | 95.7% | 0.971 | 0.5s |
| leaf | 372 | 89.5% | 95.4% | 96.8% | 0.971 | 0.4s |
| none | 38 | 97.4% | 73.7% | - | - | 0.3s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 97.4% | 73.7% | - | - | 0.3s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| d2 | 30 | 96.7% | 100.0% | 93.3% | 0.956 | 0.4s |
| d3 | 278 | 89.6% | 94.2% | 97.5% | 0.979 | 0.4s |
| d4 | 88 | 88.6% | 97.7% | 94.4% | 0.944 | 0.4s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 97.6% | 75.6% | - | - | 0.3s |
| 1 | 369 | 89.4% | 95.4% | 96.8% | 0.971 | 0.4s |
| 2-5 | 16 | 100.0% | 93.8% | 100.0% | 1.000 | 0.4s |
| 33-100 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| 6-32 | 16 | 93.8% | 100.0% | 92.9% | 0.952 | 0.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | - | - | 0.4s |
| colloquial | 69 | 89.9% | 98.6% | 100.0% | 1.000 | 0.4s |
| condition | 32 | 81.2% | 90.6% | 100.0% | 1.000 | 0.5s |
| english | 20 | 95.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| keyword | 20 | 90.0% | 95.0% | 100.0% | 1.000 | 0.4s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |
| mixed | 13 | 92.3% | 100.0% | 85.7% | 0.857 | 0.4s |
| multi_intent | 9 | 66.7% | 77.8% | 66.7% | 0.722 | 0.6s |
| negation | 20 | 85.0% | 100.0% | 100.0% | 1.000 | 0.4s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.3s |
| oos_near | 15 | 100.0% | 40.0% | - | - | 0.6s |
| paraphrase | 177 | 91.0% | 93.2% | 98.0% | 0.980 | 0.4s |
| typo | 13 | 92.3% | 100.0% | 100.0% | 1.000 | 0.4s |
| vague | 23 | 95.7% | 100.0% | 95.7% | 0.971 | 0.5s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 91.3% | 93.5% | 100.0% | 1.000 | 0.4s |
| health | 40 | 95.0% | 100.0% | 94.1% | 0.961 | 0.4s |
| it | 101 | 91.1% | 98.0% | 93.6% | 0.936 | 0.4s |
| life | 41 | 85.4% | 95.1% | 100.0% | 1.000 | 0.4s |
| money | 49 | 77.6% | 95.9% | 96.0% | 0.980 | 0.4s |
| oos | 38 | 97.4% | 73.7% | - | - | 0.3s |
| shop | 41 | 87.8% | 90.2% | 94.7% | 0.947 | 0.5s |
| travel | 44 | 97.7% | 100.0% | 100.0% | 1.000 | 0.5s |
| work | 44 | 95.5% | 88.6% | 100.0% | 1.000 | 0.5s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 96.6% | 99.2% | 100.0% | 1.000 | 0.4s |
| hard | 124 | 84.7% | 83.9% | 94.7% | 0.947 | 0.5s |
| medium | 202 | 91.1% | 96.5% | 95.6% | 0.965 | 0.4s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 303 | 89.4% | 93.1% | 97.7% | 0.979 | 0.4s |
| test | 141 | 93.6% | 95.0% | 94.1% | 0.951 | 0.4s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 78.8% | 98.3% | 65.7% | 33.3% |
| 0.25 | 83.6% | 98.3% | 71.3% | 25.4% |
| 0.30 | 87.4% | 97.8% | 76.9% | 18.6% |
| 0.35 | 89.2% | 97.8% | 79.7% | 15.5% |
| 0.40 | 92.1% | 97.2% | 85.2% | 10.6% |
| 0.50 | 93.0% | 95.0% | 88.5% | 7.6% |
| 0.60 | 92.8% | 90.6% | 91.5% | 5.3% |
| 0.65 | 92.1% | 87.8% | 92.4% | 4.5% |

### 저장 (ingest, 56건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 92.9% [83.0–97.2] (n=56) |
|   └ 올바른 카테고리에 저장 | 95.5% [84.9–98.7] (n=44) |
|   └ 트리/루트 밖 Q&A 거절 | 83.3% [55.2–95.3] (n=12) |
| 계층 F1 | 0.984 [0.960–1.000] (n=44) |
| 초안이 검색에 안 보임 | 100.0% [92.0–100.0] (n=44) |
| 게시 성공 (버전 +1) | 100.0% [92.0–100.0] (n=44) |
| 초안→게시 카테고리 유지 | 100.0% [92.0–100.0] (n=44) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [92.0–100.0] (n=44) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 3 |
| 게시 후 probe 검색 top-k 포함 | 100.0% [85.7–100.0] (n=23) |
| 게시 후 probe 검색 top1 답변 | 100.0% [85.7–100.0] (n=23) |
| 지연 p50 (초안+게시+probe) | 0.4s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (11건)

통과 100.0% [74.1–100.0] (n=11)

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
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | health_sym | 1 |
| null | it_mobile | 1 |
| null | money | 1 |
| null | money_card_lost | 1 |
| null | shop | 1 |
| null | shop_member | 1 |
| null | travel_car | 1 |
| shop_member_grade | shop_member_coupon | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_ret_refund_card | money_card_install | 1 |

### 틀린 케이스 (67건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-SHOP-001 | 예약 판매 상품은 언제 출고되나요? | shop_ship_track | shop_ship | shallow |
| I-WORK-001 | 사원증을 잃어버렸는데 재발급은 어떻게 받나요? | work_hr_onboard | work | shallow |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_ret | false_tree |
| S-FOOD-007 | 배탈 난 뒤 관리 말고요, 여름에 식중독 안 걸리게 미리 조심할 것들 알려주세요 | food_safe_poison · no answer | food_safe_poison · top food_safe_poison#1 0.47 alternative | answered unanswerable |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item · no answer | food_store_freeze · top food_store_freeze#1 0.93 recommended | route branch, answered unanswerable |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item · no answer | food_store_freeze · top food_store_freeze#1 0.90 recommended | route branch, answered unanswerable |
| S-FOOD-517 | 쌀통에 둔 쌀이 자꾸 눅눅해지고 냄새가 나요. 쌀은 어디 두는 게 맞아요? | food_store_item · no answer | food_store · abstained · top food_store#article 0.39 reference | route shallow |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it_mobile · abstained · top it_mobile#article 0.15 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it · abstained · top it#article 0.25 reference | route false_tree |
| S-HEALTH-007 | My 8-month-old baby is choking on something and can't breathe. What should I do  | health_aid_emerg_choke · no answer | health_aid_emerg_choke · top health_aid_emerg_choke#1 0.37 reference | answered unanswerable |
| S-HEALTH-017 | 집에서 다쳤을 때 쓸 응급처치를 종류별로 한번 정리해 두고 싶어요 | health_aid | health_aid · top health_aid_injury_sprain#1 0.69 recommended | wrong top1 |
| S-IT-041 | 같은 브랜드로 가요. 옮기기 전에 백업은 어떻게 해 둬요? | it_mobile_move_android · no answer | it_mobile_move_android · top it_mobile_move_android#1 0.62 alternative | answered unanswerable |
| S-IT-043 | 아 잘못 말했어요, 새로 산 게 아이폰이 아니라 갤럭시예요. 그럼 어떻게 옮겨요? | it_mobile_move_cross · no answer | it_mobile_move_cross · top it_mobile_move_cross#1 0.39 reference | answered unanswerable |
| S-IT-051 | 누가 제 이메일 계정 비번이랑 복구 메일까지 바꿔 놔서 들어갈 수가 없어요 | it_sec_hacked · no answer | it_sec_hacked · top it_sec_hacked#1 0.34 reference | answered unanswerable |
| S-IT-503 | 전체 화면 말고 지금 띄워 놓은 창 하나만 찍고 싶어요 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.77 recommended | answered unanswerable |
| S-IT-504 | 클립보드 기록 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.82 recommended | answered unanswerable |
| S-IT-515 | 빔프로젝터 연결했는데 노트북 화면이랑 똑같이 duplicate 되게 바꾸는 키? | it_pc_win_keys (+1) | it_pc_win_keys · abstained · top it_pc_win_keys#win_shift_s 0.12 reference | abstained |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func · no answer | work · abstained · top work_pay_ot#1 0.25 reference | route domain |
| S-LIFE-001 | 드럼세탁기 코스가 다 끝났는데 옷이 물에 푹 젖은 채로 나와요. 탈수가 안 된 것 같아요 | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.80 recommended | answered unanswerable |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell · no answer | life_appl_wash_wm_drum_care_smell · top life_appl_wash_wm_drum_care_smell#1 0.66 recommended | answered unanswerable |
| S-LIFE-006 | 통돌이 세탁기로 빨래하면 검은 찌꺼기가 뭍어나와여 청소 어케해요 | life_appl_wash_wm_top_clean · no answer | life_appl_wash_wm_top_clean · top life_appl_wash_wm_top_clean#1 0.82 recommended | answered unanswerable |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.55 alternative | route shallow |
| S-LIFE-024 | 식기세척기 전용 세제가 떨어졌는데 일반 주방세제 넣고 돌려도 돼요? | life_appl_kitchen · no answer | life_appl · abstained · top life_appl#article 0.18 reference | route shallow |
| S-LIFE-030 | 빨래를 조금만 넣었을 때 탈수가 잘 안 되는데 왜 그래요? | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.77 recommended | answered unanswerable |
| S-LIFE-035 | 그럼 아까 그거 빨고 나서 방에서 말릴 때 냄새 안 나게 하려면 어떻게 해요? | life_clean_towel · no answer | life_clean_towel · top life_clean_towel#1 0.46 alternative | answered unanswerable |
| S-LIFE-036 | 겨울만 되면 거실 창틀이 곰팡이로 까매져요 | life_clean_mold · no answer | life_clean_mold · top life_clean_mold#1 0.40 alternative | answered unanswerable |
| S-MONEY-002 | 통장 만든 지 일주일 됐는데 하루에 30만 원까지만 보내져요. 앱에서 이체 한도 올리기를 눌러 봐도 안 되는데 이거 계좌 자체가 묶인 건가요? | money_bank_acct_limit · no answer | money_bank_acct_limit · top money_bank_acct_limit#1 0.36 reference | answered unanswerable |
| S-MONEY-005 | 인터넷뱅킹으로 계좌이체할 때 하루 한도를 늘리려면 OTP가 꼭 있어야 하나요? | money_bank_xfer_cap · no answer | money_bank_xfer_cap · top money_bank_xfer_cap#1 0.68 recommended | answered unanswerable |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost · no answer | money_card_lost · top money_card_lost#1 0.60 alternative | answered unanswerable |
| S-MONEY-012 | 온라인 쇼핑몰에서 결제하는데 한도 초과라고 승인이 안 나요. 쇼핑몰 오류가 아니라 제 신용카드 한도가 찬 거라는데 지금 바로 결제하려면 어떻게  | money_card_limit · no answer | money_card_limit · top money_card_limit#1 0.38 reference | answered unanswerable |
| S-MONEY-017 | 직장인인데 주말마다 과외해서 번 돈이 좀 있어요. 이것도 회사 연말정산으로 끝나나요, 아니면 제가 따로 신고해야 하나요? | money_tax_income · no answer | money_tax_income · top money_tax_income#1 0.60 alternative | answered unanswerable |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money_bank · top money_bank#article 0.57 alternative | route shallow, wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend · no answer | work_exp_claim · abstained · top work_exp_claim#1 0.29 reference | route domain |
| S-MONEY-027 | 마통은요? | money_loan_credit · no answer | money_loan_credit · top money_loan_credit#1 0.81 recommended | answered unanswerable |
| S-MONEY-033 | 한도를 당장 다시 살리는 방법은 없어요? | money_card_limit · no answer | money_card_limit · top money_card_limit#1 0.37 reference | answered unanswerable |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit · no answer | money_bank_acct_limit · top money_bank_acct_limit#1 0.58 alternative | answered unanswerable |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it#article 0.20 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.19 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money · abstained · top money#article 0.23 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | shop_member · abstained · top shop_member_leave#1 0.18 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym · abstained · top health_sym#article 0.30 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.23 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop#article 0.14 reference | route false_tree |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card · no answer | money_card_install · abstained · top money_card_install#1 0.14 reference | route domain |
| S-SHOP-015 | 배송지 주소를 잘못 넣은 것도 고쳐야 하고, 무통장 입금은 언제까지 해야 하는지도 알려 주세요 | shop_ship_addr (+1) | shop_order_pay · abstained · top shop_order_pay#1 0.18 reference | abstained |
| S-SHOP-020 | 해외 배송 상품은 통관 때문에 며칠 더 걸리나요? | shop_ship_track · no answer | shop_ship · abstained · top shop_ship#article 0.24 reference | route shallow |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay · no answer | shop_order_cancel · top shop_order_cancel#1 0.56 alternative | route branch, answered unanswerable |
| S-SHOP-031 | 그 위 등급이 되면 매달 쿠폰은 몇 장씩 주는 거예요? | shop_member_grade · no answer | shop_member_coupon · abstained · top shop_member_coupon#1 0.13 reference | route branch |
| S-SHOP-033 | 입어 보려고 택을 뗐는데 이제 반품 안 되나요? | shop_ret_return_mind · no answer | shop_ret_return_mind · top shop_ret_return_mind#1 0.55 alternative | answered unanswerable |
| S-TRAVEL-023 | 그 조건이면 지금 취소해도 한 푼도 못 받는 거예요? | travel_stay_cancel · no answer | travel_stay_cancel · top travel_stay_cancel#1 0.39 reference | answered unanswerable |
| S-WORK-001 | 오후 반차 쓰면 몇 시에 퇴근하는 거예요? | work_leave_annual_use · no answer | work_leave · abstained · top work_leave#article 0.21 reference | route shallow |
| S-WORK-011 | 영수증 잃어버림ㅠㅠ 거래처랑 점심 먹은 거 내 돈으로 냈는데 이거 정산 못 받나요 | work_exp_claim · no answer | work_exp_claim · top work_exp_claim#1 0.64 alternative | answered unanswerable |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#1 0.05 reference | route domain |
| S-WORK-021 | 경조사 생기면 회사에서 화환이나 조화도 보내주나요? | work_leave_event · no answer | work · abstained · top work#article 0.23 reference | route shallow |
| S-WORK-029 | 내일도 못 나갈 것 같은데 회사엔 어떻게 처리해요? | work_leave_sick · no answer | work_leave_sick · top work_leave_sick#1 0.60 alternative | answered unanswerable |
| S-XD-003 | 쇼핑몰 주문을 카드 결제 후 바로 취소했는데 카드 명세서에는 아직 청구돼 있어요 | shop_ret_refund_card · no answer | shop_ret_refund_card · top shop_ret_refund_card#1 0.46 alternative | answered unanswerable |
| S-XD-004 | 해외여행 가서 카드 긁으면 수수료가 얼마나 붙어요? | money_card_abroad · no answer | money_card_abroad · top money_card_abroad#1 0.50 alternative | answered unanswerable |
| S-XD-007 | 냉장고에 둔 우유가 표시된 날짜에서 이틀 지났는데 버려야 하나요? | food_safe_date · no answer | food_safe_date · top food_safe_date#1 0.35 reference | answered unanswerable |
| S-XD-009 | 회사 노트북 배터리가 부풀어 올라서 새 걸로 바꾸고 싶은데 어디에 신청해요? | work_it_laptop | work_it · top work_it_laptop#1 0.69 recommended | route shallow |

