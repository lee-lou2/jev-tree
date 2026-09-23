# 골든 데이터셋 명세

jev-tree의 검색(`search`)과 저장(`ingest`)을 평가하는 골든 데이터셋의 형식, 라벨 규칙, 작성 규칙이다.
평가 방식은 [METHOD.md](METHOD.md), 실행 계획은 [PLAN.md](PLAN.md)를 본다.

형식은 LLM·검색 평가에서 가장 널리 쓰는 구성을 따른다.

- **코퍼스 + 질의 + 정답 판정(qrels)**: BEIR·MTEB 방식. 지식베이스(코퍼스)와 케이스(질의)를 분리하고,
  정답 문서는 등급(graded relevance)으로 매긴다.
- **케이스 = 입력 + 기대 출력 + 메타데이터**: DeepEval·LangSmith·Ragas의 "golden" 예제 형식.
  한 줄에 케이스 하나인 JSONL.
- **계층 분류 라벨**: 기대 카테고리는 트리의 노드 하나(+허용 대안)로 적고, 채점은 정확 일치와 계층 지표를 함께 쓴다.

## 1. 폴더 구조

```
eval/
├── fixtures/kb/                 지식베이스(코퍼스)
│   ├── manifest.json            이름·버전·영역 코드·변형 정의·빈 리프·대용량 리프
│   ├── nodes.json               분류 트리 원본(= deep 변형)
│   └── items/*.jsonl            아이템(Q&A). 영역별 파일
├── golden/*.jsonl               골든 케이스. 영역별(it·life·money·health·travel·food·work·shop),
│                                dense(대용량 리프), cross(영역 교차·범위 밖·주입), contract(API)
└── runner/                      검증·실행·채점 도구 (cargo run --example eval)
```

`fixtures/kb`는 서버 시드 형식과 호환된다. 평가 전용 필드(`tier`, `ref`, `manifest.json`)는 서버가 무시한다.
변형별 단일 시드 파일은 `cargo run --release --example eval -- export --variant standard --out /tmp/seed.json`으로 만든다.

## 2. 지식베이스

### 2.1 범위

8개 영역, 누구나 이해할 수 있는 범용 주제로 구성한다.

| 코드 | 루트 id | 영역 | 성격 |
|---|---|---|---|
| IT | `it` | IT·디지털 | 컴퓨터·스마트폰·인터넷·오피스·계정 보안 |
| LIFE | `life` | 생활·가정 | 가전 고장, 청소·세탁, 집 관리 |
| MONEY | `money` | 금융·경제생활 | 은행·카드·대출·저축·세금·보험 |
| HEALTH | `health` | 건강 | 흔한 증상, 응급처치, 운동, 수면, 영양 |
| TRAVEL | `travel` | 여행·교통 | 항공·숙소·해외 준비·대중교통·자동차 |
| FOOD | `food` | 요리·식생활 | 조리, 식재료 보관, 식품 안전, 주방 도구 |
| WORK | `work` | 직장생활·인사 | 가상의 회사 인사 규정과 사내 절차 |
| SHOP | `shop` | 쇼핑·주문 | 가상의 온라인 쇼핑몰 주문·배송·반품·혜택 |

WORK와 SHOP의 답변은 가상의 회사·쇼핑몰 규정이다. 나머지는 일반 상식 수준의 안내다.
답변의 사실성은 평가 대상이 아니다. 평가 대상은 **올바른 노드로 가는지, 올바른 아이템이 위에 오는지**다.

영역 경계에는 일부러 헷갈리는 쌍을 둔다. 예: 회사 법인카드(WORK) ↔ 개인 카드(MONEY),
쇼핑몰 카드 환불(SHOP) ↔ 카드사 업무(MONEY), 배탈 증상(HEALTH) ↔ 식중독 예방(FOOD),
와이파이 비밀번호(IT 공유기) ↔ 계정 비밀번호(IT 보안), 이체 한도(MONEY 은행) ↔ 카드 한도(MONEY 카드).
각 노드의 `description`이 그 경계를 문장으로 말한다.

### 2.2 노드 (`nodes.json`)

```json
{"id": "it_net_wifi_drop", "parent_id": "it_net_wifi", "name": "와이파이 끊김",
 "description": "연결은 되지만 몇 분마다 끊기거나 ... 연결은 유지되는데 느리면 와이파이 속도 저하.",
 "examples": ["와이파이가 수시로 끊겨요", "잠깐 자리 비우면 연결이 끊어져요"]}
```

