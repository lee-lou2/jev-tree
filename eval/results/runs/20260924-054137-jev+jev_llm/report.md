# 평가 결과 — 20260924-054137-jev+jev_llm

- 실행: 2026-09-24T05:41:37Z → 2026-09-24T07:11:47Z (UTC) · git `99cb09e` (커밋 안 된 변경 있음)
- 데이터셋: `kb` v1.0.0 · kb `489e5fca3c7c` · cases `b2a362fcd60d`
- 설정: routers ["jev","jev_llm"] · variants ["deep","standard","shallow","flat","sparse"] · suites ["all"] · split all · repeat 1 · concurrency 10 · beam_width "case/default" · limit "case/default"
- 모델(jev): jev `jev-latest`
- 모델(jev_llm): jev `jev-latest` · LLM `gpt-6-luna`
- 메모: one-shot LLM routing (v0.5.0); jev_llm=gpt-6-luna

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev | deep | 445 | 91.9% | 88.3% | 0.914 | 91.2% | 0.916 | 0.906 | 0.712 | 4.4% | 2.9% | 1.7s / 3.2s | 4.4 | 0 | 86.8% | 100.0% |
| jev | standard | 444 | 91.9% | 89.2% | 0.920 | 91.2% | 0.917 | 0.908 | 0.712 | 2.9% | 3.2% | 1.7s / 3.3s | 4.1 | 0 | 86.8% | 100.0% |
| jev | shallow | 400 | 95.5% | 92.2% | 0.929 | 95.4% | 0.957 | 0.943 | 0.691 | 3.6% | 1.4% | 1.3s / 2.7s | 3.2 | 0 | 91.5% | 100.0% |
| jev | flat | 369 | 97.3% | 97.0% | 0.970 | 97.2% | 0.974 | 0.960 | 0.958 | 0.0% | 0.3% | 1.0s / 2.6s | 2.1 | 0 | 100.0% | 100.0% |
| jev | sparse | 444 | 84.9% | 89.6% | 0.924 | 90.0% | 0.901 | 0.904 | 0.712 | 18.2% | 6.1% | 1.6s / 2.5s | 3.8 | 0 | 85.7% | 100.0% |
| jev_llm | deep | 445 | 96.4% | 95.1% | 0.966 | 95.8% | 0.964 | 0.953 | 0.882 | 0.0% | 1.6% | 8.0s / 44.4s | 1.2 | 0 | 95.6% | 100.0% |
| jev_llm | standard | 444 | 96.6% | 95.7% | 0.967 | 96.5% | 0.973 | 0.963 | 0.848 | 1.5% | 0.8% | 12.4s / 46.7s | 1.2 | 0 | 97.1% | 100.0% |
| jev_llm | shallow | 400 | 97.5% | 95.2% | 0.958 | 97.7% | 0.979 | 0.965 | 0.780 | 3.6% | 0.3% | 11.8s / 35.7s | 1.2 | 0 | 96.6% | 100.0% |
| jev_llm | flat | 369 | 98.6% | 98.9% | 0.989 | 98.5% | 0.986 | 0.970 | 1.000 | 0.0% | 0.3% | 12.3s / 90.7s | 1.2 | 0 | 100.0% | 100.0% |
| jev_llm | sparse | 444 | 88.1% | 95.9% | 0.973 | 96.1% | 0.962 | 0.963 | 0.882 | 17.0% | 3.3% | 12.6s / 62.8s | 1.0 | 0 | 94.6% | 100.0% |

## jev · deep

### 검색 (445건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 91.9% [89.0–94.1] (n=445) |
|   └ 답할 수 있는 질문 | 91.2% [88.0–93.7] (n=377) |
|   └ 답할 수 없는 질문(보류가 정답) | 95.6% [87.8–98.5] (n=68) |
| 확신 정답률 (top1 정답 + recommended) | 85.7% [81.8–88.9] (n=377) |
| 라우팅 정확도 (정확 일치) | 88.3% [85.0–91.0] (n=445) |
| 계층 F1 (hF1) | 0.914 [0.890–0.938] (n=445) |
| 트리 밖 판정 precision | 100.0% [84.5–100.0] (n=21) |
| 트리 밖 판정 recall | 55.3% [39.7–69.9] (n=38) |
| 트리 밖 판정 F1 | 0.712 |
| 오보류율 (답할 수 있는데 보류) | 2.9% [1.6–5.1] (n=377) |
| 오답변율 (답할 수 없는데 답함) | 4.4% [1.5–12.2] (n=68) |
| Hit@1 | 91.2% [88.0–93.7] (n=377) |
| Hit@3 | 91.8% [88.6–94.1] (n=377) |
| Hit@k (limit) | 92.3% [89.2–94.6] (n=377) |
| MRR@k | 0.916 [0.889–0.944] (n=377) |
| nDCG@k | 0.906 [0.878–0.933] (n=377) |
| Recall@k | 0.910 [0.882–0.938] (n=377) |
| recommended 정밀도 | 0.960 [0.941–0.980] (n=332) |
| 라우팅이 맞았을 때 Hit@1 | 98.8% [97.1–99.5] (n=345) |
| 라우팅이 맞았을 때 MRR | 0.993 [0.986–1.000] (n=345) |
| 지연 p50 / p90 / p95 / 평균 | 1.7s / 2.9s / 3.2s / 1.8s |
| 라우팅 구간 평균 | 1.2s |
| 랭킹 후보 수 평균 | 16.2 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 4.41 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 | d5 | d6 | d7 |
|---|---:|---:|---:|---:|---:|---:|---:|
| 일치율 | 97.2% (n=360) | 94.3% (n=350) | 91.2% (n=331) | 92.1% (n=151) | 85.7% (n=21) | 80.0% (n=10) | 75.0% (n=8) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 18 | 6 | 10 | 452 | 22 | 5 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.9s |
| internal | 24 | 66.7% | 79.2% | 66.7% | 0.708 | 2.0s |
| leaf | 353 | 92.9% | 92.4% | 92.9% | 0.931 | 1.7s |
| no_answer | 27 | 96.3% | 88.9% | - | - | 1.3s |
| oos | 38 | 94.7% | 55.3% | - | - | 0.8s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 91.4% | 90.5% | 91.3% | 0.913 | 1.8s |
| single | 340 | 92.1% | 87.6% | 91.2% | 0.918 | 1.7s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 340 | 92.1% | 87.6% | 91.2% | 0.918 | 1.7s |
| coref | 19 | 73.7% | 73.7% | 73.7% | 0.737 | 1.8s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 1.7s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.8s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.6s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 2.0s |
| long | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 1.8s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.5s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 1.9s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.6s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 385 | 91.4% | 87.5% | 90.9% | 0.913 | 1.8s |
| root | 43 | 93.0% | 90.7% | 92.9% | 0.929 | 0.7s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 35 | 77.1% | 82.9% | 66.7% | 0.708 | 1.8s |
| leaf | 372 | 93.0% | 92.2% | 92.9% | 0.931 | 1.7s |
| none | 38 | 94.7% | 55.3% | - | - | 0.8s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 94.7% | 55.3% | - | - | 0.8s |
| d1 | 10 | 60.0% | 60.0% | 60.0% | 0.600 | 2.0s |
| d2 | 30 | 86.7% | 93.3% | 78.9% | 0.842 | 1.7s |
| d3 | 202 | 92.1% | 92.1% | 92.0% | 0.920 | 1.5s |
| d4 | 144 | 95.1% | 93.1% | 95.0% | 0.954 | 1.9s |
| d5 | 11 | 90.9% | 90.9% | 90.9% | 0.909 | 2.2s |
| d6 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 3.5s |
| d7 | 8 | 75.0% | 75.0% | 75.0% | 0.750 | 2.9s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 95.1% | 58.5% | - | - | 0.9s |
| 101+ | 4 | 50.0% | 50.0% | 50.0% | 0.500 | 4.1s |
| 2-5 | 319 | 92.5% | 91.8% | 92.4% | 0.927 | 1.6s |
| 33-100 | 61 | 91.8% | 90.2% | 91.5% | 0.915 | 2.5s |
| 6-32 | 20 | 85.0% | 95.0% | 72.7% | 0.795 | 1.8s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 87.5% | 100.0% | 1.000 | 1.5s |
| colloquial | 69 | 95.7% | 97.1% | 95.4% | 0.962 | 1.7s |
| condition | 32 | 87.5% | 84.4% | 87.1% | 0.871 | 2.0s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 1.7s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 1.8s |
| keyword | 20 | 85.0% | 85.0% | 85.0% | 0.850 | 1.7s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.8s |
| mixed | 13 | 92.3% | 92.3% | 92.3% | 0.923 | 1.9s |
| multi_intent | 9 | 100.0% | 77.8% | 100.0% | 1.000 | 1.8s |
| negation | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.6s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.4s |
| oos_near | 15 | 93.3% | 0.0% | - | - | 2.4s |
| paraphrase | 177 | 92.1% | 91.0% | 91.4% | 0.914 | 1.6s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 2.0s |
| vague | 24 | 66.7% | 79.2% | 66.7% | 0.708 | 2.0s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 87.0% | 87.0% | 86.0% | 0.866 | 1.8s |
| health | 40 | 100.0% | 100.0% | 100.0% | 1.000 | 1.6s |
| it | 101 | 97.0% | 95.0% | 96.9% | 0.969 | 2.1s |
| life | 42 | 90.5% | 88.1% | 89.5% | 0.895 | 1.6s |
| money | 49 | 89.8% | 91.8% | 89.1% | 0.897 | 1.8s |
| oos | 38 | 94.7% | 55.3% | - | - | 0.8s |
| shop | 41 | 80.5% | 82.9% | 78.9% | 0.803 | 1.6s |
| travel | 44 | 95.5% | 95.5% | 94.9% | 0.949 | 1.6s |
| work | 44 | 86.4% | 86.4% | 87.8% | 0.890 | 1.4s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 94.9% | 95.8% | 94.5% | 0.955 | 1.6s |
| hard | 125 | 90.4% | 77.6% | 89.5% | 0.895 | 1.8s |
| medium | 202 | 91.1% | 90.6% | 90.1% | 0.903 | 1.7s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 304 | 92.1% | 87.2% | 91.5% | 0.918 | 1.7s |
| test | 141 | 91.5% | 90.8% | 90.8% | 0.914 | 1.6s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 90.8% | 97.3% | 91.7% | 11.8% |
| 0.25 | 91.9% | 97.1% | 93.2% | 4.4% |
| 0.30 | 91.9% | 97.1% | 93.2% | 4.4% |
| 0.35 | 91.7% | 96.3% | 93.7% | 4.4% |
| 0.40 | 91.7% | 96.0% | 94.0% | 4.4% |
| 0.50 | 91.2% | 94.2% | 95.2% | 2.9% |
| 0.60 | 89.2% | 89.9% | 97.1% | 1.5% |
| 0.65 | 87.6% | 87.8% | 97.3% | 1.5% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 86.8% [76.7–92.9] (n=68) |
|   └ 올바른 카테고리에 저장 | 92.9% [83.0–97.2] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 58.3% [32.0–80.7] (n=12) |
| 계층 F1 | 0.963 [0.925–1.000] (n=56) |
| 초안이 검색에 안 보임 | 100.0% [93.6–100.0] (n=56) |
| 게시 성공 (버전 +1) | 100.0% [93.6–100.0] (n=56) |
| 초안→게시 카테고리 유지 | 100.0% [93.6–100.0] (n=56) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 98.2% [90.6–99.7] (n=56) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 7 |
| 게시 후 probe 검색 top-k 포함 | 91.3% [73.2–97.6] (n=23) |
| 게시 후 probe 검색 top1 답변 | 91.3% [73.2–97.6] (n=23) |
| 지연 p50 (초안+게시+probe) | 3.0s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| null | money | 3 |
| food | food_cook | 2 |
| food_store_item | food_store_freeze | 2 |
| money_bank_acct_limit | money_bank_xfer_cap | 2 |
| null | it | 2 |
| null | life | 2 |
| work_leave_event | work_pay | 2 |
| food_tool | food_tool_pan | 1 |
| food_tool_pan | life_appl_kitchen | 1 |
| it_office_excel_func | it | 1 |
| it_office_excel_func | work | 1 |
| it_pc_peri_monitor | it_pc_win | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| it_sec_phish | money | 1 |
| life | life_home | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| life_appl_wash_dryer_shrink | life_clean | 1 |
| life_appl_wash_wm_drum_care_gasket | life_clean_mold | 1 |
| life_appl_wash_wm_drum_care_smell | life_clean_towel | 1 |
| money_bank_cert | it_sec | 1 |

### 틀린 케이스 (65건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-IT-003 | How do I turn on Bluetooth on my Windows laptop when the toggle is missing? | it_pc_peri_bt | it_pc_win | branch |
| I-MONEY-006 | 휴면 예금으로 넘어간 돈은 영영 사라지나요? | money_bank_acct_dormant | money_save_deposit | branch |
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit | false_tree |
| I-SHOP-007 | 호텔 예약을 취소하면 수수료가 얼마나 나오나요? | null | shop_order_cancel | false_tree |
| I-WORK-001 | 사원증을 잃어버렸는데 재발급은 어떻게 받나요? | work_hr_onboard | work_hr | shallow |
| I-XD-002 | 체크카드로 결제한 주문을 취소하면 환불은 며칠 걸리나요? | shop_ret_refund_card | shop_order_cancel | branch |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_ret_refund | false_tree |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food_cook · top food_cook#article 0.58 alternative | route deep, wrong top1 |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.47 alternative | wrong top1 |
| S-FOOD-012 | 스테인리스 냄비 바닥에 까맣게 탄 자국은 어떻게 벗겨요? | food_tool · no answer | food_tool_pan · abstained · top food_tool_pan#2 0.04 reference | route deep |
| S-FOOD-013 | 그거 처음에 어떻게 해야 하는 건데요? | food_tool_pan | life_appl_kitchen · abstained · top life_appl_kitchen#article 0.13 reference | route domain, abstained |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.79 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.87 recommended | route branch, wrong top1 |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it_mobile · abstained · top it_mobile_batt_drain#1 0.06 reference | route false_tree |
| S-GAP-010 | 중고거래로 물건값을 보냈는데 상대가 잠적했어요. 사기 신고는 어디에 하나요? | null · no answer | money · abstained · top money_bank_xfer_wrong#2 0.21 reference | route false_tree |
| S-GAP-011 | 임대차 계약갱신청구권을 쓰려면 집주인에게 언제 말해야 하나요? | null · no answer | life · abstained · top life_appl_wash_dryer#article 0.14 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it_sec · abstained · top it_sec_hacked#3 0.05 reference | route false_tree |
| S-IT-031 | 엄마라면서 폰 액정이 깨졌다고 상품권 좀 사 달라는 문자가 왔는데 이상해요 | it_sec_phish | money · top money_bank_xfer_cap#2 0.35 reference | route domain, wrong top1 |
| S-IT-035 | 듀얼 모니터 쓰는데 작업 표시줄을 한쪽 화면에만 나오게 할 수 있나요? | it_pc_peri_monitor · no answer | it_pc_win · abstained · top it_pc_win_keys#win_p 0.14 reference | route branch |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · top work_leave_annual_left#1 0.30 reference | route domain, wrong top1 |
| S-IT-531 | 두 번째, 세 번째로 큰 값을 구하는 함수는? | it_office_excel_func | it · top it_office_excel_func#large 0.95 recommended | route shallow |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell | life_clean_towel · top life_clean_towel#1 0.59 alternative | route branch, wrong top1 |
| S-LIFE-004 | 드럼 고무패킹 곰팡이 | life_appl_wash_wm_drum_care_gasket | life_clean_mold · abstained · top life_clean_mold#2 0.20 reference | route branch, abstained |
| S-LIFE-008 | 건조기 돌리면 안 되고 그냥 널어서 말려야 하는 옷은 어떤 거예요? | life_appl_wash_dryer_shrink | life_clean · abstained · top life_clean_towel#2 0.21 reference | route branch, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.63 alternative | route shallow |
| S-LIFE-020 | 자취 시작하고 집안일이 처음이라 뭐가 고장 나거나 지저분해지면 어디서부터 봐야 할지 모르겠어요 | life | life_home · top life_home#article 0.84 recommended | route deep, wrong top1 |
| S-MONEY-002 | 통장 만든 지 일주일 됐는데 하루에 30만 원까지만 보내져요. 앱에서 이체 한도 올리기를 눌러 봐도 안 되는데 이거 계좌 자체가 묶인 건가요? | money_bank_acct_limit | money_bank_xfer_cap · top money_bank_xfer_cap#3 0.55 alternative | route branch, wrong top1 |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec_phish#3 0.08 reference | route domain, abstained |
| S-MONEY-021 | 제 명의 신용카드 관련해서 이것저것 궁금한 게 있는데요 | money_card | money_card · top money_card_lost#4 0.72 recommended | wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.87 recommended | route domain, wrong top1 |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit | money_bank_xfer_cap · top money_bank_xfer_cap#1 0.41 alternative | route branch, wrong top1 |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card_limit#2 0.08 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.24 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · top money_save#article 0.54 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health_aid_emerg#article 0.09 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_card_lost#4 0.20 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.09 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money_save · abstained · top money_save#article 0.07 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_wm_drum_care_smell#1 0.21 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it · abstained · top it_mobile_batt_drain#2 0.22 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym_stomach · abstained · top health_sym_stomach#3 0.17 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.11 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_track#3 0.14 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.35 reference | route domain, wrong top1 |
| S-SHOP-009 | 카드로 산 운동화를 반품했는데 카드사 앱 승인 내역에 아직 취소가 안 보여요. 보통 며칠 걸려요? | shop_ret_refund_card | money_card · abstained · top money_card#article 0.07 reference | route domain, abstained |
| S-SHOP-017 | 온라인으로 산 물건 때문에 문의하려는데 뭐부터 봐야 할지 모르겠어요 | shop | shop_ret · top shop_ret#article 0.77 recommended | route deep, wrong top1 |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.46 alternative | wrong top1 |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money_tax · abstained · top money_tax#article 0.18 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.51 alternative | route branch, wrong top1 |
| S-TRAVEL-018 | 해외 나가기 전에 서류나 폰 쪽으로 챙겨야 할 게 뭐가 있죠? | travel_prep | travel_prep_data · top travel_prep_data#3 0.58 alternative | route deep, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.75 recommended | wrong top1 |
| S-WORK-008 | 연말정산 공제 얘기 말고요, 매달 월급에서 떼가는 건강보험이랑 소득세 항목이 뭔지 알고 싶어요 | work_pay_slip | money_tax · abstained · top money_tax#article 0.19 reference | route domain, abstained |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work_leave · top work_leave_annual_use#2 0.43 alternative | route shallow |
| S-WORK-021 | 경조사 생기면 회사에서 화환이나 조화도 보내주나요? | work_leave_event · no answer | work · top work_leave_event#2 0.43 alternative | route shallow, answered unanswerable |
| S-WORK-023 | 그 돈은 어디서 신청해요? | work_leave_event | work_pay · abstained · top work_pay_slip#2 0.14 reference | route branch, abstained |
| S-WORK-032 | 경조금 받으려면 증빙으로 뭘 올려야 하죠? | work_leave_event | work_pay · abstained · top work_pay_slip#4 0.08 reference | route branch, abstained |
| S-XD-003 | 쇼핑몰 주문을 카드 결제 후 바로 취소했는데 카드 명세서에는 아직 청구돼 있어요 | shop_ret_refund_card | shop_order_cancel · abstained · top shop_order_cancel#3 0.19 reference | route branch, abstained |
| S-XD-014 | 여행용 eSIM을 샀는데 폰에 어떻게 설치해서 써요? | travel_prep_data | it_mobile · abstained · top it_mobile#article 0.09 reference | route domain, abstained |

