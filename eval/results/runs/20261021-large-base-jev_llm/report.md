# 평가 결과 — 20261021-152355-jev_llm

- 실행: 2026-10-21T15:23:55Z → 2026-10-21T15:46:09Z (UTC) · git `0d26b76` (커밋 안 된 변경 있음)
- 데이터셋: `large-kb` v1.0.0 · kb `ebdb61493e54` · cases `ddb5a5ae0f57`
- 설정: routers ["jev_llm"] · variants ["full"] · suites ["all"] · split all · repeat 1 · concurrency 6 · beam_width "case/default" · limit "case/default"
- 모델(jev_llm): jev `jev-latest` · LLM `meta/muse-spark-1.3-contributor`
- 필터: --sample 600 --seed final
- 메모: large: baseline jev_llm (LLM routes every request; stepped walk above 800 nodes)

## 요약

| router | variant | 검색 n | E2E 정답률 | 라우팅 정확도 | hF1 | Hit@1 | MRR | nDCG | OOS F1 | 오답변율 | 오보류율 | p50 / p95 | Jev 호출/건 | 오류 | 저장 정확도 | 계약 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| jev_llm | full | 574 | 73.7% | 63.1% | 0.755 | 72.4% | 0.735 | 0.730 | 0.800 | 8.9% | 9.5% | 12.1s / 24.2s | 2.1 | 0 | 88.5% | - |

## jev_llm · full

### 검색 (574건, 오류 0건)

| 지표 | 값 |
|---|---|
| E2E 정답률 | 73.7% [69.9–77.1] (n=574) |
|   └ 답할 수 있는 질문 | 72.2% [68.2–75.9] (n=529) |
|   └ 답할 수 없는 질문(보류가 정답) | 91.1% [79.3–96.5] (n=45) |
| 확신 정답률 (top1 정답 + recommended) | 70.9% [66.9–74.6] (n=529) |
| 라우팅 정확도 (정확 일치) | 63.1% [59.0–66.9] (n=574) |
| 계층 F1 (hF1) | 0.755 [0.724–0.786] (n=574) |
| 트리 밖 판정 precision | 71.4% [56.4–82.8] (n=42) |
| 트리 밖 판정 recall | 90.9% [76.4–96.9] (n=33) |
| 트리 밖 판정 F1 | 0.800 |
| 오보류율 (답할 수 있는데 보류) | 9.5% [7.2–12.2] (n=529) |
| 오답변율 (답할 수 없는데 답함) | 8.9% [3.5–20.7] (n=45) |
| Hit@1 | 72.4% [68.4–76.0] (n=529) |
| Hit@3 | 74.5% [70.6–78.0] (n=529) |
| Hit@k (limit) | 75.0% [71.2–78.5] (n=529) |
| MRR@k | 0.735 [0.698–0.772] (n=529) |
| nDCG@k | 0.730 [0.693–0.766] (n=529) |
| Recall@k | 0.750 [0.714–0.787] (n=529) |
| recommended 정밀도 | 0.828 [0.795–0.861] (n=435) |
| 라우팅이 맞았을 때 Hit@1 | 97.2% [94.8–98.5] (n=324) |
| 라우팅이 맞았을 때 MRR | 0.983 [0.972–0.994] (n=324) |
| 지연 p50 / p90 / p95 / 평균 | 12.1s / 19.5s / 24.2s / 13.3s |
| 라우팅 구간 평균 | 13.0s |
| 랭킹 후보 수 평균 | 47.7 |
| Jev 입력 토큰 평균 (과금) | 12379 |
| LLM 토큰 평균 (입력+출력) | 6174 |
| Jev 호출 수 평균 (실측, 없으면 라운드 + 32개 배치 추정) | 2.08 |
| LLM 라우팅 호출 수 평균 | 1.93 |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–0.7] (n=574) |

**깊이별 경로 일치율** (forest 진입, 상위 k단계가 정답 경로와 같은 비율)

