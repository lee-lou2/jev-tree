# 평가 결과 — 20260924-020818-jev+jev_llm

- 실행: 2026-09-24T02:08:18Z → 2026-09-24T03:33:45Z (UTC) · git `36656f0`
- 데이터셋: `kb` v1.0.0 · kb `489e5fca3c7c` · cases `11b27f3d7b79`
- 설정: routers ["jev","jev_llm"] · variants ["deep","standard","shallow","flat","sparse"] · suites ["all"] · split all · repeat 1 · concurrency 10 · beam_width "case/default" · limit "case/default"
- 모델(jev): jev `jev-latest`
- 모델(jev_llm): jev `jev-latest` · LLM `gpt-6-luna`
- 메모: re-run from DB-registered credentials; jev_llm=gpt-6-luna

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev | deep | 433 | 92.1% | 89.6% | 0.926 | 91.3% | 0.918 | 0.907 | 0.764 | 3.1% | 2.7% | 1.2s / 2.0s | 4.4 | 0 | 86.8% | 100.0% |
| jev | standard | 432 | 92.4% | 90.0% | 0.929 | 91.6% | 0.919 | 0.909 | 0.764 | 3.1% | 3.0% | 1.1s / 1.8s | 4.1 | 0 | 86.8% | 100.0% |
| jev | shallow | 388 | 95.4% | 93.0% | 0.937 | 95.3% | 0.955 | 0.942 | 0.745 | 3.9% | 1.5% | 0.8s / 1.4s | 3.1 | 0 | 93.2% | 100.0% |
| jev | flat | 357 | 97.5% | 97.5% | 0.975 | 97.2% | 0.973 | 0.959 | 1.000 | 0.0% | 0.0% | 0.6s / 1.2s | 2.1 | 0 | 100.0% | 100.0% |
| jev | sparse | 432 | 83.6% | 89.6% | 0.926 | 89.1% | 0.895 | 0.899 | 0.764 | 19.4% | 6.9% | 1.0s / 1.4s | 3.8 | 0 | 87.5% | 100.0% |
| jev_llm | deep | 433 | 94.9% | 88.0% | 0.924 | 94.3% | 0.949 | 0.939 | 0.737 | 1.6% | 2.7% | 11.8s / 46.7s | 1.4 | 0 | 92.6% | 100.0% |
| jev_llm | standard | 432 | 93.5% | 87.7% | 0.922 | 92.9% | 0.937 | 0.931 | 0.714 | 1.6% | 2.4% | 23.5s / 52.5s | 1.4 | 0 | 95.6% | 100.0% |
| jev_llm | shallow | 388 | 94.8% | 89.2% | 0.920 | 94.7% | 0.949 | 0.941 | 0.731 | 3.9% | 2.1% | 14.5s / 46.1s | 1.4 | 0 | 93.2% | 100.0% |
| jev_llm | flat | 357 | 97.8% | 97.5% | 0.975 | 97.5% | 0.976 | 0.963 | 0.952 | 0.0% | 0.3% | 3.3s / 39.0s | 1.2 | 0 | 100.0% | 100.0% |
| jev_llm | sparse | 432 | 87.3% | 88.4% | 0.922 | 96.6% | 0.968 | 0.970 | 0.750 | 18.6% | 2.9% | 7.6s / 46.5s | 1.2 | 0 | 91.1% | 100.0% |

## jev · deep

### 검색 (433건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 92.1% [89.2–94.3] (n=433) |
|   └ 답할 수 있는 질문 | 91.3% [88.0–93.8] (n=369) |
|   └ 답할 수 없는 질문(보류가 정답) | 96.9% [89.3–99.1] (n=64) |
| 확신 정답률 (top1 정답 + recommended) | 86.2% [82.3–89.3] (n=369) |
| 라우팅 정확도 (정확 일치) | 89.6% [86.4–92.1] (n=433) |
| 계층 F1 (hF1) | 0.926 [0.903–0.948] (n=433) |
| 트리 밖 판정 precision | 100.0% [84.5–100.0] (n=21) |
| 트리 밖 판정 recall | 61.8% [45.0–76.1] (n=34) |
| 트리 밖 판정 F1 | 0.764 |
| 오보류율 (답할 수 있는데 보류) | 2.7% [1.5–4.9] (n=369) |
| 오답변율 (답할 수 없는데 답함) | 3.1% [0.9–10.7] (n=64) |
| Hit@1 | 91.3% [88.0–93.8] (n=369) |
| Hit@3 | 92.1% [88.9–94.5] (n=369) |
| Hit@k (limit) | 92.4% [89.3–94.7] (n=369) |
| MRR@k | 0.918 [0.890–0.945] (n=369) |
| nDCG@k | 0.907 [0.879–0.934] (n=369) |
| Recall@k | 0.911 [0.883–0.939] (n=369) |
| recommended 정밀도 | 0.963 [0.944–0.982] (n=327) |
| 라우팅이 맞았을 때 Hit@1 | 98.8% [97.0–99.5] (n=339) |
| 라우팅이 맞았을 때 MRR | 0.993 [0.986–1.000] (n=339) |
| 지연 p50 / p90 / p95 / 평균 | 1.2s / 1.9s / 2.0s / 1.3s |
| 라우팅 구간 평균 | 0.9s |
| 랭킹 후보 수 평균 | 15.6 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 4.39 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 | d5 | d6 | d7 |
|---|---:|---:|---:|---:|---:|---:|---:|
| 일치율 | 97.7% (n=352) | 95.0% (n=342) | 92.0% (n=323) | 91.1% (n=146) | 80.0% (n=20) | 80.0% (n=10) | 75.0% (n=8) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 19 | 6 | 8 | 447 | 18 | 3 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.7s |
| internal | 24 | 66.7% | 79.2% | 66.7% | 0.712 | 1.2s |
| leaf | 345 | 93.0% | 92.8% | 93.0% | 0.932 | 1.3s |
| no_answer | 27 | 100.0% | 92.6% | - | - | 1.1s |
| oos | 34 | 94.1% | 61.8% | - | - | 0.4s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 92.4% | 91.4% | 92.2% | 0.922 | 1.2s |
| single | 328 | 92.1% | 89.0% | 91.0% | 0.916 | 1.2s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 328 | 92.1% | 89.0% | 91.0% | 0.916 | 1.2s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 1.3s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 1.3s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.3s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.3s |
| ellipsis | 9 | 88.9% | 88.9% | 88.9% | 0.889 | 1.3s |
| long | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 1.2s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.1s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 1.2s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.3s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 373 | 91.7% | 89.0% | 91.0% | 0.915 | 1.3s |
| root | 43 | 93.0% | 90.7% | 92.9% | 0.929 | 0.5s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 35 | 77.1% | 82.9% | 66.7% | 0.712 | 1.2s |
| leaf | 364 | 93.4% | 92.9% | 93.0% | 0.932 | 1.3s |
| none | 34 | 94.1% | 61.8% | - | - | 0.4s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 34 | 94.1% | 61.8% | - | - | 0.4s |
| d1 | 10 | 60.0% | 60.0% | 60.0% | 0.600 | 1.2s |
| d2 | 30 | 86.7% | 93.3% | 78.9% | 0.846 | 1.1s |
| d3 | 199 | 94.0% | 94.0% | 93.5% | 0.935 | 1.2s |
| d4 | 140 | 94.3% | 92.9% | 94.1% | 0.945 | 1.4s |
| d5 | 10 | 80.0% | 80.0% | 80.0% | 0.800 | 1.9s |
| d6 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 2.1s |
| d7 | 8 | 75.0% | 75.0% | 75.0% | 0.750 | 2.3s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 37 | 94.6% | 64.9% | - | - | 0.4s |
| 101+ | 4 | 50.0% | 50.0% | 50.0% | 0.500 | 2.9s |
| 2-5 | 311 | 92.9% | 92.3% | 92.5% | 0.928 | 1.2s |
| 33-100 | 61 | 91.8% | 91.8% | 91.5% | 0.915 | 1.8s |
| 6-32 | 20 | 85.0% | 95.0% | 72.7% | 0.803 | 1.1s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| colloquial | 69 | 95.7% | 97.1% | 95.4% | 0.962 | 1.2s |
| condition | 32 | 87.5% | 84.4% | 87.1% | 0.871 | 1.4s |
| english | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.4s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 1.4s |
| keyword | 20 | 85.0% | 85.0% | 85.0% | 0.850 | 1.3s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.4s |
| mixed | 13 | 92.3% | 92.3% | 92.3% | 0.923 | 1.5s |
| multi_intent | 9 | 100.0% | 77.8% | 100.0% | 1.000 | 1.2s |
| negation | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.4s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.3s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 1.3s |
| paraphrase | 177 | 93.8% | 92.7% | 92.8% | 0.928 | 1.2s |
| typo | 13 | 92.3% | 92.3% | 92.3% | 0.923 | 1.5s |
| vague | 24 | 66.7% | 79.2% | 66.7% | 0.712 | 1.2s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 45 | 88.9% | 88.9% | 88.1% | 0.887 | 1.4s |
| health | 39 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| it | 100 | 97.0% | 96.0% | 96.8% | 0.968 | 1.5s |
| life | 41 | 90.2% | 87.8% | 89.2% | 0.892 | 1.3s |
| money | 48 | 89.6% | 91.7% | 88.9% | 0.896 | 1.2s |
| oos | 34 | 94.1% | 61.8% | - | - | 0.4s |
| shop | 40 | 77.5% | 80.0% | 75.7% | 0.770 | 1.1s |
| travel | 43 | 95.3% | 95.3% | 94.7% | 0.947 | 1.2s |
| work | 43 | 90.7% | 90.7% | 90.0% | 0.912 | 1.2s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 95.8% | 97.5% | 95.5% | 0.964 | 1.2s |
| hard | 113 | 90.3% | 79.6% | 88.5% | 0.885 | 1.3s |
| medium | 202 | 91.1% | 90.6% | 90.1% | 0.904 | 1.3s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 293 | 91.8% | 88.4% | 90.8% | 0.911 | 1.2s |
| test | 140 | 92.9% | 92.1% | 92.4% | 0.931 | 1.2s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 91.5% | 98.1% | 91.8% | 7.8% |
| 0.25 | 91.9% | 97.6% | 92.8% | 4.7% |
| 0.30 | 92.1% | 97.3% | 93.4% | 3.1% |
| 0.35 | 91.9% | 96.7% | 93.6% | 3.1% |
| 0.40 | 91.9% | 96.5% | 93.9% | 3.1% |
| 0.50 | 91.0% | 94.3% | 94.9% | 3.1% |
| 0.60 | 89.6% | 90.8% | 96.7% | 1.6% |
| 0.65 | 88.0% | 88.3% | 97.2% | 1.6% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 86.8% [76.7–92.9] (n=68) |
|   └ 올바른 카테고리에 저장 | 92.9% [83.0–97.2] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 58.3% [32.0–80.7] (n=12) |
| 계층 F1 | 0.963 [0.925–1.000] (n=56) |
| 초안이 검색에 안 보임 | 100.0% [93.6–100.0] (n=56) |
| 게시 성공 (버전 +1) | 98.2% [90.6–99.7] (n=56) |
| 초안→게시 카테고리 유지 | 98.2% [90.6–99.7] (n=56) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 98.2% [90.6–99.7] (n=56) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 7 |
| 게시 후 probe 검색 top-k 포함 | 87.0% [67.9–95.5] (n=23) |
| 게시 후 probe 검색 top1 답변 | 87.0% [67.9–95.5] (n=23) |
| 지연 p50 (초안+게시+probe) | 2.4s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food | food_cook | 2 |
| food_store_item | food_store_freeze | 2 |
| money_bank_acct_limit | money_bank_xfer_cap | 2 |
| null | it | 2 |
| null | money | 2 |
| food_tool | food_tool_pan | 1 |
| it_office_excel_func | work | 1 |
| it_pc_peri_monitor | it_pc_win | 1 |
| it_pc_win_perf_disk | it_mobile_space | 1 |
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

### 틀린 케이스 (59건 중 59건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-HEALTH-002 | 헬스 하고 다음 날 근육통이 심하면 운동 쉬어야 됨? | health_fit_start | health_fit_start | exact |
| I-IT-003 | How do I turn on Bluetooth on my Windows laptop when the toggle is missing? | it_pc_peri_bt | it_pc_win | branch |
| I-MONEY-006 | 휴면 예금으로 넘어간 돈은 영영 사라지나요? | money_bank_acct_dormant | money_save_deposit | branch |
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit | false_tree |
| I-SHOP-007 | 호텔 예약을 취소하면 수수료가 얼마나 나오나요? | null | shop_order_cancel | false_tree |
| I-WORK-001 | 사원증을 잃어버렸는데 재발급은 어떻게 받나요? | work_hr_onboard | work_hr | shallow |
| I-XD-002 | 체크카드로 결제한 주문을 취소하면 환불은 며칠 걸리나요? | shop_ret_refund_card | shop_order_cancel | branch |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_ret_refund | false_tree |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food_cook · top food_cook#article 0.52 alternative | route deep, wrong top1 |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.46 alternative | wrong top1 |
| S-FOOD-012 | 스테인리스 냄비 바닥에 까맣게 탄 자국은 어떻게 벗겨요? | food_tool · no answer | food_tool_pan · abstained · top food_tool_pan#2 0.03 reference | route deep |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.79 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.88 recommended | route branch, wrong top1 |
| S-IT-005 | 씨드라이브가 빨강색으로 바꼇는데 어떻게 비워요 | it_pc_win_perf_disk | it_mobile_space · top it_mobile_space#1 0.55 alternative | route branch, wrong top1 |
| S-IT-035 | 듀얼 모니터 쓰는데 작업 표시줄을 한쪽 화면에만 나오게 할 수 있나요? | it_pc_peri_monitor · no answer | it_pc_win · abstained · top it_pc_win_keys#alt_space 0.19 reference | route branch |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · top work_leave_annual_left#1 0.30 reference | route domain, wrong top1 |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell | life_clean_towel · top life_clean_towel#1 0.58 alternative | route branch, wrong top1 |
| S-LIFE-004 | 드럼 고무패킹 곰팡이 | life_appl_wash_wm_drum_care_gasket | life_clean_mold · abstained · top life_clean_mold#2 0.18 reference | route branch, abstained |
| S-LIFE-008 | 건조기 돌리면 안 되고 그냥 널어서 말려야 하는 옷은 어떤 거예요? | life_appl_wash_dryer_shrink | life_clean · abstained · top life_clean_towel#2 0.23 reference | route branch, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.66 recommended | route shallow |
| S-LIFE-020 | 자취 시작하고 집안일이 처음이라 뭐가 고장 나거나 지저분해지면 어디서부터 봐야 할지 모르겠어요 | life | life_home · top life_home#article 0.85 recommended | route deep, wrong top1 |
| S-MONEY-002 | 통장 만든 지 일주일 됐는데 하루에 30만 원까지만 보내져요. 앱에서 이체 한도 올리기를 눌러 봐도 안 되는데 이거 계좌 자체가 묶인 건가요? | money_bank_acct_limit | money_bank_xfer_cap · top money_bank_xfer_cap#3 0.55 alternative | route branch, wrong top1 |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec_phish#1 0.09 reference | route domain, abstained |
| S-MONEY-021 | 제 명의 신용카드 관련해서 이것저것 궁금한 게 있는데요 | money_card | money_card · top money_card_lost#4 0.73 recommended | wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.87 recommended | route domain, wrong top1 |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit | money_bank_xfer_cap · top money_bank_xfer_cap#1 0.40 alternative | route branch, wrong top1 |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card_limit#2 0.10 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.26 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · top money_save#article 0.52 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health_aid_emerg#article 0.08 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_bank_xfer_cap#2 0.18 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.08 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money_save · abstained · top money_save#article 0.06 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_wm_drum_care_smell#2 0.16 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it · abstained · top it_mobile_batt_drain#2 0.21 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym_stomach · abstained · top health_sym_stomach#3 0.15 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.13 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_track#3 0.14 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.39 reference | route domain, wrong top1 |
| S-SHOP-009 | 카드로 산 운동화를 반품했는데 카드사 앱 승인 내역에 아직 취소가 안 보여요. 보통 며칠 걸려요? | shop_ret_refund_card | money_card · abstained · top money_card_limit#2 0.08 reference | route domain, abstained |
| S-SHOP-017 | 온라인으로 산 물건 때문에 문의하려는데 뭐부터 봐야 할지 모르겠어요 | shop | shop_ret · top shop_ret#article 0.78 recommended | route deep, wrong top1 |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.44 alternative | wrong top1 |
| S-SHOP-024 | 무통장으로 낸 건요? | shop_ret_refund_bank | shop_order_pay · abstained · top shop_order_pay#3 0.30 reference | route branch, abstained |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money_tax · abstained · top money_tax#article 0.20 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.49 alternative | route branch, wrong top1 |
| S-TRAVEL-018 | 해외 나가기 전에 서류나 폰 쪽으로 챙겨야 할 게 뭐가 있죠? | travel_prep | travel_prep_data · top travel_prep_data#3 0.54 alternative | route deep, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.77 recommended | wrong top1 |
| S-WORK-008 | 연말정산 공제 얘기 말고요, 매달 월급에서 떼가는 건강보험이랑 소득세 항목이 뭔지 알고 싶어요 | work_pay_slip | money_tax · abstained · top money_tax#article 0.17 reference | route domain, abstained |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work_leave · top work_leave_annual_use#2 0.43 alternative | route shallow |
| S-WORK-032 | 경조금 받으려면 증빙으로 뭘 올려야 하죠? | work_leave_event | work_pay · abstained · top work_pay#article 0.07 reference | route branch, abstained |
| S-XD-003 | 쇼핑몰 주문을 카드 결제 후 바로 취소했는데 카드 명세서에는 아직 청구돼 있어요 | shop_ret_refund_card | shop_order_cancel · abstained · top shop_order_cancel#3 0.20 reference | route branch, abstained |
| S-XD-014 | 여행용 eSIM을 샀는데 폰에 어떻게 설치해서 써요? | travel_prep_data | it_mobile · abstained · top it_mobile#article 0.09 reference | route domain, abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.61 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.84 recommended | route branch, wrong top1 |
| S-XD-045 | 요리나 식재료 관련해서 도움을 받고 싶어요 | food | food_cook · top food_cook#article 0.62 alternative | route deep, wrong top1 |
| S-XD-047 | 카드로 산 물건 결제를 취소했는데 명세서에 아직 남아 있어요 | shop_ret_refund_card | shop_order_pay · top shop_order_pay#1 0.50 alternative | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.82 recommended | route false_tree, answered unanswerable |

