# 평가 결과 — 20261021-151952-jev_llm

- 실행: 2026-10-21T15:19:52Z → 2026-10-21T15:34:21Z (UTC) · git `0d26b76` (커밋 안 된 변경 있음)
- 데이터셋: `large-kb` v1.0.0 · kb `ebdb61493e54` · cases `ddb5a5ae0f57`
- 설정: routers ["jev_llm"] · variants ["full"] · suites ["all"] · split all · repeat 1 · concurrency 8 · beam_width "case/default" · limit "case/default"
- 모델(jev_llm): jev `jev-latest` · LLM `meta/muse-spark-1.3-contributor`
- 필터: --sample 1500 --seed final
- 메모: large: final jev_llm (coarse map + BM25 + LLM review + rescue >= 0.75)

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev_llm | full | 1436 | 83.8% | 75.5% | 0.827 | 84.5% | 0.855 | 0.844 | 0.800 | 24.2% | 2.9% | 1.8s / 13.7s | 3.7 | 0 | 76.6% | - |

## jev_llm · full

### 검색 (1436건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 83.8% [81.8–85.6] (n=1436) |
|   └ 답할 수 있는 질문 | 84.5% [82.4–86.4] (n=1316) |
|   └ 답할 수 없는 질문(보류가 정답) | 75.8% [67.4–82.6] (n=120) |
| 확신 정답률 (top1 정답 + recommended) | 83.7% [81.6–85.6] (n=1316) |
| 라우팅 정확도 (정확 일치) | 75.5% [73.2–77.6] (n=1436) |
| 계층 F1 (hF1) | 0.827 [0.809–0.844] (n=1436) |
| 트리 밖 판정 precision | 76.5% [66.2–84.4] (n=81) |
| 트리 밖 판정 recall | 83.8% [73.8–90.5] (n=74) |
| 트리 밖 판정 F1 | 0.800 |
| 오보류율 (답할 수 있는데 보류) | 2.9% [2.1–3.9] (n=1316) |
| 오답변율 (답할 수 없는데 답함) | 24.2% [17.4–32.6] (n=120) |
| Hit@1 | 84.5% [82.4–86.4] (n=1316) |
| Hit@3 | 86.2% [84.3–88.0] (n=1316) |
| Hit@k (limit) | 86.9% [84.9–88.6] (n=1316) |
| MRR@k | 0.855 [0.836–0.873] (n=1316) |
| nDCG@k | 0.844 [0.826–0.863] (n=1316) |
| Recall@k | 0.868 [0.850–0.886] (n=1316) |
| recommended 정밀도 | 0.831 [0.813–0.850] (n=1246) |
| 라우팅이 맞았을 때 Hit@1 | 97.9% [96.8–98.6] (n=989) |
| 라우팅이 맞았을 때 MRR | 0.987 [0.981–0.992] (n=989) |
| 지연 p50 / p90 / p95 / 평균 | 1.8s / 11.1s / 13.7s / 4.6s |
| 라우팅 구간 평균 | 4.4s |
| 랭킹 후보 수 평균 | 36.5 |
| Jev 입력 토큰 평균 (과금) | 36361 |
| LLM 토큰 평균 (입력+출력) | 3273 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 3.71 |
| LLM 라우팅 호출 수 평균 | 0.50 |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–0.3] (n=1436) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 89.5% (n=1362) | 81.8% (n=1362) | 75.7% (n=1261) | 71.0% (n=383) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|---:|
| 139 | 5 | 127 | 1133 | 20 | 12 | 64 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 196 | 77.6% | 85.7% | 77.6% | 0.826 | 1.9s |
| leaf | 1120 | 85.7% | 73.3% | 85.7% | 0.860 | 3.5s |
| no_answer | 46 | 47.8% | 71.7% | - | - | 4.6s |
| oos | 74 | 93.2% | 83.8% | - | - | 0.5s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 162 | 90.7% | 82.7% | 92.2% | 0.922 | 0.8s |
| single | 1274 | 82.9% | 74.6% | 83.5% | 0.846 | 3.4s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 1274 | 82.9% | 74.6% | 83.5% | 0.846 | 3.4s |
| coref | 29 | 96.6% | 79.3% | 96.6% | 0.966 | 4.7s |
| correct | 29 | 93.1% | 93.1% | 100.0% | 1.000 | 0.8s |
| ellipsis | 21 | 85.7% | 81.0% | 85.7% | 0.857 | 3.5s |
| refine | 33 | 90.9% | 75.8% | 90.6% | 0.906 | 0.8s |
| shift | 30 | 90.0% | 83.3% | 92.3% | 0.923 | 0.7s |
| slot | 20 | 85.0% | 85.0% | 84.2% | 0.842 | 5.2s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 1436 | 83.8% | 75.5% | 84.5% | 0.855 | 1.8s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 196 | 77.6% | 85.7% | 77.6% | 0.826 | 1.9s |
| leaf | 1166 | 84.2% | 73.2% | 85.7% | 0.860 | 3.7s |
| none | 74 | 93.2% | 83.8% | - | - | 0.5s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 74 | 93.2% | 83.8% | - | - | 0.5s |
| d2 | 101 | 83.2% | 89.1% | 83.2% | 0.868 | 1.9s |
| d3 | 878 | 83.1% | 75.2% | 84.7% | 0.855 | 0.9s |
| d4 | 383 | 83.6% | 71.0% | 84.4% | 0.849 | 5.4s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 74 | 93.2% | 83.8% | - | - | 0.5s |
| 101+ | 4 | 100.0% | 100.0% | 100.0% | 1.000 | 5.1s |
| 2-5 | 159 | 87.4% | 78.0% | 87.6% | 0.882 | 3.5s |
| 33-100 | 151 | 79.5% | 82.1% | 80.0% | 0.835 | 3.7s |
| 6-32 | 1048 | 83.1% | 73.5% | 84.6% | 0.853 | 3.3s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| colloquial | 263 | 84.8% | 76.8% | 87.3% | 0.876 | 0.9s |
| condition | 95 | 87.4% | 76.8% | 88.2% | 0.891 | 1.0s |
| english | 45 | 77.8% | 75.6% | 77.3% | 0.773 | 0.8s |
| keyword | 100 | 77.0% | 60.0% | 77.3% | 0.773 | 4.9s |
| long | 166 | 89.2% | 79.5% | 91.9% | 0.919 | 3.3s |
| mixed | 55 | 92.7% | 74.5% | 92.5% | 0.925 | 0.7s |
| negation | 39 | 89.7% | 53.8% | 91.9% | 0.919 | 4.6s |
| oos_far | 41 | 97.6% | 90.2% | - | - | 0.5s |
| oos_near | 33 | 87.9% | 75.8% | - | - | 1.9s |
| paraphrase | 333 | 81.4% | 71.5% | 82.2% | 0.825 | 5.0s |
| typo | 70 | 84.3% | 75.7% | 87.7% | 0.877 | 0.9s |
| vague | 196 | 77.6% | 85.7% | 77.6% | 0.826 | 1.9s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| abroad | 38 | 86.8% | 84.2% | 89.2% | 0.892 | 0.9s |
| appliance | 39 | 82.1% | 76.9% | 84.2% | 0.855 | 3.3s |
| bank | 51 | 88.2% | 80.4% | 90.0% | 0.910 | 0.8s |
| car | 41 | 87.8% | 80.5% | 92.1% | 0.934 | 0.8s |
| card | 41 | 78.0% | 65.9% | 76.9% | 0.769 | 5.1s |
| career | 50 | 88.0% | 78.0% | 89.1% | 0.902 | 4.3s |
| civil | 42 | 73.8% | 69.0% | 75.0% | 0.771 | 5.7s |
| cook | 39 | 87.2% | 84.6% | 91.9% | 0.919 | 0.7s |
| fitness | 49 | 89.8% | 85.7% | 89.8% | 0.918 | 0.7s |
| health | 30 | 86.7% | 80.0% | 89.7% | 0.897 | 0.9s |
| homecare | 54 | 85.2% | 70.4% | 84.9% | 0.858 | 3.5s |
| house | 40 | 77.5% | 75.0% | 81.1% | 0.811 | 3.5s |
| hr | 36 | 91.7% | 72.2% | 91.7% | 0.917 | 5.3s |
| insure | 49 | 81.6% | 73.5% | 81.6% | 0.823 | 0.9s |
| itdesk | 51 | 76.5% | 70.6% | 77.6% | 0.776 | 5.1s |
| legal | 42 | 78.6% | 64.3% | 77.5% | 0.787 | 6.8s |
| loan | 36 | 77.8% | 63.9% | 77.8% | 0.778 | 5.2s |
| medical | 38 | 86.8% | 76.3% | 86.5% | 0.885 | 4.7s |
| member | 55 | 85.5% | 76.4% | 86.8% | 0.900 | 5.7s |
| net | 32 | 87.5% | 78.1% | 87.1% | 0.871 | 6.0s |
| office | 36 | 91.7% | 80.6% | 91.4% | 0.932 | 4.9s |
| oos | 74 | 93.2% | 83.8% | - | - | 0.5s |
| parent | 38 | 97.4% | 92.1% | 97.2% | 0.986 | 0.7s |
| payroll | 47 | 68.1% | 59.6% | 74.4% | 0.750 | 7.1s |
| pc | 30 | 90.0% | 86.7% | 90.0% | 0.925 | 0.7s |
| pension | 51 | 78.4% | 72.5% | 79.6% | 0.818 | 0.9s |
| pet | 36 | 86.1% | 77.8% | 86.1% | 0.866 | 0.7s |
| phone | 31 | 87.1% | 83.9% | 87.1% | 0.871 | 4.4s |
| returns | 53 | 83.0% | 73.6% | 86.0% | 0.860 | 5.6s |
| security | 37 | 67.6% | 48.6% | 66.7% | 0.681 | 6.9s |
| shop | 40 | 82.5% | 70.0% | 82.5% | 0.838 | 6.9s |
| tax | 28 | 82.1% | 78.6% | 82.1% | 0.839 | 4.6s |
| transit | 43 | 81.4% | 74.4% | 85.4% | 0.854 | 4.1s |
| utility | 39 | 82.1% | 82.1% | 83.8% | 0.843 | 0.8s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| hard | 196 | 77.6% | 85.7% | 77.6% | 0.826 | 1.9s |
| medium | 1240 | 84.8% | 73.9% | 85.7% | 0.860 | 1.7s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 978 | 84.6% | 75.8% | 85.6% | 0.865 | 3.7s |
| test | 458 | 82.1% | 74.9% | 82.1% | 0.831 | 0.9s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 81.8% | 98.6% | 82.1% | 47.5% |
| 0.25 | 82.3% | 98.4% | 82.7% | 41.7% |
| 0.30 | 83.0% | 97.9% | 83.7% | 33.3% |
| 0.35 | 83.1% | 97.7% | 84.0% | 31.7% |
| 0.40 | 83.4% | 97.2% | 84.6% | 28.3% |
| 0.50 | 84.2% | 96.0% | 86.4% | 17.5% |
| 0.60 | 84.2% | 94.6% | 87.6% | 15.0% |
| 0.65 | 83.9% | 93.5% | 88.4% | 13.3% |

