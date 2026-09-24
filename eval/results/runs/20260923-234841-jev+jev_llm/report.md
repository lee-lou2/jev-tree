# 평가 결과 — 20260923-234841-jev+jev_llm

- 실행: 2026-09-23T23:48:41Z → 2026-09-24T01:17:29Z (UTC) · git `eafd27a`
- 데이터셋: `kb` v1.0.0 · kb `489e5fca3c7c` · cases `11b27f3d7b79`
- 설정: routers ["jev","jev_llm"] · variants ["deep","standard","shallow","flat","sparse"] · suites ["all"] · split all · repeat 1 · concurrency 10 · beam_width "case/default" · limit "case/default"
- 모델(jev): jev `jev-latest`
- 모델(jev_llm): jev `jev-latest` · LLM `gpt-6-luna`
- 메모: both routers; jev_llm=gpt-6-luna via proxy.zvzo.ai

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev | deep | 433 | 92.4% | 89.6% | 0.924 | 91.6% | 0.921 | 0.909 | 0.764 | 3.1% | 3.0% | 1.7s / 2.9s | 4.4 | 0 | 86.8% | 100.0% |
| jev | standard | 432 | 92.1% | 89.8% | 0.928 | 91.3% | 0.919 | 0.910 | 0.764 | 3.1% | 2.7% | 1.6s / 2.8s | 4.1 | 0 | 86.8% | 100.0% |
| jev | shallow | 388 | 95.4% | 93.0% | 0.937 | 95.3% | 0.955 | 0.942 | 0.745 | 3.9% | 1.5% | 1.2s / 2.2s | 3.2 | 0 | 91.5% | 100.0% |
| jev | flat | 357 | 97.5% | 97.5% | 0.975 | 97.2% | 0.973 | 0.959 | 1.000 | 0.0% | 0.0% | 0.9s / 1.6s | 2.1 | 0 | 100.0% | 100.0% |
| jev | sparse | 432 | 84.5% | 89.6% | 0.926 | 90.8% | 0.909 | 0.912 | 0.764 | 19.0% | 6.3% | 1.7s / 2.2s | 3.8 | 0 | 87.5% | 100.0% |
| jev_llm | deep | 433 | 95.2% | 88.7% | 0.927 | 94.6% | 0.951 | 0.943 | 0.764 | 1.6% | 2.2% | 15.1s / 47.0s | 1.5 | 0 | 97.1% | 100.0% |
| jev_llm | standard | 432 | 94.0% | 88.2% | 0.920 | 93.2% | 0.938 | 0.930 | 0.737 | 1.6% | 3.0% | 14.6s / 44.8s | 1.3 | 0 | 94.1% | 100.0% |
| jev_llm | shallow | 388 | 94.8% | 89.2% | 0.918 | 94.4% | 0.948 | 0.938 | 0.653 | 2.0% | 2.4% | 18.1s / 51.6s | 1.4 | 0 | 93.2% | 100.0% |
| jev_llm | flat | 357 | 98.0% | 97.8% | 0.978 | 97.8% | 0.979 | 0.965 | 0.952 | 0.0% | 0.3% | 3.6s / 35.4s | 1.2 | 0 | 100.0% | 100.0% |
| jev_llm | sparse | 432 | 86.6% | 88.0% | 0.921 | 95.4% | 0.960 | 0.963 | 0.691 | 19.0% | 2.3% | 13.9s / 45.1s | 1.0 | 0 | 92.9% | 100.0% |

## jev · deep

### 검색 (433건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 92.4% [89.5–94.5] (n=433) |
|   └ 답할 수 있는 질문 | 91.6% [88.3–94.0] (n=369) |
|   └ 답할 수 없는 질문(보류가 정답) | 96.9% [89.3–99.1] (n=64) |
| 확신 정답률 (top1 정답 + recommended) | 86.4% [82.6–89.6] (n=369) |
| 라우팅 정확도 (정확 일치) | 89.6% [86.4–92.1] (n=433) |
| 계층 F1 (hF1) | 0.924 [0.901–0.947] (n=433) |
| 트리 밖 판정 precision | 100.0% [84.5–100.0] (n=21) |
| 트리 밖 판정 recall | 61.8% [45.0–76.1] (n=34) |
| 트리 밖 판정 F1 | 0.764 |
| 오보류율 (답할 수 있는데 보류) | 3.0% [1.7–5.3] (n=369) |
| 오답변율 (답할 수 없는데 답함) | 3.1% [0.9–10.7] (n=64) |
| Hit@1 | 91.6% [88.3–94.0] (n=369) |
| Hit@3 | 92.4% [89.3–94.7] (n=369) |
| Hit@k (limit) | 92.7% [89.6–94.9] (n=369) |
| MRR@k | 0.921 [0.894–0.948] (n=369) |
| nDCG@k | 0.909 [0.882–0.937] (n=369) |
| Recall@k | 0.913 [0.886–0.941] (n=369) |
| recommended 정밀도 | 0.962 [0.942–0.981] (n=328) |
| 라우팅이 맞았을 때 Hit@1 | 98.8% [97.0–99.5] (n=339) |
| 라우팅이 맞았을 때 MRR | 0.993 [0.987–1.000] (n=339) |
| 지연 p50 / p90 / p95 / 평균 | 1.7s / 2.6s / 2.9s / 1.8s |
| 라우팅 구간 평균 | 1.3s |
| 랭킹 후보 수 평균 | 16.1 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 4.41 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 | d5 | d6 | d7 |
|---|---:|---:|---:|---:|---:|---:|---:|
| 일치율 | 97.4% (n=352) | 94.7% (n=342) | 91.6% (n=323) | 91.1% (n=146) | 85.0% (n=20) | 80.0% (n=10) | 75.0% (n=8) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 18 | 5 | 9 | 447 | 18 | 4 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 1.1s |
| internal | 24 | 70.8% | 83.3% | 70.8% | 0.760 | 1.7s |
| leaf | 345 | 93.0% | 92.5% | 93.0% | 0.932 | 1.8s |
| no_answer | 27 | 100.0% | 92.6% | - | - | 1.5s |
| oos | 34 | 94.1% | 61.8% | - | - | 0.5s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 92.4% | 91.4% | 92.2% | 0.922 | 1.8s |
| single | 328 | 92.4% | 89.0% | 91.4% | 0.920 | 1.7s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 328 | 92.4% | 89.0% | 91.4% | 0.920 | 1.7s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 1.7s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 1.9s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.8s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.9s |
| ellipsis | 9 | 88.9% | 88.9% | 88.9% | 0.889 | 2.0s |
| long | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 1.6s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.7s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 1.7s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.9s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 373 | 92.0% | 89.0% | 91.3% | 0.918 | 1.8s |
| root | 43 | 93.0% | 90.7% | 92.9% | 0.929 | 0.8s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 35 | 80.0% | 85.7% | 70.8% | 0.760 | 1.6s |
| leaf | 364 | 93.4% | 92.6% | 93.0% | 0.932 | 1.8s |
| none | 34 | 94.1% | 61.8% | - | - | 0.5s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 34 | 94.1% | 61.8% | - | - | 0.5s |
| d1 | 10 | 60.0% | 60.0% | 60.0% | 0.600 | 1.6s |
| d2 | 30 | 90.0% | 96.7% | 84.2% | 0.908 | 1.5s |
| d3 | 199 | 93.5% | 93.5% | 92.9% | 0.929 | 1.6s |
| d4 | 140 | 94.3% | 92.1% | 94.1% | 0.945 | 2.0s |
| d5 | 10 | 90.0% | 90.0% | 90.0% | 0.900 | 2.3s |
| d6 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 2.8s |
| d7 | 8 | 75.0% | 75.0% | 75.0% | 0.750 | 3.0s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 37 | 94.6% | 64.9% | - | - | 0.5s |
| 101+ | 4 | 50.0% | 50.0% | 50.0% | 0.500 | 4.3s |
| 2-5 | 311 | 92.9% | 92.3% | 92.5% | 0.928 | 1.7s |
| 33-100 | 61 | 91.8% | 90.2% | 91.5% | 0.915 | 2.6s |
| 6-32 | 20 | 90.0% | 100.0% | 81.8% | 0.909 | 1.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 87.5% | 100.0% | 1.000 | 1.6s |
| colloquial | 69 | 95.7% | 97.1% | 95.4% | 0.962 | 1.6s |
| condition | 32 | 87.5% | 84.4% | 87.1% | 0.871 | 2.0s |
| english | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 2.3s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 1.8s |
| keyword | 20 | 85.0% | 85.0% | 85.0% | 0.850 | 2.0s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 2.0s |
| mixed | 13 | 92.3% | 92.3% | 92.3% | 0.923 | 2.1s |
| multi_intent | 9 | 100.0% | 77.8% | 100.0% | 1.000 | 1.8s |
| negation | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.9s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.4s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 2.0s |
| paraphrase | 177 | 93.2% | 92.1% | 92.1% | 0.921 | 1.7s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 2.0s |
| vague | 24 | 70.8% | 83.3% | 70.8% | 0.760 | 1.7s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 45 | 88.9% | 88.9% | 88.1% | 0.887 | 1.8s |
| health | 39 | 100.0% | 100.0% | 100.0% | 1.000 | 1.6s |
| it | 100 | 97.0% | 95.0% | 96.8% | 0.968 | 2.3s |
| life | 41 | 90.2% | 87.8% | 89.2% | 0.892 | 1.8s |
| money | 48 | 89.6% | 91.7% | 88.9% | 0.900 | 1.6s |
| oos | 34 | 94.1% | 61.8% | - | - | 0.5s |
| shop | 40 | 77.5% | 80.0% | 75.7% | 0.770 | 1.6s |
| travel | 43 | 97.7% | 97.7% | 97.4% | 0.974 | 1.6s |
| work | 43 | 90.7% | 90.7% | 90.0% | 0.912 | 1.7s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 94.9% | 95.8% | 94.5% | 0.955 | 1.6s |
| hard | 113 | 90.3% | 79.6% | 88.5% | 0.885 | 1.7s |
| medium | 202 | 92.1% | 91.6% | 91.2% | 0.916 | 1.8s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 293 | 92.5% | 88.7% | 91.6% | 0.919 | 1.7s |
| test | 140 | 92.1% | 91.4% | 91.5% | 0.924 | 1.8s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 91.9% | 98.4% | 92.1% | 6.2% |
| 0.25 | 92.1% | 97.6% | 93.1% | 4.7% |
| 0.30 | 92.4% | 97.0% | 93.9% | 3.1% |
| 0.35 | 92.1% | 96.5% | 94.1% | 3.1% |
| 0.40 | 92.1% | 95.9% | 94.7% | 3.1% |
| 0.50 | 91.0% | 93.8% | 95.4% | 3.1% |
| 0.60 | 89.4% | 90.2% | 97.0% | 1.6% |
| 0.65 | 88.2% | 88.6% | 97.3% | 1.6% |

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
| food | food_cook | 2 |
| food_store_item | food_store_freeze | 2 |
| money_bank_acct_limit | money_bank_xfer_cap | 2 |
| null | it | 2 |
| null | money | 2 |
| food_tool | food_tool_pan | 1 |
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
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | health | 1 |

### 틀린 케이스 (58건 중 58건)

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
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food_cook · top food_cook#article 0.55 alternative | route deep, wrong top1 |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.48 alternative | wrong top1 |
| S-FOOD-012 | 스테인리스 냄비 바닥에 까맣게 탄 자국은 어떻게 벗겨요? | food_tool · no answer | food_tool_pan · abstained · top food_tool_pan#2 0.04 reference | route deep |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.77 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.87 recommended | route branch, wrong top1 |
| S-IT-031 | 엄마라면서 폰 액정이 깨졌다고 상품권 좀 사 달라는 문자가 왔는데 이상해요 | it_sec_phish | money · top money_bank_xfer_cap#2 0.32 reference | route domain, wrong top1 |
| S-IT-035 | 듀얼 모니터 쓰는데 작업 표시줄을 한쪽 화면에만 나오게 할 수 있나요? | it_pc_peri_monitor · no answer | it_pc_win · abstained · top it_pc_win_keys#alt_space 0.17 reference | route branch |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · abstained · top work_leave_annual_left#1 0.26 reference | route domain, abstained |
| S-IT-531 | 두 번째, 세 번째로 큰 값을 구하는 함수는? | it_office_excel_func | it · top it_office_excel_func#large 0.96 recommended | route shallow |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell | life_clean_towel · top life_clean_towel#1 0.57 alternative | route branch, wrong top1 |
| S-LIFE-004 | 드럼 고무패킹 곰팡이 | life_appl_wash_wm_drum_care_gasket | life_clean_mold · abstained · top life_clean_mold#2 0.18 reference | route branch, abstained |
| S-LIFE-008 | 건조기 돌리면 안 되고 그냥 널어서 말려야 하는 옷은 어떤 거예요? | life_appl_wash_dryer_shrink | life_clean · abstained · top life_clean_towel#2 0.25 reference | route branch, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_ac#2 0.70 recommended | route shallow |
| S-LIFE-020 | 자취 시작하고 집안일이 처음이라 뭐가 고장 나거나 지저분해지면 어디서부터 봐야 할지 모르겠어요 | life | life_home · top life_home#article 0.84 recommended | route deep, wrong top1 |
| S-MONEY-002 | 통장 만든 지 일주일 됐는데 하루에 30만 원까지만 보내져요. 앱에서 이체 한도 올리기를 눌러 봐도 안 되는데 이거 계좌 자체가 묶인 건가요? | money_bank_acct_limit | money_bank_xfer_cap · top money_bank_xfer_cap#3 0.57 alternative | route branch, wrong top1 |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec_phish#1 0.10 reference | route domain, abstained |
| S-MONEY-021 | 제 명의 신용카드 관련해서 이것저것 궁금한 게 있는데요 | money_card | money_card · top money_card_lost#4 0.73 recommended | wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.87 recommended | route domain, wrong top1 |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit | money_bank_xfer_cap · top money_bank_xfer_cap#1 0.38 reference | route branch, wrong top1 |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card_limit#2 0.08 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.25 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · top money_save#article 0.53 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health_aid_injury_cut#3 0.09 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_loan_credit#2 0.18 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.08 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money_save · abstained · top money_save#article 0.07 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_wm_drum_water_drain#1 0.16 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.23 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym_stomach · abstained · top health_sym_stomach#3 0.14 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.14 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_addr#2 0.13 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.37 reference | route domain, wrong top1 |
| S-SHOP-009 | 카드로 산 운동화를 반품했는데 카드사 앱 승인 내역에 아직 취소가 안 보여요. 보통 며칠 걸려요? | shop_ret_refund_card | money_card · abstained · top money_card#article 0.08 reference | route domain, abstained |
| S-SHOP-017 | 온라인으로 산 물건 때문에 문의하려는데 뭐부터 봐야 할지 모르겠어요 | shop | shop_ret · top shop_ret#article 0.79 recommended | route deep, wrong top1 |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.46 alternative | wrong top1 |
| S-SHOP-024 | 무통장으로 낸 건요? | shop_ret_refund_bank | shop_order_pay · abstained · top shop_order_pay#3 0.27 reference | route branch, abstained |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money_tax · abstained · top money_tax#article 0.20 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.51 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.75 recommended | wrong top1 |
| S-WORK-008 | 연말정산 공제 얘기 말고요, 매달 월급에서 떼가는 건강보험이랑 소득세 항목이 뭔지 알고 싶어요 | work_pay_slip | money_tax · abstained · top money_tax#article 0.18 reference | route domain, abstained |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work_leave · top work_leave_annual_use#2 0.44 alternative | route shallow |
| S-WORK-032 | 경조금 받으려면 증빙으로 뭘 올려야 하죠? | work_leave_event | work_pay · abstained · top work_pay_slip#4 0.07 reference | route branch, abstained |
| S-XD-003 | 쇼핑몰 주문을 카드 결제 후 바로 취소했는데 카드 명세서에는 아직 청구돼 있어요 | shop_ret_refund_card | shop_order_cancel · abstained · top shop_order_cancel#3 0.22 reference | route branch, abstained |
| S-XD-014 | 여행용 eSIM을 샀는데 폰에 어떻게 설치해서 써요? | travel_prep_data | it_mobile · abstained · top it_mobile#article 0.09 reference | route domain, abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.59 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.84 recommended | route branch, wrong top1 |
| S-XD-045 | 요리나 식재료 관련해서 도움을 받고 싶어요 | food | food_cook · top food_cook#article 0.63 alternative | route deep, wrong top1 |
| S-XD-047 | 카드로 산 물건 결제를 취소했는데 명세서에 아직 남아 있어요 | shop_ret_refund_card | shop_order_pay · top shop_order_pay#1 0.50 alternative | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.83 recommended | route false_tree, answered unanswerable |

## jev · standard