## jev · standard

### 검색 (444건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 91.9% [89.0–94.1] (n=444) |
|   └ 답할 수 있는 질문 | 91.0% [87.6–93.5] (n=376) |
|   └ 답할 수 없는 질문(보류가 정답) | 97.1% [89.9–99.2] (n=68) |
| 확신 정답률 (top1 정답 + recommended) | 86.4% [82.6–89.5] (n=376) |
| 라우팅 정확도 (정확 일치) | 89.2% [86.0–91.7] (n=444) |
| 계층 F1 (hF1) | 0.920 [0.896–0.943] (n=444) |
| 트리 밖 판정 precision | 100.0% [84.5–100.0] (n=21) |
| 트리 밖 판정 recall | 55.3% [39.7–69.9] (n=38) |
| 트리 밖 판정 F1 | 0.712 |
| 오보류율 (답할 수 있는데 보류) | 3.2% [1.8–5.5] (n=376) |
| 오답변율 (답할 수 없는데 답함) | 2.9% [0.8–10.1] (n=68) |
| Hit@1 | 91.2% [87.9–93.7] (n=376) |
| Hit@3 | 91.8% [88.5–94.1] (n=376) |
| Hit@k (limit) | 92.6% [89.4–94.8] (n=376) |
| MRR@k | 0.917 [0.889–0.944] (n=376) |
| nDCG@k | 0.908 [0.881–0.935] (n=376) |
| Recall@k | 0.914 [0.886–0.941] (n=376) |
| recommended 정밀도 | 0.961 [0.942–0.980] (n=335) |
| 라우팅이 맞았을 때 Hit@1 | 98.6% [96.7–99.4] (n=347) |
| 라우팅이 맞았을 때 MRR | 0.990 [0.982–0.999] (n=347) |
| 지연 p50 / p90 / p95 / 평균 | 1.7s / 2.8s / 3.3s / 1.8s |
| 라우팅 구간 평균 | 1.2s |
| 랭킹 후보 수 평균 | 16.0 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 4.13 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 97.5% (n=359) | 94.8% (n=349) | 92.7% (n=330) | 91.5% (n=82) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 17 | 7 | 9 | 455 | 22 | 2 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 1.0s |
| internal | 23 | 56.5% | 73.9% | 56.5% | 0.616 | 1.8s |
| leaf | 353 | 93.2% | 93.5% | 93.5% | 0.936 | 1.7s |
| no_answer | 27 | 100.0% | 92.6% | - | - | 1.5s |
| oos | 38 | 94.7% | 55.3% | - | - | 0.7s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 93.3% | 92.4% | 93.2% | 0.932 | 1.7s |
| single | 339 | 91.4% | 88.2% | 90.5% | 0.911 | 1.6s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 339 | 91.4% | 88.2% | 90.5% | 0.911 | 1.6s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 1.8s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 1.8s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.6s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.7s |
| ellipsis | 9 | 88.9% | 88.9% | 88.9% | 0.889 | 1.8s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.8s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.7s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 1.6s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.7s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 384 | 91.4% | 88.5% | 90.9% | 0.914 | 1.7s |
| root | 43 | 93.0% | 90.7% | 92.9% | 0.929 | 0.9s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 70.6% | 79.4% | 56.5% | 0.616 | 1.7s |
| leaf | 372 | 93.5% | 93.5% | 93.5% | 0.936 | 1.7s |
| none | 38 | 94.7% | 55.3% | - | - | 0.7s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 94.7% | 55.3% | - | - | 0.7s |
| d1 | 10 | 60.0% | 60.0% | 60.0% | 0.600 | 1.8s |
| d2 | 30 | 80.0% | 90.0% | 68.4% | 0.746 | 1.5s |
| d3 | 278 | 93.2% | 93.9% | 93.1% | 0.933 | 1.6s |
| d4 | 88 | 94.3% | 92.0% | 94.1% | 0.941 | 2.1s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 95.1% | 58.5% | - | - | 0.9s |
| 101+ | 4 | 50.0% | 50.0% | 50.0% | 0.500 | 4.4s |
| 2-5 | 319 | 93.1% | 93.1% | 93.1% | 0.933 | 1.6s |
| 33-100 | 61 | 91.8% | 91.8% | 91.5% | 0.915 | 2.8s |
| 6-32 | 19 | 73.7% | 89.5% | 50.0% | 0.592 | 1.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 2.4s |
| colloquial | 69 | 95.7% | 97.1% | 95.4% | 0.962 | 1.6s |
| condition | 32 | 87.5% | 87.5% | 90.3% | 0.903 | 1.7s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 1.8s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 1.4s |
| keyword | 20 | 85.0% | 85.0% | 85.0% | 0.850 | 1.7s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.9s |
| mixed | 13 | 92.3% | 92.3% | 92.3% | 0.923 | 1.7s |
| multi_intent | 9 | 100.0% | 88.9% | 100.0% | 1.000 | 1.5s |
| negation | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.9s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.3s |
| oos_near | 15 | 93.3% | 0.0% | - | - | 1.7s |
| paraphrase | 177 | 93.8% | 92.7% | 92.8% | 0.928 | 1.6s |
| typo | 13 | 92.3% | 92.3% | 92.3% | 0.923 | 1.7s |
| vague | 23 | 56.5% | 73.9% | 56.5% | 0.616 | 1.8s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 89.1% | 89.1% | 88.4% | 0.890 | 1.9s |
| health | 40 | 97.5% | 100.0% | 97.2% | 0.977 | 1.5s |
| it | 101 | 96.0% | 95.0% | 95.8% | 0.958 | 1.9s |
| life | 41 | 90.2% | 87.8% | 89.2% | 0.892 | 1.8s |
| money | 49 | 91.8% | 95.9% | 93.5% | 0.940 | 1.6s |
| oos | 38 | 94.7% | 55.3% | - | - | 0.7s |
| shop | 41 | 75.6% | 78.0% | 73.7% | 0.750 | 1.5s |
| travel | 44 | 95.5% | 95.5% | 94.9% | 0.949 | 1.6s |
| work | 44 | 90.9% | 93.2% | 90.2% | 0.915 | 1.7s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 94.9% | 96.6% | 94.5% | 0.955 | 1.6s |
| hard | 124 | 91.9% | 80.6% | 91.8% | 0.918 | 1.6s |
| medium | 202 | 90.1% | 90.1% | 89.0% | 0.893 | 1.7s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 303 | 91.7% | 87.8% | 90.7% | 0.910 | 1.7s |
| test | 141 | 92.2% | 92.2% | 92.4% | 0.931 | 1.6s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 91.2% | 98.4% | 91.2% | 8.8% |
| 0.25 | 91.9% | 97.3% | 93.0% | 4.4% |
| 0.30 | 91.9% | 96.8% | 93.4% | 2.9% |
| 0.35 | 91.9% | 96.5% | 93.7% | 2.9% |
| 0.40 | 91.9% | 96.0% | 94.2% | 2.9% |
| 0.50 | 91.0% | 93.9% | 95.2% | 2.9% |
| 0.60 | 89.4% | 90.7% | 96.5% | 1.5% |
| 0.65 | 88.3% | 88.8% | 97.0% | 1.5% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 86.8% [76.7–92.9] (n=68) |
|   └ 올바른 카테고리에 저장 | 92.9% [83.0–97.2] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 58.3% [32.0–80.7] (n=12) |
| 계층 F1 | 0.965 [0.929–1.000] (n=56) |
| 초안이 검색에 안 보임 | 100.0% [93.6–100.0] (n=56) |
| 게시 성공 (버전 +1) | 100.0% [93.6–100.0] (n=56) |
| 초안→게시 카테고리 유지 | 100.0% [93.6–100.0] (n=56) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 98.2% [90.6–99.7] (n=56) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 7 |
| 게시 후 probe 검색 top-k 포함 | 91.3% [73.2–97.6] (n=23) |
| 게시 후 probe 검색 top1 답변 | 91.3% [73.2–97.6] (n=23) |
| 지연 p50 (초안+게시+probe) | 2.8s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| null | it | 3 |
| null | money | 3 |
| food | food_cook | 2 |
| food_store_item | food_store_freeze | 2 |
| null | life | 2 |
| food_tool | food_tool_pan | 1 |
| it_office_excel_func | work | 1 |
| it_pc_peri_monitor | it_pc_win | 1 |
| it_pc_win_perf_disk | it_mobile_space | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| it_sec_phish | money | 1 |
| life | life_home | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| life_appl_wash_dryer_shrink | life_clean | 1 |
| life_appl_wash_wm_drum_care_gasket | life_clean_mold | 1 |
| life_appl_wash_wm_drum_care_smell | life_clean_towel | 1 |
| money_bank_cert | it_sec | 1 |
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | health | 1 |

### 틀린 케이스 (63건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-IT-003 | How do I turn on Bluetooth on my Windows laptop when the toggle is missing? | it_pc_peri_bt | it_pc_win | branch |
| I-MONEY-006 | 휴면 예금으로 넘어간 돈은 영영 사라지나요? | money_bank_acct_dormant | money_save_deposit | branch |
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit | false_tree |
| I-SHOP-007 | 호텔 예약을 취소하면 수수료가 얼마나 나오나요? | null | shop_order_cancel | false_tree |
| I-WORK-001 | 사원증을 잃어버렸는데 재발급은 어떻게 받나요? | work_hr_onboard | work_hr | shallow |
| I-XD-002 | 체크카드로 결제한 주문을 취소하면 환불은 며칠 걸리나요? | shop_ret_refund_card | shop_order_cancel | branch |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_ret | false_tree |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food_cook · top food_cook#article 0.62 alternative | route deep, wrong top1 |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.46 alternative | wrong top1 |
| S-FOOD-012 | 스테인리스 냄비 바닥에 까맣게 탄 자국은 어떻게 벗겨요? | food_tool · no answer | food_tool_pan · abstained · top food_tool_pan#2 0.04 reference | route deep |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.80 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.88 recommended | route branch, wrong top1 |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it_mobile · abstained · top it_mobile_batt_drain#1 0.05 reference | route false_tree |
| S-GAP-010 | 중고거래로 물건값을 보냈는데 상대가 잠적했어요. 사기 신고는 어디에 하나요? | null · no answer | money · abstained · top money_bank_xfer_wrong#2 0.24 reference | route false_tree |
| S-GAP-011 | 임대차 계약갱신청구권을 쓰려면 집주인에게 언제 말해야 하나요? | null · no answer | life · abstained · top life_appl_wash_dryer_slow#2 0.15 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it · abstained · top it_pc_win_perf_slow#1 0.19 reference | route false_tree |
| S-HEALTH-017 | 집에서 다쳤을 때 쓸 응급처치를 종류별로 한번 정리해 두고 싶어요 | health_aid | health_aid · top health_aid_injury_sprain#1 0.68 recommended | wrong top1 |
| S-IT-005 | 씨드라이브가 빨강색으로 바꼇는데 어떻게 비워요 | it_pc_win_perf_disk | it_mobile_space · top it_mobile_space#1 0.49 alternative | route branch, wrong top1 |
| S-IT-031 | 엄마라면서 폰 액정이 깨졌다고 상품권 좀 사 달라는 문자가 왔는데 이상해요 | it_sec_phish | money · abstained · top money_bank_xfer_cap#2 0.25 reference | route domain, abstained |
| S-IT-035 | 듀얼 모니터 쓰는데 작업 표시줄을 한쪽 화면에만 나오게 할 수 있나요? | it_pc_peri_monitor · no answer | it_pc_win · abstained · top it_pc_win_keys#alt_space 0.19 reference | route branch |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · abstained · top work_leave_event#1 0.23 reference | route domain, abstained |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell | life_clean_towel · top life_clean_towel#1 0.57 alternative | route branch, wrong top1 |
| S-LIFE-004 | 드럼 고무패킹 곰팡이 | life_appl_wash_wm_drum_care_gasket | life_clean_mold · abstained · top life_clean_mold#2 0.20 reference | route branch, abstained |
| S-LIFE-008 | 건조기 돌리면 안 되고 그냥 널어서 말려야 하는 옷은 어떤 거예요? | life_appl_wash_dryer_shrink | life_clean · abstained · top life_clean_towel#2 0.19 reference | route branch, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_ac#2 0.68 recommended | route shallow |
| S-LIFE-020 | 자취 시작하고 집안일이 처음이라 뭐가 고장 나거나 지저분해지면 어디서부터 봐야 할지 모르겠어요 | life | life_home · top life_home#article 0.84 recommended | route deep, wrong top1 |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec_2fa#2 0.08 reference | route domain, abstained |
| S-MONEY-021 | 제 명의 신용카드 관련해서 이것저것 궁금한 게 있는데요 | money_card | money_card · top money_card_lost#4 0.70 recommended | wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.86 recommended | route domain, wrong top1 |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card_limit#2 0.09 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.26 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · top money_save#article 0.57 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health#article 0.08 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_card_lost#4 0.22 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.07 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money_save · abstained · top money_save#article 0.09 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_wm_top_clean#2 0.13 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.21 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym_stomach · abstained · top health_sym_stomach#3 0.15 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.14 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_addr#2 0.14 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.36 reference | route domain, wrong top1 |
| S-SHOP-009 | 카드로 산 운동화를 반품했는데 카드사 앱 승인 내역에 아직 취소가 안 보여요. 보통 며칠 걸려요? | shop_ret_refund_card | money_card · abstained · top money_card_limit#2 0.09 reference | route domain, abstained |
| S-SHOP-017 | 온라인으로 산 물건 때문에 문의하려는데 뭐부터 봐야 할지 모르겠어요 | shop | shop_ret · top shop_ret#article 0.76 recommended | route deep, wrong top1 |
| S-SHOP-018 | 받은 물건이 맘에 안 들어서 처리하고 싶은데, 돌려보내는 거랑 바꾸는 거 중에 어떤 선택지가 있는지부터 알려 주세요 | shop_ret | shop_ret_return_mind · top shop_ret_return_mind#1 0.40 reference | route deep, wrong top1 |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.46 alternative | wrong top1 |
| S-SHOP-024 | 무통장으로 낸 건요? | shop_ret_refund_bank | shop_order_pay · top shop_order_pay#3 0.31 reference | route branch, wrong top1 |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money_tax · abstained · top money_tax#article 0.21 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.50 alternative | route branch, wrong top1 |
| S-TRAVEL-018 | 해외 나가기 전에 서류나 폰 쪽으로 챙겨야 할 게 뭐가 있죠? | travel_prep | travel_prep_data · top travel_prep_data#1 0.55 alternative | route deep, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.73 recommended | wrong top1 |
| S-WORK-008 | 연말정산 공제 얘기 말고요, 매달 월급에서 떼가는 건강보험이랑 소득세 항목이 뭔지 알고 싶어요 | work_pay_slip | money_tax · abstained · top money_tax#article 0.19 reference | route domain, abstained |
| S-WORK-032 | 경조금 받으려면 증빙으로 뭘 올려야 하죠? | work_leave_event | work_pay · abstained · top work_pay#article 0.07 reference | route branch, abstained |
| S-XD-003 | 쇼핑몰 주문을 카드 결제 후 바로 취소했는데 카드 명세서에는 아직 청구돼 있어요 | shop_ret_refund_card | shop_order_cancel · abstained · top shop_order_cancel#3 0.20 reference | route branch, abstained |
| S-XD-014 | 여행용 eSIM을 샀는데 폰에 어떻게 설치해서 써요? | travel_prep_data | it_mobile · abstained · top it_mobile#article 0.10 reference | route domain, abstained |
| S-XD-016 | 미국에 사는 동생한테 돈을 보내려는데 카드로도 보낼 수 있어요? 뭐가 필요해요? | money_bank_xfer_abroad | money_bank_xfer_abroad · abstained · top money_bank_xfer_abroad#1 0.30 reference | abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.59 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.78 recommended | route branch, wrong top1 |

