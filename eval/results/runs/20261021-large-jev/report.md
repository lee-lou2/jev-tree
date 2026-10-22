# 평가 결과 — 20261021-151952-jev

- 실행: 2026-10-21T15:19:52Z → 2026-10-21T15:23:35Z (UTC) · git `0d26b76` (커밋 안 된 변경 있음)
- 데이터셋: `large-kb` v1.0.0 · kb `ebdb61493e54` · cases `ddb5a5ae0f57`
- 설정: routers ["jev"] · variants ["full"] · suites ["all"] · split all · repeat 1 · concurrency 6 · beam_width "case/default" · limit "case/default"
- 모델(jev): jev `jev-latest`
- 필터: --sample 1500 --seed final
- 메모: large: final jev (coarse map + BM25 + rescue >= 0.75)

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev | full | 1436 | 82.0% | 68.5% | 0.775 | 82.6% | 0.838 | 0.826 | 0.776 | 24.2% | 3.0% | 0.9s / 1.3s | 4.5 | 0 | 73.4% | - |

## jev · full

### 검색 (1436건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 82.0% [80.0–83.9] (n=1436) |
|   └ 답할 수 있는 질문 | 82.6% [80.5–84.6] (n=1316) |
|   └ 답할 수 없는 질문(보류가 정답) | 75.8% [67.4–82.6] (n=120) |
| 확신 정답률 (top1 정답 + recommended) | 81.9% [79.7–83.9] (n=1316) |
| 라우팅 정확도 (정확 일치) | 68.5% [66.1–70.9] (n=1436) |
| 계층 F1 (hF1) | 0.775 [0.756–0.795] (n=1436) |
| 트리 밖 판정 precision | 78.1% [67.3–86.0] (n=73) |
| 트리 밖 판정 recall | 77.0% [66.3–85.1] (n=74) |
| 트리 밖 판정 F1 | 0.776 |
| 오보류율 (답할 수 있는데 보류) | 3.0% [2.2–4.1] (n=1316) |
| 오답변율 (답할 수 없는데 답함) | 24.2% [17.4–32.6] (n=120) |
| Hit@1 | 82.6% [80.5–84.6] (n=1316) |
| Hit@3 | 84.9% [82.8–86.7] (n=1316) |
| Hit@k (limit) | 85.7% [83.7–87.5] (n=1316) |
| MRR@k | 0.838 [0.819–0.858] (n=1316) |
| nDCG@k | 0.826 [0.807–0.845] (n=1316) |
| Recall@k | 0.857 [0.838–0.876] (n=1316) |
| recommended 정밀도 | 0.822 [0.803–0.841] (n=1231) |
| 라우팅이 맞았을 때 Hit@1 | 97.8% [96.6–98.6] (n=899) |
| 라우팅이 맞았을 때 MRR | 0.987 [0.981–0.993] (n=899) |
| 지연 p50 / p90 / p95 / 평균 | 0.9s / 1.1s / 1.3s / 0.9s |
| 라우팅 구간 평균 | 0.6s |
| 랭킹 후보 수 평균 | 43.7 |
| Jev 입력 토큰 평균 (과금) | 38659 |
| LLM 토큰 평균 (입력+출력) | 0 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 4.46 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 85.5% (n=1362) | 76.6% (n=1362) | 68.8% (n=1261) | 62.1% (n=383) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|---:|
| 153 | 3 | 185 | 1031 | 17 | 17 | 94 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 196 | 74.0% | 80.6% | 74.0% | 0.805 | 0.9s |
| leaf | 1120 | 84.1% | 66.2% | 84.1% | 0.844 | 0.9s |
| no_answer | 46 | 47.8% | 60.9% | - | - | 0.9s |
| oos | 74 | 93.2% | 77.0% | - | - | 0.5s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 162 | 90.1% | 71.0% | 92.2% | 0.922 | 0.9s |
| single | 1274 | 81.0% | 68.2% | 81.3% | 0.827 | 0.9s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 1274 | 81.0% | 68.2% | 81.3% | 0.827 | 0.9s |
| coref | 29 | 100.0% | 69.0% | 100.0% | 1.000 | 0.9s |
| correct | 29 | 93.1% | 82.8% | 96.2% | 0.962 | 0.8s |
| ellipsis | 21 | 85.7% | 76.2% | 85.7% | 0.857 | 0.9s |
| refine | 33 | 87.9% | 63.6% | 90.6% | 0.906 | 0.9s |
| shift | 30 | 86.7% | 70.0% | 92.3% | 0.923 | 0.9s |
| slot | 20 | 85.0% | 65.0% | 84.2% | 0.842 | 0.9s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 1436 | 82.0% | 68.5% | 82.6% | 0.838 | 0.9s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 196 | 74.0% | 80.6% | 74.0% | 0.805 | 0.9s |
| leaf | 1166 | 82.7% | 66.0% | 84.1% | 0.844 | 0.9s |
| none | 74 | 93.2% | 77.0% | - | - | 0.5s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 74 | 93.2% | 77.0% | - | - | 0.5s |
| d2 | 101 | 80.2% | 86.1% | 80.2% | 0.853 | 0.9s |
| d3 | 878 | 80.5% | 68.6% | 81.9% | 0.830 | 0.9s |
| d4 | 383 | 83.8% | 62.1% | 84.9% | 0.853 | 0.9s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 74 | 93.2% | 77.0% | - | - | 0.5s |
| 101+ | 4 | 75.0% | 75.0% | 75.0% | 0.786 | 1.3s |
| 2-5 | 159 | 84.9% | 70.4% | 85.0% | 0.856 | 0.8s |
| 33-100 | 151 | 75.5% | 76.8% | 76.0% | 0.810 | 0.9s |
| 6-32 | 1048 | 81.8% | 66.4% | 83.3% | 0.840 | 0.9s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| colloquial | 263 | 81.7% | 67.7% | 84.5% | 0.847 | 0.9s |
| condition | 95 | 85.3% | 66.3% | 86.0% | 0.866 | 0.9s |
| english | 45 | 80.0% | 71.1% | 79.5% | 0.807 | 0.8s |
| keyword | 100 | 77.0% | 53.0% | 77.3% | 0.773 | 0.9s |
| long | 166 | 89.8% | 75.9% | 91.9% | 0.919 | 0.9s |
| mixed | 55 | 89.1% | 70.9% | 88.7% | 0.887 | 0.8s |
| negation | 39 | 84.6% | 61.5% | 86.5% | 0.865 | 0.9s |
| oos_far | 41 | 92.7% | 90.2% | - | - | 0.5s |
| oos_near | 33 | 93.9% | 60.6% | - | - | 0.5s |
| paraphrase | 333 | 79.3% | 61.0% | 80.0% | 0.806 | 0.9s |
| typo | 70 | 85.7% | 72.9% | 89.2% | 0.892 | 0.8s |
| vague | 196 | 74.0% | 80.6% | 74.0% | 0.805 | 0.9s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| abroad | 38 | 86.8% | 78.9% | 89.2% | 0.892 | 0.9s |
| appliance | 39 | 82.1% | 66.7% | 84.2% | 0.851 | 0.9s |
| bank | 51 | 84.3% | 78.4% | 86.0% | 0.890 | 1.0s |
| car | 41 | 87.8% | 82.9% | 89.5% | 0.908 | 0.8s |
| card | 41 | 70.7% | 56.1% | 69.2% | 0.692 | 0.9s |
| career | 50 | 84.0% | 68.0% | 84.8% | 0.848 | 0.8s |
| civil | 42 | 76.2% | 66.7% | 77.5% | 0.801 | 0.9s |
| cook | 39 | 87.2% | 84.6% | 91.9% | 0.919 | 1.1s |
| fitness | 49 | 89.8% | 85.7% | 89.8% | 0.918 | 0.9s |
| health | 30 | 86.7% | 76.7% | 86.2% | 0.862 | 0.9s |
| homecare | 54 | 88.9% | 70.4% | 88.7% | 0.896 | 0.9s |
| house | 40 | 80.0% | 67.5% | 83.8% | 0.838 | 0.8s |
| hr | 36 | 86.1% | 61.1% | 86.1% | 0.861 | 0.8s |
| insure | 49 | 79.6% | 67.3% | 79.6% | 0.796 | 0.8s |
| itdesk | 51 | 74.5% | 58.8% | 75.5% | 0.755 | 0.9s |
| legal | 42 | 69.0% | 47.6% | 67.5% | 0.700 | 0.9s |
| loan | 36 | 83.3% | 61.1% | 83.3% | 0.847 | 0.9s |
| medical | 38 | 86.8% | 65.8% | 86.5% | 0.891 | 0.9s |
| member | 55 | 80.0% | 56.4% | 81.1% | 0.847 | 0.9s |
| net | 32 | 78.1% | 68.8% | 80.6% | 0.806 | 0.9s |
| office | 36 | 86.1% | 69.4% | 85.7% | 0.886 | 0.9s |
| oos | 74 | 93.2% | 77.0% | - | - | 0.5s |
| parent | 38 | 89.5% | 84.2% | 91.7% | 0.931 | 0.8s |
| payroll | 47 | 70.2% | 57.4% | 76.7% | 0.767 | 0.9s |
| pc | 30 | 86.7% | 76.7% | 86.7% | 0.888 | 0.9s |
| pension | 51 | 76.5% | 62.7% | 77.6% | 0.798 | 0.8s |
| pet | 36 | 86.1% | 75.0% | 86.1% | 0.866 | 0.8s |
| phone | 31 | 80.6% | 67.7% | 80.6% | 0.840 | 0.8s |
| returns | 53 | 75.5% | 60.4% | 78.0% | 0.805 | 0.9s |
| security | 37 | 70.3% | 45.9% | 69.4% | 0.694 | 0.8s |
| shop | 40 | 82.5% | 62.5% | 82.5% | 0.838 | 0.9s |
| tax | 28 | 85.7% | 75.0% | 85.7% | 0.875 | 0.8s |
| transit | 43 | 81.4% | 74.4% | 85.4% | 0.854 | 0.8s |
| utility | 39 | 82.1% | 76.9% | 83.8% | 0.847 | 0.8s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| hard | 196 | 74.0% | 80.6% | 74.0% | 0.805 | 0.9s |
| medium | 1240 | 83.3% | 66.6% | 84.1% | 0.844 | 0.9s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 978 | 82.1% | 68.3% | 82.8% | 0.841 | 0.9s |
| test | 458 | 81.9% | 69.0% | 82.1% | 0.832 | 0.9s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 79.8% | 98.6% | 80.0% | 50.8% |
| 0.25 | 80.1% | 98.4% | 80.4% | 47.5% |
| 0.30 | 80.7% | 97.9% | 81.3% | 40.0% |
| 0.35 | 81.1% | 97.4% | 82.1% | 35.0% |
| 0.40 | 81.6% | 97.0% | 82.8% | 28.3% |
| 0.50 | 82.5% | 95.4% | 85.0% | 17.5% |
| 0.60 | 82.4% | 93.7% | 86.4% | 15.0% |
| 0.65 | 82.3% | 92.3% | 87.6% | 13.3% |

