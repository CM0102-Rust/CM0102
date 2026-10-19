# 174ª leva — a pessoa na lista do técnico, a ficha de contrato e o desgaste diário

**Rotinas:**
- `pessoa_na_lista()` = 0x69fc60 (manager_manager.cpp, thiscall): a pessoa está na lista (iterador 0x68d1b0, devolvido no fim)?;
- `ficha_de_contrato()` = 0x4cf6d0 (contract_manager.cpp, thiscall): a ficha de contrato (0x50 bytes) da pessoa;
- `desgaste_diario()` = 0x62c2f0 (injury.cpp, thiscall): o desgaste do dia na palavra +0xc (idade, sorteios de 25 e 13, teto de goleiro);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (7):** 7 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 173ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 174`
