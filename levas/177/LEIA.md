# 177ª leva — o bit do contrato, a marca da competição e o registro do clube na liga

**Rotinas:**
- `bits_do_registro()` = 0x7a7eb0 (player_regen.cpp, stdcall): os bits 4..5 do byte +0xb do registro de 79 bytes da pessoa;
- `competicao_marcada()` = 0x684790 (league.cpp, thiscall): a competição tem +0xbe e a marca 2 de +0xd9?;
- `registro_da_liga()` = 0x6826d0 (league.cpp, thiscall): o registro (59 bytes) da tabela +0xb1;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 2 acusadas. `m_marca` escapou: nas ligas inglesas as competições com +0xbe ligado nunca têm os bits 1 e 2 de +0xd9 diferentes (falta de dados; vai para a carreira ampla). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 176ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 177`
