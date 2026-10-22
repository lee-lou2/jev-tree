# 평가 결과 — 20261021-152819-jev

- 실행: 2026-10-21T15:28:19Z → 2026-10-21T15:31:17Z (UTC) · git `0d26b76` (커밋 안 된 변경 있음)
- 데이터셋: `large-kb` v1.0.0 · kb `ebdb61493e54` · cases `ddb5a5ae0f57`
- 설정: routers ["jev"] · variants ["full"] · suites ["all"] · split all · repeat 1 · concurrency 6 · beam_width "case/default" · limit "case/default"
- 모델(jev): jev `jev-preview`
- 필터: --sample 500 --seed final
- 메모: large: final jev on jev-preview

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev | full | 481 | 83.0% | 70.5% | 0.779 | 83.0% | 0.843 | 0.834 | 0.677 | 17.5% | 5.5% | 1.0s / 8.6s | 4.5 | 1 | 78.9% | - |

## jev · full

### 검색 (481건, 오류 1건)

오류 코드: upstream_error 1

| 지표 | 값 |
|---|---|
| E2E 정답률 | 83.0% [79.3–86.0] (n=481) |
|   └ 답할 수 있는 질문 | 83.0% [79.2–86.2] (n=441) |
|   └ 답할 수 없는 질문(보류가 정답) | 82.5% [68.0–91.3] (n=40) |
| 확신 정답률 (top1 정답 + recommended) | 81.2% [77.3–84.6] (n=441) |
| 라우팅 정확도 (정확 일치) | 70.5% [66.3–74.4] (n=481) |
| 계층 F1 (hF1) | 0.779 [0.744–0.813] (n=481) |
| 트리 밖 판정 precision | 65.6% [48.3–79.6] (n=32) |
| 트리 밖 판정 recall | 70.0% [52.1–83.3] (n=30) |
| 트리 밖 판정 F1 | 0.677 |
| 오보류율 (답할 수 있는데 보류) | 5.5% [3.7–8.0] (n=440) |
| 오답변율 (답할 수 없는데 답함) | 17.5% [8.7–32.0] (n=40) |
| Hit@1 | 83.0% [79.2–86.2] (n=441) |
| Hit@3 | 85.7% [82.1–88.7] (n=441) |
| Hit@k (limit) | 86.2% [82.6–89.1] (n=441) |
| MRR@k | 0.843 [0.810–0.876] (n=441) |
| nDCG@k | 0.834 [0.801–0.866] (n=441) |
| Recall@k | 0.861 [0.828–0.893] (n=441) |
| recommended 정밀도 | 0.849 [0.818–0.881] (n=398) |
| 라우팅이 맞았을 때 Hit@1 | 97.1% [94.6–98.5] (n=311) |
| 라우팅이 맞았을 때 MRR | 0.981 [0.968–0.993] (n=311) |
| 지연 p50 / p90 / p95 / 평균 | 1.0s / 3.8s / 8.6s / 2.1s |
| 라우팅 구간 평균 | 1.3s |
| 랭킹 후보 수 평균 | 44.8 |
| Jev 입력 토큰 평균 (과금) | 39054 |
| LLM 토큰 평균 (입력+출력) | 0 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 4.49 |
| LLM 라우팅 호출 수 평균 | 0.00 |
| LLM 실패 → Jev 빔 대체율 | - |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 84.7% (n=450) | 77.6% (n=450) | 71.9% (n=417) | 65.2% (n=135) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | domain | error | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|---:|
| 34 | 60 | 1 | 354 | 12 | 9 | 30 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 66 | 77.3% | 86.4% | 77.3% | 0.836 | 1.0s |
| leaf | 375 | 84.0% | 67.7% | 84.0% | 0.844 | 1.0s |
| no_answer | 10 | 50.0% | 70.0% | - | - | 0.9s |
| oos | 30 | 93.3% | 70.0% | - | - | 1.2s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 49 | 93.9% | 77.6% | 93.8% | 0.938 | 1.2s |
| single | 432 | 81.7% | 69.7% | 81.7% | 0.831 | 1.0s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 432 | 81.7% | 69.7% | 81.7% | 0.831 | 1.0s |
| coref | 9 | 100.0% | 77.8% | 100.0% | 1.000 | 1.3s |
| correct | 10 | 100.0% | 90.0% | 100.0% | 1.000 | 1.2s |
| ellipsis | 4 | 100.0% | 75.0% | 100.0% | 1.000 | 1.7s |
| refine | 9 | 77.8% | 44.4% | 77.8% | 0.778 | 1.6s |
| shift | 9 | 100.0% | 100.0% | 100.0% | 1.000 | 0.9s |
| slot | 8 | 87.5% | 75.0% | 87.5% | 0.875 | 1.2s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 481 | 83.0% | 70.5% | 83.0% | 0.843 | 1.0s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 66 | 77.3% | 86.4% | 77.3% | 0.836 | 1.0s |
| leaf | 385 | 83.1% | 67.8% | 84.0% | 0.844 | 1.0s |
| none | 30 | 93.3% | 70.0% | - | - | 1.2s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 30 | 93.3% | 70.0% | - | - | 1.2s |
| d2 | 33 | 75.8% | 84.8% | 75.8% | 0.813 | 1.0s |
| d3 | 282 | 81.9% | 71.6% | 82.8% | 0.838 | 1.0s |
| d4 | 136 | 84.6% | 64.7% | 85.2% | 0.859 | 1.1s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 30 | 93.3% | 70.0% | - | - | 1.2s |
| 101+ | 2 | 50.0% | 50.0% | 50.0% | 0.500 | 12.6s |
| 2-5 | 48 | 85.4% | 75.0% | 85.4% | 0.865 | 1.1s |
| 33-100 | 49 | 79.6% | 81.6% | 79.6% | 0.838 | 1.0s |
| 6-32 | 352 | 82.4% | 68.5% | 83.3% | 0.842 | 1.0s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| colloquial | 96 | 81.2% | 70.8% | 83.0% | 0.835 | 1.0s |
| condition | 29 | 79.3% | 65.5% | 78.6% | 0.786 | 1.0s |
| english | 13 | 92.3% | 76.9% | 92.3% | 0.923 | 1.1s |
| keyword | 28 | 64.3% | 39.3% | 66.7% | 0.667 | 1.1s |
| long | 61 | 91.8% | 80.3% | 93.3% | 0.933 | 1.0s |
| mixed | 15 | 73.3% | 66.7% | 73.3% | 0.767 | 1.0s |
| negation | 13 | 84.6% | 61.5% | 84.6% | 0.846 | 1.0s |
| oos_far | 17 | 94.1% | 82.4% | - | - | 0.6s |
| oos_near | 13 | 92.3% | 53.8% | - | - | 3.3s |
| paraphrase | 106 | 84.0% | 65.1% | 83.5% | 0.840 | 1.2s |
| typo | 24 | 91.7% | 70.8% | 95.5% | 0.955 | 1.2s |
| vague | 66 | 77.3% | 86.4% | 77.3% | 0.836 | 1.0s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| abroad | 15 | 100.0% | 80.0% | 100.0% | 1.000 | 0.8s |
| appliance | 13 | 61.5% | 61.5% | 61.5% | 0.641 | 0.9s |
| bank | 19 | 94.7% | 78.9% | 94.7% | 0.974 | 0.8s |
| car | 7 | 100.0% | 71.4% | 100.0% | 1.000 | 0.9s |
| card | 21 | 85.7% | 71.4% | 85.0% | 0.850 | 0.9s |
| career | 15 | 86.7% | 53.3% | 85.7% | 0.857 | 0.8s |
| civil | 12 | 66.7% | 66.7% | 66.7% | 0.715 | 0.9s |
| cook | 14 | 85.7% | 92.9% | 92.3% | 0.962 | 0.8s |
| fitness | 21 | 85.7% | 85.7% | 85.7% | 0.881 | 0.9s |
| health | 9 | 100.0% | 88.9% | 100.0% | 1.000 | 0.8s |
| homecare | 17 | 82.4% | 47.1% | 82.4% | 0.824 | 0.9s |
| house | 12 | 66.7% | 66.7% | 70.0% | 0.700 | 1.0s |
| hr | 14 | 78.6% | 64.3% | 78.6% | 0.786 | 0.9s |
| insure | 13 | 61.5% | 46.2% | 61.5% | 0.615 | 0.8s |
| itdesk | 16 | 81.2% | 75.0% | 86.7% | 0.867 | 1.0s |
| legal | 15 | 73.3% | 46.7% | 71.4% | 0.750 | 1.1s |
| loan | 9 | 77.8% | 66.7% | 77.8% | 0.778 | 0.8s |
| medical | 14 | 92.9% | 71.4% | 92.9% | 0.929 | 1.4s |
| member | 23 | 82.6% | 60.9% | 82.6% | 0.841 | 1.6s |
| net | 10 | 90.0% | 70.0% | 90.0% | 0.900 | 9.2s |
| office | 11 | 81.8% | 63.6% | 81.8% | 0.848 | 1.4s |
| oos | 30 | 93.3% | 70.0% | - | - | 1.2s |
| parent | 12 | 100.0% | 91.7% | 100.0% | 1.000 | 4.3s |
| payroll | 18 | 66.7% | 72.2% | 70.6% | 0.706 | 2.4s |
| pc | 8 | 100.0% | 100.0% | 100.0% | 1.000 | 1.7s |
| pension | 21 | 85.7% | 81.0% | 85.7% | 0.873 | 2.5s |
| pet | 12 | 91.7% | 75.0% | 91.7% | 0.917 | 1.7s |
| phone | 6 | 83.3% | 83.3% | 83.3% | 0.833 | 1.8s |
| returns | 14 | 71.4% | 42.9% | 71.4% | 0.750 | 3.8s |
| security | 12 | 75.0% | 66.7% | 81.8% | 0.864 | 4.9s |
| shop | 9 | 88.9% | 66.7% | 88.9% | 0.889 | 1.5s |
| tax | 10 | 70.0% | 70.0% | 70.0% | 0.750 | 1.4s |
| transit | 15 | 86.7% | 86.7% | 86.7% | 0.867 | 1.4s |
| utility | 14 | 71.4% | 78.6% | 71.4% | 0.732 | 1.1s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| hard | 66 | 77.3% | 86.4% | 77.3% | 0.836 | 1.0s |
| medium | 415 | 83.9% | 68.0% | 84.0% | 0.844 | 1.0s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 319 | 81.8% | 68.7% | 82.1% | 0.835 | 1.0s |
| test | 162 | 85.2% | 74.1% | 84.8% | 0.857 | 1.1s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 81.5% | 97.7% | 82.2% | 37.5% |
| 0.25 | 81.9% | 97.7% | 82.6% | 32.5% |
| 0.30 | 82.1% | 96.6% | 83.8% | 30.0% |
| 0.35 | 82.7% | 95.9% | 84.9% | 22.5% |
| 0.40 | 82.7% | 94.8% | 85.9% | 22.5% |
| 0.50 | 83.5% | 93.9% | 87.5% | 10.0% |
| 0.60 | 82.9% | 91.6% | 88.9% | 10.0% |
| 0.65 | 82.5% | 90.0% | 89.9% | 5.0% |

