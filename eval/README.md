# eval — jev-tree 평가

> **In English** — This directory holds a Korean golden dataset and runner for measuring how
> well `jev-tree` files and finds knowledge in its taxonomy. Content is Korean because the
> golden set tests Korean user utterances; the runner code is English like the rest of the crate.
> Start with [`PLAN.md`](PLAN.md) for the hypotheses and evaluation matrix,
> [`DATASET.md`](DATASET.md) for the schema, and [`results/README.md`](results/README.md) for the
> measured baseline (tree depth hurts routing; ranking abstention is the second line of defence).
> Run `cargo run --release --example eval -- help`.

jev-tree의 검색(`search`)과 저장(`ingest`) 품질을 재는 골든 데이터셋, 실행기, 평가 방식과 계획.

| 문서 | 내용 |
|---|---|
| [DATASET.md](DATASET.md) | 골든 데이터셋 형식, 라벨 규칙, 작성 규칙 |
| [METHOD.md](METHOD.md) | 평가 방식: 무엇을 어떻게 재고 판정하는지 |
| [PLAN.md](PLAN.md) | 평가 계획: 가설, 매트릭스, 단계, 합격 기준, 비용, 운영 |
| [golden/STATS.md](golden/STATS.md) | 데이터셋 구성 통계 (자동 생성) |
| [results/README.md](results/README.md) | 결과 기록 규칙과 기록표 |

## 한눈에

- **지식베이스**: 범용 8개 영역(IT·생활·금융·건강·여행·요리·직장·쇼핑), 노드 219개(리프 147), 아이템 669개.
  빈 리프 3개, 아이템이 55–83개씩 몰린 대용량 리프 3개.
- **트리 변형 5종**: 같은 아이템을 `deep`(깊이 7) · `standard`(4) · `shallow`(2) · `flat`(1)로,
  `sparse`는 standard 트리에 리프당 아이템 1개만 남긴다.
- **케이스 513건**: search 433 · ingest 68 · contract 12.
  단일 턴·여러 턴(9가지 문맥 패턴), 루트·가상 루트·시작 노드 진입, 막연한 질문, 답 없는 질문, 범위 밖 질문,
  프롬프트 주입, 영역 경계 혼동을 담았다. 약 30%는 `test` split.
- **라우터 2종**: `jev`(Jev만) · `jev_llm`(LLM 라우팅 + Jev 랭킹).

## 폴더

```
eval/
├── fixtures/kb/        지식베이스: manifest.json · nodes.json · items/*.jsonl
├── golden/             골든 케이스: 영역별 · dense(대용량 리프) · cross(교차·범위 밖) · contract(API)
├── runner/             실행기 소스 (cargo example `eval`)
└── results/            실행 결과와 기록표
```

## 빠른 시작

저장소 루트에서 실행한다.

```bash
# 1) 데이터 점검 (키 필요 없음)
cargo run --release --example eval -- validate --strict
cargo run --release --example eval -- run --mock oracle --router both --variant all   # 하네스 점검, 라우팅 100%여야 정상

# 2) 스모크: 60건만
export JEV_EVAL_JEV_KEY=...                       # Jev(TypeSafe) 키
cargo run --release --example eval -- run --router jev --sample 60 --note "smoke"

# 3) 전체: 두 라우터 × 다섯 변형
export JEV_EVAL_LLM_BASE_URL=https://api.openai.com/v1 JEV_EVAL_LLM_TOKEN=... JEV_EVAL_LLM_MODEL=...
cargo run --release --example eval -- run --router both --variant all --note "baseline"

# 4) 비교
cargo run --release --example eval -- compare eval/results/runs/<A> eval/results/runs/<B>
cargo run --release --example eval -- compare eval/results/runs/<run> --a jev/standard --b jev_llm/standard
```

`run --dry-run`은 셀별 케이스 수만 보여 준다. 전체 옵션은 `cargo run --release --example eval -- help`.

## 키 설정

| 용도 | 환경 변수 (앞의 것이 우선) |
|---|---|
| Jev 키 (필수) | `JEV_EVAL_JEV_KEY` → `JEV_TREE_INIT_API_KEY` → `TYPESAFE_API_KEY` |
| Jev 모델·주소 (선택) | `JEV_EVAL_JEV_MODEL` (기본 `jev-latest`), `JEV_EVAL_JEV_BASE_URL` |
| LLM (`jev_llm`에 필수) | `JEV_EVAL_LLM_BASE_URL`, `JEV_EVAL_LLM_TOKEN`, `JEV_EVAL_LLM_MODEL` (`JEV_TREE_INIT_*`도 읽음) |

로컬 서버 UI에 이미 키를 저장해 두었다면 `--settings-db data/demo.db`로 그 값을 읽는다.
DB는 읽기 전용으로 열고, 옆의 `.jev-tree.key`·`.jev-tree.salt`가 있어야 한다. 키와 토큰은 결과 파일에 남지 않는다.

## 어떻게 도는가

러너는 서버를 띄우지 않고 `POST /api/run`과 같은 함수(`engine::execute`)를 요청 검증·제한 시간과 함께 직접 부른다.
셀(라우터 × 변형)마다 임시 SQLite에 변형 시드를 넣고, search는 한 DB를 공유하며 ingest는 시나리오마다 새 DB를 쓴다.
운영 DB(`data/demo.db`)는 건드리지 않는다.

변형 하나를 서버에 올려 UI로 보고 싶으면:

```bash
cargo run --release --example eval -- export --variant deep --out /tmp/kb-deep.json
JEV_TREE_DB=/tmp/kb-deep.db JEV_TREE_SEED=/tmp/kb-deep.json PORT=8769 cargo run --release
```

## 결과 읽기

`eval/results/runs/<run_id>/report.md`의 순서:

1. **요약**: 셀별 E2E 정답률, 라우팅 정확도, Hit@1, MRR, 트리 밖 판정 F1, 오답변·오보류율, 지연, 저장 정확도, 계약 통과율
2. **셀별 상세**: 지표와 95% 구간, 깊이별 경로 일치율, 라우팅 오류 유형
3. **세부 분석**: 기대 결과·문맥 패턴·진입점·깊이·후보 수·스타일·영역·난이도별 표
4. **보류 임계값 스윕**: 0.30을 바꿨을 때의 E2E·커버리지·오답변율
5. **저장·계약 결과**, **자주 틀린 착지**, **틀린 케이스 목록**

지표 정의는 [METHOD.md](METHOD.md).
