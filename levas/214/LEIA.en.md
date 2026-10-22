# Batch 214 — the league table row order, the league round-robin grid and the last-day goal margin; the match stored in the news item, the cup tie result for the news and the cup round match day; the formation position lookup, the club score for a youth player and the match watched by the human

**Routines:**
- `ordem_das_linhas_da_tabela()` = 0x683210 (league.cpp, cdecl): the table qsort comparison: points, the league tiebreak rules and the club id;
- `montar_grade_de_turnos()` = 0x6844b0 (league.cpp, stdcall): builds the grid where every club meets each other once, with balanced home and away games;
- `margem_do_ultimo_dia()` = 0x68a4d0 (league.cpp, thiscall): on the last matchday, the goal margin a club needs for the title, the top places or survival, and the deciding-match flag;
- `partida_das_vagas_da_noticia()` = 0x4b0010 (comp_util.cpp, cdecl): rebuilds the fixture from eighteen consecutive news slots and resolves its clubs, stadium and competition;
- `resultado_da_eliminatoria_na_noticia()` = 0x4b0890 (comp_util.cpp, cdecl): the category of the cup tie result for the club, from the final margin, the side and the reputation gap;
- `dia_da_rodada_da_copa()` = 0x521f10 (Cup.cpp, cdecl): stores one of the eight cup round match days: the venue flags, the weekday, the time slot and the code;
- `mapa_de_casas_da_formacao()` = 0x5b7160 (formation.cpp, thiscall): expands each formation slot's positions into the pitch cell tables and their mirrored copy;
- `nota_do_clube_para_o_jovem()` = 0x7ad220 (player_regen.cpp, stdcall): the random score of how well the club suits the youth player: reputation against ability, youth facilities, human control and nation;
- `partida_assistida_pelo_humano()` = 0x5fe290 (human_manager.cpp, thiscall): whether the fixture is the one the human manager chose to watch;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (9):** 8 caught. `lt_ultimo_dia` escaped: the margin is only asked by 0x68a3e0 (match importance), which stores the byte in a field; in the real game comparing (to 2002-01-01) it is not called, and in the career to 2002-06-01 0xfe instead of 0xff (no margin) changes no decision. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (5afd987e1a1a3ef3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b11bd5cb63f51d72).

**Redo**, on top of batch 213's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 214`
