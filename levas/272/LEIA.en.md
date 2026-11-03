# Batch 272 — the Champions League: the phase 1 group draw (same-nation partners, halves, pots, prize, stages and the losers to the UEFA Cup), the pot order and the predicted positions; the Champions League: the phase 2 group draw (same-group pairs, halves, nations, prize, stages, third places to the UEFA Cup) and the qualifiers' order; the group of a stage by id; the Champions League: the round schedule of the group stages (the dates of both phases, by half); the league: the season expectations and a club relegation; the competition: all stages complete, the club list and the club progress

**Routines:**
- `campeoes_sortear_grupos()` = 0x585290 (european_cup.cpp, thiscall, ret 0): the 32 clubs sorted by pot, the same-nation partners in opposite halves, the draw of the 8 groups pot by pot, the swap of same-nation clubs in a group, the coefficient, the 1,800,000 prize and its news, the 8 stages and the third qualifying round losers into the UEFA Cup;
- `campeoes_ordem_dos_potes()` = 0x586f60 (european_cup.cpp, cdecl): the pot comparison: the club's European seed (+0x1df), and on a tie the higher reputation first;
- `campeoes_posicoes_previstas()` = 0x586900 (european_cup.cpp, thiscall, ret 0): the entrants sorted by strength and each one's predicted position: the knockout from the final back, the second and first group phases and the qualifying rounds;
- `campeoes_sortear_segunda_fase()` = 0x585ab0 (european_cup.cpp, thiscall, ret 4): the 16 qualifiers sorted, the same-group pairs, each one's group choice (shuffled order, fewest teams, no same group nor nation), the redraw up to 20 times, the coefficient, the 1,200,000 prize and its news, stages 8 to 11 and the third places into the UEFA Cup;
- `campeoes_ordem_da_segunda_fase()` = 0x5871e0 (european_cup.cpp, cdecl): the qualifiers comparison: more same-nation clubs first, then the higher nation word +0x8e, then the group winner before the runner-up;
- `grupo_da_fase()` = 0x4b01d0 (comp_util.cpp, cdecl): the group (the containing stage) of a stage or round by id, by the fixed ranges; −1 without a group;
- `campeoes_rodadas_da_fase()` = 0x583c50 (european_cup.cpp, thiscall, ret 0x10): in the group stages, the 6-round schedule (phase 1 dates in September and October and phase 2 from November to March, by half) with fixture 0 of each round empty; the qualifying rounds and the knockout stay with the original;
- `liga_expectativas_da_temporada()` = 0x68aa80 (league.cpp, thiscall, ret 0): the season expectations of each club in the table, through the manager manager, with the news;
- `liga_rebaixar_clube()` = 0x6831a0 (league.cpp, thiscall, ret 0xc): the club goes down to the division (keeping the previous one), the relegation clauses and the human manager leaving when the division is not played;
- `competicao_todas_completas()` = 0x48ce10 (comp.cpp, thiscall, ret 0): in the top competition, whether it and every one down the chain are complete;
- `competicao_lista_de_clubes()` = 0x48e1c0 (comp.cpp, thiscall, ret 8): the vector of the distinct clubs of all stages (up to 200) and the count;
- `competicao_progresso_do_clube()` = 0x48cc70 (comp.cpp, thiscall, ret 0x10): the stage id, the status and the round the club reached, from the last stage back;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (11):** 10 caught. `cqh6_lista_com_repetidos` escaped: no phase of the career to 2002-06-01 has a repeated club in the list, so removing the repeat check changes nothing. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (8845ebc9).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 151bdf85, original 2aacab3a (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 271's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 272`

**Fields touched:**
- champions league +4 id, +0xc the stages, +0x14 table (6 bytes: +0 club), +0x30 current stage, +0x3e qualifying teams (word), +0x40 year, +0xa7 qualifying table (6 bytes: +0 club, +5 1 winner / 2 loser of the third round), +0xb1 (byte cleared without 32); value and pointer; reads and writes (campeoes_sortear_grupos)
- UEFA cup ([[0xadadfc] + 4 × [0x9cf6f0]]) +0x14 table (status 5), +0x36 count (word), +0x3e teams (word), +0xa7 table (+4/+5 cleared); value and pointer; writes (campeoes_sortear_grupos)
- club +0x53 nation, +0x80 reputation (word), +0x1db European competition (id), +0x1df European seed (char); value, id and pointer; reads (campeoes_sortear_grupos, campeoes_ordem_dos_potes), writes +0x1db (campeoes_sortear_grupos)
- champions league +8 statistics (Comp_Stats), +0x14 entrants (6 bytes: +0 club), +0x36 count (word), +0x3c qualifying rounds (word), +0xa3 qualifying round table (0x68 bytes, +0x1a ties); pointer and value; reads (campeoes_posicoes_previstas)
- competition statistics record +0x72 position (byte); value; writes through 0x4a2540 (campeoes_posicoes_previstas)
- champions league +4 id, +0xc the stages (0..7 phase 1, 8..11 phase 2), +0x30 current stage, +0x40 year, +0x50 ruling body index (char); value and pointer; reads and writes (campeoes_sortear_segunda_fase)
- phase 2 draw entry (7 bytes): +0 club, +4 same-nation count, +5 phase 1 group, +6 0 winner / 1 runner-up; value; writes (campeoes_sortear_segunda_fase), reads (campeoes_ordem_da_segunda_fase)
- UEFA cup +0x14 table (status 5), +0x36 count, +0x3e teams, +0xa7 table; club +0x53 nation, +0x1db European competition (id; −1 none); nation +0x8e (word) [inference: the nation's coefficient strength]; value, id and pointer; reads and writes (campeoes_sortear_segunda_fase), reads (campeoes_ordem_da_segunda_fase)
- champions league +8 statistics (Comp_Stats), +0x14 entrants (6 bytes: +0 club), +0x36 count (word), +0x3c qualifying rounds (word), +0xa3 qualifying round table (0x68 bytes, +0x1a ties); pointer and value; reads (campeoes_posicoes_previstas)
- competition statistics record +0x72 position (byte); value; writes through 0x4a2540 (campeoes_posicoes_previstas)
- champions league +0x40 year; value; reads (campeoes_rodadas_da_fase)
- round schedule (0x41 bytes per round): +0 day of year, +2 year − base, +4 code, +0x3d value; fixture 0 at +5 (+5 value, +9 a, +0xa b, +0xb c); value; writes through 0x68a160 and 0x68a1c0 (campeoes_rodadas_da_fase)
- league +0x1c the one above (index; < 0 at the top), +0x20 the one below (index into [0xadadfc]), +0x3e clubs, +0xb1 table (0x3b-byte rows, +0 club); value and pointer; reads (liga_expectativas_da_temporada, liga_cadeia_de_promocoes)
- club +0x57 competition, +0x5b previous competition; pointer; reads and writes (liga_rebaixar_clube)
- competition +0xc the stages, +0x1c the one above (index), +0x20 the one below (index into [0xadadfc]), +0x30 last stage, +0x3a id, +0x3e teams, +0x43 kind (2 league), +0xa3 rounds (0x68 bytes: +7 round, +0x1a ties), +0xa7 teams (6 bytes: +0 club, +5 status), +0xab current round (char), +0xad ties (0x35 bytes: +0/+2 the teams), +0xb1 table (0x3b bytes: +0 club, +0x37 status); value and pointer; reads (competicao_todas_completas, competicao_lista_de_clubes, competicao_progresso_do_clube)

**Of interest to:** CMRTE, GPF2