| 필드 | 규칙 |
|---|---|
| `id` | snake_case, 부모 id를 접두어로 확장 |
| `name` | 전체 트리에서 유일 (flat 변형에서 형제가 되므로) |
| `description` | 범위 + 헷갈리는 이웃과의 차이. 180자 이하(LLM 라우터가 자름) |
| `examples` | 실제 사용자 표현 1–3개. Jev 선택지 라벨에 들어간다. **골든 질의로 재사용 금지** |
| `tier` | 중간 노드에만. `core` = standard에 남음, `fine` = deep에만 있음 |

### 2.3 아이템 (`items/<domain>.jsonl`)

```json
{"ref": "it_net_wifi_drop#1", "category_id": "it_net_wifi_drop", "kind": "qa",
 "question": "노트북 와이파이가 절전 모드에서 깨어나면 끊겨요. 어떻게 하나요?",
 "answer": "장치 관리자 > 네트워크 어댑터 > 전원 관리에서 '전원을 절약하기 위해 이 장치를 끌 수 있음'을 해제하세요."}
```

| 필드 | 규칙 |
|---|---|
| `ref` | `<category_id>#<접미사>`. qa는 `#1`, `#2`…, 중간 노드 안내문은 `#article`, 대용량 리프는 `#<슬러그>` |
| `kind` | `qa`(그대로 답으로 보낼 수 있음) 또는 `article`(중간 노드의 개요 안내) |
| `question` | 240자 이하. **전체 KB에서 유일**(서버 upsert와 ref↔id 매핑이 질문 문자열을 쓴다) |
| `answer` | 1–3문장, 구체적 절차·조건·기한. 40–220자 권장 |

배치 규칙:

- 리프마다 `qa` 2–4개(평균 2.5). 약 15%는 1개, 약 15%는 4개로 분포를 둔다.
- 중간 노드와 루트마다 `article` 정확히 1개. 막연한 질문이 그 노드에 멈췄을 때 돌려줄 개요다.
  질문은 "○○ 문제는 어떻게 구분해서 해결하나요?"처럼, 답은 하위 주제로 안내하는 2–4문장.
- `manifest.empty_leaves`(해충 퇴치, 피부 가려움, 택시 이용)는 아이템 0개로 둔다. 빈 카테고리 동작을 본다.
- `manifest.dense_leaves`(윈도우 단축키, 엑셀 함수, 재료별 보관법)는 50–90개. 후보가 많은 랭킹을 본다.
- 같은 리프 안의 아이템은 서로 다른 하위 질문이다. 형제 리프를 가르는 조건(아이폰/안드로이드, 드럼/통돌이,
  카드/계좌)은 질문과 답에 드러나게 쓴다.

### 2.4 구조 변형 (`manifest.variants`)

아이템은 그대로 두고 트리 모양만 바꿔 깊이의 영향을 따로 떼어 본다.
중간 노드를 빼면 그 자식은 가장 가까운 남은 조상에 붙고, 빠진 노드의 `article`도 빠진다.
`sparse`는 트리는 standard 그대로 두고 아이템 수만 줄여 "아이템이 적은 경우"를 같은 질문으로 비교한다.

| 변형 | 남기는 노드 | 리프당 아이템 | 최대 깊이 | 노드 수 | 아이템 | 용도 |
|---|---|---|---:|---:|---:|---|
| `deep` | 전부 | 원본(0–83) | 7 | 219 | 669 | 깊은 트리 (세탁기 갈래가 7단) |
| `standard` | 루트 + `core` + 리프 | 원본 | 4 | 198 | 648 | 기본 평가 대상 |
| `shallow` | 루트 + 리프 | 원본 | 2 | 155 | 605 | 영역 → 리프, 넓고 얕은 트리 |
| `flat` | 리프만 | 원본 | 1 | 147 | 597 | 147개 최상위 분류 |
| `sparse` | standard와 같음 | 첫 qa 1개 | 4 | 198 | 195 | 아이템이 적은 경우 |

