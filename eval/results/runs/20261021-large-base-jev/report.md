# 평가 결과 — 20261021-150303-jev

- 실행: 2026-10-21T15:03:03Z → 2026-10-21T15:06:32Z (UTC) · git `0d26b76` (커밋 안 된 변경 있음)
- 데이터셋: `large-kb` v1.0.0 · kb `ebdb61493e54` · cases `ddb5a5ae0f57`
- 설정: routers ["jev"] · variants ["full"] · suites ["all"] · split all · repeat 1 · concurrency 6 · beam_width "case/default" · limit "case/default"
- 모델(jev): jev `jev-latest`
- 필터: --sample 1500 --seed final
- 메모: large: baseline jev (overlap recall)

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev | full | 1436 | 68.4% | 47.4% | 0.670 | 67.8% | 0.690 | 0.681 | 0.678 | 25.0% | 8.4% | 0.7s / 1.6s | 5.2 | 0 | 73.4% | - |

## jev · full

### 검색 (1436건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 68.4% [65.9–70.7] (n=1436) |
|   └ 답할 수 있는 질문 | 67.8% [65.2–70.3] (n=1316) |
|   └ 답할 수 없는 질문(보류가 정답) | 75.0% [66.6–81.9] (n=120) |
| 확신 정답률 (top1 정답 + recommended) | 66.9% [64.4–69.4] (n=1316) |
| 라우팅 정확도 (정확 일치) | 47.4% [44.9–50.0] (n=1436) |
| 계층 F1 (hF1) | 0.670 [0.649–0.690] (n=1436) |
| 트리 밖 판정 precision | 57.5% [48.0–66.5] (n=106) |
| 트리 밖 판정 recall | 82.4% [72.2–89.4] (n=74) |
| 트리 밖 판정 F1 | 0.678 |
| 오보류율 (답할 수 있는데 보류) | 8.4% [7.0–10.0] (n=1316) |
| 오답변율 (답할 수 없는데 답함) | 25.0% [18.1–33.4] (n=120) |
| Hit@1 | 67.8% [65.2–70.3] (n=1316) |
| Hit@3 | 70.1% [67.5–72.5] (n=1316) |
| Hit@k (limit) | 70.6% [68.1–73.0] (n=1316) |
| MRR@k | 0.690 [0.665–0.714] (n=1316) |
| nDCG@k | 0.681 [0.657–0.706] (n=1316) |
| Recall@k | 0.706 [0.681–0.731] (n=1316) |
| recommended 정밀도 | 0.770 [0.746–0.793] (n=1074) |
| 라우팅이 맞았을 때 Hit@1 | 97.7% [96.1–98.6] (n=601) |
| 라우팅이 맞았을 때 MRR | 0.986 [0.979–0.994] (n=601) |
| 지연 p50 / p90 / p95 / 평균 | 0.7s / 1.5s / 1.6s / 0.8s |
| 라우팅 구간 평균 | 0.5s |
| 랭킹 후보 수 평균 | 94.7 |
| Jev 입력 토큰 평균 (과금) | 45358 |
| LLM 토큰 평균 (입력+출력) | 0 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 5.16 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 81.3% (n=1362) | 60.7% (n=1362) | 46.6% (n=1261) | 42.3% (n=383) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|---:|
| 147 | 7 | 214 | 728 | 45 | 13 | 346 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 196 | 68.9% | 67.3% | 68.9% | 0.761 | 0.7s |
| leaf | 1120 | 67.6% | 41.9% | 67.6% | 0.677 | 0.7s |
| no_answer | 46 | 41.3% | 41.3% | - | - | 0.8s |
| oos | 74 | 95.9% | 82.4% | - | - | 0.4s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 162 | 68.5% | 35.2% | 71.2% | 0.712 | 0.8s |
| single | 1274 | 68.4% | 49.0% | 67.3% | 0.687 | 0.7s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 1274 | 68.4% | 49.0% | 67.3% | 0.687 | 0.7s |
| coref | 29 | 79.3% | 24.1% | 79.3% | 0.793 | 0.9s |
| correct | 29 | 62.1% | 44.8% | 69.2% | 0.692 | 0.7s |
| ellipsis | 21 | 66.7% | 28.6% | 66.7% | 0.667 | 0.8s |
| refine | 33 | 66.7% | 36.4% | 68.8% | 0.688 | 0.7s |
| shift | 30 | 73.3% | 50.0% | 80.8% | 0.808 | 0.7s |
| slot | 20 | 60.0% | 20.0% | 57.9% | 0.579 | 0.8s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 1436 | 68.4% | 47.4% | 67.8% | 0.690 | 0.7s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 196 | 68.9% | 67.3% | 68.9% | 0.761 | 0.7s |
| leaf | 1166 | 66.6% | 41.9% | 67.6% | 0.677 | 0.7s |
| none | 74 | 95.9% | 82.4% | - | - | 0.4s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 74 | 95.9% | 82.4% | - | - | 0.4s |
| d2 | 101 | 75.2% | 72.3% | 75.2% | 0.823 | 0.8s |
| d3 | 878 | 66.1% | 43.8% | 67.1% | 0.681 | 0.7s |
| d4 | 383 | 66.6% | 42.3% | 67.2% | 0.673 | 0.7s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 74 | 95.9% | 82.4% | - | - | 0.4s |
| 101+ | 4 | 100.0% | 75.0% | 100.0% | 1.000 | 1.0s |
| 2-5 | 159 | 71.7% | 48.4% | 71.2% | 0.716 | 0.8s |
| 33-100 | 151 | 67.5% | 62.3% | 68.0% | 0.738 | 0.8s |
| 6-32 | 1048 | 65.9% | 42.6% | 67.1% | 0.677 | 0.7s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| colloquial | 263 | 66.5% | 48.7% | 68.1% | 0.683 | 0.7s |
| condition | 95 | 66.3% | 46.3% | 67.7% | 0.677 | 0.7s |
| english | 45 | 68.9% | 11.1% | 68.2% | 0.682 | 1.4s |
| keyword | 100 | 62.0% | 50.0% | 60.8% | 0.608 | 0.7s |
| long | 166 | 75.3% | 57.8% | 76.9% | 0.769 | 0.7s |
| mixed | 55 | 67.3% | 32.7% | 67.9% | 0.679 | 0.8s |
| negation | 39 | 69.2% | 51.3% | 70.3% | 0.703 | 0.8s |
| oos_far | 41 | 97.6% | 92.7% | - | - | 0.3s |
| oos_near | 33 | 93.9% | 69.7% | - | - | 0.4s |
| paraphrase | 333 | 62.8% | 24.9% | 63.7% | 0.641 | 0.8s |
| typo | 70 | 67.1% | 62.9% | 69.2% | 0.692 | 0.7s |
| vague | 196 | 68.9% | 67.3% | 68.9% | 0.761 | 0.7s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| abroad | 38 | 71.1% | 52.6% | 73.0% | 0.743 | 0.7s |
| appliance | 39 | 69.2% | 51.3% | 71.1% | 0.724 | 0.8s |
| bank | 51 | 78.4% | 52.9% | 80.0% | 0.814 | 0.7s |
| car | 41 | 78.0% | 48.8% | 84.2% | 0.855 | 0.7s |
| card | 41 | 58.5% | 41.5% | 59.0% | 0.590 | 0.8s |
| career | 50 | 68.0% | 38.0% | 65.2% | 0.663 | 0.7s |
| civil | 42 | 64.3% | 52.4% | 65.0% | 0.677 | 0.8s |
| cook | 39 | 76.9% | 51.3% | 78.4% | 0.784 | 0.7s |
| fitness | 49 | 65.3% | 38.8% | 65.3% | 0.673 | 0.8s |
| health | 30 | 63.3% | 36.7% | 65.5% | 0.672 | 0.8s |
| homecare | 54 | 77.8% | 50.0% | 77.4% | 0.774 | 0.7s |
| house | 40 | 67.5% | 57.5% | 67.6% | 0.689 | 0.8s |
| hr | 36 | 75.0% | 50.0% | 75.0% | 0.750 | 0.7s |
| insure | 49 | 69.4% | 46.9% | 69.4% | 0.694 | 0.7s |
| itdesk | 51 | 64.7% | 35.3% | 63.3% | 0.633 | 0.8s |
| legal | 42 | 50.0% | 45.2% | 52.5% | 0.533 | 0.8s |
| loan | 36 | 66.7% | 38.9% | 66.7% | 0.688 | 0.7s |
| medical | 38 | 65.8% | 42.1% | 64.9% | 0.662 | 0.8s |
| member | 55 | 60.0% | 38.2% | 60.4% | 0.610 | 0.8s |
| net | 32 | 75.0% | 46.9% | 77.4% | 0.774 | 0.7s |
| office | 36 | 61.1% | 47.2% | 62.9% | 0.657 | 0.7s |
| oos | 74 | 95.9% | 82.4% | - | - | 0.4s |
| parent | 38 | 76.3% | 44.7% | 77.8% | 0.792 | 0.8s |
| payroll | 47 | 46.8% | 29.8% | 51.2% | 0.512 | 0.8s |
| pc | 30 | 80.0% | 56.7% | 80.0% | 0.828 | 0.8s |
| pension | 51 | 66.7% | 52.9% | 67.3% | 0.696 | 0.7s |
| pet | 36 | 75.0% | 33.3% | 75.0% | 0.769 | 0.8s |
| phone | 31 | 71.0% | 45.2% | 71.0% | 0.734 | 0.7s |
| returns | 53 | 54.7% | 47.2% | 56.0% | 0.587 | 0.7s |
| security | 37 | 56.8% | 37.8% | 55.6% | 0.556 | 0.8s |
| shop | 40 | 47.5% | 42.5% | 47.5% | 0.505 | 0.7s |
| tax | 28 | 71.4% | 50.0% | 71.4% | 0.714 | 0.7s |
| transit | 43 | 74.4% | 51.2% | 78.0% | 0.793 | 0.7s |
| utility | 39 | 71.8% | 53.8% | 73.0% | 0.730 | 0.7s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| hard | 196 | 68.9% | 67.3% | 68.9% | 0.761 | 0.7s |
| medium | 1240 | 68.3% | 44.3% | 67.6% | 0.677 | 0.7s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 978 | 67.4% | 47.1% | 67.1% | 0.685 | 0.7s |
| test | 458 | 70.5% | 48.0% | 69.2% | 0.699 | 0.7s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 66.8% | 95.8% | 67.9% | 44.2% |
| 0.25 | 67.0% | 95.1% | 68.6% | 41.7% |
| 0.30 | 67.5% | 93.9% | 69.8% | 35.0% |
| 0.35 | 68.2% | 92.4% | 71.4% | 27.5% |
| 0.40 | 68.8% | 91.3% | 72.8% | 20.0% |
| 0.50 | 69.4% | 87.2% | 76.6% | 10.8% |
| 0.60 | 69.4% | 83.2% | 80.3% | 7.5% |
| 0.65 | 69.2% | 81.1% | 82.0% | 5.8% |