### 저장 (ingest, 19건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 78.9% [56.7–91.5] (n=19) |
|   └ 올바른 카테고리에 저장 | 78.9% [56.7–91.5] (n=19) |
|   └ 트리/루트 밖 Q&A 거절 | - |
| 계층 F1 | 0.807 [0.632–0.982] (n=19) |
| 초안이 검색에 안 보임 | 100.0% [82.4–100.0] (n=18) |
| 게시 성공 (버전 +1) | 100.0% [82.4–100.0] (n=18) |
| 초안→게시 카테고리 유지 | 100.0% [82.4–100.0] (n=18) |
| 중복 탐지 precision | - |
| 중복 탐지 recall | - |
| 갱신/신규 판정 일치 | 100.0% [82.4–100.0] (n=18) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 0 |
| 게시 후 probe 검색 top-k 포함 | - |
| 게시 후 probe 검색 top1 답변 | - |
| 지연 p50 (초안+게시+probe) | 0.8s |
| LLM 실패 → Jev 빔 대체율 | - |

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| null | car | 2 |
| abroad_03_01_2 | abroad_03_01 | 1 |
| abroad_04_04_1 | abroad_04 | 1 |
| abroad_04_04_2 | abroad_03_02 | 1 |
| appliance_04_01 | phone_01_02_2 | 1 |
| appliance_05_05 | phone | 1 |
| appliance_07_02 | itdesk_03_04 | 1 |
| appliance_07_03 | appliance_07 | 1 |
| appliance_07_03_1 | appliance_01 | 1 |
| bank_01_05 | bank | 1 |
| bank_02_04_2 | insure_06_02 | 1 |
| bank_07_02_2 | bank_07_02 | 1 |
| bank_08_04 | insure_07_04 | 1 |
| car_05_03 | car_05 | 1 |
| car_07_03 | car_02_04 | 1 |
| card_01_04 | null | 1 |
| card_02_01 | card_02 | 1 |
| card_04_04_1 | card_01 | 1 |
| card_05_01_1 | security_01 | 1 |
| card_05_01_2 | card_02_05 | 1 |