### 검색 (432건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 92.1% [89.2–94.3] (n=432) |
|   └ 답할 수 있는 질문 | 91.3% [88.0–93.8] (n=368) |
|   └ 답할 수 없는 질문(보류가 정답) | 96.9% [89.3–99.1] (n=64) |
| 확신 정답률 (top1 정답 + recommended) | 86.7% [82.8–89.8] (n=368) |
| 라우팅 정확도 (정확 일치) | 89.8% [86.6–92.3] (n=432) |
| 계층 F1 (hF1) | 0.928 [0.906–0.950] (n=432) |
| 트리 밖 판정 precision | 100.0% [84.5–100.0] (n=21) |
| 트리 밖 판정 recall | 61.8% [45.0–76.1] (n=34) |
| 트리 밖 판정 F1 | 0.764 |
| 오보류율 (답할 수 있는데 보류) | 2.7% [1.5–4.9] (n=368) |
| 오답변율 (답할 수 없는데 답함) | 3.1% [0.9–10.7] (n=64) |
| Hit@1 | 91.3% [88.0–93.8] (n=368) |
| Hit@3 | 92.7% [89.5–94.9] (n=368) |
| Hit@k (limit) | 92.7% [89.5–94.9] (n=368) |
| MRR@k | 0.919 [0.892–0.946] (n=368) |
| nDCG@k | 0.910 [0.883–0.937] (n=368) |
| Recall@k | 0.914 [0.887–0.942] (n=368) |
| recommended 정밀도 | 0.964 [0.945–0.983] (n=328) |
| 라우팅이 맞았을 때 Hit@1 | 98.5% [96.6–99.4] (n=339) |
| 라우팅이 맞았을 때 MRR | 0.992 [0.984–0.999] (n=339) |
| 지연 p50 / p90 / p95 / 평균 | 1.6s / 2.5s / 2.8s / 1.7s |
| 라우팅 구간 평균 | 1.2s |
| 랭킹 후보 수 평균 | 15.8 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 4.11 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 97.7% (n=351) | 94.7% (n=341) | 92.5% (n=322) | 90.1% (n=81) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 17 | 7 | 8 | 447 | 18 | 3 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 1.1s |
| internal | 23 | 56.5% | 73.9% | 56.5% | 0.638 | 1.7s |
| leaf | 345 | 93.6% | 93.3% | 93.6% | 0.938 | 1.7s |
| no_answer | 27 | 100.0% | 92.6% | - | - | 1.5s |
| oos | 34 | 94.1% | 61.8% | - | - | 0.5s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 93.3% | 92.4% | 93.2% | 0.932 | 1.6s |
| single | 327 | 91.7% | 89.0% | 90.6% | 0.914 | 1.6s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 327 | 91.7% | 89.0% | 90.6% | 0.914 | 1.6s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 1.6s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 1.4s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.6s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.6s |
| ellipsis | 9 | 88.9% | 88.9% | 88.9% | 0.889 | 1.6s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.8s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.7s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 1.6s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.6s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 372 | 91.7% | 89.2% | 91.0% | 0.916 | 1.7s |
| root | 43 | 93.0% | 90.7% | 92.9% | 0.929 | 0.7s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 70.6% | 79.4% | 56.5% | 0.638 | 1.6s |
| leaf | 364 | 94.0% | 93.4% | 93.6% | 0.938 | 1.6s |
| none | 34 | 94.1% | 61.8% | - | - | 0.5s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 34 | 94.1% | 61.8% | - | - | 0.5s |
| d1 | 10 | 60.0% | 60.0% | 60.0% | 0.600 | 1.8s |
| d2 | 30 | 80.0% | 90.0% | 68.4% | 0.772 | 1.5s |
| d3 | 271 | 93.7% | 94.1% | 93.3% | 0.935 | 1.6s |
| d4 | 87 | 94.3% | 90.8% | 94.0% | 0.940 | 2.1s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 37 | 94.6% | 64.9% | - | - | 0.5s |
| 101+ | 4 | 50.0% | 50.0% | 50.0% | 0.500 | 4.3s |
| 2-5 | 311 | 93.6% | 93.2% | 93.2% | 0.936 | 1.6s |
| 33-100 | 61 | 91.8% | 90.2% | 91.5% | 0.915 | 2.5s |
| 6-32 | 19 | 73.7% | 89.5% | 50.0% | 0.617 | 1.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.7s |
| colloquial | 69 | 95.7% | 97.1% | 95.4% | 0.962 | 1.5s |
| condition | 32 | 90.6% | 87.5% | 90.3% | 0.903 | 1.8s |
| english | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.8s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 1.8s |
| keyword | 20 | 85.0% | 80.0% | 85.0% | 0.850 | 1.7s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.8s |
| mixed | 13 | 92.3% | 92.3% | 92.3% | 0.923 | 1.7s |
| multi_intent | 9 | 100.0% | 88.9% | 100.0% | 1.000 | 1.6s |
| negation | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.7s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.4s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 1.9s |
| paraphrase | 177 | 94.4% | 93.2% | 93.5% | 0.935 | 1.6s |
| typo | 13 | 92.3% | 92.3% | 92.3% | 0.923 | 1.8s |
| vague | 23 | 56.5% | 73.9% | 56.5% | 0.638 | 1.7s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 45 | 88.9% | 88.9% | 88.1% | 0.893 | 1.6s |
| health | 39 | 97.4% | 100.0% | 97.1% | 0.981 | 1.6s |
| it | 100 | 97.0% | 95.0% | 96.8% | 0.968 | 2.1s |
| life | 40 | 90.0% | 87.5% | 88.9% | 0.889 | 1.8s |
| money | 48 | 93.8% | 95.8% | 93.3% | 0.941 | 1.7s |
| oos | 34 | 94.1% | 61.8% | - | - | 0.5s |
| shop | 40 | 75.0% | 77.5% | 73.0% | 0.743 | 1.5s |
| travel | 43 | 95.3% | 95.3% | 94.7% | 0.947 | 1.5s |
| work | 43 | 90.7% | 93.0% | 90.0% | 0.912 | 1.6s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 95.8% | 96.6% | 95.5% | 0.964 | 1.6s |
| hard | 112 | 92.0% | 82.1% | 90.9% | 0.909 | 1.7s |
| medium | 202 | 90.1% | 90.1% | 89.0% | 0.896 | 1.6s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 292 | 91.4% | 88.4% | 90.4% | 0.909 | 1.6s |
| test | 140 | 93.6% | 92.9% | 93.2% | 0.939 | 1.6s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 91.7% | 98.4% | 91.8% | 6.2% |
| 0.25 | 92.1% | 97.6% | 93.1% | 3.1% |
| 0.30 | 92.1% | 97.3% | 93.3% | 3.1% |
| 0.35 | 91.9% | 96.7% | 93.6% | 3.1% |
| 0.40 | 91.9% | 96.5% | 93.8% | 3.1% |
| 0.50 | 90.7% | 94.0% | 94.8% | 3.1% |
| 0.60 | 89.4% | 90.5% | 96.7% | 1.6% |
| 0.65 | 88.4% | 88.9% | 97.3% | 1.6% |

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
| food | food_cook | 2 |
| food_store_item | food_store_freeze | 2 |
| null | it | 2 |
| null | money | 2 |
| food_tool | food_tool_pan | 1 |
| it_office_excel_func | work | 1 |
| it_pc_peri_monitor | it_pc_win | 1 |
| it_pc_win_keys | it | 1 |
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

### 틀린 케이스 (58건 중 58건)

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
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food_cook · top food_cook#article 0.59 alternative | route deep, wrong top1 |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.47 alternative | wrong top1 |
| S-FOOD-012 | 스테인리스 냄비 바닥에 까맣게 탄 자국은 어떻게 벗겨요? | food_tool · no answer | food_tool_pan · abstained · top food_tool_pan#2 0.04 reference | route deep |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.81 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.88 recommended | route branch, wrong top1 |
| S-HEALTH-017 | 집에서 다쳤을 때 쓸 응급처치를 종류별로 한번 정리해 두고 싶어요 | health_aid | health_aid · top health_aid_injury_sprain#1 0.63 alternative | wrong top1 |
| S-IT-005 | 씨드라이브가 빨강색으로 바꼇는데 어떻게 비워요 | it_pc_win_perf_disk | it_mobile_space · top it_mobile_space#1 0.51 alternative | route branch, wrong top1 |
| S-IT-035 | 듀얼 모니터 쓰는데 작업 표시줄을 한쪽 화면에만 나오게 할 수 있나요? | it_pc_peri_monitor · no answer | it_pc_win · abstained · top it_pc_win_keys#win_p 0.15 reference | route branch |
| S-IT-504 | 클립보드 기록 | it_pc_win_keys | it · top it_pc_win_keys#win_v 0.90 recommended | route shallow |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · abstained · top work_pay_sev#2 0.25 reference | route domain, abstained |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell | life_clean_towel · top life_clean_towel#1 0.56 alternative | route branch, wrong top1 |
| S-LIFE-004 | 드럼 고무패킹 곰팡이 | life_appl_wash_wm_drum_care_gasket | life_clean_mold · abstained · top life_clean_mold#2 0.21 reference | route branch, abstained |
| S-LIFE-008 | 건조기 돌리면 안 되고 그냥 널어서 말려야 하는 옷은 어떤 거예요? | life_appl_wash_dryer_shrink | life_clean · abstained · top life_clean_towel#2 0.20 reference | route branch, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.67 recommended | route shallow |
| S-LIFE-020 | 자취 시작하고 집안일이 처음이라 뭐가 고장 나거나 지저분해지면 어디서부터 봐야 할지 모르겠어요 | life | life_home · top life_home#article 0.85 recommended | route deep, wrong top1 |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec_phish#3 0.08 reference | route domain, abstained |
| S-MONEY-021 | 제 명의 신용카드 관련해서 이것저것 궁금한 게 있는데요 | money_card | money_card · top money_card_lost#4 0.69 recommended | wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.87 recommended | route domain, wrong top1 |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card_limit#2 0.09 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.23 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · top money_save#article 0.56 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health_aid_injury_cut#3 0.08 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_card_lost#4 0.19 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.08 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money_save · abstained · top money_save#article 0.07 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_dryer_shrink#2 0.13 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it · abstained · top it_pc_win_perf_slow#1 0.21 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym_stomach · abstained · top health_sym_stomach#3 0.16 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.11 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_missing#2 0.13 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.35 reference | route domain, wrong top1 |
| S-SHOP-009 | 카드로 산 운동화를 반품했는데 카드사 앱 승인 내역에 아직 취소가 안 보여요. 보통 며칠 걸려요? | shop_ret_refund_card | money_card · abstained · top money_card_limit#2 0.09 reference | route domain, abstained |
| S-SHOP-017 | 온라인으로 산 물건 때문에 문의하려는데 뭐부터 봐야 할지 모르겠어요 | shop | shop_ret · top shop_ret#article 0.79 recommended | route deep, wrong top1 |
| S-SHOP-018 | 받은 물건이 맘에 안 들어서 처리하고 싶은데, 돌려보내는 거랑 바꾸는 거 중에 어떤 선택지가 있는지부터 알려 주세요 | shop_ret | shop_ret_return_mind · top shop_ret_return_mind#1 0.41 alternative | route deep, wrong top1 |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.47 alternative | wrong top1 |
| S-SHOP-024 | 무통장으로 낸 건요? | shop_ret_refund_bank | shop_order_pay · top shop_order_pay#3 0.32 reference | route branch, wrong top1 |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money_tax · abstained · top money_tax#article 0.21 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.53 alternative | route branch, wrong top1 |
| S-TRAVEL-018 | 해외 나가기 전에 서류나 폰 쪽으로 챙겨야 할 게 뭐가 있죠? | travel_prep | travel_prep_data · top travel_prep_data#3 0.57 alternative | route deep, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.74 recommended | wrong top1 |
| S-WORK-008 | 연말정산 공제 얘기 말고요, 매달 월급에서 떼가는 건강보험이랑 소득세 항목이 뭔지 알고 싶어요 | work_pay_slip | money_tax · abstained · top money_tax#article 0.18 reference | route domain, abstained |
| S-WORK-032 | 경조금 받으려면 증빙으로 뭘 올려야 하죠? | work_leave_event | work_pay · abstained · top work_pay_slip#4 0.08 reference | route branch, abstained |
| S-XD-003 | 쇼핑몰 주문을 카드 결제 후 바로 취소했는데 카드 명세서에는 아직 청구돼 있어요 | shop_ret_refund_card | shop_order_cancel · abstained · top shop_order_cancel#3 0.20 reference | route branch, abstained |
| S-XD-014 | 여행용 eSIM을 샀는데 폰에 어떻게 설치해서 써요? | travel_prep_data | it_mobile · abstained · top it_mobile#article 0.10 reference | route domain, abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.61 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.85 recommended | route branch, wrong top1 |
| S-XD-045 | 요리나 식재료 관련해서 도움을 받고 싶어요 | food | food_cook · top food_cook#article 0.59 alternative | route deep, wrong top1 |
| S-XD-047 | 카드로 산 물건 결제를 취소했는데 명세서에 아직 남아 있어요 | shop_ret_refund_card | shop_order_pay · top shop_order_pay#1 0.50 alternative | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.82 recommended | route false_tree, answered unanswerable |

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
| MRR@k | 0.955 [0.933–0.977] (n=337) |
| nDCG@k | 0.942 [0.919–0.964] (n=337) |
| Recall@k | 0.944 [0.921–0.967] (n=337) |
| recommended 정밀도 | 0.981 [0.965–0.996] (n=305) |
| 라우팅이 맞았을 때 Hit@1 | 99.4% [97.8–99.8] (n=323) |
| 라우팅이 맞았을 때 MRR | 0.996 [0.991–1.000] (n=323) |
| 지연 p50 / p90 / p95 / 평균 | 1.2s / 1.9s / 2.2s / 1.3s |
| 라우팅 구간 평균 | 0.8s |
| 랭킹 후보 수 평균 | 16.5 |
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
| 5 | 9 | 415 | 18 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.8s |
| internal | 10 | 90.0% | 100.0% | 90.0% | 0.933 | 1.9s |
| leaf | 327 | 95.4% | 95.7% | 95.4% | 0.956 | 1.2s |
| no_answer | 16 | 100.0% | 100.0% | - | - | 1.2s |
| oos | 32 | 93.8% | 59.4% | - | - | 0.4s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 94.3% | 93.3% | 94.2% | 0.942 | 1.2s |
| single | 283 | 95.8% | 92.9% | 95.7% | 0.961 | 1.2s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 283 | 95.8% | 92.9% | 95.7% | 0.961 | 1.2s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 1.2s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 1.3s |
| disambig | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.2s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.1s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 1.1s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.1s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 357 | 95.2% | 93.0% | 95.0% | 0.952 | 1.2s |
| root | 31 | 96.8% | 93.5% | 100.0% | 1.000 | 0.5s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 10 | 90.0% | 100.0% | 90.0% | 0.933 | 1.9s |
| leaf | 346 | 95.7% | 96.0% | 95.4% | 0.956 | 1.2s |
| none | 32 | 93.8% | 59.4% | - | - | 0.4s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 32 | 93.8% | 59.4% | - | - | 0.4s |
| d1 | 10 | 90.0% | 100.0% | 90.0% | 0.933 | 1.9s |
| d2 | 346 | 95.7% | 96.0% | 95.4% | 0.956 | 1.2s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 35 | 94.3% | 62.9% | - | - | 0.5s |
| 101+ | 4 | 75.0% | 100.0% | 75.0% | 0.833 | 3.4s |
| 2-5 | 293 | 95.9% | 96.2% | 95.7% | 0.959 | 1.2s |
| 33-100 | 56 | 94.6% | 94.6% | 94.5% | 0.945 | 2.0s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.4s |
| colloquial | 66 | 98.5% | 100.0% | 98.4% | 0.992 | 1.2s |
| condition | 32 | 93.8% | 93.8% | 93.5% | 0.935 | 1.3s |
| english | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.5s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 1.1s |
| keyword | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 1.2s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| mixed | 13 | 92.3% | 92.3% | 92.3% | 0.923 | 1.2s |
| multi_intent | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| negation | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 1.3s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.3s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 1.7s |
| paraphrase | 149 | 94.0% | 93.3% | 93.5% | 0.935 | 1.1s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 1.1s |
| vague | 10 | 90.0% | 100.0% | 90.0% | 0.933 | 1.9s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 39 | 92.3% | 94.9% | 92.1% | 0.930 | 1.3s |
| health | 35 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| it | 92 | 96.7% | 96.7% | 96.6% | 0.966 | 1.2s |
| life | 35 | 100.0% | 100.0% | 100.0% | 1.000 | 1.2s |
| money | 43 | 95.3% | 95.3% | 95.1% | 0.951 | 1.2s |
| oos | 32 | 93.8% | 59.4% | - | - | 0.4s |
| shop | 35 | 88.6% | 88.6% | 87.9% | 0.879 | 1.2s |
| travel | 38 | 97.4% | 97.4% | 97.1% | 0.971 | 1.2s |
| work | 39 | 92.3% | 94.9% | 91.9% | 0.932 | 1.2s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 109 | 97.2% | 98.2% | 97.0% | 0.975 | 1.2s |
| hard | 100 | 92.0% | 84.0% | 92.1% | 0.921 | 1.2s |
| medium | 179 | 96.1% | 95.0% | 95.6% | 0.958 | 1.2s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 260 | 95.4% | 92.7% | 95.6% | 0.958 | 1.2s |
| test | 128 | 95.3% | 93.8% | 94.4% | 0.949 | 1.2s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 94.8% | 98.5% | 95.5% | 7.8% |
| 0.25 | 95.1% | 98.5% | 95.8% | 5.9% |
| 0.30 | 95.4% | 98.5% | 96.1% | 3.9% |
| 0.35 | 95.4% | 97.6% | 97.0% | 2.0% |
| 0.40 | 95.4% | 97.3% | 97.3% | 2.0% |
| 0.50 | 94.6% | 96.4% | 97.2% | 2.0% |
| 0.60 | 91.5% | 92.3% | 97.8% | 2.0% |
| 0.65 | 89.9% | 90.2% | 98.0% | 2.0% |

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
| null | money | 3 |
| food_store_item | food_store_freeze | 2 |
| null | it | 2 |
| it_office_excel_func | work | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| it_sec_phish | money_bank_xfer_wrong | 1 |
| money_bank_cert | it_sec_2fa | 1 |
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | food_safe_poison | 1 |
| null | health | 1 |
| null | health_sym_stomach | 1 |
| null | life | 1 |
| null | money_card_lost | 1 |
| null | shop | 1 |
| null | travel | 1 |
| shop_order_pay | shop_order_cancel | 1 |
| shop_order_receipt | money | 1 |
| shop_order_receipt | money_tax_income | 1 |
| shop_ret_refund_card | money | 1 |

