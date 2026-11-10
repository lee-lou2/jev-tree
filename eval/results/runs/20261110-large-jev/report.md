# 평가 결과 — 20261110-large-jev

- 실행: 2026-11-10T11:59:56Z → 2026-11-10T12:04:15Z (UTC) · git `501a5f6` (커밋 안 된 변경 있음)
- 데이터셋: `large-kb` v1.0.0 · kb `98e745ae3b9e` · cases `c132143eb41e`
- 설정: routers ["jev"] · variants ["full"] · suites ["all"] · split all · repeat 1 · concurrency 5 · beam_width "case/default" · limit "case/default"
- 모델(jev): jev `jev-latest`
- 필터: --sample 1500 --seed final
- 메모: 0.11.0 (large): hints, word-match pool, hold 0.55

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev | full | 1436 | 86.8% | 82.7% | 0.875 | 88.1% | 0.903 | 0.895 | 0.851 | 23.3% | 2.4% | 0.8s / 1.2s | 3.9 | 0 | 79.7% | - |

## jev · full

### 검색 (1436건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 86.8% [85.0–88.5] (n=1436) |
|   └ 답할 수 있는 질문 | 87.8% [85.9–89.4] (n=1316) |
|   └ 답할 수 없는 질문(보류가 정답) | 76.7% [68.3–83.3] (n=120) |
| 확신 정답률 (top1 정답 + recommended) | 87.2% [85.2–88.9] (n=1316) |
| 라우팅 정확도 (정확 일치) | 82.7% [80.7–84.6] (n=1436) |
| 계층 F1 (hF1) | 0.875 [0.860–0.891] (n=1436) |
| 트리 밖 판정 precision | 89.6% [80.0–94.8] (n=67) |
| 트리 밖 판정 recall | 81.1% [70.7–88.4] (n=74) |
| 트리 밖 판정 F1 | 0.851 |
| 오보류율 (답할 수 있는데 보류) | 2.4% [1.7–3.3] (n=1316) |
| 오답변율 (답할 수 없는데 답함) | 23.3% [16.7–31.7] (n=120) |
| Hit@1 | 88.1% [86.3–89.8] (n=1316) |
| Hit@3 | 92.6% [91.0–93.9] (n=1316) |
| Hit@k (limit) | 93.6% [92.2–94.8] (n=1316) |
| MRR@k | 0.903 [0.888–0.918] (n=1316) |
| nDCG@k | 0.895 [0.881–0.909] (n=1316) |
| Recall@k | 0.936 [0.923–0.949] (n=1316) |
| recommended 정밀도 | 0.819 [0.801–0.837] (n=1282) |
| 라우팅이 맞았을 때 Hit@1 | 97.3% [96.1–98.1] (n=1102) |
| 라우팅이 맞았을 때 MRR | 0.983 [0.977–0.989] (n=1102) |
| 지연 p50 / p90 / p95 / 평균 | 0.8s / 1.0s / 1.2s / 0.8s |
| 라우팅 구간 평균 | 0.6s |
| 랭킹 후보 수 평균 | 49.0 |
| Jev 입력 토큰 평균 (과금) | 40238 |
| LLM 토큰 평균 (입력+출력) | 0 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 3.92 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM이 경로를 정한 비율 (Jev가 불확실해 리뷰) | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 92.3% (n=1362) | 87.2% (n=1362) | 83.7% (n=1261) | 80.9% (n=383) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|---:|
| 82 | 6 | 102 | 1239 | 7 | 14 | 50 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 196 | 63.3% | 77.0% | 64.3% | 0.735 | 0.8s |
| leaf | 1120 | 92.1% | 84.9% | 92.3% | 0.933 | 0.8s |
| no_answer | 46 | 43.5% | 56.5% | - | - | 0.8s |
| oos | 74 | 97.3% | 81.1% | - | - | 0.8s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 162 | 96.3% | 93.8% | 100.0% | 1.000 | 0.8s |
| single | 1274 | 85.6% | 81.3% | 86.6% | 0.890 | 0.8s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 1274 | 85.6% | 81.3% | 86.6% | 0.890 | 0.8s |
| coref | 29 | 100.0% | 96.6% | 100.0% | 1.000 | 0.7s |
| correct | 29 | 93.1% | 96.6% | 100.0% | 1.000 | 0.8s |
| ellipsis | 21 | 100.0% | 100.0% | 100.0% | 1.000 | 0.8s |
| refine | 33 | 97.0% | 93.9% | 100.0% | 1.000 | 0.8s |
| shift | 30 | 90.0% | 90.0% | 100.0% | 1.000 | 0.8s |
| slot | 20 | 100.0% | 85.0% | 100.0% | 1.000 | 0.7s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 1436 | 86.8% | 82.7% | 88.1% | 0.903 | 0.8s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 196 | 63.3% | 77.0% | 64.3% | 0.735 | 0.8s |
| leaf | 1166 | 90.1% | 83.8% | 92.3% | 0.933 | 0.8s |
| none | 74 | 97.3% | 81.1% | - | - | 0.8s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 74 | 97.3% | 81.1% | - | - | 0.8s |
| d2 | 101 | 70.3% | 81.2% | 71.3% | 0.782 | 0.8s |
| d3 | 878 | 87.0% | 83.8% | 89.1% | 0.910 | 0.8s |
| d4 | 383 | 88.8% | 80.9% | 90.6% | 0.920 | 0.8s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 74 | 97.3% | 81.1% | - | - | 0.8s |
| 101+ | 4 | 75.0% | 75.0% | 75.0% | 0.792 | 1.1s |
| 2-5 | 159 | 91.8% | 86.2% | 94.1% | 0.941 | 0.8s |
| 33-100 | 151 | 69.5% | 80.1% | 71.3% | 0.779 | 0.8s |
| 6-32 | 1048 | 87.9% | 82.7% | 89.8% | 0.916 | 0.8s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| colloquial | 263 | 91.3% | 89.7% | 94.0% | 0.950 | 0.8s |
| condition | 95 | 95.8% | 90.5% | 96.8% | 0.982 | 0.8s |
| english | 45 | 86.7% | 73.3% | 86.4% | 0.864 | 0.8s |
| keyword | 100 | 89.0% | 87.0% | 91.8% | 0.941 | 0.8s |
| long | 166 | 95.2% | 92.8% | 98.1% | 0.986 | 0.8s |
| mixed | 55 | 96.4% | 90.9% | 96.2% | 0.962 | 0.8s |
| negation | 39 | 94.9% | 84.6% | 97.3% | 0.973 | 0.7s |
| oos_far | 41 | 97.6% | 87.8% | - | - | 0.7s |
| oos_near | 33 | 97.0% | 72.7% | - | - | 0.9s |
| paraphrase | 333 | 84.1% | 69.7% | 85.6% | 0.866 | 0.8s |
| typo | 70 | 91.4% | 94.3% | 96.9% | 0.982 | 0.7s |
| vague | 196 | 63.3% | 77.0% | 64.3% | 0.735 | 0.8s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| abroad | 38 | 86.8% | 89.5% | 89.2% | 0.905 | 1.0s |
| appliance | 39 | 89.7% | 87.2% | 92.1% | 0.925 | 0.9s |
| bank | 51 | 92.2% | 90.2% | 94.0% | 0.947 | 1.0s |
| car | 41 | 92.7% | 90.2% | 94.7% | 0.963 | 0.9s |
| card | 41 | 85.4% | 78.0% | 84.6% | 0.846 | 0.9s |
| career | 50 | 88.0% | 82.0% | 91.3% | 0.924 | 0.8s |
| civil | 42 | 78.6% | 83.3% | 80.0% | 0.873 | 0.8s |
| cook | 39 | 89.7% | 92.3% | 94.6% | 0.959 | 0.7s |
| fitness | 49 | 89.8% | 91.8% | 89.8% | 0.919 | 0.8s |
| health | 30 | 96.7% | 93.3% | 96.6% | 0.983 | 0.7s |
| homecare | 54 | 90.7% | 85.2% | 94.3% | 0.955 | 0.7s |
| house | 40 | 90.0% | 82.5% | 97.3% | 0.973 | 0.7s |
| hr | 36 | 94.4% | 86.1% | 94.4% | 0.954 | 0.7s |
| insure | 49 | 81.6% | 79.6% | 81.6% | 0.848 | 0.8s |
| itdesk | 51 | 82.4% | 82.4% | 83.7% | 0.888 | 0.7s |
| legal | 42 | 83.3% | 76.2% | 87.5% | 0.900 | 0.7s |
| loan | 36 | 88.9% | 77.8% | 88.9% | 0.894 | 0.7s |
| medical | 38 | 84.2% | 73.7% | 83.8% | 0.860 | 0.7s |
| member | 55 | 83.6% | 83.6% | 84.9% | 0.885 | 0.7s |
| net | 32 | 90.6% | 81.2% | 93.5% | 0.935 | 0.8s |
| office | 36 | 88.9% | 80.6% | 88.6% | 0.914 | 0.7s |
| oos | 74 | 97.3% | 81.1% | - | - | 0.8s |
| parent | 38 | 97.4% | 92.1% | 97.2% | 0.981 | 0.9s |
| payroll | 47 | 74.5% | 70.2% | 79.1% | 0.822 | 0.9s |
| pc | 30 | 90.0% | 83.3% | 90.0% | 0.939 | 0.8s |
| pension | 51 | 78.4% | 78.4% | 83.7% | 0.878 | 0.8s |
| pet | 36 | 88.9% | 80.6% | 88.9% | 0.892 | 0.7s |
| phone | 31 | 77.4% | 74.2% | 77.4% | 0.822 | 0.7s |
| returns | 53 | 75.5% | 73.6% | 80.0% | 0.833 | 0.8s |
| security | 37 | 78.4% | 75.7% | 80.6% | 0.829 | 0.7s |
| shop | 40 | 90.0% | 82.5% | 90.0% | 0.915 | 0.8s |
| tax | 28 | 89.3% | 82.1% | 89.3% | 0.905 | 0.8s |
| transit | 43 | 81.4% | 83.7% | 85.4% | 0.866 | 0.7s |
| utility | 39 | 89.7% | 92.3% | 89.2% | 0.928 | 0.8s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| hard | 196 | 63.3% | 77.0% | 64.3% | 0.735 | 0.8s |
| medium | 1240 | 90.6% | 83.6% | 92.3% | 0.933 | 0.8s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 978 | 87.1% | 83.1% | 88.8% | 0.909 | 0.8s |
| test | 458 | 86.2% | 81.9% | 86.7% | 0.890 | 0.8s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 85.2% | 99.5% | 84.9% | 47.5% |
| 0.25 | 85.4% | 99.5% | 85.0% | 45.0% |
| 0.30 | 85.5% | 99.5% | 85.2% | 43.3% |
| 0.35 | 86.0% | 99.1% | 86.0% | 37.5% |
| 0.40 | 86.5% | 98.9% | 86.6% | 31.7% |
| 0.50 | 86.8% | 98.3% | 87.5% | 25.0% |
| 0.60 | 87.0% | 97.4% | 88.3% | 20.0% |
| 0.65 | 86.8% | 95.8% | 89.5% | 17.5% |