## jev · standard

### 검색 (432건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 92.4% [89.5–94.5] (n=432) |
|   └ 답할 수 있는 질문 | 91.6% [88.3–94.0] (n=368) |
|   └ 답할 수 없는 질문(보류가 정답) | 96.9% [89.3–99.1] (n=64) |
| 확신 정답률 (top1 정답 + recommended) | 86.4% [82.5–89.5] (n=368) |
| 라우팅 정확도 (정확 일치) | 90.0% [86.9–92.5] (n=432) |
| 계층 F1 (hF1) | 0.929 [0.906–0.951] (n=432) |
| 트리 밖 판정 precision | 100.0% [84.5–100.0] (n=21) |
| 트리 밖 판정 recall | 61.8% [45.0–76.1] (n=34) |
| 트리 밖 판정 F1 | 0.764 |
| 오보류율 (답할 수 있는데 보류) | 3.0% [1.7–5.3] (n=368) |
| 오답변율 (답할 수 없는데 답함) | 3.1% [0.9–10.7] (n=64) |
| Hit@1 | 91.6% [88.3–94.0] (n=368) |
| Hit@3 | 91.8% [88.6–94.2] (n=368) |
| Hit@k (limit) | 92.7% [89.5–94.9] (n=368) |
| MRR@k | 0.919 [0.891–0.946] (n=368) |
| nDCG@k | 0.909 [0.882–0.937] (n=368) |
| Recall@k | 0.914 [0.887–0.942] (n=368) |
| recommended 정밀도 | 0.968 [0.949–0.986] (n=326) |
| 라우팅이 맞았을 때 Hit@1 | 98.8% [97.0–99.5] (n=340) |
| 라우팅이 맞았을 때 MRR | 0.992 [0.983–1.000] (n=340) |
| 지연 p50 / p90 / p95 / 평균 | 1.1s / 1.6s / 1.8s / 1.1s |
| 라우팅 구간 평균 | 0.8s |
| 랭킹 후보 수 평균 | 15.6 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 4.11 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 97.4% (n=351) | 95.0% (n=341) | 92.9% (n=322) | 92.6% (n=81) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 16 | 7 | 9 | 448 | 18 | 2 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.9s |
| internal | 23 | 56.5% | 73.9% | 56.5% | 0.614 | 1.1s |
| leaf | 345 | 93.9% | 93.6% | 93.9% | 0.939 | 1.1s |
| no_answer | 27 | 100.0% | 92.6% | - | - | 1.0s |
| oos | 34 | 94.1% | 61.8% | - | - | 0.3s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 93.3% | 92.4% | 93.2% | 0.932 | 1.1s |
| single | 327 | 92.0% | 89.3% | 90.9% | 0.914 | 1.1s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 327 | 92.0% | 89.3% | 90.9% | 0.914 | 1.1s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 1.1s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 1.1s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.0s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.1s |
| ellipsis | 9 | 88.9% | 88.9% | 88.9% | 0.889 | 1.2s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.1s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 1.1s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 372 | 91.9% | 89.5% | 91.3% | 0.916 | 1.1s |
| root | 43 | 93.0% | 90.7% | 92.9% | 0.929 | 0.5s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 0.5s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 70.6% | 79.4% | 56.5% | 0.614 | 1.1s |
| leaf | 364 | 94.2% | 93.7% | 93.9% | 0.939 | 1.1s |
| none | 34 | 94.1% | 61.8% | - | - | 0.3s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 34 | 94.1% | 61.8% | - | - | 0.3s |
| d1 | 10 | 60.0% | 60.0% | 60.0% | 0.600 | 1.1s |
| d2 | 30 | 80.0% | 90.0% | 68.4% | 0.743 | 1.1s |
| d3 | 271 | 93.7% | 93.7% | 93.3% | 0.933 | 1.0s |
| d4 | 87 | 95.4% | 93.1% | 95.2% | 0.952 | 1.4s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 37 | 94.6% | 64.9% | - | - | 0.3s |
| 101+ | 4 | 50.0% | 50.0% | 50.0% | 0.500 | 2.8s |
| 2-5 | 311 | 93.9% | 93.2% | 93.6% | 0.936 | 1.1s |
| 33-100 | 61 | 91.8% | 91.8% | 91.5% | 0.915 | 1.7s |
| 6-32 | 19 | 73.7% | 89.5% | 50.0% | 0.588 | 1.0s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.1s |
| colloquial | 69 | 97.1% | 97.1% | 96.9% | 0.969 | 1.1s |
| condition | 32 | 90.6% | 87.5% | 90.3% | 0.903 | 1.1s |
| english | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.4s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 1.1s |
| keyword | 20 | 85.0% | 85.0% | 85.0% | 0.850 | 1.1s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.1s |
| mixed | 13 | 92.3% | 92.3% | 92.3% | 0.923 | 1.2s |
| multi_intent | 9 | 100.0% | 88.9% | 100.0% | 1.000 | 1.1s |
| negation | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.1s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.3s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 1.1s |
| paraphrase | 177 | 93.8% | 92.7% | 92.8% | 0.928 | 1.1s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| vague | 23 | 56.5% | 73.9% | 56.5% | 0.614 | 1.1s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 45 | 88.9% | 88.9% | 88.1% | 0.887 | 1.2s |
| health | 39 | 97.4% | 100.0% | 97.1% | 0.975 | 1.0s |
| it | 100 | 97.0% | 96.0% | 96.8% | 0.968 | 1.3s |
| life | 40 | 90.0% | 87.5% | 88.9% | 0.889 | 1.3s |
| money | 48 | 93.8% | 95.8% | 93.3% | 0.939 | 1.0s |
| oos | 34 | 94.1% | 61.8% | - | - | 0.3s |
| shop | 40 | 75.0% | 77.5% | 73.0% | 0.743 | 1.0s |
| travel | 43 | 95.3% | 95.3% | 94.7% | 0.947 | 1.0s |
| work | 43 | 93.0% | 93.0% | 92.5% | 0.925 | 1.1s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 95.8% | 96.6% | 95.5% | 0.959 | 1.1s |
| hard | 112 | 92.0% | 82.1% | 90.9% | 0.909 | 1.1s |
| medium | 202 | 90.6% | 90.6% | 89.5% | 0.898 | 1.1s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 292 | 91.8% | 89.0% | 90.8% | 0.911 | 1.1s |
| test | 140 | 93.6% | 92.1% | 93.2% | 0.934 | 1.1s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 91.7% | 97.8% | 92.3% | 7.8% |
| 0.25 | 92.1% | 97.3% | 93.4% | 4.7% |
| 0.30 | 92.4% | 97.0% | 93.9% | 3.1% |
| 0.35 | 92.1% | 96.5% | 94.1% | 3.1% |
| 0.40 | 92.1% | 95.9% | 94.6% | 3.1% |
| 0.50 | 91.0% | 93.8% | 95.4% | 3.1% |
| 0.60 | 89.6% | 90.8% | 96.7% | 1.6% |
| 0.65 | 88.2% | 88.3% | 97.5% | 1.6% |

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
| 지연 p50 (초안+게시+probe) | 2.0s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food | food_cook | 2 |
| food_store_item | food_store_freeze | 2 |
| null | it | 2 |
| null | money | 2 |
| food_tool | food_tool_pan | 1 |
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
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | health | 1 |
| null | health_sym_stomach | 1 |
| null | life | 1 |

### 틀린 케이스 (56건 중 56건)

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
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food_cook · top food_cook#article 0.61 alternative | route deep, wrong top1 |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.46 alternative | wrong top1 |
| S-FOOD-012 | 스테인리스 냄비 바닥에 까맣게 탄 자국은 어떻게 벗겨요? | food_tool · no answer | food_tool_pan · abstained · top food_tool_pan#2 0.04 reference | route deep |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.81 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.88 recommended | route branch, wrong top1 |
| S-HEALTH-017 | 집에서 다쳤을 때 쓸 응급처치를 종류별로 한번 정리해 두고 싶어요 | health_aid | health_aid · top health_aid_injury_sprain#1 0.62 alternative | wrong top1 |
| S-IT-031 | 엄마라면서 폰 액정이 깨졌다고 상품권 좀 사 달라는 문자가 왔는데 이상해요 | it_sec_phish | money · abstained · top money_bank_xfer_cap#2 0.29 reference | route domain, abstained |
| S-IT-035 | 듀얼 모니터 쓰는데 작업 표시줄을 한쪽 화면에만 나오게 할 수 있나요? | it_pc_peri_monitor · no answer | it_pc_win · abstained · top it_pc_win_keys#alt_space 0.16 reference | route branch |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · abstained · top work_pay_sev#2 0.22 reference | route domain, abstained |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell | life_clean_towel · top life_clean_towel#1 0.57 alternative | route branch, wrong top1 |
| S-LIFE-004 | 드럼 고무패킹 곰팡이 | life_appl_wash_wm_drum_care_gasket | life_clean_mold · abstained · top life_clean_mold#2 0.18 reference | route branch, abstained |
| S-LIFE-008 | 건조기 돌리면 안 되고 그냥 널어서 말려야 하는 옷은 어떤 거예요? | life_appl_wash_dryer_shrink | life_clean · abstained · top life_clean_towel#2 0.20 reference | route branch, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.68 recommended | route shallow |
| S-LIFE-020 | 자취 시작하고 집안일이 처음이라 뭐가 고장 나거나 지저분해지면 어디서부터 봐야 할지 모르겠어요 | life | life_home · top life_home#article 0.85 recommended | route deep, wrong top1 |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec_hacked#3 0.08 reference | route domain, abstained |
| S-MONEY-021 | 제 명의 신용카드 관련해서 이것저것 궁금한 게 있는데요 | money_card | money_card · top money_card_lost#4 0.73 recommended | wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.86 recommended | route domain, wrong top1 |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card_limit#2 0.09 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_keys#alt_tab 0.26 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · top money_save#article 0.58 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health_sym#article 0.08 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_card_lost#4 0.24 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.08 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money_save · abstained · top money_save#article 0.07 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_wm_drum_water_spin#2 0.11 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it · abstained · top it_pc_win_perf_slow#1 0.20 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym_stomach · abstained · top health_sym_stomach#3 0.15 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.11 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_addr#2 0.12 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.36 reference | route domain, wrong top1 |
| S-SHOP-009 | 카드로 산 운동화를 반품했는데 카드사 앱 승인 내역에 아직 취소가 안 보여요. 보통 며칠 걸려요? | shop_ret_refund_card | money_card · abstained · top money_card_limit#2 0.08 reference | route domain, abstained |
| S-SHOP-017 | 온라인으로 산 물건 때문에 문의하려는데 뭐부터 봐야 할지 모르겠어요 | shop | shop_ret · top shop_ret#article 0.76 recommended | route deep, wrong top1 |
| S-SHOP-018 | 받은 물건이 맘에 안 들어서 처리하고 싶은데, 돌려보내는 거랑 바꾸는 거 중에 어떤 선택지가 있는지부터 알려 주세요 | shop_ret | shop_ret_return_mind · top shop_ret_return_mind#1 0.39 reference | route deep, wrong top1 |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.46 alternative | wrong top1 |
| S-SHOP-024 | 무통장으로 낸 건요? | shop_ret_refund_bank | shop_order_pay · top shop_order_pay#3 0.34 reference | route branch, wrong top1 |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money_tax · abstained · top money_tax#article 0.18 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.51 alternative | route branch, wrong top1 |
| S-TRAVEL-018 | 해외 나가기 전에 서류나 폰 쪽으로 챙겨야 할 게 뭐가 있죠? | travel_prep | travel_prep_data · top travel_prep_data#3 0.58 alternative | route deep, wrong top1 |
| S-WORK-008 | 연말정산 공제 얘기 말고요, 매달 월급에서 떼가는 건강보험이랑 소득세 항목이 뭔지 알고 싶어요 | work_pay_slip | money_tax · abstained · top money_tax#article 0.17 reference | route domain, abstained |
| S-WORK-032 | 경조금 받으려면 증빙으로 뭘 올려야 하죠? | work_leave_event | work_pay · abstained · top work_pay#article 0.08 reference | route branch, abstained |
| S-XD-003 | 쇼핑몰 주문을 카드 결제 후 바로 취소했는데 카드 명세서에는 아직 청구돼 있어요 | shop_ret_refund_card | shop_order_cancel · abstained · top shop_order_cancel#3 0.20 reference | route branch, abstained |
| S-XD-014 | 여행용 eSIM을 샀는데 폰에 어떻게 설치해서 써요? | travel_prep_data | it_mobile · abstained · top it_mobile#article 0.10 reference | route domain, abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.59 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.82 recommended | route branch, wrong top1 |
| S-XD-045 | 요리나 식재료 관련해서 도움을 받고 싶어요 | food | food_cook · top food_cook#article 0.61 alternative | route deep, wrong top1 |
| S-XD-047 | 카드로 산 물건 결제를 취소했는데 명세서에 아직 남아 있어요 | shop_ret_refund_card | shop_order_pay · top shop_order_pay#1 0.48 alternative | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.85 recommended | route false_tree, answered unanswerable |

## jev · shallow

### 검색 (388건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 95.4% [92.8–97.0] (n=388) |
|   └ 답할 수 있는 질문 | 95.3% [92.4–97.1] (n=337) |
|   └ 답할 수 없는 질문(보류가 정답) | 96.1% [86.8–98.9] (n=51) |
| 확신 정답률 (top1 정답 + recommended) | 88.7% [84.9–91.7] (n=337) |
| 라우팅 정확도 (정확 일치) | 93.0% [90.1–95.2] (n=388) |
| 계층 F1 (hF1) | 0.937 [0.913–0.960] (n=388) |
| 트리 밖 판정 precision | 100.0% [83.2–100.0] (n=19) |
| 트리 밖 판정 recall | 59.4% [42.3–74.5] (n=32) |
| 트리 밖 판정 F1 | 0.745 |
| 오보류율 (답할 수 있는데 보류) | 1.5% [0.6–3.4] (n=337) |
| 오답변율 (답할 수 없는데 답함) | 3.9% [1.1–13.2] (n=51) |
| Hit@1 | 95.3% [92.4–97.1] (n=337) |
| Hit@3 | 95.8% [93.1–97.5] (n=337) |
| Hit@k (limit) | 95.8% [93.1–97.5] (n=337) |
| MRR@k | 0.955 [0.934–0.977] (n=337) |
| nDCG@k | 0.942 [0.920–0.964] (n=337) |
| Recall@k | 0.944 [0.921–0.967] (n=337) |
| recommended 정밀도 | 0.975 [0.958–0.992] (n=306) |
| 라우팅이 맞았을 때 Hit@1 | 99.4% [97.8–99.8] (n=323) |
| 라우팅이 맞았을 때 MRR | 0.997 [0.993–1.000] (n=323) |
| 지연 p50 / p90 / p95 / 평균 | 0.8s / 1.3s / 1.4s / 0.8s |
| 라우팅 구간 평균 | 0.5s |
| 랭킹 후보 수 평균 | 16.4 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 3.15 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 |
|---|---:|---:|
| 일치율 | 97.3% (n=336) | 95.4% (n=326) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_tree |
|---:|---:|---:|---:|
| 5 | 9 | 416 | 17 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.6s |
| internal | 10 | 90.0% | 100.0% | 90.0% | 0.950 | 1.3s |
| leaf | 327 | 95.4% | 95.7% | 95.4% | 0.956 | 0.8s |
| no_answer | 16 | 100.0% | 100.0% | - | - | 0.8s |
| oos | 32 | 93.8% | 59.4% | - | - | 0.3s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 94.3% | 93.3% | 94.2% | 0.942 | 0.8s |
| single | 283 | 95.8% | 92.9% | 95.7% | 0.962 | 0.8s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 283 | 95.8% | 92.9% | 95.7% | 0.962 | 0.8s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 0.8s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 0.8s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 0.8s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 0.8s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 357 | 95.2% | 93.0% | 95.0% | 0.953 | 0.8s |
| root | 31 | 96.8% | 93.5% | 100.0% | 1.000 | 0.3s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 10 | 90.0% | 100.0% | 90.0% | 0.950 | 1.3s |
| leaf | 346 | 95.7% | 96.0% | 95.4% | 0.956 | 0.8s |
| none | 32 | 93.8% | 59.4% | - | - | 0.3s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 32 | 93.8% | 59.4% | - | - | 0.3s |
| d1 | 10 | 90.0% | 100.0% | 90.0% | 0.950 | 1.3s |
| d2 | 346 | 95.7% | 96.0% | 95.4% | 0.956 | 0.8s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 35 | 94.3% | 62.9% | - | - | 0.3s |
| 101+ | 4 | 75.0% | 100.0% | 75.0% | 0.875 | 2.5s |
| 2-5 | 293 | 95.9% | 96.2% | 95.7% | 0.959 | 0.8s |
| 33-100 | 56 | 94.6% | 94.6% | 94.5% | 0.945 | 1.3s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| colloquial | 66 | 98.5% | 100.0% | 98.4% | 0.992 | 0.8s |
| condition | 32 | 93.8% | 93.8% | 93.5% | 0.935 | 0.8s |
| english | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| keyword | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 0.8s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| mixed | 13 | 92.3% | 92.3% | 92.3% | 0.923 | 0.8s |
| multi_intent | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| negation | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 0.8s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.2s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 1.1s |
| paraphrase | 149 | 94.0% | 93.3% | 93.5% | 0.935 | 0.8s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| vague | 10 | 90.0% | 100.0% | 90.0% | 0.950 | 1.3s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 39 | 92.3% | 94.9% | 92.1% | 0.934 | 0.8s |
| health | 35 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| it | 92 | 96.7% | 96.7% | 96.6% | 0.966 | 0.8s |
| life | 35 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| money | 43 | 95.3% | 95.3% | 95.1% | 0.951 | 0.7s |
| oos | 32 | 93.8% | 59.4% | - | - | 0.3s |
| shop | 35 | 88.6% | 88.6% | 87.9% | 0.879 | 0.8s |
| travel | 38 | 97.4% | 97.4% | 97.1% | 0.971 | 0.8s |
| work | 39 | 92.3% | 94.9% | 91.9% | 0.932 | 0.7s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 109 | 97.2% | 98.2% | 97.0% | 0.975 | 0.8s |
| hard | 100 | 92.0% | 84.0% | 92.1% | 0.921 | 0.8s |
| medium | 179 | 96.1% | 95.0% | 95.6% | 0.959 | 0.8s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 260 | 95.4% | 92.7% | 95.6% | 0.959 | 0.8s |
| test | 128 | 95.3% | 93.8% | 94.4% | 0.949 | 0.8s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 95.1% | 98.5% | 95.8% | 5.9% |
| 0.25 | 95.1% | 98.5% | 95.8% | 5.9% |
| 0.30 | 95.4% | 98.5% | 96.1% | 3.9% |
| 0.35 | 95.4% | 97.6% | 97.0% | 2.0% |
| 0.40 | 95.4% | 97.3% | 97.3% | 2.0% |
| 0.50 | 94.3% | 96.1% | 97.2% | 2.0% |
| 0.60 | 91.2% | 92.6% | 97.1% | 2.0% |
| 0.65 | 89.9% | 90.5% | 97.7% | 2.0% |