### 틀린 케이스 (34건 중 34건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | life | false_tree |
| I-SHOP-007 | 호텔 예약을 취소하면 수수료가 얼마나 나오나요? | null | shop_order_cancel | false_tree |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_order_cancel | false_tree |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food · top food_safe_poison#2 0.63 alternative | wrong top1 |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.80 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.86 recommended | route branch, wrong top1 |
| S-IT-031 | 엄마라면서 폰 액정이 깨졌다고 상품권 좀 사 달라는 문자가 왔는데 이상해요 | it_sec_phish | money_bank_xfer_wrong · abstained · top money_bank_xfer_wrong#2 0.07 reference | route domain, abstained |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · top work_leave_event#1 0.31 reference | route domain, wrong top1 |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec_2fa · abstained · top it_sec_2fa#2 0.04 reference | route domain, abstained |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.86 recommended | route domain, wrong top1 |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · top it_sec_phish#2 0.33 reference | route false_tree, answered unanswerable |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · abstained · top money_tax_yearend#4 0.28 reference | route false_tree |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health_aid_injury_cut#3 0.08 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_tax_yearend#4 0.15 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel · abstained · top travel_local_metro#3 0.13 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money · abstained · top money_loan_credit#1 0.16 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_wm_drum_water_drain#2 0.12 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it · abstained · top it_mobile_lost#2 0.25 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym_stomach · abstained · top health_sym_stomach#3 0.15 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.16 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_addr#2 0.16 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.36 reference | route domain, wrong top1 |
| S-SHOP-009 | 카드로 산 운동화를 반품했는데 카드사 앱 승인 내역에 아직 취소가 안 보여요. 보통 며칠 걸려요? | shop_ret_refund_card | money · abstained · top money_ins#2 0.14 reference | route domain, abstained |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money · abstained · top money_bank_acct_limit#1 0.12 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.58 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.73 recommended | wrong top1 |
| S-WORK-008 | 연말정산 공제 얘기 말고요, 매달 월급에서 떼가는 건강보험이랑 소득세 항목이 뭔지 알고 싶어요 | work_pay_slip | money_tax_income · abstained · top money_tax_income#2 0.08 reference | route domain, abstained |
| S-XD-014 | 여행용 eSIM을 샀는데 폰에 어떻게 설치해서 써요? | travel_prep_data | it · top it#article 0.34 reference | route domain, wrong top1 |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.58 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.83 recommended | route branch, wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.84 recommended | route false_tree, answered unanswerable |
| S-XD-052 | 상한 음식을 먹고 계속 토해요. 어떻게 해야 돼요? | null · no answer | food_safe_poison · abstained · top food_safe_poison#2 0.15 reference | route false_tree |

## jev · flat

### 검색 (357건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 97.5% [95.3–98.7] (n=357) |
|   └ 답할 수 있는 질문 | 97.2% [94.7–98.5] (n=317) |
|   └ 답할 수 없는 질문(보류가 정답) | 100.0% [91.2–100.0] (n=40) |
| 확신 정답률 (top1 정답 + recommended) | 90.2% [86.5–93.0] (n=317) |
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
| recommended 정밀도 | 0.982 [0.967–0.996] (n=291) |
| 라우팅이 맞았을 때 Hit@1 | 99.7% [98.2–99.9] (n=309) |
| 라우팅이 맞았을 때 MRR | 0.998 [0.995–1.000] (n=309) |
| 지연 p50 / p90 / p95 / 평균 | 0.9s / 1.4s / 1.6s / 0.9s |
| 라우팅 구간 평균 | 0.5s |
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
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 0.5s |
| leaf | 317 | 97.2% | 97.5% | 97.2% | 0.973 | 0.9s |
| no_answer | 16 | 100.0% | 93.8% | - | - | 0.8s |
| oos | 21 | 100.0% | 100.0% | - | - | 0.4s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 94.3% | 94.3% | 94.2% | 0.942 | 0.9s |
| single | 252 | 98.8% | 98.8% | 98.6% | 0.988 | 0.9s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 252 | 98.8% | 98.8% | 98.6% | 0.988 | 0.9s |
| coref | 19 | 84.2% | 84.2% | 84.2% | 0.842 | 0.8s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| disambig | 20 | 85.0% | 85.0% | 85.0% | 0.850 | 0.8s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| shift | 14 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 347 | 97.4% | 97.4% | 97.1% | 0.972 | 0.9s |
| root | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.4s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| leaf | 336 | 97.3% | 97.3% | 97.2% | 0.973 | 0.9s |
| none | 21 | 100.0% | 100.0% | - | - | 0.4s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 21 | 100.0% | 100.0% | - | - | 0.4s |
| d1 | 336 | 97.3% | 97.3% | 97.2% | 0.973 | 0.9s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 24 | 100.0% | 100.0% | - | - | 0.4s |
| 2-5 | 283 | 97.2% | 97.2% | 97.0% | 0.972 | 0.9s |
| 33-100 | 50 | 98.0% | 98.0% | 98.0% | 0.980 | 1.5s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.0s |
| colloquial | 66 | 98.5% | 100.0% | 98.4% | 0.992 | 0.9s |
| condition | 31 | 96.8% | 96.8% | 96.8% | 0.968 | 0.9s |
| english | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| keyword | 20 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 1.0s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| multi_intent | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| negation | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 1.1s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.4s |
| oos_near | 11 | 100.0% | 100.0% | - | - | 0.4s |
| paraphrase | 129 | 95.3% | 94.6% | 94.7% | 0.947 | 0.9s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 36 | 97.2% | 97.2% | 97.1% | 0.971 | 1.0s |
| health | 32 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| it | 89 | 98.9% | 98.9% | 98.8% | 0.988 | 0.9s |
| life | 33 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| money | 41 | 95.1% | 95.1% | 94.9% | 0.949 | 0.9s |
| oos | 21 | 100.0% | 100.0% | - | - | 0.4s |
| shop | 32 | 93.8% | 93.8% | 93.3% | 0.933 | 0.9s |
| travel | 36 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| work | 37 | 91.9% | 91.9% | 91.4% | 0.929 | 0.8s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 99 | 99.0% | 100.0% | 98.9% | 0.995 | 0.9s |
| hard | 95 | 91.6% | 90.5% | 89.5% | 0.895 | 0.9s |
| medium | 163 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 237 | 97.9% | 97.5% | 97.7% | 0.977 | 0.9s |
| test | 120 | 96.7% | 97.5% | 96.1% | 0.966 | 0.9s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 97.5% | 100.0% | 97.2% | 0.0% |
| 0.25 | 97.5% | 100.0% | 97.2% | 0.0% |
| 0.30 | 97.5% | 100.0% | 97.2% | 0.0% |
| 0.35 | 97.2% | 99.7% | 97.2% | 0.0% |
| 0.40 | 97.2% | 99.4% | 97.5% | 0.0% |
| 0.50 | 96.4% | 98.1% | 97.7% | 0.0% |
| 0.60 | 93.0% | 94.0% | 98.0% | 0.0% |
| 0.65 | 91.3% | 91.8% | 98.3% | 0.0% |

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
| 지연 p50 (초안+게시+probe) | 1.6s |
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
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.86 recommended | route branch, wrong top1 |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost | it_sec_hacked · top it_sec_hacked#1 0.37 reference | route domain, wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.86 recommended | route domain, wrong top1 |
| S-SHOP-010 | 이 쇼핑몰에서 3개월 할부로 산 가방을 반품하면 이미 나간 할부 수수료도 돌려받아요? | shop_ret_refund_card | money_card_install · top money_card_install#3 0.43 alternative | route domain, wrong top1 |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.59 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.76 recommended | wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#2 0.08 reference | route domain |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.65 alternative | route branch, wrong top1 |
| S-XD-024 | 한도 좀 올리고 싶은데 어떻게 해요? | work_exp_card | money_card_limit · top money_card_limit#1 0.84 recommended | route domain, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.82 recommended | route branch, wrong top1 |

## jev · sparse

### 검색 (432건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 84.5% [80.8–87.6] (n=432) |
|   └ 답할 수 있는 질문 | 89.7% [84.2–93.4] (n=174) |
|   └ 답할 수 없는 질문(보류가 정답) | 81.0% [75.8–85.3] (n=258) |
| 확신 정답률 (top1 정답 + recommended) | 77.6% [70.8–83.1] (n=174) |
| 라우팅 정확도 (정확 일치) | 89.6% [86.3–92.1] (n=432) |
| 계층 F1 (hF1) | 0.926 [0.903–0.949] (n=432) |
| 트리 밖 판정 precision | 100.0% [84.5–100.0] (n=21) |
| 트리 밖 판정 recall | 61.8% [45.0–76.1] (n=34) |
| 트리 밖 판정 F1 | 0.764 |
| 오보류율 (답할 수 있는데 보류) | 6.3% [3.6–11.0] (n=174) |
| 오답변율 (답할 수 없는데 답함) | 19.0% [14.7–24.2] (n=258) |
| Hit@1 | 90.8% [85.6–94.3] (n=174) |
| Hit@3 | 90.8% [85.6–94.3] (n=174) |
| Hit@k (limit) | 91.4% [86.3–94.7] (n=174) |
| MRR@k | 0.909 [0.867–0.952] (n=174) |
| nDCG@k | 0.912 [0.871–0.954] (n=174) |
| Recall@k | 0.902 [0.860–0.945] (n=174) |
| recommended 정밀도 | 0.895 [0.847–0.943] (n=149) |
| 라우팅이 맞았을 때 Hit@1 | 98.1% [94.6–99.4] (n=160) |
| 라우팅이 맞았을 때 MRR | 0.983 [0.963–1.000] (n=160) |
| 지연 p50 / p90 / p95 / 평균 | 1.7s / 2.1s / 2.2s / 1.6s |
| 라우팅 구간 평균 | 1.2s |
| 랭킹 후보 수 평균 | 2.5 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 3.81 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 97.4% (n=351) | 94.7% (n=341) | 92.2% (n=322) | 90.1% (n=81) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 15 | 7 | 9 | 436 | 17 | 4 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dropped | 194 | 75.8% | 92.3% | - | - | 1.7s |
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 1.3s |
| internal | 23 | 69.6% | 73.9% | 69.6% | 0.707 | 1.6s |
| leaf | 151 | 92.7% | 94.7% | 94.0% | 0.940 | 1.7s |
| no_answer | 27 | 100.0% | 88.9% | - | - | 1.5s |
| oos | 34 | 94.1% | 61.8% | - | - | 0.4s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 81.9% | 92.4% | 92.9% | 0.929 | 1.7s |
| single | 327 | 85.3% | 88.7% | 90.2% | 0.903 | 1.6s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 327 | 85.3% | 88.7% | 90.2% | 0.903 | 1.6s |
| coref | 19 | 73.7% | 84.2% | 100.0% | 1.000 | 1.7s |
| correct | 8 | 75.0% | 87.5% | 80.0% | 0.800 | 1.6s |
| disambig | 20 | 80.0% | 90.0% | 90.0% | 0.900 | 1.7s |
| distract | 9 | 88.9% | 100.0% | 100.0% | 1.000 | 1.6s |
| ellipsis | 9 | 66.7% | 88.9% | 75.0% | 0.750 | 1.7s |
| long | 8 | 75.0% | 100.0% | 100.0% | 1.000 | 1.7s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 1.7s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 1.7s |
| slot | 9 | 77.8% | 100.0% | 100.0% | 1.000 | 1.7s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 372 | 84.1% | 89.0% | 90.3% | 0.905 | 1.7s |
| root | 43 | 88.4% | 90.7% | 100.0% | 1.000 | 0.7s |
| start | 17 | 82.4% | 100.0% | 100.0% | 1.000 | 0.8s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 79.4% | 79.4% | 69.6% | 0.707 | 1.5s |
| leaf | 364 | 84.1% | 93.1% | 94.0% | 0.940 | 1.7s |
| none | 34 | 94.1% | 61.8% | - | - | 0.4s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 34 | 94.1% | 61.8% | - | - | 0.4s |
| d1 | 10 | 60.0% | 60.0% | 60.0% | 0.600 | 1.5s |
| d2 | 30 | 90.0% | 90.0% | 80.0% | 0.817 | 1.5s |
| d3 | 271 | 83.0% | 93.4% | 94.7% | 0.947 | 1.6s |
| d4 | 87 | 86.2% | 92.0% | 91.4% | 0.914 | 1.9s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 37 | 94.6% | 64.9% | - | - | 0.5s |
| 1 | 361 | 83.9% | 93.1% | 94.0% | 0.940 | 1.7s |
| 2-5 | 16 | 93.8% | 87.5% | 85.7% | 0.857 | 1.5s |
| 33-100 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 1.7s |
| 6-32 | 16 | 62.5% | 68.8% | 57.1% | 0.589 | 1.6s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 87.5% | 87.5% | - | - | 1.6s |
| colloquial | 69 | 84.1% | 97.1% | 96.7% | 0.967 | 1.7s |
| condition | 32 | 78.1% | 87.5% | 92.9% | 0.929 | 1.8s |
| english | 12 | 91.7% | 100.0% | 100.0% | 1.000 | 1.8s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 1.8s |
| keyword | 20 | 80.0% | 85.0% | 90.0% | 0.900 | 1.7s |
| long | 12 | 91.7% | 100.0% | 100.0% | 1.000 | 1.7s |
| mixed | 13 | 92.3% | 92.3% | 85.7% | 0.857 | 1.8s |
| multi_intent | 9 | 66.7% | 88.9% | 77.8% | 0.778 | 1.8s |
| negation | 20 | 75.0% | 90.0% | 87.5% | 0.875 | 1.9s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 0.4s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 1.7s |
| paraphrase | 177 | 86.4% | 92.1% | 96.1% | 0.961 | 1.6s |
| typo | 13 | 92.3% | 100.0% | 100.0% | 1.000 | 1.8s |
| vague | 23 | 69.6% | 73.9% | 69.6% | 0.707 | 1.6s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 45 | 80.0% | 88.9% | 86.7% | 0.867 | 1.6s |
| health | 39 | 89.7% | 100.0% | 93.8% | 0.953 | 1.6s |
| it | 100 | 90.0% | 95.0% | 95.7% | 0.957 | 1.9s |
| life | 40 | 77.5% | 87.5% | 86.4% | 0.864 | 1.8s |
| money | 48 | 75.0% | 95.8% | 95.8% | 0.958 | 1.5s |
| oos | 34 | 94.1% | 61.8% | - | - | 0.4s |
| shop | 40 | 75.0% | 77.5% | 66.7% | 0.667 | 1.7s |
| travel | 43 | 86.0% | 95.3% | 93.3% | 0.933 | 1.7s |
| work | 43 | 88.4% | 90.7% | 100.0% | 1.000 | 1.6s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 91.5% | 95.8% | 98.0% | 0.980 | 1.5s |
| hard | 112 | 80.4% | 81.2% | 93.8% | 0.938 | 1.7s |
| medium | 202 | 82.7% | 90.6% | 85.7% | 0.860 | 1.7s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 292 | 83.9% | 88.4% | 91.9% | 0.921 | 1.7s |
| test | 140 | 85.7% | 92.1% | 88.2% | 0.882 | 1.6s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 81.2% | 95.4% | 68.4% | 25.2% |
| 0.25 | 83.6% | 95.4% | 71.5% | 21.3% |
| 0.30 | 84.5% | 93.7% | 73.6% | 19.0% |
| 0.35 | 87.5% | 92.5% | 79.0% | 13.2% |
| 0.40 | 88.7% | 90.8% | 82.2% | 10.5% |
| 0.50 | 89.8% | 88.5% | 86.1% | 7.4% |
| 0.60 | 89.1% | 82.2% | 89.7% | 5.0% |
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
| 지연 p50 (초안+게시+probe) | 2.9s |
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
| food_tool | food_tool_pan | 1 |
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
| money_tax_yearend | work_exp_claim | 1 |
| null | food | 1 |
| null | health | 1 |
| null | health_sym_stomach | 1 |