### 틀린 케이스 (158건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-INSURE-003 | 자동차등록증 사진을 업로드하라는 안내가 나왔을 때 제출 방법은요? | insure_02_05 | error invalid_request | false_none |
| I-INSURE-005 | 정기보험 만기가 되었는데 받은 만기축하금이 없는 이유는 무엇인가요? | insure_04_02 | insure_07_04 | branch |
| I-PAYROLL-001 | 수습 3개월 동안 첫급여가 기본급의 90%로 지급되는 조건은? | payroll_02_02 | career_05_04 | domain |
| I-SHOP-003 | 배송비 쿠폰과 상품 할인쿠폰을 같은 주문에 같이 쓸 수 있나요? | shop_03_04 | member_05_02_2 | domain |
| S-ABROAD-029 | 다음 달 인천에서 두바이 경유해서 런던 가는 표를 샀어요. 경유가 껴서 짐 규정이 첫 구간 기준인지 헷갈려요. 여정 상세에서는 뭘 순서대로 보면 | abroad_03_01_2 | abroad_03_01 · top abroad_03_01_2#5 0.80 recommended | route shallow |
| S-ABROAD-055 | 장기체류 비자 체류목적 기재 | abroad_04_04_1 | abroad_04 · top abroad_04_04_1#11 0.88 recommended | route shallow |
| S-ABROAD-057 | 기내식 햄샌드위치 안 먹고 챙겼는데 육포랑 같이 그냥 들고 나가도 돼? | abroad_04_04_2 | abroad_03_02 · top abroad_04_04_2#3 0.78 recommended | route branch |
| S-APPLIANCE-044 | 약하게나 보통으로 돌릴 때는 멀쩡한데 그게 아니라 제일 센 강력으로 올리면 1분도 안 돼서 바로 꺼져버려요 | appliance_04_01 | phone_01_02_2 · top phone_01_02_2#1 0.75 recommended | route domain, wrong top1 |
| S-APPLIANCE-072 | 야간모드 음량제한 대화선명 | appliance_05_05 | phone · abstained · top phone_03_02#1 0.39 reference | route domain, abstained |
| S-APPLIANCE-088 | 출근 전에 reservation 걸어서 오후 5시에 finish 되게 설정 가능해요? | appliance_07_02 | itdesk_03_04 · abstained · top itdesk_03_04#1 0.40 reference | route domain, abstained |
| S-APPLIANCE-090 | 밤 저소음 건조불량 | appliance_07_03_1 | appliance_01 · top transit_06_05#4 0.75 recommended | route branch, wrong top1 |
| S-APPLIANCE-111 | 식기세척기 세척 결과가 이상해요 | appliance_07_03 | appliance_07 · top appliance_07#article 0.89 recommended | route shallow, wrong top1 |
| S-BANK-013 | 한도계좌 걸렸는데 풀려면 뭐 해야됨? | bank_01_05 | bank · top bank_01_05#10 0.90 recommended | route shallow |
| S-BANK-026 | 보험료 나가는 날이 월급 들어오기 전이라 맨날 잔부족 뜸ㅠ 뒤로 미룰 수 있음? | bank_02_04_2 | insure_06_02 · top bank_02_04_2#3 0.91 recommended | route domain |
| S-BANK-082 | 인증서 비번 바꿨는데 다시 등록해야됨? | bank_07_02_2 | bank_07_02 · top bank_07_02_2#37 0.96 recommended | route shallow |
| S-BANK-097 | 만기일이 weekend랑 겹치는데 payout 언제 들어와요? | bank_08_04 | insure_07_04 · top bank_08_04#2 0.83 recommended | route domain |
| S-BANK-102 | 안 쓰는 계좌들 전체적으로 정리하고 싶어요 | bank_02 | bank_02 · top bank_02_03#2 0.82 recommended | wrong top1 |
| S-CAR-066 | 리스차 승게받으려고 하는데 어케 내명의로 바꾸나요? 리스회사 동의서 받아야되나여ㅠ 신용심사 통과해야된다던데 수수료도 있나요? | car_05_03 | car_05 · top car_05_03#10 0.93 recommended | route shallow |
| S-CAR-084 | 밧데리 0프로까지 방전됐는데 괜찬아요 경고등떴음 어케해야돼 ㅠㅠ | car_07_03 | car_02_04 · top car_07_03#6 0.85 recommended | route branch |
| S-CARD-015 | 영업점에서 그 자리에서 바로 뽑는 게 아니라 앱으로 우편 재발급 신청하려는데 돈은 얼마나 나가고 도착까지 며칠 걸려요? | card_01_04 | null · abstained · top no items | route false_none, abstained |
| S-CARD-019 | 그거 매달 문자로 받아보려면 어디 들어가서 켜야 돼요? | card_02_01 | card_02 · top card_02_01#14 0.91 recommended | route shallow |
| S-CARD-053 | 리볼빙 잔액 아직 있는데 새 카드 만들 수 있음? 한도 올리는 것도 안 돼? | card_04_04_1 | card_01 · abstained · top card_01_04#11 0.37 reference | route branch, abstained |
| S-CARD-057 | 간편비밀번호 잊음 재설정 | card_05_01_1 | security_01 · top card_05_01_1#5 0.91 recommended | route domain |
| S-CARD-058 | 미국 직구 사이트에서 150만원짜리 카메라 결제하려는데 국내 한도는 남았는데 계속 거절돼요. 해외이용한도가 따로 있는 건가요, 이럴 땐 어떻게  | card_05_01_2 | card_02_05 · top card_05_04#3 0.81 recommended | route branch, wrong top1 |
| S-CARD-077 | I want to keep using my own credit card but cancel only the supplementary card I | card_06_05 | card_01_03 · top card_06_05#7 0.94 recommended | route branch |
| S-CAREER-017 | 밖에 있는데 마감이 얼마 안 남아서 폰으로 원서를 내야 해요. 번호랑 메일주소를 잘못 적을까봐 걱정인데 미리 체크할 요령 있을까요? | career_01_05 · no answer | null · abstained · top no items | route false_none |
| S-CAREER-034 | 90초 영상답변 표정 시선 | career_03_03_1 | career_04 · top career_03_03_1#3 0.91 recommended | route branch |
| S-CAREER-044 | 회사에서 미는 가치 3개 있던데 면접 때 뭘 골라 말하는 게 좋음? 팁 좀 ㅠㅠ | career_04_02 | career_04_04 · top career_04_02#5 0.87 recommended | route branch |
| S-CAREER-059 | 작년에 회사 주식 싸게 사서 판 차액이 컸는데, 새 곳에 얘기하는 받는 돈 셈할 때 그 일회성 돈까지 합해서 말해야 할까요? | career_05_01_1 | career_05_01 · top career_05_01_1#16 0.51 alternative | route shallow |
| S-CAREER-060 | 옮겨갈 회사에서 예전에 다니던 곳 세금 종이 꼭 달라고 하는데, 이거 안 넘겨주면 법에 걸리거나 불이익 있나요? | career_05_01_1 | payroll_03_04 · top payroll_03_04_2#9 0.65 alternative | route domain, wrong top1 |
| S-CAREER-071 | 수습 탈락해서 2개월만에 짤리게 됐는데 퇴직금이랑 실업급여 받을 수 있음? | career_05_04 | pension_04_01 · top pension_04_01#2 0.74 recommended | route domain, wrong top1 |
| S-CAREER-114 | 내일배움카드가 뭔가요? | career_07_03 | career_07 · top career_07_03#article 0.44 alternative | route shallow |
| S-CIVIL-015 | 개명 등본 이전이름 | civil_01_05 | civil_01 · top civil_01_05#6 0.83 recommended | route shallow |
| S-CIVIL-025 | 세대주 사망 신고 기한 | civil_02_03 | civil · top civil_02_03#1 0.95 recommended | route shallow |
| S-CIVIL-035 | 아파서 외국 병원 진료가야됨 1회용으로 끊으려면 뭐 내야돼? 보호자도 같이 가는데 | civil_03_01_2 | medical_04 · abstained · top medical_04#article 0.29 reference | route domain, abstained |
| S-CIVIL-097 | 엄마 거동 불편해서 못 가는데 내가 대신 뽑아줄 수 있음? | civil_07_05 | civil_01_04 · top civil_01_04#10 1.00 recommended | route branch, wrong top1 |
| S-CIVIL-102 | 인터넷으로 전입신고 하는 방법 알려줘요 | civil_02_02 | civil_02_02 · top civil_02_02_1#1 0.85 recommended | wrong top1 |
| S-CIVIL-107 | 본인서명확인서 받으려는데 어떻게 해요? | civil_04_03 | civil_04_03 · top civil_04_03_1#3 0.78 recommended | wrong top1 |
| S-COOK-033 | 양념불고기 내일 구울건데 하루 재워둘겸 냉장 어떻게 함? 그냥 뚜껑 덮어두면 돼? | cook_03_03 · no answer | cook_03_03 · top cook_03_03#1 0.62 alternative | answered unanswerable |
| S-COOK-041 | 소분 냉동해둔 고기들 date 쓰고 first-in first-out으로 관리하는 labeling 팁 있어? expiration 안 넘기게 st | cook_04_01 | homecare_06_02 · top homecare_06_02#6 0.98 recommended | route domain, wrong top1 |
| S-FITNESS-026 | I'm totally new to working out and I can't figure out the hip hinge at all. Ever | fitness_03_02_2 | fitness_01_05 · top fitness_01_05#2 0.52 alternative | route branch, wrong top1 |
| S-FITNESS-064 | 리프팅벨트 세탁 관리 | fitness_06_02 | homecare_02 · top fitness_06_02#9 0.94 recommended | route domain |
| S-FITNESS-104 | 햇살 있는 시간에 짧게 산에 다녀오려는데 땀 잘 빠지는 옷차림이랑 미끄럽지 않은 발에 신는 거 위주로 배낭에 뭘 넣어 가면 좋을까요? 면 티 입 | fitness_09_04 | null · abstained · top no items | route false_none, abstained |
| S-FITNESS-108 | 엉덩이 운동이 고민인데 뭐부터 해요? | fitness_03_02 | fitness_03_02 · top fitness_03_02_1#1 0.75 recommended | wrong top1 |
| S-HEALTH-031 | 내일 중요한 발표가 있는데 생각만 하면 속이 울렁거리고 가슴이 답답해요. 긴장할 때마다 이래서 아무것도 못 하겠어요. 빨리 진정하려면 어떻게 해 | health_03_03 | health_07_04 · top health_03_03#6 0.93 recommended | route branch |
| S-HOMECARE-010 | 운동화에 묻은 거가 아니라 등산 다녀오고 방수 재킷이랑 등산복에 튄 흙탕물인데 어떻게 빨아야 돼요? 방수 옷은 섬유유연제 넣으면 안 된다던데 맞 | homecare_01_05 | homecare_02_04 · top homecare_01_05#5 0.90 recommended | route branch |
| S-HOMECARE-011 | 김치통을 한동안 안 열었더니 뚜껑이랑 고무패킹에 까만 점이 꼈어요. 냄새도 좀 나고 찝찝하네요. 패킹 분리해서 어떻게 닦고 소독해야 돼요? | homecare_01_05 | cook_08_03 · top cook_08_03#2 1.01 recommended | route domain, wrong top1 |
| S-HOMECARE-013 | 비 주룩주룩 올 때 기계 바람 안 쓰고 평평한 곳에 뉘라는 택 기호 봤는데 줄 한 개랑 줄 세 개는 뭐가 달라? 스웨터 늘어나? | homecare_02_01_1 | homecare_02_01 · top homecare_02_01_1#6 0.93 recommended | route shallow |
| S-HOMECARE-020 | 겨울에 매일 맨 울 목도리에 보풀이 잔뜩 일었어요. 끝에 술 장식이 있어서 올 나갈까봐 손대기 무서워요. 양면 다 어떻게 빗질하고 보관해야 할까 | homecare_02_03_2 | homecare_02_03 · top homecare_02_03_2#7 0.93 recommended | route shallow |
| S-HOMECARE-045 | 새 아파트에 막 들어왔는데 화장실 바닥 구멍에서 역한 악취가 자꾸 올라와요 어디를 살펴봐야 할까요? | homecare_04_04_2 | homecare_04_04 · top homecare_04_04_2#6 0.91 recommended | route shallow |
| S-HOMECARE-064 | 포대로 많이 사 온 밥 지을 곡류를 좀 안 생기게 나눠 담아 주방 창고에 두는 법은? | homecare_06_02 | cook_03_04_1 · top cook_03_04_1#8 0.81 recommended | route domain, wrong top1 |
| S-HOMECARE-072 | 음식물쓰레기 밀폐통 기름받이 | homecare_07_01_2 | homecare_08_02 · abstained · top homecare_08_02#5 0.30 reference | route branch, abstained |
| S-HOMECARE-094 | 신생아 때 입히던 작은 유아복들 나눠주려고 하는데 따로 빨래하는 방법 정해져 있어요? | homecare_08_05 | homecare · top homecare_08_05#14 0.94 recommended | route shallow |
| S-HOMECARE-113 | 크고 무거운 쓰레기 어떻게 버려요? | homecare_08_03 | homecare_08 · top homecare_08_03#article 0.75 recommended | route shallow |
| S-HOUSE-008 | 도장 찍기 전에 예약금 백만원 미리 송금하래요, 트러블 없이 전달하려면 어떻게 하면 될까요? | house_01_04 | house_02_03 · top house_02_03#2 0.71 recommended | route branch, wrong top1 |
| S-HOUSE-011 | 현관 비번키 지문 잘 먹는지 어케 봄? | house_01_05 | null · abstained · top no items | route false_none, abstained |
| S-HOUSE-029 | 신규계약 보증보험 신청기한 | house_03_03 · no answer | house_03_03 · top loan_04_03#2 0.86 recommended | answered unanswerable |
| S-HOUSE-074 | 지난달 15일에 이사했는데 갤럭시 보험사 앱에서 계약자 정보를 아직 안 고쳤어요. 300만 원 입원비 청구 서류가 옛날 집으로 가면 어떡하죠?  | house_08_04 | insure_06_01 · top insure_06_01#1 0.63 alternative | route domain, wrong top1 |
| S-HOUSE-077 | 그럼 언제 얼마나 하면 민원 안 들어와요? | house_08_05 | legal_03 · top house_08_05#11 0.94 recommended | route domain |
| S-HR-024 | 와이프 어제 출산했는데 출산휴가 언제까지 접수해야돼? | hr_02_04 | null · abstained · top no items | route false_none, abstained |

