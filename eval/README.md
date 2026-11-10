# eval — jev-tree 평가

> **In English** — Korean golden dataset and runner for measuring how well `jev-tree` files and
> finds knowledge in its taxonomy. Content is Korean because the golden set tests Korean user
> utterances; the runner code is English like the rest of the crate. Read [`PLAN.md`](PLAN.md)
> for the hypotheses and matrix, [`results/README.md`](results/README.md) for the measured
> baseline and improvement proposals.

검색(`search`)과 저장(`ingest`) 품질을 재는 골든 데이터셋, 실행기, 평가 방식과 계획.

## 한눈에

| | |
|---|---|
| **지식베이스** | 범용 8개 영역, 카테고리 219개(리프 147), 아이템 669개 |
| **트리 변형 5종** | `deep`(7단) · `standard`(4) · `shallow`(2) · `flat`(1) · `sparse`(리프당 1개) |
| **케이스 525건** | 검색 445 · 저장 68 · API 계약 12. 여러 턴 문맥 9종, 루트/시작 노드 진입, 막연한·답 없는 질문, 범위 밖, 프롬프트 주입, 영역 경계 혼동 |
| **라우터 2종** | `jev`(Jev 한 번 선택, 큰 트리는 거친 지도 + BM25) · `jev_llm`(같은 경로 + Jev가 불확실한 선택만 LLM이 리뷰) |
| **split** | 약 30%가 `test`(id 해시) |
| **최신 기준선** | `jev`/standard **E2E 97.3% · Hit@1 97.3%** (v0.8.0 보류 캘리브레이션) · `jev_llm`(muse-spark) **97.7% / 97.6%** |

트리 변형은 **같은 아이템을 깊이만 바꿔** 담아서 구조의 영향을 분리해 봅니다. 빈 리프 3개와
아이템 55–83개가 몰린 대용량 리프 3개를 일부러 넣었습니다.

## 폴더

```
fixtures/kb/   지식베이스 — manifest.json · nodes.json · items/*.jsonl
golden/        골든 케이스 — 영역별 · dense(대용량 리프) · cross(교차·범위 밖·주입) · router(라우터 회귀) · contract(API)
runner/        실행기 소스 (cargo example `eval`)
results/       실행 결과와 기록표
tools/         secret-scan.sh — 민감 정보 검사
```

## 빠른 시작

저장소 루트에서 실행합니다.

```bash
# 1) 데이터 점검 (키 불필요) — 오류·경고 0이 정상
cargo run --release --example eval -- validate --strict

# 2) 하네스 점검 (키 불필요) — 스크립트 평가기, 라우팅·저장 100%가 정상
cargo run --release --example eval -- run --mock oracle --router both --variant all

# 3) 실제 평가
cargo run --release --example eval -- run --router both --variant all --note "baseline"

# 4) 두 설정 비교 (짝지어 McNemar 검정, MRR 부트스트랩)
cargo run --release --example eval -- compare eval/results/runs/<A> eval/results/runs/<B> \
                                            --a jev/standard --b jev_llm/standard
```

`run --dry-run`은 셀별 케이스 수만 보여 줍니다. 전체 옵션은 `… -- help`.

## 키 설정

러너는 **DB에 등록된 설정을 먼저** 읽습니다.

```bash
cargo run --release --example eval -- … --settings-db /path/to/data/db.sqlite3
```

DB에 값이 있으면 그 값을 쓰고, 없을 때만 환경 변수를 씁니다(앞 것이 우선). Jev 모델과 주소는 환경 변수로만 정합니다.

| 용도 | 환경 변수 |
|---|---|
| Jev 키 (필수) | `JEV_EVAL_JEV_KEY` → `JEV_TREE_INIT_API_KEY` → `TYPESAFE_API_KEY` |
| Jev 모델·주소 | `JEV_EVAL_JEV_MODEL` (기본 `jev-latest`), `JEV_EVAL_JEV_BASE_URL` |
| LLM (`jev_llm`) | `JEV_EVAL_LLM_BASE_URL`, `JEV_EVAL_LLM_TOKEN`, `JEV_EVAL_LLM_MODEL` |

키와 토큰은 어떤 결과 파일에도 남지 않습니다. `eval/tools/secret-scan.sh`로 검사합니다.

## 어떻게 도는가

서버를 띄우지 않고 `POST /api/run`과 같은 함수(`engine::execute`)를 요청 검증·제한 시간과 함께
직접 부릅니다. 셀(라우터 × 변형)마다 임시 SQLite에 시드를 넣고, 검색은 한 DB를 공유하며
저장 시나리오마다 새 DB를 씁니다. 작업 중인 DB는 건드리지 않습니다.

시작 전 **preflight**가 키와 LLM 설정을 한 건으로 확인합니다. LLM 리뷰 호출이 실패하면
그대로 진행하지 않고 멈춥니다(설정이 틀린 결과를 기록하지 않기 위해).

## 내 트리로 평가하기