### 틀린 케이스 (98건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-IT-003 | How do I turn on Bluetooth on my Windows laptop when the toggle is missing? | it_pc_peri_bt | it_pc_win | branch |
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-SHOP-007 | 호텔 예약을 취소하면 수수료가 얼마나 나오나요? | null | shop_order_cancel | false_tree |
| I-WORK-001 | 사원증을 잃어버렸는데 재발급은 어떻게 받나요? | work_hr_onboard | work_hr | shallow |
| I-XD-002 | 체크카드로 결제한 주문을 취소하면 환불은 며칠 걸리나요? | shop_ret_refund_card | shop_order_cancel | branch |
| I-XD-006 | 숙박 예약 사이트에서 잡은 호텔을 취소하면 환불은 누구에게 요청하나요? | null | shop_ret | false_tree |
| S-FOOD-001 | 밥솥으로 밥을 했는데 밥알 가운데가 생쌀처럼 씹혀요 | food_cook_base_rice · no answer | food_cook_base_rice · top food_cook_base_rice#1 0.35 reference | answered unanswerable |
| S-FOOD-007 | 배탈 난 뒤 관리 말고요, 여름에 식중독 안 걸리게 미리 조심할 것들 알려주세요 | food_safe_poison · no answer | food_safe_poison · top food_safe_poison#1 0.46 alternative | answered unanswerable |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food_cook · top food_cook#article 0.55 alternative | route deep, wrong top1 |
| S-FOOD-012 | 스테인리스 냄비 바닥에 까맣게 탄 자국은 어떻게 벗겨요? | food_tool · no answer | food_tool_pan · abstained · top food_tool_pan#1 0.02 reference | route deep |
| S-FOOD-014 | 그거 깔끔하게 까는 요령 좀요 | food_cook_side_egg · no answer | food_cook_side_egg · top food_cook_side_egg#1 0.31 reference | answered unanswerable |
| S-FOOD-015 | 그럼 소면은요? | food_cook_base_noodle · no answer | food_cook_base_noodle · top food_cook_base_noodle#1 0.30 reference | answered unanswerable |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item · no answer | food_store_freeze · top food_store_freeze#1 0.82 recommended | route branch, answered unanswerable |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item · no answer | food_store_freeze · top food_store_freeze#1 0.88 recommended | route branch, answered unanswerable |
| S-HEALTH-002 | 진통제 거의 매일 먹는데 이거 괜찮은거임? 안 먹으면 머리 깨질 것 같음 | health_sym_head · no answer | health_sym_head · top health_sym_head#1 0.31 reference | answered unanswerable |
| S-HEALTH-007 | My 8-month-old baby is choking on something and can't breathe. What should I do  | health_aid_emerg_choke · no answer | health_aid_emerg_choke · top health_aid_emerg_choke#1 0.39 reference | answered unanswerable |
| S-HEALTH-009 | 달리기 말고 헬스장에서 근력이랑 유산소를 섞어서 하려는데 뭐부터 해야 돼요? | health_fit_start · no answer | health_fit_start · top health_fit_start#1 0.42 alternative | answered unanswerable |
| S-HEALTH-017 | 집에서 다쳤을 때 쓸 응급처치를 종류별로 한번 정리해 두고 싶어요 | health_aid | health_aid · top health_aid_injury_sprain#1 0.68 recommended | wrong top1 |
| S-IT-028 | 사이트마다 비밀번호를 다르게 쓰면 다 기억할 수가 없어요. | it_sec_pw · no answer | it_sec_pw · top it_sec_pw#1 0.41 alternative | answered unanswerable |
| S-IT-031 | 엄마라면서 폰 액정이 깨졌다고 상품권 좀 사 달라는 문자가 왔는데 이상해요 | it_sec_phish · no answer | money · abstained · top money_bank_xfer_wrong#1 0.14 reference | route domain |
| S-IT-035 | 듀얼 모니터 쓰는데 작업 표시줄을 한쪽 화면에만 나오게 할 수 있나요? | it_pc_peri_monitor · no answer | it_pc_win · abstained · top it_pc_win#article 0.07 reference | route branch |
| S-IT-041 | 같은 브랜드로 가요. 옮기기 전에 백업은 어떻게 해 둬요? | it_mobile_move_android · no answer | it_mobile_move_android · top it_mobile_move_android#1 0.69 recommended | answered unanswerable |
| S-IT-043 | 아 잘못 말했어요, 새로 산 게 아이폰이 아니라 갤럭시예요. 그럼 어떻게 옮겨요? | it_mobile_move_cross · no answer | it_mobile_move_cross · top it_mobile_move_cross#1 0.34 reference | answered unanswerable |
| S-IT-051 | 누가 제 이메일 계정 비번이랑 복구 메일까지 바꿔 놔서 들어갈 수가 없어요 | it_sec_hacked · no answer | it_sec_hacked · top it_sec_hacked#1 0.32 reference | answered unanswerable |
| S-IT-503 | 전체 화면 말고 지금 띄워 놓은 창 하나만 찍고 싶어요 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.76 recommended | answered unanswerable |
| S-IT-504 | 클립보드 기록 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.81 recommended | answered unanswerable |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func · no answer | work · abstained · top work_pay_ot#1 0.21 reference | route domain |
| S-IT-531 | 두 번째, 세 번째로 큰 값을 구하는 함수는? | it_office_excel_func · no answer | it · abstained · top it_sec_phish#1 0.09 reference | route shallow |
| S-LIFE-001 | 드럼세탁기 코스가 다 끝났는데 옷이 물에 푹 젖은 채로 나와요. 탈수가 안 된 것 같아요 | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.79 recommended | answered unanswerable |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell · no answer | life_clean_towel · top life_clean_towel#1 0.53 alternative | route branch, answered unanswerable |
| S-LIFE-004 | 드럼 고무패킹 곰팡이 | life_appl_wash_wm_drum_care_gasket | life_clean_mold · abstained · top life_clean_mold#1 0.17 reference | route branch, abstained |
| S-LIFE-006 | 통돌이 세탁기로 빨래하면 검은 찌꺼기가 뭍어나와여 청소 어케해요 | life_appl_wash_wm_top_clean · no answer | life_appl_wash_wm_top_clean · top life_appl_wash_wm_top_clean#1 0.82 recommended | answered unanswerable |
| S-LIFE-008 | 건조기 돌리면 안 되고 그냥 널어서 말려야 하는 옷은 어떤 거예요? | life_appl_wash_dryer_shrink | life_clean · abstained · top life_clean_towel#1 0.06 reference | route branch, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.53 alternative | route shallow |
| S-LIFE-020 | 자취 시작하고 집안일이 처음이라 뭐가 고장 나거나 지저분해지면 어디서부터 봐야 할지 모르겠어요 | life | life_home · top life_home#article 0.85 recommended | route deep, wrong top1 |
| S-LIFE-030 | 빨래를 조금만 넣었을 때 탈수가 잘 안 되는데 왜 그래요? | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.81 recommended | answered unanswerable |
| S-LIFE-035 | 그럼 아까 그거 빨고 나서 방에서 말릴 때 냄새 안 나게 하려면 어떻게 해요? | life_clean_towel · no answer | life_clean_towel · top life_clean_towel#1 0.57 alternative | answered unanswerable |
| S-LIFE-036 | 겨울만 되면 거실 창틀이 곰팡이로 까매져요 | life_clean_mold · no answer | life_clean_mold · top life_clean_mold#1 0.41 alternative | answered unanswerable |
| S-MONEY-002 | 통장 만든 지 일주일 됐는데 하루에 30만 원까지만 보내져요. 앱에서 이체 한도 올리기를 눌러 봐도 안 되는데 이거 계좌 자체가 묶인 건가요? | money_bank_acct_limit · no answer | money_bank_acct_limit · top money_bank_acct_limit#1 0.33 reference | answered unanswerable |
| S-MONEY-005 | 인터넷뱅킹으로 계좌이체할 때 하루 한도를 늘리려면 OTP가 꼭 있어야 하나요? | money_bank_xfer_cap · no answer | money_bank_xfer_cap · top money_bank_xfer_cap#1 0.68 recommended | answered unanswerable |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec#article 0.06 reference | route domain, abstained |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost · no answer | money_card_lost · top money_card_lost#1 0.48 alternative | answered unanswerable |
| S-MONEY-012 | 온라인 쇼핑몰에서 결제하는데 한도 초과라고 승인이 안 나요. 쇼핑몰 오류가 아니라 제 신용카드 한도가 찬 거라는데 지금 바로 결제하려면 어떻게  | money_card_limit · no answer | money_card_limit · top money_card_limit#1 0.36 reference | answered unanswerable |
| S-MONEY-017 | 직장인인데 주말마다 과외해서 번 돈이 좀 있어요. 이것도 회사 연말정산으로 끝나나요, 아니면 제가 따로 신고해야 하나요? | money_tax_income · no answer | money_tax_income · top money_tax_income#1 0.45 alternative | answered unanswerable |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend · no answer | work_exp_claim · abstained · top work_exp_claim#1 0.15 reference | route domain |
| S-MONEY-027 | 마통은요? | money_loan_credit · no answer | money_loan_credit · top money_loan_credit#1 0.53 alternative | answered unanswerable |
| S-MONEY-033 | 한도를 당장 다시 살리는 방법은 없어요? | money_card_limit · no answer | money_card_limit · top money_card_limit#1 0.39 reference | answered unanswerable |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit · no answer | money_bank_acct_limit · top money_bank_acct_limit#1 0.55 alternative | answered unanswerable |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card#article 0.05 reference | route false_tree |
| S-MONEY-039 | 휴면 예금이 서민금융진흥원으로 넘어갔다는데 그럼 이제 못 돌려받는 거예요? | money_bank_acct_dormant · no answer | money_bank_acct_dormant · top money_bank_acct_dormant#1 0.33 reference | answered unanswerable |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it · abstained · top it_pc_win_power_boot#1 0.12 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money · top money_save#article 0.63 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health#article 0.07 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_card_install#1 0.10 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.06 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money_save · abstained · top money_save#article 0.05 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_dryer_slow#1 0.10 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it · abstained · top it_net_wifi_join#1 0.14 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym_stomach · abstained · top health_sym_stomach#1 0.06 reference | route false_tree |

## jev_llm · deep

### 검색 (433건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 95.2% [92.7–96.8] (n=433) |
|   └ 답할 수 있는 질문 | 94.6% [91.8–96.5] (n=369) |
|   └ 답할 수 없는 질문(보류가 정답) | 98.4% [91.7–99.7] (n=64) |
| 확신 정답률 (top1 정답 + recommended) | 89.2% [85.6–91.9] (n=369) |
| 라우팅 정확도 (정확 일치) | 88.7% [85.4–91.3] (n=433) |
| 계층 F1 (hF1) | 0.927 [0.905–0.949] (n=433) |
| 트리 밖 판정 precision | 100.0% [84.5–100.0] (n=21) |
| 트리 밖 판정 recall | 61.8% [45.0–76.1] (n=34) |
| 트리 밖 판정 F1 | 0.764 |
| 오보류율 (답할 수 있는데 보류) | 2.2% [1.1–4.2] (n=369) |
| 오답변율 (답할 수 없는데 답함) | 1.6% [0.3–8.3] (n=64) |
| Hit@1 | 94.6% [91.8–96.5] (n=369) |
| Hit@3 | 95.4% [92.7–97.1] (n=369) |
| Hit@k (limit) | 95.9% [93.4–97.5] (n=369) |
| MRR@k | 0.951 [0.930–0.972] (n=369) |
| nDCG@k | 0.943 [0.921–0.964] (n=369) |
| Recall@k | 0.949 [0.928–0.971] (n=369) |
| recommended 정밀도 | 0.974 [0.958–0.989] (n=333) |
| 라우팅이 맞았을 때 Hit@1 | 98.8% [97.0–99.5] (n=333) |
| 라우팅이 맞았을 때 MRR | 0.992 [0.985–1.000] (n=333) |
| 지연 p50 / p90 / p95 / 평균 | 15.1s / 40.2s / 47.0s / 18.7s |
| 라우팅 구간 평균 | 18.2s |
| 랭킹 후보 수 평균 | 18.6 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.49 |
| LLM 라우팅 호출 수 평균 | 1.83 |
| LLM 실패 → Jev 빔 대체율 | 4.4% [2.8–6.8] (n=433) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 | d5 | d6 | d7 |
|---|---:|---:|---:|---:|---:|---:|---:|
| 일치율 | 98.0% (n=352) | 94.2% (n=342) | 90.4% (n=323) | 87.7% (n=146) | 100.0% (n=20) | 100.0% (n=10) | 100.0% (n=8) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_tree | shallow |
|---:|---:|---:|---:|---:|
| 8 | 7 | 450 | 15 | 21 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 22.6s |
| internal | 24 | 87.5% | 100.0% | 87.5% | 0.917 | 13.6s |
| leaf | 345 | 95.1% | 89.6% | 95.1% | 0.954 | 16.0s |
| no_answer | 27 | 100.0% | 100.0% | - | - | 10.2s |
| oos | 34 | 97.1% | 61.8% | - | - | 10.1s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 94.3% | 87.6% | 94.2% | 0.942 | 14.8s |
| single | 328 | 95.4% | 89.0% | 94.7% | 0.955 | 15.5s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 328 | 95.4% | 89.0% | 94.7% | 0.955 | 15.5s |
| coref | 19 | 94.7% | 89.5% | 94.7% | 0.947 | 17.3s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 16.5s |
| disambig | 20 | 90.0% | 85.0% | 90.0% | 0.900 | 12.2s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 16.7s |
| ellipsis | 9 | 100.0% | 77.8% | 100.0% | 1.000 | 15.6s |
| long | 8 | 87.5% | 75.0% | 87.5% | 0.875 | 30.8s |
| refine | 9 | 88.9% | 77.8% | 88.9% | 0.889 | 9.4s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 13.2s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 19.8s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 373 | 94.9% | 87.7% | 94.6% | 0.952 | 15.8s |
| root | 43 | 95.3% | 93.0% | 92.9% | 0.929 | 10.3s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 9.4s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 35 | 91.4% | 100.0% | 87.5% | 0.917 | 10.8s |
| leaf | 364 | 95.3% | 90.1% | 95.1% | 0.954 | 15.9s |
| none | 34 | 97.1% | 61.8% | - | - | 10.1s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 34 | 97.1% | 61.8% | - | - | 10.1s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 13.6s |
| d2 | 30 | 90.0% | 100.0% | 84.2% | 0.895 | 9.4s |
| d3 | 199 | 95.0% | 91.0% | 94.6% | 0.946 | 15.5s |
| d4 | 140 | 95.0% | 87.1% | 94.9% | 0.956 | 16.2s |
| d5 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 27.4s |
| d6 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 30.9s |
| d7 | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 4.5s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 37 | 97.3% | 64.9% | - | - | 11.9s |
| 101+ | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 16.5s |
| 2-5 | 311 | 95.8% | 92.6% | 95.6% | 0.960 | 14.4s |
| 33-100 | 61 | 91.8% | 78.7% | 91.5% | 0.915 | 22.7s |
| 6-32 | 20 | 90.0% | 100.0% | 81.8% | 0.886 | 9.2s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 87.5% | 100.0% | 1.000 | 14.3s |
| colloquial | 69 | 97.1% | 97.1% | 96.9% | 0.977 | 16.5s |
| condition | 32 | 96.9% | 90.6% | 96.8% | 0.968 | 13.6s |
| english | 12 | 100.0% | 66.7% | 100.0% | 1.000 | 18.7s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 5.5s |
| keyword | 20 | 80.0% | 80.0% | 80.0% | 0.800 | 18.8s |
| long | 12 | 100.0% | 91.7% | 100.0% | 1.000 | 16.4s |
| mixed | 13 | 92.3% | 84.6% | 92.3% | 0.923 | 12.6s |
| multi_intent | 9 | 88.9% | 55.6% | 88.9% | 0.944 | 19.4s |
| negation | 20 | 100.0% | 90.0% | 100.0% | 1.000 | 23.1s |
| oos_far | 9 | 100.0% | 88.9% | - | - | 9.2s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 15.1s |
| paraphrase | 177 | 95.5% | 92.7% | 94.2% | 0.942 | 14.4s |
| typo | 13 | 100.0% | 92.3% | 100.0% | 1.000 | 20.1s |
| vague | 24 | 87.5% | 100.0% | 87.5% | 0.917 | 13.6s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 45 | 93.3% | 95.6% | 92.9% | 0.935 | 20.7s |
| health | 39 | 100.0% | 97.4% | 100.0% | 1.000 | 15.2s |
| it | 100 | 96.0% | 86.0% | 95.8% | 0.958 | 17.6s |
| life | 41 | 95.1% | 92.7% | 94.6% | 0.946 | 14.8s |
| money | 48 | 93.8% | 87.5% | 93.3% | 0.939 | 13.4s |
| oos | 34 | 97.1% | 61.8% | - | - | 10.1s |
| shop | 40 | 87.5% | 85.0% | 86.5% | 0.878 | 16.4s |
| travel | 43 | 100.0% | 97.7% | 100.0% | 1.000 | 14.4s |
| work | 43 | 93.0% | 93.0% | 92.5% | 0.950 | 14.5s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 95.8% | 93.2% | 95.5% | 0.964 | 15.5s |
| hard | 113 | 94.7% | 81.4% | 93.6% | 0.942 | 14.4s |
| medium | 202 | 95.0% | 90.1% | 94.5% | 0.948 | 15.8s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 293 | 95.2% | 88.1% | 94.4% | 0.949 | 14.9s |
| test | 140 | 95.0% | 90.0% | 94.9% | 0.956 | 15.7s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 94.2% | 98.6% | 94.6% | 7.8% |
| 0.25 | 95.2% | 98.4% | 95.9% | 1.6% |
| 0.30 | 95.2% | 97.8% | 96.4% | 1.6% |
| 0.35 | 94.9% | 97.3% | 96.7% | 1.6% |
| 0.40 | 94.9% | 97.0% | 96.9% | 1.6% |
| 0.50 | 94.5% | 95.7% | 97.7% | 1.6% |
| 0.60 | 92.6% | 92.7% | 98.5% | 0.0% |
| 0.65 | 90.8% | 90.2% | 98.8% | 0.0% |

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
| 게시 후 probe 검색 top-k 포함 | 82.6% [62.9–93.0] (n=23) |
| 게시 후 probe 검색 top1 답변 | 82.6% [62.9–93.0] (n=23) |
| 지연 p50 (초안+게시+probe) | 38.7s |
| LLM 실패 → Jev 빔 대체율 | 1.7% [0.3–9.1] (n=58) |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| it_office_excel_func | it | 3 |
| food_store_item | food_store_freeze | 2 |
| it_office_excel_func | it_office_excel | 2 |
| it_pc_win_keys | it | 2 |
| null | money | 2 |
| health_sym_stomach | health_sym | 1 |
| it_office_excel_func | it_pc | 1 |
| it_office_excel_func | work | 1 |
| it_office_excel_func | work_pay | 1 |
| it_pc_peri_monitor | it | 1 |
| it_pc_peri_monitor | it_pc_peri | 1 |
| it_pc_win_keys | it_pc_win | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| life_clean_grease | food_tool | 1 |
| life_clean_stain | life_appl_wash | 1 |
| money_bank_acct_limit | work_hr_cert | 1 |
| money_bank_cert | it_sec | 1 |
| money_bank_xfer_cap | money_bank | 1 |
| money_card_lost | money_card | 1 |