## jev · shallow

### 검색 (400건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 95.5% [93.0–97.1] (n=400) |
|   └ 답할 수 있는 질문 | 95.4% [92.6–97.1] (n=345) |
|   └ 답할 수 없는 질문(보류가 정답) | 96.4% [87.7–99.0] (n=55) |
| 확신 정답률 (top1 정답 + recommended) | 89.3% [85.6–92.1] (n=345) |
| 라우팅 정확도 (정확 일치) | 92.2% [89.2–94.5] (n=400) |
| 계층 F1 (hF1) | 0.929 [0.904–0.953] (n=400) |
| 트리 밖 판정 precision | 100.0% [83.2–100.0] (n=19) |
| 트리 밖 판정 recall | 52.8% [37.0–68.0] (n=36) |
| 트리 밖 판정 F1 | 0.691 |
| 오보류율 (답할 수 있는데 보류) | 1.4% [0.6–3.3] (n=345) |
| 오답변율 (답할 수 없는데 답함) | 3.6% [1.0–12.3] (n=55) |
| Hit@1 | 95.4% [92.6–97.1] (n=345) |
| Hit@3 | 95.9% [93.3–97.6] (n=345) |
| Hit@k (limit) | 95.9% [93.3–97.6] (n=345) |
| MRR@k | 0.957 [0.935–0.978] (n=345) |
| nDCG@k | 0.943 [0.921–0.965] (n=345) |
| Recall@k | 0.945 [0.923–0.967] (n=345) |
| recommended 정밀도 | 0.981 [0.966–0.996] (n=314) |
| 라우팅이 맞았을 때 Hit@1 | 99.4% [97.8–99.8] (n=331) |
| 라우팅이 맞았을 때 MRR | 0.997 [0.993–1.000] (n=331) |
| 지연 p50 / p90 / p95 / 평균 | 1.3s / 2.4s / 2.7s / 1.5s |
| 라우팅 구간 평균 | 0.8s |
| 랭킹 후보 수 평균 | 17.2 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 3.18 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 |
|---|---:|---:|
| 일치율 | 97.4% (n=344) | 95.5% (n=334) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_tree |
|---:|---:|---:|---:|
| 5 | 9 | 423 | 22 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.7s |
| internal | 10 | 90.0% | 100.0% | 90.0% | 0.950 | 2.5s |
| leaf | 335 | 95.5% | 95.8% | 95.5% | 0.957 | 1.3s |
| no_answer | 16 | 100.0% | 100.0% | - | - | 1.2s |
| oos | 36 | 94.4% | 52.8% | - | - | 0.9s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 94.3% | 93.3% | 94.2% | 0.942 | 1.2s |
| single | 295 | 95.9% | 91.9% | 95.9% | 0.963 | 1.3s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 295 | 95.9% | 91.9% | 95.9% | 0.963 | 1.3s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 1.3s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 1.3s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.2s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.3s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.4s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 1.1s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.1s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 369 | 95.4% | 92.1% | 95.1% | 0.954 | 1.3s |
| root | 31 | 96.8% | 93.5% | 100.0% | 1.000 | 0.6s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 10 | 90.0% | 100.0% | 90.0% | 0.950 | 2.5s |
| leaf | 354 | 95.8% | 96.0% | 95.5% | 0.957 | 1.3s |
| none | 36 | 94.4% | 52.8% | - | - | 0.9s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 36 | 94.4% | 52.8% | - | - | 0.9s |
| d1 | 10 | 90.0% | 100.0% | 90.0% | 0.950 | 2.5s |
| d2 | 354 | 95.8% | 96.0% | 95.5% | 0.957 | 1.3s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 39 | 94.9% | 56.4% | - | - | 0.9s |
| 101+ | 4 | 75.0% | 100.0% | 75.0% | 0.875 | 5.1s |
| 2-5 | 301 | 96.0% | 96.3% | 95.8% | 0.960 | 1.2s |
| 33-100 | 56 | 94.6% | 94.6% | 94.5% | 0.945 | 2.2s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.6s |
| colloquial | 66 | 98.5% | 100.0% | 98.4% | 0.992 | 1.2s |
| condition | 32 | 93.8% | 93.8% | 93.5% | 0.935 | 1.4s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 1.4s |
| keyword | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.2s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.4s |
| mixed | 13 | 92.3% | 92.3% | 92.3% | 0.923 | 1.3s |
| multi_intent | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| negation | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 1.4s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.4s |
| oos_near | 15 | 93.3% | 0.0% | - | - | 1.9s |
| paraphrase | 149 | 94.0% | 93.3% | 93.5% | 0.935 | 1.2s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 1.3s |
| vague | 10 | 90.0% | 100.0% | 90.0% | 0.950 | 2.5s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 40 | 92.5% | 95.0% | 92.3% | 0.936 | 1.5s |
| health | 36 | 100.0% | 100.0% | 100.0% | 1.000 | 1.3s |
| it | 93 | 96.8% | 96.8% | 96.7% | 0.967 | 1.5s |
| life | 36 | 100.0% | 100.0% | 100.0% | 1.000 | 1.3s |
| money | 44 | 95.5% | 95.5% | 95.2% | 0.952 | 1.2s |
| oos | 36 | 94.4% | 52.8% | - | - | 0.9s |
| shop | 36 | 88.9% | 88.9% | 88.2% | 0.882 | 1.2s |
| travel | 39 | 97.4% | 97.4% | 97.1% | 0.971 | 1.2s |
| work | 40 | 92.5% | 95.0% | 92.1% | 0.934 | 1.2s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 109 | 97.2% | 98.2% | 97.0% | 0.975 | 1.2s |
| hard | 112 | 92.9% | 82.1% | 92.9% | 0.929 | 1.2s |
| medium | 179 | 96.1% | 95.0% | 95.6% | 0.959 | 1.3s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 271 | 95.6% | 91.5% | 95.8% | 0.960 | 1.2s |
| test | 129 | 95.3% | 93.8% | 94.5% | 0.950 | 1.3s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 94.8% | 98.6% | 95.4% | 9.1% |
| 0.25 | 95.5% | 98.6% | 96.2% | 3.6% |
| 0.30 | 95.5% | 98.6% | 96.2% | 3.6% |
| 0.35 | 95.5% | 97.7% | 97.0% | 1.8% |
| 0.40 | 95.2% | 97.1% | 97.3% | 1.8% |
| 0.50 | 94.5% | 96.2% | 97.3% | 1.8% |
| 0.60 | 91.8% | 92.8% | 97.5% | 1.8% |
| 0.65 | 90.5% | 90.7% | 98.1% | 1.8% |

### 저장 (ingest, 59건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 91.5% [81.6–96.3] (n=59) |
|   └ 올바른 카테고리에 저장 | 100.0% [92.7–100.0] (n=49) |
|   └ 트리/루트 밖 Q&A 거절 | 50.0% [23.7–76.3] (n=10) |
| 계층 F1 | 1.000 [1.000–1.000] (n=49) |
| 초안이 검색에 안 보임 | 100.0% [92.7–100.0] (n=49) |
| 게시 성공 (버전 +1) | 100.0% [92.7–100.0] (n=49) |
| 초안→게시 카테고리 유지 | 100.0% [92.7–100.0] (n=49) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [92.7–100.0] (n=49) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 89.5% [68.6–97.1] (n=19) |
| 게시 후 probe 검색 top1 답변 | 89.5% [68.6–97.1] (n=19) |
| 지연 p50 (초안+게시+probe) | 2.1s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| null | it | 4 |
| food_store_item | food_store_freeze | 2 |
| null | life | 2 |
| null | money | 2 |
| it_office_excel_func | work | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| it_sec_phish | money_bank_xfer_wrong | 1 |
| money_bank_cert | it_sec_2fa | 1 |
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | food_safe_poison | 1 |
| null | health | 1 |
| null | health_sym_stomach | 1 |
| null | money_bank_xfer_wrong | 1 |
| null | money_card_lost | 1 |
| null | money_save_etf | 1 |
| null | shop | 1 |
| null | travel | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_order_receipt | money | 1 |

### 틀린 케이스 (38건 중 38건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | life | false_tree |
| I-SHOP-007 | 호텔 예약을 취소하면 수수료가 얼마나 나오나요? | null | shop_order_cancel | false_tree |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_order_cancel | false_tree |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food · top food_safe_poison#2 0.60 alternative | wrong top1 |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.81 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.88 recommended | route branch, wrong top1 |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.23 reference | route false_tree |
| S-GAP-010 | 중고거래로 물건값을 보냈는데 상대가 잠적했어요. 사기 신고는 어디에 하나요? | null · no answer | money_bank_xfer_wrong · abstained · top money_bank_xfer_wrong#2 0.22 reference | route false_tree |
| S-GAP-011 | 임대차 계약갱신청구권을 쓰려면 집주인에게 언제 말해야 하나요? | null · no answer | life · abstained · top life_appl_wash_wm_drum_water_drain#2 0.14 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it · abstained · top it#article 0.17 reference | route false_tree |
| S-IT-031 | 엄마라면서 폰 액정이 깨졌다고 상품권 좀 사 달라는 문자가 왔는데 이상해요 | it_sec_phish | money_bank_xfer_wrong · abstained · top money_bank_xfer_wrong#2 0.07 reference | route domain, abstained |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · top work_exp_claim#3 0.34 reference | route domain, wrong top1 |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec_2fa · abstained · top it_sec_2fa#2 0.04 reference | route domain, abstained |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.87 recommended | route domain, wrong top1 |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · top it_sec_phish#2 0.33 reference | route false_tree, answered unanswerable |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money_save_etf · abstained · top money_save_etf#2 0.09 reference | route false_tree |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health#article 0.08 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_tax_yearend#4 0.14 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel · abstained · top travel_local_metro#3 0.15 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money · abstained · top money_loan_credit#1 0.15 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life#article 0.11 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it · abstained · top it_mobile_lost#2 0.23 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym_stomach · abstained · top health_sym_stomach#3 0.16 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.17 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_addr#2 0.15 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.36 reference | route domain, wrong top1 |
| S-SHOP-009 | 카드로 산 운동화를 반품했는데 카드사 앱 승인 내역에 아직 취소가 안 보여요. 보통 며칠 걸려요? | shop_ret_refund_card | money · abstained · top money_tax_yearend#2 0.13 reference | route domain, abstained |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money · abstained · top money_bank_acct_limit#3 0.11 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.57 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.74 recommended | wrong top1 |
| S-WORK-008 | 연말정산 공제 얘기 말고요, 매달 월급에서 떼가는 건강보험이랑 소득세 항목이 뭔지 알고 싶어요 | work_pay_slip | money_tax_income · abstained · top money_tax_income#2 0.08 reference | route domain, abstained |
| S-XD-014 | 여행용 eSIM을 샀는데 폰에 어떻게 설치해서 써요? | travel_prep_data | it · top it#article 0.35 reference | route domain, wrong top1 |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.62 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.84 recommended | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.84 recommended | route false_tree, answered unanswerable |
| S-XD-052 | 상한 음식을 먹고 계속 토해요. 어떻게 해야 돼요? | null · no answer | food_safe_poison · abstained · top food_safe_poison#2 0.15 reference | route false_tree |

## jev · flat

### 검색 (369건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 97.3% [95.1–98.5] (n=369) |
|   └ 답할 수 있는 질문 | 96.9% [94.4–98.3] (n=325) |
|   └ 답할 수 없는 질문(보류가 정답) | 100.0% [92.0–100.0] (n=44) |
| 확신 정답률 (top1 정답 + recommended) | 91.1% [87.5–93.7] (n=325) |
| 라우팅 정확도 (정확 일치) | 97.0% [94.7–98.3] (n=369) |
| 계층 F1 (hF1) | 0.970 [0.953–0.988] (n=369) |
| 트리 밖 판정 precision | 100.0% [85.7–100.0] (n=23) |
| 트리 밖 판정 recall | 92.0% [75.0–97.8] (n=25) |
| 트리 밖 판정 F1 | 0.958 |
| 오보류율 (답할 수 있는데 보류) | 0.3% [0.1–1.7] (n=325) |
| 오답변율 (답할 수 없는데 답함) | 0.0% [0.0–8.0] (n=44) |
| Hit@1 | 97.2% [94.8–98.5] (n=325) |
| Hit@3 | 97.5% [95.2–98.7] (n=325) |
| Hit@k (limit) | 97.5% [95.2–98.7] (n=325) |
| MRR@k | 0.974 [0.957–0.991] (n=325) |
| nDCG@k | 0.960 [0.942–0.978] (n=325) |
| Recall@k | 0.960 [0.941–0.979] (n=325) |
| recommended 정밀도 | 0.982 [0.968–0.997] (n=301) |
| 라우팅이 맞았을 때 Hit@1 | 99.7% [98.2–99.9] (n=317) |
| 라우팅이 맞았을 때 MRR | 0.998 [0.995–1.000] (n=317) |
| 지연 p50 / p90 / p95 / 평균 | 1.0s / 2.0s / 2.6s / 1.2s |
| 라우팅 구간 평균 | 0.7s |
| 랭킹 후보 수 평균 | 11.8 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 2.12 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 |
|---|---:|
| 일치율 | 96.7% (n=334) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_tree |
|---:|---:|---:|---:|
| 4 | 5 | 408 | 2 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.5s |
| leaf | 325 | 96.9% | 97.5% | 97.2% | 0.974 | 1.0s |
| no_answer | 16 | 100.0% | 93.8% | - | - | 1.0s |
| oos | 25 | 100.0% | 92.0% | - | - | 0.5s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 94.3% | 94.3% | 94.2% | 0.942 | 1.0s |
| single | 264 | 98.5% | 98.1% | 98.6% | 0.989 | 1.0s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 264 | 98.5% | 98.1% | 98.6% | 0.989 | 1.0s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 1.0s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| disambig | 20 | 85.0% | 85.0% | 85.0% | 0.850 | 0.9s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.0s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.0s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.5s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| shift | 14 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 359 | 97.2% | 96.9% | 97.2% | 0.973 | 1.0s |
| root | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| leaf | 344 | 97.1% | 97.4% | 97.2% | 0.974 | 1.0s |
| none | 25 | 100.0% | 92.0% | - | - | 0.5s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 25 | 100.0% | 92.0% | - | - | 0.5s |
| d1 | 344 | 97.1% | 97.4% | 97.2% | 0.974 | 1.0s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 28 | 100.0% | 92.9% | - | - | 0.5s |
| 2-5 | 291 | 96.9% | 97.3% | 97.1% | 0.973 | 1.0s |
| 33-100 | 50 | 98.0% | 98.0% | 98.0% | 0.980 | 2.0s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| colloquial | 66 | 98.5% | 100.0% | 98.4% | 0.992 | 1.0s |
| condition | 31 | 93.5% | 96.8% | 96.8% | 0.968 | 1.2s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| keyword | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 1.0s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 1.0s |
| multi_intent | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.3s |
| negation | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 1.2s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.5s |
| oos_near | 15 | 100.0% | 86.7% | - | - | 0.6s |
| paraphrase | 129 | 95.3% | 94.6% | 94.7% | 0.947 | 1.0s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 1.3s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 37 | 97.3% | 97.3% | 97.2% | 0.972 | 1.3s |
| health | 33 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| it | 90 | 98.9% | 98.9% | 98.9% | 0.989 | 1.3s |
| life | 34 | 100.0% | 100.0% | 100.0% | 1.000 | 1.0s |
| money | 42 | 92.9% | 95.2% | 95.0% | 0.950 | 0.9s |
| oos | 25 | 100.0% | 92.0% | - | - | 0.5s |
| shop | 33 | 93.9% | 93.9% | 93.5% | 0.935 | 1.0s |
| travel | 37 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| work | 38 | 92.1% | 92.1% | 91.7% | 0.931 | 1.4s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 99 | 99.0% | 100.0% | 98.9% | 0.995 | 1.0s |
| hard | 107 | 91.6% | 89.7% | 90.5% | 0.905 | 1.0s |
| medium | 163 | 100.0% | 100.0% | 100.0% | 1.000 | 1.1s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 248 | 98.0% | 96.8% | 97.7% | 0.977 | 1.0s |
| test | 121 | 95.9% | 97.5% | 96.2% | 0.966 | 1.0s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 97.6% | 100.0% | 97.2% | 0.0% |
| 0.25 | 97.6% | 100.0% | 97.2% | 0.0% |
| 0.30 | 97.3% | 99.7% | 97.2% | 0.0% |
| 0.35 | 97.3% | 99.7% | 97.2% | 0.0% |
| 0.40 | 97.0% | 99.1% | 97.5% | 0.0% |
| 0.50 | 96.2% | 97.8% | 97.8% | 0.0% |
| 0.60 | 93.2% | 93.8% | 98.4% | 0.0% |
| 0.65 | 92.1% | 92.6% | 98.3% | 0.0% |

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
| 지연 p50 (초안+게시+probe) | 2.2s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (11건)