케이스가 가리키는 노드(`expect.node`, `root_node`, `start_node`)가 어떤 변형에 없으면 그 변형에서는 건너뛴다.
`also_ok` 중 없는 노드는 조용히 빠진다. 정답 아이템이 모두 빠진 answerable 케이스는 건너뛴다.
단, `sparse`에서는 리프가 그대로 있고 답만 빠진 것이므로 **보류가 정답인 케이스(`dropped`)로 바뀐다**.
남은 아이템 하나가 다른 하위 질문이라 이 질문의 답이 아니기 때문이다.

## 3. 케이스

한 줄에 하나. `type`은 `search`, `ingest`, `contract` 중 하나다.

### 3.1 공통 필드

| 필드 | 필수 | 설명 |
|---|---|---|
| `id` | O | `S-<영역>-NNN`(search) · `I-<영역>-NNN`(ingest) · `C-API-NNN`(contract). 영역은 manifest 코드, 교차 영역은 `XD`, 범위 밖은 `OOS` |
| `type` | O | `search` · `ingest` · `contract` |
| `input` | O | 요청. 아래 표 |
| `expect` | O | 기대 결과. 아래 표 |
| `tags` | O | `style`, `diff` 필수. 문맥이 있으면 `ctx` 필수 |
| `note` | 권장 | 라벨 근거 한 줄. 검수자가 읽는다 |
| `split` | - | `dev`/`test`. 비우면 id 해시로 약 30%가 `test` |
| `group` | - | 짝 비교용 묶음 id (같은 질의의 forest/root 버전 등) |
| `variants` | - | 이 변형들에서만 실행 |

### 3.2 `input`

| 필드 | 대상 | 설명 |
|---|---|---|
| `query` | search | 현재 요청 |
| `question`, `answer` | ingest | 저장할 Q&A |
| `context` | 둘 다 | 이전 턴. `[{"role":"user","text":"..."},{"role":"assistant","text":"..."}]`, 최대 32 |
| `root_node` | 둘 다 | 가상 루트 노드 id. 러너가 변형별 경로(`it/it_net`)로 바꿔 `root`에 넣는다 |
| `start_node` | 둘 다 | 강제 시작 노드 id |
| `limit`, `beam_width` | search | 기본 8, 3 |
| `raw` | contract | 만들어진 요청 위에 덮어쓸 원시 필드 |

### 3.3 `expect` — search

| 필드 | 설명 |
|---|---|
| `node` | 가장 알맞은 착지 노드. 리프 또는 중간 노드(막연한 질문). `null` = 트리 밖(또는 `root_node` 밖) |
| `also_ok` | 똑같이 맞다고 볼 다른 노드. 진짜 모호할 때만(다중 의도, 영역 경계) |
| `items` | `{ref: 등급}`. 2 = 이 질문에 대한 답, 1 = 부분적으로 도움. 등급 2가 하나 이상이어야 answerable |
| `answerable` | 기본은 `items`가 있으면 true. 맞는 노드는 있지만 KB에 답이 없으면 `false`로 적고 `items`를 비운다 |

### 3.4 `expect` — ingest

| 필드 | 설명 |
|---|---|
| `node` | 저장되어야 할 카테고리. `null` = 거절(400, 저장 안 됨)이 정답 |
| `also_ok` | 허용 대안 카테고리 |
| `duplicates` | `node`에 착지했을 때 `duplicate_ids`에 나와야 할 ref. 엔진의 트리그램 규칙(질문 3-gram 겹침 ≥ 0.6)으로 **검증기가 계산해 대조**한다 |
| `updates` | 같은 질문 문자열이 이미 있어 그 행을 갱신해야 할 때 그 ref. 이때 `duplicates`에도 같은 ref가 들어간다 |

`probe`(선택): `{"query": "...", "context": [...]}`. 게시 후 이 질의로 검색해 새 아이템이 top-k에 나오는지 본다.
원문 질문을 베끼지 말고 바꿔 말한다.

### 3.5 `expect` — contract

`error`: 기대 오류 코드(`invalid_request`, `not_found`). 모델 호출 전에 끝나는 결정적 검사만 넣는다.

### 3.6 태그