### 틀린 케이스 (56건 중 56건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-HEALTH-002 | 헬스 하고 다음 날 근육통이 심하면 운동 쉬어야 됨? | health_fit_start | health_fit_start | exact |
| I-OOS-001 | 내일 서울에 비가 오나요? | null | travel_local | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it_office_excel_func | false_tree |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.43 alternative | wrong top1 |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.78 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.88 recommended | route branch, wrong top1 |
| S-IT-009 | I plugged an external monitor into my laptop with HDMI but it shows nothing | it_pc_peri_monitor | it · top it_pc_peri_monitor#1 0.96 recommended | route shallow |
| S-IT-046 | 아까 그 순서 문제 다시요. 마우스를 오른쪽 끝으로 밀면 왼쪽 화면에서 튀어나와요 | it_pc_peri_monitor | it_pc_peri · top it_pc_peri_monitor#2 0.94 recommended | route shallow |
| S-IT-505 | windows shortcut to create a new virtual desktop | it_pc_win_keys | it · top it_pc_win_keys#win_ctrl_d 0.92 recommended | route shallow |
| S-IT-507 | 작업관리자 바로 뜨게하는 단추키 머에요 | it_pc_win_keys | it_pc_win · top it_pc_win_keys#ctrl_shift_esc 0.96 recommended | route shallow |
| S-IT-516 | 그럼 녹화는요? | it_pc_win_keys | it · top it_pc_win_keys#win_alt_r 0.93 recommended | route shallow |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · top work_leave_annual_left#1 0.33 reference | route domain, wrong top1 |
| S-IT-520 | VLOOKUP 말고요, 찾는 열이 가져올 열보다 오른쪽에 있을 때는 어떻게 해요? | it_office_excel_func | it · top it_office_excel_func#index_match 0.85 recommended | route shallow |
| S-IT-521 | 근무일수 계산 함수 | it_office_excel_func | work_pay · top work_pay_slip#2 0.40 alternative | route domain, wrong top1 |
| S-IT-522 | excel formula to add up values that meet several criteria at once | it_office_excel_func | it · top it_office_excel_func#sumifs 0.94 recommended | route shallow |
| S-IT-525 | lookup 했는데 값이 없으면 #N/A 대신 blank나 '없음'으로 보이게 하는 function 있어요? | it_office_excel_func | it_office_excel · top it_office_excel_func#ifna 0.94 recommended | route shallow |
| S-IT-526 | 셀에 있는 이메일 주소에서 골뱅이 앞 아이디 부분만 떼어 내고 싶어요 | it_office_excel_func | it_pc · abstained · top it_pc_win_keys#alt_space 0.16 reference | route branch, abstained |
| S-IT-527 | 매달 거래처별 매출 파일을 받는데 거래처 이름이 수백 줄에 중복돼서 들어가 있어요. 보고서 첫 장에 거래처 목록을 한 번씩만 깔끔하게 뽑아 두고 | it_office_excel_func | it_office_excel · top it_office_excel_func#unique 0.92 recommended | route shallow |
| S-IT-531 | 두 번째, 세 번째로 큰 값을 구하는 함수는? | it_office_excel_func | it · top it_office_excel_func#large 0.95 recommended | route shallow |
| S-LIFE-015 | 가스레인지 삼발이 찌든 기름때 | life_clean_grease | food_tool · abstained · top food_tool#article 0.27 reference | route domain, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.67 recommended | route shallow |
| S-LIFE-029 | 피요. 이미 말라서 딱딱하게 굳었어요 | life_clean_stain | life_appl_wash · abstained · top life_appl_wash_wm_drum_care#article 0.09 reference | route branch, abstained |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec_hacked#3 0.08 reference | route domain, abstained |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost | money_card · top money_card_lost#4 0.96 recommended | route shallow |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money_bank · top money_bank_xfer_cap#1 0.69 recommended | route shallow |
| S-MONEY-021 | 제 명의 신용카드 관련해서 이것저것 궁금한 게 있는데요 | money_card | money_card · top money_card_lost#4 0.68 recommended | wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | money_tax · top money_tax_yearend#4 0.55 alternative | route shallow |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit | work_hr_cert · abstained · top work_hr_cert#2 0.22 reference | route domain, abstained |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card_limit#2 0.09 reference | route false_tree |
| S-OOS-001 | 내일 서울 날씨 어때요? 우산 챙겨야 하나 | null · no answer | travel · abstained · top travel_stay_cancel#2 0.18 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it_office_excel_func · abstained · top it_office_excel_func#ifs 0.23 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money_save · top money_save#article 0.53 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health_aid_emerg#article 0.09 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_loan_credit#2 0.21 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.08 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money · abstained · top money_card_lost#4 0.18 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_wm_drum_care_smell#1 0.16 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it_sec · abstained · top it_sec_phish#1 0.09 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym · abstained · top health_sym#article 0.20 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.12 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | it · abstained · top it_pc_win_keys#alt_space 0.21 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.37 reference | route domain, wrong top1 |
| S-SHOP-015 | 배송지 주소를 잘못 넣은 것도 고쳐야 하고, 무통장 입금은 언제까지 해야 하는지도 알려 주세요 | shop_ship_addr (+1) | shop · top shop_ship_addr#1 0.80 recommended | route shallow |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.48 alternative | wrong top1 |
| S-SHOP-025 | 벌써 택배사로 넘어갔다는데 받는 사람 전화번호를 잘못 적었어요 | shop_ship_addr | shop_ship · top shop_ship_addr#2 0.91 recommended | route shallow |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money_tax · abstained · top money_tax#article 0.19 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.50 alternative | route branch, wrong top1 |
| S-TRAVEL-024 | 영국은요? | travel_prep_visa | travel_prep · top travel_prep_visa#3 0.64 alternative | route shallow |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.76 recommended | wrong top1 |
| S-WORK-009 | Do I need my manager's approval before working late on a weekday to get overtime | work_pay_ot | work · top work_pay_ot#1 0.95 recommended | route shallow |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work · top work#article 0.64 alternative | route shallow, wrong top1 |
| S-WORK-032 | 경조금 받으려면 증빙으로 뭘 올려야 하죠? | work_leave_event | work_pay · abstained · top work_pay_ot#1 0.06 reference | route branch, abstained |
| S-XD-006 | 날짜 지난 우유를 모르고 마셨는데 배가 살살 아프고 계속 토할 것 같아요 | health_sym_stomach | health_sym · top health_sym_stomach#3 0.91 recommended | route shallow |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.59 alternative | route branch, wrong top1 |
| S-XD-027 | 환불은 언제 들어와요? | money_tax_yearend | money · top money_tax_yearend#2 0.94 recommended | route shallow |
| S-XD-047 | 카드로 산 물건 결제를 취소했는데 명세서에 아직 남아 있어요 | shop_ret_refund_card | shop_order_cancel · abstained · top shop_order_cancel#3 0.26 reference | route branch, abstained |

## jev_llm · standard

### 검색 (432건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 94.0% [91.3–95.9] (n=432) |
|   └ 답할 수 있는 질문 | 93.2% [90.2–95.4] (n=368) |
|   └ 답할 수 없는 질문(보류가 정답) | 98.4% [91.7–99.7] (n=64) |
| 확신 정답률 (top1 정답 + recommended) | 87.8% [84.0–90.7] (n=368) |
| 라우팅 정확도 (정확 일치) | 88.2% [84.8–90.9] (n=432) |
| 계층 F1 (hF1) | 0.920 [0.896–0.943] (n=432) |
| 트리 밖 판정 precision | 91.3% [73.2–97.6] (n=23) |
| 트리 밖 판정 recall | 61.8% [45.0–76.1] (n=34) |
| 트리 밖 판정 F1 | 0.737 |
| 오보류율 (답할 수 있는데 보류) | 3.0% [1.7–5.3] (n=368) |
| 오답변율 (답할 수 없는데 답함) | 1.6% [0.3–8.3] (n=64) |
| Hit@1 | 93.2% [90.2–95.4] (n=368) |
| Hit@3 | 94.3% [91.4–96.2] (n=368) |
| Hit@k (limit) | 94.8% [92.1–96.7] (n=368) |
| MRR@k | 0.938 [0.914–0.962] (n=368) |
| nDCG@k | 0.930 [0.907–0.954] (n=368) |
| Recall@k | 0.938 [0.915–0.962] (n=368) |
| recommended 정밀도 | 0.972 [0.956–0.988] (n=328) |
| 라우팅이 맞았을 때 Hit@1 | 98.2% [96.1–99.2] (n=330) |
| 라우팅이 맞았을 때 MRR | 0.989 [0.979–0.998] (n=330) |
| 지연 p50 / p90 / p95 / 평균 | 14.6s / 37.6s / 44.8s / 17.5s |
| 라우팅 구간 평균 | 17.0s |
| 랭킹 후보 수 평균 | 16.7 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.33 |
| LLM 라우팅 호출 수 평균 | 1.84 |
| LLM 실패 → Jev 빔 대체율 | 1.4% [0.6–3.0] (n=432) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 96.6% (n=351) | 93.3% (n=341) | 89.4% (n=322) | 75.3% (n=81) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 7 | 10 | 445 | 2 | 17 | 19 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 23.7s |
| internal | 23 | 78.3% | 100.0% | 78.3% | 0.858 | 13.5s |
| leaf | 345 | 94.2% | 89.0% | 94.2% | 0.943 | 15.2s |
| no_answer | 27 | 100.0% | 100.0% | - | - | 14.4s |
| oos | 34 | 97.1% | 61.8% | - | - | 8.4s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 93.3% | 88.6% | 93.2% | 0.932 | 15.8s |
| single | 327 | 94.2% | 88.1% | 93.2% | 0.940 | 14.3s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 327 | 94.2% | 88.1% | 93.2% | 0.940 | 14.3s |
| coref | 19 | 89.5% | 89.5% | 89.5% | 0.895 | 14.6s |
| correct | 8 | 87.5% | 87.5% | 87.5% | 0.875 | 28.3s |
| disambig | 20 | 90.0% | 85.0% | 90.0% | 0.900 | 17.7s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 14.4s |
| ellipsis | 9 | 88.9% | 77.8% | 88.9% | 0.889 | 14.3s |
| long | 8 | 87.5% | 75.0% | 87.5% | 0.875 | 28.1s |
| refine | 9 | 100.0% | 88.9% | 100.0% | 1.000 | 33.8s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 13.6s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 17.9s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 372 | 93.5% | 87.1% | 93.1% | 0.937 | 15.9s |
| root | 43 | 95.3% | 93.0% | 92.9% | 0.929 | 4.2s |
| start | 17 | 100.0% | 100.0% | 100.0% | 1.000 | 4.1s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 85.3% | 100.0% | 78.3% | 0.858 | 12.2s |
| leaf | 364 | 94.5% | 89.6% | 94.2% | 0.943 | 15.4s |
| none | 34 | 97.1% | 61.8% | - | - | 8.4s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 34 | 97.1% | 61.8% | - | - | 8.4s |
| d1 | 10 | 90.0% | 100.0% | 90.0% | 0.950 | 8.3s |
| d2 | 30 | 86.7% | 100.0% | 78.9% | 0.854 | 14.4s |
| d3 | 271 | 94.8% | 93.0% | 94.5% | 0.947 | 16.1s |
| d4 | 87 | 93.1% | 78.2% | 92.9% | 0.929 | 14.2s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 37 | 97.3% | 64.9% | - | - | 8.4s |
| 101+ | 4 | 75.0% | 100.0% | 75.0% | 0.875 | 8.3s |
| 2-5 | 311 | 95.5% | 93.2% | 95.3% | 0.955 | 14.4s |
| 33-100 | 61 | 88.5% | 72.1% | 88.1% | 0.881 | 19.0s |
| 6-32 | 19 | 84.2% | 100.0% | 70.0% | 0.789 | 14.4s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 13.4s |
| colloquial | 69 | 95.7% | 95.7% | 95.4% | 0.962 | 15.8s |
| condition | 32 | 96.9% | 90.6% | 96.8% | 0.968 | 17.8s |
| english | 12 | 100.0% | 66.7% | 100.0% | 1.000 | 22.1s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 19.7s |
| keyword | 20 | 80.0% | 80.0% | 80.0% | 0.800 | 14.4s |
| long | 12 | 100.0% | 91.7% | 100.0% | 1.000 | 15.2s |
| mixed | 13 | 92.3% | 84.6% | 92.3% | 0.923 | 10.9s |
| multi_intent | 9 | 100.0% | 66.7% | 100.0% | 1.000 | 16.4s |
| negation | 20 | 95.0% | 80.0% | 95.0% | 0.950 | 23.6s |
| oos_far | 9 | 100.0% | 88.9% | - | - | 4.3s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 21.1s |
| paraphrase | 177 | 94.4% | 92.1% | 92.8% | 0.928 | 13.7s |
| typo | 13 | 100.0% | 92.3% | 100.0% | 1.000 | 16.0s |
| vague | 23 | 78.3% | 100.0% | 78.3% | 0.858 | 13.5s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 45 | 91.1% | 95.6% | 90.5% | 0.925 | 16.3s |
| health | 39 | 97.4% | 100.0% | 97.1% | 0.976 | 10.8s |
| it | 100 | 94.0% | 82.0% | 93.7% | 0.937 | 10.9s |
| life | 40 | 97.5% | 92.5% | 97.2% | 0.972 | 16.4s |
| money | 48 | 91.7% | 87.5% | 91.1% | 0.917 | 15.2s |
| oos | 34 | 97.1% | 61.8% | - | - | 8.4s |
| shop | 40 | 87.5% | 90.0% | 86.5% | 0.878 | 24.1s |
| travel | 43 | 100.0% | 100.0% | 100.0% | 1.000 | 17.8s |
| work | 43 | 90.7% | 88.4% | 90.0% | 0.912 | 15.6s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 95.8% | 93.2% | 95.5% | 0.964 | 12.5s |
| hard | 112 | 93.8% | 79.5% | 92.2% | 0.922 | 17.9s |
| medium | 202 | 93.1% | 90.1% | 92.3% | 0.929 | 14.5s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 292 | 93.5% | 87.0% | 92.4% | 0.930 | 15.1s |
| test | 140 | 95.0% | 90.7% | 94.9% | 0.956 | 14.5s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 93.3% | 97.8% | 94.2% | 6.2% |
| 0.25 | 94.0% | 97.3% | 95.5% | 1.6% |
| 0.30 | 94.0% | 97.0% | 95.8% | 1.6% |
| 0.35 | 93.8% | 96.7% | 95.8% | 1.6% |
| 0.40 | 93.8% | 96.5% | 96.1% | 1.6% |
| 0.50 | 93.5% | 95.4% | 96.9% | 1.6% |
| 0.60 | 91.7% | 92.4% | 97.6% | 0.0% |
| 0.65 | 89.6% | 89.1% | 98.5% | 0.0% |

### 저장 (ingest, 68건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 94.1% [85.8–97.7] (n=68) |
|   └ 올바른 카테고리에 저장 | 100.0% [93.6–100.0] (n=56) |
|   └ 트리/루트 밖 Q&A 거절 | 66.7% [39.1–86.2] (n=12) |
| 계층 F1 | 1.000 [1.000–1.000] (n=56) |
| 초안이 검색에 안 보임 | 100.0% [93.6–100.0] (n=56) |
| 게시 성공 (버전 +1) | 100.0% [93.6–100.0] (n=56) |
| 초안→게시 카테고리 유지 | 100.0% [93.6–100.0] (n=56) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [93.6–100.0] (n=56) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 87.0% [67.9–95.5] (n=23) |
| 게시 후 probe 검색 top1 답변 | 87.0% [67.9–95.5] (n=23) |
| 지연 p50 (초안+게시+probe) | 40.4s |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–6.0] (n=60) |

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
| it_pc_win_keys | it | 2 |
| it_pc_win_keys | it_pc_win | 2 |
| null | money_save | 2 |
| shop_order_receipt | money_tax | 2 |
| it_office_excel_func | work | 1 |
| it_office_excel_func | work_hr | 1 |
| it_office_excel_func | work_pay | 1 |
| it_pc_peri_monitor | it | 1 |
| it_pc_peri_monitor | it_pc_peri | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| life_appl_wash_wm_drum_care_smell | life_clean_towel | 1 |
| life_clean_stain | life_clean | 1 |
| money_bank_acct_limit | work_hr_cert | 1 |
| money_bank_cert | it_sec | 1 |

### 틀린 케이스 (61건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-001 | 내일 서울에 비가 오나요? | null | travel | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit | false_tree |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food · top food_safe_poison#2 0.60 alternative | wrong top1 |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_pan#2 0.46 alternative | wrong top1 |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.79 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.85 recommended | route branch, wrong top1 |
| S-HEALTH-017 | 집에서 다쳤을 때 쓸 응급처치를 종류별로 한번 정리해 두고 싶어요 | health_aid | health_aid · top health_aid_injury_sprain#1 0.64 alternative | wrong top1 |
| S-IT-009 | I plugged an external monitor into my laptop with HDMI but it shows nothing | it_pc_peri_monitor | it · top it_pc_peri_monitor#1 0.96 recommended | route shallow |
| S-IT-046 | 아까 그 순서 문제 다시요. 마우스를 오른쪽 끝으로 밀면 왼쪽 화면에서 튀어나와요 | it_pc_peri_monitor | it_pc_peri · top it_pc_peri_monitor#2 0.94 recommended | route shallow |
| S-IT-505 | windows shortcut to create a new virtual desktop | it_pc_win_keys | it · top it_pc_win_keys#win_ctrl_d 0.95 recommended | route shallow |
| S-IT-507 | 작업관리자 바로 뜨게하는 단추키 머에요 | it_pc_win_keys | it_pc_win · top it_pc_win_keys#ctrl_shift_esc 0.96 recommended | route shallow |
| S-IT-512 | Win+D 말고, 창을 전부 최소화했다가 따로 되돌리는 키가 있다던데요 | it_pc_win_keys | it_pc_win · top it_pc_win_keys#win_m 0.94 recommended | route shallow |
| S-IT-516 | 그럼 녹화는요? | it_pc_win_keys | it · top it_pc_win_keys#win_alt_r 0.92 recommended | route shallow |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · abstained · top work_pay_sev#2 0.24 reference | route domain, abstained |
| S-IT-519 | B열에 '완료'라고 적힌 칸이 몇 개인지만 알면 돼요 | it_office_excel_func | it_office · top it_office_excel_func#countif 0.81 recommended | route shallow |
| S-IT-520 | VLOOKUP 말고요, 찾는 열이 가져올 열보다 오른쪽에 있을 때는 어떻게 해요? | it_office_excel_func | it · top it_office_excel_func#index_match 0.84 recommended | route shallow |
| S-IT-521 | 근무일수 계산 함수 | it_office_excel_func | work_pay · top work_pay_slip#2 0.40 alternative | route domain, wrong top1 |
| S-IT-522 | excel formula to add up values that meet several criteria at once | it_office_excel_func | it_office · top it_office_excel_func#sumifs 0.95 recommended | route shallow |
| S-IT-524 | 입사일만 넣으면 몇 년 다녔는지 자동으로 나오게 하고 싶은데ㅠ 방법 없나 | it_office_excel_func | work_hr · abstained · top work_hr_cert#2 0.08 reference | route domain, abstained |
| S-IT-525 | lookup 했는데 값이 없으면 #N/A 대신 blank나 '없음'으로 보이게 하는 function 있어요? | it_office_excel_func | it_office_excel · top it_office_excel_func#ifna 0.94 recommended | route shallow |
| S-IT-526 | 셀에 있는 이메일 주소에서 골뱅이 앞 아이디 부분만 떼어 내고 싶어요 | it_office_excel_func | it · top it_office_excel_func#find 0.90 recommended | route shallow |
| S-IT-527 | 매달 거래처별 매출 파일을 받는데 거래처 이름이 수백 줄에 중복돼서 들어가 있어요. 보고서 첫 장에 거래처 목록을 한 번씩만 깔끔하게 뽑아 두고 | it_office_excel_func | it_office_excel · top it_office_excel_func#unique 0.92 recommended | route shallow |
| S-IT-528 | 4.56을 5로, 4.44는 4로 만드는 것처럼 소수점 첫째 자리에서 반올림해 정수로 만들려면요? | it_office_excel_func | null · abstained · top no items | route false_none, abstained |
| S-IT-529 | 텍스트 합치기 구분자 넣어서 | it_office_excel_func | null · abstained · top no items | route false_none, abstained |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell | life_clean_towel · top life_clean_towel#1 0.57 alternative | route branch, wrong top1 |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_ac#2 0.71 recommended | route shallow |
| S-LIFE-029 | 피요. 이미 말라서 딱딱하게 굳었어요 | life_clean_stain | life_clean · top life_clean_stain#3 0.91 recommended | route shallow |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec_phish#1 0.11 reference | route domain, abstained |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost | money_card · top money_card_lost#4 0.96 recommended | route shallow |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money_bank · top money_bank_xfer_cap#1 0.62 alternative | route shallow |
| S-MONEY-021 | 제 명의 신용카드 관련해서 이것저것 궁금한 게 있는데요 | money_card | money_card · top money_card_lost#4 0.74 recommended | wrong top1 |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | work_exp_claim · top work_exp_claim#2 0.86 recommended | route domain, wrong top1 |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit | work_hr_cert · abstained · top work_hr_cert#2 0.21 reference | route domain, abstained |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card_limit#2 0.13 reference | route false_tree |
| S-OOS-001 | 내일 서울 날씨 어때요? 우산 챙겨야 하나 | null · no answer | travel_local · abstained · top travel_local_metro#3 0.06 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it_office_excel_func · abstained · top it_office_excel_func#ifs 0.24 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money_save · top money_save#article 0.54 alternative | route false_tree, answered unanswerable |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health_sym#article 0.07 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_card_lost#4 0.21 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car · abstained · top travel_car#article 0.08 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money_save · abstained · top money_save#article 0.07 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_wm_top_clean#2 0.13 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it_sec · abstained · top it_sec_phish#1 0.09 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym · abstained · top health_sym#article 0.22 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.13 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_track#3 0.14 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax · top money_tax_income#3 0.39 reference | route domain, wrong top1 |
| S-SHOP-019 | 여기 회원이면 받을 수 있는 혜택을 전체적으로 알고 싶어요 | shop_member | shop_member · top shop_member_grade#2 0.46 alternative | wrong top1 |
| S-SHOP-028 | 아 잠깐만요, 저 사업자 아니고 개인이에요. 무통장으로 입금했는데 그럼 뭘 신청하면 돼요? | shop_order_receipt | money_tax · abstained · top money_tax#article 0.19 reference | route domain, abstained |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.55 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.74 recommended | wrong top1 |
| S-WORK-008 | 연말정산 공제 얘기 말고요, 매달 월급에서 떼가는 건강보험이랑 소득세 항목이 뭔지 알고 싶어요 | work_pay_slip | money_tax · abstained · top money_tax#article 0.17 reference | route domain, abstained |
| S-WORK-009 | Do I need my manager's approval before working late on a weekday to get overtime | work_pay_ot | work_pay · top work_pay_ot#1 0.95 recommended | route shallow |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work · top work_pay_slip#2 0.67 recommended | route shallow |
| S-WORK-024 | 해외는요? | work_exp_trip | travel_stay · abstained · top travel_stay#article 0.04 reference | route domain, abstained |
| S-WORK-032 | 경조금 받으려면 증빙으로 뭘 올려야 하죠? | work_leave_event | work_pay · abstained · top work_pay_slip#4 0.08 reference | route branch, abstained |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.62 alternative | route branch, wrong top1 |
| S-XD-027 | 환불은 언제 들어와요? | money_tax_yearend | money · top money_tax_yearend#2 0.93 recommended | route shallow |