### 저장 (ingest, 64건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 79.7% [68.3–87.7] (n=64) |
|   └ 올바른 카테고리에 저장 | 79.7% [68.3–87.7] (n=64) |
|   └ 트리/루트 밖 Q&A 거절 | - |
| 계층 F1 | 0.864 [0.790–0.938] (n=64) |
| 초안이 검색에 안 보임 | 100.0% [94.3–100.0] (n=64) |
| 게시 성공 (버전 +1) | 100.0% [94.3–100.0] (n=64) |
| 초안→게시 카테고리 유지 | 100.0% [94.3–100.0] (n=64) |
| 중복 탐지 precision | - |
| 중복 탐지 recall | - |
| 갱신/신규 판정 일치 | 100.0% [94.3–100.0] (n=64) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 0 |
| 게시 후 probe 검색 top-k 포함 | - |
| 게시 후 probe 검색 top1 답변 | - |
| 지연 p50 (초안+게시+probe) | 1.2s |
| LLM이 경로를 정한 비율 (Jev가 불확실해 리뷰) | - |

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| null | transit_07 | 3 |
| null | house | 2 |
| abroad_01_04_1 | abroad_02 | 1 |
| abroad_01_04_2 | abroad_02_01 | 1 |
| abroad_03 | null | 1 |
| abroad_06_02_2 | legal_03 | 1 |
| appliance_02_02 | appliance_02 | 1 |
| appliance_02_02 | appliance_02_01 | 1 |
| appliance_05_03 | parent_07_03_2 | 1 |
| appliance_06_05 | appliance_01_04 | 1 |
| appliance_07_03 | appliance_07 | 1 |
| bank_01_01_1 | abroad_04 | 1 |
| bank_03_02 | bank_04_04 | 1 |
| bank_04_03_1 | security_02 | 1 |
| bank_05_05 | bank_06_04 | 1 |
| bank_05_05 | payroll | 1 |
| car_01_02_1 | car_02_03 | 1 |
| car_03_02 | insure_03_04 | 1 |
| car_04_02 | career_03_03 | 1 |
| car_06_03 | car_06 | 1 |

