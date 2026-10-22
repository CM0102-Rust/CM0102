#!/bin/bash
# Roda a fila sozinha: para cada leva N já integrada (levas/N existe e não foi publicada), roda a cadeia,
# confere o resultado e publica (o publicar_leva.py integra a N+1). Para (código 1) quando algo pede gente:
# mutação que escapou sem ser acusada comparando, divergência, carreira fora da referência, falha ao publicar,
# ou o jogo aberto (o Lucas jogando). Uso: bash engenharia-reversa/fila_continua.sh N
cd "$(dirname "$0")/.." || exit 1
# as referências vêm do gen_docs.py (mudam com uma zeragem nova do original determinizado)
REF_CARREIRA=$(python -c "import sys; sys.path.insert(0, 'engenharia-reversa'); import gen_docs; print(gen_docs.REF_CARREIRA)")
REF_JOGO=$(python -c "import sys; sys.path.insert(0, 'engenharia-reversa'); import gen_docs; print(gen_docs.REF_JOGO)")
n=$1
while [ -d "levas/$n" ]; do
  if tasklist | grep -qi cm0102; then echo "PARADA: jogo aberto antes da leva $n"; exit 1; fi
  echo "=== leva $n: cadeia ($(date +%H:%M))"
  bash engenharia-reversa/cadeia.sh "$n" > /dev/null 2>&1
  c="levas/$n/cadeia.txt"
  grep -q "=== FIM" "$c" || { echo "PARADA: cadeia $n sem FIM"; exit 1; }
  grep -q "determinismo: OK" "$c" || { echo "PARADA: cadeia $n sem determinismo"; exit 1; }
  if grep -a "^execucao" "$c" | grep -v "impressao $REF_CARREIRA\|impressao $REF_JOGO" | grep -q .; then
    echo "PARADA: cadeia $n com carreira fora da referência"; exit 1
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
    if r.startswith('ACUSADA') or ('comparando' in r and 'DIVERGENCIA' in r) or ('comparando' in r and 'EXCECAO' in r):
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