## jev_llm · shallow

### 검색 (388건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 94.8% [92.2–96.6] (n=388) |
|   └ 답할 수 있는 질문 | 94.4% [91.4–96.4] (n=337) |
|   └ 답할 수 없는 질문(보류가 정답) | 98.0% [89.7–99.7] (n=51) |
| 확신 정답률 (top1 정답 + recommended) | 87.8% [83.9–90.9] (n=337) |
| 라우팅 정확도 (정확 일치) | 89.2% [85.7–91.9] (n=388) |
| 계층 F1 (hF1) | 0.918 [0.892–0.943] (n=388) |
| 트리 밖 판정 precision | 94.1% [73.0–99.0] (n=17) |
| 트리 밖 판정 recall | 50.0% [33.6–66.4] (n=32) |
| 트리 밖 판정 F1 | 0.653 |
| 오보류율 (답할 수 있는데 보류) | 2.4% [1.2–4.6] (n=337) |
| 오답변율 (답할 수 없는데 답함) | 2.0% [0.3–10.3] (n=51) |
| Hit@1 | 94.4% [91.4–96.4] (n=337) |
| Hit@3 | 95.3% [92.4–97.1] (n=337) |
| Hit@k (limit) | 95.3% [92.4–97.1] (n=337) |
| MRR@k | 0.948 [0.925–0.971] (n=337) |
| nDCG@k | 0.938 [0.915–0.962] (n=337) |
| Recall@k | 0.942 [0.918–0.966] (n=337) |
| recommended 정밀도 | 0.978 [0.963–0.994] (n=302) |
| 라우팅이 맞았을 때 Hit@1 | 99.0% [97.2–99.7] (n=312) |
| 라우팅이 맞았을 때 MRR | 0.995 [0.990–1.000] (n=312) |
| 지연 p50 / p90 / p95 / 평균 | 18.1s / 47.2s / 51.6s / 22.4s |
| 라우팅 구간 평균 | 22.0s |
| 랭킹 후보 수 평균 | 17.0 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.42 |
| LLM 라우팅 호출 수 평균 | 1.84 |
| LLM 실패 → Jev 빔 대체율 | 5.7% [3.8–8.4] (n=388) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 |
|---|---:|---:|
| 일치율 | 97.3% (n=336) | 91.4% (n=326) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 8 | 8 | 401 | 1 | 19 | 10 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 33.2s |
| internal | 10 | 80.0% | 100.0% | 80.0% | 0.900 | 34.9s |
| leaf | 327 | 94.8% | 92.4% | 94.8% | 0.950 | 19.6s |
| no_answer | 16 | 100.0% | 93.8% | - | - | 13.4s |
| oos | 32 | 96.9% | 50.0% | - | - | 14.7s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 93.3% | 90.5% | 93.2% | 0.932 | 19.7s |
| single | 283 | 95.4% | 88.7% | 94.9% | 0.955 | 17.2s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 283 | 95.4% | 88.7% | 94.9% | 0.955 | 17.2s |
| coref | 19 | 84.2% | 78.9% | 84.2% | 0.842 | 24.4s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 27.1s |
| disambig | 20 | 95.0% | 90.0% | 95.0% | 0.950 | 24.4s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 24.8s |
| ellipsis | 9 | 88.9% | 88.9% | 88.9% | 0.889 | 14.1s |
| long | 8 | 75.0% | 75.0% | 75.0% | 0.750 | 21.3s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 14.0s |
| shift | 14 | 100.0% | 92.9% | 100.0% | 1.000 | 24.7s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 25.3s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 357 | 94.7% | 89.1% | 94.0% | 0.945 | 20.3s |
| root | 31 | 96.8% | 90.3% | 100.0% | 1.000 | 2.2s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 10 | 80.0% | 100.0% | 80.0% | 0.900 | 34.9s |
| leaf | 346 | 95.1% | 92.5% | 94.8% | 0.950 | 18.8s |
| none | 32 | 96.9% | 50.0% | - | - | 14.7s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 32 | 96.9% | 50.0% | - | - | 14.7s |
| d1 | 10 | 80.0% | 100.0% | 80.0% | 0.900 | 34.9s |
| d2 | 346 | 95.1% | 92.5% | 94.8% | 0.950 | 18.8s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 35 | 97.1% | 54.3% | - | - | 14.7s |
| 101+ | 4 | 50.0% | 100.0% | 50.0% | 0.750 | 47.6s |
| 2-5 | 293 | 97.3% | 95.2% | 97.1% | 0.973 | 17.2s |
| 33-100 | 56 | 83.9% | 78.6% | 83.6% | 0.836 | 30.6s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 14.8s |
| colloquial | 66 | 95.5% | 97.0% | 95.2% | 0.960 | 21.3s |
| condition | 32 | 96.9% | 93.8% | 96.8% | 0.968 | 15.9s |
| english | 12 | 100.0% | 83.3% | 100.0% | 1.000 | 39.2s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 38.4s |
| keyword | 20 | 85.0% | 85.0% | 85.0% | 0.850 | 17.0s |
| long | 12 | 91.7% | 91.7% | 91.7% | 0.917 | 24.7s |
| mixed | 13 | 100.0% | 92.3% | 100.0% | 1.000 | 15.7s |
| multi_intent | 9 | 100.0% | 66.7% | 100.0% | 1.000 | 28.0s |
| negation | 20 | 90.0% | 90.0% | 90.0% | 0.900 | 24.2s |
| oos_far | 9 | 100.0% | 77.8% | - | - | 12.0s |
| oos_near | 11 | 100.0% | 0.0% | - | - | 26.6s |
| paraphrase | 149 | 94.6% | 91.9% | 94.3% | 0.943 | 16.1s |
| typo | 13 | 100.0% | 92.3% | 100.0% | 1.000 | 17.7s |
| vague | 10 | 80.0% | 100.0% | 80.0% | 0.900 | 34.9s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 39 | 89.7% | 92.3% | 89.5% | 0.908 | 24.7s |
| health | 35 | 100.0% | 97.1% | 100.0% | 1.000 | 23.7s |
| it | 92 | 90.2% | 87.0% | 89.9% | 0.904 | 21.9s |
| life | 35 | 100.0% | 97.1% | 100.0% | 1.000 | 16.8s |
| money | 43 | 97.7% | 93.0% | 97.6% | 0.976 | 17.2s |
| oos | 32 | 96.9% | 50.0% | - | - | 14.7s |
| shop | 35 | 94.3% | 94.3% | 93.9% | 0.939 | 24.2s |
| travel | 38 | 100.0% | 100.0% | 100.0% | 1.000 | 14.1s |
| work | 39 | 92.3% | 89.7% | 91.9% | 0.932 | 7.4s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 109 | 97.2% | 94.5% | 97.0% | 0.975 | 15.6s |
| hard | 100 | 91.0% | 75.0% | 89.5% | 0.895 | 17.8s |
| medium | 179 | 95.5% | 93.9% | 95.0% | 0.956 | 19.7s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 260 | 93.8% | 87.7% | 93.4% | 0.939 | 17.7s |
| test | 128 | 96.9% | 92.2% | 96.3% | 0.968 | 18.4s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 94.3% | 98.5% | 94.9% | 5.9% |
| 0.25 | 94.3% | 97.9% | 95.5% | 5.9% |
| 0.30 | 94.8% | 97.6% | 96.4% | 2.0% |
| 0.35 | 94.6% | 97.0% | 96.6% | 2.0% |
| 0.40 | 94.3% | 96.4% | 96.9% | 2.0% |
| 0.50 | 93.6% | 95.3% | 97.2% | 2.0% |
| 0.60 | 91.2% | 92.0% | 97.7% | 2.0% |
| 0.65 | 89.2% | 89.3% | 98.0% | 2.0% |

### 저장 (ingest, 59건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 93.2% [83.8–97.3] (n=59) |
|   └ 올바른 카테고리에 저장 | 98.0% [89.3–99.6] (n=49) |
|   └ 트리/루트 밖 Q&A 거절 | 70.0% [39.7–89.2] (n=10) |
| 계층 F1 | 0.993 [0.980–1.000] (n=49) |
| 초안이 검색에 안 보임 | 100.0% [92.7–100.0] (n=49) |
| 게시 성공 (버전 +1) | 100.0% [92.7–100.0] (n=49) |
| 초안→게시 카테고리 유지 | 98.0% [89.3–99.6] (n=49) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [92.7–100.0] (n=49) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 8 |
| 게시 후 probe 검색 top-k 포함 | 100.0% [83.2–100.0] (n=19) |
| 게시 후 probe 검색 top1 답변 | 100.0% [83.2–100.0] (n=19) |
| 지연 p50 (초안+게시+probe) | 45.9s |
| LLM 실패 → Jev 빔 대체율 | 5.8% [2.0–15.6] (n=52) |

### API 계약 (12건)

통과 100.0% [75.7–100.0] (n=12)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| it_office_excel_func | work | 3 |
| food_store_item | food_store_freeze | 2 |
| it_pc_win_keys | it | 2 |
| money_tax_yearend | money | 2 |
| null | health | 2 |
| null | money | 2 |
| null | travel | 2 |
| food_tool_pan | life_clean_grease | 1 |
| health_sym_head | health | 1 |
| it_office_excel_func | it | 1 |
| it_office_excel_func | it_office_excel_print | 1 |
| it_office_excel_func | null | 1 |
| it_pc_peri_monitor | it | 1 |
| it_pc_peri_monitor | it_pc_win_keys | 1 |
| it_pc_win_keys | it_pc_mac | 1 |
| it_pc_win_keys | it_pc_win_power_boot | 1 |
| life_appl_air_ac | life | 1 |
| money_bank_acct_limit | work_hr_cert | 1 |
| null | food | 1 |
| null | food_safe_poison | 1 |

### 틀린 케이스 (49건 중 49건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it_office_excel_func | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit_start | false_tree |
| I-WORK-001 | 사원증을 잃어버렸는데 재발급은 어떻게 받나요? | work_hr_onboard | work | shallow |
| S-FOOD-009 | 자취 시작해서 부엌일이 전부 처음이에요. 뭐부터 알아 두면 좋을까요? | food | food · top food_safe_poison#2 0.63 alternative | wrong top1 |
| S-FOOD-013 | 그거 처음에 어떻게 해야 하는 건데요? | food_tool_pan | life_clean_grease · abstained · top life_clean_grease#1 0.08 reference | route domain, abstained |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item | food_store_freeze · top food_store_freeze#1 0.81 recommended | route branch, wrong top1 |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.87 recommended | route branch, wrong top1 |
| S-HEALTH-015 | 요즘 밤에 잠도 잘 못 자고, 오후만 되면 머리가 조이듯 지끈거려요. 둘 다 어떻게 관리하면 좋을까요? | health_sym_head (+1) | health · top health_sleep#1 0.64 alternative | route shallow |
| S-IT-009 | I plugged an external monitor into my laptop with HDMI but it shows nothing | it_pc_peri_monitor | it · top it_pc_peri_monitor#1 0.95 recommended | route shallow |
| S-IT-046 | 아까 그 순서 문제 다시요. 마우스를 오른쪽 끝으로 밀면 왼쪽 화면에서 튀어나와요 | it_pc_peri_monitor | it_pc_win_keys · abstained · top it_pc_win_keys#alt_space 0.23 reference | route branch, abstained |
| S-IT-505 | windows shortcut to create a new virtual desktop | it_pc_win_keys | it · top it_pc_win_keys#win_ctrl_d 0.93 recommended | route shallow |
| S-IT-507 | 작업관리자 바로 뜨게하는 단추키 머에요 | it_pc_win_keys | it · top it_pc_win_keys#ctrl_shift_esc 0.95 recommended | route shallow |
| S-IT-512 | Win+D 말고, 창을 전부 최소화했다가 따로 되돌리는 키가 있다던데요 | it_pc_win_keys | it_pc_win_power_boot · abstained · top it_pc_win_power_boot#2 0.02 reference | route branch, abstained |
| S-IT-516 | 그럼 녹화는요? | it_pc_win_keys | it_pc_mac · top it_pc_mac#3 0.53 alternative | route branch, wrong top1 |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func | work · top work_exp_claim#3 0.31 reference | route domain, wrong top1 |
| S-IT-521 | 근무일수 계산 함수 | it_office_excel_func | work · top work_pay_slip#2 0.47 alternative | route domain, wrong top1 |
| S-IT-524 | 입사일만 넣으면 몇 년 다녔는지 자동으로 나오게 하고 싶은데ㅠ 방법 없나 | it_office_excel_func | work · abstained · top work_exp_claim#3 0.29 reference | route domain, abstained |
| S-IT-525 | lookup 했는데 값이 없으면 #N/A 대신 blank나 '없음'으로 보이게 하는 function 있어요? | it_office_excel_func | it · top it_office_excel_func#ifna 0.94 recommended | route shallow |
| S-IT-527 | 매달 거래처별 매출 파일을 받는데 거래처 이름이 수백 줄에 중복돼서 들어가 있어요. 보고서 첫 장에 거래처 목록을 한 번씩만 깔끔하게 뽑아 두고 | it_office_excel_func | it_office_excel_print · abstained · top it_office_excel_print#2 0.05 reference | route branch, abstained |
| S-IT-528 | 4.56을 5로, 4.44는 4로 만드는 것처럼 소수점 첫째 자리에서 반올림해 정수로 만들려면요? | it_office_excel_func | null · abstained · top no items | route false_none, abstained |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life · top life_appl_air_purifier#1 0.64 alternative | route shallow |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend | money · top money_tax_yearend#4 0.62 alternative | route shallow |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit | work_hr_cert · abstained · top work_hr_cert#2 0.23 reference | route domain, abstained |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card_limit · abstained · top money_card_limit#2 0.03 reference | route false_tree |
| S-OOS-001 | 내일 서울 날씨 어때요? 우산 챙겨야 하나 | null · no answer | travel · abstained · top travel_local_metro#3 0.13 reference | route false_tree |
| S-OOS-005 | 세계에서 제일 높은 산이 어디야? | null · no answer | travel · abstained · top travel_local_metro#2 0.07 reference | route false_tree |
| S-OOS-009 | 파이썬에서 리스트를 정렬하는 코드 알려줘 | null · no answer | it_office_excel_func · abstained · top it_office_excel_func#ifs 0.20 reference | route false_tree |
| S-OOS-010 | 요즘 사면 오를 만한 주식 종목 좀 찍어 주세요 | null · no answer | money_save_etf · abstained · top money_save_etf#2 0.09 reference | route false_tree |
| S-OOS-011 | 강아지 예방접종은 생후 몇 주부터 맞혀요? | null · no answer | health · abstained · top health_fit_stretch#1 0.09 reference | route false_tree |
| S-OOS-012 | 전세 계약할 때 확정일자는 어디서 받아요? | null · no answer | money · abstained · top money_ins#2 0.17 reference | route false_tree |
| S-OOS-013 | 자동차 엔진오일은 몇 km마다 갈아야 해요? | null · no answer | travel_car_rent · abstained · top travel_car_rent#2 0.02 reference | route false_tree |
| S-OOS-014 | 비트코인 거래소 가입은 어떻게 해요? | null · no answer | money · abstained · top money_tax_yearend#4 0.14 reference | route false_tree |
| S-OOS-015 | 이사 가면 전입신고는 어디서 해요? | null · no answer | life · abstained · top life_appl_wash_wm_drum_water_drain#2 0.12 reference | route false_tree |
| S-OOS-016 | 넷플릭스 구독 해지하는 방법 알려줘 | null · no answer | it · abstained · top it_mobile_lost#2 0.26 reference | route false_tree |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health · abstained · top health#article 0.29 reference | route false_tree |
| S-OOS-018 | 혹시 오늘 저녁 메뉴 추천해 줄 수 있어요? | null · no answer | food · abstained · top food#article 0.15 reference | route false_tree |
| S-OOS-020 | iPhone 16 Pro 가격이 얼마예요? | null · no answer | shop · abstained · top shop_ship_addr#2 0.13 reference | route false_tree |
| S-SHOP-003 | 사업자 세금계산서 | shop_order_receipt | money_tax_income · top money_tax_income#3 0.37 reference | route domain, wrong top1 |
| S-SHOP-030 | 하나만 취소하려면 어떻게 해요? | shop_order_pay | shop_order_cancel · top shop_order_cancel#2 0.55 alternative | route branch, wrong top1 |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.74 recommended | wrong top1 |
| S-WORK-008 | 연말정산 공제 얘기 말고요, 매달 월급에서 떼가는 건강보험이랑 소득세 항목이 뭔지 알고 싶어요 | work_pay_slip | money_tax_yearend · abstained · top money_tax_yearend#2 0.07 reference | route domain, abstained |
| S-WORK-017 | 반차 신청하는 법이랑 급여명세서 보는 곳 둘 다 알려주세요 | work_leave_annual_use (+1) | work · top work_pay_slip#2 0.64 alternative | route shallow |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#3 0.07 reference | route domain |
| S-XD-027 | 환불은 언제 들어와요? | money_tax_yearend | money · top money_tax_yearend#2 0.94 recommended | route shallow |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.81 recommended | route branch, wrong top1 |
| S-XD-044 | IT 쪽으로 이것저것 물어보고 싶은데 어떤 것들을 다뤄요? | it | it · top it_pc_win_keys#alt_space 0.68 recommended | wrong top1 |
| S-XD-049 | 회사 법인카드를 잃어버렸어요. 어디에 신고해요? | null · no answer | money_card_lost · top money_card_lost#1 0.83 recommended | route false_tree, answered unanswerable |
| S-XD-052 | 상한 음식을 먹고 계속 토해요. 어떻게 해야 돼요? | null · no answer | food_safe_poison · abstained · top food_safe_poison#2 0.15 reference | route false_tree |