| | d1 | d2 | d3 | d4 |
|---|---:|---:|---:|---:|
| 일치율 | 84.3% (n=541) | 72.6% (n=541) | 61.5% (n=499) | 56.4% (n=163) |

**라우팅 결과 유형** (exact 정답 · shallow 너무 얕게 멈춤 · deep 너무 깊이 감 · branch 같은 영역 다른 갈래 · domain 다른 영역 · false_none 트리 밖으로 판단 · false_tree 트리 밖인데 착지 · error 실행 오류)

| branch | deep | domain | exact | false_none | false_tree | shallow |
|---:|---:|---:|---:|---:|---:|---:|
| 48 | 1 | 76 | 385 | 12 | 3 | 75 |

### 세부 분석

**기대 결과 유형**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 85 | 76.5% | 87.1% | 77.6% | 0.832 | 10.4s |
| leaf | 444 | 71.4% | 56.3% | 71.4% | 0.716 | 12.8s |
| no_answer | 12 | 75.0% | 66.7% | - | - | 15.3s |
| oos | 33 | 97.0% | 90.9% | - | - | 5.9s |

**턴 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| multi | 56 | 69.6% | 46.4% | 69.1% | 0.691 | 13.5s |
| single | 518 | 74.1% | 64.9% | 72.8% | 0.740 | 11.9s |

**문맥 패턴**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 518 | 74.1% | 64.9% | 72.8% | 0.740 | 11.9s |
| coref | 10 | 70.0% | 40.0% | 70.0% | 0.700 | 14.0s |
| correct | 13 | 76.9% | 61.5% | 75.0% | 0.750 | 17.1s |
| ellipsis | 5 | 40.0% | 0.0% | 40.0% | 0.400 | 17.5s |
| refine | 10 | 50.0% | 40.0% | 50.0% | 0.500 | 14.1s |
| shift | 9 | 100.0% | 77.8% | 100.0% | 1.000 | 11.5s |
| slot | 9 | 66.7% | 33.3% | 66.7% | 0.667 | 14.2s |

**진입점**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| forest | 574 | 73.7% | 63.1% | 72.4% | 0.735 | 12.1s |

**목표 노드 종류**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| internal | 85 | 76.5% | 87.1% | 77.6% | 0.832 | 10.4s |
| leaf | 456 | 71.5% | 56.6% | 71.4% | 0.716 | 12.9s |
| none | 33 | 97.0% | 90.9% | - | - | 5.9s |

**목표 깊이**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| - | 33 | 97.0% | 90.9% | - | - | 5.9s |
| d2 | 42 | 81.0% | 88.1% | 81.0% | 0.853 | 10.0s |
| d3 | 336 | 70.5% | 60.4% | 70.9% | 0.719 | 12.3s |
| d4 | 163 | 73.6% | 56.4% | 73.3% | 0.736 | 13.3s |

**착지 후보 수**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| 0 | 33 | 97.0% | 90.9% | - | - | 5.9s |
| 101+ | 2 | 100.0% | 100.0% | 100.0% | 1.000 | 11.6s |
| 2-5 | 65 | 73.8% | 60.0% | 73.8% | 0.738 | 11.8s |
| 33-100 | 62 | 79.0% | 82.3% | 79.0% | 0.825 | 10.6s |
| 6-32 | 412 | 70.9% | 58.3% | 71.0% | 0.719 | 12.8s |

**질의 스타일**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| colloquial | 111 | 70.3% | 57.7% | 71.3% | 0.713 | 13.1s |
| condition | 36 | 63.9% | 58.3% | 62.9% | 0.629 | 13.6s |
| english | 16 | 62.5% | 18.8% | 62.5% | 0.625 | 11.7s |
| keyword | 32 | 59.4% | 53.1% | 58.1% | 0.581 | 13.1s |
| long | 71 | 87.3% | 81.7% | 87.0% | 0.870 | 11.9s |
| mixed | 17 | 88.2% | 82.4% | 88.2% | 0.882 | 13.2s |
| negation | 16 | 81.2% | 68.8% | 81.2% | 0.812 | 13.4s |
| oos_far | 20 | 100.0% | 95.0% | - | - | 5.1s |
| oos_near | 13 | 92.3% | 84.6% | - | - | 7.1s |
| paraphrase | 127 | 66.1% | 38.6% | 66.1% | 0.669 | 13.4s |
| typo | 30 | 73.3% | 70.0% | 71.4% | 0.714 | 12.4s |
| vague | 85 | 76.5% | 87.1% | 77.6% | 0.832 | 10.4s |