### 저장 (ingest, 64건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 73.4% [61.5–82.7] (n=64) |
|   └ 올바른 카테고리에 저장 | 73.4% [61.5–82.7] (n=64) |
|   └ 트리/루트 밖 Q&A 거절 | - |
| 계층 F1 | 0.822 [0.741–0.903] (n=64) |
| 초안이 검색에 안 보임 | 100.0% [94.3–100.0] (n=63) |
| 게시 성공 (버전 +1) | 100.0% [94.3–100.0] (n=63) |
| 초안→게시 카테고리 유지 | 100.0% [94.3–100.0] (n=63) |
| 중복 탐지 precision | - |
| 중복 탐지 recall | - |
| 갱신/신규 판정 일치 | 100.0% [94.3–100.0] (n=63) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 0 |
| 게시 후 probe 검색 top-k 포함 | - |
| 게시 후 probe 검색 top1 답변 | - |
| 지연 p50 (초안+게시+probe) | 0.8s |
| LLM 실패 → Jev 빔 대체율 | - |

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| insure_03_05 | car_03_04 | 2 |
| legal_01_02_2 | returns_05 | 2 |
| legal_01_04 | legal_01_05 | 2 |
| null | car | 2 |
| null | house | 2 |
| abroad_01_04_1 | abroad_02 | 1 |
| abroad_01_04_2 | abroad_02_01 | 1 |
| abroad_03_01_2 | abroad_03_01 | 1 |
| abroad_04_04_1 | abroad_04 | 1 |
| abroad_04_04_2 | abroad_03_02 | 1 |
| abroad_05_05 | abroad_05 | 1 |
| abroad_06_02_2 | legal_03 | 1 |
| abroad_07_02 | phone_07_03 | 1 |
| appliance_02_02 | appliance_02 | 1 |
| appliance_02_02 | cook_03 | 1 |
| appliance_02_06 | cook_03 | 1 |
| appliance_03_05_1 | appliance_03_02 | 1 |
| appliance_03_05_2 | utility_03_02 | 1 |
| appliance_04_01 | pc_02_04 | 1 |
| appliance_04_02 | car_02_04 | 1 |