### 저장 (ingest, 64건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 73.4% [61.5–82.7] (n=64) |
|   └ 올바른 카테고리에 저장 | 73.4% [61.5–82.7] (n=64) |
|   └ 트리/루트 밖 Q&A 거절 | - |
| 계층 F1 | 0.836 [0.761–0.912] (n=64) |
| 초안이 검색에 안 보임 | 100.0% [94.3–100.0] (n=64) |
| 게시 성공 (버전 +1) | 100.0% [94.3–100.0] (n=64) |
| 초안→게시 카테고리 유지 | 100.0% [94.3–100.0] (n=64) |
| 중복 탐지 precision | - |
| 중복 탐지 recall | - |
| 갱신/신규 판정 일치 | 100.0% [94.3–100.0] (n=64) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 0 |
| 게시 후 probe 검색 top-k 포함 | - |
| 게시 후 probe 검색 top1 답변 | - |
| 지연 p50 (초안+게시+probe) | 0.7s |
| LLM 실패 → Jev 빔 대체율 | - |

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| null | car | 3 |
| pet_02_04 | pet_02 | 3 |
| bank_06_03 | bank_06 | 2 |
| card_02_03 | card | 2 |
| career_03_03_1 | null | 2 |
| career_07_01_1 | career_07 | 2 |
| civil_01_05 | civil_01 | 2 |
| hr_08_01 | hr_08 | 2 |
| insure_01_01_1 | insure_01_03 | 2 |
| medical_01_01_1 | medical | 2 |
| medical_06_02 | medical | 2 |
| member_01_05 | member_01 | 2 |
| null | house | 2 |
| pet_01_02 | pet | 2 |
| tax_07_01 | tax | 2 |
| transit_05_04_2 | transit_05 | 2 |
| utility_02_03_2 | utility | 2 |
| abroad_01_02 | abroad | 1 |
| abroad_01_04_1 | abroad_02 | 1 |
| abroad_01_04_2 | abroad_02_01 | 1 |

