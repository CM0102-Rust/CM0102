# 179ª leva — as ofertas de contrato e a oferta de transferência fechada

**Rotinas:**
- `oferta_de_contrato()` = 0x4cc9e0 (contract_manager.cpp, thiscall): a oferta de contrato da pessoa para o clube, na lista do gerente de contratos;
- `oferta_acertada()` = 0x4cca30 (contract_manager.cpp, thiscall): a oferta de contrato acertada (estado 2) da pessoa;
- `oferta_fechada()` = 0x8d0d90 (transfer_offer.cpp, stdcall): a oferta de transferência está encerrada (pelo estado e, no estado 0xf, pela fase);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (4):** 2 acusadas. `f_estado` escapou: na carreira as ofertas de transferência consultadas nunca estão no estado 4. `f_fase` escapou: nenhuma oferta no estado 0xf está na fase 2 quando é consultada. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 178ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 179`
