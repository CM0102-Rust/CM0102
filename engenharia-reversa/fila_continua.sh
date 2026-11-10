#!/bin/bash
# Roda a fila sozinha: para cada leva N já integrada (levas/N existe e não foi publicada), roda a cadeia,
# confere o resultado e publica (o publicar_leva.py integra a N+1). Para (código 1) quando algo pede gente:
# mutação que escapou sem ser acusada comparando, prova do save DIFERENTE (levas/N/SAVE), divergência, jogo ou carreira que não chegou à data, falha ao publicar,
# ou um jogo aberto no jogo_testes (o do Lucas, no jogo/, não conta desde 25/10). Uso: bash engenharia-reversa/fila_continua.sh N
cd "$(dirname "$0")/.." || exit 1
n=$1
while [ -d "levas/$n" ]; do
  # só o jogo dos testes (jogo_testes/) conta: desde 25/10 o mutex leva o caminho do exe e o Lucas joga no jogo/
  # ao mesmo tempo; outras sessões têm exes com o mesmo nome (o jogo falso do CMRTE se chama cm0102.exe)
  if python -c "import os, psutil, sys; k = os.path.join('sistemas', 'cm0102', 'jogo_testes', ''); sys.exit(any(k in (p.info['exe'] or '').lower() for p in psutil.process_iter(['exe'])))"; then :; else echo "PARADA: jogo aberto antes da leva $n"; exit 1; fi
  echo "=== leva $n: cadeia ($(date +%H:%M))"
  bash engenharia-reversa/cadeia.sh "$n" > /dev/null 2>&1
  c="levas/$n/cadeia.txt"
  grep -q "=== FIM" "$c" || { echo "PARADA: cadeia $n sem FIM"; exit 1; }
  grep -q "determinismo: OK" "$c" || { echo "PARADA: cadeia $n sem determinismo"; exit 1; }
  # leva do save: a prova (a mesma DLL com e sem as rotinas) tem de dar IGUAL
  if [ -f "levas/$n/SAVE" ] && ! grep -aq "^prova da leva $n: IGUAL" "$c"; then
    echo "PARADA: cadeia $n: $(grep -a "^prova da leva $n:" "$c" | tail -1 | grep . || echo "prova do save ausente")"; exit 1
  fi
  # desde 31/10/2026 (Lucas) não há referência fixa: a carreira original é só registrada; para se o jogo caiu ou não chegou à data
  if ! python -c "import sys; sys.path.insert(0, 'engenharia-reversa'); import gen_docs; sys.exit(0 if gen_docs.cadeia($n)[3] else 1)"; then
    echo "PARADA: cadeia $n com jogo ou carreira que não chegou à data (ou Rust ×2 diferentes)"; exit 1
  fi
  if grep -a -E "^ +(data|motor|aleatorio): .* [1-9][0-9]* divergencias" "$c" | grep -q .; then
    echo "PARADA: cadeia $n com divergência no jogo real"; exit 1
  fi
  # mutação que escapou da carreira e também do comparando pede explicação (ESCAPADAS) à mão
  esc=$(python - "$c" <<'EOF'
import re, sys
linhas = open(sys.argv[1], encoding='utf-8', errors='replace').read().split('\n')
acusadas, escaparam = set(), set()
for l in linhas:
    m = re.match(r'mutacao (\S+)\s+\d+s (.*)', l)
    if not m:
        continue
    nome, r = m.groups()
    if r.startswith('ACUSADA') or ('comparando' in r and ('DIVERGENCIA' in r or 'DIVERGENCE' in r)) or ('comparando' in r and ('EXCECAO' in r or 'EXCEPTION' in r)):
        acusadas.add(nome)
    elif r.startswith('IGUAL A BASE') or 'comparando' in r:
        escaparam.add(nome)
print(' '.join(sorted(escaparam - acusadas)))
EOF
)
  if [ -n "$esc" ]; then echo "PARADA: leva $n com mutações escapadas: $esc"; exit 1; fi
  if ! python engenharia-reversa/publicar_leva.py "$n" > "${TMP:-/tmp}/publicar_$n.log" 2>&1; then
    # sem próximo item na fila o publicar termina com erro depois de publicar: confere o commit
    if git log -1 --format=%s | grep -q "^[A-Za-z]*: ${n}a leva"; then echo "publicada a $n; fila vazia"; exit 0; fi
    echo "PARADA: falha ao publicar a $n (${TMP:-/tmp}/publicar_$n.log)"; exit 1
  fi
  echo "publicada a $n: $(git log -1 --format=%h)"
  n=$((n + 1))
done
echo "fim da fila em $n"
