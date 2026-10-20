# 191ª leva — o clube do contrato acertado, o jogo pela seleção e a punição guardada

**Rotinas:**
- `clube_do_contrato_acertado()` = 0x8c5450 (transfer_manager.cpp, thiscall): o clube do contrato acertado da pessoa (desliga a marca quando o contrato sumiu);
- `registrar_jogo_pela_selecao()` = 0x8ce920 (transfer_manager.cpp, thiscall): registra o jogo do jogador pela seleção nos bits dos últimos jogos;
- `punicao_guardada()` = 0x55cb60 (discipline.cpp, thiscall): a punição guardada do jogador numa vaga, ou nenhuma;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (3):** 1 acusadas. `c_clube` escapou: na carreira o clube do contrato acertado nunca é pedido com a marca ligada e o contrato achado. `p_vaga` escapou: nenhum jogador tem duas punições guardadas em vagas diferentes quando a busca roda. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 190ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 191`
