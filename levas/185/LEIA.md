# 185ª leva — a marca de contrato acertado e a vaga da oferta

**Rotinas:**
- `marcar_contrato_acertado()` = 0x8c5410 (transfer_manager.cpp, thiscall): liga a marca de contrato acertado (0x40) da pessoa no gerente de transferências;
- `desmarcar_contrato_acertado()` = 0x8c5430 (transfer_manager.cpp, thiscall): desliga a marca de contrato acertado da pessoa;
- `vaga_da_oferta()` = 0x8c61e0 (transfer_manager.cpp, thiscall): o endereço da vaga de uma oferta (fora da faixa, o começo da tabela);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 1 acusadas. `t_desmarca` escapou: quando a marca de contrato acertado é desligada, a pessoa nunca tem também a marca 0x80. `t_vaga` escapou: a carreira nunca pede a vaga de oferta igual à quantidade (fora da faixa). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 184ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 185`
