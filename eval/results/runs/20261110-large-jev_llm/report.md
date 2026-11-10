# 평가 결과 — 20261110-large-jev_llm

- 실행: 2026-11-10T12:13:15Z → 2026-11-10T12:21:14Z (UTC) · git `86af620` (커밋 안 된 변경 있음)
- 데이터셋: `large-kb` v1.0.0 · kb `98e745ae3b9e` · cases `c132143eb41e`
- 설정: routers ["jev_llm"] · variants ["full"] · suites ["all"] · split all · repeat 1 · concurrency 6 · beam_width "case/default" · limit "case/default"
- 모델(jev_llm): jev `jev-latest` · LLM `meta/muse-ember-1.0`
- 필터: --sample 1500 --seed final
- 메모: 0.11.0 (large): Jev + muse-ember-1.0 (low) reviews picks below 0.8

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev_llm | full | 1436 | 87.5% | 86.2% | 0.901 | 89.1% | 0.911 | 0.905 | 0.871 | 25.0% | 1.5% | 0.8s / 5.9s | 3.8 | 0 | 78.1% | - |

## jev_llm · full

### 검색 (1436건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 87.5% [85.7–89.1] (n=1436) |
|   └ 답할 수 있는 질문 | 88.7% [86.9–90.3] (n=1316) |
|   └ 답할 수 없는 질문(보류가 정답) | 75.0% [66.6–81.9] (n=120) |
| 확신 정답률 (top1 정답 + recommended) | 88.1% [86.2–89.7] (n=1316) |
| 라우팅 정확도 (정확 일치) | 86.2% [84.3–87.9] (n=1436) |
| 계층 F1 (hF1) | 0.901 [0.887–0.915] (n=1436) |
| 트리 밖 판정 precision | 92.4% [83.5–96.7] (n=66) |
| 트리 밖 판정 recall | 82.4% [72.2–89.4] (n=74) |
| 트리 밖 판정 F1 | 0.871 |
| 오보류율 (답할 수 있는데 보류) | 1.5% [1.0–2.3] (n=1316) |
| 오답변율 (답할 수 없는데 답함) | 25.0% [18.1–33.4] (n=120) |
| Hit@1 | 89.1% [87.3–90.7] (n=1316) |
| Hit@3 | 93.1% [91.6–94.3] (n=1316) |
| Hit@k (limit) | 94.3% [92.9–95.4] (n=1316) |
| MRR@k | 0.911 [0.897–0.926] (n=1316) |
| nDCG@k | 0.905 [0.891–0.918] (n=1316) |
| Recall@k | 0.943 [0.930–0.956] (n=1316) |
| recommended 정밀도 | 0.823 [0.806–0.841] (n=1297) |
| 라우팅이 맞았을 때 Hit@1 | 96.5% [95.3–97.4] (n=1151) |
| 라우팅이 맞았을 때 MRR | 0.978 [0.971–0.985] (n=1151) |
| 지연 p50 / p90 / p95 / 평균 | 0.8s / 3.9s / 5.9s / 1.9s |
| 라우팅 구간 평균 | 1.6s |
| 랭킹 후보 수 평균 | 45.8 |
| Jev 입력 토큰 평균 (과금) | 39384 |
| LLM 토큰 평균 (입력+출력) | 2388 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 3.81 |
| LLM 라우팅 호출 수 평균 | 0.30 |
| LLM이 경로를 정한 비율 (Jev가 불확실해 리뷰) | 29.2% [26.9–31.6] (n=1436) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 94.3% (n=1362) | 89.9% (n=1362) | 87.3% (n=1261) | 82.0% (n=383) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|---:|
| 77 | 6 | 77 | 1288 | 5 | 13 | 34 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 196 | 68.4% | 84.2% | 69.4% | 0.770 | 0.9s |
| leaf | 1120 | 92.2% | 88.0% | 92.6% | 0.936 | 0.8s |
| no_answer | 46 | 41.3% | 56.5% | - | - | 3.0s |
| oos | 74 | 95.9% | 82.4% | - | - | 0.7s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 162 | 96.3% | 93.8% | 99.3% | 0.993 | 0.7s |
| single | 1274 | 86.4% | 85.2% | 87.8% | 0.900 | 0.8s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 1274 | 86.4% | 85.2% | 87.8% | 0.900 | 0.8s |
| coref | 29 | 100.0% | 93.1% | 100.0% | 1.000 | 0.7s |
| correct | 29 | 96.6% | 96.6% | 100.0% | 1.000 | 0.7s |
| ellipsis | 21 | 100.0% | 100.0% | 100.0% | 1.000 | 0.7s |
| refine | 33 | 97.0% | 97.0% | 100.0% | 1.000 | 0.7s |
| shift | 30 | 86.7% | 90.0% | 96.2% | 0.962 | 0.8s |
| slot | 20 | 100.0% | 85.0% | 100.0% | 1.000 | 0.7s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 1436 | 87.5% | 86.2% | 89.1% | 0.911 | 0.8s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 196 | 68.4% | 84.2% | 69.4% | 0.770 | 0.9s |
| leaf | 1166 | 90.2% | 86.8% | 92.6% | 0.936 | 0.8s |
| none | 74 | 95.9% | 82.4% | - | - | 0.7s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 74 | 95.9% | 82.4% | - | - | 0.7s |
| d2 | 101 | 72.3% | 85.1% | 73.3% | 0.802 | 0.9s |
| d3 | 878 | 88.4% | 88.5% | 90.5% | 0.921 | 0.8s |
| d4 | 383 | 88.0% | 82.0% | 90.3% | 0.919 | 0.8s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 74 | 95.9% | 82.4% | - | - | 0.7s |
| 101+ | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 3.0s |
| 2-5 | 159 | 92.5% | 90.6% | 94.1% | 0.944 | 0.8s |
| 33-100 | 151 | 71.5% | 85.4% | 72.7% | 0.792 | 0.9s |
| 6-32 | 1048 | 88.5% | 85.9% | 90.8% | 0.924 | 0.8s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| colloquial | 263 | 90.9% | 90.1% | 93.6% | 0.948 | 0.7s |
| condition | 95 | 96.8% | 93.7% | 97.8% | 0.987 | 0.8s |
| english | 45 | 88.9% | 75.6% | 88.6% | 0.886 | 0.9s |
| keyword | 100 | 88.0% | 90.0% | 90.7% | 0.935 | 0.8s |
| long | 166 | 95.8% | 95.2% | 99.4% | 0.997 | 0.7s |
| mixed | 55 | 96.4% | 92.7% | 96.2% | 0.972 | 0.8s |
| negation | 39 | 94.9% | 84.6% | 97.3% | 0.973 | 0.8s |
| oos_far | 41 | 97.6% | 85.4% | - | - | 0.7s |
| oos_near | 33 | 93.9% | 78.8% | - | - | 0.9s |
| paraphrase | 333 | 84.1% | 76.6% | 85.9% | 0.869 | 1.0s |
| typo | 70 | 91.4% | 92.9% | 96.9% | 0.982 | 0.7s |
| vague | 196 | 68.4% | 84.2% | 69.4% | 0.770 | 0.9s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| abroad | 38 | 86.8% | 84.2% | 89.2% | 0.914 | 0.8s |
| appliance | 39 | 89.7% | 87.2% | 92.1% | 0.928 | 0.7s |
| bank | 51 | 92.2% | 90.2% | 94.0% | 0.947 | 0.7s |
| car | 41 | 92.7% | 92.7% | 94.7% | 0.963 | 0.8s |
| card | 41 | 87.8% | 87.8% | 87.2% | 0.880 | 0.8s |
| career | 50 | 82.0% | 82.0% | 89.1% | 0.913 | 0.8s |
| civil | 42 | 81.0% | 85.7% | 82.5% | 0.887 | 0.7s |
| cook | 39 | 87.2% | 87.2% | 91.9% | 0.932 | 0.8s |
| fitness | 49 | 87.8% | 95.9% | 89.8% | 0.918 | 0.7s |
| health | 30 | 93.3% | 90.0% | 96.6% | 0.983 | 0.8s |
| homecare | 54 | 85.2% | 81.5% | 88.7% | 0.906 | 0.7s |
| house | 40 | 87.5% | 87.5% | 94.6% | 0.959 | 0.7s |
| hr | 36 | 94.4% | 91.7% | 94.4% | 0.954 | 0.7s |
| insure | 49 | 81.6% | 77.6% | 81.6% | 0.848 | 0.8s |
| itdesk | 51 | 84.3% | 82.4% | 85.7% | 0.884 | 1.0s |
| legal | 42 | 83.3% | 81.0% | 87.5% | 0.891 | 2.2s |
| loan | 36 | 94.4% | 83.3% | 94.4% | 0.944 | 1.0s |
| medical | 38 | 89.5% | 89.5% | 89.2% | 0.911 | 0.9s |
| member | 55 | 85.5% | 90.9% | 86.8% | 0.902 | 0.8s |
| net | 32 | 93.8% | 90.6% | 96.8% | 0.968 | 0.9s |
| office | 36 | 86.1% | 83.3% | 85.7% | 0.886 | 0.8s |
| oos | 74 | 95.9% | 82.4% | - | - | 0.7s |
| parent | 38 | 97.4% | 97.4% | 97.2% | 0.986 | 0.8s |
| payroll | 47 | 83.0% | 76.6% | 86.0% | 0.872 | 0.8s |
| pc | 30 | 86.7% | 93.3% | 86.7% | 0.911 | 0.8s |
| pension | 51 | 84.3% | 84.3% | 87.8% | 0.917 | 0.8s |
| pet | 36 | 91.7% | 88.9% | 91.7% | 0.921 | 0.8s |
| phone | 31 | 90.3% | 90.3% | 90.3% | 0.935 | 0.7s |
| returns | 53 | 81.1% | 81.1% | 84.0% | 0.873 | 0.7s |
| security | 37 | 75.7% | 75.7% | 77.8% | 0.801 | 1.0s |
| shop | 40 | 90.0% | 87.5% | 90.0% | 0.915 | 0.8s |
| tax | 28 | 85.7% | 89.3% | 85.7% | 0.876 | 0.8s |
| transit | 43 | 83.7% | 81.4% | 87.8% | 0.878 | 0.8s |
| utility | 39 | 89.7% | 94.9% | 91.9% | 0.935 | 0.7s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| hard | 196 | 68.4% | 84.2% | 69.4% | 0.770 | 0.9s |
| medium | 1240 | 90.6% | 86.5% | 92.6% | 0.936 | 0.8s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 978 | 87.6% | 86.6% | 89.7% | 0.918 | 0.8s |
| test | 458 | 87.3% | 85.4% | 87.9% | 0.897 | 0.8s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 86.1% | 99.6% | 85.8% | 46.7% |
| 0.25 | 86.2% | 99.6% | 85.9% | 45.8% |
| 0.30 | 86.4% | 99.6% | 86.1% | 43.3% |
| 0.35 | 86.6% | 99.6% | 86.2% | 40.8% |
| 0.40 | 87.3% | 99.5% | 87.0% | 32.5% |
| 0.50 | 87.5% | 98.9% | 87.7% | 27.5% |
| 0.60 | 87.7% | 97.8% | 88.8% | 20.8% |
| 0.65 | 87.5% | 96.9% | 89.4% | 18.3% |