**영역**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| abroad | 19 | 84.2% | 52.6% | 84.2% | 0.842 | 12.2s |
| appliance | 16 | 93.8% | 56.2% | 93.8% | 0.938 | 12.1s |
| bank | 25 | 68.0% | 48.0% | 68.0% | 0.680 | 10.8s |
| car | 11 | 90.9% | 90.9% | 90.0% | 0.900 | 12.1s |
| card | 23 | 73.9% | 60.9% | 72.7% | 0.727 | 13.3s |
| career | 16 | 56.2% | 56.2% | 66.7% | 0.700 | 13.7s |
| civil | 13 | 53.8% | 61.5% | 53.8% | 0.564 | 12.1s |
| cook | 16 | 87.5% | 62.5% | 93.3% | 0.933 | 12.2s |
| fitness | 24 | 66.7% | 58.3% | 66.7% | 0.708 | 12.4s |
| health | 11 | 72.7% | 54.5% | 70.0% | 0.764 | 11.5s |
| homecare | 24 | 79.2% | 50.0% | 79.2% | 0.792 | 12.8s |
| house | 15 | 73.3% | 73.3% | 69.2% | 0.692 | 13.1s |
| hr | 17 | 76.5% | 70.6% | 76.5% | 0.765 | 11.9s |
| insure | 16 | 68.8% | 56.2% | 68.8% | 0.688 | 11.4s |
| itdesk | 22 | 72.7% | 45.5% | 71.4% | 0.714 | 13.5s |
| legal | 16 | 68.8% | 62.5% | 66.7% | 0.700 | 14.7s |
| loan | 10 | 70.0% | 50.0% | 70.0% | 0.700 | 11.6s |
| medical | 16 | 68.8% | 62.5% | 66.7% | 0.667 | 13.8s |
| member | 26 | 65.4% | 57.7% | 65.4% | 0.692 | 13.8s |
| net | 12 | 66.7% | 58.3% | 66.7% | 0.667 | 15.2s |
| office | 12 | 83.3% | 75.0% | 83.3% | 0.875 | 12.3s |
| oos | 33 | 97.0% | 90.9% | - | - | 5.9s |
| parent | 14 | 92.9% | 85.7% | 92.9% | 0.929 | 11.9s |
| payroll | 20 | 50.0% | 55.0% | 52.6% | 0.535 | 13.5s |
| pc | 11 | 100.0% | 100.0% | 100.0% | 1.000 | 10.8s |
| pension | 24 | 83.3% | 83.3% | 83.3% | 0.858 | 12.5s |
| pet | 15 | 93.3% | 66.7% | 93.3% | 0.933 | 11.7s |
| phone | 10 | 100.0% | 90.0% | 100.0% | 1.000 | 10.8s |
| returns | 16 | 50.0% | 43.8% | 50.0% | 0.500 | 16.2s |
| security | 14 | 57.1% | 50.0% | 53.8% | 0.538 | 18.0s |
| shop | 11 | 54.5% | 54.5% | 54.5% | 0.545 | 11.9s |
| tax | 13 | 53.8% | 61.5% | 53.8% | 0.577 | 10.8s |
| transit | 16 | 87.5% | 75.0% | 87.5% | 0.875 | 11.3s |
| utility | 17 | 41.2% | 41.2% | 41.2% | 0.412 | 12.0s |

