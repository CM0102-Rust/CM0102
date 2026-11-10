#!/bin/bash
# The full pattern of a typed-records batch (10/11/2026), for each MODULE=Type pair given:
#   tipar_registros.py MODULE, --exe, --assinaturas Type, --acessos MODULE, then --propagar and --ajustar over all the
#   batch's types, limpar_tipos.py, and a final build. Each tool compiles and backs out a function that does not compile.
# Usage: bash engenharia-reversa/tipar_leva.sh offer=Offer cup=Cup ...
# [pt] O padrão completo de uma leva de registros tipados, para cada par MÓDULO=Tipo.
set -e
cd "$(dirname "$0")/.."
tipos=""
for par in "$@"; do
  mod=${par%%=*}; tipo=${par##*=}
  tipos="${tipos:+$tipos,}$tipo"
  echo "=== $mod ($tipo)"
  python engenharia-reversa/tipar_registros.py "$mod" --gravar 2>&1 | tail -2
  python engenharia-reversa/tipar_registros.py --exe --gravar 2>&1 | tail -1
  python engenharia-reversa/tipar_registros.py --assinaturas "$tipo" --gravar 2>&1 | tail -1
  python engenharia-reversa/tipar_registros.py --ajustar "$tipo" 2>&1 | tail -1
  python engenharia-reversa/tipar_registros.py --acessos "$mod" --gravar 2>&1 | tail -1
done
echo "=== propagar $tipos"
python engenharia-reversa/tipar_registros.py --propagar "$tipos" --gravar 2>&1 | tail -1
python engenharia-reversa/tipar_registros.py --ajustar "$tipos" 2>&1 | tail -1
python engenharia-reversa/tipar_registros.py --exe --gravar 2>&1 | tail -1
python engenharia-reversa/limpar_tipos.py --gravar 2>&1 | tail -1
(cd rust && cargo build --release 2>&1 | grep -E "^error|Finished" | head -5)
echo FIM
