# 골든 데이터셋 통계

범용 생활·업무 지식베이스 — `kb` v1.0.0 · kb hash `489e5fca3c7c`

`cargo run --release --example eval -- stats --out eval/golden/STATS.md`로 다시 만든다.

## 지식베이스 변형

| variant | 설명 | nodes | leaves | max depth | depth histogram | items (qa/article) | leaves by item count (0/1/2-5/6+) | largest pool |
|---|---|---:|---:|---:|---|---|---|---|
| deep | 원본 그대로. 최대 깊이 7 | 219 | 147 | 7 | d1:8 d2:37 d3:106 d4:51 d5:9 d6:4 d7:4 | 669 (597/72) | 3/0/141/3 | it (234) |
| flat | 리프만 남긴 1단 구조(147개 최상위 분류) | 147 | 147 | 1 | d1:147 | 597 (597/0) | 3/0/141/3 | it_office_excel_func (83) |
| shallow | 중간 단계를 모두 걷어낸 2단 구조(영역 → 리프) | 155 | 147 | 2 | d1:8 d2:147 | 605 (597/8) | 3/0/141/3 | it (220) |
| sparse | standard 트리에서 리프마다 첫 qa 1개만 남김. 정답이 빠진 질문은 보류가 정답 | 198 | 147 | 4 | d1:8 d2:37 d3:122 d4:31 | 195 (144/51) | 3/144/0/0 | it (44) |
| standard | fine 단계를 걷어낸 표준형. 최대 깊이 4 | 198 | 147 | 4 | d1:8 d2:37 d3:122 d4:31 | 648 (597/51) | 3/0/141/3 | it (230) |

## 영역별 지식베이스 규모 (deep 기준)

| domain | nodes | leaves | items |
|---|---:|---:|---:|
| it | 48 | 33 | 234 |
| life | 33 | 20 | 63 |
| money | 28 | 20 | 64 |
| health | 22 | 15 | 45 |
| travel | 24 | 16 | 49 |
| food | 18 | 11 | 114 |
| work | 24 | 17 | 54 |
| shop | 22 | 15 | 46 |

## 케이스 구성

총 513건

| type | cases | share |
|---|---:|---:|
| contract | 12 | 2% |
| ingest | 68 | 13% |
| search | 433 | 84% |

| split | cases | share |
|---|---:|---:|
| dev | 352 | 69% |
| test | 161 | 31% |

| domain | cases | share |
|---|---:|---:|
| api | 12 | 2% |
| food | 52 | 10% |
| health | 44 | 9% |
| it | 111 | 22% |
| life | 47 | 9% |
| money | 54 | 11% |
| oos | 46 | 9% |
| shop | 47 | 9% |
| travel | 49 | 10% |
| work | 51 | 10% |

| turns | cases | share |
|---|---:|---:|
| multi | 115 | 22% |
| single | 398 | 78% |

| entry | cases | share |
|---|---:|---:|
| forest | 435 | 85% |
| root | 61 | 12% |
| start | 17 | 3% |

| style | cases | share |
|---|---:|---:|
| - | 12 | 2% |
| canonical | 16 | 3% |
| colloquial | 81 | 16% |
| condition | 34 | 7% |
| english | 14 | 3% |
| injection | 4 | 1% |
| keyword | 20 | 4% |
| long | 12 | 2% |
| mixed | 13 | 3% |
| multi_intent | 9 | 2% |
| negation | 20 | 4% |
| oos_far | 10 | 2% |
| oos_near | 13 | 3% |
| paraphrase | 218 | 42% |
| typo | 13 | 3% |
| vague | 24 | 5% |

| ctx | cases | share |
|---|---:|---:|
| - | 399 | 78% |
| coref | 21 | 4% |
| correct | 8 | 2% |
| disambig | 21 | 4% |
| distract | 9 | 2% |
| ellipsis | 13 | 3% |
| long | 8 | 2% |
| refine | 10 | 2% |
| shift | 14 | 3% |
| slot | 10 | 2% |

| diff | cases | share |
|---|---:|---:|
| - | 12 | 2% |
| easy | 144 | 28% |
| hard | 123 | 24% |
| medium | 234 | 46% |

| expect | cases | share |
|---|---:|---:|
| empty_leaf | 3 | 1% |
| error | 12 | 2% |
| file | 56 | 11% |
| internal | 24 | 5% |
| leaf | 345 | 67% |
| no_answer | 27 | 5% |
| oos | 34 | 7% |
| reject | 12 | 2% |

## 변형별 적용 케이스

| variant | search | ingest | contract | skipped |
|---|---:|---:|---:|---:|
| deep | 433 | 68 | 12 | 0 |
| flat | 357 | 50 | 11 | 95 |
| shallow | 388 | 59 | 12 | 54 |
| sparse | 432 | 56 | 11 | 14 |
| standard | 432 | 68 | 12 | 1 |

## 검색 케이스의 목표 깊이·후보 수 (standard)

| target depth | cases | share |
|---|---:|---:|
| d0 | 34 | 8% |
| d1 | 10 | 2% |
| d2 | 30 | 7% |
| d3 | 271 | 63% |
| d4 | 87 | 20% |

| pool size | cases | share |
|---|---:|---:|
| 0 | 37 | 9% |
| 101+ | 4 | 1% |
| 2-5 | 311 | 72% |
| 33-100 | 61 | 14% |
| 6-32 | 19 | 4% |

## 리프 커버리지

147 / 147 리프가 한 번 이상 기대 노드로 쓰였다.