### 저장 (ingest, 59건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 93.2% [83.8–97.3] (n=59) |
|   └ 올바른 카테고리에 저장 | 100.0% [92.7–100.0] (n=49) |
|   └ 트리/루트 밖 Q&A 거절 | 60.0% [31.3–83.2] (n=10) |
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
| 지연 p50 (초안+게시+probe) | 1.5s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 2 |
| null | it | 2 |
| null | life | 2 |
| it_office_excel_func | work | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| it_sec_phish | money_bank_xfer_wrong | 1 |
| money_bank_cert | it_sec_2fa | 1 |
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | food_safe_poison | 1 |
| null | health | 1 |
| null | health_sym_stomach | 1 |
| null | money | 1 |
| null | money_card_lost | 1 |
| null | money_save_etf | 1 |
| null | shop | 1 |
| null | travel | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_order_receipt | money | 1 |
| shop_order_receipt | money_tax_income | 1 |

### 틀린 케이스 (33건 중 33건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-SHOP-007 | 호텔 예약을 취소하면 수수료가 얼마나 나오나요? | null | shop_order_cancel | false_tree |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_order_cancel | false_tree |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food · top food_safe_poison#2 0.61 alternative | wrong top1 |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.81 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.88 recommended | route branch, wrong top1 |
| S-IT-031 | 엄마라면서 폰 액정이 깨졌다고 상품권 좀 사 달라는 문자가 왔는데 이상해요 | it_sec_phish | money_bank_xfer_wrong · abstained · top money_bank_xfer_wrong#2 0.06 reference | route domain, abstained |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · top work_exp_claim#3 0.32 reference | route domain, wrong top1 |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec_2fa · abstained · top it_sec_2fa#2 0.04 reference | route domain, abstained |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.86 recommended | route domain, wrong top1 |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · top it_mobile_lost#2 0.32 reference | route false_tree, answered unanswerable |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money_save_etf · abstained · top money_save_etf#2 0.10 reference | route false_tree |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health#article 0.08 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | life · abstained · top life_home_drain#2 0.13 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel · abstained · top travel_local_metro#2 0.15 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money · abstained · top money_tax_yearend#4 0.19 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life#article 0.10 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it · abstained · top it_mobile_lost#2 0.26 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym_stomach · abstained · top health_sym_stomach#3 0.15 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.16 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_addr#2 0.17 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.37 reference | route domain, wrong top1 |
| S-SHOP-009 | 카드로 산 운동화를 반품했는데 카드사 앱 승인 내역에 아직 취소가 안 보여요. 보통 며칠 걸려요? | shop_ret_refund_card | money · abstained · top money_ins#2 0.14 reference | route domain, abstained |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money · abstained · top money_tax_yearend#4 0.13 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.66 recommended | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.73 recommended | wrong top1 |
| S-WORK-008 | 연말정산 공제 얘기 말고요, 매달 월급에서 떼가는 건강보험이랑 소득세 항목이 뭔지 알고 싶어요 | work_pay_slip | money_tax_income · abstained · top money_tax_income#2 0.08 reference | route domain, abstained |
| S-XD-014 | 여행용 eSIM을 샀는데 폰에 어떻게 설치해서 써요? | travel_prep_data | it · top it#article 0.32 reference | route domain, wrong top1 |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.60 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.83 recommended | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.83 recommended | route false_tree, answered unanswerable |
| S-XD-052 | 상한 음식을 먹고 계속 토해요. 어떻게 해야 돼요? | null · no answer | food_safe_poison · abstained · top food_safe_poison#2 0.14 reference | route false_tree |

## jev · flat

### 검색 (357건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 97.5% [95.3–98.7] (n=357) |
|   └ 답할 수 있는 질문 | 97.2% [94.7–98.5] (n=317) |
|   └ 답할 수 없는 질문(보류가 정답) | 100.0% [91.2–100.0] (n=40) |
| 확신 정답률 (top1 정답 + recommended) | 90.9% [87.2–93.6] (n=317) |
| 라우팅 정확도 (정확 일치) | 97.5% [95.3–98.7] (n=357) |
| 계층 F1 (hF1) | 0.975 [0.959–0.991] (n=357) |
| 트리 밖 판정 precision | 100.0% [84.5–100.0] (n=21) |
| 트리 밖 판정 recall | 100.0% [84.5–100.0] (n=21) |
| 트리 밖 판정 F1 | 1.000 |
| 오보류율 (답할 수 있는데 보류) | 0.0% [0.0–1.2] (n=317) |
| 오답변율 (답할 수 없는데 답함) | 0.0% [0.0–8.8] (n=40) |
| Hit@1 | 97.2% [94.7–98.5] (n=317) |
| Hit@3 | 97.5% [95.1–98.7] (n=317) |
| Hit@k (limit) | 97.5% [95.1–98.7] (n=317) |
| MRR@k | 0.973 [0.956–0.991] (n=317) |
| nDCG@k | 0.959 [0.940–0.977] (n=317) |
| Recall@k | 0.959 [0.939–0.979] (n=317) |
| recommended 정밀도 | 0.983 [0.969–0.998] (n=293) |
| 라우팅이 맞았을 때 Hit@1 | 99.7% [98.2–99.9] (n=309) |
| 라우팅이 맞았을 때 MRR | 0.998 [0.995–1.000] (n=309) |
| 지연 p50 / p90 / p95 / 평균 | 0.6s / 1.1s / 1.2s / 0.6s |
| 라우팅 구간 평균 | 0.3s |
| 랭킹 후보 수 평균 | 12.1 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 2.13 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 |
|---|---:|
| 일치율 | 96.6% (n=326) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact |
|---:|---:|---:|
| 4 | 5 | 398 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.3s |
| leaf | 317 | 97.2% | 97.5% | 97.2% | 0.973 | 0.6s |
| no_answer | 16 | 100.0% | 93.8% | - | - | 0.6s |
| oos | 21 | 100.0% | 100.0% | - | - | 0.3s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 94.3% | 94.3% | 94.2% | 0.942 | 0.6s |
| single | 252 | 98.8% | 98.8% | 98.6% | 0.988 | 0.6s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 252 | 98.8% | 98.8% | 98.6% | 0.988 | 0.6s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 0.6s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| disambig | 20 | 85.0% | 85.0% | 85.0% | 0.850 | 0.6s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| shift | 14 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 347 | 97.4% | 97.4% | 97.1% | 0.972 | 0.6s |
| root | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.3s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| leaf | 336 | 97.3% | 97.3% | 97.2% | 0.973 | 0.6s |
| none | 21 | 100.0% | 100.0% | - | - | 0.3s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 21 | 100.0% | 100.0% | - | - | 0.3s |
| d1 | 336 | 97.3% | 97.3% | 97.2% | 0.973 | 0.6s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 24 | 100.0% | 100.0% | - | - | 0.3s |
| 2-5 | 283 | 97.2% | 97.2% | 97.0% | 0.972 | 0.6s |
| 33-100 | 50 | 98.0% | 98.0% | 98.0% | 0.980 | 1.1s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| colloquial | 66 | 98.5% | 100.0% | 98.4% | 0.992 | 0.6s |
| condition | 31 | 96.8% | 96.8% | 96.8% | 0.968 | 0.6s |
| english | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.7s |
| keyword | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| multi_intent | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| negation | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 0.6s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.3s |
| oos_near | 11 | 100.0% | 100.0% | - | - | 0.3s |
| paraphrase | 129 | 95.3% | 94.6% | 94.7% | 0.947 | 0.6s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 36 | 97.2% | 97.2% | 97.1% | 0.971 | 0.6s |
| health | 32 | 100.0% | 100.0% | 100.0% | 1.000 | 0.7s |
| it | 89 | 98.9% | 98.9% | 98.8% | 0.988 | 0.6s |
| life | 33 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| money | 41 | 95.1% | 95.1% | 94.9% | 0.949 | 0.5s |
| oos | 21 | 100.0% | 100.0% | - | - | 0.3s |
| shop | 32 | 93.8% | 93.8% | 93.3% | 0.933 | 0.6s |
| travel | 36 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |
| work | 37 | 91.9% | 91.9% | 91.4% | 0.929 | 0.5s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 99 | 99.0% | 100.0% | 98.9% | 0.995 | 0.6s |
| hard | 95 | 91.6% | 90.5% | 89.5% | 0.895 | 0.6s |
| medium | 163 | 100.0% | 100.0% | 100.0% | 1.000 | 0.6s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 237 | 97.9% | 97.5% | 97.7% | 0.977 | 0.6s |
| test | 120 | 96.7% | 97.5% | 96.1% | 0.966 | 0.6s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 97.5% | 100.0% | 97.2% | 0.0% |
| 0.25 | 97.5% | 100.0% | 97.2% | 0.0% |
| 0.30 | 97.5% | 100.0% | 97.2% | 0.0% |
| 0.35 | 97.2% | 99.7% | 97.2% | 0.0% |
| 0.40 | 97.2% | 99.4% | 97.5% | 0.0% |
| 0.50 | 96.4% | 98.1% | 97.7% | 0.0% |
| 0.60 | 93.0% | 93.7% | 98.3% | 0.0% |
| 0.65 | 91.9% | 92.4% | 98.3% | 0.0% |

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
| 지연 p50 (초안+게시+probe) | 1.2s |
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
| shop_order_pay | shop_order_cancel | 1 |
| shop_ret_refund_card | money_card_install | 1 |
| work_exp_card | money_card_limit | 1 |
| work_exp_trip | travel_air_ticket_mile | 1 |
| work_exp_trip | work_exp_claim | 1 |

### 틀린 케이스 (10건 중 10건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.87 recommended | route branch, wrong top1 |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost | it_sec_hacked · top it_sec_hacked#1 0.36 reference | route domain, wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.86 recommended | route domain, wrong top1 |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card | money_card_install · top money_card_install#3 0.41 alternative | route domain, wrong top1 |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.55 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.74 recommended | wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#2 0.08 reference | route domain |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.59 alternative | route branch, wrong top1 |
| S-XD-024 | 한도 좀 올리고 싶은데 어떻게 해요? | work_exp_card | money_card_limit · top money_card_limit#1 0.80 recommended | route domain, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.79 recommended | route branch, wrong top1 |

## jev · sparse

### 검색 (432건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 83.6% [79.8–86.8] (n=432) |
|   └ 답할 수 있는 질문 | 87.9% [82.3–92.0] (n=174) |
|   └ 답할 수 없는 질문(보류가 정답) | 80.6% [75.4–85.0] (n=258) |
| 확신 정답률 (top1 정답 + recommended) | 77.6% [70.8–83.1] (n=174) |
| 라우팅 정확도 (정확 일치) | 89.6% [86.3–92.1] (n=432) |
| 계층 F1 (hF1) | 0.926 [0.903–0.948] (n=432) |
| 트리 밖 판정 precision | 100.0% [84.5–100.0] (n=21) |
| 트리 밖 판정 recall | 61.8% [45.0–76.1] (n=34) |
| 트리 밖 판정 F1 | 0.764 |
| 오보류율 (답할 수 있는데 보류) | 6.9% [4.0–11.7] (n=174) |
| 오답변율 (답할 수 없는데 답함) | 19.4% [15.0–24.6] (n=258) |
| Hit@1 | 89.1% [83.6–92.9] (n=174) |
| Hit@3 | 89.7% [84.2–93.4] (n=174) |
| Hit@k (limit) | 90.2% [84.9–93.8] (n=174) |
| MRR@k | 0.895 [0.850–0.940] (n=174) |
| nDCG@k | 0.899 [0.855–0.943] (n=174) |
| Recall@k | 0.891 [0.846–0.936] (n=174) |
| recommended 정밀도 | 0.895 [0.847–0.943] (n=149) |
| 라우팅이 맞았을 때 Hit@1 | 96.2% [92.1–98.3] (n=160) |
| 라우팅이 맞았을 때 MRR | 0.967 [0.941–0.994] (n=160) |
| 지연 p50 / p90 / p95 / 평균 | 1.0s / 1.3s / 1.4s / 1.0s |
| 라우팅 구간 평균 | 0.7s |
| 랭킹 후보 수 평균 | 2.4 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 3.82 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 97.4% (n=351) | 94.4% (n=341) | 92.2% (n=322) | 91.4% (n=81) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 16 | 7 | 9 | 436 | 17 | 3 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dropped | 194 | 75.3% | 91.8% | - | - | 1.0s |
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.7s |
| internal | 23 | 65.2% | 73.9% | 65.2% | 0.685 | 1.0s |
| leaf | 151 | 91.4% | 94.7% | 92.7% | 0.927 | 1.0s |
| no_answer | 27 | 100.0% | 92.6% | - | - | 1.0s |
| oos | 34 | 94.1% | 61.8% | - | - | 0.3s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 81.9% | 91.4% | 92.9% | 0.929 | 1.0s |
| single | 327 | 84.1% | 89.0% | 87.9% | 0.884 | 1.0s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 327 | 84.1% | 89.0% | 87.9% | 0.884 | 1.0s |
| coref | 19 | 78.9% | 78.9% | 100.0% | 1.000 | 1.0s |
| correct | 8 | 75.0% | 87.5% | 80.0% | 0.800 | 1.0s |
| disambig | 20 | 80.0% | 90.0% | 90.0% | 0.900 | 1.1s |
| distract | 9 | 88.9% | 100.0% | 100.0% | 1.000 | 1.0s |
| ellipsis | 9 | 77.8% | 88.9% | 75.0% | 0.750 | 1.0s |
| long | 8 | 75.0% | 100.0% | 100.0% | 1.000 | 1.0s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.1s |
| shift | 14 | 92.9% | 92.9% | 100.0% | 1.000 | 1.1s |
| slot | 9 | 66.7% | 100.0% | 100.0% | 1.000 | 1.0s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 372 | 83.1% | 89.0% | 88.5% | 0.889 | 1.0s |
| root | 43 | 88.4% | 90.7% | 100.0% | 1.000 | 0.5s |
| start | 17 | 82.4% | 100.0% | 100.0% | 1.000 | 0.5s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 76.5% | 79.4% | 65.2% | 0.685 | 1.0s |
| leaf | 364 | 83.2% | 93.1% | 92.7% | 0.927 | 1.0s |
| none | 34 | 94.1% | 61.8% | - | - | 0.3s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 34 | 94.1% | 61.8% | - | - | 0.3s |
| d1 | 10 | 60.0% | 60.0% | 60.0% | 0.600 | 1.0s |
| d2 | 30 | 83.3% | 90.0% | 73.3% | 0.783 | 0.9s |
| d3 | 271 | 83.0% | 93.4% | 93.9% | 0.939 | 1.0s |
| d4 | 87 | 83.9% | 92.0% | 88.6% | 0.886 | 1.3s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 37 | 94.6% | 64.9% | - | - | 0.3s |
| 1 | 361 | 83.1% | 93.1% | 92.7% | 0.927 | 1.0s |
| 2-5 | 16 | 87.5% | 87.5% | 71.4% | 0.786 | 0.9s |
| 33-100 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| 6-32 | 16 | 62.5% | 68.8% | 57.1% | 0.589 | 1.0s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 87.5% | 100.0% | - | - | 1.0s |
| colloquial | 69 | 81.2% | 97.1% | 96.7% | 0.967 | 1.0s |
| condition | 32 | 78.1% | 87.5% | 92.9% | 0.929 | 1.1s |
| english | 12 | 91.7% | 100.0% | 100.0% | 1.000 | 1.0s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 1.0s |
| keyword | 20 | 80.0% | 80.0% | 90.0% | 0.900 | 1.0s |
| long | 12 | 91.7% | 100.0% | 100.0% | 1.000 | 1.0s |
| mixed | 13 | 84.6% | 92.3% | 71.4% | 0.714 | 1.1s |
| multi_intent | 9 | 55.6% | 88.9% | 66.7% | 0.667 | 1.0s |
| negation | 20 | 75.0% | 90.0% | 87.5% | 0.875 | 1.0s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.3s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 1.0s |
| paraphrase | 177 | 87.0% | 92.1% | 96.1% | 0.961 | 1.0s |
| typo | 13 | 92.3% | 100.0% | 100.0% | 1.000 | 1.0s |
| vague | 23 | 65.2% | 73.9% | 65.2% | 0.685 | 1.0s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 45 | 77.8% | 88.9% | 80.0% | 0.833 | 1.0s |
| health | 39 | 87.2% | 100.0% | 93.8% | 0.953 | 1.0s |
| it | 100 | 88.0% | 95.0% | 93.5% | 0.935 | 1.2s |
| life | 40 | 75.0% | 87.5% | 86.4% | 0.864 | 1.2s |
| money | 48 | 75.0% | 95.8% | 95.8% | 0.958 | 1.0s |
| oos | 34 | 94.1% | 61.8% | - | - | 0.3s |
| shop | 40 | 72.5% | 77.5% | 61.1% | 0.611 | 1.0s |
| travel | 43 | 90.7% | 95.3% | 93.3% | 0.933 | 1.0s |
| work | 43 | 88.4% | 90.7% | 100.0% | 1.000 | 1.0s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 91.5% | 95.8% | 98.0% | 0.980 | 1.0s |
| hard | 112 | 79.5% | 82.1% | 90.6% | 0.906 | 1.0s |
| medium | 202 | 81.2% | 90.1% | 83.5% | 0.843 | 1.0s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 292 | 82.5% | 88.7% | 89.4% | 0.900 | 1.0s |
| test | 140 | 85.7% | 91.4% | 88.2% | 0.882 | 1.0s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 81.0% | 94.8% | 68.0% | 24.4% |
| 0.25 | 82.4% | 94.8% | 69.8% | 22.1% |
| 0.30 | 83.6% | 93.1% | 72.2% | 19.4% |
| 0.35 | 86.8% | 91.4% | 78.2% | 13.2% |
| 0.40 | 88.0% | 89.7% | 81.4% | 10.5% |
| 0.50 | 89.4% | 87.4% | 86.0% | 7.4% |
| 0.60 | 88.9% | 82.2% | 89.1% | 5.0% |
| 0.65 | 88.4% | 79.3% | 90.6% | 4.3% |