| 태그 | 값 | 뜻 |
|---|---|---|
| `style` | `canonical` | 아이템 질문과 거의 같은 표현 (쉬운 기준선, 10% 이하) |
| | `paraphrase` | 같은 뜻, 다른 표현 |
| | `colloquial` | 구어체·줄임말·감탄사 ("폰 배터리 왜케 빨리 닳음ㅠ") |
| | `keyword` | 1–3 단어 ("공유기 초기화") |
| | `long` | 사연이 긴 여러 문장, 핵심은 중간이나 끝 |
| | `typo` | 자연스러운 오타·띄어쓰기 오류 |
| | `english` | 영어 질의 |
| | `mixed` | 한영 혼용 |
| | `negation` | 부정·배제로 방향을 트는 질의 ("환불 말고 교환") |
| | `multi_intent` | 한 질의에 두 요청. `also_ok`로 두 노드를 모두 허용 |
| | `vague` | 하위 사례를 특정하지 않음 → 중간 노드에 멈추는 것이 정답 |
| | `condition` | 형제 리프를 가르는 조건이 핵심 (아이폰/안드로이드, 카드/계좌) |
| | `injection` | 지시 무시·특정 id 선택 같은 프롬프트 주입이 섞인 질의 |
| | `oos_near` | 영역과 가깝지만 KB가 다루지 않음 (주식 종목 추천, 파이썬 설치) |
| | `oos_far` | 전혀 무관 (날씨, 스포츠 결과) |
| `ctx` | `coref` | 지시어로 앞 턴을 가리킴 ("그거 어디서 확인해요?") |
| | `ellipsis` | 주어·목적어 생략 ("그럼 안드로이드는요?") |
| | `refine` | 앞 턴의 막연한 요청을 이번 턴이 구체화 |
| | `slot` | 조건이 여러 턴에 흩어져 있음 (결제 수단은 앞 턴, 질문은 이번 턴) |
| | `shift` | 앞 턴과 다른 주제로 전환. 이번 턴을 따라야 함 |
| | `distract` | 앞 턴에 무관한 잡담·다른 기기 이야기 |
| | `correct` | 앞에서 말한 조건을 이번 턴이 정정 ("아, 갤럭시가 아니라 아이폰이에요") |
| | `disambig` | 이번 턴만으로는 모호하고 앞 턴이 갈래를 정함 |
| | `long` | 9턴 이상의 긴 대화 |
| `diff` | `easy` · `medium` · `hard` | 작성자 판단 난이도 |

러너가 자동으로 붙이는 파생 태그: `domain`(기대 노드의 최상위 영역, 없으면 `oos`), `turns`(single/multi),
`entry`(forest/root/start), `level`(leaf/internal/none), `depth`(변형 기준 목표 깊이), `pool`(착지 시 랭킹 후보 수 구간),
`expect`(leaf · internal · no_answer · empty_leaf · dropped · oos · file · reject · error).

## 4. 라벨 규칙

1. **node는 "가장 구체적으로 맞는 노드"다.** 질문이 리프의 조건을 말하면 리프, 주제만 말하면 그 주제 노드.
   예: "노트북이 이상해요" → `it_pc`, "와이파이 문제 전반" → `it_net_wifi`.
2. **also_ok는 아껴 쓴다.** 두 노드 중 어느 쪽 답도 사용자를 만족시킬 때만. 애매함을 덮으려고 쓰지 않는다.
3. **등급 2는 "이 아이템을 그대로 답으로 보내도 된다"**, 등급 1은 "관련 있지만 부족"이다.
   중간 노드 케이스의 등급 2는 보통 그 노드의 `article`이다.
4. **정답 아이템은 기대 노드(또는 also_ok)의 서브트리 안에 있어야 한다.** 랭킹은 착지한 서브트리만 보기 때문이다.
5. **answerable=false**: 맞는 노드는 있지만 KB에 답이 없는 질문(빈 리프 포함). 시스템은 `abstained: true`로 답을 보류해야 한다.
6. **node=null**: 트리 밖 질문, 또는 `root_node` 밖 질문. 시스템은 `leaf_id: null`을 내야 한다.
7. **`root_node` 케이스**: node는 root_node 서브트리 안이거나 null이다. root_node가 리프면 descent 없이 그 리프에 머무르므로
   node는 그 리프이고, 무관한 질문이면 answerable=false로 적는다.
8. **`start_node` 케이스**: 엔진은 start_node 아래로만 내려가고 `__none__` 대신 `__stop__`만 고를 수 있다.
   서브트리 밖 질문의 정답은 `node = start_node`, `answerable = false`다 (null이 아니다).
