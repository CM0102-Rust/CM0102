# 198ª leva — as telas da partida (6): a tela dos jogos do dia e o capitão padrão

**Rotinas:**
- `abrir_jogos_do_dia()` = 0x717cc0 (match_screens.cpp, cdecl): abre a tela dos jogos do dia numa vaga com o grupo de jogos (caixa de erro sem grupo);
- `responder_jogos_do_dia()` = 0x718930 (match_screens.cpp, cdecl): a resposta da tela dos jogos do dia: outro grupo de jogos, o andamento da competição ou a barra de menu;
- `vaga_do_mais_bem_avaliado()` = 0x727e20 (match_screens.cpp, cdecl): a vaga dos onze com a maior nota da comissão (o capitão padrão), −1 sem ninguém;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 197ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 198`