### 저장 (ingest, 64건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 78.1% [66.6–86.5] (n=64) |
|   └ 올바른 카테고리에 저장 | 78.1% [66.6–86.5] (n=64) |
|   └ 트리/루트 밖 Q&A 거절 | - |
| 계층 F1 | 0.843 [0.764–0.922] (n=64) |
| 초안이 검색에 안 보임 | 100.0% [94.3–100.0] (n=64) |
| 게시 성공 (버전 +1) | 100.0% [94.3–100.0] (n=64) |
| 초안→게시 카테고리 유지 | 100.0% [94.3–100.0] (n=64) |
| 중복 탐지 precision | - |
| 중복 탐지 recall | - |
| 갱신/신규 판정 일치 | 100.0% [94.3–100.0] (n=64) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 0 |
| 게시 후 probe 검색 top-k 포함 | - |
| 게시 후 probe 검색 top1 답변 | - |
| 지연 p50 (초안+게시+probe) | 1.1s |
| LLM이 경로를 정한 비율 (Jev가 불확실해 리뷰) | 40.6% [29.5–52.9] (n=64) |

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| null | transit_07_01_3 | 3 |
| itdesk_06_01 | pc_02_05 | 2 |
| null | car | 2 |
| abroad_01_04_1 | abroad_02_05 | 1 |
| abroad_01_04_2 | abroad_01_04 | 1 |
| abroad_01_04_2 | abroad_02_01 | 1 |
| abroad_02_03 | abroad_01_03 | 1 |
| abroad_06_02_2 | legal_03 | 1 |
| abroad_07_02 | phone_07_03 | 1 |
| appliance_02_02 | appliance_02 | 1 |
| appliance_02_02 | appliance_02_01 | 1 |
| appliance_05_03 | parent_07_03_2 | 1 |
| appliance_06_05 | appliance_01_04 | 1 |
| appliance_07_03 | appliance_07 | 1 |
| bank_01_01_1 | civil_06_05 | 1 |
| bank_02 | bank_02_03 | 1 |
| bank_04_03_1 | security_02_02 | 1 |
| bank_05_05 | bank_06_04 | 1 |
| bank_05_05 | payroll_02 | 1 |
| car_01_02_1 | car_02_03 | 1 |