### 저장 (ingest, 56건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 87.5% [76.4–93.8] (n=56) |
|   └ 올바른 카테고리에 저장 | 93.2% [81.8–97.7] (n=44) |
|   └ 트리/루트 밖 Q&A 거절 | 66.7% [39.1–86.2] (n=12) |
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
| 지연 p50 (초안+게시+probe) | 1.9s |
| LLM 실패 → Jev 빔 대체율 | - |

### API 계약 (11건)

통과 100.0% [74.1–100.0] (n=11)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food | food_cook | 2 |
| food_store_item | food_store_freeze | 2 |
| null | it | 2 |
| null | money | 2 |
| work_leave_event | work_pay | 2 |
| food_tool | food_tool_pan | 1 |
| it_office_excel_func | work | 1 |
| it_pc_peri_monitor | it_pc_win | 1 |
| it_pc_win_keys | it | 1 |
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

### 틀린 케이스 (101건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-IT-003 | How do I turn on Bluetooth on my Windows laptop when the toggle is missing? | it_pc_peri_bt | it_pc_win | branch |
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-SHOP-007 | 호텔 예약을 취소하면 수수료가 얼마나 나오나요? | null | shop_order_cancel | false_tree |
| I-WORK-001 | 사원증을 잃어버렸는데 재발급은 어떻게 받나요? | work_hr_onboard | work_hr | shallow |
| I-XD-002 | 체크카드로 결제한 주문을 취소하면 환불은 며칠 걸리나요? | shop_ret_refund_card | shop_order_cancel | branch |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_ret | false_tree |
| S-FOOD-001 | 밥솥으로 밥을 했는데 밥알 가운데가 생쌀처럼 씹혀요 | food_cook_base_rice · no answer | food_cook_base_rice · top food_cook_base_rice#1 0.34 reference | answered unanswerable |
| S-FOOD-007 | 배탈 난 뒤 관리 말고요, 여름에 식중독 안 걸리게 미리 조심할 것들 알려주세요 | food_safe_poison · no answer | food_safe_poison · top food_safe_poison#1 0.47 alternative | answered unanswerable |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food_cook · top food_cook#article 0.58 alternative | route deep, wrong top1 |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_knife#1 0.40 alternative | wrong top1 |
| S-FOOD-012 | 스테인리스 냄비 바닥에 까맣게 탄 자국은 어떻게 벗겨요? | food_tool · no answer | food_tool_pan · abstained · top food_tool_pan#1 0.02 reference | route deep |
| S-FOOD-014 | 그거 깔끔하게 까는 요령 좀요 | food_cook_side_egg · no answer | food_cook_side_egg · top food_cook_side_egg#1 0.31 reference | answered unanswerable |
| S-FOOD-018 | 아 근데 냉장고에 있는 요거트가 날짜가 이틀 지났는데 먹어도 될까요? | food_safe_date · no answer | food_safe_date · top food_safe_date#1 0.30 reference | answered unanswerable |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item · no answer | food_store_freeze · top food_store_freeze#1 0.83 recommended | route branch, answered unanswerable |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item · no answer | food_store_freeze · top food_store_freeze#1 0.89 recommended | route branch, answered unanswerable |
| S-HEALTH-002 | 진통제 거의 매일 먹는데 이거 괜찮은거임? 안 먹으면 머리 깨질 것 같음 | health_sym_head · no answer | health_sym_head · top health_sym_head#1 0.30 reference | answered unanswerable |
| S-HEALTH-007 | My 8-month-old baby is choking on something and can't breathe. What should I do  | health_aid_emerg_choke · no answer | health_aid_emerg_choke · top health_aid_emerg_choke#1 0.38 reference | answered unanswerable |
| S-HEALTH-009 | 달리기 말고 헬스장에서 근력이랑 유산소를 섞어서 하려는데 뭐부터 해야 돼요? | health_fit_start · no answer | health_fit_start · top health_fit_start#1 0.37 reference | answered unanswerable |
| S-HEALTH-017 | 집에서 다쳤을 때 쓸 응급처치를 종류별로 한번 정리해 두고 싶어요 | health_aid | health_aid · top health_aid_injury_cut#1 0.65 recommended | wrong top1 |
| S-HEALTH-025 | 그게 원인일 수도 있어요? 요즘 새벽까지 눈이 말똥말똥해서요 | health_sleep · no answer | health_sleep · top health_sleep#1 0.31 reference | answered unanswerable |
| S-IT-028 | 사이트마다 비밀번호를 다르게 쓰면 다 기억할 수가 없어요. | it_sec_pw · no answer | it_sec_pw · top it_sec_pw#1 0.42 alternative | answered unanswerable |
| S-IT-031 | 엄마라면서 폰 액정이 깨졌다고 상품권 좀 사 달라는 문자가 왔는데 이상해요 | it_sec_phish · no answer | money · abstained · top money_bank_xfer_wrong#1 0.14 reference | route domain |
| S-IT-035 | 듀얼 모니터 쓰는데 작업 표시줄을 한쪽 화면에만 나오게 할 수 있나요? | it_pc_peri_monitor · no answer | it_pc_win · abstained · top it_pc_win#article 0.09 reference | route branch |
| S-IT-041 | 같은 브랜드로 가요. 옮기기 전에 백업은 어떻게 해 둬요? | it_mobile_move_android · no answer | it_mobile_move_android · top it_mobile_move_android#1 0.67 recommended | answered unanswerable |
| S-IT-043 | 아 잘못 말했어요, 새로 산 게 아이폰이 아니라 갤럭시예요. 그럼 어떻게 옮겨요? | it_mobile_move_cross · no answer | it_mobile_move_cross · top it_mobile_move_cross#1 0.30 reference | answered unanswerable |
| S-IT-051 | 누가 제 이메일 계정 비번이랑 복구 메일까지 바꿔 놔서 들어갈 수가 없어요 | it_sec_hacked · no answer | it_sec_hacked · top it_sec_hacked#1 0.33 reference | answered unanswerable |
| S-IT-502 | 캡처하면 바로 파일로 저장되는 키 뭐였지ㅋㅋ 맨날 그림판에 붙여넣기 귀찮음 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.31 reference | answered unanswerable |
| S-IT-503 | 전체 화면 말고 지금 띄워 놓은 창 하나만 찍고 싶어요 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.77 recommended | answered unanswerable |
| S-IT-504 | 클립보드 기록 | it_pc_win_keys · no answer | it · top it_net_router_reset#1 0.79 recommended | route shallow, answered unanswerable |
| S-IT-515 | 빔프로젝터 연결했는데 노트북 화면이랑 똑같이 duplicate 되게 바꾸는 키? | it_pc_win_keys (+1) | it_pc_win_keys · abstained · top it_pc_win_keys#win_shift_s 0.02 reference | abstained |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func · no answer | work · abstained · top work_pay_ot#1 0.18 reference | route domain |
| S-LIFE-001 | 드럼세탁기 코스가 다 끝났는데 옷이 물에 푹 젖은 채로 나와요. 탈수가 안 된 것 같아요 | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.78 recommended | answered unanswerable |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell · no answer | life_clean_towel · top life_clean_towel#1 0.53 alternative | route branch, answered unanswerable |
| S-LIFE-004 | 드럼 고무패킹 곰팡이 | life_appl_wash_wm_drum_care_gasket | life_clean_mold · abstained · top life_clean_mold#1 0.17 reference | route branch, abstained |
| S-LIFE-006 | 통돌이 세탁기로 빨래하면 검은 찌꺼기가 뭍어나와여 청소 어케해요 | life_appl_wash_wm_top_clean · no answer | life_appl_wash_wm_top_clean · top life_appl_wash_wm_top_clean#1 0.80 recommended | answered unanswerable |
| S-LIFE-008 | 건조기 돌리면 안 되고 그냥 널어서 말려야 하는 옷은 어떤 거예요? | life_appl_wash_dryer_shrink | life_clean · abstained · top life_clean_towel#1 0.06 reference | route branch, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.52 alternative | route shallow |
| S-LIFE-020 | 자취 시작하고 집안일이 처음이라 뭐가 고장 나거나 지저분해지면 어디서부터 봐야 할지 모르겠어요 | life | life_home · top life_home#article 0.85 recommended | route deep, wrong top1 |
| S-LIFE-030 | 빨래를 조금만 넣었을 때 탈수가 잘 안 되는데 왜 그래요? | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.79 recommended | answered unanswerable |
| S-LIFE-035 | 그럼 아까 그거 빨고 나서 방에서 말릴 때 냄새 안 나게 하려면 어떻게 해요? | life_clean_towel · no answer | life_clean_towel · top life_clean_towel#1 0.52 alternative | answered unanswerable |
| S-LIFE-036 | 겨울만 되면 거실 창틀이 곰팡이로 까매져요 | life_clean_mold · no answer | life_clean_mold · top life_clean_mold#1 0.42 alternative | answered unanswerable |
| S-LIFE-038 | 세면대 물이 고인 채로 한참 걸려서 빠져요 | life_home_drain · no answer | life_home_drain · top life_home_drain#1 0.30 reference | answered unanswerable |
| S-MONEY-002 | 통장 만든 지 일주일 됐는데 하루에 30만 원까지만 보내져요. 앱에서 이체 한도 올리기를 눌러 봐도 안 되는데 이거 계좌 자체가 묶인 건가요? | money_bank_acct_limit · no answer | money_bank_acct_limit · top money_bank_acct_limit#1 0.36 reference | answered unanswerable |
| S-MONEY-005 | 인터넷뱅킹으로 계좌이체할 때 하루 한도를 늘리려면 OTP가 꼭 있어야 하나요? | money_bank_xfer_cap · no answer | money_bank_xfer_cap · top money_bank_xfer_cap#1 0.68 recommended | answered unanswerable |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec#article 0.06 reference | route domain, abstained |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost · no answer | money_card_lost · top money_card_lost#1 0.48 alternative | answered unanswerable |
| S-MONEY-012 | 온라인 쇼핑몰에서 결제하는데 한도 초과라고 승인이 안 나요. 쇼핑몰 오류가 아니라 제 신용카드 한도가 찬 거라는데 지금 바로 결제하려면 어떻게  | money_card_limit · no answer | money_card_limit · top money_card_limit#1 0.38 reference | answered unanswerable |
| S-MONEY-017 | 직장인인데 주말마다 과외해서 번 돈이 좀 있어요. 이것도 회사 연말정산으로 끝나나요, 아니면 제가 따로 신고해야 하나요? | money_tax_income · no answer | money_tax_income · top money_tax_income#1 0.43 alternative | answered unanswerable |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend · no answer | work_exp_claim · abstained · top work_exp_claim#1 0.14 reference | route domain |
| S-MONEY-027 | 마통은요? | money_loan_credit · no answer | money_loan_credit · top money_loan_credit#1 0.56 alternative | answered unanswerable |
| S-MONEY-033 | 한도를 당장 다시 살리는 방법은 없어요? | money_card_limit · no answer | money_card_limit · top money_card_limit#1 0.40 reference | answered unanswerable |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit · no answer | money_bank_acct_limit · top money_bank_acct_limit#1 0.60 alternative | answered unanswerable |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card#article 0.05 reference | route false_tree |
| S-MONEY-039 | 휴면 예금이 서민금융진흥원으로 넘어갔다는데 그럼 이제 못 돌려받는 거예요? | money_bank_acct_dormant · no answer | money_bank_acct_dormant · top money_bank_acct_dormant#1 0.31 reference | answered unanswerable |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc#article 0.10 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · top money_save#article 0.59 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health#article 0.08 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_loan_score#1 0.11 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.07 reference | route false_tree |

## jev_llm · deep

### 검색 (433건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 94.9% [92.4–96.6] (n=433) |
|   └ 답할 수 있는 질문 | 94.3% [91.5–96.2] (n=369) |
|   └ 답할 수 없는 질문(보류가 정답) | 98.4% [91.7–99.7] (n=64) |
| 확신 정답률 (top1 정답 + recommended) | 88.9% [85.3–91.7] (n=369) |
| 라우팅 정확도 (정확 일치) | 88.0% [84.6–90.7] (n=433) |
| 계층 F1 (hF1) | 0.924 [0.902–0.947] (n=433) |
| 트리 밖 판정 precision | 91.3% [73.2–97.6] (n=23) |
| 트리 밖 판정 recall | 61.8% [45.0–76.1] (n=34) |
| 트리 밖 판정 F1 | 0.737 |
| 오보류율 (답할 수 있는데 보류) | 2.7% [1.5–4.9] (n=369) |
| 오답변율 (답할 수 없는데 답함) | 1.6% [0.3–8.3] (n=64) |
| Hit@1 | 94.3% [91.5–96.2] (n=369) |
| Hit@3 | 95.7% [93.1–97.3] (n=369) |
| Hit@k (limit) | 95.7% [93.1–97.3] (n=369) |
| MRR@k | 0.949 [0.927–0.971] (n=369) |
| nDCG@k | 0.939 [0.918–0.961] (n=369) |
| Recall@k | 0.945 [0.923–0.967] (n=369) |
| recommended 정밀도 | 0.972 [0.956–0.987] (n=333) |
| 라우팅이 맞았을 때 Hit@1 | 98.8% [96.9–99.5] (n=330) |
| 라우팅이 맞았을 때 MRR | 0.993 [0.986–1.000] (n=330) |
| 지연 p50 / p90 / p95 / 평균 | 11.8s / 42.4s / 46.7s / 17.8s |
| 라우팅 구간 평균 | 17.4s |
| 랭킹 후보 수 평균 | 17.6 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.38 |
| LLM 라우팅 호출 수 평균 | 1.84 |
| LLM 실패 → Jev 빔 대체율 | 2.3% [1.3–4.2] (n=433) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 | d5 | d6 | d7 |
|---|---:|---:|---:|---:|---:|---:|---:|
| 일치율 | 97.4% (n=352) | 93.6% (n=342) | 89.5% (n=323) | 85.6% (n=146) | 100.0% (n=20) | 100.0% (n=10) | 100.0% (n=8) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 7 | 7 | 444 | 2 | 18 | 23 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 15.5s |
| internal | 24 | 87.5% | 100.0% | 87.5% | 0.924 | 14.8s |
| leaf | 345 | 94.8% | 88.7% | 94.8% | 0.951 | 12.0s |
| no_answer | 27 | 100.0% | 100.0% | - | - | 17.6s |
| oos | 34 | 97.1% | 61.8% | - | - | 6.2s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 95.2% | 88.6% | 95.1% | 0.951 | 13.2s |
| single | 328 | 94.8% | 87.8% | 94.0% | 0.948 | 11.3s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 328 | 94.8% | 87.8% | 94.0% | 0.948 | 11.3s |
| coref | 19 | 94.7% | 89.5% | 94.7% | 0.947 | 16.9s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 23.8s |
| disambig | 20 | 90.0% | 85.0% | 90.0% | 0.900 | 9.8s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 18.6s |
| ellipsis | 9 | 100.0% | 77.8% | 100.0% | 1.000 | 15.4s |
| long | 8 | 100.0% | 87.5% | 100.0% | 1.000 | 29.5s |
| refine | 9 | 88.9% | 77.8% | 88.9% | 0.889 | 12.2s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 8.8s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 40.7s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 373 | 94.4% | 86.6% | 94.0% | 0.946 | 12.5s |
| root | 43 | 97.7% | 95.3% | 96.4% | 0.964 | 3.8s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 5.0s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 35 | 91.4% | 100.0% | 87.5% | 0.924 | 12.8s |
| leaf | 364 | 95.1% | 89.3% | 94.8% | 0.951 | 12.1s |
| none | 34 | 97.1% | 61.8% | - | - | 6.2s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 34 | 97.1% | 61.8% | - | - | 6.2s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 33.6s |
| d2 | 30 | 90.0% | 100.0% | 84.2% | 0.904 | 10.9s |
| d3 | 199 | 94.5% | 90.5% | 94.0% | 0.940 | 13.7s |
| d4 | 140 | 95.0% | 85.7% | 94.9% | 0.956 | 11.2s |
| d5 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 16.8s |
| d6 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 46.7s |
| d7 | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 23.8s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 37 | 97.3% | 64.9% | - | - | 7.2s |
| 101+ | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 42.7s |
| 2-5 | 311 | 96.1% | 92.9% | 95.9% | 0.964 | 13.0s |
| 33-100 | 61 | 88.5% | 72.1% | 88.1% | 0.881 | 8.2s |
| 6-32 | 20 | 90.0% | 100.0% | 81.8% | 0.894 | 14.8s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 21.1s |
| colloquial | 69 | 97.1% | 95.7% | 96.9% | 0.977 | 12.2s |
| condition | 32 | 93.8% | 87.5% | 93.5% | 0.935 | 12.5s |
| english | 12 | 100.0% | 58.3% | 100.0% | 1.000 | 12.5s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 42.3s |
| keyword | 20 | 75.0% | 75.0% | 75.0% | 0.750 | 12.2s |
| long | 12 | 100.0% | 91.7% | 100.0% | 1.000 | 16.6s |
| mixed | 13 | 92.3% | 84.6% | 92.3% | 0.923 | 17.5s |
| multi_intent | 9 | 88.9% | 66.7% | 88.9% | 0.944 | 8.6s |
| negation | 20 | 95.0% | 80.0% | 95.0% | 0.950 | 9.8s |
| oos_far | 9 | 100.0% | 88.9% | - | - | 23.1s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 8.0s |
| paraphrase | 177 | 96.6% | 93.2% | 95.7% | 0.957 | 10.9s |
| typo | 13 | 100.0% | 92.3% | 100.0% | 1.000 | 13.8s |
| vague | 24 | 87.5% | 100.0% | 87.5% | 0.924 | 14.8s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 45 | 93.3% | 95.6% | 92.9% | 0.937 | 12.1s |
| health | 39 | 100.0% | 97.4% | 100.0% | 1.000 | 11.7s |
| it | 100 | 94.0% | 82.0% | 93.7% | 0.937 | 10.5s |
| life | 41 | 95.1% | 92.7% | 94.6% | 0.946 | 14.7s |
| money | 48 | 95.8% | 89.6% | 95.6% | 0.963 | 9.1s |
| oos | 34 | 97.1% | 61.8% | - | - | 6.2s |
| shop | 40 | 90.0% | 87.5% | 89.2% | 0.905 | 9.8s |
| travel | 43 | 100.0% | 97.7% | 100.0% | 1.000 | 24.3s |
| work | 43 | 90.7% | 90.7% | 90.0% | 0.925 | 22.5s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 94.1% | 92.4% | 93.6% | 0.945 | 12.3s |
| hard | 113 | 94.7% | 80.5% | 93.6% | 0.942 | 10.9s |
| medium | 202 | 95.5% | 89.6% | 95.0% | 0.954 | 11.9s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 293 | 94.9% | 87.0% | 94.0% | 0.946 | 11.1s |
| test | 140 | 95.0% | 90.0% | 94.9% | 0.956 | 13.0s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 94.2% | 98.1% | 95.1% | 6.2% |
| 0.25 | 94.9% | 97.8% | 96.1% | 1.6% |
| 0.30 | 94.9% | 97.3% | 96.7% | 1.6% |
| 0.35 | 94.7% | 97.0% | 96.7% | 1.6% |
| 0.40 | 94.7% | 96.5% | 97.2% | 1.6% |
| 0.50 | 94.2% | 95.4% | 97.7% | 1.6% |
| 0.60 | 92.8% | 93.0% | 98.5% | 0.0% |
| 0.65 | 90.5% | 90.2% | 98.5% | 0.0% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 92.6% [83.9–96.8] (n=68) |
|   └ 올바른 카테고리에 저장 | 100.0% [93.6–100.0] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 58.3% [32.0–80.7] (n=12) |
| 계층 F1 | 1.000 [1.000–1.000] (n=56) |
| 초안이 검색에 안 보임 | 100.0% [93.6–100.0] (n=56) |
| 게시 성공 (버전 +1) | 100.0% [93.6–100.0] (n=56) |
| 초안→게시 카테고리 유지 | 100.0% [93.6–100.0] (n=56) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [93.6–100.0] (n=56) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 91.3% [73.2–97.6] (n=23) |
| 게시 후 probe 검색 top1 답변 | 91.3% [73.2–97.6] (n=23) |
| 지연 p50 (초안+게시+probe) | 26.7s |
| LLM 실패 → Jev 빔 대체율 | 4.9% [1.7–13.5] (n=61) |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| it_office_excel_func | it | 3 |
| it_office_excel_func | it_office_excel | 3 |
| food_store_item | food_store_freeze | 2 |
| it_office_excel_func | null | 2 |
| it_pc_win_keys | it | 2 |
| it_pc_win_keys | it_pc_win | 2 |
| null | money | 2 |
| health_sym_stomach | health_sym | 1 |
| it_office_excel_func | it_pc_win_power_boot | 1 |
| it_office_excel_func | work | 1 |
| it_office_excel_func | work_leave | 1 |
| it_pc_peri_monitor | it | 1 |
| it_pc_peri_monitor | it_pc_peri | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| life_clean_grease | food_tool | 1 |
| life_clean_stain | life_appl_wash | 1 |
| money_bank_cert | it_sec | 1 |
| money_bank_xfer_cap | money_bank | 1 |
| money_card_lost | money_card | 1 |