골든셋(219노드)과 [`large`](large/README.md)(1,902노드)는 **깔끔하게 만든** 트리다. 노드 설명이 그 노드에 든 것을
빠짐없이 말하고, 같은 질문이 두 노드에 들어 있지 않다. 사람이 운영하는 트리는 그렇지 않다. 0.10.1은 골든셋에서
E2E 98%였지만, 운영 중인 고객 응대 트리(246노드·2,534아이템)의 실제 문의 489건에서는 61%였고 열 건 중 한 건은
"트리 밖"이라며 결과를 하나도 돌려주지 않았다. 0.11의 변경은 그 측정에서 나왔다([results/README.md](results/README.md)).
공개 벤치마크가 높다고 내 트리에서 잘 된다는 뜻이 아니다. 직접 잰다.

데이터는 저장소에 넣지 않는다(실제 문의와 운영 지식이다). 같은 형식으로 저장소 밖에 만들면 러너가 그대로 읽는다.

1. **지식베이스** — 운영 DB의 `.backup` 사본에서 노드와 활성 아이템을 `fixtures/kb/` 형식([DATASET.md](DATASET.md) §2)으로
   내보낸다. `ref`는 `<category_id>#<아이템 id>`. 질문 문자열이 겹치는 행은 뒤쪽 행 끝에 폭 없는 공백(U+200B)을 붙여
   구분한다(러너의 ref↔id 대조가 질문 문자열을 쓴다). 변형은 `full` 하나면 된다.
2. **질의** — 실제 문의 기록에서 뽑는다. 상담 에이전트가 검색할 때처럼 **한 문장 질의로 바꾸고 개인 식별 정보는 뺀다**
   (주문번호·이름·연락처가 든 원문을 평가기에 보내지 않는다). 기록이 없는 서브트리는 아이템 하나를 주고
   "이 항목이 답이 되는 문의"를 받아 쓴다.
3. **정답 라벨** — 판정 모델에게 그 서브트리의 **아이템 전부**(질문과 답 앞부분)와 질의를 주고, 그대로 보내면 해결되는
   항목(등급 2)과 일부만 도움이 되는 항목(등급 1)을 고르게 한다. 하나도 없으면 `answerable: false`(보류가 정답).
   라우터나 리뷰어와 **다른 계열의 모델**로 판정한다.
4. **풀링** — 실행한 뒤, 1위로 나왔지만 라벨에 없는 아이템을 같은 판정 모델에게 질의와 한 쌍씩 다시 묻는다. 첫 판정이
   놓친 정답 때문에 어떤 변형이 손해 보지 않게 한다. 비교하는 모든 실행의 1위를 같은 풀에 넣는다.
5. **실행** — 운영 트리는 골든셋의 규칙(snake_case id, 숲 전체에서 유일한 이름)을 따르지 않으므로 `--lenient`를 붙인다.

```bash
cargo run --release --example eval -- run --eval-dir /path/outside/the/repo --variant full \
    --suite search --lenient --settings-db /path/to/db.sqlite3 --out /path/outside/runs/<name> --no-history
```

읽을 숫자는 셋이다. 답이 있는 질의에서 **1위가 정답인 비율**과 **정답이 결과 안에 있는 비율**(에이전트가 결과를 읽고
쓰는 경우 이쪽이 실제 품질이다), 답이 없는 질의에서 **보류한 비율**. 비슷한 아이템이 여럿인 트리에서는 판정 모델의
"정답/일부" 구분이 흔들리므로 첫째 숫자는 낮게 나온다.

## 문서

| 문서 | 내용 |
|---|---|
| [DATASET.md](DATASET.md) | 데이터셋 형식, 라벨 규칙, 작성 규칙 |
| [METHOD.md](METHOD.md) | 평가 방식 — 무엇을 어떻게 재고 판정하는지 |
| [PLAN.md](PLAN.md) | 가설, 매트릭스, 단계, 합격 기준, 비용, 운영 |
| [golden/STATS.md](golden/STATS.md) | 데이터셋 구성 통계 (자동 생성) |
| [results/README.md](results/README.md) | 기준선 수치, 라우터 비교, 개선 제안, 기록표 |

## golden/router.jsonl — 라우터 회귀 케이스 (S-GAP-*)

`src/jev.rs`의 LLM 라우터가 "루트 → 어휘 후보 20개"에서 "범위 안 전체 노드를 한 번에 선택"으로
바뀌면서 없앤 약점을 고정하는 회귀 케이스다. 12건.

| 묶음 | 무엇을 보는가 | 이전(2단계) | 이후(한 번에) |
|---|---|---:|---:|
| S-GAP-001..008 | 설명과 어휘가 전혀 안 겹치는 질의(영문 ↔ 한글 설명) → 깊은 노드 | 5/8 | **8/8** |
| S-GAP-009..012 | 아주 가까운 범위 밖 질의 → `leaf_id: null` | 1/4 | **3/4** |
| 전체 라우팅 정확도 | | **50.0%** | **91.7%** |
| 지연 p50 | | 39.2초 | **12.5초** |

이전 라우터는 2단계였고, 둘째 단계가 **어휘 후보 20개**만 봤다. 질의와 설명이 단어를 하나도
안 공유하면(영문 질의 ↔ 한글 설명, 동의어) 정답 노드가 후보에서 밀려 한 단계 위에 멈추거나,
범위 밖 질문이 루트 선택에서 `__none__`을 고를 기회가 끝나 트리에 안겼다. 한 번에 고르면
둘 다 사라진다.