### 틀린 케이스 (794건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-APPLIANCE-005 | 문 닫힘 표시가 깜빡이면서 작동이 바로 멈춰요 | appliance_06_03 | appliance | shallow |
| I-BANK-005 | 매달 나가는 자동송금 내역을 한 번에 조회하고 싶을 때 어디서 보나요? | bank_03_02 | bank_02_04 | branch |
| I-CAREER-003 | 이직이 잦은 경력 3년차가 임원 앞에서 퇴사 사유를 어떻게 말할까요? | career_04_02 | career_01_01_3 | branch |
| I-FITNESS-002 | 퇴근 후 밤 9시에 처음 러닝할 때 식사와 간격은 어떻게 두나요 | fitness_04_01 | fitness_08_04 | branch |
| I-HOUSE-004 | 임차권등기명령은 관할 법원에 직접 가야만 신청되나요? | house_05_02 | legal_05_02_1 | domain |
| I-INSURE-003 | 자동차등록증 사진을 업로드하라는 안내가 나왔을 때 제출 방법은요? | insure_02_05 | car_05 | domain |
| I-INSURE-005 | 정기보험 만기가 되었는데 받은 만기축하금이 없는 이유는 무엇인가요? | insure_04_02 | insure_07_04 | branch |
| I-LEGAL-002 | 진정 조사 중에 사업주가 합의하자고 하면 취하와 진행 중단은 어떻게 하나요? | legal_04_01_1 | legal_04 | shallow |
| I-LEGAL-003 | 체당금 지급까지 기간과 진행 상황 확인법은 어떻게 되나요? | legal_04_01_3 | legal_04_01 | shallow |
| I-LOAN-004 | 공동인증서 없이 간편인증으로 평가사 상세 보고서를 발급받는 순서가 궁금해요? | loan_01_05_2 | loan_01_01 | branch |
| I-NET-005 | 1년 유학으로 출국할 때 휴대폰 회선을 일시정지하려면 어떻게 하나요? | net_06_02 | net | shallow |
| I-PAYROLL-001 | 수습 3개월 동안 첫급여가 기본급의 90%로 지급되는 조건은? | payroll_02_02 | career_05_04 | domain |
| I-PENSION-002 | 본인 노령연금과 배우자 유족연금을 동시에 받을 수 있나요? | pension_03_03_1 | pension_03 | shallow |
| I-PENSION-004 | 육아휴직이나 장기휴직 기간도 DB형 근속연수에 포함되나요? | pension_05_01 | pension_04_02_2 | branch |
| I-PHONE-002 | 밤에 스마트폰 화면이 너무 하얘서 따뜻한 색온도로 바꾸려면 어떻게 하나요? | phone_02_03_2 | phone_02_03 | shallow |
| I-SHOP-003 | 배송비 쿠폰과 상품 할인쿠폰을 같은 주문에 같이 쓸 수 있나요? | shop_03_04 | member_05_02_2 | domain |
| I-SHOP-005 | 국내상품과 해외직구 상품을 한 번에 같이 주문할 수 있나요? | shop_06_01 | shop_01_01 | branch |
| S-ABROAD-003 | layover랑 stopover 차이가 헷갈려요. 24 hours 기준이라던데 schedule 짤 때 그냥 waiting 짧게 당일 연결로 할지 | abroad_01_02 | abroad · top abroad_01_02#10 0.95 recommended | route shallow |
| S-ABROAD-007 | 할인표로 산 비행기 타는 날 아침에 갑자기 못 가게 되면 낸 돈은 한 푼도 못 되돌려받나요? | abroad_01_04_1 | abroad_02 · top abroad_02_05#3 0.78 recommended | route branch, wrong top1 |
| S-ABROAD-009 | 왕복 귀국편 변경 차액 | abroad_01_04_2 | abroad_02_01 · top abroad_02_01#5 0.81 recommended | route branch, wrong top1 |
| S-ABROAD-029 | 다음 달 인천에서 두바이 경유해서 런던 가는 표를 샀어요. 경유가 껴서 짐 규정이 첫 구간 기준인지 헷갈려요. 여정 상세에서는 뭘 순서대로 보면 | abroad_03_01_2 | abroad_03_01 · top abroad_03_01_2#5 0.82 recommended | route shallow |
| S-ABROAD-042 | 열다섯 살 넘으면 누구나 비상문 옆 넓은 칸 이용할 수 있나요? 임신 중이거나 연세가 많으시면 제한되는지도 알고 싶어요. | abroad_03_05 | abroad_03 · top abroad_03_05#2 0.94 recommended | route shallow |
| S-ABROAD-044 | 그거 거기서 경고 뜨면 탑승 안 되는 거야? 그때는 어떡해야 돼? | abroad_04_01 | abroad_04 · top abroad_04_01#5 0.89 recommended | route shallow |
| S-ABROAD-048 | 뉴욕 가려고 사전허가 신청했다가 탈락 통보를 받았어요. 다음 달에 사증 없이 그대로 비행기 타고 갈 수 있나요? | abroad_04_03_1 | abroad_04 · top abroad_04_03_1#7 0.91 recommended | route shallow |
| S-ABROAD-055 | 장기체류 비자 체류목적 기재 | abroad_04_04_1 | abroad_04 · top abroad_04_04_1#11 0.87 recommended | route shallow |
| S-ABROAD-057 | 기내식 햄샌드위치 안 먹고 챙겼는데 육포랑 같이 그냥 들고 나가도 돼? | abroad_04_04_2 | abroad_03_02 · top abroad_03_02_1#10 0.35 reference | route branch, wrong top1 |
| S-ABROAD-059 | layover가 27시간이라 overnight을 해야 하는데 이건 무조건 immigration 받고 baggage 찾은 다음 outside에서  | abroad_04_05 | abroad_04 · top abroad_04_05#12 0.85 recommended | route shallow |
| S-ABROAD-061 | 토욜 밤에 앱으로 달러 사면 환율 언제꺼로 잡혀? | abroad_05_01 | abroad_05 · top abroad_05_01#6 0.94 recommended | route shallow |
| S-ABROAD-072 | 어제 시내 환급처에서 카드 보증 걸고 돈 먼저 받았거든요 근데 출국할 때 세관 확인 깜빡하면 카드에서 다시 빠져나가는 거예요? | abroad_05_04 · no answer | abroad_05_04 · top abroad_05_04#10 0.68 recommended | answered unanswerable |
| S-ABROAD-073 | 유럽 5박6일 가는데 환율 결제날짜 걸로 해? 아님 돌아와서 정산하는 날 걸로 해? 뭐가 나아? | abroad_05_05 | abroad_05_01 · top abroad_05_01#1 0.41 alternative | route branch, wrong top1 |
| S-ABROAD-074 | 네 명이 함께 떠나는 일정에서 모두가 부담하는 금액과 각자 부담하는 금액을 나눠서 정리하는 방법 알려주세요 | abroad_05_05 | null · abstained · top no items | route false_none, abstained |
| S-ABROAD-081 | 이웃 세대에서 하는 인테리어 작업이 너무 시끄러워 밤에 눈 붙이기가 힘든데 어떻게 해결하고 보상받나요? | abroad_06_02_2 | legal_03 · top legal_03_01#5 0.51 alternative | route domain, wrong top1 |
| S-ABROAD-094 | 스마트폰 보상청구 순서 | abroad_07_02 | phone_07_03 · top phone_07_03#5 0.58 alternative | route domain, wrong top1 |
| S-ABROAD-097 | Something disappeared from my hotel room safe, should I leave everything as it i | abroad_07_03 | null · abstained · top no items | route false_none, abstained |
| S-ABROAD-107 | 짐 싸는 게 걱정돼요 알려주세요 | abroad_03 | abroad_03_02 · top abroad_03_02#article 0.71 recommended | route deep, wrong top1 |
| S-ABROAD-117 | 여행 안전 준비 어떻게 해야 하죠? | abroad_07 | abroad · top abroad#article 0.75 recommended | route shallow, wrong top1 |
| S-APPLIANCE-004 | 빨래 돌리다가 멈췄는데 앞 뚜껑이 꽉 붙어서 안 따지고 세탁물도 못 꺼내겠어요, 고정 램프도 계속 들어와 있어요 | appliance_01_02 | appliance_01 · top appliance_01_02#5 0.86 recommended | route shallow |
| S-APPLIANCE-005 | 빨래 막판에 물기 빼는 단계에서 집에 있는 앞문형이 격하게 떨리더니 설치해 둔 자리에서 자꾸 벗어나요 왜 그런 거예요? | appliance_01_03 | appliance_01 · top appliance_01_03#1 0.91 recommended | route shallow |
| S-APPLIANCE-014 | 냉장실 안쪽 선반이 아니라 문짝에 붙은 포켓에 넣어둔 우유가 자꾸 미지근해지는데 온도 문제인가요, 어떻게 해야 돼요? | appliance_02_01 | appliance_02 · top appliance_02_01#7 0.95 recommended | route shallow |
| S-APPLIANCE-017 | 그거 물 고이면 어떡해? 거기 잎채소랑 과일 고습 저습 어떻게 나눠 넣음? | appliance_02_02 | cook_03 · top cook_03_01#1 0.67 recommended | route domain, wrong top1 |
| S-APPLIANCE-018 | 지난주 이사하고 냉장고 자리 잡았는데 문이 자꾸 저절로 벌어져요. 앞다리 수평이 안 맞는 느낌인데 기울기가 이상해요. 조절다리 어느 방향으로 돌 | appliance_02_02 · no answer | appliance_02 · top appliance_02_02#8 0.47 alternative | route shallow, answered unanswerable |
| S-APPLIANCE-028 | 30일 지난 김장독이 새콤해졌는데 추가 발효 늦추려면 어쩌죠? | appliance_02_06 | cook · abstained · top cook_07_02#8 0.35 reference | route domain, abstained |
| S-APPLIANCE-039 | 2021년형 삼성 스탠드 에어컨 17평형인데 3일 전에 극세필터 물세척해서 끼웠거든요. 오늘 25도 냉방으로 1시간 틀었더니 시큼한 냄새가 계속 | appliance_03_05_1 | appliance_03_02 · top appliance_03_02#7 0.70 recommended | route branch, wrong top1 |
| S-APPLIANCE-041 | 작년 여름엔 매일 8시간 넘게 틀었어요. 올해는 냄새는 없는데 미리 점검받고 싶어요. 몇 년에 한 번씩 받는 게 적당하고 언제가 한가해요? | appliance_03_05_2 | appliance_03 · top appliance_03_05_2#3 0.95 recommended | route shallow |
| S-APPLIANCE-045 | 한겨울에 창가 쪽 창고에 보관했더니 금방 방전돼서 멈추는데 이유가 뭘까요? | appliance_04_02 | car_02_04 · top car_02_04_2#27 0.71 recommended | route domain, wrong top1 |
| S-APPLIANCE-065 | 초등생 애가 밤마다 야한 영상 멋대로 틀까 봐 걱정돼요 비번 눌러야 켜지도록 차단하는 기능 있어요? | appliance_05_03 | parent_07 · top parent_07_03_2#6 0.76 recommended | route domain, wrong top1 |
| S-APPLIANCE-067 | 선 없이 폰에 나오는 걸 거실 티비로 띄워 보는데 자꾸 뚝뚝 멈춰요. 인터넷 박스 쪽은 어디를 손봐야 부드럽게 볼 수 있을까요? | appliance_05_04_1 | appliance_05_04 · top appliance_05_04_1#3 0.89 recommended | route shallow |
| S-APPLIANCE-072 | 야간모드 음량제한 대화선명 | appliance_05_05 | phone_02_03 · abstained · top phone_02_03_1#6 0.27 reference | route domain, abstained |
| S-APPLIANCE-083 | 깨끗이 훔쳐냈는데도 묵은 향이 가시지 않아요. 시판 약품 없이 평소 문 틈 벌려두고 바람 통하게 두는 요령이 궁금해요. | appliance_06_05 | homecare_03_02_2 · top homecare_03_02_2#2 0.52 alternative | route domain, wrong top1 |
| S-APPLIANCE-085 | 식세기 하단 소금넣는곳에 전용연수소금 한봉지 1키로쯤 물있어도 그냥 가득넣으면대죠? ㅋㅋ 집에있는 식용굵은소금 넣으면 안되는거 맞죠?? 보충등  | appliance_07_01 | appliance_07 · top appliance_07_01#8 0.87 recommended | route shallow |
| S-APPLIANCE-088 | 출근 전에 reservation 걸어서 오후 5시에 finish 되게 설정 가능해요? | appliance_07_02 | office_06_01_1 · top office_06_01_1#1 0.35 reference | route domain, wrong top1 |
| S-APPLIANCE-089 | 얘만 뽀송하게 하려면 뭘 켜야 돼요? | appliance_07_03_1 | appliance_07_02 · top appliance_07_02#4 0.88 recommended | route branch, wrong top1 |
| S-APPLIANCE-090 | 밤 저소음 건조불량 | appliance_07_03_1 | appliance_01_05 · top appliance_01_05#article 0.61 alternative | route branch, wrong top1 |
| S-APPLIANCE-100 | 냉장고가 이상한 것 같아요 봐주세요 | appliance_02 | appliance · top appliance_02#article 0.66 recommended | route shallow |
| S-APPLIANCE-111 | 식기세척기 세척 결과가 이상해요 | appliance_07_03 | appliance_07 · top appliance_07#article 0.91 recommended | route shallow, wrong top1 |
| S-BANK-003 | 기한이 끝난 패스포트나 폰에 저장해둔 주민증 사진으로 통과될까요? | bank_01_01_1 · no answer | abroad_04 · top abroad_04_02#6 0.38 reference | route domain, answered unanswerable |
| S-BANK-006 | Can I open a regular account at a branch with just my passport, or do I need to  | bank_01_02 | bank · top bank_01_02#5 0.92 recommended | route shallow |
| S-BANK-007 | 계좌개설 첫입금 금액 | bank_01_02 | bank_01 · top bank_01_02#10 0.91 recommended | route shallow |
| S-BANK-013 | 한도계좌 걸렸는데 풀려면 뭐 해야됨? | bank_01_05 | bank · top bank_01_05#10 0.86 recommended | route shallow |
| S-BANK-026 | 보험료 나가는 날이 월급 들어오기 전이라 맨날 잔부족 뜸ㅠ 뒤로 미룰 수 있음? | bank_02_04_2 | insure_06_02 · top insure_06_02_1#6 0.68 recommended | route domain, wrong top1 |

