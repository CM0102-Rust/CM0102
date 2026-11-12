#!/bin/bash
# Cadeia de testes de uma leva do motor (ver docs/COMO_CONTINUAR.md e levas/LEIA.md).
# Uso (Git Bash, na raiz ou em qualquer pasta): bash engenharia-reversa/cadeia.sh N [AAAA-MM-DD | -]
#   1. mutacoes de mutacoes/levaN.py pela carreira automatica ate 2002-06-01, confirmando no modo
#      comparacao ate a data dada (padrao 2001-08-20; "-" so pela carreira); leva do save (levas/N/SAVE,
#      criado pelo fila.py): pela prova do save (mutacoes.py --prova-save N), que a carreira nao exercita;
#   1b. so leva do save: === PROVA DO SAVE (prova_save.py --leva N: a mesma DLL com e sem as rotinas;
#       com levas/N/CARGA tambem a carga do save de ferias, nas mutacoes e na prova);
#   2. jogo real comparando ate 2002-01-01 (divergencias e excecoes do ddraw.log);
#   3. carreiras Rust x2 (iguais entre si) e original ate 2002-06-01 (desde 31/10/2026 a original e so registrada);
#      depois das Rust x2, o detector de lixo (simular.py --detector, 01/11/2026): uma terceira com a pilha particular
#      envenenada, so registro ("detector de lixo: igual" ou "DIFERENTE (...)"); nao decide a cadeia.
#   Leva de competicoes (levas/N/LIGAS, criado pelo fila.py, 03/11/2026): mutacoes, jogo real e carreiras com essas
#   ligas, e o jogo real comparando ate 2002-06-01 (fim de temporada: acessos, copas, vagas). Desde 05/11/2026 (Lucas)
#   esse jogo real fica adiado ("=== JOGO REAL ADIADO") e roda uma vez so, acumulado, na auditoria das 8 ligas;
#   COMPARAR_LIGAS=1 o roda na cadeia.
#   LIGAS = MUNDO (12/11/2026): todas as nacoes do nation.dat (simular.py), ate 2002-08-01 (passa pela virada de
#   temporada); o jogo real comparando roda na cadeia (nao e adiado: a auditoria das 8 ligas nao cobre essas rotinas).
# Saida completa em levas/N/cadeia.txt. Com a base divergindo, o mutacoes.py sai com erro e o resto
# nao roda. Testes pesados passam pela trava do laboratorio (laboratorio.py).
RAIZ="$(cd "$(dirname "$0")/.." && pwd)"
N=$1
DATA=${2:-2001-08-20}
mkdir -p "$RAIZ/levas/$N"
cd "$RAIZ/engenharia-reversa"
CONF="--confirmar $DATA"; [ "$DATA" = "-" ] && CONF=""
SAVE=""; [ -f "$RAIZ/levas/$N/SAVE" ] && SAVE=1
FIM=2002-06-01; [ "$(head -1 "$RAIZ/levas/$N/LIGAS" 2>/dev/null | tr -d '\r')" = MUNDO ] && FIM=2002-08-01
MODO="--carreira $FIM $CONF --confirmar-pt"; [ -n "$SAVE" ] && MODO="--prova-save $N"
LIGAS=""; ATE_REAL=2002-01-01; LIM_REAL=3600
if [ -f "$RAIZ/levas/$N/LIGAS" ]; then LIGAS="$(head -1 "$RAIZ/levas/$N/LIGAS" | tr -d '\r')"; ATE_REAL=$FIM; LIM_REAL=30000; fi
LIM_CAR=2400; [ "$LIGAS" = MUNDO ] && LIM_CAR=20000
L=(); M=(); [ -n "$LIGAS" ] && L=(--ligas "$LIGAS") && M=(--ligas "$LIGAS" --limite 3600)
(if python -u mutacoes.py mutacoes/leva$N.py $MODO "${M[@]}"; then
  if [ -n "$SAVE" ]; then
    python laboratorio.py | head -1
    echo "=== PROVA DO SAVE"
    python -u prova_save.py --leva $N
  fi
  if [ -n "$LIGAS" ] && [ "$LIGAS" != MUNDO ] && [ "$COMPARAR_LIGAS" != 1 ]; then
    # 05/11/2026 (Lucas): a comparacao longa de 8 ligas roda uma vez so, depois de muitas levas (auditoria.ps1, de madrugada)
    echo "=== JOGO REAL ADIADO (comparacao de 8 ligas ate 2002-06-01 acumulada na auditoria; COMPARAR_LIGAS=1 roda aqui)"
  else
  python laboratorio.py | head -1
  echo "=== JOGO REAL"
  python -u simular.py --ate $ATE_REAL --motor sim --modo comparar --limite $LIM_REAL "${L[@]}"
  grep -aE "DIVERG|EXCECAO|EXCEPTION" ../jogo_testes/ddraw.log | head -5 | cut -c1-300
  grep -aE "comparacoes|comparisons" ../jogo_testes/ddraw.log | tail -3
  grep -a "sonda_lixo" ../jogo_testes/ddraw.log | tail -1
  fi
  python laboratorio.py | head -1
  echo "=== CARREIRA"
  python -u simular.py --ate $FIM --motor sim --modo rust --repetir 2 --detector --limite $LIM_CAR "${L[@]}" && python -u simular.py --ate $FIM --motor sim --modo original --limite $LIM_CAR "${L[@]}"
  python laboratorio.py | head -1
else
  echo "=== MUTACOES FALHARAM"
fi
echo "=== FIM") > "$RAIZ/levas/$N/cadeia.txt" 2>&1