### 틀린 케이스 (61건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-OOS-001 | 내일 서울에 비가 오나요? | null | travel | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit | false_tree |
| I-WORK-007 | 연말정산에서 월세 세액공제를 받으려면 어떤 서류가 필요한가요? | null | work_pay | false_tree |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_order_cancel | false_tree |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.45 alternative | wrong top1 |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.80 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.89 recommended | route branch, wrong top1 |
| S-IT-009 | I plugged an external monitor into my laptop with HDMI but it shows nothing | it_pc_peri_monitor | it · top it_pc_peri_monitor#1 0.96 recommended | route shallow |
| S-IT-046 | 아까 그 순서 문제 다시요. 마우스를 오른쪽 끝으로 밀면 왼쪽 화면에서 튀어나와요 | it_pc_peri_monitor | it_pc_peri · top it_pc_peri_monitor#2 0.94 recommended | route shallow |
| S-IT-505 | windows shortcut to create a new virtual desktop | it_pc_win_keys | it · top it_pc_win_keys#win_ctrl_d 0.93 recommended | route shallow |
| S-IT-507 | 작업관리자 바로 뜨게하는 단추키 머에요 | it_pc_win_keys | it_pc_win · top it_pc_win_keys#ctrl_shift_esc 0.95 recommended | route shallow |
| S-IT-512 | Win+D 말고, 창을 전부 최소화했다가 따로 되돌리는 키가 있다던데요 | it_pc_win_keys | it_pc_win · top it_pc_win_keys#win_m 0.94 recommended | route shallow |
| S-IT-516 | 그럼 녹화는요? | it_pc_win_keys | it · top it_pc_win_keys#win_alt_r 0.92 recommended | route shallow |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · abstained · top work_leave_annual_left#1 0.28 reference | route domain, abstained |
| S-IT-519 | B열에 '완료'라고 적힌 칸이 몇 개인지만 알면 돼요 | it_office_excel_func | it_pc_win_power_boot · abstained · top it_pc_win_power_boot#2 0.11 reference | route branch, abstained |
| S-IT-520 | VLOOKUP 말고요, 찾는 열이 가져올 열보다 오른쪽에 있을 때는 어떻게 해요? | it_office_excel_func | it · top it_office_excel_func#index_match 0.85 recommended | route shallow |
| S-IT-521 | 근무일수 계산 함수 | it_office_excel_func | work_leave · top work_leave_annual_use#4 0.38 reference | route domain, wrong top1 |
| S-IT-522 | excel formula to add up values that meet several criteria at once | it_office_excel_func | it · top it_office_excel_func#sumifs 0.94 recommended | route shallow |
| S-IT-524 | 입사일만 넣으면 몇 년 다녔는지 자동으로 나오게 하고 싶은데ㅠ 방법 없나 | it_office_excel_func | it_office_excel · top it_office_excel_func#datedif 0.93 recommended | route shallow |
| S-IT-525 | lookup 했는데 값이 없으면 #N/A 대신 blank나 '없음'으로 보이게 하는 function 있어요? | it_office_excel_func | it_office_excel · top it_office_excel_func#ifna 0.94 recommended | route shallow |
| S-IT-526 | 셀에 있는 이메일 주소에서 골뱅이 앞 아이디 부분만 떼어 내고 싶어요 | it_office_excel_func | it · top it_office_excel_func#find 0.87 recommended | route shallow |
| S-IT-527 | 매달 거래처별 매출 파일을 받는데 거래처 이름이 수백 줄에 중복돼서 들어가 있어요. 보고서 첫 장에 거래처 목록을 한 번씩만 깔끔하게 뽑아 두고 | it_office_excel_func | it_office_excel · top it_office_excel_func#unique 0.92 recommended | route shallow |
| S-IT-528 | 4.56을 5로, 4.44는 4로 만드는 것처럼 소수점 첫째 자리에서 반올림해 정수로 만들려면요? | it_office_excel_func | null · abstained · top no items | route false_none, abstained |
| S-IT-529 | 텍스트 합치기 구분자 넣어서 | it_office_excel_func | null · abstained · top no items | route false_none, abstained |
| S-LIFE-015 | 가스레인지 삼발이 찌든 기름때 | life_clean_grease | food_tool · abstained · top food_tool#article 0.28 reference | route domain, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.70 recommended | route shallow |
| S-LIFE-029 | 피요. 이미 말라서 딱딱하게 굳었어요 | life_clean_stain | life_appl_wash · abstained · top life_appl_wash_wm_drum_care#article 0.09 reference | route branch, abstained |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec_phish#3 0.08 reference | route domain, abstained |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost | money_card · top money_card_lost#4 0.95 recommended | route shallow |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money_bank · top money_bank_xfer_cap#1 0.65 recommended | route shallow |
| S-MONEY-021 | 제 명의 신용카드 관련해서 이것저것 궁금한 게 있는데요 | money_card | money_card · top money_card_lost#4 0.70 recommended | wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | money_tax · top money_tax_yearend#4 0.60 alternative | route shallow |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card_limit#2 0.08 reference | route false_tree |
| S-OOS-001 | 내일 서울 날씨 어때요? 우산 챙겨야 하나 | null · no answer | travel · abstained · top travel_air_bag_lost#2 0.18 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it_office_excel_func · abstained · top it_office_excel_func#ifs 0.23 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money_save · top money_save#article 0.55 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health_aid_emerg#article 0.08 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_card_lost#4 0.20 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.09 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money · abstained · top money_card_lost#4 0.18 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_wm_drum_water_drain#1 0.16 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it_sec · abstained · top it_sec_hacked#1 0.07 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym · abstained · top health_sym#article 0.23 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.13 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_track#3 0.13 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.36 reference | route domain, wrong top1 |
| S-SHOP-005 | My order says delivered but there's nothing at my door. What should I check firs | shop_ship_missing | shop_ship · top shop_ship_missing#1 0.95 recommended | route shallow |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.46 alternative | wrong top1 |
| S-SHOP-025 | 벌써 택배사로 넘어갔다는데 받는 사람 전화번호를 잘못 적었어요 | shop_ship_addr | shop_ship · top shop_ship_addr#2 0.91 recommended | route shallow |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money_tax · abstained · top money_tax#article 0.21 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.59 alternative | route branch, wrong top1 |
| S-TRAVEL-024 | 영국은요? | travel_prep_visa | travel_prep · top travel_prep_visa#3 0.70 recommended | route shallow |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.73 recommended | wrong top1 |
| S-WORK-008 | 연말정산 공제 얘기 말고요, 매달 월급에서 떼가는 건강보험이랑 소득세 항목이 뭔지 알고 싶어요 | work_pay_slip | money_tax · abstained · top money_tax#article 0.18 reference | route domain, abstained |
| S-WORK-009 | Do I need my manager's approval before working late on a weekday to get overtime | work_pay_ot | work · top work_pay_ot#1 0.95 recommended | route shallow |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work · top work#article 0.67 recommended | route shallow, wrong top1 |
| S-WORK-032 | 경조금 받으려면 증빙으로 뭘 올려야 하죠? | work_leave_event | work_pay · abstained · top work_pay_slip#4 0.07 reference | route branch, abstained |
| S-XD-006 | 날짜 지난 우유를 모르고 마셨는데 배가 살살 아프고 계속 토할 것 같아요 | health_sym_stomach | health_sym · top health_sym_stomach#3 0.91 recommended | route shallow |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.57 alternative | route branch, wrong top1 |

## jev_llm · standard

### 검색 (432건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 93.5% [90.8–95.5] (n=432) |
|   └ 답할 수 있는 질문 | 92.7% [89.5–94.9] (n=368) |
|   └ 답할 수 없는 질문(보류가 정답) | 98.4% [91.7–99.7] (n=64) |
| 확신 정답률 (top1 정답 + recommended) | 87.2% [83.4–90.3] (n=368) |
| 라우팅 정확도 (정확 일치) | 87.7% [84.3–90.5] (n=432) |
| 계층 F1 (hF1) | 0.922 [0.899–0.944] (n=432) |
| 트리 밖 판정 precision | 90.9% [72.2–97.5] (n=22) |
| 트리 밖 판정 recall | 58.8% [42.2–73.6] (n=34) |
| 트리 밖 판정 F1 | 0.714 |
| 오보류율 (답할 수 있는데 보류) | 2.4% [1.3–4.6] (n=368) |
| 오답변율 (답할 수 없는데 답함) | 1.6% [0.3–8.3] (n=64) |
| Hit@1 | 92.9% [89.8–95.1] (n=368) |
| Hit@3 | 94.3% [91.4–96.2] (n=368) |
| Hit@k (limit) | 94.8% [92.1–96.7] (n=368) |
| MRR@k | 0.937 [0.913–0.961] (n=368) |
| nDCG@k | 0.931 [0.908–0.955] (n=368) |
| Recall@k | 0.940 [0.916–0.963] (n=368) |
| recommended 정밀도 | 0.969 [0.952–0.985] (n=327) |
| 라우팅이 맞았을 때 Hit@1 | 98.2% [96.1–99.2] (n=329) |
| 라우팅이 맞았을 때 MRR | 0.989 [0.980–0.998] (n=329) |
| 지연 p50 / p90 / p95 / 평균 | 23.5s / 46.7s / 52.5s / 24.8s |
| 라우팅 구간 평균 | 24.4s |
| 랭킹 후보 수 평균 | 16.3 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.45 |
| LLM 라우팅 호출 수 평균 | 1.81 |
| LLM 실패 → Jev 빔 대체율 | 5.3% [3.6–7.9] (n=432) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 97.4% (n=351) | 94.4% (n=341) | 90.4% (n=322) | 76.5% (n=81) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|---:|
| 9 | 1 | 7 | 444 | 2 | 17 | 20 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 38.1s |
| internal | 23 | 73.9% | 95.7% | 73.9% | 0.821 | 15.8s |
| leaf | 345 | 93.9% | 89.0% | 94.2% | 0.945 | 24.1s |
| no_answer | 27 | 100.0% | 100.0% | - | - | 13.3s |
| oos | 34 | 97.1% | 58.8% | - | - | 12.3s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 94.3% | 89.5% | 94.2% | 0.942 | 24.1s |
| single | 327 | 93.3% | 87.2% | 92.5% | 0.935 | 23.4s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 327 | 93.3% | 87.2% | 92.5% | 0.935 | 23.4s |
| coref | 19 | 89.5% | 84.2% | 89.5% | 0.895 | 25.0s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 23.4s |
| disambig | 20 | 90.0% | 85.0% | 90.0% | 0.900 | 24.2s |
| distract | 9 | 88.9% | 88.9% | 88.9% | 0.889 | 38.5s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 14.2s |
| long | 8 | 100.0% | 87.5% | 100.0% | 1.000 | 22.8s |
| refine | 9 | 100.0% | 88.9% | 100.0% | 1.000 | 25.9s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 23.4s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 26.1s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 372 | 93.0% | 86.8% | 92.8% | 0.936 | 24.2s |
| root | 43 | 95.3% | 90.7% | 92.9% | 0.929 | 11.3s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 5.3s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 82.4% | 97.1% | 73.9% | 0.821 | 13.3s |
| leaf | 364 | 94.2% | 89.6% | 94.2% | 0.945 | 24.1s |
| none | 34 | 97.1% | 58.8% | - | - | 12.3s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 34 | 97.1% | 58.8% | - | - | 12.3s |
| d1 | 10 | 80.0% | 90.0% | 80.0% | 0.850 | 15.8s |
| d2 | 30 | 86.7% | 100.0% | 78.9% | 0.862 | 13.4s |
| d3 | 271 | 94.8% | 93.0% | 94.9% | 0.953 | 24.0s |
| d4 | 87 | 92.0% | 78.2% | 91.7% | 0.917 | 25.3s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 37 | 97.3% | 62.2% | - | - | 13.1s |
| 101+ | 4 | 50.0% | 75.0% | 50.0% | 0.625 | 15.8s |
| 2-5 | 311 | 95.2% | 92.9% | 95.3% | 0.958 | 23.9s |
| 33-100 | 61 | 88.5% | 73.8% | 88.1% | 0.881 | 24.1s |
| 6-32 | 19 | 84.2% | 100.0% | 70.0% | 0.787 | 12.9s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 25.2s |
| colloquial | 69 | 94.2% | 94.2% | 93.8% | 0.946 | 23.5s |
| condition | 32 | 93.8% | 90.6% | 96.8% | 0.968 | 25.7s |
| english | 12 | 100.0% | 66.7% | 100.0% | 1.000 | 34.3s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 42.6s |
| keyword | 20 | 80.0% | 80.0% | 80.0% | 0.800 | 34.5s |
| long | 12 | 100.0% | 91.7% | 100.0% | 1.000 | 28.2s |
| mixed | 13 | 92.3% | 84.6% | 92.3% | 0.923 | 33.8s |
| multi_intent | 9 | 88.9% | 55.6% | 88.9% | 0.944 | 23.7s |
| negation | 20 | 95.0% | 80.0% | 95.0% | 0.950 | 23.2s |
| oos_far | 9 | 100.0% | 88.9% | - | - | 9.2s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 33.8s |
| paraphrase | 177 | 95.5% | 92.7% | 94.2% | 0.942 | 22.9s |
| typo | 13 | 100.0% | 92.3% | 100.0% | 1.000 | 23.0s |
| vague | 23 | 73.9% | 95.7% | 73.9% | 0.821 | 15.8s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 45 | 86.7% | 91.1% | 85.7% | 0.881 | 30.0s |
| health | 39 | 97.4% | 97.4% | 97.1% | 0.975 | 34.6s |
| it | 100 | 94.0% | 83.0% | 93.7% | 0.937 | 24.3s |
| life | 40 | 95.0% | 90.0% | 94.4% | 0.944 | 23.4s |
| money | 48 | 93.8% | 89.6% | 95.6% | 0.961 | 13.3s |
| oos | 34 | 97.1% | 58.8% | - | - | 12.3s |
| shop | 40 | 87.5% | 90.0% | 86.5% | 0.878 | 24.4s |
| travel | 43 | 100.0% | 100.0% | 100.0% | 1.000 | 23.9s |
| work | 43 | 90.7% | 90.7% | 90.0% | 0.925 | 22.8s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 95.8% | 93.2% | 95.5% | 0.964 | 24.0s |
| hard | 112 | 93.8% | 79.5% | 93.5% | 0.942 | 23.2s |
| medium | 202 | 92.1% | 89.1% | 91.2% | 0.919 | 23.5s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 292 | 93.2% | 86.6% | 92.0% | 0.928 | 23.6s |
| test | 140 | 94.3% | 90.0% | 94.9% | 0.956 | 23.3s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 93.3% | 98.4% | 93.7% | 4.7% |
| 0.25 | 93.8% | 97.8% | 94.7% | 1.6% |
| 0.30 | 93.5% | 97.6% | 94.7% | 1.6% |
| 0.35 | 93.5% | 97.3% | 95.0% | 1.6% |
| 0.40 | 93.5% | 97.0% | 95.3% | 1.6% |
| 0.50 | 93.5% | 96.2% | 96.1% | 1.6% |
| 0.60 | 91.2% | 91.8% | 97.6% | 0.0% |
| 0.65 | 89.1% | 88.9% | 98.2% | 0.0% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 95.6% [87.8–98.5] (n=68) |
|   └ 올바른 카테고리에 저장 | 100.0% [93.6–100.0] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 75.0% [46.8–91.1] (n=12) |
| 계층 F1 | 1.000 [1.000–1.000] (n=56) |
| 초안이 검색에 안 보임 | 100.0% [93.6–100.0] (n=56) |
| 게시 성공 (버전 +1) | 100.0% [93.6–100.0] (n=56) |
| 초안→게시 카테고리 유지 | 100.0% [93.6–100.0] (n=56) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [93.6–100.0] (n=56) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 91.3% [73.2–97.6] (n=23) |
| 게시 후 probe 검색 top1 답변 | 91.3% [73.2–97.6] (n=23) |
| 지연 p50 (초안+게시+probe) | 49.3s |
| LLM 실패 → Jev 빔 대체율 | 1.7% [0.3–9.0] (n=59) |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 2 |
| it_office_excel_func | it | 2 |
| it_office_excel_func | it_office | 2 |
| it_office_excel_func | it_office_excel | 2 |
| it_office_excel_func | null | 2 |
| it_pc_win_keys | it_pc_win | 2 |
| null | life | 2 |
| food | food_cook | 1 |
| food_cook_side_fry | life_home_drain | 1 |
| health_sym_head | health | 1 |
| it_office_excel_func | work | 1 |
| it_office_excel_func | work_hr | 1 |
| it_office_excel_func | work_pay_sev | 1 |
| it_pc_peri_monitor | it | 1 |
| it_pc_peri_monitor | it_pc_peri | 1 |
| it_pc_win_keys | it | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| life_appl_wash_dryer_shrink | life_clean | 1 |
| life_appl_wash_wm_drum_care_smell | life_clean_towel | 1 |

