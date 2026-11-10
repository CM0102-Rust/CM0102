#!/bin/bash
# The readability style for code that comes in (10/11/2026): run by publicar_leva.py right after a batch is
# integrated, so a queue item written in the old style (raw derefs, Portuguese locals and messages, raw float
# addresses, untyped records) enters in the current one and is tested that way by its own chain. Every tool is
# idempotent on code already in style and puts back any function that does not compile.
# Usage: bash engenharia-reversa/padronizar.sh
# [pt] O estilo de legibilidade para o código que entra: roda depois de integrar uma leva (publicar_leva.py).
set -e
cd "$(dirname "$0")"
export PYTHONIOENCODING=utf-8
# a queue item written against the old API (a function that took usize and now takes Club): the compiler's messages
# fix the calls first (`.addr()` / `Club(..)`), so the passes below start from code that builds
TIPOS=$(python -c "import re;print(','.join(re.findall(r'pub\(crate\) struct (\w+)\(pub\(crate\) usize\);', open('../rust/src/motor/records.rs',encoding='utf-8').read())))")
python tipar_registros.py --ajustar "$TIPOS" 2>&1 | tail -1
for passo in "unificar_acessos.py --gravar" "tabelas_globais.py --gravar" "expandir_leitoras.py --gravar" \
             "nomear_rdata.py --gravar" "traduzir_locais.py --gravar" "textos_em_ingles.py --todos --gravar" \
             "nomear_deslocamentos.py --gravar"; do
  echo "== $passo"
  python $passo 2>&1 | grep -v "^volta" | tail -1
done
for par in club=Club staff=Staff fixture=Fixture match_player=MatchPlayer contract=Contract offer=Offer \
           player_sheet=PlayerSheet league=League nation=Nation; do
  mod=${par%%=*}; tipo=${par##*=}
  echo "== tipar $mod"
  python tipar_registros.py "$mod" --gravar 2>&1 | grep -v "^volta" | tail -1
  python tipar_registros.py --acessos "$mod" --gravar 2>&1 | grep -v "^volta" | tail -1
done
python tipar_registros.py --exe --gravar 2>&1 | tail -1
python limpar_tipos.py --gravar 2>&1 | grep -v "^volta" | tail -1
# the compiler's own fixes (unnecessary parentheses, unused imports and mut), checked by it (readability batch 33)
(cd ../rust && cargo fix --lib --release --allow-dirty 2>&1 | tail -1)
python espacos_do_cargo_fix.py HEAD
echo "padronizado"