통과 100.0% [74.1–100.0] (n=11)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| money_card_lost | it_sec_hacked | 1 |
| money_tax_yearend | work_exp_claim | 1 |
| null | shop_member_leave | 1 |
| null | travel_prep_data | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_ret_refund_card | money_card_install | 1 |
| work_exp_card | money_card_limit | 1 |
| work_exp_trip | travel_air_ticket_mile | 1 |
| work_exp_trip | work_exp_claim | 1 |

### 틀린 케이스 (13건 중 13건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.88 recommended | route branch, wrong top1 |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | travel_prep_data · abstained · top travel_prep_data#3 0.11 reference | route false_tree |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost | it_sec_hacked · top it_sec_hacked#1 0.37 reference | route domain, wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.87 recommended | route domain, wrong top1 |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | shop_member_leave · abstained · top shop_member_leave#1 0.09 reference | route false_tree |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card | money_card_install · top money_card_install#3 0.41 alternative | route domain, wrong top1 |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.60 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.77 recommended | wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#2 0.07 reference | route domain |
| S-XD-016 | 미국에 사는 동생한테 돈을 보내려는데 카드로도 보낼 수 있어요? 뭐가 필요해요? | money_bank_xfer_abroad | money_bank_xfer_abroad · abstained · top money_bank_xfer_abroad#1 0.29 reference | abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.58 alternative | route branch, wrong top1 |
| S-XD-024 | 한도 좀 올리고 싶은데 어떻게 해요? | work_exp_card | money_card_limit · top money_card_limit#1 0.81 recommended | route domain, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.82 recommended | route branch, wrong top1 |

## jev · sparse

### 검색 (444건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 84.9% [81.3–87.9] (n=444) |
|   └ 답할 수 있는 질문 | 89.4% [84.1–93.1] (n=180) |
|   └ 답할 수 없는 질문(보류가 정답) | 81.8% [76.7–86.0] (n=264) |
| 확신 정답률 (top1 정답 + recommended) | 78.3% [71.8–83.7] (n=180) |
| 라우팅 정확도 (정확 일치) | 89.6% [86.5–92.1] (n=444) |
| 계층 F1 (hF1) | 0.924 [0.901–0.947] (n=444) |
| 트리 밖 판정 precision | 100.0% [84.5–100.0] (n=21) |
| 트리 밖 판정 recall | 55.3% [39.7–69.9] (n=38) |
| 트리 밖 판정 F1 | 0.712 |
| 오보류율 (답할 수 있는데 보류) | 6.1% [3.4–10.6] (n=180) |
| 오답변율 (답할 수 없는데 답함) | 18.2% [14.0–23.3] (n=264) |
| Hit@1 | 90.0% [84.7–93.6] (n=180) |
| Hit@3 | 90.0% [84.7–93.6] (n=180) |
| Hit@k (limit) | 90.6% [85.4–94.0] (n=180) |
| MRR@k | 0.901 [0.858–0.945] (n=180) |
| nDCG@k | 0.904 [0.861–0.947] (n=180) |
| Recall@k | 0.894 [0.851–0.938] (n=180) |
| recommended 정밀도 | 0.908 [0.863–0.953] (n=154) |
| 라우팅이 맞았을 때 Hit@1 | 97.0% [93.1–98.7] (n=166) |
| 라우팅이 맞았을 때 MRR | 0.971 [0.946–0.996] (n=166) |
| 지연 p50 / p90 / p95 / 평균 | 1.6s / 2.2s / 2.5s / 1.6s |
| 라우팅 구간 평균 | 1.2s |
| 랭킹 후보 수 평균 | 2.4 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 3.82 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 97.8% (n=359) | 95.4% (n=349) | 93.3% (n=330) | 92.7% (n=82) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 15 | 7 | 8 | 446 | 22 | 2 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dropped | 196 | 76.5% | 93.4% | - | - | 1.6s |
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 1.0s |
| internal | 23 | 69.6% | 73.9% | 69.6% | 0.704 | 1.5s |
| leaf | 157 | 92.4% | 94.9% | 93.0% | 0.930 | 1.7s |
| no_answer | 27 | 100.0% | 92.6% | - | - | 1.6s |
| oos | 38 | 94.7% | 55.3% | - | - | 0.5s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 81.9% | 92.4% | 92.9% | 0.929 | 1.6s |
| single | 339 | 85.8% | 88.8% | 89.1% | 0.893 | 1.6s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 339 | 85.8% | 88.8% | 89.1% | 0.893 | 1.6s |
| coref | 19 | 78.9% | 84.2% | 100.0% | 1.000 | 1.6s |
| correct | 8 | 75.0% | 87.5% | 80.0% | 0.800 | 1.4s |
| disambig | 20 | 80.0% | 90.0% | 90.0% | 0.900 | 1.7s |
| distract | 9 | 88.9% | 100.0% | 100.0% | 1.000 | 1.5s |
| ellipsis | 9 | 66.7% | 88.9% | 75.0% | 0.750 | 1.7s |
| long | 8 | 75.0% | 100.0% | 100.0% | 1.000 | 1.7s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.5s |
| shift | 14 | 92.9% | 92.9% | 100.0% | 1.000 | 1.7s |
| slot | 9 | 77.8% | 100.0% | 100.0% | 1.000 | 1.6s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 384 | 84.9% | 89.1% | 89.5% | 0.896 | 1.6s |
| root | 43 | 86.0% | 90.7% | 100.0% | 1.000 | 0.7s |
| start | 17 | 82.4% | 100.0% | 100.0% | 1.000 | 0.8s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 79.4% | 79.4% | 69.6% | 0.704 | 1.4s |
| leaf | 372 | 84.4% | 94.1% | 93.0% | 0.930 | 1.6s |
| none | 38 | 94.7% | 55.3% | - | - | 0.5s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 94.7% | 55.3% | - | - | 0.5s |
| d1 | 10 | 60.0% | 60.0% | 60.0% | 0.600 | 1.5s |
| d2 | 30 | 90.0% | 90.0% | 80.0% | 0.813 | 1.4s |
| d3 | 278 | 83.8% | 94.2% | 94.1% | 0.941 | 1.5s |
| d4 | 88 | 85.2% | 93.2% | 88.9% | 0.889 | 2.0s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 95.1% | 58.5% | - | - | 0.7s |
| 1 | 369 | 84.3% | 94.0% | 93.0% | 0.930 | 1.6s |
| 2-5 | 16 | 93.8% | 87.5% | 85.7% | 0.857 | 1.6s |
| 33-100 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 1.8s |
| 6-32 | 16 | 62.5% | 68.8% | 57.1% | 0.586 | 1.4s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 87.5% | 100.0% | - | - | 1.6s |
| colloquial | 69 | 84.1% | 97.1% | 96.7% | 0.967 | 1.5s |
| condition | 32 | 81.2% | 87.5% | 92.9% | 0.929 | 1.8s |
| english | 20 | 95.0% | 100.0% | 100.0% | 1.000 | 1.8s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 1.6s |
| keyword | 20 | 80.0% | 85.0% | 90.0% | 0.900 | 1.6s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.6s |
| mixed | 13 | 84.6% | 92.3% | 71.4% | 0.714 | 1.8s |
| multi_intent | 9 | 55.6% | 88.9% | 66.7% | 0.667 | 1.7s |
| negation | 20 | 75.0% | 90.0% | 87.5% | 0.875 | 1.8s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.4s |
| oos_near | 15 | 93.3% | 0.0% | - | - | 1.5s |
| paraphrase | 177 | 86.4% | 93.2% | 96.1% | 0.961 | 1.5s |
| typo | 13 | 92.3% | 100.0% | 100.0% | 1.000 | 1.6s |
| vague | 23 | 69.6% | 73.9% | 69.6% | 0.704 | 1.5s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 78.3% | 89.1% | 86.7% | 0.867 | 1.6s |
| health | 40 | 90.0% | 100.0% | 94.1% | 0.953 | 1.5s |
| it | 101 | 89.1% | 97.0% | 93.6% | 0.936 | 1.9s |
| life | 41 | 75.6% | 87.8% | 86.4% | 0.864 | 1.7s |
| money | 49 | 77.6% | 95.9% | 96.0% | 0.960 | 1.6s |
| oos | 38 | 94.7% | 55.3% | - | - | 0.5s |
| shop | 41 | 75.6% | 78.0% | 63.2% | 0.632 | 1.6s |
| travel | 44 | 90.9% | 95.5% | 93.8% | 0.938 | 1.4s |
| work | 44 | 88.6% | 93.2% | 100.0% | 1.000 | 1.3s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 90.7% | 97.5% | 98.0% | 0.980 | 1.5s |
| hard | 124 | 83.1% | 80.6% | 92.1% | 0.921 | 1.6s |
| medium | 202 | 82.7% | 90.6% | 84.6% | 0.848 | 1.6s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 303 | 83.2% | 88.1% | 90.7% | 0.909 | 1.6s |
| test | 141 | 88.7% | 92.9% | 88.2% | 0.882 | 1.6s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 80.9% | 95.0% | 68.1% | 25.4% |
| 0.25 | 82.9% | 95.0% | 70.7% | 22.0% |
| 0.30 | 84.9% | 93.9% | 74.2% | 18.2% |
| 0.35 | 86.7% | 91.7% | 78.2% | 14.0% |
| 0.40 | 88.5% | 91.1% | 81.8% | 10.6% |
| 0.50 | 89.6% | 87.8% | 86.4% | 7.2% |
| 0.60 | 89.9% | 83.3% | 90.7% | 4.2% |
| 0.65 | 89.0% | 80.0% | 91.6% | 3.8% |

### 저장 (ingest, 56건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 85.7% [74.3–92.6] (n=56) |
|   └ 올바른 카테고리에 저장 | 93.2% [81.8–97.7] (n=44) |
|   └ 트리/루트 밖 Q&A 거절 | 58.3% [32.0–80.7] (n=12) |
| 계층 F1 | 0.971 [0.935–1.000] (n=44) |
| 초안이 검색에 안 보임 | 100.0% [92.0–100.0] (n=44) |
| 게시 성공 (버전 +1) | 100.0% [92.0–100.0] (n=44) |
| 초안→게시 카테고리 유지 | 100.0% [92.0–100.0] (n=44) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [92.0–100.0] (n=44) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 3 |
| 게시 후 probe 검색 top-k 포함 | 91.3% [73.2–97.6] (n=23) |
| 게시 후 probe 검색 top1 답변 | 91.3% [73.2–97.6] (n=23) |
| 지연 p50 (초안+게시+probe) | 2.8s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (11건)

통과 100.0% [74.1–100.0] (n=11)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| null | life | 3 |
| food | food_cook | 2 |
| food_store_item | food_store_freeze | 2 |
| null | it | 2 |
| null | money | 2 |
| food_tool | food_tool_pan | 1 |
| it_office_excel_func | work | 1 |
| it_pc_peri_monitor | it_pc_win | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life | life_home | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| life_appl_wash_dryer_shrink | life_clean | 1 |
| life_appl_wash_wm_drum_care_gasket | life_clean_mold | 1 |
| life_appl_wash_wm_drum_care_smell | life_clean_towel | 1 |
| money_bank_cert | it_sec | 1 |
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | health | 1 |
| null | health_sym_stomach | 1 |
| null | it_mobile | 1 |