### 틀린 케이스 (501건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-APPLIANCE-004 | TV 화면에서 유료 멤버십을 구독하고 해지하는 경로는 어디인가요? | appliance_05_03 | error invalid_request | false_none |
| I-APPLIANCE-005 | 문 닫힘 표시가 깜빡이면서 작동이 바로 멈춰요 | appliance_06_03 | appliance_01_02 | branch |
| I-BANK-005 | 매달 나가는 자동송금 내역을 한 번에 조회하고 싶을 때 어디서 보나요? | bank_03_02 | bank_02_04 | branch |
| I-CAR-002 | 내 차 다음 검사 유효기간은 어디서 확인하나요? | car_05_02_1 | car_05_02 | shallow |
| I-CAREER-003 | 이직이 잦은 경력 3년차가 임원 앞에서 퇴사 사유를 어떻게 말할까요? | career_04_02 | career_01_01_3 | branch |
| I-FITNESS-002 | 퇴근 후 밤 9시에 처음 러닝할 때 식사와 간격은 어떻게 두나요 | fitness_04_01 | fitness_08_04 | branch |
| I-HOUSE-004 | 임차권등기명령은 관할 법원에 직접 가야만 신청되나요? | house_05_02 | legal_05_02_1 | domain |
| I-INSURE-003 | 자동차등록증 사진을 업로드하라는 안내가 나왔을 때 제출 방법은요? | insure_02_05 | car_05 | domain |
| I-INSURE-005 | 정기보험 만기가 되었는데 받은 만기축하금이 없는 이유는 무엇인가요? | insure_04_02 | insure_07_04 | branch |
| I-LOAN-004 | 공동인증서 없이 간편인증으로 평가사 상세 보고서를 발급받는 순서가 궁금해요? | loan_01_05_2 | loan_01_01 | branch |
| I-NET-005 | 1년 유학으로 출국할 때 휴대폰 회선을 일시정지하려면 어떻게 하나요? | net_06_02 | net_05 | branch |
| I-PAYROLL-001 | 수습 3개월 동안 첫급여가 기본급의 90%로 지급되는 조건은? | payroll_02_02 | career_05_04 | domain |
| I-PENSION-002 | 본인 노령연금과 배우자 유족연금을 동시에 받을 수 있나요? | pension_03_03_1 | pension_03 | shallow |
| I-PENSION-004 | 육아휴직이나 장기휴직 기간도 DB형 근속연수에 포함되나요? | pension_05_01 | pension_04_02_2 | branch |
| I-PHONE-002 | 밤에 스마트폰 화면이 너무 하얘서 따뜻한 색온도로 바꾸려면 어떻게 하나요? | phone_02_03_2 | phone_02_03 | shallow |
| I-SHOP-003 | 배송비 쿠폰과 상품 할인쿠폰을 같은 주문에 같이 쓸 수 있나요? | shop_03_04 | member_05_02_2 | domain |
| I-SHOP-005 | 국내상품과 해외직구 상품을 한 번에 같이 주문할 수 있나요? | shop_06_01 | shop_01_01 | branch |
| S-ABROAD-007 | 할인표로 산 비행기 타는 날 아침에 갑자기 못 가게 되면 낸 돈은 한 푼도 못 되돌려받나요? | abroad_01_04_1 | abroad_02 · top abroad_02_05#3 0.73 recommended | route branch, wrong top1 |
| S-ABROAD-009 | 왕복 귀국편 변경 차액 | abroad_01_04_2 | abroad_02_01 · top abroad_02_01#5 0.81 recommended | route branch, wrong top1 |
| S-ABROAD-029 | 다음 달 인천에서 두바이 경유해서 런던 가는 표를 샀어요. 경유가 껴서 짐 규정이 첫 구간 기준인지 헷갈려요. 여정 상세에서는 뭘 순서대로 보면 | abroad_03_01_2 | abroad_03_01 · top abroad_03_01_2#5 0.83 recommended | route shallow |
| S-ABROAD-055 | 장기체류 비자 체류목적 기재 | abroad_04_04_1 | abroad_04 · top abroad_04_04_1#11 0.85 recommended | route shallow |
| S-ABROAD-057 | 기내식 햄샌드위치 안 먹고 챙겼는데 육포랑 같이 그냥 들고 나가도 돼? | abroad_04_04_2 | abroad_03_02 · top abroad_04_04_2#3 0.78 recommended | route branch |
| S-ABROAD-072 | 어제 시내 환급처에서 카드 보증 걸고 돈 먼저 받았거든요 근데 출국할 때 세관 확인 깜빡하면 카드에서 다시 빠져나가는 거예요? | abroad_05_04 · no answer | abroad_05_04 · top abroad_05_04#10 0.66 recommended | answered unanswerable |
| S-ABROAD-073 | 유럽 5박6일 가는데 환율 결제날짜 걸로 해? 아님 돌아와서 정산하는 날 걸로 해? 뭐가 나아? | abroad_05_05 | abroad_05 · top abroad_05_05#2 0.85 recommended | route shallow |
| S-ABROAD-081 | 이웃 세대에서 하는 인테리어 작업이 너무 시끄러워 밤에 눈 붙이기가 힘든데 어떻게 해결하고 보상받나요? | abroad_06_02_2 | legal_03 · top legal_03_01#5 0.54 alternative | route domain, wrong top1 |
| S-ABROAD-094 | 스마트폰 보상청구 순서 | abroad_07_02 | phone_07_03 · top phone_07_03#5 0.67 recommended | route domain, wrong top1 |
| S-APPLIANCE-017 | 그거 물 고이면 어떡해? 거기 잎채소랑 과일 고습 저습 어떻게 나눠 넣음? | appliance_02_02 | cook_03 · top appliance_02_02#6 0.93 recommended | route domain |
| S-APPLIANCE-018 | 지난주 이사하고 냉장고 자리 잡았는데 문이 자꾸 저절로 벌어져요. 앞다리 수평이 안 맞는 느낌인데 기울기가 이상해요. 조절다리 어느 방향으로 돌 | appliance_02_02 · no answer | appliance_02 · top appliance_02_02#8 0.46 alternative | route shallow, answered unanswerable |
| S-APPLIANCE-028 | 30일 지난 김장독이 새콤해졌는데 추가 발효 늦추려면 어쩌죠? | appliance_02_06 | cook_03 · top appliance_02_06#3 0.87 recommended | route domain |
| S-APPLIANCE-039 | 2021년형 삼성 스탠드 에어컨 17평형인데 3일 전에 극세필터 물세척해서 끼웠거든요. 오늘 25도 냉방으로 1시간 틀었더니 시큼한 냄새가 계속 | appliance_03_05_1 | appliance_03_02 · top appliance_03_05_1#4 0.89 recommended | route branch |
| S-APPLIANCE-041 | 작년 여름엔 매일 8시간 넘게 틀었어요. 올해는 냄새는 없는데 미리 점검받고 싶어요. 몇 년에 한 번씩 받는 게 적당하고 언제가 한가해요? | appliance_03_05_2 | utility_03_02 · top appliance_03_05_2#3 0.94 recommended | route domain |
| S-APPLIANCE-044 | 약하게나 보통으로 돌릴 때는 멀쩡한데 그게 아니라 제일 센 강력으로 올리면 1분도 안 돼서 바로 꺼져버려요 | appliance_04_01 | pc_02_04 · top pc_02_04#2 0.63 alternative | route domain, wrong top1 |
| S-APPLIANCE-045 | 한겨울에 창가 쪽 창고에 보관했더니 금방 방전돼서 멈추는데 이유가 뭘까요? | appliance_04_02 | car_02_04 · top car_01_04#6 0.75 recommended | route domain, wrong top1 |
| S-APPLIANCE-065 | 초등생 애가 밤마다 야한 영상 멋대로 틀까 봐 걱정돼요 비번 눌러야 켜지도록 차단하는 기능 있어요? | appliance_05_03 | parent_07_03_2 · top parent_07_03_2#6 0.68 recommended | route domain, wrong top1 |
| S-APPLIANCE-072 | 야간모드 음량제한 대화선명 | appliance_05_05 | phone_02_03 · top appliance_05_05#2 0.93 recommended | route domain |
| S-APPLIANCE-083 | 깨끗이 훔쳐냈는데도 묵은 향이 가시지 않아요. 시판 약품 없이 평소 문 틈 벌려두고 바람 통하게 두는 요령이 궁금해요. | appliance_06_05 | appliance_01_04 · top appliance_01_04#5 0.80 recommended | route branch, wrong top1 |
| S-APPLIANCE-088 | 출근 전에 reservation 걸어서 오후 5시에 finish 되게 설정 가능해요? | appliance_07_02 | office_06_01_1 · top office_06_01_1#1 0.41 alternative | route domain, wrong top1 |
| S-APPLIANCE-090 | 밤 저소음 건조불량 | appliance_07_03_1 | appliance_01 · top appliance_07_03_1#7 0.96 recommended | route branch |
| S-APPLIANCE-111 | 식기세척기 세척 결과가 이상해요 | appliance_07_03 | appliance_07 · top appliance_07#article 0.90 recommended | route shallow, wrong top1 |
| S-BANK-003 | 기한이 끝난 패스포트나 폰에 저장해둔 주민증 사진으로 통과될까요? | bank_01_01_1 · no answer | abroad_04 · top civil_06_04#7 0.83 recommended | route domain, answered unanswerable |
| S-BANK-007 | 계좌개설 첫입금 금액 | bank_01_02 | bank_01 · top bank_01_02#10 0.91 recommended | route shallow |
| S-BANK-013 | 한도계좌 걸렸는데 풀려면 뭐 해야됨? | bank_01_05 | bank_04_02 · top bank_04_02#5 0.85 recommended | route branch, wrong top1 |
| S-BANK-026 | 보험료 나가는 날이 월급 들어오기 전이라 맨날 잔부족 뜸ㅠ 뒤로 미룰 수 있음? | bank_02_04_2 | insure_06_02 · top bank_02_04_2#3 0.91 recommended | route domain |
| S-BANK-045 | 와이파이로만 쓰는 태블릿이 하나 있는데 은행 이체할 때 쓰려고 해요. 폰 말고 이걸로 인증번호 생성하도록 만들 수 있는지 궁금합니다. | bank_04_03_1 | bank_04_03 · top bank_04_03_1#7 0.91 recommended | route shallow |
| S-BANK-046 | 업무용이랑 일상용 기기를 번갈아 쓰는데 양쪽 모두에 인증수단을 함께 설정해 쓸 수 있을까요? | bank_04_03_1 | security_02 · top security_02_02_3#11 0.66 recommended | route domain, wrong top1 |
| S-BANK-057 | The expected refund date for my mistaken transfer has passed but the money still | bank_05_02 | bank_05_02 · top bank_05_02#4 0.91 recommended | wrong top1 |
| S-BANK-062 | 외국으로 돈 보내는 거 신청해놨는데 아직 발송되기 전이면 철회할 수 있어요? | bank_05_05 | bank_06_04 · top bank_06_04#7 0.89 recommended | route branch, wrong top1 |
| S-BANK-063 | 월급 여러 명꺼 한 번에 보내기 직전인데 전부 취소됨? | bank_05_05 | payroll · top payroll_04_05#7 0.45 alternative | route domain, wrong top1 |
| S-BANK-079 | 은행 안 가고 비대면으로 처음 발급돼? | bank_07_02_1 | bank_01_01 · top bank_07_02_1#7 0.89 recommended | route branch |
| S-BANK-082 | 인증서 비번 바꿨는데 다시 등록해야됨? | bank_07_02_2 | bank_07_02 · top bank_07_02_2#37 0.94 recommended | route shallow |
| S-BANK-097 | 만기일이 weekend랑 겹치는데 payout 언제 들어와요? | bank_08_04 | insure_07_04 · top bank_08_04#2 0.87 recommended | route domain |
| S-BANK-102 | 안 쓰는 계좌들 전체적으로 정리하고 싶어요 | bank_02 | bank_02 · top bank_02_03#2 0.82 recommended | wrong top1 |
| S-BANK-107 | 돈을 잘못 보낸 것 같은데 어떡하죠 | bank_05 | bank_05 · top bank_05_01#1 0.79 recommended | wrong top1 |
| S-CAR-003 | 타이어 공기압 경고등 대처 | car_01_02_1 | car_02_03 · top car_02_03#5 0.83 recommended | route branch, wrong top1 |
| S-CAR-038 | 어제 오후 3시쯤 대형마트 지하주차장에서 후진 빼다가 통로 지나던 차랑 부딪혔어요. 상대는 제가 100% 잘못이라는데 후진·문열림 과실비율표 어 | car_03_02 · no answer | insure_03_04 · top insure_03_04#2 0.80 recommended | route domain, answered unanswerable |
| S-CAR-047 | 필기 떨어졌어ㅠ 내일 바로 또 볼 수 있음? 횟수 제한 같은 거 있어? | car_04_02 | career_03_03 · abstained · top career_03_03_2#8 0.23 reference | route domain, abstained |
| S-CAR-066 | 리스차 승게받으려고 하는데 어케 내명의로 바꾸나요? 리스회사 동의서 받아야되나여ㅠ 신용심사 통과해야된다던데 수수료도 있나요? | car_05_03 | car_05 · top car_05_03#10 0.93 recommended | route shallow |
| S-CAR-084 | 밧데리 0프로까지 방전됐는데 괜찬아요 경고등떴음 어케해야돼 ㅠㅠ | car_07_03 | car_02_04 · top car_02_04#article 0.74 recommended | route branch, wrong top1 |
| S-CAR-089 | 겨울타이어 교체시기 공기압 | car_08_01 | car_01_02 · top car_08_01#8 0.96 recommended | route branch |
| S-CAR-108 | 과태료 관련해서 전반적으로 알려주세요 | car_06_03 | car_06 · top car_06#article 0.80 recommended | route shallow, wrong top1 |

