# 190ª leva — o contrato de empréstimo, as marcas de lance da lista de observados e o técnico substituto

**Rotinas:**
- `tirar_contrato_de_emprestimo()` = 0x4d0bd0 (contract_manager.cpp, thiscall): libera a vaga do contrato de empréstimo da pessoa, se houver;
- `desmarcar_lances_pelo_jogador()` = 0x84aeb0 (shortlist_manager.cpp, thiscall): tira a marca de lance das vagas da lista de observados do clube que miram o jogador;
- `tecnico_substituto()` = 0x600e90 (human_manager.cpp, cdecl): o auxiliar do clube ou, sem ele, o membro da comissão de maior reputação;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 2 acusadas. `s_empate` escapou: o técnico substituto só é pedido para o clube humano, e na comissão dele nunca há empate de reputação. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 189ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 190`
