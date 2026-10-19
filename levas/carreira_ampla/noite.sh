#!/bin/bash
# A carreira ampla da madrugada (so depois do "boa noite" do Lucas): prova geral original x Rust e o lote
# das mutacoes que escaparam por falta de dados. Uso: bash levas/carreira_ampla/noite.sh AAAA-MM-DD(prova) AAAA-MM-DD(lote) lote.py
cd "$(dirname "$0")/../../engenharia-reversa" || exit 1
LIGAS="ENG ESP ITA FRA BRA JPN AUS USA"
SAIDA=../levas/carreira_ampla/noite_$(date +%Y-%m-%d).txt
{
  echo "=== PROVA GERAL AMPLA ate $1 ($LIGAS)"
  python -u simular.py --ate "$1" --motor sim --modo rust --ligas "$LIGAS" --limite 7200
  python -u simular.py --ate "$1" --motor sim --modo original --ligas "$LIGAS" --limite 7200
  echo "=== LOTE $3 ate $2"
  python -u mutacoes.py "mutacoes/$3" --carreira "$2" --ligas "$LIGAS" --limite 7200
  echo "=== FIM"
} >> "$SAIDA" 2>&1