### 틀린 케이스 (63건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-OOS-001 | 내일 서울에 비가 오나요? | null | travel | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it_office_excel_func | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit | false_tree |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food · top food_safe_poison#2 0.58 alternative | wrong top1 |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.47 alternative | wrong top1 |
| S-FOOD-021 | 튀기고 남은 기름 그냥 싱크대에 부어 버려도 돼요? | food_cook_side_fry | life_home_drain · top life_home_drain#4 0.54 alternative | route domain, wrong top1 |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.81 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.88 recommended | route branch, wrong top1 |
| S-HEALTH-015 | 요즘 밤에 잠도 잘 못 자고, 오후만 되면 머리가 조이듯 지끈거려요. 둘 다 어떻게 관리하면 좋을까요? | health_sym_head (+1) | health · top health_sym_head#1 0.62 alternative | route shallow |
| S-HEALTH-017 | 집에서 다쳤을 때 쓸 응급처치를 종류별로 한번 정리해 두고 싶어요 | health_aid | health_aid · top health_aid_injury_sprain#1 0.65 alternative | wrong top1 |
| S-IT-009 | I plugged an external monitor into my laptop with HDMI but it shows nothing | it_pc_peri_monitor | it · top it_pc_peri_monitor#1 0.96 recommended | route shallow |
| S-IT-046 | 아까 그 순서 문제 다시요. 마우스를 오른쪽 끝으로 밀면 왼쪽 화면에서 튀어나와요 | it_pc_peri_monitor | it_pc_peri · top it_pc_peri_monitor#2 0.95 recommended | route shallow |
| S-IT-505 | windows shortcut to create a new virtual desktop | it_pc_win_keys | it · top it_pc_win_keys#win_ctrl_d 0.94 recommended | route shallow |
| S-IT-507 | 작업관리자 바로 뜨게하는 단추키 머에요 | it_pc_win_keys | it_pc_win · top it_pc_win_keys#ctrl_shift_esc 0.95 recommended | route shallow |
| S-IT-512 | Win+D 말고, 창을 전부 최소화했다가 따로 되돌리는 키가 있다던데요 | it_pc_win_keys | it_pc_win · top it_pc_win_keys#win_m 0.94 recommended | route shallow |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · abstained · top work_pay_sev#2 0.24 reference | route domain, abstained |
| S-IT-519 | B열에 '완료'라고 적힌 칸이 몇 개인지만 알면 돼요 | it_office_excel_func | it_office · top it_office_excel_func#countif 0.78 recommended | route shallow |
| S-IT-520 | VLOOKUP 말고요, 찾는 열이 가져올 열보다 오른쪽에 있을 때는 어떻게 해요? | it_office_excel_func | it · top it_office_excel_func#index_match 0.86 recommended | route shallow |
| S-IT-521 | 근무일수 계산 함수 | it_office_excel_func | work_pay_sev · top work_pay_sev#2 0.39 reference | route domain, wrong top1 |
| S-IT-522 | excel formula to add up values that meet several criteria at once | it_office_excel_func | it_office · top it_office_excel_func#sumifs 0.95 recommended | route shallow |
| S-IT-524 | 입사일만 넣으면 몇 년 다녔는지 자동으로 나오게 하고 싶은데ㅠ 방법 없나 | it_office_excel_func | work_hr · abstained · top work_hr#article 0.08 reference | route domain, abstained |
| S-IT-525 | lookup 했는데 값이 없으면 #N/A 대신 blank나 '없음'으로 보이게 하는 function 있어요? | it_office_excel_func | it_office_excel · top it_office_excel_func#ifna 0.93 recommended | route shallow |
| S-IT-526 | 셀에 있는 이메일 주소에서 골뱅이 앞 아이디 부분만 떼어 내고 싶어요 | it_office_excel_func | it · top it_office_excel_func#find 0.89 recommended | route shallow |
| S-IT-527 | 매달 거래처별 매출 파일을 받는데 거래처 이름이 수백 줄에 중복돼서 들어가 있어요. 보고서 첫 장에 거래처 목록을 한 번씩만 깔끔하게 뽑아 두고 | it_office_excel_func | it_office_excel · top it_office_excel_func#unique 0.92 recommended | route shallow |
| S-IT-528 | 4.56을 5로, 4.44는 4로 만드는 것처럼 소수점 첫째 자리에서 반올림해 정수로 만들려면요? | it_office_excel_func | null · abstained · top no items | route false_none, abstained |
| S-IT-529 | 텍스트 합치기 구분자 넣어서 | it_office_excel_func | null · abstained · top no items | route false_none, abstained |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell | life_clean_towel · top life_clean_towel#1 0.56 alternative | route branch, wrong top1 |
| S-LIFE-008 | 건조기 돌리면 안 되고 그냥 널어서 말려야 하는 옷은 어떤 거예요? | life_appl_wash_dryer_shrink | life_clean · abstained · top life_clean_towel#2 0.21 reference | route branch, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_ac#2 0.67 recommended | route shallow |
| S-LIFE-029 | 피요. 이미 말라서 딱딱하게 굳었어요 | life_clean_stain | life_clean · top life_clean_stain#3 0.92 recommended | route shallow |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec#article 0.08 reference | route domain, abstained |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost | money_card · top money_card_lost#4 0.96 recommended | route shallow |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money_bank · top money_bank_xfer_cap#1 0.63 alternative | route shallow |
| S-MONEY-021 | 제 명의 신용카드 관련해서 이것저것 궁금한 게 있는데요 | money_card | money_card · top money_card_lost#4 0.73 recommended | wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | money_tax · top money_tax_yearend#4 0.61 alternative | route shallow |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card_limit#2 0.11 reference | route false_tree |
| S-OOS-001 | 내일 서울 날씨 어때요? 우산 챙겨야 하나 | null · no answer | travel_local · abstained · top travel_local_metro#3 0.06 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it_office_excel_func · abstained · top it_office_excel_func#ifs 0.24 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · top money_save#article 0.59 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health_aid_emerg_cpr#1 0.08 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | life · abstained · top life_appl_wash_wm_top_clean#2 0.12 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.09 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money_save · abstained · top money_save#article 0.07 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_wm_drum_care_smell#2 0.12 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it_sec · abstained · top it_sec_2fa#2 0.10 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym · abstained · top health_sym#article 0.22 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.13 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_addr#2 0.12 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.34 reference | route domain, wrong top1 |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.45 alternative | wrong top1 |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money_tax · abstained · top money_tax#article 0.20 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.51 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.75 recommended | wrong top1 |
| S-WORK-009 | Do I need my manager's approval before working late on a weekday to get overtime | work_pay_ot | work_pay · top work_pay_ot#1 0.94 recommended | route shallow |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work · top work#article 0.65 recommended | route shallow, wrong top1 |
| S-WORK-032 | 경조금 받으려면 증빙으로 뭘 올려야 하죠? | work_leave_event | work_pay · abstained · top work_pay#article 0.07 reference | route branch, abstained |
| S-XD-016 | 미국에 사는 동생한테 돈을 보내려는데 카드로도 보낼 수 있어요? 뭐가 필요해요? | money_bank_xfer_abroad | money_bank_xfer_abroad · abstained · top money_bank_xfer_abroad#1 0.29 reference | abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.59 alternative | route branch, wrong top1 |
| S-XD-027 | 환불은 언제 들어와요? | money_tax_yearend | money · top money_tax_yearend#2 0.94 recommended | route shallow |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.83 recommended | route branch, wrong top1 |

## jev_llm · shallow

### 검색 (388건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 94.8% [92.2–96.6] (n=388) |
|   └ 답할 수 있는 질문 | 94.7% [91.7–96.6] (n=337) |
|   └ 답할 수 없는 질문(보류가 정답) | 96.1% [86.8–98.9] (n=51) |
| 확신 정답률 (top1 정답 + recommended) | 89.0% [85.2–91.9] (n=337) |
| 라우팅 정확도 (정확 일치) | 89.2% [85.7–91.9] (n=388) |
| 계층 F1 (hF1) | 0.920 [0.895–0.945] (n=388) |
| 트리 밖 판정 precision | 95.0% [76.4–99.1] (n=20) |
| 트리 밖 판정 recall | 59.4% [42.3–74.5] (n=32) |
| 트리 밖 판정 F1 | 0.731 |
| 오보류율 (답할 수 있는데 보류) | 2.1% [1.0–4.2] (n=337) |
| 오답변율 (답할 수 없는데 답함) | 3.9% [1.1–13.2] (n=51) |
| Hit@1 | 94.7% [91.7–96.6] (n=337) |
| Hit@3 | 95.3% [92.4–97.1] (n=337) |
| Hit@k (limit) | 95.3% [92.4–97.1] (n=337) |
| MRR@k | 0.949 [0.926–0.972] (n=337) |
| nDCG@k | 0.941 [0.918–0.965] (n=337) |
| Recall@k | 0.945 [0.921–0.968] (n=337) |
| recommended 정밀도 | 0.979 [0.964–0.995] (n=306) |
| 라우팅이 맞았을 때 Hit@1 | 99.4% [97.7–99.8] (n=309) |
| 라우팅이 맞았을 때 MRR | 0.996 [0.991–1.000] (n=309) |
| 지연 p50 / p90 / p95 / 평균 | 14.5s / 42.6s / 46.1s / 18.8s |
| 라우팅 구간 평균 | 18.4s |
| 랭킹 후보 수 평균 | 19.2 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.41 |
| LLM 라우팅 호출 수 평균 | 1.85 |
| LLM 실패 → Jev 빔 대체율 | 2.8% [1.6–5.0] (n=388) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 |
|---|---:|---:|
| 일치율 | 96.7% (n=336) | 90.8% (n=326) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 6 | 10 | 401 | 1 | 17 | 12 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 30.7s |
| internal | 10 | 90.0% | 100.0% | 90.0% | 0.933 | 15.9s |
| leaf | 327 | 94.8% | 91.4% | 94.8% | 0.950 | 14.6s |
| no_answer | 16 | 100.0% | 93.8% | - | - | 17.6s |
| oos | 32 | 93.8% | 59.4% | - | - | 9.4s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 93.3% | 90.5% | 93.2% | 0.932 | 14.3s |
| single | 283 | 95.4% | 88.7% | 95.3% | 0.957 | 14.6s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 283 | 95.4% | 88.7% | 95.3% | 0.957 | 14.6s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 19.3s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 14.3s |
| disambig | 20 | 95.0% | 90.0% | 95.0% | 0.950 | 14.5s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 9.3s |
| ellipsis | 9 | 100.0% | 88.9% | 100.0% | 1.000 | 19.2s |
| long | 8 | 75.0% | 75.0% | 75.0% | 0.750 | 24.5s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 11.6s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 13.7s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 19.7s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 357 | 94.7% | 89.1% | 94.4% | 0.946 | 15.7s |
| root | 31 | 96.8% | 90.3% | 100.0% | 1.000 | 3.4s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 10 | 90.0% | 100.0% | 90.0% | 0.933 | 15.9s |
| leaf | 346 | 95.1% | 91.6% | 94.8% | 0.950 | 14.9s |
| none | 32 | 93.8% | 59.4% | - | - | 9.4s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 32 | 93.8% | 59.4% | - | - | 9.4s |
| d1 | 10 | 90.0% | 100.0% | 90.0% | 0.933 | 15.9s |
| d2 | 346 | 95.1% | 91.6% | 94.8% | 0.950 | 14.9s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 35 | 94.3% | 62.9% | - | - | 11.2s |
| 101+ | 4 | 75.0% | 100.0% | 75.0% | 0.833 | 15.6s |
| 2-5 | 293 | 96.6% | 94.2% | 96.4% | 0.966 | 14.3s |
| 33-100 | 56 | 87.5% | 78.6% | 87.3% | 0.873 | 16.1s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 14.9s |
| colloquial | 66 | 97.0% | 98.5% | 96.8% | 0.976 | 14.6s |
| condition | 32 | 93.8% | 90.6% | 93.5% | 0.935 | 22.8s |
| english | 12 | 100.0% | 83.3% | 100.0% | 1.000 | 23.2s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 25.1s |
| keyword | 20 | 85.0% | 85.0% | 85.0% | 0.850 | 15.7s |
| long | 12 | 91.7% | 91.7% | 91.7% | 0.917 | 16.3s |
| mixed | 13 | 100.0% | 92.3% | 100.0% | 1.000 | 13.9s |
| multi_intent | 9 | 100.0% | 44.4% | 100.0% | 1.000 | 23.6s |
| negation | 20 | 95.0% | 90.0% | 95.0% | 0.950 | 23.2s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 5.6s |
| oos_near | 11 | 90.9% | 9.1% | - | - | 15.1s |
| paraphrase | 149 | 94.0% | 91.3% | 93.5% | 0.935 | 12.9s |
| typo | 13 | 100.0% | 92.3% | 100.0% | 1.000 | 14.5s |
| vague | 10 | 90.0% | 100.0% | 90.0% | 0.933 | 15.9s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 39 | 92.3% | 94.9% | 92.1% | 0.930 | 16.9s |
| health | 35 | 100.0% | 97.1% | 100.0% | 1.000 | 18.9s |
| it | 92 | 93.5% | 87.0% | 93.3% | 0.933 | 15.7s |
| life | 35 | 100.0% | 97.1% | 100.0% | 1.000 | 19.7s |
| money | 43 | 95.3% | 90.7% | 95.1% | 0.951 | 19.5s |
| oos | 32 | 93.8% | 59.4% | - | - | 9.4s |
| shop | 35 | 88.6% | 85.7% | 87.9% | 0.879 | 11.4s |
| travel | 38 | 100.0% | 100.0% | 100.0% | 1.000 | 8.1s |
| work | 39 | 92.3% | 89.7% | 91.9% | 0.932 | 12.3s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 109 | 97.2% | 96.3% | 97.0% | 0.975 | 11.5s |
| hard | 100 | 91.0% | 76.0% | 90.8% | 0.908 | 19.2s |
| medium | 179 | 95.5% | 92.2% | 95.0% | 0.952 | 15.0s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 260 | 94.6% | 88.1% | 94.8% | 0.949 | 15.0s |
| test | 128 | 95.3% | 91.4% | 94.4% | 0.949 | 13.4s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 94.3% | 98.8% | 94.7% | 7.8% |
| 0.25 | 94.6% | 98.5% | 95.2% | 5.9% |
| 0.30 | 94.8% | 97.9% | 96.1% | 3.9% |
| 0.35 | 94.6% | 96.7% | 96.9% | 2.0% |
| 0.40 | 94.6% | 96.4% | 97.2% | 2.0% |
| 0.50 | 94.1% | 95.5% | 97.5% | 2.0% |
| 0.60 | 91.8% | 92.3% | 98.1% | 2.0% |
| 0.65 | 90.2% | 90.5% | 98.0% | 2.0% |