**난이도**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| hard | 85 | 76.5% | 87.1% | 77.6% | 0.832 | 10.4s |
| medium | 489 | 73.2% | 58.9% | 71.4% | 0.716 | 12.6s |

**split**

| 값 | n | E2E | 라우팅 | Hit@1 | MRR | p50 |
|---|---:|---:|---:|---:|---:|---:|
| dev | 379 | 75.5% | 64.1% | 74.5% | 0.756 | 12.0s |
| test | 195 | 70.3% | 61.0% | 68.3% | 0.694 | 12.2s |

**보류 임계값 스윕** (top 점수 < t 이면 보류한다고 가정. 현재 엔진은 0.30)

| t | E2E | 답변 커버리지 | 답변 정밀도 | 오답변율 |
|---:|---:|---:|---:|---:|
| 0.20 | 72.5% | 96.4% | 73.4% | 26.7% |
| 0.25 | 73.0% | 94.9% | 75.0% | 20.0% |
| 0.30 | 73.3% | 94.1% | 75.8% | 15.6% |
| 0.35 | 73.5% | 92.4% | 77.4% | 13.3% |
| 0.40 | 74.0% | 90.9% | 79.1% | 6.7% |
| 0.50 | 73.5% | 87.7% | 81.4% | 6.7% |
| 0.60 | 73.7% | 84.1% | 84.9% | 0.0% |
| 0.65 | 73.2% | 82.2% | 86.2% | 0.0% |

### 저장 (ingest, 26건)

| 지표 | 값 |
|---|---|
| 저장 정확도 (저장 + 거절) | 88.5% [71.0–96.0] (n=26) |
|   └ 올바른 카테고리에 저장 | 88.5% [71.0–96.0] (n=26) |
|   └ 트리/루트 밖 Q&A 거절 | - |
| 계층 F1 | 0.885 [0.759–1.000] (n=26) |
| 초안이 검색에 안 보임 | 100.0% [87.1–100.0] (n=26) |
| 게시 성공 (버전 +1) | 100.0% [87.1–100.0] (n=26) |
| 초안→게시 카테고리 유지 | 100.0% [87.1–100.0] (n=26) |
| 중복 탐지 precision | - |
| 중복 탐지 recall | - |
| 갱신/신규 판정 일치 | 100.0% [87.1–100.0] (n=26) |
| 기존 게시 답변이 초안으로 내려간 건수 (관찰) | 0 |
| 게시 후 probe 검색 top-k 포함 | - |
| 게시 후 probe 검색 top1 답변 | - |
| 지연 p50 (초안+게시+probe) | 9.4s |
| LLM 실패 → Jev 빔 대체율 | 0.0% [0.0–12.9] (n=26) |

### 자주 틀린 착지 (기대 → 실제)

| 기대 | 실제 | 건수 |
|---|---|---:|
| bank_04_01 | bank_04 | 2 |
| abroad_01_04_1 | abroad_02_05 | 1 |
| abroad_02_03 | abroad_01_03 | 1 |
| abroad_03_01_3 | abroad_03_01 | 1 |
| abroad_04_01 | abroad_01_03 | 1 |
| abroad_04_04_1 | abroad_04 | 1 |
| abroad_04_04_2 | abroad_04_04 | 1 |
| abroad_05_03 | abroad_05 | 1 |
| abroad_05_04 | abroad_05 | 1 |
| abroad_07_03 | abroad | 1 |
| appliance_04_01 | appliance | 1 |
| appliance_04_03 | appliance_04 | 1 |
| appliance_05_02_1 | appliance_05_02 | 1 |
| appliance_05_03 | parent_07_03_2 | 1 |
| appliance_05_06 | appliance_05 | 1 |
| appliance_07_02 | appliance | 1 |
| appliance_07_03_1 | appliance | 1 |
| bank_01_05 | bank_04_02 | 1 |
| bank_02 | bank_02_03 | 1 |
| bank_02_04_2 | insure_06_02_1 | 1 |

### 틀린 케이스 (228건 중 60건)