### 저장 (ingest, 64건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 76.6% [64.9–85.3] (n=64) |
|   └ 올바른 카테고리에 저장 | 76.6% [64.9–85.3] (n=64) |
|   └ 트리/루트 밖 Q&A 거절 | - |
| 계층 F1 | 0.846 [0.770–0.922] (n=64) |
| 초안이 검색에 안 보임 | 100.0% [94.3–100.0] (n=63) |
| 게시 성공 (버전 +1) | 100.0% [94.3–100.0] (n=63) |
| 초안→게시 카테고리 유지 | 100.0% [94.3–100.0] (n=63) |
| 중복 탐지 precision | - |
| 중복 탐지 recall | - |
| 갱신/신규 판정 일치 | 100.0% [94.3–100.0] (n=63) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 0 |
| 게시 후 probe 검색 top-k 포함 | - |
| 게시 후 probe 검색 top1 답변 | - |
| 지연 p50 (초안+게시+probe) | 1.1s |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–5.7] (n=63) |

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| itdesk_06_01 | pc_02_05 | 2 |
| legal_01_02_2 | returns_05_03_1 | 2 |
| loan_01_05_2 | loan_01_02 | 2 |
| null | car | 2 |
| null | transit_07_01_3 | 2 |
| abroad_01_04_1 | abroad_02_02 | 1 |
| abroad_01_04_2 | abroad_01_04 | 1 |
| abroad_01_04_2 | abroad_02_01 | 1 |
| abroad_04_04_1 | abroad_04 | 1 |
| abroad_06_02_2 | legal_03 | 1 |
| abroad_07_02 | phone_07_03 | 1 |
| appliance_02_02 | appliance_02 | 1 |
| appliance_02_02 | appliance_02_01 | 1 |
| appliance_03_05_1 | appliance_03_02 | 1 |
| appliance_04_01 | appliance_04_02 | 1 |
| appliance_04_02 | null | 1 |
| appliance_05_03 | parent_07_03_2 | 1 |
| appliance_06_05 | appliance_01_04 | 1 |
| appliance_07_02 | appliance | 1 |
| appliance_07_03 | appliance_07 | 1 |

