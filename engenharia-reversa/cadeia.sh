#!/bin/bash
# Cadeia de testes de uma leva do motor (ver docs/COMO_CONTINUAR.md e levas/LEIA.md).
# Uso (Git Bash, na raiz ou em qualquer pasta): bash engenharia-reversa/cadeia.sh N [AAAA-MM-DD | -]
#   1. mutacoes de mutacoes/levaN.py pela carreira automatica ate 2002-06-01, confirmando no modo
#      comparacao ate a data dada (padrao 2001-08-20; "-" so pela carreira); leva do save (levas/N/SAVE,
#      criado pelo fila.py): pela prova do save (mutacoes.py --prova-save N), que a carreira nao exercita;
#   1b. so leva do save: === PROVA DO SAVE (prova_save.py --leva N: a mesma DLL com e sem as rotinas;
#       com levas/N/CARGA tambem a carga do save de ferias, nas mutacoes e na prova);
#   2. jogo real comparando ate 2002-01-01 (divergencias e excecoes do ddraw.log);
#   3. carreiras Rust x2 (iguais entre si) e original ate 2002-06-01 (desde 31/10/2026 a original e so registrada).
# Saida completa em levas/N/cadeia.txt. Com a base divergindo, o mutacoes.py sai com erro e o resto
# nao roda. Testes pesados passam pela trava do laboratorio (laboratorio.py).
RAIZ="$(cd "$(dirname "$0")/.." && pwd)"
N=$1
DATA=${2:-2001-08-20}
mkdir -p "$RAIZ/levas/$N"
cd "$RAIZ/engenharia-reversa"
CONF="--confirmar $DATA"; [ "$DATA" = "-" ] && CONF=""
SAVE=""; [ -f "$RAIZ/levas/$N/SAVE" ] && SAVE=1
MODO="--carreira 2002-06-01 $CONF --confirmar-pt"; [ -n "$SAVE" ] && MODO="--prova-save $N"
(if python -u mutacoes.py mutacoes/leva$N.py $MODO; then
  if [ -n "$SAVE" ]; then
    python laboratorio.py | head -1
    echo "=== PROVA DO SAVE"
    python -u prova_save.py --leva $N
  fi
  python laboratorio.py | head -1
  echo "=== JOGO REAL"
  python -u simular.py --ate 2002-01-01 --motor sim --modo comparar --limite 3600
  grep -aE "DIVERG|EXCECAO" ../jogo_testes/ddraw.log | head -5 | cut -c1-300
  grep -aE "comparacoes" ../jogo_testes/ddraw.log | tail -3
  grep -a "sonda_lixo" ../jogo_testes/ddraw.log | tail -1
  python laboratorio.py | head -1
  echo "=== CARREIRA"
  python -u simular.py --ate 2002-06-01 --motor sim --modo rust --repetir 2 --limite 2400 && python -u simular.py --ate 2002-06-01 --motor sim --modo original --limite 2400
  python laboratorio.py | head -1
else
  echo "=== MUTACOES FALHARAM"
fi
echo "=== FIM") > "$RAIZ/levas/$N/cadeia.txt" 2>&1