### 틀린 케이스 (100건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-IT-003 | How do I turn on Bluetooth on my Windows laptop when the toggle is missing? | it_pc_peri_bt | it_pc_win | branch |
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit | false_tree |
| I-SHOP-007 | 호텔 예약을 취소하면 수수료가 얼마나 나오나요? | null | shop_order_cancel | false_tree |
| I-WORK-001 | 사원증을 잃어버렸는데 재발급은 어떻게 받나요? | work_hr_onboard | work_hr | shallow |
| I-XD-002 | 체크카드로 결제한 주문을 취소하면 환불은 며칠 걸리나요? | shop_ret_refund_card | shop_order_cancel | branch |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_ret | false_tree |
| S-FOOD-001 | 밥솥으로 밥을 했는데 밥알 가운데가 생쌀처럼 씹혀요 | food_cook_base_rice · no answer | food_cook_base_rice · top food_cook_base_rice#1 0.34 reference | answered unanswerable |
| S-FOOD-007 | 배탈 난 뒤 관리 말고요, 여름에 식중독 안 걸리게 미리 조심할 것들 알려주세요 | food_safe_poison · no answer | food_safe_poison · top food_safe_poison#1 0.51 alternative | answered unanswerable |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food_cook · top food_cook#article 0.56 alternative | route deep, wrong top1 |
| S-FOOD-012 | 스테인리스 냄비 바닥에 까맣게 탄 자국은 어떻게 벗겨요? | food_tool · no answer | food_tool_pan · abstained · top food_tool_pan#1 0.02 reference | route deep |
| S-FOOD-014 | 그거 깔끔하게 까는 요령 좀요 | food_cook_side_egg · no answer | food_cook_side_egg · top food_cook_side_egg#1 0.32 reference | answered unanswerable |
| S-FOOD-015 | 그럼 소면은요? | food_cook_base_noodle · no answer | food_cook_base_noodle · top food_cook_base_noodle#1 0.31 reference | answered unanswerable |
| S-FOOD-018 | 아 근데 냉장고에 있는 요거트가 날짜가 이틀 지났는데 먹어도 될까요? | food_safe_date · no answer | food_safe_date · top food_safe_date#1 0.32 reference | answered unanswerable |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item · no answer | food_store_freeze · top food_store_freeze#1 0.82 recommended | route branch, answered unanswerable |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item · no answer | food_store_freeze · top food_store_freeze#1 0.89 recommended | route branch, answered unanswerable |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it_mobile · abstained · top it_mobile#article 0.05 reference | route false_tree |
| S-GAP-010 | 중고거래로 물건값을 보냈는데 상대가 잠적했어요. 사기 신고는 어디에 하나요? | null · no answer | money · abstained · top money_bank_xfer_wrong#1 0.28 reference | route false_tree |
| S-GAP-011 | 임대차 계약갱신청구권을 쓰려면 집주인에게 언제 말해야 하나요? | null · no answer | life · abstained · top life_appl_wash_dryer_slow#1 0.12 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it_sec · abstained · top it_sec#article 0.04 reference | route false_tree |
| S-HEALTH-002 | 진통제 거의 매일 먹는데 이거 괜찮은거임? 안 먹으면 머리 깨질 것 같음 | health_sym_head · no answer | health_sym_head · top health_sym_head#1 0.31 reference | answered unanswerable |
| S-HEALTH-007 | My 8-month-old baby is choking on something and can't breathe. What should I do  | health_aid_emerg_choke · no answer | health_aid_emerg_choke · top health_aid_emerg_choke#1 0.39 reference | answered unanswerable |
| S-HEALTH-009 | 달리기 말고 헬스장에서 근력이랑 유산소를 섞어서 하려는데 뭐부터 해야 돼요? | health_fit_start · no answer | health_fit_start · top health_fit_start#1 0.43 alternative | answered unanswerable |
| S-HEALTH-017 | 집에서 다쳤을 때 쓸 응급처치를 종류별로 한번 정리해 두고 싶어요 | health_aid | health_aid · top health_aid_injury_sprain#1 0.65 recommended | wrong top1 |
| S-IT-028 | 사이트마다 비밀번호를 다르게 쓰면 다 기억할 수가 없어요. | it_sec_pw · no answer | it_sec_pw · top it_sec_pw#1 0.43 alternative | answered unanswerable |
| S-IT-035 | 듀얼 모니터 쓰는데 작업 표시줄을 한쪽 화면에만 나오게 할 수 있나요? | it_pc_peri_monitor · no answer | it_pc_win · abstained · top it_pc_win#article 0.08 reference | route branch |
| S-IT-041 | 같은 브랜드로 가요. 옮기기 전에 백업은 어떻게 해 둬요? | it_mobile_move_android · no answer | it_mobile_move_android · top it_mobile_move_android#1 0.59 alternative | answered unanswerable |
| S-IT-043 | 아 잘못 말했어요, 새로 산 게 아이폰이 아니라 갤럭시예요. 그럼 어떻게 옮겨요? | it_mobile_move_cross · no answer | it_mobile_move_cross · top it_mobile_move_cross#1 0.37 reference | answered unanswerable |
| S-IT-051 | 누가 제 이메일 계정 비번이랑 복구 메일까지 바꿔 놔서 들어갈 수가 없어요 | it_sec_hacked · no answer | it_sec_hacked · top it_sec_hacked#1 0.33 reference | answered unanswerable |
| S-IT-503 | 전체 화면 말고 지금 띄워 놓은 창 하나만 찍고 싶어요 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.77 recommended | answered unanswerable |
| S-IT-504 | 클립보드 기록 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.78 recommended | answered unanswerable |
| S-IT-515 | 빔프로젝터 연결했는데 노트북 화면이랑 똑같이 duplicate 되게 바꾸는 키? | it_pc_win_keys (+1) | it_pc_win_keys · abstained · top it_pc_win_keys#win_shift_s 0.02 reference | abstained |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func · no answer | work · abstained · top work_pay_ot#1 0.21 reference | route domain |
| S-LIFE-001 | 드럼세탁기 코스가 다 끝났는데 옷이 물에 푹 젖은 채로 나와요. 탈수가 안 된 것 같아요 | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.78 recommended | answered unanswerable |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell · no answer | life_clean_towel · top life_clean_towel#1 0.52 alternative | route branch, answered unanswerable |
| S-LIFE-004 | 드럼 고무패킹 곰팡이 | life_appl_wash_wm_drum_care_gasket | life_clean_mold · abstained · top life_clean_mold#1 0.17 reference | route branch, abstained |
| S-LIFE-006 | 통돌이 세탁기로 빨래하면 검은 찌꺼기가 뭍어나와여 청소 어케해요 | life_appl_wash_wm_top_clean · no answer | life_appl_wash_wm_top_clean · top life_appl_wash_wm_top_clean#1 0.83 recommended | answered unanswerable |
| S-LIFE-008 | 건조기 돌리면 안 되고 그냥 널어서 말려야 하는 옷은 어떤 거예요? | life_appl_wash_dryer_shrink | life_clean · abstained · top life_clean#article 0.06 reference | route branch, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.52 alternative | route shallow |
| S-LIFE-020 | 자취 시작하고 집안일이 처음이라 뭐가 고장 나거나 지저분해지면 어디서부터 봐야 할지 모르겠어요 | life | life_home · top life_home#article 0.85 recommended | route deep, wrong top1 |
| S-LIFE-030 | 빨래를 조금만 넣었을 때 탈수가 잘 안 되는데 왜 그래요? | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.79 recommended | answered unanswerable |
| S-LIFE-035 | 그럼 아까 그거 빨고 나서 방에서 말릴 때 냄새 안 나게 하려면 어떻게 해요? | life_clean_towel · no answer | life_clean_towel · top life_clean_towel#1 0.49 alternative | answered unanswerable |
| S-LIFE-036 | 겨울만 되면 거실 창틀이 곰팡이로 까매져요 | life_clean_mold · no answer | life_clean_mold · top life_clean_mold#1 0.40 alternative | answered unanswerable |
| S-LIFE-038 | 세면대 물이 고인 채로 한참 걸려서 빠져요 | life_home_drain · no answer | life_home_drain · top life_home_drain#1 0.33 reference | answered unanswerable |
| S-MONEY-002 | 통장 만든 지 일주일 됐는데 하루에 30만 원까지만 보내져요. 앱에서 이체 한도 올리기를 눌러 봐도 안 되는데 이거 계좌 자체가 묶인 건가요? | money_bank_acct_limit · no answer | money_bank_acct_limit · top money_bank_acct_limit#1 0.36 reference | answered unanswerable |
| S-MONEY-005 | 인터넷뱅킹으로 계좌이체할 때 하루 한도를 늘리려면 OTP가 꼭 있어야 하나요? | money_bank_xfer_cap · no answer | money_bank_xfer_cap · top money_bank_xfer_cap#1 0.70 recommended | answered unanswerable |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec#article 0.06 reference | route domain, abstained |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost · no answer | money_card_lost · top money_card_lost#1 0.48 alternative | answered unanswerable |
| S-MONEY-012 | 온라인 쇼핑몰에서 결제하는데 한도 초과라고 승인이 안 나요. 쇼핑몰 오류가 아니라 제 신용카드 한도가 찬 거라는데 지금 바로 결제하려면 어떻게  | money_card_limit · no answer | money_card_limit · top money_card_limit#1 0.40 reference | answered unanswerable |
| S-MONEY-017 | 직장인인데 주말마다 과외해서 번 돈이 좀 있어요. 이것도 회사 연말정산으로 끝나나요, 아니면 제가 따로 신고해야 하나요? | money_tax_income · no answer | money_tax_income · top money_tax_income#1 0.41 alternative | answered unanswerable |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend · no answer | work_exp_claim · abstained · top work_exp_claim#1 0.14 reference | route domain |
| S-MONEY-027 | 마통은요? | money_loan_credit · no answer | money_loan_credit · top money_loan_credit#1 0.54 alternative | answered unanswerable |
| S-MONEY-033 | 한도를 당장 다시 살리는 방법은 없어요? | money_card_limit · no answer | money_card_limit · top money_card_limit#1 0.38 reference | answered unanswerable |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit · no answer | money_bank_acct_limit · top money_bank_acct_limit#1 0.59 alternative | answered unanswerable |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card#article 0.04 reference | route false_tree |
| S-MONEY-039 | 휴면 예금이 서민금융진흥원으로 넘어갔다는데 그럼 이제 못 돌려받는 거예요? | money_bank_acct_dormant · no answer | money_bank_acct_dormant · top money_bank_acct_dormant#1 0.30 reference | answered unanswerable |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_power_boot#1 0.16 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · top money_save#article 0.58 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health#article 0.07 reference | route false_tree |

## jev_llm · deep

### 검색 (445건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 96.4% [94.2–97.8] (n=445) |
|   └ 답할 수 있는 질문 | 95.8% [93.2–97.4] (n=377) |
|   └ 답할 수 없는 질문(보류가 정답) | 100.0% [94.7–100.0] (n=68) |
| 확신 정답률 (top1 정답 + recommended) | 89.7% [86.2–92.3] (n=377) |
| 라우팅 정확도 (정확 일치) | 95.1% [92.6–96.7] (n=445) |
| 계층 F1 (hF1) | 0.966 [0.950–0.981] (n=445) |
| 트리 밖 판정 precision | 100.0% [88.6–100.0] (n=30) |
| 트리 밖 판정 recall | 78.9% [63.7–88.9] (n=38) |
| 트리 밖 판정 F1 | 0.882 |
| 오보류율 (답할 수 있는데 보류) | 1.6% [0.7–3.4] (n=377) |
| 오답변율 (답할 수 없는데 답함) | 0.0% [0.0–5.3] (n=68) |
| Hit@1 | 95.8% [93.2–97.4] (n=377) |
| Hit@3 | 96.8% [94.5–98.2] (n=377) |
| Hit@k (limit) | 97.1% [94.9–98.4] (n=377) |
| MRR@k | 0.964 [0.945–0.982] (n=377) |
| nDCG@k | 0.953 [0.934–0.971] (n=377) |
| Recall@k | 0.958 [0.940–0.977] (n=377) |
| recommended 정밀도 | 0.973 [0.959–0.988] (n=342) |
| 라우팅이 맞았을 때 Hit@1 | 98.6% [96.8–99.4] (n=364) |
| 라우팅이 맞았을 때 MRR | 0.992 [0.986–0.999] (n=364) |
| 지연 p50 / p90 / p95 / 평균 | 8.0s / 38.7s / 44.4s / 13.0s |
| 라우팅 구간 평균 | 12.4s |
| 랭킹 후보 수 평균 | 14.6 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.21 |
| LLM 라우팅 호출 수 평균 | 0.98 |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–0.9] (n=445) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 | d5 | d6 | d7 |
|---|---:|---:|---:|---:|---:|---:|---:|
| 일치율 | 98.6% (n=360) | 98.0% (n=350) | 95.5% (n=331) | 97.4% (n=151) | 100.0% (n=21) | 100.0% (n=10) | 100.0% (n=8) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 7 | 1 | 4 | 488 | 11 | 2 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 3.1s |
| internal | 24 | 83.3% | 100.0% | 83.3% | 0.906 | 6.3s |
| leaf | 353 | 96.6% | 96.3% | 96.6% | 0.967 | 8.7s |
| no_answer | 27 | 100.0% | 96.3% | - | - | 7.4s |
| oos | 38 | 100.0% | 78.9% | - | - | 6.3s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 96.2% | 95.2% | 96.1% | 0.961 | 7.9s |
| single | 340 | 96.5% | 95.0% | 95.6% | 0.964 | 8.2s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 340 | 96.5% | 95.0% | 95.6% | 0.964 | 8.2s |
| coref | 19 | 89.5% | 89.5% | 89.5% | 0.895 | 11.7s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 11.9s |
| disambig | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 6.3s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 7.6s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 3.5s |
| long | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 12.0s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 9.1s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 7.7s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 9.4s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 385 | 95.8% | 94.5% | 95.3% | 0.960 | 8.0s |
| root | 43 | 100.0% | 97.7% | 100.0% | 1.000 | 8.2s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 4.5s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 35 | 88.6% | 97.1% | 83.3% | 0.906 | 6.4s |
| leaf | 372 | 96.8% | 96.5% | 96.6% | 0.967 | 8.6s |
| none | 38 | 100.0% | 78.9% | - | - | 6.3s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 100.0% | 78.9% | - | - | 6.3s |
| d1 | 10 | 90.0% | 100.0% | 90.0% | 0.950 | 6.9s |
| d2 | 30 | 90.0% | 96.7% | 84.2% | 0.908 | 7.7s |
| d3 | 202 | 95.0% | 95.0% | 94.7% | 0.947 | 9.7s |
| d4 | 144 | 98.6% | 97.9% | 98.6% | 0.989 | 7.4s |
| d5 | 11 | 100.0% | 100.0% | 100.0% | 1.000 | 7.9s |
| d6 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 32.1s |
| d7 | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 40.0s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 100.0% | 80.5% | - | - | 5.2s |
| 101+ | 4 | 75.0% | 100.0% | 75.0% | 0.875 | 10.9s |
| 2-5 | 319 | 96.6% | 96.2% | 96.4% | 0.966 | 7.8s |
| 33-100 | 61 | 96.7% | 96.7% | 96.6% | 0.966 | 12.1s |
| 6-32 | 20 | 90.0% | 100.0% | 81.8% | 0.909 | 10.0s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 11.1s |
| colloquial | 69 | 95.7% | 97.1% | 95.4% | 0.962 | 8.8s |
| condition | 32 | 96.9% | 93.8% | 96.8% | 0.968 | 9.3s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 2.5s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 12.8s |
| keyword | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 11.5s |
| long | 12 | 91.7% | 91.7% | 91.7% | 0.917 | 10.7s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 8.6s |
| multi_intent | 9 | 100.0% | 77.8% | 100.0% | 1.000 | 7.3s |
| negation | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 10.2s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 1.9s |
| oos_near | 15 | 100.0% | 53.3% | - | - | 8.0s |
| paraphrase | 177 | 97.2% | 96.6% | 96.4% | 0.964 | 9.2s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 11.0s |
| vague | 24 | 83.3% | 100.0% | 83.3% | 0.906 | 6.3s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 93.5% | 95.7% | 93.0% | 0.948 | 9.5s |
| health | 40 | 100.0% | 100.0% | 100.0% | 1.000 | 7.1s |
| it | 101 | 96.0% | 96.0% | 95.8% | 0.958 | 8.5s |
| life | 42 | 100.0% | 97.6% | 100.0% | 1.000 | 4.8s |
| money | 49 | 95.9% | 98.0% | 95.7% | 0.967 | 2.7s |
| oos | 38 | 100.0% | 78.9% | - | - | 6.3s |
| shop | 41 | 90.2% | 92.7% | 89.5% | 0.908 | 9.7s |
| travel | 44 | 97.7% | 97.7% | 97.4% | 0.974 | 11.8s |
| work | 44 | 95.5% | 95.5% | 95.1% | 0.963 | 11.1s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 95.8% | 97.5% | 95.5% | 0.964 | 9.4s |
| hard | 125 | 95.2% | 87.2% | 93.0% | 0.930 | 7.9s |
| medium | 202 | 97.5% | 98.5% | 97.2% | 0.979 | 7.9s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 304 | 95.7% | 93.4% | 95.0% | 0.954 | 8.5s |
| test | 141 | 97.9% | 98.6% | 97.5% | 0.983 | 7.5s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 96.0% | 98.9% | 96.3% | 2.9% |
| 0.25 | 96.0% | 98.7% | 96.5% | 2.9% |
| 0.30 | 96.4% | 98.4% | 97.3% | 0.0% |
| 0.35 | 96.2% | 98.1% | 97.3% | 0.0% |
| 0.40 | 96.0% | 97.6% | 97.6% | 0.0% |
| 0.50 | 95.3% | 96.0% | 98.3% | 0.0% |
| 0.60 | 92.6% | 92.8% | 98.3% | 0.0% |
| 0.65 | 91.2% | 90.7% | 98.8% | 0.0% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 95.6% [87.8–98.5] (n=68) |
|   └ 올바른 카테고리에 저장 | 100.0% [93.6–100.0] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 75.0% [46.8–91.1] (n=12) |
| 계층 F1 | 1.000 [1.000–1.000] (n=56) |
| 초안이 검색에 안 보임 | 100.0% [93.6–100.0] (n=56) |
| 게시 성공 (버전 +1) | 98.2% [90.6–99.7] (n=56) |
| 초안→게시 카테고리 유지 | 96.4% [87.9–99.0] (n=56) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [93.6–100.0] (n=56) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 91.3% [73.2–97.6] (n=23) |
| 게시 후 probe 검색 top1 답변 | 91.3% [73.2–97.6] (n=23) |
| 지연 p50 (초안+게시+probe) | 18.7s |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–6.1] (n=59) |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 1 |
| food_tool | food_tool_pan | 1 |
| it_mobile_lost | travel_local_metro | 1 |
| it_mobile_lost | travel_local_taxi | 1 |
| it_net_down | it_net_wifi_slow | 1 |
| it_office_excel_func | work_hr_cert | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| money_card_lost | money_card_abroad | 1 |
| null | food | 1 |
| null | health | 1 |
| null | it | 1 |
| null | it_mobile | 1 |
| null | it_net_wifi_slow | 1 |
| null | it_sec_pw | 1 |
| null | money_card | 1 |
| null | money_save_etf | 1 |
| shop_member_grade | shop_member_coupon | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_order_receipt | money_tax_income | 1 |
| travel_prep_passport | travel_prep_visa | 1 |

