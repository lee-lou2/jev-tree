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
| **케이스 513건** | 검색 433 · 저장 68 · API 계약 12. 여러 턴 문맥 9종, 루트/시작 노드 진입, 막연한·답 없는 질문, 범위 밖, 프롬프트 주입, 영역 경계 혼동 |
| **라우터 2종** | `jev`(Jev 빔 라우팅) · `jev_llm`(LLM 라우팅 + Jev 랭킹) |
| **split** | 약 30%가 `test`(id 해시) |

트리 변형은 **같은 아이템을 깊이만 바꿔** 담아서 구조의 영향을 분리해 봅니다. 빈 리프 3개와
아이템 55–83개가 몰린 대용량 리프 3개를 일부러 넣었습니다.

## 폴더

```
fixtures/kb/   지식베이스 — manifest.json · nodes.json · items/*.jsonl
golden/        골든 케이스 — 영역별 · dense(대용량 리프) · cross(교차·범위 밖·주입) · contract(API)
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
cargo run --release --example eval -- … --settings-db /path/to/data/demo.db
```

DB에 없거나 덮어쓸 때만 환경 변수를 씁니다(앞 것이 우선).

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

시작 전 **preflight**가 키와 LLM 설정을 한 건으로 확인합니다. LLM 호출이 실패해 Jev 빔으로
대체되면 그대로 진행하지 않고 멈춥니다(설정이 틀린 결과를 기록하지 않기 위해).

## 문서

| 문서 | 내용 |
|---|---|
| [DATASET.md](DATASET.md) | 데이터셋 형식, 라벨 규칙, 작성 규칙 |
| [METHOD.md](METHOD.md) | 평가 방식 — 무엇을 어떻게 재고 판정하는지 |
| [PLAN.md](PLAN.md) | 가설, 매트릭스, 단계, 합격 기준, 비용, 운영 |
| [golden/STATS.md](golden/STATS.md) | 데이터셋 구성 통계 (자동 생성) |
| [results/README.md](results/README.md) | 기준선 수치, 라우터 비교, 개선 제안, 기록표 |