## jev_llm · flat

### 검색 (357건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 98.0% [96.0–99.0] (n=357) |
|   └ 답할 수 있는 질문 | 97.8% [95.5–98.9] (n=317) |
|   └ 답할 수 없는 질문(보류가 정답) | 100.0% [91.2–100.0] (n=40) |
| 확신 정답률 (top1 정답 + recommended) | 90.9% [87.2–93.6] (n=317) |
| 라우팅 정확도 (정확 일치) | 97.8% [95.6–98.9] (n=357) |
| 계층 F1 (hF1) | 0.978 [0.962–0.993] (n=357) |
| 트리 밖 판정 precision | 95.2% [77.3–99.2] (n=21) |
| 트리 밖 판정 recall | 95.2% [77.3–99.2] (n=21) |
| 트리 밖 판정 F1 | 0.952 |
| 오보류율 (답할 수 있는데 보류) | 0.3% [0.1–1.8] (n=317) |
| 오답변율 (답할 수 없는데 답함) | 0.0% [0.0–8.8] (n=40) |
| Hit@1 | 97.8% [95.5–98.9] (n=317) |
| Hit@3 | 98.1% [95.9–99.1] (n=317) |
| Hit@k (limit) | 98.1% [95.9–99.1] (n=317) |
| MRR@k | 0.979 [0.964–0.995] (n=317) |
| nDCG@k | 0.965 [0.948–0.981] (n=317) |
| Recall@k | 0.965 [0.948–0.983] (n=317) |
| recommended 정밀도 | 0.987 [0.974–1.000] (n=292) |
| 라우팅이 맞았을 때 Hit@1 | 99.7% [98.2–99.9] (n=311) |
| 라우팅이 맞았을 때 MRR | 0.998 [0.995–1.000] (n=311) |
| 지연 p50 / p90 / p95 / 평균 | 3.6s / 24.9s / 35.4s / 9.3s |
| 라우팅 구간 평균 | 8.9s |
| 랭킹 후보 수 평균 | 12.1 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.17 |
| LLM 라우팅 호출 수 평균 | 0.97 |
| LLM 실패 → Jev 빔 대체율 | 1.1% [0.4–2.8] (n=357) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 |
|---|---:|
| 일치율 | 97.5% (n=326) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_none | false_tree |
|---:|---:|---:|---:|---:|
| 4 | 2 | 399 | 1 | 1 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 4.2s |
| leaf | 317 | 97.8% | 98.1% | 97.8% | 0.979 | 3.7s |
| no_answer | 16 | 100.0% | 93.8% | - | - | 4.3s |
| oos | 21 | 100.0% | 95.2% | - | - | 3.0s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 97.1% | 97.1% | 97.1% | 0.971 | 3.2s |
| single | 252 | 98.4% | 98.0% | 98.1% | 0.984 | 3.9s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 252 | 98.4% | 98.0% | 98.1% | 0.984 | 3.9s |
| coref | 19 | 89.5% | 89.5% | 89.5% | 0.895 | 7.2s |
| correct | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 16.9s |
| disambig | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 2.4s |
| distract | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 4.6s |
| ellipsis | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 22.4s |
| long | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 2.7s |
| refine | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 4.3s |
| shift | 14 | 100.0% | 100.0% | 100.0% | 1.000 | 2.9s |
| slot | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 4.8s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 347 | 98.0% | 97.7% | 97.7% | 0.979 | 3.9s |
| root | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 0.3s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| leaf | 336 | 97.9% | 97.9% | 97.8% | 0.979 | 3.8s |
| none | 21 | 100.0% | 95.2% | - | - | 3.0s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 21 | 100.0% | 95.2% | - | - | 3.0s |
| d1 | 336 | 97.9% | 97.9% | 97.8% | 0.979 | 3.8s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 24 | 100.0% | 95.8% | - | - | 3.1s |
| 2-5 | 283 | 98.2% | 98.2% | 98.1% | 0.983 | 3.7s |
| 33-100 | 50 | 96.0% | 96.0% | 95.9% | 0.959 | 4.0s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 5.5s |
| colloquial | 66 | 98.5% | 100.0% | 98.4% | 0.992 | 3.9s |
| condition | 31 | 100.0% | 100.0% | 100.0% | 1.000 | 4.4s |
| english | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 3.5s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 2.1s |
| keyword | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 4.7s |
| long | 12 | 100.0% | 100.0% | 100.0% | 1.000 | 2.6s |
| mixed | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 5.8s |
| multi_intent | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 14.5s |
| negation | 20 | 95.0% | 95.0% | 95.0% | 0.950 | 3.3s |
| oos_far | 9 | 100.0% | 100.0% | - | - | 3.0s |
| oos_near | 11 | 100.0% | 90.9% | - | - | 3.1s |
| paraphrase | 129 | 96.9% | 96.1% | 96.5% | 0.965 | 3.3s |
| typo | 13 | 100.0% | 100.0% | 100.0% | 1.000 | 3.3s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 36 | 97.2% | 97.2% | 97.1% | 0.971 | 2.6s |
| health | 32 | 100.0% | 100.0% | 100.0% | 1.000 | 4.3s |
| it | 89 | 96.6% | 96.6% | 96.5% | 0.965 | 5.1s |
| life | 33 | 100.0% | 100.0% | 100.0% | 1.000 | 3.6s |
| money | 41 | 97.6% | 97.6% | 97.4% | 0.974 | 3.0s |
| oos | 21 | 100.0% | 95.2% | - | - | 3.0s |
| shop | 32 | 100.0% | 100.0% | 100.0% | 1.000 | 4.3s |
| travel | 36 | 100.0% | 100.0% | 100.0% | 1.000 | 3.4s |
| work | 37 | 94.6% | 94.6% | 94.3% | 0.957 | 2.9s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 99 | 98.0% | 99.0% | 97.8% | 0.984 | 3.5s |
| hard | 95 | 95.8% | 94.7% | 94.7% | 0.947 | 3.6s |
| medium | 163 | 99.4% | 98.8% | 99.3% | 0.993 | 3.9s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 237 | 97.5% | 96.6% | 97.2% | 0.972 | 3.6s |
| test | 120 | 99.2% | 100.0% | 99.0% | 0.995 | 3.9s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 97.8% | 99.7% | 97.8% | 2.5% |
| 0.25 | 98.0% | 99.7% | 98.1% | 0.0% |
| 0.30 | 98.0% | 99.7% | 98.1% | 0.0% |
| 0.35 | 97.8% | 99.1% | 98.4% | 0.0% |
| 0.40 | 97.8% | 99.1% | 98.4% | 0.0% |
| 0.50 | 96.9% | 98.1% | 98.4% | 0.0% |
| 0.60 | 94.1% | 94.6% | 98.7% | 0.0% |
| 0.65 | 91.9% | 92.1% | 98.6% | 0.0% |

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
| 지연 p50 (초안+게시+probe) | 15.1s |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–7.6] (n=47) |

### API 계약 (11건)

통과 100.0% [74.1–100.0] (n=11)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 1 |
| it_mobile_lost | travel_local_metro | 1 |
| it_office_excel_func | null | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| money_card_lost | money_card_abroad | 1 |
| null | health_sym_stomach | 1 |
| work_exp_trip | travel_air_ticket_mile | 1 |
| work_exp_trip | work_exp_claim | 1 |

### 틀린 케이스 (9건 중 9건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item | food_store_freeze · top food_store_freeze#1 0.86 recommended | route branch, wrong top1 |
| S-IT-017 | 지하철에 아이폰을 두고 내렸어요. 지금 어디 있는지 확인할 방법이 있을까요? | it_mobile_lost | travel_local_metro · top travel_local_metro#2 0.67 recommended | route domain, wrong top1 |
| S-IT-529 | 텍스트 합치기 구분자 넣어서 | it_office_excel_func | null · abstained · top no items | route false_none, abstained |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost | money_card_abroad · top money_card_abroad#3 0.30 reference | route branch, wrong top1 |
| S-OOS-017 | 고양이가 이틀째 사료를 안 먹어요 | null · no answer | health_sym_stomach · abstained · top health_sym_stomach#3 0.15 reference | route false_tree |
| S-WORK-003 | 연차 아직 8개나 남았는데 연말 지나면 걍 날아가는 거임?? 돈으로라도 주나 | work_leave_annual_left | work_leave_annual_left · top work_leave_annual_left#2 0.78 recommended | wrong top1 |
| S-WORK-020 | 출장 다니면서 쌓인 항공 마일리지는 개인적으로 써도 되나요? | work_exp_trip · no answer | travel_air_ticket_mile · abstained · top travel_air_ticket_mile#2 0.08 reference | route domain |
| S-XD-021 | 비밀번호 바꾸는 건 어디서 해요? | it_sec_hacked | it_sec_pw · top it_sec_pw#3 0.58 alternative | route branch, wrong top1 |
| S-XD-038 | 그거 영수증은 언제까지 올려야 해요? | work_exp_trip | work_exp_claim · top work_exp_claim#2 0.78 recommended | route branch, wrong top1 |

## jev_llm · sparse

### 검색 (432건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 86.6% [83.0–89.5] (n=432) |
|   └ 답할 수 있는 질문 | 94.8% [90.5–97.3] (n=174) |
|   └ 답할 수 없는 질문(보류가 정답) | 81.0% [75.8–85.3] (n=258) |
| 확신 정답률 (top1 정답 + recommended) | 81.6% [75.2–86.7] (n=174) |
| 라우팅 정확도 (정확 일치) | 88.0% [84.6–90.7] (n=432) |
| 계층 F1 (hF1) | 0.921 [0.898–0.944] (n=432) |
| 트리 밖 판정 precision | 90.5% [71.1–97.3] (n=21) |
| 트리 밖 판정 recall | 55.9% [39.5–71.1] (n=34) |
| 트리 밖 판정 F1 | 0.691 |
| 오보류율 (답할 수 있는데 보류) | 2.3% [0.9–5.8] (n=174) |
| 오답변율 (답할 수 없는데 답함) | 19.0% [14.7–24.2] (n=258) |
| Hit@1 | 95.4% [91.2–97.7] (n=174) |
| Hit@3 | 96.6% [92.7–98.4] (n=174) |
| Hit@k (limit) | 97.1% [93.5–98.8] (n=174) |
| MRR@k | 0.960 [0.933–0.988] (n=174) |
| nDCG@k | 0.963 [0.937–0.989] (n=174) |
| Recall@k | 0.963 [0.936–0.989] (n=174) |
| recommended 정밀도 | 0.881 [0.832–0.930] (n=158) |
| 라우팅이 맞았을 때 Hit@1 | 98.2% [94.8–99.4] (n=164) |
| 라우팅이 맞았을 때 MRR | 0.986 [0.970–1.000] (n=164) |
| 지연 p50 / p90 / p95 / 평균 | 13.9s / 35.2s / 45.1s / 17.4s |
| 라우팅 구간 평균 | 17.1s |
| 랭킹 후보 수 평균 | 3.2 |
| Jev 호출 수 평균 (라운드 + 32개 배치) | 1.00 |
| LLM 라우팅 호출 수 평균 | 1.84 |
| LLM 실패 → Jev 빔 대체율 | 1.4% [0.6–3.0] (n=432) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 97.2% (n=351) | 94.1% (n=341) | 90.1% (n=322) | 75.3% (n=81) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|
| 6 | 8 | 432 | 2 | 19 | 21 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dropped | 194 | 75.8% | 86.1% | - | - | 14.3s |
| empty_leaf | 3 | 100.0% | 100.0% | - | - | 14.3s |
| internal | 23 | 91.3% | 100.0% | 91.3% | 0.946 | 13.8s |
| leaf | 151 | 95.4% | 93.4% | 96.0% | 0.962 | 14.2s |
| no_answer | 27 | 100.0% | 100.0% | - | - | 13.8s |
| oos | 34 | 94.1% | 55.9% | - | - | 11.6s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 105 | 85.7% | 89.5% | 95.2% | 0.952 | 14.2s |
| single | 327 | 86.9% | 87.5% | 95.5% | 0.963 | 13.8s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 327 | 86.9% | 87.5% | 95.5% | 0.963 | 13.8s |
| coref | 19 | 89.5% | 89.5% | 100.0% | 1.000 | 13.6s |
| correct | 8 | 75.0% | 87.5% | 80.0% | 0.800 | 17.3s |
| disambig | 20 | 80.0% | 85.0% | 90.0% | 0.900 | 12.5s |
| distract | 9 | 88.9% | 88.9% | 100.0% | 1.000 | 31.5s |
| ellipsis | 9 | 88.9% | 88.9% | 100.0% | 1.000 | 15.2s |
| long | 8 | 75.0% | 87.5% | 100.0% | 1.000 | 14.3s |
| refine | 9 | 100.0% | 88.9% | 100.0% | 1.000 | 23.4s |
| shift | 14 | 92.9% | 92.9% | 100.0% | 1.000 | 17.3s |
| slot | 9 | 77.8% | 100.0% | 100.0% | 1.000 | 9.8s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 372 | 86.8% | 87.1% | 95.2% | 0.958 | 14.3s |
| root | 43 | 86.0% | 90.7% | 100.0% | 1.000 | 10.8s |
| start | 17 | 82.4% | 100.0% | 100.0% | 1.000 | 11.3s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 34 | 94.1% | 100.0% | 91.3% | 0.946 | 13.8s |
| leaf | 364 | 85.2% | 89.8% | 96.0% | 0.962 | 14.2s |
| none | 34 | 94.1% | 55.9% | - | - | 11.6s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 34 | 94.1% | 55.9% | - | - | 11.6s |
| d1 | 10 | 100.0% | 100.0% | 100.0% | 1.000 | 13.7s |
| d2 | 30 | 93.3% | 100.0% | 86.7% | 0.917 | 13.8s |
| d3 | 271 | 84.5% | 93.4% | 94.7% | 0.950 | 16.2s |
| d4 | 87 | 86.2% | 78.2% | 100.0% | 1.000 | 13.1s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 37 | 94.6% | 59.5% | - | - | 11.6s |
| 1 | 361 | 85.0% | 89.8% | 96.0% | 0.962 | 14.2s |
| 2-5 | 16 | 93.8% | 100.0% | 85.7% | 0.929 | 14.7s |
| 33-100 | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 13.7s |
| 6-32 | 16 | 93.8% | 100.0% | 92.9% | 0.946 | 12.6s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| canonical | 8 | 87.5% | 100.0% | - | - | 14.6s |
| colloquial | 69 | 87.0% | 95.7% | 100.0% | 1.000 | 13.9s |
| condition | 32 | 81.2% | 87.5% | 100.0% | 1.000 | 19.0s |
| english | 12 | 91.7% | 66.7% | 100.0% | 1.000 | 22.5s |
| injection | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 23.9s |
| keyword | 20 | 80.0% | 80.0% | 100.0% | 1.000 | 13.8s |
| long | 12 | 91.7% | 91.7% | 100.0% | 1.000 | 15.5s |
| mixed | 13 | 92.3% | 84.6% | 85.7% | 0.857 | 13.3s |
| multi_intent | 9 | 66.7% | 44.4% | 66.7% | 0.704 | 20.3s |
| negation | 20 | 80.0% | 85.0% | 100.0% | 1.000 | 9.5s |
| oos_far | 9 | 100.0% | 88.9% | - | - | 4.4s |
| oos_near | 11 | 90.9% | 0.0% | - | - | 15.8s |
| paraphrase | 177 | 87.0% | 92.7% | 96.1% | 0.961 | 14.0s |
| typo | 13 | 84.6% | 92.3% | 100.0% | 1.000 | 13.4s |
| vague | 23 | 91.3% | 100.0% | 91.3% | 0.946 | 13.8s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| food | 45 | 84.4% | 93.3% | 93.3% | 0.967 | 15.9s |
| health | 39 | 92.3% | 100.0% | 93.8% | 0.953 | 12.1s |
| it | 100 | 88.0% | 81.0% | 97.8% | 0.978 | 13.2s |
| life | 40 | 82.5% | 95.0% | 100.0% | 1.000 | 12.7s |
| money | 48 | 77.1% | 87.5% | 95.8% | 0.958 | 15.2s |
| oos | 34 | 94.1% | 55.9% | - | - | 11.6s |
| shop | 40 | 80.0% | 90.0% | 83.3% | 0.833 | 17.1s |
| travel | 43 | 90.7% | 100.0% | 100.0% | 1.000 | 21.5s |
| work | 43 | 90.7% | 93.0% | 94.4% | 0.963 | 21.1s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| easy | 118 | 91.5% | 93.2% | 100.0% | 1.000 | 14.1s |
| hard | 112 | 81.2% | 77.7% | 90.6% | 0.917 | 16.5s |
| medium | 202 | 86.6% | 90.6% | 94.5% | 0.953 | 13.5s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 292 | 84.9% | 87.3% | 95.1% | 0.960 | 14.7s |
| test | 140 | 90.0% | 89.3% | 96.1% | 0.961 | 13.5s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 82.2% | 98.3% | 69.2% | 26.7% |
| 0.25 | 85.0% | 98.3% | 72.8% | 22.1% |
| 0.30 | 86.6% | 97.7% | 75.3% | 19.0% |
| 0.35 | 88.7% | 96.6% | 79.2% | 15.1% |
| 0.40 | 90.7% | 94.8% | 83.9% | 10.9% |
| 0.50 | 91.9% | 93.7% | 87.0% | 8.5% |
| 0.60 | 90.5% | 87.4% | 88.8% | 6.6% |
| 0.65 | 89.4% | 82.8% | 89.9% | 5.4% |