### 틀린 케이스 (31건 중 31건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-HEALTH-002 | 헬스 하고 다음 날 근육통이 심하면 운동 쉬어야 됨? | health_fit_start | health_fit_start | exact |
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it_office_excel_func | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit_start | false_tree |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food · top food_safe_poison#2 0.61 alternative | wrong top1 |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.45 alternative | wrong top1 |
| S-FOOD-012 | 스테인리스 냄비 바닥에 까맣게 탄 자국은 어떻게 벗겨요? | food_tool · no answer | food_tool_pan · abstained · top food_tool_pan#2 0.04 reference | route deep |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.87 recommended | route branch, wrong top1 |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it_net_wifi_slow · abstained · top it_net_wifi_slow#2 0.04 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it_sec_pw · abstained · top it_sec_pw#3 0.03 reference | route false_tree |
| S-IT-017 | 지하철에 아이폰을 두고 내렸어요. 지금 어디 있는지 확인할 방법이 있을까요? | it_mobile_lost | travel_local_metro · top travel_local_metro#2 0.64 alternative | route domain, wrong top1 |
| S-IT-023 | 어제 저녁부터 집에 있는 폰, 노트북, TV 전부 인터넷이 안 돼요. 공유기 불은 들어와 있고 와이파이도 잡히긴 하는데 사이트가 하나도 안 열려 | it_net_down | it_net_wifi_slow · abstained · top it_net_wifi_slow#2 0.10 reference | route branch, abstained |
| S-IT-524 | 입사일만 넣으면 몇 년 다녔는지 자동으로 나오게 하고 싶은데ㅠ 방법 없나 | it_office_excel_func | work_hr_cert · abstained · top work_hr_cert#2 0.10 reference | route domain, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.65 recommended | route shallow |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost | money_card_abroad · abstained · top money_card_abroad#3 0.28 reference | route branch, abstained |
| S-MONEY-021 | 제 명의 신용카드 관련해서 이것저것 궁금한 게 있는데요 | money_card | money_card · top money_card_lost#4 0.71 recommended | wrong top1 |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card_limit#2 0.10 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.28 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money_save_etf · abstained · top money_save_etf#2 0.04 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health · abstained · top health#article 0.26 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.13 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | it_mobile · abstained · top it_mobile#article 0.06 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.35 reference | route domain, wrong top1 |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.47 alternative | wrong top1 |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.48 alternative | route branch, wrong top1 |
| S-SHOP-031 | 그 위 등급이 되면 매달 쿠폰은 몇 장씩 주는 거예요? | shop_member_grade | shop_member_coupon · abstained · top shop_member_coupon#2 0.07 reference | route branch, abstained |
| S-TRAVEL-008 | 여권 만료일이 넉 달 뒤인데 이걸로 동남아 여행 가도 문제없을까요? | travel_prep_passport | travel_prep_visa · abstained · top travel_prep_visa#2 0.25 reference | route branch, abstained |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.74 recommended | wrong top1 |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work · top work_pay_slip#2 0.66 recommended | route shallow |
| S-XD-013 | 택시에 휴대폰을 두고 내렸어요. 폰 위치부터 찾을 수 있나요? | it_mobile_lost | travel_local_taxi · abstained · top no items | route domain, abstained |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.84 recommended | route branch, wrong top1 |

## jev_llm · standard

### 검색 (444건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 96.6% [94.5–97.9] (n=444) |
|   └ 답할 수 있는 질문 | 96.3% [93.8–97.8] (n=376) |
|   └ 답할 수 없는 질문(보류가 정답) | 98.5% [92.1–99.7] (n=68) |
| 확신 정답률 (top1 정답 + recommended) | 90.4% [87.0–93.0] (n=376) |
| 라우팅 정확도 (정확 일치) | 95.7% [93.4–97.2] (n=444) |
| 계층 F1 (hF1) | 0.967 [0.951–0.983] (n=444) |
| 트리 밖 판정 precision | 100.0% [87.9–100.0] (n=28) |
| 트리 밖 판정 recall | 73.7% [58.0–85.0] (n=38) |
| 트리 밖 판정 F1 | 0.848 |
| 오보류율 (답할 수 있는데 보류) | 0.8% [0.3–2.3] (n=376) |
| 오답변율 (답할 수 없는데 답함) | 1.5% [0.3–7.9] (n=68) |
| Hit@1 | 96.5% [94.2–98.0] (n=376) |
| Hit@3 | 98.1% [96.2–99.1] (n=376) |
| Hit@k (limit) | 98.1% [96.2–99.1] (n=376) |
| MRR@k | 0.973 [0.958–0.988] (n=376) |
| nDCG@k | 0.963 [0.947–0.978] (n=376) |
| Recall@k | 0.969 [0.953–0.984] (n=376) |
| recommended 정밀도 | 0.981 [0.969–0.994] (n=344) |
| 라우팅이 맞았을 때 Hit@1 | 98.4% [96.5–99.3] (n=368) |
| 라우팅이 맞았을 때 MRR | 0.991 [0.985–0.998] (n=368) |
| 지연 p50 / p90 / p95 / 평균 | 12.4s / 39.0s / 46.7s / 16.9s |
| 라우팅 구간 평균 | 16.4s |
| 랭킹 후보 수 평균 | 15.5 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.25 |
| LLM 라우팅 호출 수 평균 | 0.98 |
| LLM 실패 → Jev 빔 대체율 | 0.2% [0.0–1.3] (n=444) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 99.2% (n=359) | 98.6% (n=349) | 97.0% (n=330) | 97.6% (n=82) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 4 | 1 | 3 | 491 | 12 | 1 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 7.0s |
| internal | 23 | 73.9% | 95.7% | 73.9% | 0.841 | 12.4s |
| leaf | 353 | 97.7% | 98.0% | 98.0% | 0.982 | 12.3s |
| no_answer | 27 | 100.0% | 96.3% | - | - | 14.9s |
| oos | 38 | 97.4% | 73.7% | - | - | 12.7s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 96.2% | 95.2% | 96.1% | 0.961 | 11.9s |
| single | 339 | 96.8% | 95.9% | 96.7% | 0.977 | 12.5s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 339 | 96.8% | 95.9% | 96.7% | 0.977 | 12.5s |
| coref | 19 | 89.5% | 89.5% | 89.5% | 0.895 | 11.2s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 12.5s |
| disambig | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 11.2s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 12.7s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 14.6s |
| long | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 13.5s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 22.9s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 11.7s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 10.7s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 384 | 96.1% | 95.3% | 96.2% | 0.970 | 12.3s |
| root | 43 | 100.0% | 97.7% | 100.0% | 1.000 | 11.7s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 13.0s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 82.4% | 97.1% | 73.9% | 0.841 | 13.7s |
| leaf | 372 | 97.8% | 97.8% | 98.0% | 0.982 | 12.3s |
| none | 38 | 97.4% | 73.7% | - | - | 12.7s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 97.4% | 73.7% | - | - | 12.7s |
| d1 | 10 | 90.0% | 100.0% | 90.0% | 0.950 | 9.8s |
| d2 | 30 | 83.3% | 96.7% | 73.7% | 0.833 | 13.7s |
| d3 | 278 | 97.1% | 97.5% | 97.3% | 0.975 | 12.4s |
| d4 | 88 | 100.0% | 98.9% | 100.0% | 1.000 | 12.3s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 97.6% | 75.6% | - | - | 11.7s |
| 101+ | 4 | 75.0% | 100.0% | 75.0% | 0.875 | 9.9s |
| 2-5 | 319 | 97.5% | 97.8% | 97.7% | 0.980 | 12.3s |
| 33-100 | 61 | 98.4% | 98.4% | 98.3% | 0.983 | 12.1s |
| 6-32 | 19 | 78.9% | 94.7% | 60.0% | 0.733 | 20.4s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 13.7s |
| colloquial | 69 | 97.1% | 98.6% | 96.9% | 0.977 | 11.8s |
| condition | 32 | 90.6% | 90.6% | 93.5% | 0.935 | 12.4s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 12.8s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 10.0s |
| keyword | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 12.8s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 21.8s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 12.1s |
| multi_intent | 9 | 100.0% | 88.9% | 100.0% | 1.000 | 26.0s |
| negation | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 13.2s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 19.8s |
| oos_near | 15 | 93.3% | 40.0% | - | - | 8.0s |
| paraphrase | 177 | 98.3% | 97.7% | 97.8% | 0.978 | 12.5s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 4.0s |
| vague | 23 | 73.9% | 95.7% | 73.9% | 0.841 | 12.4s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 93.5% | 97.8% | 93.0% | 0.953 | 12.8s |
| health | 40 | 97.5% | 100.0% | 97.2% | 0.986 | 11.9s |
| it | 101 | 99.0% | 99.0% | 99.0% | 0.990 | 12.1s |
| life | 41 | 100.0% | 97.6% | 100.0% | 1.000 | 12.4s |
| money | 49 | 95.9% | 100.0% | 97.8% | 0.986 | 13.0s |
| oos | 38 | 97.4% | 73.7% | - | - | 12.7s |
| shop | 41 | 87.8% | 90.2% | 86.8% | 0.882 | 12.0s |
| travel | 44 | 100.0% | 100.0% | 100.0% | 1.000 | 12.9s |
| work | 44 | 95.5% | 95.5% | 95.1% | 0.963 | 12.3s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 98.3% | 100.0% | 98.2% | 0.991 | 12.4s |
| hard | 124 | 93.5% | 86.3% | 92.9% | 0.929 | 12.7s |
| medium | 202 | 97.5% | 99.0% | 97.2% | 0.983 | 12.2s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 303 | 96.7% | 94.7% | 96.1% | 0.969 | 12.8s |
| test | 141 | 96.5% | 97.9% | 97.5% | 0.982 | 11.7s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 96.4% | 99.5% | 96.3% | 4.4% |
| 0.25 | 96.8% | 99.5% | 96.8% | 1.5% |
| 0.30 | 96.6% | 99.2% | 96.8% | 1.5% |
| 0.35 | 96.4% | 98.9% | 96.8% | 1.5% |
| 0.40 | 96.4% | 98.7% | 97.0% | 1.5% |
| 0.50 | 96.2% | 97.9% | 97.6% | 1.5% |
| 0.60 | 93.5% | 93.6% | 98.6% | 0.0% |
| 0.65 | 91.9% | 91.5% | 98.8% | 0.0% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 97.1% [89.9–99.2] (n=68) |
|   └ 올바른 카테고리에 저장 | 100.0% [93.6–100.0] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 83.3% [55.2–95.3] (n=12) |
| 계층 F1 | 1.000 [1.000–1.000] (n=56) |
| 초안이 검색에 안 보임 | 100.0% [93.6–100.0] (n=56) |
| 게시 성공 (버전 +1) | 98.2% [90.6–99.7] (n=56) |
| 초안→게시 카테고리 유지 | 96.4% [87.9–99.0] (n=56) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [93.6–100.0] (n=56) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 87.0% [67.9–95.5] (n=23) |
| 게시 후 probe 검색 top1 답변 | 87.0% [67.9–95.5] (n=23) |
| 지연 p50 (초안+게시+probe) | 41.2s |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–6.2] (n=58) |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| null | it | 3 |
| null | it_mobile | 2 |
| food_store_item | food_store_freeze | 1 |
| it_mobile_lost | travel_local_taxi | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| null | food | 1 |
| null | health_sym_stomach | 1 |
| null | money_card | 1 |
| null | money_save | 1 |
| null | travel_car | 1 |
| shop_member_grade | shop_member_coupon | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_ret | shop_ret_swap | 1 |
| shop_ret_refund_card | money_card_install | 1 |
| work_exp_trip | travel_air_ticket_mile | 1 |
| work_exp_trip | work_exp_claim | 1 |

### 틀린 케이스 (29건 중 29건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-HEALTH-002 | 헬스 하고 다음 날 근육통이 심하면 운동 쉬어야 됨? | health_fit_start | health_fit_start | exact |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it_office_excel_func | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit_start | false_tree |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food · top food_safe_poison#2 0.58 alternative | wrong top1 |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.48 alternative | wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.88 recommended | route branch, wrong top1 |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it_mobile · abstained · top it_mobile#article 0.06 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it · abstained · top it_pc_win_perf_slow#1 0.17 reference | route false_tree |
| S-HEALTH-017 | 집에서 다쳤을 때 쓸 응급처치를 종류별로 한번 정리해 두고 싶어요 | health_aid | health_aid · top health_aid_injury_sprain#1 0.62 alternative | wrong top1 |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_ac#2 0.67 recommended | route shallow |
| S-MONEY-021 | 제 명의 신용카드 관련해서 이것저것 궁금한 게 있는데요 | money_card | money_card · top money_card_lost#4 0.70 recommended | wrong top1 |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card_limit#2 0.10 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.24 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money_save · top money_save#article 0.57 alternative | route false_tree, answered unanswerable |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.08 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it · abstained · top it_pc_win_perf_slow#1 0.22 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym_stomach · abstained · top health_sym_stomach#3 0.16 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.13 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | it_mobile · abstained · top it_mobile_batt_drain#3 0.08 reference | route false_tree |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card | money_card_install · top money_card_install#3 0.40 reference | route domain, wrong top1 |
| S-SHOP-018 | 받은 물건이 맘에 안 들어서 처리하고 싶은데, 돌려보내는 거랑 바꾸는 거 중에 어떤 선택지가 있는지부터 알려 주세요 | shop_ret | shop_ret_swap · top shop_ret_swap#1 0.52 alternative | route deep, wrong top1 |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.48 alternative | wrong top1 |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.58 alternative | route branch, wrong top1 |
| S-SHOP-031 | 그 위 등급이 되면 매달 쿠폰은 몇 장씩 주는 거예요? | shop_member_grade | shop_member_coupon · abstained · top shop_member_coupon#2 0.07 reference | route branch, abstained |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.74 recommended | wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#2 0.07 reference | route domain |
| S-XD-013 | 택시에 휴대폰을 두고 내렸어요. 폰 위치부터 찾을 수 있나요? | it_mobile_lost | travel_local_taxi · abstained · top no items | route domain, abstained |
| S-XD-016 | 미국에 사는 동생한테 돈을 보내려는데 카드로도 보낼 수 있어요? 뭐가 필요해요? | money_bank_xfer_abroad | money_bank_xfer_abroad · abstained · top money_bank_xfer_abroad#1 0.30 reference | abstained |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.80 recommended | route branch, wrong top1 |

## jev_llm · shallow

### 검색 (400건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 97.5% [95.5–98.6] (n=400) |
|   └ 답할 수 있는 질문 | 97.7% [95.5–98.8] (n=345) |
|   └ 답할 수 없는 질문(보류가 정답) | 96.4% [87.7–99.0] (n=55) |
| 확신 정답률 (top1 정답 + recommended) | 91.0% [87.5–93.6] (n=345) |
| 라우팅 정확도 (정확 일치) | 95.2% [92.7–96.9] (n=400) |
| 계층 F1 (hF1) | 0.958 [0.938–0.977] (n=400) |
| 트리 밖 판정 precision | 100.0% [85.7–100.0] (n=23) |
| 트리 밖 판정 recall | 63.9% [47.6–77.5] (n=36) |
| 트리 밖 판정 F1 | 0.780 |
| 오보류율 (답할 수 있는데 보류) | 0.3% [0.1–1.6] (n=345) |
| 오답변율 (답할 수 없는데 답함) | 3.6% [1.0–12.3] (n=55) |
| Hit@1 | 97.7% [95.5–98.8] (n=345) |
| Hit@3 | 98.3% [96.3–99.2] (n=345) |
| Hit@k (limit) | 98.3% [96.3–99.2] (n=345) |
| MRR@k | 0.979 [0.965–0.994] (n=345) |
| nDCG@k | 0.965 [0.949–0.981] (n=345) |
| Recall@k | 0.967 [0.951–0.984] (n=345) |
| recommended 정밀도 | 0.983 [0.969–0.997] (n=319) |
| 라우팅이 맞았을 때 Hit@1 | 99.4% [97.9–99.8] (n=339) |
| 라우팅이 맞았을 때 MRR | 0.997 [0.992–1.000] (n=339) |
| 지연 p50 / p90 / p95 / 평균 | 11.8s / 32.1s / 35.7s / 13.4s |
| 라우팅 구간 평균 | 13.0s |
| 랭킹 후보 수 평균 | 14.6 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.24 |
| LLM 라우팅 호출 수 평균 | 0.97 |
| LLM 실패 → Jev 빔 대체율 | 0.2% [0.0–1.4] (n=400) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 |
|---|---:|---:|
| 일치율 | 99.1% (n=344) | 97.3% (n=334) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_tree |
|---:|---:|---:|---:|
| 4 | 2 | 438 | 15 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 10.9s |
| internal | 10 | 90.0% | 100.0% | 90.0% | 0.933 | 22.4s |
| leaf | 335 | 97.9% | 98.2% | 97.9% | 0.981 | 11.8s |
| no_answer | 16 | 100.0% | 100.0% | - | - | 7.9s |
| oos | 36 | 94.4% | 63.9% | - | - | 12.2s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 97.1% | 96.2% | 97.1% | 0.971 | 11.7s |
| single | 295 | 97.6% | 94.9% | 97.9% | 0.983 | 11.8s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 295 | 97.6% | 94.9% | 97.9% | 0.983 | 11.8s |
| coref | 19 | 89.5% | 89.5% | 89.5% | 0.895 | 9.7s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 22.1s |
| disambig | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 12.0s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 11.4s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 11.9s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 12.5s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 12.4s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 10.9s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 11.8s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 369 | 97.6% | 95.7% | 97.6% | 0.978 | 11.9s |
| root | 31 | 96.8% | 90.3% | 100.0% | 1.000 | 5.7s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 10 | 90.0% | 100.0% | 90.0% | 0.933 | 22.4s |
| leaf | 354 | 98.0% | 98.3% | 97.9% | 0.981 | 11.8s |
| none | 36 | 94.4% | 63.9% | - | - | 12.2s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 36 | 94.4% | 63.9% | - | - | 12.2s |
| d1 | 10 | 90.0% | 100.0% | 90.0% | 0.933 | 22.4s |
| d2 | 354 | 98.0% | 98.3% | 97.9% | 0.981 | 11.8s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 39 | 94.9% | 66.7% | - | - | 11.9s |
| 101+ | 4 | 75.0% | 100.0% | 75.0% | 0.833 | 25.4s |
| 2-5 | 301 | 98.3% | 98.7% | 98.3% | 0.984 | 11.4s |
| 33-100 | 56 | 96.4% | 96.4% | 96.4% | 0.964 | 13.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 11.4s |
| colloquial | 66 | 98.5% | 100.0% | 98.4% | 0.992 | 10.3s |
| condition | 32 | 93.8% | 90.6% | 93.5% | 0.935 | 11.9s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 10.7s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 12.6s |
| keyword | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 12.3s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 12.4s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 12.3s |
| multi_intent | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 8.9s |
| negation | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 13.6s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 12.4s |
| oos_near | 15 | 93.3% | 33.3% | - | - | 10.0s |
| paraphrase | 149 | 97.3% | 96.6% | 97.6% | 0.976 | 11.9s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 12.6s |
| vague | 10 | 90.0% | 100.0% | 90.0% | 0.933 | 22.4s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 40 | 92.5% | 95.0% | 92.3% | 0.932 | 12.9s |
| health | 36 | 100.0% | 100.0% | 100.0% | 1.000 | 10.9s |
| it | 93 | 98.9% | 98.9% | 98.9% | 0.989 | 12.4s |
| life | 36 | 100.0% | 100.0% | 100.0% | 1.000 | 2.2s |
| money | 44 | 100.0% | 100.0% | 100.0% | 1.000 | 3.0s |
| oos | 36 | 94.4% | 63.9% | - | - | 12.2s |
| shop | 36 | 94.4% | 94.4% | 94.1% | 0.941 | 11.7s |
| travel | 39 | 100.0% | 100.0% | 100.0% | 1.000 | 12.0s |
| work | 40 | 95.0% | 97.5% | 94.7% | 0.961 | 12.4s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 109 | 99.1% | 100.0% | 99.0% | 0.995 | 11.6s |
| hard | 112 | 93.8% | 84.8% | 94.0% | 0.940 | 11.7s |
| medium | 179 | 98.9% | 98.9% | 98.8% | 0.990 | 12.0s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 271 | 97.4% | 94.5% | 97.9% | 0.980 | 12.0s |
| test | 129 | 97.7% | 96.9% | 97.2% | 0.977 | 10.7s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 97.2% | 99.7% | 97.1% | 5.5% |
| 0.25 | 97.2% | 99.7% | 97.1% | 5.5% |
| 0.30 | 97.5% | 99.7% | 97.4% | 3.6% |
| 0.35 | 97.2% | 99.1% | 97.7% | 1.8% |
| 0.40 | 97.2% | 99.1% | 97.7% | 1.8% |
| 0.50 | 96.5% | 98.0% | 97.9% | 1.8% |
| 0.60 | 93.2% | 93.9% | 98.2% | 1.8% |
| 0.65 | 92.0% | 92.2% | 98.4% | 1.8% |

