# 193ª leva — as cláusulas do contrato: percentual da próxima venda, bônus por jogo e bônus pela seleção

**Rotinas:**
- `tirar_percentual_da_proxima_venda()` = 0x8cb4f0 (transfer_manager.cpp, thiscall): tira o percentual da próxima venda do jogador (marca e registro da lista);
- `tirar_bonus_por_jogo()` = 0x8cb590 (transfer_manager.cpp, thiscall): tira o bônus por jogo no time principal (marca e registro da lista);
- `tirar_bonus_pela_selecao()` = 0x8cb630 (transfer_manager.cpp, thiscall): tira o bônus por jogo pela seleção (marca e registro da lista);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 0 acusadas. `l_venda` escapou: nenhuma cláusula de contrato (percentual da próxima venda, bônus por jogo, bônus pela seleção) é tirada na carreira até 2002-06. `l_jogo` escapou: nenhuma cláusula de contrato (percentual da próxima venda, bônus por jogo, bônus pela seleção) é tirada na carreira até 2002-06. `l_conta` escapou: nenhuma cláusula de contrato (percentual da próxima venda, bônus por jogo, bônus pela seleção) é tirada na carreira até 2002-06. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 192ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 193`