### 저장 (ingest, 56건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 92.9% [83.0–97.2] (n=56) |
|   └ 올바른 카테고리에 저장 | 100.0% [92.0–100.0] (n=44) |
|   └ 트리/루트 밖 Q&A 거절 | 66.7% [39.1–86.2] (n=12) |
| 계층 F1 | 1.000 [1.000–1.000] (n=44) |
| 초안이 검색에 안 보임 | 100.0% [92.0–100.0] (n=44) |
| 게시 성공 (버전 +1) | 100.0% [92.0–100.0] (n=44) |
| 초안→게시 카테고리 유지 | 97.7% [88.2–99.6] (n=44) |
| 중복 탐지 precision | 100.0% |
| 중복 탐지 recall | 100.0% |
| 갱신/신규 판정 일치 | 100.0% [92.0–100.0] (n=44) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 3 |
| 게시 후 probe 검색 top-k 포함 | 87.0% [67.9–95.5] (n=23) |
| 게시 후 probe 검색 top1 답변 | 87.0% [67.9–95.5] (n=23) |
| 지연 p50 (초안+게시+probe) | 37.7s |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–7.4] (n=48) |

### API 계약 (11건)

통과 100.0% [74.1–100.0] (n=11)

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| food_store_item | food_store_freeze | 2 |
| it_office_excel_func | it | 2 |
| it_office_excel_func | it_office | 2 |
| it_office_excel_func | it_office_excel | 2 |
| it_office_excel_func | null | 2 |
| it_pc_win_keys | it | 2 |
| it_pc_win_keys | it_pc_win | 2 |
| null | life | 2 |
| food_cook_side_fry | life_home_drain | 1 |
| it_net_wifi_drop | it | 1 |
| it_office_excel_func | work | 1 |
| it_office_excel_func | work_hr | 1 |
| it_office_excel_func | work_leave | 1 |
| it_pc_peri_monitor | it | 1 |
| it_pc_peri_monitor | it_pc_peri | 1 |
| it_sec_hacked | it_sec_pw | 1 |
| life_appl_air_ac | life_appl_air | 1 |
| life_clean_stain | life_clean | 1 |
| money_bank_cert | it_sec | 1 |
| money_bank_xfer_cap | money_bank | 1 |

### 틀린 케이스 (101건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-MONEY-007 | 회사 법인카드를 잃어버렸을 때 누구에게 먼저 보고하나요? | null | money_card_lost | false_tree |
| I-OOS-002 | 파이썬에서 리스트를 정렬하려면 어떻게 하나요? | null | it_office_excel_func | false_tree |
| I-OOS-003 | 강아지 산책은 하루에 몇 번 시키는 게 좋나요? | null | health_fit | false_tree |
| I-SHOP-007 | 호텔 예약을 취소하면 수수료가 얼마나 나오나요? | null | shop_order_cancel | false_tree |
| S-FOOD-001 | 밥솥으로 밥을 했는데 밥알 가운데가 생쌀처럼 씹혀요 | food_cook_base_rice · no answer | food_cook_base_rice · top food_cook_base_rice#1 0.36 reference | answered unanswerable |
| S-FOOD-007 | 배탈 난 뒤 관리 말고요, 여름에 식중독 안 걸리게 미리 조심할 것들 알려주세요 | food_safe_poison · no answer | food_safe_poison · top food_safe_poison#1 0.45 alternative | answered unanswerable |
| S-FOOD-010 | 주방 조리도구들 오래 쓰려면 전반적으로 어떻게 관리해야 해요? | food_tool | food_tool · top food_tool_knife#1 0.40 alternative | wrong top1 |
| S-FOOD-018 | 아 근데 냉장고에 있는 요거트가 날짜가 이틀 지났는데 먹어도 될까요? | food_safe_date · no answer | food_safe_date · top food_safe_date#1 0.31 reference | answered unanswerable |
| S-FOOD-021 | 튀기고 남은 기름 그냥 싱크대에 부어 버려도 돼요? | food_cook_side_fry · no answer | life_home_drain · abstained · top life_home_drain#1 0.03 reference | route domain |
| S-FOOD-503 | 다진마늘 냉동 보관 | food_store_item · no answer | food_store_freeze · top food_store_freeze#1 0.84 recommended | route branch, answered unanswerable |
| S-FOOD-514 | 남는 건 얼려도 되나요? 어떻게 얼려요? | food_store_item · no answer | food_store_freeze · top food_store_freeze#1 0.89 recommended | route branch, answered unanswerable |
| S-HEALTH-007 | My 8-month-old baby is choking on something and can't breathe. What should I do  | health_aid_emerg_choke · no answer | health_aid_emerg_choke · top health_aid_emerg_choke#1 0.38 reference | answered unanswerable |
| S-HEALTH-009 | 달리기 말고 헬스장에서 근력이랑 유산소를 섞어서 하려는데 뭐부터 해야 돼요? | health_fit_start · no answer | health_fit_start · top health_fit_start#1 0.39 reference | answered unanswerable |
| S-HEALTH-017 | 집에서 다쳤을 때 쓸 응급처치를 종류별로 한번 정리해 두고 싶어요 | health_aid | health_aid · top health_aid_injury_cut#1 0.65 recommended | wrong top1 |
| S-IT-009 | I plugged an external monitor into my laptop with HDMI but it shows nothing | it_pc_peri_monitor | it · top it_pc_peri_monitor#1 0.96 recommended | route shallow |
| S-IT-028 | 사이트마다 비밀번호를 다르게 쓰면 다 기억할 수가 없어요. | it_sec_pw · no answer | it_sec_pw · top it_sec_pw#1 0.38 reference | answered unanswerable |
| S-IT-041 | 같은 브랜드로 가요. 옮기기 전에 백업은 어떻게 해 둬요? | it_mobile_move_android · no answer | it_mobile_move_android · top it_mobile_move_android#1 0.72 recommended | answered unanswerable |
| S-IT-043 | 아 잘못 말했어요, 새로 산 게 아이폰이 아니라 갤럭시예요. 그럼 어떻게 옮겨요? | it_mobile_move_cross · no answer | it_mobile_move_cross · top it_mobile_move_cross#1 0.38 reference | answered unanswerable |
| S-IT-046 | 아까 그 순서 문제 다시요. 마우스를 오른쪽 끝으로 밀면 왼쪽 화면에서 튀어나와요 | it_pc_peri_monitor · no answer | it_pc_peri · abstained · top it_pc_peri#article 0.19 reference | route shallow |
| S-IT-051 | 누가 제 이메일 계정 비번이랑 복구 메일까지 바꿔 놔서 들어갈 수가 없어요 | it_sec_hacked · no answer | it_sec_hacked · top it_sec_hacked#1 0.32 reference | answered unanswerable |
| S-IT-502 | 캡처하면 바로 파일로 저장되는 키 뭐였지ㅋㅋ 맨날 그림판에 붙여넣기 귀찮음 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.30 reference | answered unanswerable |
| S-IT-503 | 전체 화면 말고 지금 띄워 놓은 창 하나만 찍고 싶어요 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.77 recommended | answered unanswerable |
| S-IT-504 | 클립보드 기록 | it_pc_win_keys · no answer | it_pc_win_keys · top it_pc_win_keys#win_shift_s 0.82 recommended | answered unanswerable |
| S-IT-505 | windows shortcut to create a new virtual desktop | it_pc_win_keys · no answer | it · abstained · top it_pc_win#article 0.12 reference | route shallow |
| S-IT-507 | 작업관리자 바로 뜨게하는 단추키 머에요 | it_pc_win_keys · no answer | it_pc_win · top it_pc_win_perf_slow#1 0.71 recommended | route shallow, answered unanswerable |
| S-IT-512 | Win+D 말고, 창을 전부 최소화했다가 따로 되돌리는 키가 있다던데요 | it_pc_win_keys · no answer | it_pc_win · abstained · top it_pc_win#article 0.07 reference | route shallow |
| S-IT-516 | 그럼 녹화는요? | it_pc_win_keys · no answer | it · abstained · top it_pc_win#article 0.12 reference | route shallow |
| S-IT-518 | 부서가 영업이면서 직급이 과장인 사람이 몇 명인지 세고 싶어요 | it_office_excel_func · no answer | work · abstained · top work_leave_sick#1 0.17 reference | route domain |
| S-IT-519 | B열에 '완료'라고 적힌 칸이 몇 개인지만 알면 돼요 | it_office_excel_func · no answer | it_office · abstained · top it_office_excel#article 0.10 reference | route shallow |
| S-IT-520 | VLOOKUP 말고요, 찾는 열이 가져올 열보다 오른쪽에 있을 때는 어떻게 해요? | it_office_excel_func · no answer | it · abstained · top it_office_excel#article 0.16 reference | route shallow |
| S-IT-521 | 근무일수 계산 함수 | it_office_excel_func · no answer | work_leave · top work_leave_event#1 0.40 reference | route domain, answered unanswerable |
| S-IT-522 | excel formula to add up values that meet several criteria at once | it_office_excel_func · no answer | it_office · abstained · top it_office_excel#article 0.13 reference | route shallow |
| S-IT-524 | 입사일만 넣으면 몇 년 다녔는지 자동으로 나오게 하고 싶은데ㅠ 방법 없나 | it_office_excel_func · no answer | work_hr · abstained · top work_hr#article 0.06 reference | route domain |
| S-IT-525 | lookup 했는데 값이 없으면 #N/A 대신 blank나 '없음'으로 보이게 하는 function 있어요? | it_office_excel_func · no answer | it_office_excel · abstained · top it_office_excel#article 0.06 reference | route shallow |
| S-IT-526 | 셀에 있는 이메일 주소에서 골뱅이 앞 아이디 부분만 떼어 내고 싶어요 | it_office_excel_func · no answer | it · abstained · top it_office_excel#article 0.14 reference | route shallow |
| S-IT-527 | 매달 거래처별 매출 파일을 받는데 거래처 이름이 수백 줄에 중복돼서 들어가 있어요. 보고서 첫 장에 거래처 목록을 한 번씩만 깔끔하게 뽑아 두고 | it_office_excel_func · no answer | it_office_excel · abstained · top it_office_excel#article 0.11 reference | route shallow |
| S-IT-528 | 4.56을 5로, 4.44는 4로 만드는 것처럼 소수점 첫째 자리에서 반올림해 정수로 만들려면요? | it_office_excel_func · no answer | null · abstained · top no items | route false_none |
| S-IT-529 | 텍스트 합치기 구분자 넣어서 | it_office_excel_func · no answer | null · abstained · top no items | route false_none |
| S-LIFE-001 | 드럼세탁기 코스가 다 끝났는데 옷이 물에 푹 젖은 채로 나와요. 탈수가 안 된 것 같아요 | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.77 recommended | answered unanswerable |
| S-LIFE-003 | 드럼 돌린 빨래에서 쉰내 개심함.. 세제 많이 넣어서 그런가 | life_appl_wash_wm_drum_care_smell · no answer | life_appl_wash_wm_drum_care_smell · top life_appl_wash_wm_drum_care_smell#1 0.64 alternative | answered unanswerable |
| S-LIFE-006 | 통돌이 세탁기로 빨래하면 검은 찌꺼기가 뭍어나와여 청소 어케해요 | life_appl_wash_wm_top_clean · no answer | life_appl_wash_wm_top_clean · top life_appl_wash_wm_top_clean#1 0.82 recommended | answered unanswerable |
| S-LIFE-018 | 에어컨 필터는 몇 주에 한 번 씻어야 하는지, 그리고 공기청정기 필터는 언제 새로 사야 하는지 둘 다 알려주세요 | life_appl_air_ac (+1) | life_appl_air · top life_appl_air_purifier#1 0.51 alternative | route shallow |
| S-LIFE-029 | 피요. 이미 말라서 딱딱하게 굳었어요 | life_clean_stain · no answer | life_clean · abstained · top life_clean_stain#1 0.22 reference | route shallow |
| S-LIFE-030 | 빨래를 조금만 넣었을 때 탈수가 잘 안 되는데 왜 그래요? | life_appl_wash_wm_drum_water_spin · no answer | life_appl_wash_wm_drum_water_spin · top life_appl_wash_wm_drum_water_spin#1 0.80 recommended | answered unanswerable |
| S-LIFE-035 | 그럼 아까 그거 빨고 나서 방에서 말릴 때 냄새 안 나게 하려면 어떻게 해요? | life_clean_towel · no answer | life_clean_towel · top life_clean_towel#1 0.54 alternative | answered unanswerable |
| S-LIFE-036 | 겨울만 되면 거실 창틀이 곰팡이로 까매져요 | life_clean_mold · no answer | life_clean_mold · top life_clean_mold#1 0.38 reference | answered unanswerable |
| S-LIFE-038 | 세면대 물이 고인 채로 한참 걸려서 빠져요 | life_home_drain · no answer | life_home_drain · top life_home_drain#1 0.32 reference | answered unanswerable |
| S-MONEY-002 | 통장 만든 지 일주일 됐는데 하루에 30만 원까지만 보내져요. 앱에서 이체 한도 올리기를 눌러 봐도 안 되는데 이거 계좌 자체가 묶인 건가요? | money_bank_acct_limit · no answer | money_bank_acct_limit · top money_bank_acct_limit#1 0.34 reference | answered unanswerable |
| S-MONEY-005 | 인터넷뱅킹으로 계좌이체할 때 하루 한도를 늘리려면 OTP가 꼭 있어야 하나요? | money_bank_xfer_cap · no answer | money_bank_xfer_cap · top money_bank_xfer_cap#1 0.69 recommended | answered unanswerable |
| S-MONEY-007 | 공동인증서 expire 되기 전에 renew 하려면 어디서 해요? | money_bank_cert | it_sec · abstained · top it_sec#article 0.07 reference | route domain, abstained |
| S-MONEY-008 | 해외 결제 수수료 물어보려는 게 아니에요. 카드는 제 지갑에 그대로 있는데 제가 쓰지도 않은 해외 결제 승인 문자가 왔어요 | money_card_lost · no answer | money_card · top money_card_lost#1 0.70 recommended | route shallow, answered unanswerable |
| S-MONEY-012 | 온라인 쇼핑몰에서 결제하는데 한도 초과라고 승인이 안 나요. 쇼핑몰 오류가 아니라 제 신용카드 한도가 찬 거라는데 지금 바로 결제하려면 어떻게  | money_card_limit · no answer | shop_order_pay · abstained · top shop_order_pay#1 0.06 reference | route domain |
| S-MONEY-017 | 직장인인데 주말마다 과외해서 번 돈이 좀 있어요. 이것도 회사 연말정산으로 끝나나요, 아니면 제가 따로 신고해야 하나요? | money_tax_income · no answer | money_tax_income · top money_tax_income#1 0.44 alternative | answered unanswerable |
| S-MONEY-019 | 이체 한도도 올려야 하고 공동인증서도 곧 만료라 갱신해야 해요. 둘 다 어떻게 하나요? | money_bank_xfer_cap (+1) | money_bank · top money_bank_cert#1 0.61 alternative | route shallow |
| S-MONEY-025 | 근데 회사 제출 기한을 벌써 넘겼으면 그건 이제 못 받는 거예요? | money_tax_yearend · no answer | money_tax · abstained · top money_tax#article 0.12 reference | route shallow |
| S-MONEY-027 | 마통은요? | money_loan_credit · no answer | money_loan_credit · top money_loan_credit#1 0.56 alternative | answered unanswerable |
| S-MONEY-033 | 한도를 당장 다시 살리는 방법은 없어요? | money_card_limit · no answer | money_card_limit · top money_card_limit#1 0.38 reference | answered unanswerable |
| S-MONEY-034 | 아직 회사에서 재직증명서 같은 걸 못 받는데, 그런 서류 없이 푸는 방법은 없어요? | money_bank_acct_limit · no answer | money_bank_acct_limit · top money_bank_acct_limit#1 0.58 alternative | answered unanswerable |
| S-MONEY-038 | 쇼핑몰에서 카드로 산 옷을 반품했는데 환불이 카드에 언제 반영돼요? | null · no answer | money_card · abstained · top money_card#article 0.05 reference | route false_tree |
| S-MONEY-039 | 휴면 예금이 서민금융진흥원으로 넘어갔다는데 그럼 이제 못 돌려받는 거예요? | money_bank_acct_dormant · no answer | money_bank_acct_dormant · top money_bank_acct_dormant#1 0.32 reference | answered unanswerable |