### 저장 (ingest, 59건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 96.6% [88.5–99.1] (n=59) |
|   └ 올바른 카테고리에 저장 | 100.0% [92.7–100.0] (n=49) |
|   └ 트리/루트 밖 Q&A 거절 | 80.0% [49.0–94.3] (n=10) |
| 계층 F1 | 1.000 [1.000–1.000] (n=49) |
| 초안이 검색에 안 보임 | 100.0% [92.7–100.0] (n=49) |
| 게시 성공 (버전 +1) | 100.0% [92.7–100.0] (n=49) |
| 초안→게시 카테고리 유지 | 100.0% [92.7–100.0] (n=49) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [92.7–100.0] (n=49) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 94.7% [75.4–99.1] (n=19) |
| 게시 후 probe 검색 top1 답변 | 94.7% [75.4–99.1] (n=19) |
| 지연 p50 (초안+게시+probe) | 23.6s |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–7.0] (n=51) |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 2 |
| null | it | 2 |
| null | it_sec_pw | 2 |
| null | money_save_etf | 2 |
| it_mobile_lost | travel_local_taxi | 1 |
| null | food | 1 |
| null | food_safe_poison | 1 |
| null | health | 1 |
| null | money_card_limit | 1 |
| null | money_card_lost | 1 |
| null | money_loan_home | 1 |
| null | travel_prep_data | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_ret_refund_card | money_card_install | 1 |
| work_exp_trip | work_exp_claim | 1 |

### 틀린 케이스 (23건 중 23건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it_office_excel_func | false_tree |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food · top food_safe_poison#2 0.61 alternative | wrong top1 |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.80 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.87 recommended | route branch, wrong top1 |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | travel_prep_data · abstained · top travel_prep_data#3 0.12 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it_sec_pw · abstained · top it_sec_pw#3 0.03 reference | route false_tree |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card_limit · abstained · top money_card_limit#2 0.03 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · top it_mobile_lost#2 0.35 reference | route false_tree, answered unanswerable |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money_save_etf · abstained · top money_save_etf#2 0.09 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money_loan_home · abstained · top money_loan_home#3 0.04 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money_save_etf · abstained · top money_save_etf#2 0.05 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it_sec_pw · abstained · top it_sec_pw#3 0.03 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health · abstained · top health#article 0.28 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.16 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | it · abstained · top it_pc_win_keys#alt_tab 0.19 reference | route false_tree |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card | money_card_install · top money_card_install#3 0.42 alternative | route domain, wrong top1 |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.56 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.75 recommended | wrong top1 |
| S-XD-013 | 택시에 휴대폰을 두고 내렸어요. 폰 위치부터 찾을 수 있나요? | it_mobile_lost | travel_local_taxi · abstained · top no items | route domain, abstained |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.79 recommended | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.83 recommended | route false_tree, answered unanswerable |
| S-XD-052 | 상한 음식을 먹고 계속 토해요. 어떻게 해야 돼요? | null · no answer | food_safe_poison · abstained · top food_safe_poison#2 0.14 reference | route false_tree |

## jev_llm · flat

### 검색 (369건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 98.6% [96.9–99.4] (n=369) |
|   └ 답할 수 있는 질문 | 98.5% [96.4–99.3] (n=325) |
|   └ 답할 수 없는 질문(보류가 정답) | 100.0% [92.0–100.0] (n=44) |
| 확신 정답률 (top1 정답 + recommended) | 91.7% [88.2–94.2] (n=325) |
| 라우팅 정확도 (정확 일치) | 98.9% [97.2–99.6] (n=369) |
| 계층 F1 (hF1) | 0.989 [0.979–1.000] (n=369) |
| 트리 밖 판정 precision | 100.0% [86.7–100.0] (n=25) |
| 트리 밖 판정 recall | 100.0% [86.7–100.0] (n=25) |
| 트리 밖 판정 F1 | 1.000 |
| 오보류율 (답할 수 있는데 보류) | 0.3% [0.1–1.7] (n=325) |
| 오답변율 (답할 수 없는데 답함) | 0.0% [0.0–8.0] (n=44) |
| Hit@1 | 98.5% [96.4–99.3] (n=325) |
| Hit@3 | 98.8% [96.9–99.5] (n=325) |
| Hit@k (limit) | 98.8% [96.9–99.5] (n=325) |
| MRR@k | 0.986 [0.974–0.999] (n=325) |
| nDCG@k | 0.970 [0.956–0.985] (n=325) |
| Recall@k | 0.971 [0.956–0.987] (n=325) |
| recommended 정밀도 | 0.990 [0.980–1.000] (n=301) |
| 라우팅이 맞았을 때 Hit@1 | 99.7% [98.3–99.9] (n=321) |
| 라우팅이 맞았을 때 MRR | 0.998 [0.995–1.000] (n=321) |
| 지연 p50 / p90 / p95 / 평균 | 12.3s / 87.4s / 90.7s / 22.8s |
| 라우팅 구간 평균 | 22.4s |
| 랭킹 후보 수 평균 | 12.0 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.24 |
| LLM 라우팅 호출 수 평균 | 0.97 |
| LLM 실패 → Jev 빔 대체율 | 9.2% [6.7–12.6] (n=369) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 |
|---|---:|
| 일치율 | 98.5% (n=334) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | exact |
|---:|---:|
| 4 | 415 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 22.8s |
| leaf | 325 | 98.5% | 98.8% | 98.5% | 0.986 | 12.4s |
| no_answer | 16 | 100.0% | 100.0% | - | - | 22.5s |
| oos | 25 | 100.0% | 100.0% | - | - | 4.2s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 97.1% | 97.1% | 97.1% | 0.971 | 12.0s |
| single | 264 | 99.2% | 99.6% | 99.1% | 0.993 | 12.5s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 264 | 99.2% | 99.6% | 99.1% | 0.993 | 12.5s |
| coref | 19 | 89.5% | 89.5% | 89.5% | 0.895 | 12.0s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 21.9s |
| disambig | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 12.3s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 12.1s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 11.9s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 4.7s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 12.3s |
| shift | 14 | 100.0% | 100.0% | 100.0% | 1.000 | 12.3s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 10.6s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 359 | 98.6% | 98.9% | 98.4% | 0.986 | 12.4s |
| root | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.3s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| leaf | 344 | 98.5% | 98.8% | 98.5% | 0.986 | 12.4s |
| none | 25 | 100.0% | 100.0% | - | - | 4.2s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 25 | 100.0% | 100.0% | - | - | 4.2s |
| d1 | 344 | 98.5% | 98.8% | 98.5% | 0.986 | 12.4s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 28 | 100.0% | 100.0% | - | - | 4.4s |
| 2-5 | 291 | 98.6% | 99.0% | 98.6% | 0.987 | 12.2s |
| 33-100 | 50 | 98.0% | 98.0% | 98.0% | 0.980 | 21.9s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 13.1s |
| colloquial | 66 | 98.5% | 100.0% | 98.4% | 0.992 | 11.6s |
| condition | 31 | 96.8% | 96.8% | 96.8% | 0.968 | 20.9s |
| english | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 11.9s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 22.7s |
| keyword | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 23.8s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 13.0s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 12.3s |
| multi_intent | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 12.2s |
| negation | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 11.8s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 4.4s |
| oos_near | 15 | 100.0% | 100.0% | - | - | 4.0s |
| paraphrase | 129 | 97.7% | 97.7% | 97.3% | 0.973 | 12.4s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 20.2s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 37 | 97.3% | 97.3% | 97.2% | 0.972 | 23.9s |
| health | 33 | 100.0% | 100.0% | 100.0% | 1.000 | 12.4s |
| it | 90 | 100.0% | 100.0% | 100.0% | 1.000 | 13.3s |
| life | 34 | 100.0% | 100.0% | 100.0% | 1.000 | 12.7s |
| money | 42 | 100.0% | 100.0% | 100.0% | 1.000 | 12.1s |
| oos | 25 | 100.0% | 100.0% | - | - | 4.2s |
| shop | 33 | 93.9% | 93.9% | 93.5% | 0.935 | 2.4s |
| travel | 37 | 100.0% | 100.0% | 100.0% | 1.000 | 4.9s |
| work | 38 | 94.7% | 97.4% | 94.4% | 0.958 | 21.9s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 99 | 99.0% | 100.0% | 98.9% | 0.995 | 12.7s |
| hard | 107 | 96.3% | 96.3% | 95.2% | 0.952 | 12.2s |
| medium | 163 | 100.0% | 100.0% | 100.0% | 1.000 | 12.1s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 248 | 98.8% | 98.8% | 98.6% | 0.986 | 12.4s |
| test | 121 | 98.3% | 99.2% | 98.1% | 0.986 | 12.3s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 98.6% | 99.7% | 98.8% | 0.0% |
| 0.25 | 98.6% | 99.7% | 98.8% | 0.0% |
| 0.30 | 98.6% | 99.7% | 98.8% | 0.0% |
| 0.35 | 98.1% | 99.1% | 98.8% | 0.0% |
| 0.40 | 98.1% | 99.1% | 98.8% | 0.0% |
| 0.50 | 97.6% | 98.5% | 98.8% | 0.0% |
| 0.60 | 94.3% | 94.5% | 99.0% | 0.0% |
| 0.65 | 92.7% | 92.6% | 99.0% | 0.0% |

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
| 게시 후 probe 검색 top-k 포함 | 100.0% [83.2–100.0] (n=19) |
| 게시 후 probe 검색 top1 답변 | 100.0% [83.2–100.0] (n=19) |
| 지연 p50 (초안+게시+probe) | 44.4s |
| LLM 실패 → Jev 빔 대체율 | 17.0% [8.9–30.1] (n=47) |

### API 계약 (11건)

통과 100.0% [74.1–100.0] (n=11)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_ret_refund_card | shop_ret_return_mind | 1 |
| work_exp_trip | work_exp_claim | 1 |

### 틀린 케이스 (5건 중 5건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.85 recommended | route branch, wrong top1 |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card | shop_ret_return_mind · abstained · top shop_ret_return_mind#2 0.09 reference | route branch, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.52 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.73 recommended | wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.79 recommended | route branch, wrong top1 |

## jev_llm · sparse

### 검색 (444건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 88.1% [84.7–90.8] (n=444) |
|   └ 답할 수 있는 질문 | 95.6% [91.5–97.7] (n=180) |
|   └ 답할 수 없는 질문(보류가 정답) | 83.0% [78.0–87.0] (n=264) |
| 확신 정답률 (top1 정답 + recommended) | 82.2% [76.0–87.1] (n=180) |
| 라우팅 정확도 (정확 일치) | 95.9% [93.7–97.4] (n=444) |
| 계층 F1 (hF1) | 0.973 [0.959–0.987] (n=444) |
| 트리 밖 판정 precision | 100.0% [88.6–100.0] (n=30) |
| 트리 밖 판정 recall | 78.9% [63.7–88.9] (n=38) |
| 트리 밖 판정 F1 | 0.882 |
| 오보류율 (답할 수 있는데 보류) | 3.3% [1.5–7.1] (n=180) |
| 오답변율 (답할 수 없는데 답함) | 17.0% [13.0–22.0] (n=264) |
| Hit@1 | 96.1% [92.2–98.1] (n=180) |
| Hit@3 | 96.1% [92.2–98.1] (n=180) |
| Hit@k (limit) | 96.7% [92.9–98.5] (n=180) |
| MRR@k | 0.962 [0.935–0.990] (n=180) |
| nDCG@k | 0.963 [0.936–0.990] (n=180) |
| Recall@k | 0.958 [0.931–0.986] (n=180) |
| recommended 정밀도 | 0.910 [0.868–0.953] (n=160) |
| 라우팅이 맞았을 때 Hit@1 | 97.7% [94.3–99.1] (n=175) |
| 라우팅이 맞았을 때 MRR | 0.978 [0.957–0.999] (n=175) |
| 지연 p50 / p90 / p95 / 평균 | 12.6s / 46.3s / 62.8s / 18.7s |
| 라우팅 구간 평균 | 18.5s |
| 랭킹 후보 수 평균 | 1.9 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 0.98 |
| LLM 라우팅 호출 수 평균 | 0.98 |
| LLM 실패 → Jev 빔 대체율 | 2.3% [1.2–4.1] (n=444) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 99.4% (n=359) | 98.9% (n=349) | 96.7% (n=330) | 97.6% (n=82) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 5 | 1 | 2 | 479 | 10 | 3 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dropped | 196 | 77.6% | 97.4% | - | - | 13.3s |
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 5.3s |
| internal | 23 | 91.3% | 95.7% | 91.3% | 0.922 | 5.1s |
| leaf | 157 | 96.2% | 97.5% | 96.8% | 0.968 | 12.1s |
| no_answer | 27 | 100.0% | 100.0% | - | - | 16.7s |
| oos | 38 | 97.4% | 78.9% | - | - | 12.8s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 86.7% | 97.1% | 100.0% | 1.000 | 12.2s |
| single | 339 | 88.5% | 95.6% | 94.9% | 0.951 | 12.8s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 339 | 88.5% | 95.6% | 94.9% | 0.951 | 12.8s |
| coref | 19 | 84.2% | 94.7% | 100.0% | 1.000 | 12.9s |
| correct | 8 | 87.5% | 100.0% | 100.0% | 1.000 | 12.2s |
| disambig | 20 | 85.0% | 95.0% | 100.0% | 1.000 | 4.7s |
| distract | 9 | 88.9% | 100.0% | 100.0% | 1.000 | 12.1s |
| ellipsis | 9 | 88.9% | 100.0% | 100.0% | 1.000 | 22.8s |
| long | 8 | 75.0% | 100.0% | 100.0% | 1.000 | 12.4s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 15.6s |
| shift | 14 | 92.9% | 92.9% | 100.0% | 1.000 | 12.1s |
| slot | 9 | 77.8% | 100.0% | 100.0% | 1.000 | 4.4s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 384 | 88.0% | 95.6% | 95.9% | 0.960 | 12.4s |
| root | 43 | 90.7% | 97.7% | 100.0% | 1.000 | 12.8s |
| start | 17 | 82.4% | 100.0% | 100.0% | 1.000 | 22.3s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 94.1% | 97.1% | 91.3% | 0.922 | 12.2s |
| leaf | 372 | 86.6% | 97.6% | 96.8% | 0.968 | 12.7s |
| none | 38 | 97.4% | 78.9% | - | - | 12.8s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 38 | 97.4% | 78.9% | - | - | 12.8s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 3.7s |
| d2 | 30 | 93.3% | 96.7% | 86.7% | 0.880 | 14.4s |
| d3 | 278 | 86.0% | 97.1% | 96.6% | 0.966 | 12.6s |
| d4 | 88 | 87.5% | 98.9% | 97.2% | 0.972 | 12.8s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 41 | 97.6% | 80.5% | - | - | 12.5s |
| 1 | 369 | 86.4% | 97.6% | 96.8% | 0.968 | 12.7s |
| 2-5 | 16 | 93.8% | 93.8% | 85.7% | 0.857 | 18.2s |
| 33-100 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 12.0s |
| 6-32 | 16 | 93.8% | 100.0% | 92.9% | 0.943 | 9.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 87.5% | 100.0% | - | - | 13.9s |
| colloquial | 69 | 85.5% | 100.0% | 100.0% | 1.000 | 12.0s |
| condition | 32 | 75.0% | 93.8% | 92.9% | 0.929 | 13.4s |
| english | 20 | 95.0% | 100.0% | 100.0% | 1.000 | 12.4s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 15.5s |
| keyword | 20 | 90.0% | 90.0% | 100.0% | 1.000 | 12.1s |
| long | 12 | 91.7% | 100.0% | 100.0% | 1.000 | 16.0s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 11.9s |
| multi_intent | 9 | 66.7% | 77.8% | 66.7% | 0.667 | 31.6s |
| negation | 20 | 80.0% | 95.0% | 100.0% | 1.000 | 13.6s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 11.3s |
| oos_near | 15 | 93.3% | 53.3% | - | - | 4.5s |
| paraphrase | 177 | 89.3% | 98.3% | 98.0% | 0.980 | 13.5s |
| typo | 13 | 92.3% | 100.0% | 100.0% | 1.000 | 10.0s |
| vague | 23 | 91.3% | 95.7% | 91.3% | 0.922 | 5.1s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 46 | 84.8% | 95.7% | 93.3% | 0.933 | 18.2s |
| health | 40 | 92.5% | 100.0% | 94.1% | 0.953 | 16.5s |
| it | 101 | 89.1% | 99.0% | 95.7% | 0.957 | 12.6s |
| life | 41 | 85.4% | 97.6% | 100.0% | 1.000 | 12.7s |
| money | 49 | 77.6% | 95.9% | 100.0% | 1.000 | 11.8s |
| oos | 38 | 97.4% | 78.9% | - | - | 12.8s |
| shop | 41 | 85.4% | 95.1% | 94.7% | 0.947 | 12.2s |
| travel | 44 | 93.2% | 97.7% | 93.8% | 0.938 | 13.0s |
| work | 44 | 88.6% | 97.7% | 94.7% | 0.947 | 12.8s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 93.2% | 98.3% | 98.0% | 0.980 | 12.1s |
| hard | 124 | 83.1% | 90.3% | 89.5% | 0.895 | 12.5s |
| medium | 202 | 88.1% | 98.0% | 97.8% | 0.980 | 12.8s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 303 | 86.5% | 95.4% | 95.3% | 0.955 | 12.8s |
| test | 141 | 91.5% | 97.2% | 98.0% | 0.980 | 12.5s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 84.2% | 98.3% | 72.1% | 23.9% |
| 0.25 | 85.4% | 97.8% | 73.9% | 22.0% |
| 0.30 | 88.1% | 96.7% | 78.5% | 17.0% |
| 0.35 | 90.3% | 95.0% | 83.3% | 12.5% |
| 0.40 | 91.2% | 93.9% | 85.7% | 10.2% |
| 0.50 | 92.3% | 92.2% | 89.2% | 7.2% |
| 0.60 | 91.2% | 86.7% | 91.2% | 5.3% |
| 0.65 | 90.3% | 82.8% | 92.5% | 4.2% |