### 틀린 케이스 (402건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-BANK-005 | 매달 나가는 자동송금 내역을 한 번에 조회하고 싶을 때 어디서 보나요? | bank_03_02 | bank_02_04 | branch |
| I-CAR-002 | 내 차 다음 검사 유효기간은 어디서 확인하나요? | car_05_02_1 | car_05_02 | shallow |
| I-CAREER-003 | 이직이 잦은 경력 3년차가 임원 앞에서 퇴사 사유를 어떻게 말할까요? | career_04_02 | career_01_01_3 | branch |
| I-FITNESS-002 | 퇴근 후 밤 9시에 처음 러닝할 때 식사와 간격은 어떻게 두나요 | fitness_04_01 | fitness_08_04 | branch |
| I-HOUSE-004 | 임차권등기명령은 관할 법원에 직접 가야만 신청되나요? | house_05_02 | legal_05_02_1 | domain |
| I-INSURE-005 | 정기보험 만기가 되었는데 받은 만기축하금이 없는 이유는 무엇인가요? | insure_04_02 | insure_07_04 | branch |
| I-LOAN-004 | 공동인증서 없이 간편인증으로 평가사 상세 보고서를 발급받는 순서가 궁금해요? | loan_01_05_2 | loan_01_04 | branch |
| I-LOAN-005 | 10년 단기로 갚을 때 원금균등의 이자 절감 체감이 큰 이유는 무엇인가요? | loan_03_04 | loan_06_01_2 | branch |
| I-PAYROLL-001 | 수습 3개월 동안 첫급여가 기본급의 90%로 지급되는 조건은? | payroll_02_02 | career_05_04 | domain |
| I-PAYROLL-003 | 산재 치료는 꼭 산재 지정병원에서 받아야 하나요? | payroll_07_04 | error invalid_request | false_none |
| I-PENSION-002 | 본인 노령연금과 배우자 유족연금을 동시에 받을 수 있나요? | pension_03_03_1 | pension_03 | shallow |
| I-PENSION-004 | 육아휴직이나 장기휴직 기간도 DB형 근속연수에 포함되나요? | pension_05_01 | pension_04_02_2 | branch |
| I-PHONE-002 | 밤에 스마트폰 화면이 너무 하얘서 따뜻한 색온도로 바꾸려면 어떻게 하나요? | phone_02_03_2 | phone_02_03 | shallow |
| I-SHOP-003 | 배송비 쿠폰과 상품 할인쿠폰을 같은 주문에 같이 쓸 수 있나요? | shop_03_04 | member_05_02_2 | domain |
| I-SHOP-005 | 국내상품과 해외직구 상품을 한 번에 같이 주문할 수 있나요? | shop_06_01 | shop_01_01 | branch |
| S-ABROAD-007 | 할인표로 산 비행기 타는 날 아침에 갑자기 못 가게 되면 낸 돈은 한 푼도 못 되돌려받나요? | abroad_01_04_1 | abroad_02_02 · top abroad_02_02#7 0.66 recommended | route branch, wrong top1 |
| S-ABROAD-009 | 왕복 귀국편 변경 차액 | abroad_01_04_2 | abroad_02_01 · top abroad_02_01#5 0.80 recommended | route branch, wrong top1 |
| S-ABROAD-010 | 12월 붐빌 때 날짜 바꾸기 편한 표 종류마다 비행 점수 쌓이는 퍼센트가 어느 정도 벌어지나요? | abroad_01_04_2 | abroad_01_04 · top abroad_01_04_2#5 0.79 recommended | route shallow |
| S-ABROAD-055 | 장기체류 비자 체류목적 기재 | abroad_04_04_1 | abroad_04 · top abroad_04_04_1#11 0.86 recommended | route shallow |
| S-ABROAD-072 | 어제 시내 환급처에서 카드 보증 걸고 돈 먼저 받았거든요 근데 출국할 때 세관 확인 깜빡하면 카드에서 다시 빠져나가는 거예요? | abroad_05_04 · no answer | abroad_05_04 · top abroad_05_04#10 0.66 recommended | answered unanswerable |
| S-ABROAD-081 | 이웃 세대에서 하는 인테리어 작업이 너무 시끄러워 밤에 눈 붙이기가 힘든데 어떻게 해결하고 보상받나요? | abroad_06_02_2 | legal_03 · top legal_03_01#5 0.54 alternative | route domain, wrong top1 |
| S-ABROAD-094 | 스마트폰 보상청구 순서 | abroad_07_02 | phone_07_03 · top phone_07_03#5 0.68 recommended | route domain, wrong top1 |
| S-APPLIANCE-017 | 그거 물 고이면 어떡해? 거기 잎채소랑 과일 고습 저습 어떻게 나눠 넣음? | appliance_02_02 | appliance_02_01 · top appliance_02_01#5 0.71 recommended | route branch, wrong top1 |
| S-APPLIANCE-018 | 지난주 이사하고 냉장고 자리 잡았는데 문이 자꾸 저절로 벌어져요. 앞다리 수평이 안 맞는 느낌인데 기울기가 이상해요. 조절다리 어느 방향으로 돌 | appliance_02_02 · no answer | appliance_02 · top appliance_02_02#8 0.46 alternative | route shallow, answered unanswerable |
| S-APPLIANCE-039 | 2021년형 삼성 스탠드 에어컨 17평형인데 3일 전에 극세필터 물세척해서 끼웠거든요. 오늘 25도 냉방으로 1시간 틀었더니 시큼한 냄새가 계속 | appliance_03_05_1 | appliance_03_02 · top appliance_03_05_1#4 0.85 recommended | route branch |
| S-APPLIANCE-044 | 약하게나 보통으로 돌릴 때는 멀쩡한데 그게 아니라 제일 센 강력으로 올리면 1분도 안 돼서 바로 꺼져버려요 | appliance_04_01 | appliance_04_02 · top appliance_04_02#3 0.95 recommended | route branch, wrong top1 |
| S-APPLIANCE-045 | 한겨울에 창가 쪽 창고에 보관했더니 금방 방전돼서 멈추는데 이유가 뭘까요? | appliance_04_02 | null · abstained · top no items | route false_none, abstained |
| S-APPLIANCE-065 | 초등생 애가 밤마다 야한 영상 멋대로 틀까 봐 걱정돼요 비번 눌러야 켜지도록 차단하는 기능 있어요? | appliance_05_03 | parent_07_03_2 · top parent_07_03_2#6 0.73 recommended | route domain, wrong top1 |
| S-APPLIANCE-083 | 깨끗이 훔쳐냈는데도 묵은 향이 가시지 않아요. 시판 약품 없이 평소 문 틈 벌려두고 바람 통하게 두는 요령이 궁금해요. | appliance_06_05 | appliance_01_04 · top appliance_01_04#5 0.81 recommended | route branch, wrong top1 |
| S-APPLIANCE-088 | 출근 전에 reservation 걸어서 오후 5시에 finish 되게 설정 가능해요? | appliance_07_02 | appliance · top appliance_07_02#6 0.96 recommended | route shallow |
| S-APPLIANCE-111 | 식기세척기 세척 결과가 이상해요 | appliance_07_03 | appliance_07 · top appliance_07#article 0.91 recommended | route shallow, wrong top1 |
| S-BANK-003 | 기한이 끝난 패스포트나 폰에 저장해둔 주민증 사진으로 통과될까요? | bank_01_01_1 · no answer | abroad_04 · top civil_06_04#7 0.82 recommended | route domain, answered unanswerable |
| S-BANK-007 | 계좌개설 첫입금 금액 | bank_01_02 | bank_01 · top bank_01_02#10 0.91 recommended | route shallow |
| S-BANK-013 | 한도계좌 걸렸는데 풀려면 뭐 해야됨? | bank_01_05 | bank_01 · top bank_01_05#10 0.86 recommended | route shallow |
| S-BANK-026 | 보험료 나가는 날이 월급 들어오기 전이라 맨날 잔부족 뜸ㅠ 뒤로 미룰 수 있음? | bank_02_04_2 | insure_06_02_1 · top bank_02_04_2#3 0.91 recommended | route domain |
| S-BANK-046 | 업무용이랑 일상용 기기를 번갈아 쓰는데 양쪽 모두에 인증수단을 함께 설정해 쓸 수 있을까요? | bank_04_03_1 | security_02 · top security_02_02_3#11 0.68 recommended | route domain, wrong top1 |
| S-BANK-062 | 외국으로 돈 보내는 거 신청해놨는데 아직 발송되기 전이면 철회할 수 있어요? | bank_05_05 | bank_06_04 · top bank_06_04#7 0.91 recommended | route branch, wrong top1 |
| S-BANK-063 | 월급 여러 명꺼 한 번에 보내기 직전인데 전부 취소됨? | bank_05_05 | payroll · top payroll_04_05#7 0.48 alternative | route domain, wrong top1 |
| S-BANK-082 | 인증서 비번 바꿨는데 다시 등록해야됨? | bank_07_02_2 | bank_07_02 · top bank_07_02_2#37 0.95 recommended | route shallow |
| S-BANK-086 | 어제 뱅킹앱 최신버전으로 업데이트했더니 켜자마자 튕겨버려요. 급하게 확인해야 하는데 로그인은 커녕 실행 자체가 안 돼요. 캐시 지우고 재부팅한  | bank_07_04 | phone_05_03_1 · top bank_07_04#2 0.93 recommended | route domain |
| S-BANK-102 | 안 쓰는 계좌들 전체적으로 정리하고 싶어요 | bank_02 | bank_02_03 · top bank_02_03#2 0.94 recommended | route deep, wrong top1 |
| S-BANK-107 | 돈을 잘못 보낸 것 같은데 어떡하죠 | bank_05 | bank_05 · top bank_05_01#1 0.79 recommended | wrong top1 |
| S-CAR-003 | 타이어 공기압 경고등 대처 | car_01_02_1 | car_02_03 · top car_02_03#5 0.83 recommended | route branch, wrong top1 |
| S-CAR-028 | 지난주에 스마트키 겉에 금속 커버를 씌웠는데요. 그 뒤로 멀리서는 문이 안 열리고 바로 앞에 가야 열려요. 뭘 점검해봐야 할까요? | car_02_04_2 | car_02_04 · top car_02_04_2#20 0.93 recommended | route shallow |
| S-CAR-035 | 길 가던 분을 차로 쳤는데 차를 멈춘 뒤에 앰뷸런스 부르기와 경찰에 알리기를 어떤 차례로 진행해야 하나요? 쓰러진 분은 옮기지 말고 기다려야 하 | car_03_01_2 | car_03_01 · top car_03_01_2#1 0.91 recommended | route shallow |
| S-CAR-038 | 어제 오후 3시쯤 대형마트 지하주차장에서 후진 빼다가 통로 지나던 차랑 부딪혔어요. 상대는 제가 100% 잘못이라는데 후진·문열림 과실비율표 어 | car_03_02 · no answer | insure_03_04 · top insure_03_04#2 0.80 recommended | route domain, answered unanswerable |
| S-CAR-055 | 확인해보니 만 73세시고 2종 보통 맞으세요. 5년마다 갱신에 2시간 교육 들어야 한다던데 병원 검사부터 받아야 하나요, 준비물이랑 비용도 알려 | car_04_04 · no answer | car_04_04 · top car_04_04#2 0.38 reference | answered unanswerable |
| S-CAR-066 | 리스차 승게받으려고 하는데 어케 내명의로 바꾸나요? 리스회사 동의서 받아야되나여ㅠ 신용심사 통과해야된다던데 수수료도 있나요? | car_05_03 | null · abstained · top no items | route false_none, abstained |
| S-CAR-089 | 겨울타이어 교체시기 공기압 | car_08_01 | car_01_02 · top car_08_01#8 0.96 recommended | route branch |
| S-CAR-090 | 수막으로 차가 둥둥 떴을 때가 아니라 그냥 폭우 쏟아질 때 고속도로 몇 키로로 가야 돼요? 앞차랑 간격은 얼마나 벌려야 해요? | car_08_01 | car_08 · top car_08_01#2 0.95 recommended | route shallow |
| S-CAR-108 | 과태료 관련해서 전반적으로 알려주세요 | car_06_03 | car_06 · top car_06#article 0.80 recommended | route shallow, wrong top1 |
| S-CARD-006 | 월급에서 집세랑 빚 갚고 남는 돈이 적다고 승인 안 났는데, 또 넣어보려면 뭘 준비해야 할까요? | card_01_01_3 | loan_02_04 · top loan_02_04#7 0.76 recommended | route domain, wrong top1 |
| S-CARD-012 | 자녀카드 온라인 업종 차단 | card_01_03 | card_02_03 · top card_01_03#8 0.94 recommended | route branch |
| S-CARD-015 | 영업점에서 그 자리에서 바로 뽑는 게 아니라 앱으로 우편 재발급 신청하려는데 돈은 얼마나 나가고 도착까지 며칠 걸려요? | card_01_04 | bank_04_03_2 · top card_01_04#7 0.82 recommended | route domain |
| S-CARD-019 | 그거 매달 문자로 받아보려면 어디 들어가서 켜야 돼요? | card_02_01 | card_02 · top card_02_01#14 0.87 recommended | route shallow |
| S-CARD-023 | 65세 넘은 아빠 보이스피싱 걱정됨ㅠ 해외랑 온라인 현금인출 한방에 잠그는거 어케함? | card_02_03 | bank_04_04 · top card_02_03#17 0.86 recommended | route domain |
| S-CARD-025 | 분할 납부 승인을 금지하고 전액 즉시 납부 방식으로만 이용하려면 뭘 해야 돼요? | card_02_03 | card_04_04 · abstained · top card_04_04#article 0.31 reference | route branch, abstained |
| S-CARD-034 | 실물카드는 멀쩡히 있는데 폰 속 모바일카드만 털린듯ㅠ 이거만 따로 정지됨? | card_03_01 | card_07 · top card_03_01#15 0.95 recommended | route branch |
| S-CARD-054 | 토요일 밤 11시쯤 부여받은 계좌로 미리 갚을 돈을 보냈어요. 통장에서는 출금됐는데 앱 상환내역에는 아직 안 뜨더라고요. 이럴 때 보통 언제 반 | card_04_05 | loan_06_02 · top card_04_05#7 0.80 recommended | route domain |
| S-CARD-058 | 미국 직구 사이트에서 150만원짜리 카메라 결제하려는데 국내 한도는 남았는데 계속 거절돼요. 해외이용한도가 따로 있는 건가요, 이럴 땐 어떻게  | card_05_01_2 | card_05_04 · top card_05_04#3 0.92 recommended | route branch, wrong top1 |