### 저장 (ingest, 59건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 93.2% [83.8–97.3] (n=59) |
|   └ 올바른 카테고리에 저장 | 100.0% [92.7–100.0] (n=49) |
|   └ 트리/루트 밖 Q&A 거절 | 60.0% [31.3–83.2] (n=10) |
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
| 지연 p50 (초안+게시+probe) | 36.1s |
| LLM 실패 → Jev 빔 대체율 | 3.8% [1.0–12.8] (n=53) |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| it_pc_win_keys | it | 4 |
| it_office_excel_func | work | 3 |
| null | it | 3 |
| food_store_item | food_store_freeze | 2 |
| null | health | 2 |
| health_sym_head | health | 1 |
| it_office_excel_func | it | 1 |
| it_office_excel_func | it_office_excel_print | 1 |
| it_office_excel_func | null | 1 |
| it_pc_peri_monitor | it | 1 |
| it_pc_peri_monitor | it_pc_win_keys | 1 |
| life_appl_air_ac | life | 1 |
| money_bank_acct_limit | work_hr_cert | 1 |
| money_bank_xfer_cap | money | 1 |
| money_tax_yearend | money | 1 |
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | food_safe_poison | 1 |
| null | life | 1 |
| null | money | 1 |

### 틀린 케이스 (48건 중 48건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit_start | false_tree |
| I-SHOP-007 | 호텔 예약을 취소하면 수수료가 얼마나 나오나요? | null | shop_order_cancel | false_tree |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food · top food_safe_poison#2 0.59 alternative | wrong top1 |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.81 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.88 recommended | route branch, wrong top1 |
| S-HEALTH-015 | 요즘 밤에 잠도 잘 못 자고, 오후만 되면 머리가 조이듯 지끈거려요. 둘 다 어떻게 관리하면 좋을까요? | health_sym_head (+1) | health · top health_sleep#1 0.62 alternative | route shallow |
| S-IT-009 | I plugged an external monitor into my laptop with HDMI but it shows nothing | it_pc_peri_monitor | it · top it_pc_peri_monitor#1 0.96 recommended | route shallow |
| S-IT-046 | 아까 그 순서 문제 다시요. 마우스를 오른쪽 끝으로 밀면 왼쪽 화면에서 튀어나와요 | it_pc_peri_monitor | it_pc_win_keys · top it_pc_win_keys#alt_space 0.33 reference | route branch, wrong top1 |
| S-IT-505 | windows shortcut to create a new virtual desktop | it_pc_win_keys | it · top it_pc_win_keys#win_ctrl_d 0.93 recommended | route shallow |
| S-IT-507 | 작업관리자 바로 뜨게하는 단추키 머에요 | it_pc_win_keys | it · top it_pc_win_keys#ctrl_shift_esc 0.95 recommended | route shallow |
| S-IT-512 | Win+D 말고, 창을 전부 최소화했다가 따로 되돌리는 키가 있다던데요 | it_pc_win_keys | it · top it_pc_win_keys#win_m 0.94 recommended | route shallow |
| S-IT-516 | 그럼 녹화는요? | it_pc_win_keys | it · top it_pc_win_keys#win_alt_r 0.93 recommended | route shallow |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · top work_exp_claim#3 0.33 reference | route domain, wrong top1 |
| S-IT-521 | 근무일수 계산 함수 | it_office_excel_func | work · top work_pay_ot#1 0.50 alternative | route domain, wrong top1 |
| S-IT-524 | 입사일만 넣으면 몇 년 다녔는지 자동으로 나오게 하고 싶은데ㅠ 방법 없나 | it_office_excel_func | work · abstained · top work_exp_claim#3 0.26 reference | route domain, abstained |
| S-IT-525 | lookup 했는데 값이 없으면 #N/A 대신 blank나 '없음'으로 보이게 하는 function 있어요? | it_office_excel_func | it · top it_office_excel_func#ifna 0.94 recommended | route shallow |
| S-IT-527 | 매달 거래처별 매출 파일을 받는데 거래처 이름이 수백 줄에 중복돼서 들어가 있어요. 보고서 첫 장에 거래처 목록을 한 번씩만 깔끔하게 뽑아 두고 | it_office_excel_func | it_office_excel_print · abstained · top it_office_excel_print#2 0.05 reference | route branch, abstained |
| S-IT-528 | 4.56을 5로, 4.44는 4로 만드는 것처럼 소수점 첫째 자리에서 반올림해 정수로 만들려면요? | it_office_excel_func | null · abstained · top no items | route false_none, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life · top life_appl_air_ac#2 0.70 recommended | route shallow |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money · top money_bank_xfer_cap#1 0.59 alternative | route shallow |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.86 recommended | route domain, wrong top1 |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit | work_hr_cert · abstained · top work_hr_cert#2 0.25 reference | route domain, abstained |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card_install · abstained · top money_card_install#3 0.02 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · top it_sec_phish#2 0.34 reference | route false_tree, answered unanswerable |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money_save_etf · abstained · top money_save_etf#2 0.09 reference | route false_tree |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health#article 0.07 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel · abstained · top travel_local_metro#2 0.14 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money · abstained · top money_bank_acct_limit#3 0.16 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_wm_drum_water_drain#2 0.12 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it · abstained · top it_mobile_lost#2 0.25 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health · abstained · top health#article 0.28 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.15 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | it · abstained · top it_pc_win_keys#alt_tab 0.16 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.37 reference | route domain, wrong top1 |
| S-SHOP-009 | 카드로 산 운동화를 반품했는데 카드사 앱 승인 내역에 아직 취소가 안 보여요. 보통 며칠 걸려요? | shop_ret_refund_card | money · abstained · top money_card_lost#4 0.12 reference | route domain, abstained |
| S-SHOP-015 | 배송지 주소를 잘못 넣은 것도 고쳐야 하고, 무통장 입금은 언제까지 해야 하는지도 알려 주세요 | shop_ship_addr (+1) | shop · top shop_ship_addr#1 0.78 recommended | route shallow |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money · abstained · top money_bank_acct_limit#1 0.12 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.54 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.76 recommended | wrong top1 |
| S-WORK-008 | 연말정산 공제 얘기 말고요, 매달 월급에서 떼가는 건강보험이랑 소득세 항목이 뭔지 알고 싶어요 | work_pay_slip | money · abstained · top money#article 0.21 reference | route domain, abstained |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work · top work_pay_slip#2 0.68 recommended | route shallow |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#3 0.08 reference | route domain |
| S-XD-027 | 환불은 언제 들어와요? | money_tax_yearend | money · top money_tax_yearend#2 0.94 recommended | route shallow |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.82 recommended | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.84 recommended | route false_tree, answered unanswerable |
| S-XD-052 | 상한 음식을 먹고 계속 토해요. 어떻게 해야 돼요? | null · no answer | food_safe_poison · abstained · top food_safe_poison#2 0.14 reference | route false_tree |

## jev_llm · flat

### 검색 (357건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 97.8% [95.6–98.9] (n=357) |
|   └ 답할 수 있는 질문 | 97.5% [95.1–98.7] (n=317) |
|   └ 답할 수 없는 질문(보류가 정답) | 100.0% [91.2–100.0] (n=40) |
| 확신 정답률 (top1 정답 + recommended) | 90.2% [86.5–93.0] (n=317) |
| 라우팅 정확도 (정확 일치) | 97.5% [95.3–98.7] (n=357) |
| 계층 F1 (hF1) | 0.975 [0.959–0.991] (n=357) |
| 트리 밖 판정 precision | 95.2% [77.3–99.2] (n=21) |
| 트리 밖 판정 recall | 95.2% [77.3–99.2] (n=21) |
| 트리 밖 판정 F1 | 0.952 |
| 오보류율 (답할 수 있는데 보류) | 0.3% [0.1–1.8] (n=317) |
| 오답변율 (답할 수 없는데 답함) | 0.0% [0.0–8.8] (n=40) |
| Hit@1 | 97.5% [95.1–98.7] (n=317) |
| Hit@3 | 97.8% [95.5–98.9] (n=317) |
| Hit@k (limit) | 97.8% [95.5–98.9] (n=317) |
| MRR@k | 0.976 [0.960–0.993] (n=317) |
| nDCG@k | 0.963 [0.946–0.981] (n=317) |
| Recall@k | 0.964 [0.945–0.982] (n=317) |
| recommended 정밀도 | 0.985 [0.971–0.998] (n=290) |
| 라우팅이 맞았을 때 Hit@1 | 99.7% [98.2–99.9] (n=310) |
| 라우팅이 맞았을 때 MRR | 0.998 [0.995–1.000] (n=310) |
| 지연 p50 / p90 / p95 / 평균 | 3.3s / 34.8s / 39.0s / 8.3s |
| 라우팅 구간 평균 | 8.0s |
| 랭킹 후보 수 평균 | 12.1 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.19 |
| LLM 라우팅 호출 수 평균 | 0.97 |
| LLM 실패 → Jev 빔 대체율 | 3.1% [1.7–5.4] (n=357) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 |
|---|---:|
| 일치율 | 96.9% (n=326) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_none | false_tree |
|---:|---:|---:|---:|---:|
| 5 | 2 | 398 | 1 | 1 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 3.1s |
| leaf | 317 | 97.5% | 97.8% | 97.5% | 0.976 | 3.3s |
| no_answer | 16 | 100.0% | 93.8% | - | - | 3.1s |
| oos | 21 | 100.0% | 95.2% | - | - | 2.9s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 96.2% | 96.2% | 96.1% | 0.961 | 3.0s |
| single | 252 | 98.4% | 98.0% | 98.1% | 0.984 | 3.4s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 252 | 98.4% | 98.0% | 98.1% | 0.984 | 3.4s |
| coref | 19 | 89.5% | 89.5% | 89.5% | 0.895 | 3.3s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 3.9s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 4.0s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 3.0s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 3.8s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 3.7s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 2.7s |
| shift | 14 | 100.0% | 100.0% | 100.0% | 1.000 | 2.3s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 3.0s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 347 | 97.7% | 97.4% | 97.4% | 0.976 | 3.4s |
| root | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.3s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| leaf | 336 | 97.6% | 97.6% | 97.5% | 0.976 | 3.3s |
| none | 21 | 100.0% | 95.2% | - | - | 2.9s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 21 | 100.0% | 95.2% | - | - | 2.9s |
| d1 | 336 | 97.6% | 97.6% | 97.5% | 0.976 | 3.3s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 24 | 100.0% | 95.8% | - | - | 2.9s |
| 2-5 | 283 | 97.9% | 97.9% | 97.8% | 0.979 | 3.2s |
| 33-100 | 50 | 96.0% | 96.0% | 95.9% | 0.959 | 4.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 4.2s |
| colloquial | 66 | 98.5% | 100.0% | 98.4% | 0.992 | 3.0s |
| condition | 31 | 100.0% | 100.0% | 100.0% | 1.000 | 4.3s |
| english | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 2.6s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 4.8s |
| keyword | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 2.4s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 2.9s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 5.3s |
| multi_intent | 9 | 88.9% | 88.9% | 88.9% | 0.889 | 16.4s |
| negation | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 4.3s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 3.4s |
| oos_near | 11 | 100.0% | 90.9% | - | - | 2.8s |
| paraphrase | 129 | 96.1% | 95.3% | 95.6% | 0.956 | 3.1s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 3.9s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 36 | 94.4% | 94.4% | 94.3% | 0.943 | 3.7s |
| health | 32 | 100.0% | 100.0% | 100.0% | 1.000 | 3.7s |
| it | 89 | 97.8% | 97.8% | 97.7% | 0.977 | 3.8s |
| life | 33 | 97.0% | 97.0% | 96.8% | 0.968 | 3.9s |
| money | 41 | 100.0% | 100.0% | 100.0% | 1.000 | 3.2s |
| oos | 21 | 100.0% | 95.2% | - | - | 2.9s |
| shop | 32 | 96.9% | 96.9% | 96.7% | 0.967 | 4.4s |
| travel | 36 | 100.0% | 100.0% | 100.0% | 1.000 | 2.3s |
| work | 37 | 94.6% | 94.6% | 94.3% | 0.957 | 3.3s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 99 | 98.0% | 99.0% | 97.8% | 0.984 | 3.0s |
| hard | 95 | 94.7% | 92.6% | 93.4% | 0.934 | 3.7s |
| medium | 163 | 99.4% | 99.4% | 99.3% | 0.993 | 3.3s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 237 | 97.9% | 97.5% | 97.7% | 0.977 | 3.2s |
| test | 120 | 97.5% | 97.5% | 97.1% | 0.976 | 3.6s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 97.5% | 99.7% | 97.5% | 2.5% |
| 0.25 | 97.8% | 99.7% | 97.8% | 0.0% |
| 0.30 | 97.8% | 99.7% | 97.8% | 0.0% |
| 0.35 | 97.5% | 99.4% | 97.8% | 0.0% |
| 0.40 | 97.5% | 99.4% | 97.8% | 0.0% |
| 0.50 | 95.5% | 97.2% | 97.7% | 0.0% |
| 0.60 | 93.6% | 94.3% | 98.3% | 0.0% |
| 0.65 | 91.3% | 91.5% | 98.6% | 0.0% |

### 저장 (ingest, 50건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 100.0% [92.9–100.0] (n=50) |
|   └ 올바른 카테고리에 저장 | 100.0% [92.4–100.0] (n=47) |
|   └ 트리/루트 밖 Q&A 거절 | 100.0% [43.8–100.0] (n=3) |
| 계층 F1 | 1.000 [1.000–1.000] (n=47) |
| 초안이 검색에 안 보임 | 100.0% [92.4–100.0] (n=47) |
| 게시 성공 (버전 +1) | 97.9% [88.9–99.6] (n=47) |
| 초안→게시 카테고리 유지 | 97.9% [88.9–99.6] (n=47) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [92.4–100.0] (n=47) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 94.7% [75.4–99.1] (n=19) |
| 게시 후 probe 검색 top1 답변 | 94.7% [75.4–99.1] (n=19) |
| 지연 p50 (초안+게시+probe) | 11.1s |
| LLM 실패 → Jev 빔 대체율 | 10.6% [4.6–22.6] (n=47) |

### API 계약 (11건)

통과 100.0% [74.1–100.0] (n=11)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 2 |
| it_mobile_lost | travel_local_metro | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | null | 1 |
| null | money_save_etf | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| work_exp_trip | travel_air_ticket_mile | 1 |
| work_exp_trip | work_exp_claim | 1 |

### 틀린 케이스 (11건 중 11건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-WORK-001 | 사원증을 잃어버렸는데 재발급은 어떻게 받나요? | work_hr_onboard | work_hr_onboard | exact |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.81 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.87 recommended | route branch, wrong top1 |
| S-IT-017 | 지하철에 아이폰을 두고 내렸어요. 지금 어디 있는지 확인할 방법이 있을까요? | it_mobile_lost | travel_local_metro · top travel_local_metro#2 0.60 alternative | route domain, wrong top1 |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | null · abstained · top no items | route false_none, abstained |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money_save_etf · abstained · top money_save_etf#2 0.09 reference | route false_tree |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.56 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.77 recommended | wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#2 0.07 reference | route domain |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.60 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.84 recommended | route branch, wrong top1 |

## jev_llm · sparse