### 저장 (ingest, 56건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 94.6% [85.4–98.2] (n=56) |
|   └ 올바른 카테고리에 저장 | 97.7% [88.2–99.6] (n=44) |
|   └ 트리/루트 밖 Q&A 거절 | 83.3% [55.2–95.3] (n=12) |
| 계층 F1 | 0.992 [0.978–1.000] (n=44) |
| 초안이 검색에 안 보임 | 100.0% [92.0–100.0] (n=44) |
| 게시 성공 (버전 +1) | 100.0% [92.0–100.0] (n=44) |
| 초안→게시 카테고리 유지 | 97.7% [88.2–99.6] (n=44) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [92.0–100.0] (n=44) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 3 |
| 게시 후 probe 검색 top-k 포함 | 91.3% [73.2–97.6] (n=23) |
| 게시 후 probe 검색 top1 답변 | 91.3% [73.2–97.6] (n=23) |
| 지연 p50 (초안+게시+probe) | 31.8s |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–7.7] (n=46) |

### API 계약 (11건)

통과 100.0% [74.1–100.0] (n=11)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 1 |
| food_tool | food_tool_pan | 1 |
| it_mobile_lost | travel_local_taxi | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| money_bank_xfer_cap | money_bank | 1 |
| money_card_lost | money_card | 1 |
| null | food | 1 |
| null | health_sym_stomach | 1 |
| null | it | 1 |
| null | it_mobile | 1 |
| null | it_sec_pw | 1 |
| null | money_card | 1 |
| null | money_save | 1 |
| null | shop | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_order_receipt | money_tax_income | 1 |
| travel_prep_passport | travel_prep_visa | 1 |
| work_exp_trip | work_exp_claim | 1 |

### 틀린 케이스 (66건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it_office_excel_func | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit_start | false_tree |
| I-WORK-001 | 사원증을 잃어버렸는데 재발급은 어떻게 받나요? | work_hr_onboard | work_hr_resign | branch |
| S-FOOD-001 | 밥솥으로 밥을 했는데 밥알 가운데가 생쌀처럼 씹혀요 | food_cook_base_rice · no answer | food_cook_base_rice · top food_cook_base_rice#1 0.34 reference | answered unanswerable |
| S-FOOD-007 | 배탈 난 뒤 관리 말고요, 여름에 식중독 안 걸리게 미리 조심할 것들 알려주세요 | food_safe_poison · no answer | food_safe_poison · top food_safe_poison#1 0.48 alternative | answered unanswerable |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool_pan · abstained · top food_tool_pan#1 0.24 reference | route deep, abstained |
| S-FOOD-014 | 그거 깔끔하게 까는 요령 좀요 | food_cook_side_egg · no answer | food_cook_side_egg · top food_cook_side_egg#1 0.32 reference | answered unanswerable |
| S-FOOD-018 | 아 근데 냉장고에 있는 요거트가 날짜가 이틀 지났는데 먹어도 될까요? | food_safe_date · no answer | food_safe_date · top food_safe_date#1 0.30 reference | answered unanswerable |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item · no answer | food_store_freeze · top food_store_freeze#1 0.83 recommended | route branch, answered unanswerable |
| S-GAP-009 | 요금제 앱에서 테더링을 켜면 데이터가 두 배로 빠르게 닳는다는데 맞나요? | null · no answer | it_mobile · abstained · top it_mobile#article 0.06 reference | route false_tree |
| S-GAP-012 | 유튜브 프리미엄 자동결제를 해지하려면 어디로 가야 해요? | null · no answer | it_sec_pw · abstained · top it_sec_pw#1 0.01 reference | route false_tree |
| S-HEALTH-007 | My 8-month-old baby is choking on something and can't breathe. What should I do  | health_aid_emerg_choke · no answer | health_aid_emerg_choke · top health_aid_emerg_choke#1 0.38 reference | answered unanswerable |
| S-HEALTH-009 | 달리기 말고 헬스장에서 근력이랑 유산소를 섞어서 하려는데 뭐부터 해야 돼요? | health_fit_start · no answer | health_fit_start · top health_fit_start#1 0.48 alternative | answered unanswerable |
| S-HEALTH-017 | 집에서 다쳤을 때 쓸 응급처치를 종류별로 한번 정리해 두고 싶어요 | health_aid | health_aid · top health_aid_injury_cut#1 0.68 recommended | wrong top1 |
| S-IT-028 | 사이트마다 비밀번호를 다르게 쓰면 다 기억할 수가 없어요. | it_sec_pw · no answer | it_sec_pw · top it_sec_pw#1 0.39 reference | answered unanswerable |
| S-IT-041 | 같은 브랜드로 가요. 옮기기 전에 백업은 어떻게 해 둬요? | it_mobile_move_android · no answer | it_mobile_move_android · top it_mobile_move_android#1 0.69 recommended | answered unanswerable |
| S-IT-043 | 아 잘못 말했어요, 새로 산 게 아이폰이 아니라 갤럭시예요. 그럼 어떻게 옮겨요? | it_mobile_move_cross · no answer | it_mobile_move_cross · top it_mobile_move_cross#1 0.38 reference | answered unanswerable |
| S-IT-051 | 누가 제 이메일 계정 비번이랑 복구 메일까지 바꿔 놔서 들어갈 수가 없어요 | it_sec_hacked · no answer | it_sec_hacked · top it_sec_hacked#1 0.34 reference | answered unanswerable |
| S-IT-502 | 캡처하면 바로 파일로 저장되는 키 뭐였지ㅋㅋ 맨날 그림판에 붙여넣기 귀찮음 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.30 reference | answered unanswerable |
| S-IT-503 | 전체 화면 말고 지금 띄워 놓은 창 하나만 찍고 싶어요 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.76 recommended | answered unanswerable |
| S-IT-504 | 클립보드 기록 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.82 recommended | answered unanswerable |
| S-LIFE-001 | 드럼세탁기 코스가 다 끝났는데 옷이 물에 푹 젖은 채로 나와요. 탈수가 안 된 것 같아요 | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.80 recommended | answered unanswerable |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell · no answer | life_appl_wash_wm_drum_care_smell · top life_appl_wash_wm_drum_care_smell#1 0.65 alternative | answered unanswerable |
| S-LIFE-006 | 통돌이 세탁기로 빨래하면 검은 찌꺼기가 뭍어나와여 청소 어케해요 | life_appl_wash_wm_top_clean · no answer | life_appl_wash_wm_top_clean · top life_appl_wash_wm_top_clean#1 0.82 recommended | answered unanswerable |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.53 alternative | route shallow |
| S-LIFE-030 | 빨래를 조금만 넣었을 때 탈수가 잘 안 되는데 왜 그래요? | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.80 recommended | answered unanswerable |
| S-LIFE-035 | 그럼 아까 그거 빨고 나서 방에서 말릴 때 냄새 안 나게 하려면 어떻게 해요? | life_clean_towel · no answer | life_clean_towel · top life_clean_towel#1 0.59 alternative | answered unanswerable |
| S-LIFE-036 | 겨울만 되면 거실 창틀이 곰팡이로 까매져요 | life_clean_mold · no answer | life_clean_mold · top life_clean_mold#1 0.40 reference | answered unanswerable |
| S-MONEY-002 | 통장 만든 지 일주일 됐는데 하루에 30만 원까지만 보내져요. 앱에서 이체 한도 올리기를 눌러 봐도 안 되는데 이거 계좌 자체가 묶인 건가요? | money_bank_acct_limit · no answer | money_bank_acct_limit · top money_bank_acct_limit#1 0.34 reference | answered unanswerable |
| S-MONEY-005 | 인터넷뱅킹으로 계좌이체할 때 하루 한도를 늘리려면 OTP가 꼭 있어야 하나요? | money_bank_xfer_cap · no answer | money_bank_xfer_cap · top money_bank_xfer_cap#1 0.67 recommended | answered unanswerable |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost · no answer | money_card · top money_card_lost#1 0.72 recommended | route shallow, answered unanswerable |
| S-MONEY-012 | 온라인 쇼핑몰에서 결제하는데 한도 초과라고 승인이 안 나요. 쇼핑몰 오류가 아니라 제 신용카드 한도가 찬 거라는데 지금 바로 결제하려면 어떻게  | money_card_limit · no answer | money_card_limit · top money_card_limit#1 0.40 alternative | answered unanswerable |
| S-MONEY-017 | 직장인인데 주말마다 과외해서 번 돈이 좀 있어요. 이것도 회사 연말정산으로 끝나나요, 아니면 제가 따로 신고해야 하나요? | money_tax_income · no answer | money_tax_income · top money_tax_income#1 0.44 alternative | answered unanswerable |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money_bank · top money_bank_cert#1 0.58 alternative | route shallow |
| S-MONEY-027 | 마통은요? | money_loan_credit · no answer | money_loan_credit · top money_loan_credit#1 0.55 alternative | answered unanswerable |
| S-MONEY-033 | 한도를 당장 다시 살리는 방법은 없어요? | money_card_limit · no answer | money_card_limit · top money_card_limit#1 0.37 reference | answered unanswerable |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit · no answer | money_bank_acct_limit · top money_bank_acct_limit#1 0.60 alternative | answered unanswerable |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card#article 0.05 reference | route false_tree |
| S-MONEY-039 | 휴면 예금이 서민금융진흥원으로 넘어갔다는데 그럼 이제 못 돌려받는 거예요? | money_bank_acct_dormant · no answer | money_bank_acct_dormant · top money_bank_acct_dormant#1 0.33 reference | answered unanswerable |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_power_boot#1 0.16 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money_save · top money_save#article 0.53 alternative | route false_tree, answered unanswerable |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym_stomach · abstained · top health_sym_stomach#1 0.06 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.12 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_missing#1 0.09 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt · no answer | money_tax_income · abstained · top money_tax_income#1 0.20 reference | route domain |
| S-SHOP-014 | 지난주 세일 때 셔츠 세 벌이랑 청바지 하나를 한 번에 주문했어요. 그런데 어제 매장에 들렀다가 같은 청바지를 더 싸게 팔길래 거기서 사 버렸거 | shop_order_cancel · no answer | shop_order_cancel · top shop_order_cancel#1 0.32 reference | answered unanswerable |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay · no answer | shop_order_cancel · top shop_order_cancel#1 0.54 alternative | route branch, answered unanswerable |
| S-SHOP-033 | 입어 보려고 택을 뗐는데 이제 반품 안 되나요? | shop_ret_return_mind · no answer | shop_ret_return_mind · top shop_ret_return_mind#1 0.56 alternative | answered unanswerable |
| S-TRAVEL-008 | 여권 만료일이 넉 달 뒤인데 이걸로 동남아 여행 가도 문제없을까요? | travel_prep_passport | travel_prep_visa · abstained · top travel_prep_visa#1 0.02 reference | route branch, abstained |
| S-TRAVEL-023 | 그 조건이면 지금 취소해도 한 푼도 못 받는 거예요? | travel_stay_cancel · no answer | travel_stay_cancel · top travel_stay_cancel#1 0.40 alternative | answered unanswerable |
| S-TRAVEL-032 | 공항에서 짐 찾았는데 캐리어 손잡이가 부러져서 나왔어요 | travel_air_bag_lost · no answer | travel_air_bag_lost · top travel_air_bag_lost#1 0.32 reference | answered unanswerable |
| S-WORK-011 | 영수증 잃어버림ㅠㅠ 거래처랑 점심 먹은 거 내 돈으로 냈는데 이거 정산 못 받나요 | work_exp_claim · no answer | work_exp_claim · top work_exp_claim#1 0.67 recommended | answered unanswerable |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work_pay_slip · abstained · top work_pay_slip#1 0.11 reference | abstained |
| S-WORK-029 | 내일도 못 나갈 것 같은데 회사엔 어떻게 처리해요? | work_leave_sick · no answer | work_leave_sick · top work_leave_sick#1 0.62 alternative | answered unanswerable |
| S-WORK-036 | 퇴사하고 나서 경력증명서 받을 방법이 있나요? | work_hr_cert · no answer | work_hr_cert · top work_hr_cert#1 0.32 reference | answered unanswerable |
| S-XD-003 | 쇼핑몰 주문을 카드 결제 후 바로 취소했는데 카드 명세서에는 아직 청구돼 있어요 | shop_ret_refund_card · no answer | shop_ret_refund_card · top shop_ret_refund_card#1 0.47 alternative | answered unanswerable |
| S-XD-004 | 해외여행 가서 카드 긁으면 수수료가 얼마나 붙어요? | money_card_abroad · no answer | money_card_abroad · top money_card_abroad#1 0.48 alternative | answered unanswerable |
| S-XD-007 | 냉장고에 둔 우유가 표시된 날짜에서 이틀 지났는데 버려야 하나요? | food_safe_date · no answer | food_safe_date · top food_safe_date#1 0.34 reference | answered unanswerable |
| S-XD-008 | 회사 노트북이 와이파이에 연결은 되는데 몇 분마다 끊겨요 | it_net_wifi_drop · no answer | it_net_wifi_drop · top it_net_wifi_drop#1 0.43 alternative | answered unanswerable |
| S-XD-013 | 택시에 휴대폰을 두고 내렸어요. 폰 위치부터 찾을 수 있나요? | it_mobile_lost | travel_local_taxi · abstained · top no items | route domain, abstained |