9. **ingest 거절**: 트리 밖 Q&A, `root_node` 밖 Q&A는 node=null. 엔진은 400을 내고 아무것도 저장하지 않아야 한다.
10. **문맥 케이스**는 현재 요청만 보면 틀리거나 모호해야 의미가 있다. 문맥 없이도 풀리면 `ctx` 케이스가 아니다.

## 5. 작성 규칙

- 한국어 사용자 발화처럼 쓴다. 존댓말·반말·구어체를 섞는다. 개인정보(실명·전화번호·계좌번호)는 넣지 않는다.
- `canonical`이 아닌 질의는 아이템 질문이나 노드 `examples`를 그대로 쓰지 않는다 (검증기가 경고).
- 해마다 바뀌는 수치(세율, 법정 한도)는 피하거나 "회사 규정"처럼 가상의 규정으로 쓴다.
- 브랜드는 누구나 아는 제품명(아이폰, 갤럭시, 윈도우, 엑셀)만 자연스럽게 쓴다.
- 문맥의 assistant 턴은 그럴듯한 상담 답변으로 쓰되 정답 아이템을 통째로 옮기지 않는다.
- 케이스 하나는 한 가지를 시험한다. 다중 의도, 문맥 전환 같은 복합 케이스는 태그로 드러낸다.

## 6. 영역별 목표 구성

한 영역(리프 L개)에서 search 케이스는 대략 다음과 같이 채운다.

| 묶음 | 개수 | 비고 |
|---|---|---|
| 단일 턴, 리프 정답 | ≈ 0.9 L | 모든 비어 있지 않은 리프를 한 번 이상. style 분포: paraphrase 25%, colloquial 15%, condition 10%, keyword 10%, long 8%, typo 7%, english·mixed 8%, canonical ≤ 10%, negation 5%, multi_intent 2% |
| 막연한 질문(중간 노드) | 2–3 | 영역 루트 1, core 중간 노드 1 이상 |
| 답 없는 영역 내 질문 | 2 | answerable=false |
| 빈 리프 | 빈 리프당 1 | answerable=false |
| 여러 턴 문맥 | 8–10 | coref 2, ellipsis·refine·slot·shift·correct·distract·disambig·long 각 1 |
| 가상 루트 | 4 | 영역 루트 안 질문, core 중간 노드 안 질문, 리프 루트, 루트 밖 질문(null) |
| start_node | 1–2 | 서브트리 안 질문, 서브트리 밖 질문(node=start, answerable=false) |

ingest 시나리오는 영역마다 6–8개: 새 Q&A 3(문체를 달리하고 2개에 probe), 여러 턴 문맥 1, 거의 같은 질문 1(duplicates),
완전히 같은 질문 1(updates), 가상 루트 안 1, 가상 루트 밖 1(null).

교차 영역 파일(`golden/cross.jsonl`)은 영역 경계 혼동, 문맥에 따라 정답이 갈리는 짝, 영역을 넘는 주제 전환,
범위 밖 질문, 프롬프트 주입을 담는다. `golden/contract.jsonl`은 모델 호출 없이 끝나는 API 계약 검사다.

## 7. 검증

```bash
cargo run --release --example eval -- validate            # 오류가 있으면 실패
cargo run --release --example eval -- validate --strict   # 경고도 실패로
cargo run --release --example eval -- stats --out eval/golden/STATS.md
```

검증기는 id·ref 유일성, 부모·노드 존재, 변형별 트리 유효성(형제 이름 중복 등), 질문 중복,
정답 아이템이 기대 노드 서브트리 안에 있는지, root_node/start_node 규칙, 태그 어휘,
문맥 형식, ingest `duplicates`/`updates`가 엔진 규칙과 일치하는지 확인한다.

## 8. 버전

`manifest.json`의 `version`을 올리는 기준:

- **major**: 노드 추가·삭제·이동, 변형 정의 변경 → 이전 결과와 직접 비교 불가
- **minor**: 케이스·아이템 추가 → 공통 케이스끼리만 비교
- **patch**: 오탈자, note 수정 → 그대로 비교

러너는 결과에 KB 해시와 케이스 해시를 함께 적는다. 해시가 다른 실행끼리는 공통 케이스 id만 짝지어 비교한다.