### 틀린 케이스 (307건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-APPLIANCE-005 | 문 닫힘 표시가 깜빡이면서 작동이 바로 멈춰요 | appliance_06_03 | appliance_01_02 | branch |
| I-CAR-002 | 내 차 다음 검사 유효기간은 어디서 확인하나요? | car_05_02_1 | car_05_02 | shallow |
| I-CAREER-003 | 이직이 잦은 경력 3년차가 임원 앞에서 퇴사 사유를 어떻게 말할까요? | career_04_02 | career_01_01_3 | branch |
| I-FITNESS-002 | 퇴근 후 밤 9시에 처음 러닝할 때 식사와 간격은 어떻게 두나요 | fitness_04_01 | fitness_08_04 | branch |
| I-INSURE-005 | 정기보험 만기가 되었는데 받은 만기축하금이 없는 이유는 무엇인가요? | insure_04_02 | insure_07_04 | branch |
| I-LOAN-004 | 공동인증서 없이 간편인증으로 평가사 상세 보고서를 발급받는 순서가 궁금해요? | loan_01_05_2 | bank_07_03 | domain |
| I-PAYROLL-001 | 수습 3개월 동안 첫급여가 기본급의 90%로 지급되는 조건은? | payroll_02_02 | career_05_04 | domain |
| I-PAYROLL-003 | 산재 치료는 꼭 산재 지정병원에서 받아야 하나요? | payroll_07_04 | medical | domain |
| I-PENSION-002 | 본인 노령연금과 배우자 유족연금을 동시에 받을 수 있나요? | pension_03_03_1 | pension_03 | shallow |
| I-PENSION-004 | 육아휴직이나 장기휴직 기간도 DB형 근속연수에 포함되나요? | pension_05_01 | pension_04_02_2 | branch |
| I-PHONE-002 | 밤에 스마트폰 화면이 너무 하얘서 따뜻한 색온도로 바꾸려면 어떻게 하나요? | phone_02_03_2 | phone_02_03 | shallow |
| I-SHOP-003 | 배송비 쿠폰과 상품 할인쿠폰을 같은 주문에 같이 쓸 수 있나요? | shop_03_04 | member_05_02_2 | domain |
| I-SHOP-005 | 국내상품과 해외직구 상품을 한 번에 같이 주문할 수 있나요? | shop_06_01 | shop_01_01 | branch |
| S-ABROAD-007 | 할인표로 산 비행기 타는 날 아침에 갑자기 못 가게 되면 낸 돈은 한 푼도 못 되돌려받나요? | abroad_01_04_1 | abroad_02 · top abroad_02_05#3 0.72 recommended | route branch, wrong top1 |
| S-ABROAD-009 | 왕복 귀국편 변경 차액 | abroad_01_04_2 | abroad_02_01 · top abroad_02_01#5 0.95 recommended | route branch, wrong top1 |
| S-ABROAD-072 | 어제 시내 환급처에서 카드 보증 걸고 돈 먼저 받았거든요 근데 출국할 때 세관 확인 깜빡하면 카드에서 다시 빠져나가는 거예요? | abroad_05_04 · no answer | abroad_05_04 · top abroad_05_04#10 0.72 recommended | answered unanswerable |
| S-ABROAD-081 | 이웃 세대에서 하는 인테리어 작업이 너무 시끄러워 밤에 눈 붙이기가 힘든데 어떻게 해결하고 보상받나요? | abroad_06_02_2 | legal_03 · top legal_03_04#1 0.59 alternative | route domain, wrong top1 |
| S-ABROAD-107 | 짐 싸는 게 걱정돼요 알려주세요 | abroad_03 | null · abstained · top fitness_03_04#17 0.26 reference | route false_none, abstained |
| S-APPLIANCE-017 | 그거 물 고이면 어떡해? 거기 잎채소랑 과일 고습 저습 어떻게 나눠 넣음? | appliance_02_02 | appliance_02_01 · top appliance_02_02#6 0.91 recommended | route branch |
| S-APPLIANCE-018 | 지난주 이사하고 냉장고 자리 잡았는데 문이 자꾸 저절로 벌어져요. 앞다리 수평이 안 맞는 느낌인데 기울기가 이상해요. 조절다리 어느 방향으로 돌 | appliance_02_02 · no answer | appliance_02 · top appliance_07_05#6 0.68 recommended | route shallow, answered unanswerable |
| S-APPLIANCE-065 | 초등생 애가 밤마다 야한 영상 멋대로 틀까 봐 걱정돼요 비번 눌러야 켜지도록 차단하는 기능 있어요? | appliance_05_03 | parent_07_03_2 · top parent_07_03_2#6 0.86 recommended | route domain, wrong top1 |
| S-APPLIANCE-083 | 깨끗이 훔쳐냈는데도 묵은 향이 가시지 않아요. 시판 약품 없이 평소 문 틈 벌려두고 바람 통하게 두는 요령이 궁금해요. | appliance_06_05 | appliance_01_04 · top appliance_01_04#5 0.66 recommended | route branch, wrong top1 |
| S-APPLIANCE-111 | 식기세척기 세척 결과가 이상해요 | appliance_07_03 | appliance_07 · top appliance_07#article 0.86 recommended | route shallow, wrong top1 |
| S-BANK-003 | 기한이 끝난 패스포트나 폰에 저장해둔 주민증 사진으로 통과될까요? | bank_01_01_1 · no answer | abroad_04 · top civil_06_04#7 0.70 recommended | route domain, answered unanswerable |
| S-BANK-034 | 미리 걸어둔 돈 보내기를 하루 전 늦은 저녁에 그만두거나 미루고 싶은데 어떻게 해요? | bank_03_02 | bank_04_04 · top bank_03_02#3 0.91 recommended | route branch |
| S-BANK-046 | 업무용이랑 일상용 기기를 번갈아 쓰는데 양쪽 모두에 인증수단을 함께 설정해 쓸 수 있을까요? | bank_04_03_1 | security_02 · top security_02_02_3#11 0.65 alternative | route domain, wrong top1 |
| S-BANK-062 | 외국으로 돈 보내는 거 신청해놨는데 아직 발송되기 전이면 철회할 수 있어요? | bank_05_05 | bank_06_04 · top bank_05_05#6 0.96 recommended | route branch |
| S-BANK-063 | 월급 여러 명꺼 한 번에 보내기 직전인데 전부 취소됨? | bank_05_05 | payroll · abstained · top payroll_04_05#7 0.40 reference | route domain, abstained |
| S-BANK-107 | 돈을 잘못 보낸 것 같은데 어떡하죠 | bank_05 | bank_05 · top bank_05_01#1 0.73 recommended | wrong top1 |
| S-CAR-003 | 타이어 공기압 경고등 대처 | car_01_02_1 | car_02_03 · top car_02_03#5 0.94 recommended | route branch, wrong top1 |
| S-CAR-038 | 어제 오후 3시쯤 대형마트 지하주차장에서 후진 빼다가 통로 지나던 차랑 부딪혔어요. 상대는 제가 100% 잘못이라는데 후진·문열림 과실비율표 어 | car_03_02 · no answer | insure_03_04 · top insure_03_04#2 0.84 recommended | route domain, answered unanswerable |
| S-CAR-047 | 필기 떨어졌어ㅠ 내일 바로 또 볼 수 있음? 횟수 제한 같은 거 있어? | car_04_02 | career_03_03 · top car_04_02#4 0.79 recommended | route domain |
| S-CAR-108 | 과태료 관련해서 전반적으로 알려주세요 | car_06_03 | car_06 · top car_06#article 0.79 recommended | route shallow, wrong top1 |
| S-CARD-006 | 월급에서 집세랑 빚 갚고 남는 돈이 적다고 승인 안 났는데, 또 넣어보려면 뭘 준비해야 할까요? | card_01_01_3 | loan_02_04 · top loan_02_04#7 0.84 recommended | route domain, wrong top1 |
| S-CARD-015 | 영업점에서 그 자리에서 바로 뽑는 게 아니라 앱으로 우편 재발급 신청하려는데 돈은 얼마나 나가고 도착까지 며칠 걸려요? | card_01_04 | civil_03_03_2 · top card_01_04#7 0.77 recommended | route domain |
| S-CARD-058 | 미국 직구 사이트에서 150만원짜리 카메라 결제하려는데 국내 한도는 남았는데 계속 거절돼요. 해외이용한도가 따로 있는 건가요, 이럴 땐 어떻게  | card_05_01_2 | card_02_05 · top card_05_01_2#2 0.90 recommended | route branch |
| S-CARD-061 | My card was declined at a restaurant for exceeding the limit, how can I raise it | card_05_02 | card_02_04 · top card_05_02#5 0.92 recommended | route branch |
| S-CARD-062 | 눈 맞추는 집에서 도수 알 다시 맞추느라 카드를 또 긁었는데, 처음에 빠져나간 돈이 되돌아왔는지 어떻게 알아보죠? | card_05_03 | null · abstained · top security_05_04#5 0.21 reference | route false_none, abstained |
| S-CARD-070 | 몇 달에 나눠서 갚는 걸로 긁었는데 저번 달 기준으로 채워야 하는 액수에 한 번에 전부 더해지나요? | card_06_01 | card_04_03 · top card_04_03#7 0.94 recommended | route branch, wrong top1 |
| S-CARD-077 | I want to keep using my own credit card but cancel only the supplementary card I | card_06_05 | card_01_03 · abstained · top card_01_03#3 0.34 reference | route branch, abstained |
| S-CARD-086 | 인증 암호를 다섯 차례 잘못 눌러서 삼십 분 동안 이용이 정지됐는데 다시 쓸 수 있게 하려면 뭘 해야 돼요? | card_07_03 | member_01_05 · top member_01_05#2 1.03 recommended | route domain, wrong top1 |
| S-CARD-089 | 만원 넘게 담았는데 저장해둔 할인티켓이 금액 지불하는 창에 안 나타나면 뭐가 문제일까요? | card_07_05 | member_05 · top member_05#article 0.62 alternative | route domain, wrong top1 |
| S-CAREER-005 | 아이 키우느라 직장을 그만둔 뒤 열두 달 정도 일을 안 했는데, 경력서에 그 공백을 어떻게 풀어쓰면 좋을까요? | career_01_01_3 | career_01 · top career_01_01_3#4 0.85 recommended | route shallow |
| S-CAREER-016 | 월별로 틀리지 않게 정리하려면요? | career_01_05 | career_01_01_2 · top career_01_05#4 0.82 recommended | route branch |
| S-CAREER-017 | 밖에 있는데 마감이 얼마 안 남아서 폰으로 원서를 내야 해요. 번호랑 메일주소를 잘못 적을까봐 걱정인데 미리 체크할 요령 있을까요? | career_01_05 · no answer | career_01_05 · top career_01_05#9 0.66 recommended | answered unanswerable |
| S-CAREER-028 | 워드에서 쓴 글을 채용 사이트에 옮겼더니 용량이 더 커졌어요. 엔터로 띄운 빈 칸은 몇 줄로 맞추는 게 좋을까요? | career_02_05 | office_02_01_2 · top office_02_01_2#2 0.63 alternative | route domain, wrong top1 |
| S-CAREER-036 | 전부 맞추긴 어려울 것 같은데 조금이라도 건지려면 극단적인 입력값은 어떻게 챙기면 좋을까요? | career_03_03_2 | null · abstained · top security_04_03#2 0.43 reference | route false_none, abstained |
| S-CAREER-055 | 작년 팀 과제할 때 상사랑 의견 충돌로 분위기가 안 좋았어요. 결국 제가 먼저 사과하고 조율했거든요. 면접에서 상사 갈등 사례 자세히 말해보라  | career_04_04_3 · no answer | career_04_04_3 · top career_04_01#6 0.70 recommended | answered unanswerable |
| S-CAREER-059 | 작년에 회사 주식 싸게 사서 판 차액이 컸는데, 새 곳에 얘기하는 받는 돈 셈할 때 그 일회성 돈까지 합해서 말해야 할까요? | career_05_01_1 | career_05_01 · top career_05_01_1#16 0.57 alternative | route shallow |
| S-CAREER-060 | 옮겨갈 회사에서 예전에 다니던 곳 세금 종이 꼭 달라고 하는데, 이거 안 넘겨주면 법에 걸리거나 불이익 있나요? | career_05_01_1 | payroll_03_04_2 · top payroll_03_04_2#9 0.75 recommended | route domain, wrong top1 |
| S-CAREER-062 | 공단 홈페이지에서 뽑는 보험 가입 기록에 나와 있는 월급 금액으로 예전에 얼마 벌었는지 보여줘도 통과될까요? | career_05_01_1 · no answer | career_01_01_2 · abstained · top career_01_01_2#1 0.45 reference | route branch |
| S-CAREER-095 | 근데 잔액은 그대로인데 왜 안 깎인거야? | career_07_03_2 · no answer | career_07 · abstained · top career_07_03#article 0.34 reference | route shallow |
| S-CAREER-114 | 내일배움카드가 뭔가요? | career_07_03 | career_07 · abstained · top career_07#article 0.49 reference | route shallow, abstained |
| S-CIVIL-010 | 법원에서 이름 고친 뒤 여행용 신분증 로마자 표기 다시 신청했는데 영어 증명서에 당장 나타나나요? | civil_01_03 | civil_03_04 · top civil_01_03#8 0.80 recommended | route branch |
| S-CIVIL-013 | 발급받은 서류에 찍힌 태어난 날짜가 호적 기록이랑 엇갈리는데 바로잡을 수 있을까요? | civil_01_05 | civil_05_02 · top civil_05_02#11 0.88 recommended | route branch, wrong top1 |
| S-CIVIL-035 | 아파서 외국 병원 진료가야됨 1회용으로 끊으려면 뭐 내야돼? 보호자도 같이 가는데 | civil_03_01_2 | medical · top civil_03_01_2#10 0.71 recommended | route domain |
| S-CIVIL-042 | 국내에서 잃어버린 게 아니라 해외여행 중에 여권 잃어버렸는데 현지 대사관에서 어떻게 다시 받나요? | civil_03_03_2 · no answer | abroad_07_03 · top abroad_07_03#1 0.85 recommended | route domain, answered unanswerable |
| S-CIVIL-046 | 여권 어제 신청했는데 취소하면 낸 돈 돌려받을 수 있음? | civil_03_05 | civil_03_05 · top civil_03_05#3 1.05 recommended | wrong top1 |
| S-CIVIL-084 | 갤럭시 S23 쓰고 있는데 어제 저녁에 인증 앱 승인 비번을 5번 넘게 틀렸더니 잠겼다고 떠요. 이거 언제 풀리고 비번 까먹었으면 어디서 다시  | civil_07_01_1 | bank_04_03_1 · top bank_04_03_1#15 0.90 recommended | route domain, wrong top1 |
| S-CIVIL-097 | 엄마 거동 불편해서 못 가는데 내가 대신 뽑아줄 수 있음? | civil_07_05 | medical_04_02_2 · top medical_04_02_2#5 0.71 recommended | route domain, wrong top1 |