| id | 질의 | 기대 | 실제 | 이유 |
|---|---|---|---|---|
| I-CARD-004 | 해외 식당에서 QR로 결제할 때 현지 통화로 자동 설정되나요? | card_07_01_2 | abroad_05_02_1 | domain |
| I-INSURE-003 | 자동차등록증 사진을 업로드하라는 안내가 나왔을 때 제출 방법은요? | insure_02_05 | car_05 | domain |
| I-SHOP-003 | 배송비 쿠폰과 상품 할인쿠폰을 같은 주문에 같이 쓸 수 있나요? | shop_03_04 | member_05_02_2 | domain |
| S-ABROAD-007 | 할인표로 산 비행기 타는 날 아침에 갑자기 못 가게 되면 낸 돈은 한 푼도 못 되돌려받나요? | abroad_01_04_1 | abroad_02_05 · top abroad_02_05#3 0.76 recommended | route branch, wrong top1 |
| S-ABROAD-021 | 아기 비행기표에 로마자 이름 오기랑 태어난 날짜를 잘못 입력했는데 바로잡으려면 뭘 내야 할까요? | abroad_02_03 | abroad_01_03 · top abroad_01_03#4 0.65 recommended | route branch, wrong top1 |
| S-ABROAD-030 | 23키로 넘었는데 1키로당 공항에서 얼마 더 내야돼? | abroad_03_01_3 | abroad_03_01 · top abroad_03_01_3#1 0.92 recommended | route shallow |
| S-ABROAD-044 | 그거 거기서 경고 뜨면 탑승 안 되는 거야? 그때는 어떡해야 돼? | abroad_04_01 | abroad_01_03 · abstained · top abroad_01_03#3 0.29 reference | route branch, abstained |
| S-ABROAD-055 | 장기체류 비자 체류목적 기재 | abroad_04_04_1 | abroad_04 · top abroad_04_04_1#11 0.89 recommended | route shallow |
| S-ABROAD-057 | 기내식 햄샌드위치 안 먹고 챙겼는데 육포랑 같이 그냥 들고 나가도 돼? | abroad_04_04_2 | abroad_04_04 · top abroad_04_04_2#3 0.84 recommended | route shallow |
| S-ABROAD-069 | 아 그럼 엔화 같은 거 미리 안 바꿔놓고 그냥 원화만 있는 채로 가서 바로 뽑을 수도 있어요? 그때 환율은 어떻게 돼요? | abroad_05_03 | abroad_05 · top abroad_05_03#12 0.89 recommended | route shallow |
| S-ABROAD-070 | 매장에서 리펀드 종이 받으려는데 여권 실물 있어야 됨? 사진 안돼? 내일 다시 와서 받아도 돼? | abroad_05_04 | abroad_05 · top abroad_05_04#3 0.90 recommended | route shallow |
| S-ABROAD-097 | Something disappeared from my hotel room safe, should I leave everything as it i | abroad_07_03 | abroad · top abroad_07_03#4 0.94 recommended | route shallow |
| S-APPLIANCE-044 | 약하게나 보통으로 돌릴 때는 멀쩡한데 그게 아니라 제일 센 강력으로 올리면 1분도 안 돼서 바로 꺼져버려요 | appliance_04_01 | appliance · top appliance_04_01#5 0.89 recommended | route shallow |
| S-APPLIANCE-048 | 이불용 흡입기계 덜덜이 헤드에 낀 섬유 찌꺼기는 뭘로 손질해요? UV살균등 달린 건데 한 달에 몇 번 해줘야 돼요? | appliance_04_03 | appliance_04 · top appliance_04_03#5 0.90 recommended | route shallow |
| S-APPLIANCE-062 | 55인치 TV인데 패널 갈아야 할 것 같아요. 무상 해당되면 부품비랑 수리비 다 무료인가요? 출장 점검 오고 나서 교체까지 며칠 걸려요? 보증  | appliance_05_02_1 | appliance_05_02 · top appliance_05_02_1#8 0.95 recommended | route shallow |
| S-APPLIANCE-065 | 초등생 애가 밤마다 야한 영상 멋대로 틀까 봐 걱정돼요 비번 눌러야 켜지도록 차단하는 기능 있어요? | appliance_05_03 | parent_07_03_2 · top parent_07_03_2#6 0.71 recommended | route domain, wrong top1 |
| S-APPLIANCE-074 | 벽면에 달아둔 거실 디스플레이의 숙임 각도와 눈높이에 맞는 상하 위치는 어떻게 조절하나요? | appliance_05_06 | appliance_05 · top appliance_05_06#7 0.94 recommended | route shallow |
| S-APPLIANCE-088 | 출근 전에 reservation 걸어서 오후 5시에 finish 되게 설정 가능해요? | appliance_07_02 | appliance · top appliance_07_02#6 0.96 recommended | route shallow |
| S-APPLIANCE-089 | 얘만 뽀송하게 하려면 뭘 켜야 돼요? | appliance_07_03_1 | appliance · top appliance_07_03_1#1 0.89 recommended | route shallow |
| S-BANK-013 | 한도계좌 걸렸는데 풀려면 뭐 해야됨? | bank_01_05 | bank_04_02 · top bank_04_02#1 0.79 recommended | route branch, wrong top1 |
| S-BANK-026 | 보험료 나가는 날이 월급 들어오기 전이라 맨날 잔부족 뜸ㅠ 뒤로 미룰 수 있음? | bank_02_04_2 | insure_06_02_1 · top insure_06_02_1#6 0.62 alternative | route domain, wrong top1 |
| S-BANK-029 | My dad passed away and he had deposits plus funds, a trust, and a safe deposit b | bank_02_05 | null · abstained · top no items | route false_none, abstained |
| S-BANK-038 | I made a cardless cash transfer at an ATM and lost the printed slip — can I have | bank_03_04 | bank_03 · top bank_03_04#8 0.94 recommended | route shallow |
| S-BANK-040 | 다음 business day 오전까지도 입금 안 됐는데 언제 customer center에 contact 해야 돼? call할 때 transac | bank_03_05 | returns_04 · top returns_04_01_1#8 0.67 recommended | route domain, wrong top1 |
| S-BANK-041 | ARS 전화 걸어서 송금하려고 하는데, 미리 등록해둔 계좌랑 처음 보내는 계좌랑 하루에 각각 얼마까지 보낼 수 있어요? | bank_04_01 | bank_04 · top bank_04_01#5 0.94 recommended | route shallow |
| S-BANK-042 | 회사 이름으로 PC 뱅킹 이용해 자금 옮기면 하루에 보낼 수 있는 상한이 보통 고객 경우와 차이가 있나요? | bank_04_01 | bank_04 · top bank_04_01#6 0.92 recommended | route shallow |
| S-BANK-057 | The expected refund date for my mistaken transfer has passed but the money still | bank_05_02 | bank_03 · top bank_03_05#9 0.76 recommended | route branch, wrong top1 |
| S-BANK-062 | 외국으로 돈 보내는 거 신청해놨는데 아직 발송되기 전이면 철회할 수 있어요? | bank_05_05 | bank_06 · top bank_06_04#7 0.86 recommended | route branch, wrong top1 |
| S-BANK-070 | 토요일이나 빨간날에 외국으로 돈 보내면 바꿔주는 값은 어떻게 매겨져요? | bank_06_03 | bank_06 · top bank_06_03#4 0.93 recommended | route shallow |
| S-BANK-079 | 은행 안 가고 비대면으로 처음 발급돼? | bank_07_02_1 | card_01 · top card_01_02#4 0.75 recommended | route domain, wrong top1 |
| S-BANK-082 | 인증서 비번 바꿨는데 다시 등록해야됨? | bank_07_02_2 | bank_07_02 · top bank_07_02_2#37 0.95 recommended | route shallow |
| S-BANK-102 | 안 쓰는 계좌들 전체적으로 정리하고 싶어요 | bank_02 | bank_02_03 · top bank_02_03#2 0.82 recommended | route deep, wrong top1 |
| S-CAR-084 | 밧데리 0프로까지 방전됐는데 괜찬아요 경고등떴음 어케해야돼 ㅠㅠ | car_07_03 | car_02_04 · top car_02_04#article 0.72 recommended | route branch, wrong top1 |
| S-CARD-010 | 수시입출금 통장이 하나도 없는 상태인데 바로 출금되는 플라스틱 신청하면서 입출금 상품을 함께 새로 트는 게 가능할까요? | card_01_02 · no answer | card_01 · abstained · top card_01_02#3 0.34 reference | route shallow |
| S-CARD-018 | 자동 음성 연결해서 앞으로 쓸 수 있는 돈이 얼마 남았는지 알아보려면 뭘 눌러서 본인임을 밝혀야 돼요? 상담 직원이랑 이야기하면 더 물어보는 것 | card_02_01 | card_05_01_2 · abstained · top card_05_01_2#1 0.24 reference | route branch, abstained |
| S-CARD-030 | 토요일에 해외직구로 결제했는데 청구액이 생각보다 달랐어요. 주말엔 환율 고시가 안 된다던데 그럼 어떻게 계산되는 건가요? | card_02_05_2 | abroad_05_02 · abstained · top abroad_05_02_1#20 0.34 reference | route domain, abstained |
| S-CARD-034 | 실물카드는 멀쩡히 있는데 폰 속 모바일카드만 털린듯ㅠ 이거만 따로 정지됨? | card_03_01 | card_03 · top card_03_01#15 0.94 recommended | route shallow |
| S-CARD-057 | 간편비밀번호 잊음 재설정 | card_05_01_1 | security_01 · top security_01#article 0.73 recommended | route domain, wrong top1 |
| S-CARD-058 | 미국 직구 사이트에서 150만원짜리 카메라 결제하려는데 국내 한도는 남았는데 계속 거절돼요. 해외이용한도가 따로 있는 건가요, 이럴 땐 어떻게  | card_05_01_2 | card_05_04 · top card_05_04#3 0.84 recommended | route branch, wrong top1 |
| S-CARD-064 | 노래 부르러 간 곳에서 더 놀려고 셀프 계산 기계에 또 태그했는데 들어갈 때 낸 돈이랑 겹쳐서 두 번 빠져나간 건지 보려고요 | card_05_03 | card · top card_05_03#37 0.87 recommended | route shallow |
| S-CARD-066 | 어제 방콕 호텔 체크인하는데 1박 15만원짜리 2박 잡았는데 보증금 35만원 사전승인이 거절됐어요, 한도는 100만원 넘게 남는데 왜 안 되는  | card_05_04 | abroad_05_02_2 · abstained · top abroad_05_02_2#1 0.42 reference | route domain, abstained |
| S-CARD-077 | I want to keep using my own credit card but cancel only the supplementary card I | card_06_05 | card_01_03 · abstained · top card_01_03#3 0.24 reference | route branch, abstained |
| S-CAREER-017 | 밖에 있는데 마감이 얼마 안 남아서 폰으로 원서를 내야 해요. 번호랑 메일주소를 잘못 적을까봐 걱정인데 미리 체크할 요령 있을까요? | career_01_05 · no answer | career_01_05 · top career_01_05#8 0.51 alternative | answered unanswerable |
| S-CAREER-052 | 처음 반 년은 뭐가 제일 버거우셨어요? | career_04_04_2 | career · top career_04_04_2#14 0.69 recommended | route shallow |
| S-CAREER-059 | 작년에 회사 주식 싸게 사서 판 차액이 컸는데, 새 곳에 얘기하는 받는 돈 셈할 때 그 일회성 돈까지 합해서 말해야 할까요? | career_05_01_1 | career · top career_05_02#12 0.60 alternative | route shallow, wrong top1 |
| S-CAREER-060 | 옮겨갈 회사에서 예전에 다니던 곳 세금 종이 꼭 달라고 하는데, 이거 안 넘겨주면 법에 걸리거나 불이익 있나요? | career_05_01_1 | tax_02 · top tax_02_06#3 0.73 recommended | route domain, wrong top1 |
| S-CAREER-071 | 수습 탈락해서 2개월만에 짤리게 됐는데 퇴직금이랑 실업급여 받을 수 있음? | career_05_04 | pension_04_01 · top pension_04_01#2 0.67 recommended | route domain, wrong top1 |
| S-CAREER-085 | 쉰 넘었고 회사 다닌 햇수가 열 해를 초과했는데 실직 지원금 나오는 총 날 수가 얼마예요? | career_07_01_1 | career_07 · top career_07_01_1#1 0.95 recommended | route shallow |
| S-CAREER-093 | 한도 거의 다써서 소진울 90% 넘었는데 잔액화면에서 추가지원가능여부 어디서 봐요? 저소득이면 200만원 더 나온다던데 ㅇㅈ되면 심사는 어케 받 | career_07_03_2 | loan_02_02 · top loan_02_02#5 0.41 alternative | route domain, wrong top1 |
| S-CAREER-105 | 필기시험 준비는 어떻게 해요? | career_03_03 | car_04_02 · abstained · top car_04_02#3 0.35 reference | route domain, abstained |
| S-CAREER-114 | 내일배움카드가 뭔가요? | career_07_03 | career_07_03 · abstained · top career_07_03#article 0.40 reference | abstained |
| S-CIVIL-015 | 개명 등본 이전이름 | civil_01_05 | civil_01_01 · abstained · top civil_01_01#article 0.34 reference | route branch, abstained |
| S-CIVIL-035 | 아파서 외국 병원 진료가야됨 1회용으로 끊으려면 뭐 내야돼? 보호자도 같이 가는데 | civil_03_01_2 | abroad_07 · top abroad_07_04#6 0.60 alternative | route domain, wrong top1 |
| S-CIVIL-057 | 이대로도 뽑을 수 있어요? | civil_04_04 | civil_01_01_3 · top civil_01_01_3#4 0.82 recommended | route branch, wrong top1 |
| S-CIVIL-084 | 갤럭시 S23 쓰고 있는데 어제 저녁에 인증 앱 승인 비번을 5번 넘게 틀렸더니 잠겼다고 떠요. 이거 언제 풀리고 비번 까먹었으면 어디서 다시  | civil_07_01_1 | security_02_02 · top security_02_02_1#1 0.49 alternative | route domain, wrong top1 |
| S-CIVIL-097 | 엄마 거동 불편해서 못 가는데 내가 대신 뽑아줄 수 있음? | civil_07_05 | civil_01_04 · top civil_01_04#10 0.89 recommended | route branch, wrong top1 |
| S-CIVIL-102 | 인터넷으로 전입신고 하는 방법 알려줘요 | civil_02_02 | civil_02_02 · top civil_02_02_1#1 0.85 recommended | wrong top1 |
| S-COOK-033 | 양념불고기 내일 구울건데 하루 재워둘겸 냉장 어떻게 함? 그냥 뚜껑 덮어두면 돼? | cook_03_03 · no answer | cook_03_03 · top cook_03_03#1 0.51 alternative | answered unanswerable |
| S-COOK-040 | 국거리 소고기 600g 샀음 한 번 끓일만큼씩 나눠 얼리려는데 몇g씩 랩 싸면 됨? | cook_04_01 | cook_03 · abstained · top cook_03_03#2 0.28 reference | route branch, abstained |
| S-COOK-052 | 주말에 비빔국수 하려고 소면 샀어요. 지난번엔 그냥 삶았더니 퍼지고 미끈거려서 망했거든요. 몇 분 삶아야 하고 찬물에 헹궈야 쫄깃해질까요? | cook_05_01 | cook · top cook_05_01#7 0.96 recommended | route shallow |

