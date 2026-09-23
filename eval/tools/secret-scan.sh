#!/usr/bin/env bash
# 민감 정보 스캐너: 커밋 전에 실제 값과 패턴을 검사한다.
set -uo pipefail
cd "$(git -C "$(dirname "$0")" rev-parse --show-toplevel)" || exit 1
echo "(스캔 루트: $(pwd))"

fail=0
echo "=== 1. 금지 파일 (키/DB/.env) 이 커밋 대상에 섞이지 않았는지"
forbidden=$(git ls-files --cached --others --exclude-standard eval src Cargo.toml 2>/dev/null \
  | grep -E '(^|/)(\.env$|\.jev-tree\.(key|salt)$|.*\.(db|db-wal|db-shm|pem|p12|key)$)' || true)
if [ -n "$forbidden" ]; then echo "  ✗ 발견: $forbidden"; fail=1; else echo "  ✓ 없음"; fi

echo
echo "=== 2. 알려진 실제 비밀 값이 소스에 나타나는지 (값은 노출하지 않음)"
scan_values() {
  local label="$1" value="$2" count
  if [ -z "$value" ]; then echo "  - ${label}: (환경변수 없음, 건너뜀)"; return; fi
  count=$(grep -RIl --binary-files=without-match -F "$value" eval src Cargo.toml 2>/dev/null | wc -l | tr -d ' ')
  if [ "$count" != "0" ]; then echo "  ✗ ${label}: 실제 값이 ${count}개 파일에 존재!"; fail=1
  else echo "  ✓ ${label}: 소스에 없음 (길이 ${#value})"; fi
}
scan_values "TYPESAFE_API_KEY (Jev)" "${TYPESAFE_API_KEY:-}"
scan_values "SLACK_USER_TOKEN" "${SLACK_USER_TOKEN:-}"
scan_values "DJANGO_ADMIN_GOOGLE_SSO_CLIENT_SECRET" "${DJANGO_ADMIN_GOOGLE_SSO_CLIENT_SECRET:-}"
if [ -n "${JEV_EVAL_LLM_TOKEN:-}" ]; then scan_values "JEV_EVAL_LLM_TOKEN" "$JEV_EVAL_LLM_TOKEN"; fi

echo
echo "=== 3. 토큰 모양 문자열 (sk-, ghp_, gho_, xox, AIza, jev_tree_sk_, v2: 암호문)"
hits=$(grep -RInE --binary-files=without-match \
  '(sk-[A-Za-z0-9_-]{16,}|ghp_[A-Za-z0-9]{20,}|gho_[A-Za-z0-9]{20,}|xox[baprs]-[A-Za-z0-9-]{10,}|AIza[0-9A-Za-z_-]{30,}|jev_tree_sk_[A-Za-z0-9]{10,}|v2:[A-Za-z0-9+/]{20,})' \
  eval src Cargo.toml 2>/dev/null | head -20 || true)
if [ -n "$hits" ]; then echo "$hits" | sed 's/^/  ✗ /'; fail=1; else echo "  ✓ 없음"; fi

echo
echo "=== 4. 개인정보 형태 (주민등록번호, 휴대폰, 계좌·카드 번호)"
python3 - <<'PY' || fail=1
import os,re,sys
pats = {
  "주민등록번호": r'(?<![\d.])\d{6}-[1-4]\d{6}(?!\d)',
  "휴대폰 번호":  r'(?<![\d.])01[016789][-. ]\d{3,4}[-. ]\d{4}(?!\d)',
  "카드·계좌 번호": r'(?<![\d.])\d{3,4}[- ]\d{3,4}[- ]\d{3,4}(?:[- ]\d{3,4})?(?!\d)',
  "하이픈 없는 긴 숫자(12~19자리)": r'(?<![\d.])\d{12,19}(?!\d)',
}
hits=[]
for root,dirs,files in os.walk('eval'):
    dirs[:]=[d for d in dirs if d!='cells']          # 셀 원본은 별도 관리
    for fn in files:
        if not fn.endswith(('.json','.jsonl','.md','.csv','.rs','.sh','.jsonc')): continue
        p=os.path.join(root,fn)
        try: text=open(p,encoding='utf-8').read()
        except Exception: continue
        for name,rx in pats.items():
            for m in re.finditer(rx,text):
                line=text.count('\n',0,m.start())+1
                hits.append(f"{p}:{line} [{name}] {m.group(0)[:24]}")
if hits:
    for h in hits[:20]: print("  ✗",h)
    if len(hits)>20: print(f"  … 외 {len(hits)-20}건")
    sys.exit(1)
print("  ✓ 없음")
PY

echo
echo "=== 5. 실행 결과에 비밀이 남았는지 (meta.json 의 models/command)"
if [ -d eval/results/runs ]; then
  for f in eval/results/runs/*/meta.json; do
    [ -e "$f" ] || continue
    echo "  - $f"
    python3 - "$f" <<'PY'
import json,sys,re
m=json.load(open(sys.argv[1]))
blob=json.dumps(m,ensure_ascii=False)
bad=[]
if re.search(r'(sk-[A-Za-z0-9_-]{16,}|gh[po]_[A-Za-z0-9]{20,}|v2:[A-Za-z0-9+/]{20,}|xox[baprs]-)', blob):
    bad.append("토큰 모양 문자열")
for k,v in (m.get("models") or {}).items():
    for field in ("jev_key","llm_token","token","key","password"):
        if field in v: bad.append(f"models.{k}.{field}")
    if re.search(r'://[^/]*:[^/@]*@', json.dumps(v)): bad.append(f"models.{k} URL 자격증명")
if bad: print("    ✗ "+", ".join(bad))
else: print("    ✓ command/models 에 비밀 없음")
print(f"    (command: {m.get('command','')[:100]})")
PY
  done
else
  echo "  - 아직 실행 결과 없음"
fi

echo
if [ "$fail" = "0" ]; then echo "RESULT: 통과 — 커밋해도 안전"; else echo "RESULT: 실패 — 반드시 수정"; exit 1; fi