### 틀린 케이스 (267건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-APPLIANCE-005 | 문 닫힘 표시가 깜빡이면서 작동이 바로 멈춰요 | appliance_06_03 | appliance_01_02 | branch |
| I-CAR-002 | 내 차 다음 검사 유효기간은 어디서 확인하나요? | car_05_02_1 | car_05_02 | shallow |
| I-CAREER-003 | 이직이 잦은 경력 3년차가 임원 앞에서 퇴사 사유를 어떻게 말할까요? | career_04_02 | career_01_01_3 | branch |
| I-FITNESS-002 | 퇴근 후 밤 9시에 처음 러닝할 때 식사와 간격은 어떻게 두나요 | fitness_04_01 | fitness_08_04 | branch |
| I-HOUSE-004 | 임차권등기명령은 관할 법원에 직접 가야만 신청되나요? | house_05_02 | legal_05_02_1 | domain |
| I-INSURE-003 | 자동차등록증 사진을 업로드하라는 안내가 나왔을 때 제출 방법은요? | insure_02_05 | car_05_03 | domain |
| I-INSURE-005 | 정기보험 만기가 되었는데 받은 만기축하금이 없는 이유는 무엇인가요? | insure_04_02 | insure_07_04 | branch |
| I-LOAN-004 | 공동인증서 없이 간편인증으로 평가사 상세 보고서를 발급받는 순서가 궁금해요? | loan_01_05_2 | loan_01_04 | branch |
| I-LOAN-005 | 10년 단기로 갚을 때 원금균등의 이자 절감 체감이 큰 이유는 무엇인가요? | loan_03_04 | loan_06_01_2 | branch |
| I-PAYROLL-001 | 수습 3개월 동안 첫급여가 기본급의 90%로 지급되는 조건은? | payroll_02_02 | career_05_04 | domain |
| I-PAYROLL-003 | 산재 치료는 꼭 산재 지정병원에서 받아야 하나요? | payroll_07_04 | medical | domain |
| I-PENSION-002 | 본인 노령연금과 배우자 유족연금을 동시에 받을 수 있나요? | pension_03_03_1 | pension_03 | shallow |
| I-PENSION-004 | 육아휴직이나 장기휴직 기간도 DB형 근속연수에 포함되나요? | pension_05_01 | pension_04_02_2 | branch |
| I-SHOP-003 | 배송비 쿠폰과 상품 할인쿠폰을 같은 주문에 같이 쓸 수 있나요? | shop_03_04 | member_05_02_2 | domain |
| S-ABROAD-007 | 할인표로 산 비행기 타는 날 아침에 갑자기 못 가게 되면 낸 돈은 한 푼도 못 되돌려받나요? | abroad_01_04_1 | abroad_02_05 · top abroad_02_05#3 0.80 recommended | route branch, wrong top1 |
| S-ABROAD-009 | 왕복 귀국편 변경 차액 | abroad_01_04_2 | abroad_02_01 · top abroad_02_01#5 0.94 recommended | route branch, wrong top1 |
| S-ABROAD-010 | 12월 붐빌 때 날짜 바꾸기 편한 표 종류마다 비행 점수 쌓이는 퍼센트가 어느 정도 벌어지나요? | abroad_01_04_2 | abroad_01_04 · top abroad_01_04_2#5 0.80 recommended | route shallow |
| S-ABROAD-021 | 아기 비행기표에 로마자 이름 오기랑 태어난 날짜를 잘못 입력했는데 바로잡으려면 뭘 내야 할까요? | abroad_02_03 | abroad_01_03 · top abroad_02_03#7 0.89 recommended | route branch |
| S-ABROAD-072 | 어제 시내 환급처에서 카드 보증 걸고 돈 먼저 받았거든요 근데 출국할 때 세관 확인 깜빡하면 카드에서 다시 빠져나가는 거예요? | abroad_05_04 · no answer | abroad_05_04 · top abroad_05_04#10 0.74 recommended | answered unanswerable |
| S-ABROAD-081 | 이웃 세대에서 하는 인테리어 작업이 너무 시끄러워 밤에 눈 붙이기가 힘든데 어떻게 해결하고 보상받나요? | abroad_06_02_2 | legal_03 · top legal_07_03#1 0.70 recommended | route domain, wrong top1 |
| S-ABROAD-094 | 스마트폰 보상청구 순서 | abroad_07_02 | phone_07_03 · top phone_07_03#5 0.69 recommended | route domain, wrong top1 |
| S-APPLIANCE-017 | 그거 물 고이면 어떡해? 거기 잎채소랑 과일 고습 저습 어떻게 나눠 넣음? | appliance_02_02 | appliance_02_01 · top appliance_02_02#6 0.91 recommended | route branch |
| S-APPLIANCE-018 | 지난주 이사하고 냉장고 자리 잡았는데 문이 자꾸 저절로 벌어져요. 앞다리 수평이 안 맞는 느낌인데 기울기가 이상해요. 조절다리 어느 방향으로 돌 | appliance_02_02 · no answer | appliance_02 · top appliance_07_05#6 0.64 alternative | route shallow, answered unanswerable |
| S-APPLIANCE-065 | 초등생 애가 밤마다 야한 영상 멋대로 틀까 봐 걱정돼요 비번 눌러야 켜지도록 차단하는 기능 있어요? | appliance_05_03 | parent_07_03_2 · top parent_07_03_2#6 0.85 recommended | route domain, wrong top1 |
| S-APPLIANCE-083 | 깨끗이 훔쳐냈는데도 묵은 향이 가시지 않아요. 시판 약품 없이 평소 문 틈 벌려두고 바람 통하게 두는 요령이 궁금해요. | appliance_06_05 | appliance_01_04 · top appliance_01_04#5 0.66 recommended | route branch, wrong top1 |
| S-APPLIANCE-111 | 식기세척기 세척 결과가 이상해요 | appliance_07_03 | appliance_07 · top appliance_07#article 0.85 recommended | route shallow, wrong top1 |
| S-BANK-003 | 기한이 끝난 패스포트나 폰에 저장해둔 주민증 사진으로 통과될까요? | bank_01_01_1 · no answer | civil_06_05 · top civil_06_04#7 0.76 recommended | route domain, answered unanswerable |
| S-BANK-046 | 업무용이랑 일상용 기기를 번갈아 쓰는데 양쪽 모두에 인증수단을 함께 설정해 쓸 수 있을까요? | bank_04_03_1 | security_02_02 · top itdesk_01_06_1#7 0.74 recommended | route domain, wrong top1 |
| S-BANK-062 | 외국으로 돈 보내는 거 신청해놨는데 아직 발송되기 전이면 철회할 수 있어요? | bank_05_05 | bank_06_04 · top bank_05_05#6 0.95 recommended | route branch |
| S-BANK-063 | 월급 여러 명꺼 한 번에 보내기 직전인데 전부 취소됨? | bank_05_05 | payroll_02 · top bank_05_05#5 0.91 recommended | route domain |
| S-BANK-102 | 안 쓰는 계좌들 전체적으로 정리하고 싶어요 | bank_02 | bank_02_03 · top bank_02_03#2 0.87 recommended | route deep, wrong top1 |
| S-BANK-107 | 돈을 잘못 보낸 것 같은데 어떡하죠 | bank_05 | bank_05 · top bank_05_01#1 0.74 recommended | wrong top1 |
| S-CAR-003 | 타이어 공기압 경고등 대처 | car_01_02_1 | car_02_03 · top car_02_03#5 0.91 recommended | route branch, wrong top1 |
| S-CAR-038 | 어제 오후 3시쯤 대형마트 지하주차장에서 후진 빼다가 통로 지나던 차랑 부딪혔어요. 상대는 제가 100% 잘못이라는데 후진·문열림 과실비율표 어 | car_03_02 · no answer | insure_03_04 · top insure_03_04#2 0.86 recommended | route domain, answered unanswerable |
| S-CAR-108 | 과태료 관련해서 전반적으로 알려주세요 | car_06_03 | car_06 · top car_06#article 0.79 recommended | route shallow, wrong top1 |
| S-CARD-006 | 월급에서 집세랑 빚 갚고 남는 돈이 적다고 승인 안 났는데, 또 넣어보려면 뭘 준비해야 할까요? | card_01_01_3 | loan_02_04 · top loan_02_04#7 0.83 recommended | route domain, wrong top1 |
| S-CARD-061 | My card was declined at a restaurant for exceeding the limit, how can I raise it | card_05_02 | card_02_04 · top card_05_02#5 0.92 recommended | route branch |
| S-CARD-062 | 눈 맞추는 집에서 도수 알 다시 맞추느라 카드를 또 긁었는데, 처음에 빠져나간 돈이 되돌아왔는지 어떻게 알아보죠? | card_05_03 | card_05_03 · top card_05_03#21 0.73 recommended | wrong top1 |
| S-CARD-070 | 몇 달에 나눠서 갚는 걸로 긁었는데 저번 달 기준으로 채워야 하는 액수에 한 번에 전부 더해지나요? | card_06_01 | card_06_01 · top card_04_03#7 0.76 recommended | wrong top1 |
| S-CARD-077 | I want to keep using my own credit card but cancel only the supplementary card I | card_06_05 | card_01_03 · top card_06_05#7 0.94 recommended | route branch |
| S-CARD-086 | 인증 암호를 다섯 차례 잘못 눌러서 삼십 분 동안 이용이 정지됐는데 다시 쓸 수 있게 하려면 뭘 해야 돼요? | card_07_03 | member_01_05 · top member_01_05#2 1.02 recommended | route domain, wrong top1 |
| S-CARD-089 | 만원 넘게 담았는데 저장해둔 할인티켓이 금액 지불하는 창에 안 나타나면 뭐가 문제일까요? | card_07_05 | member_05_02_1 · top member_05_02_1#1 0.70 recommended | route domain, wrong top1 |
| S-CAREER-005 | 아이 키우느라 직장을 그만둔 뒤 열두 달 정도 일을 안 했는데, 경력서에 그 공백을 어떻게 풀어쓰면 좋을까요? | career_01_01_3 | career_01 · top career_01_01_3#4 0.85 recommended | route shallow |
| S-CAREER-016 | 월별로 틀리지 않게 정리하려면요? | career_01_05 | career_01_01_2 · top career_01_05#4 0.82 recommended | route branch |
| S-CAREER-017 | 밖에 있는데 마감이 얼마 안 남아서 폰으로 원서를 내야 해요. 번호랑 메일주소를 잘못 적을까봐 걱정인데 미리 체크할 요령 있을까요? | career_01_05 · no answer | career_01_05 · top career_01_05#9 0.66 recommended | answered unanswerable |
| S-CAREER-028 | 워드에서 쓴 글을 채용 사이트에 옮겼더니 용량이 더 커졌어요. 엔터로 띄운 빈 칸은 몇 줄로 맞추는 게 좋을까요? | career_02_05 | office_02_01_2 · top office_02_01_2#2 0.59 alternative | route domain, wrong top1 |
| S-CAREER-036 | 전부 맞추긴 어려울 것 같은데 조금이라도 건지려면 극단적인 입력값은 어떻게 챙기면 좋을까요? | career_03_03_2 | null · abstained · top security_04_03#2 0.41 reference | route false_none, abstained |
| S-CAREER-051 | 내일 final 임원 면접인데 company vision이랑 앞으로 2-3년 business 집중 과제에서 우리 team 역할이 뭔지 여쭤보고  | career_04_04_2 | career_04_04_2 · top career_04_04_2#10 0.97 recommended | wrong top1 |
| S-CAREER-055 | 작년 팀 과제할 때 상사랑 의견 충돌로 분위기가 안 좋았어요. 결국 제가 먼저 사과하고 조율했거든요. 면접에서 상사 갈등 사례 자세히 말해보라  | career_04_04_3 · no answer | career_04_02 · top career_04_02#6 0.65 recommended | route branch, answered unanswerable |
| S-CAREER-059 | 작년에 회사 주식 싸게 사서 판 차액이 컸는데, 새 곳에 얘기하는 받는 돈 셈할 때 그 일회성 돈까지 합해서 말해야 할까요? | career_05_01_1 | career_05_01 · abstained · top career_05_01_1#16 0.51 reference | route shallow, abstained |
| S-CAREER-060 | 옮겨갈 회사에서 예전에 다니던 곳 세금 종이 꼭 달라고 하는데, 이거 안 넘겨주면 법에 걸리거나 불이익 있나요? | career_05_01_1 | payroll_03_03 · top payroll_03_04_2#9 0.58 alternative | route domain, wrong top1 |
| S-CAREER-062 | 공단 홈페이지에서 뽑는 보험 가입 기록에 나와 있는 월급 금액으로 예전에 얼마 벌었는지 보여줘도 통과될까요? | career_05_01_1 · no answer | career_01_01_2 · abstained · top career_01_01_2#1 0.46 reference | route branch |
| S-CAREER-070 | 1년 계약했는데 수습 3개월은 월급 90%만 준대ㅠ 이거 최저임금 위반 아님? | career_05_04 | legal_04_04 · top legal_04_04#4 1.00 recommended | route domain, wrong top1 |
| S-CAREER-114 | 내일배움카드가 뭔가요? | career_07_03 | career_07_03 · abstained · top career_07_03#article 0.52 reference | abstained |
| S-CIVIL-013 | 발급받은 서류에 찍힌 태어난 날짜가 호적 기록이랑 엇갈리는데 바로잡을 수 있을까요? | civil_01_05 | civil_05_02 · top civil_05_02#11 0.88 recommended | route branch, wrong top1 |
| S-CIVIL-031 | 지난번에 여러 번 쓸 수 있는 걸로 넣었는데 손에 들어오기까지 며칠쯤 걸리고 찾을 곳을 다른 구청으로 고를 수 있나요? | civil_03_01_1 | civil_03_05 · top civil_03_01_1#5 0.89 recommended | route branch |
| S-CIVIL-035 | 아파서 외국 병원 진료가야됨 1회용으로 끊으려면 뭐 내야돼? 보호자도 같이 가는데 | civil_03_01_2 | abroad_07_04 · top abroad_07_04#6 0.65 alternative | route domain, wrong top1 |
| S-CIVIL-042 | 국내에서 잃어버린 게 아니라 해외여행 중에 여권 잃어버렸는데 현지 대사관에서 어떻게 다시 받나요? | civil_03_03_2 · no answer | abroad_07_03 · top abroad_07_03#1 0.86 recommended | route domain, answered unanswerable |
| S-CIVIL-046 | 여권 어제 신청했는데 취소하면 낸 돈 돌려받을 수 있음? | civil_03_05 | civil_03_05 · top civil_03_05#3 1.05 recommended | wrong top1 |
| S-CIVIL-086 | I already set up my certificate on my home desktop, can I copy that same one to  | civil_07_01_2 | bank_07_02 · top civil_07_01_2#11 0.94 recommended | route domain |