### 검색 (432건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 87.3% [83.8–90.1] (n=432) |
|   └ 답할 수 있는 질문 | 96.0% [91.9–98.0] (n=174) |
|   └ 답할 수 없는 질문(보류가 정답) | 81.4% [76.2–85.7] (n=258) |
| 확신 정답률 (top1 정답 + recommended) | 81.0% [74.6–86.2] (n=174) |
| 라우팅 정확도 (정확 일치) | 88.4% [85.1–91.1] (n=432) |
| 계층 F1 (hF1) | 0.922 [0.899–0.945] (n=432) |
| 트리 밖 판정 precision | 95.5% [78.2–99.2] (n=22) |
| 트리 밖 판정 recall | 61.8% [45.0–76.1] (n=34) |
| 트리 밖 판정 F1 | 0.750 |
| 오보류율 (답할 수 있는데 보류) | 2.9% [1.2–6.5] (n=174) |
| 오답변율 (답할 수 없는데 답함) | 18.6% [14.3–23.8] (n=258) |
| Hit@1 | 96.6% [92.7–98.4] (n=174) |
| Hit@3 | 96.6% [92.7–98.4] (n=174) |
| Hit@k (limit) | 97.7% [94.2–99.1] (n=174) |
| MRR@k | 0.968 [0.943–0.993] (n=174) |
| nDCG@k | 0.970 [0.946–0.994] (n=174) |
| Recall@k | 0.968 [0.944–0.993] (n=174) |
| recommended 정밀도 | 0.902 [0.857–0.947] (n=153) |
| 라우팅이 맞았을 때 Hit@1 | 99.4% [96.6–99.9] (n=163) |
| 라우팅이 맞았을 때 MRR | 0.995 [0.985–1.000] (n=163) |
| 지연 p50 / p90 / p95 / 평균 | 7.6s / 45.4s / 46.5s / 15.4s |
| 라우팅 구간 평균 | 15.1s |
| 랭킹 후보 수 평균 | 3.3 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.16 |
| LLM 라우팅 호출 수 평균 | 1.81 |
| LLM 실패 → Jev 빔 대체율 | 6.9% [4.9–9.7] (n=432) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 97.2% (n=351) | 93.3% (n=341) | 90.1% (n=322) | 74.1% (n=81) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 7 | 9 | 433 | 1 | 18 | 20 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dropped | 194 | 75.8% | 86.6% | - | - | 8.4s |
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 21.0s |
| internal | 23 | 95.7% | 100.0% | 95.7% | 0.965 | 9.8s |
| leaf | 151 | 96.0% | 92.7% | 96.7% | 0.969 | 6.9s |
| no_answer | 27 | 100.0% | 100.0% | - | - | 7.9s |
| oos | 34 | 97.1% | 61.8% | - | - | 5.9s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 85.7% | 89.5% | 95.2% | 0.952 | 9.3s |
| single | 327 | 87.8% | 88.1% | 97.0% | 0.973 | 7.3s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 327 | 87.8% | 88.1% | 97.0% | 0.973 | 7.3s |
| coref | 19 | 84.2% | 89.5% | 100.0% | 1.000 | 10.2s |
| correct | 8 | 87.5% | 87.5% | 80.0% | 0.800 | 7.8s |
| disambig | 20 | 80.0% | 85.0% | 90.0% | 0.900 | 21.6s |
| distract | 9 | 88.9% | 100.0% | 100.0% | 1.000 | 16.1s |
| ellipsis | 9 | 77.8% | 88.9% | 100.0% | 1.000 | 13.4s |
| long | 8 | 87.5% | 75.0% | 100.0% | 1.000 | 11.1s |
| refine | 9 | 100.0% | 88.9% | 100.0% | 1.000 | 10.0s |
| shift | 14 | 92.9% | 92.9% | 100.0% | 1.000 | 4.1s |
| slot | 9 | 77.8% | 100.0% | 100.0% | 1.000 | 16.7s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 372 | 87.4% | 87.1% | 96.4% | 0.966 | 8.3s |
| root | 43 | 88.4% | 95.3% | 100.0% | 1.000 | 3.5s |
| start | 17 | 82.4% | 100.0% | 100.0% | 1.000 | 5.1s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 97.1% | 100.0% | 95.7% | 0.965 | 8.3s |
| leaf | 364 | 85.4% | 89.8% | 96.7% | 0.969 | 7.8s |
| none | 34 | 97.1% | 61.8% | - | - | 5.9s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 34 | 97.1% | 61.8% | - | - | 5.9s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 9.8s |
| d2 | 30 | 96.7% | 100.0% | 93.3% | 0.947 | 8.3s |
| d3 | 271 | 84.5% | 93.7% | 96.5% | 0.967 | 8.2s |
| d4 | 87 | 87.4% | 77.0% | 97.1% | 0.971 | 6.8s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 37 | 97.3% | 64.9% | - | - | 6.2s |
| 1 | 361 | 85.3% | 89.8% | 96.7% | 0.969 | 7.8s |
| 2-5 | 16 | 100.0% | 100.0% | 100.0% | 1.000 | 10.7s |
| 33-100 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 42.9s |
| 6-32 | 16 | 93.8% | 100.0% | 92.9% | 0.943 | 8.3s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 87.5% | 100.0% | - | - | 5.4s |
| colloquial | 69 | 85.5% | 95.7% | 100.0% | 1.000 | 7.1s |
| condition | 32 | 81.2% | 87.5% | 100.0% | 1.000 | 8.2s |
| english | 12 | 91.7% | 66.7% | 100.0% | 1.000 | 6.3s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 7.0s |
| keyword | 20 | 75.0% | 80.0% | 90.0% | 0.900 | 9.3s |
| long | 12 | 91.7% | 91.7% | 100.0% | 1.000 | 9.5s |
| mixed | 13 | 92.3% | 84.6% | 85.7% | 0.857 | 7.0s |
| multi_intent | 9 | 88.9% | 44.4% | 88.9% | 0.917 | 39.7s |
| negation | 20 | 80.0% | 90.0% | 100.0% | 1.000 | 6.8s |
| oos_far | 9 | 100.0% | 88.9% | - | - | 5.1s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 13.6s |
| paraphrase | 177 | 88.1% | 93.2% | 96.1% | 0.961 | 7.9s |
| typo | 13 | 84.6% | 92.3% | 100.0% | 1.000 | 3.7s |
| vague | 23 | 95.7% | 100.0% | 95.7% | 0.965 | 9.8s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 45 | 82.2% | 95.6% | 100.0% | 1.000 | 4.6s |
| health | 39 | 89.7% | 97.4% | 93.8% | 0.950 | 5.6s |
| it | 100 | 90.0% | 82.0% | 97.8% | 0.978 | 6.0s |
| life | 40 | 80.0% | 90.0% | 95.5% | 0.955 | 6.8s |
| money | 48 | 79.2% | 89.6% | 95.8% | 0.958 | 9.5s |
| oos | 34 | 97.1% | 61.8% | - | - | 5.9s |
| shop | 40 | 82.5% | 90.0% | 94.4% | 0.944 | 13.4s |
| travel | 43 | 93.0% | 100.0% | 100.0% | 1.000 | 13.6s |
| work | 43 | 90.7% | 93.0% | 94.4% | 0.958 | 13.4s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 89.8% | 92.4% | 98.0% | 0.980 | 6.0s |
| hard | 112 | 84.8% | 78.6% | 93.8% | 0.945 | 10.7s |
| medium | 202 | 87.1% | 91.6% | 96.7% | 0.969 | 7.7s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 292 | 85.6% | 87.7% | 95.9% | 0.963 | 7.4s |
| test | 140 | 90.7% | 90.0% | 98.0% | 0.980 | 8.3s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 83.3% | 97.7% | 71.2% | 25.6% |
| 0.25 | 85.4% | 97.7% | 74.0% | 22.1% |
| 0.30 | 87.3% | 97.1% | 77.0% | 18.6% |
| 0.35 | 90.0% | 96.6% | 81.8% | 13.6% |
| 0.40 | 91.2% | 94.8% | 84.9% | 10.5% |
| 0.50 | 92.6% | 93.7% | 88.5% | 7.4% |
| 0.60 | 91.0% | 87.4% | 89.8% | 5.8% |
| 0.65 | 90.0% | 82.2% | 92.2% | 3.9% |

### 저장 (ingest, 56건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 91.1% [80.7–96.1] (n=56) |
|   └ 올바른 카테고리에 저장 | 100.0% [92.0–100.0] (n=44) |
|   └ 트리/루트 밖 Q&A 거절 | 58.3% [32.0–80.7] (n=12) |
| 계층 F1 | 1.000 [1.000–1.000] (n=44) |
| 초안이 검색에 안 보임 | 100.0% [92.0–100.0] (n=44) |
| 게시 성공 (버전 +1) | 100.0% [92.0–100.0] (n=44) |
| 초안→게시 카테고리 유지 | 100.0% [92.0–100.0] (n=44) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [92.0–100.0] (n=44) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 3 |
| 게시 후 probe 검색 top-k 포함 | 91.3% [73.2–97.6] (n=23) |
| 게시 후 probe 검색 top1 답변 | 91.3% [73.2–97.6] (n=23) |
| 지연 p50 (초안+게시+probe) | 26.8s |
| LLM 실패 → Jev 빔 대체율 | 6.1% [2.1–16.5] (n=49) |

### API 계약 (11건)

통과 100.0% [74.1–100.0] (n=11)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 2 |
| it_office_excel_func | it | 2 |
| it_office_excel_func | it_office | 2 |
| it_office_excel_func | it_office_excel | 2 |
| it_pc_win_keys | it | 2 |
| it_pc_win_keys | it_pc_win | 2 |
| null | health | 2 |
| null | money | 2 |
| shop_order_receipt | money_tax | 2 |
| health_sym_head | health | 1 |
| it_net_wifi_drop | it | 1 |
| it_office_excel_func | null | 1 |
| it_office_excel_func | work | 1 |
| it_office_excel_func | work_hr | 1 |
| it_office_excel_func | work_leave | 1 |
| it_pc_peri_monitor | it | 1 |
| it_pc_peri_monitor | it_pc_peri | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| life_appl_wash_wm_drum_care_gasket | life_clean_mold | 1 |

### 틀린 케이스 (97건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-001 | 내일 서울에 비가 오나요? | null | travel | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it_office_excel_func | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit_start | false_tree |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_ret | false_tree |
| S-FOOD-001 | 밥솥으로 밥을 했는데 밥알 가운데가 생쌀처럼 씹혀요 | food_cook_base_rice · no answer | food_cook_base_rice · top food_cook_base_rice#1 0.34 reference | answered unanswerable |
| S-FOOD-007 | 배탈 난 뒤 관리 말고요, 여름에 식중독 안 걸리게 미리 조심할 것들 알려주세요 | food_safe_poison · no answer | food_safe_poison · top food_safe_poison#1 0.45 alternative | answered unanswerable |
| S-FOOD-014 | 그거 깔끔하게 까는 요령 좀요 | food_cook_side_egg · no answer | food_cook_side_egg · top food_cook_side_egg#1 0.31 reference | answered unanswerable |
| S-FOOD-015 | 그럼 소면은요? | food_cook_base_noodle · no answer | food_cook_base_noodle · top food_cook_base_noodle#1 0.32 reference | answered unanswerable |
| S-FOOD-018 | 아 근데 냉장고에 있는 요거트가 날짜가 이틀 지났는데 먹어도 될까요? | food_safe_date · no answer | food_safe_date · top food_safe_date#1 0.31 reference | answered unanswerable |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item · no answer | food_store_freeze · top food_store_freeze#1 0.80 recommended | route branch, answered unanswerable |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item · no answer | food_store_freeze · top food_store_freeze#1 0.90 recommended | route branch, answered unanswerable |
| S-HEALTH-002 | 진통제 거의 매일 먹는데 이거 괜찮은거임? 안 먹으면 머리 깨질 것 같음 | health_sym_head · no answer | health_sym_head · top health_sym_head#1 0.30 reference | answered unanswerable |
| S-HEALTH-007 | My 8-month-old baby is choking on something and can't breathe. What should I do  | health_aid_emerg_choke · no answer | health_aid_emerg_choke · top health_aid_emerg_choke#1 0.37 reference | answered unanswerable |
| S-HEALTH-009 | 달리기 말고 헬스장에서 근력이랑 유산소를 섞어서 하려는데 뭐부터 해야 돼요? | health_fit_start · no answer | health_fit_start · top health_fit_start#1 0.42 alternative | answered unanswerable |
| S-HEALTH-015 | 요즘 밤에 잠도 잘 못 자고, 오후만 되면 머리가 조이듯 지끈거려요. 둘 다 어떻게 관리하면 좋을까요? | health_sym_head (+1) | health · top health_sleep#1 0.61 alternative | route shallow |
| S-HEALTH-017 | 집에서 다쳤을 때 쓸 응급처치를 종류별로 한번 정리해 두고 싶어요 | health_aid | health_aid · top health_aid_injury_sprain#1 0.66 recommended | wrong top1 |
| S-IT-009 | I plugged an external monitor into my laptop with HDMI but it shows nothing | it_pc_peri_monitor | it · top it_pc_peri_monitor#1 0.96 recommended | route shallow |
| S-IT-028 | 사이트마다 비밀번호를 다르게 쓰면 다 기억할 수가 없어요. | it_sec_pw · no answer | it_sec_pw · top it_sec_pw#1 0.39 reference | answered unanswerable |
| S-IT-041 | 같은 브랜드로 가요. 옮기기 전에 백업은 어떻게 해 둬요? | it_mobile_move_android · no answer | it_mobile_move_android · top it_mobile_move_android#1 0.65 alternative | answered unanswerable |
| S-IT-046 | 아까 그 순서 문제 다시요. 마우스를 오른쪽 끝으로 밀면 왼쪽 화면에서 튀어나와요 | it_pc_peri_monitor · no answer | it_pc_peri · abstained · top it_pc_peri#article 0.20 reference | route shallow |
| S-IT-051 | 누가 제 이메일 계정 비번이랑 복구 메일까지 바꿔 놔서 들어갈 수가 없어요 | it_sec_hacked · no answer | it_sec_hacked · top it_sec_hacked#1 0.33 reference | answered unanswerable |
| S-IT-503 | 전체 화면 말고 지금 띄워 놓은 창 하나만 찍고 싶어요 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.79 recommended | answered unanswerable |
| S-IT-504 | 클립보드 기록 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.82 recommended | answered unanswerable |
| S-IT-505 | windows shortcut to create a new virtual desktop | it_pc_win_keys · no answer | it · abstained · top it_pc_win#article 0.12 reference | route shallow |
| S-IT-507 | 작업관리자 바로 뜨게하는 단추키 머에요 | it_pc_win_keys · no answer | it_pc_win · top it_pc_win_perf_slow#1 0.72 recommended | route shallow, answered unanswerable |
| S-IT-512 | Win+D 말고, 창을 전부 최소화했다가 따로 되돌리는 키가 있다던데요 | it_pc_win_keys · no answer | it_pc_win · abstained · top it_pc_win#article 0.06 reference | route shallow |
| S-IT-516 | 그럼 녹화는요? | it_pc_win_keys · no answer | it · abstained · top it_pc_win#article 0.12 reference | route shallow |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func · no answer | work · abstained · top work_pay_ot#1 0.22 reference | route domain |
| S-IT-519 | B열에 '완료'라고 적힌 칸이 몇 개인지만 알면 돼요 | it_office_excel_func · no answer | it_office · abstained · top it_office_excel#article 0.10 reference | route shallow |
| S-IT-520 | VLOOKUP 말고요, 찾는 열이 가져올 열보다 오른쪽에 있을 때는 어떻게 해요? | it_office_excel_func · no answer | it · abstained · top it_office_excel#article 0.19 reference | route shallow |
| S-IT-521 | 근무일수 계산 함수 | it_office_excel_func · no answer | work_leave · top work_leave_event#1 0.39 reference | route domain, answered unanswerable |
| S-IT-522 | excel formula to add up values that meet several criteria at once | it_office_excel_func · no answer | it_office · abstained · top it_office_excel#article 0.13 reference | route shallow |
| S-IT-524 | 입사일만 넣으면 몇 년 다녔는지 자동으로 나오게 하고 싶은데ㅠ 방법 없나 | it_office_excel_func · no answer | work_hr · abstained · top work_hr#article 0.06 reference | route domain |
| S-IT-525 | lookup 했는데 값이 없으면 #N/A 대신 blank나 '없음'으로 보이게 하는 function 있어요? | it_office_excel_func · no answer | it_office_excel · abstained · top it_office_excel#article 0.06 reference | route shallow |
| S-IT-526 | 셀에 있는 이메일 주소에서 골뱅이 앞 아이디 부분만 떼어 내고 싶어요 | it_office_excel_func · no answer | it · abstained · top it_office#article 0.15 reference | route shallow |
| S-IT-527 | 매달 거래처별 매출 파일을 받는데 거래처 이름이 수백 줄에 중복돼서 들어가 있어요. 보고서 첫 장에 거래처 목록을 한 번씩만 깔끔하게 뽑아 두고 | it_office_excel_func · no answer | it_office_excel · abstained · top it_office_excel#article 0.10 reference | route shallow |
| S-IT-528 | 4.56을 5로, 4.44는 4로 만드는 것처럼 소수점 첫째 자리에서 반올림해 정수로 만들려면요? | it_office_excel_func · no answer | null · abstained · top no items | route false_none |
| S-LIFE-001 | 드럼세탁기 코스가 다 끝났는데 옷이 물에 푹 젖은 채로 나와요. 탈수가 안 된 것 같아요 | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.79 recommended | answered unanswerable |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell · no answer | life_clean_towel · top life_clean_towel#1 0.55 alternative | route branch, answered unanswerable |
| S-LIFE-004 | 드럼 고무패킹 곰팡이 | life_appl_wash_wm_drum_care_gasket | life_clean_mold · abstained · top life_clean_mold#1 0.16 reference | route branch, abstained |
| S-LIFE-006 | 통돌이 세탁기로 빨래하면 검은 찌꺼기가 뭍어나와여 청소 어케해요 | life_appl_wash_wm_top_clean · no answer | life_appl_wash_wm_top_clean · top life_appl_wash_wm_top_clean#1 0.83 recommended | answered unanswerable |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.53 alternative | route shallow |
| S-LIFE-029 | 피요. 이미 말라서 딱딱하게 굳었어요 | life_clean_stain · no answer | life_clean · abstained · top life_clean_stain#1 0.24 reference | route shallow |
| S-LIFE-030 | 빨래를 조금만 넣었을 때 탈수가 잘 안 되는데 왜 그래요? | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.79 recommended | answered unanswerable |
| S-LIFE-035 | 그럼 아까 그거 빨고 나서 방에서 말릴 때 냄새 안 나게 하려면 어떻게 해요? | life_clean_towel · no answer | life_clean_towel · top life_clean_towel#1 0.48 alternative | answered unanswerable |
| S-LIFE-036 | 겨울만 되면 거실 창틀이 곰팡이로 까매져요 | life_clean_mold · no answer | life_clean_mold · top life_clean_mold#1 0.40 alternative | answered unanswerable |
| S-LIFE-038 | 세면대 물이 고인 채로 한참 걸려서 빠져요 | life_home_drain · no answer | life_home_drain · top life_home_drain#1 0.31 reference | answered unanswerable |
| S-MONEY-002 | 통장 만든 지 일주일 됐는데 하루에 30만 원까지만 보내져요. 앱에서 이체 한도 올리기를 눌러 봐도 안 되는데 이거 계좌 자체가 묶인 건가요? | money_bank_acct_limit · no answer | money_bank_acct_limit · top money_bank_acct_limit#1 0.35 reference | answered unanswerable |
| S-MONEY-005 | 인터넷뱅킹으로 계좌이체할 때 하루 한도를 늘리려면 OTP가 꼭 있어야 하나요? | money_bank_xfer_cap · no answer | money_bank_xfer_cap · top money_bank_xfer_cap#1 0.64 alternative | answered unanswerable |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec#article 0.07 reference | route domain, abstained |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost · no answer | money_card_lost · top money_card_lost#1 0.43 alternative | answered unanswerable |
| S-MONEY-012 | 온라인 쇼핑몰에서 결제하는데 한도 초과라고 승인이 안 나요. 쇼핑몰 오류가 아니라 제 신용카드 한도가 찬 거라는데 지금 바로 결제하려면 어떻게  | money_card_limit · no answer | shop_order_pay · abstained · top shop_order_pay#1 0.06 reference | route domain |
| S-MONEY-017 | 직장인인데 주말마다 과외해서 번 돈이 좀 있어요. 이것도 회사 연말정산으로 끝나나요, 아니면 제가 따로 신고해야 하나요? | money_tax_income · no answer | money_tax_income · top money_tax_income#1 0.45 alternative | answered unanswerable |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend · no answer | work_exp_claim · abstained · top work_exp_claim#1 0.14 reference | route domain |
| S-MONEY-027 | 마통은요? | money_loan_credit · no answer | money_loan_credit · top money_loan_credit#1 0.62 alternative | answered unanswerable |
| S-MONEY-033 | 한도를 당장 다시 살리는 방법은 없어요? | money_card_limit · no answer | money_card_limit · top money_card_limit#1 0.36 reference | answered unanswerable |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit · no answer | work_hr_cert · abstained · top work_hr_cert#1 0.12 reference | route domain |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card#article 0.05 reference | route false_tree |
| S-MONEY-039 | 휴면 예금이 서민금융진흥원으로 넘어갔다는데 그럼 이제 못 돌려받는 거예요? | money_bank_acct_dormant · no answer | money_bank_acct_dormant · top money_bank_acct_dormant#1 0.31 reference | answered unanswerable |

