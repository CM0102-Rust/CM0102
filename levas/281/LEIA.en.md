# Batch 281 — the league season end: the promotion and relegation chain; a competition's prize money and TV rights news; the calendar of more club cups: the Copa dos Campeões, the Copa Merconorte and the European and Asian Super Cups; the calendar of the Asian club cups and the English League Trophy: the Asian Club Championship and Cup Winners' Cup and the LDV Vans Trophy; the national-team tournaments' calendar: the Copa América, the African Nations Cup and the Confederations Cup; the calendar of the intercontinental cups and the Intertoto: the Intertoto Cup, the Intercontinental Cup, the Inter-American Cup and the CONCACAF Champions' Cup; the UEFA Cup and the Intertoto Cup: the UEFA Cup rounds, the Intertoto entrants and its club list; the super cups' clubs: the European Super Cup, the Asian Super Cup and the Serie C1 Super Cup; the Oceanian and Asian club cups: the Oceania Club Championship calendar and the Asian Club Championship and Cup Winners' Cup teams

**Routines:**
- `cadeia_de_acessos()` = 0x689c20 (league.cpp, thiscall, ret 4): at the season end, only the top league of the pyramid (no league above) closes its table and walks down the chain of divisions: for each pair (upper and lower), closes the lower one and swaps the promoted and relegated clubs (0x689c80, with contract clauses, pairwise or as a block, without checking counts);
- `noticia_do_premio()` = 0x48e3e0 (comp.cpp, thiscall, ret 0x1c): the prize money news to the club's human manager, only when the prize exceeds 50 × the club reputation: the competition, the amount, the position, the round, the kind and, with the fixture, the opponent and whether the club won;
- `noticia_da_tv()` = 0x48ea40 (comp.cpp, thiscall, ret 8): the TV rights news to the club's human manager: the club, the competition, the amount and the year;
- `cqp10_copa_dos_campeoes_tabela()` = 0x425c10 (bra_champ_cup.cpp, thiscall, ret 0x10): rule: the Copa dos Campeões has a preliminary stage (stage −1, id 0xe6) of three rounds on Mondays [inference: weekday 1], 19 and 26 June and 3 July; then two groups (stages 0 and 1, id 0x3fd + stage) of three rounds, 9, 16 and 20 July; the knockout (stage 2 on): semi-final drawn on 21/7 and played on 23/7, return 2 days later, and final drawn on 26/7, match on 27/7, return 3 days later; category 6; two-leg tie-break 1; final prize 800 thousand to the winner and 400 thousand to the runner-up;
- `cqp10_merconorte_tabela()` = 0x4c1960 (con_merc_cup.cpp, thiscall, ret 0x10): rule: the Copa Merconorte starts with groups (stages below 3, id 0x3e9 + stage: 4 groups [inference]) of six rounds on Tuesdays every 15 days, from 9/9 to 18/11; the knockout (stage 3): semi-final drawn on 19/11 and played on 25/11 over two legs (7 days), final drawn on 3/12 and a single match on 9/12; category 8; tie-break 3;
- `cqp10_supercopa_europeia_tabela()` = 0x57a3c0 (eur_super_cup.cpp, thiscall, ret 0x10): rule: the European Super Cup is a single match on 24 August (drawn on 4 July) on neutral ground (venue 0x8000: Monaco [inference]), category 8, tie-break 0x83, no prizes;
- `cqp10_supercopa_asiatica_tabela()` = 0x410910 (asia_super_cup.cpp, thiscall, ret 0x10): rule: the Asian Super Cup is two-legged, the first match on 4 August (drawn on 4 July) and the return 14 days later, category 6, two-leg tie-break 0x83, no prizes;
- `cqp11_asia_clubes_tabela()` = 0x40ae90 (asia_club_champ.cpp, thiscall, ret 0x10): rule: the Asian Club Championship has, in each zone (stages −1 and 0, ids 0x480 and 0x481), two two-legged qualifying rounds (1st drawn on 1/8 and played on 23/8, 2nd on 14/9 and 20/9; 21 days between legs in the 1st, 7 in the 2nd); the final groups (stages 1 and 2, ids 0x482 and 0x483) of three rounds on 21, 23 and 25 March of the following year with fixed pairings (matches 0 and 1 of each round); the final stage (stage 3) in May: semi-final on 24/5, third place and final on 26/5, single matches (tie-break 0x383 and 0x83); category 4 in the qualifiers and 6 after;
- `cqp11_asia_recopa_tabela()` = 0x40cf90 (asia_cup_winner.cpp, thiscall, ret 0x10): rule: the Asian Cup Winners' Cup has, in each zone (stages −1 and 0, ids 0x480 and 0x481), three two-legged qualifying rounds: 1st on 23/8, 2nd on 20/9, quarter-finals on 24 March of the following year (21 days between legs in the 1st, 7 after); the final stage (stage 1) in May: semi-final on 24/5, third place and final on 26/5, single matches; category 4 in the qualifiers and 6 after;
- `cqp11_trofeu_tabela()` = 0x56ade0 (eng_auto_cup.cpp, thiscall, ret 0x10): rule: the LDV Vans Trophy has two sections (stages −1 and 0, North and South [inference], ids 0x41d and 0x41e) of five single-match rounds: 1st on 10/12 (24 teams), 2nd on 14/1 (16 teams, 4 new), quarter-finals on 4/2, semi-final on 18/2 and the section final over two legs from 4/3 (7 days); the final (stage 1) drawn on 12/3 and played on 20/4 of the following year with venue 4 (neutral ground [inference]); category 4 in the sections and 6 in the finals; no per-round prizes;
- `cqp12_copa_america_tabela()` = 0x5e0900 (goldcup.cpp, thiscall, ret 0x10): rule: the Copa América has three groups (stages −1, 0 and 1, ids 0x41f to 0x421) of three rounds on 6, 10 and 13 July, each round with the group's two fixed matches (pairings and venue); the knockout (stage 2) in single matches: quarter-finals on 16/7 with fixed pairings (the two best third-placed teams [inference]), semi-final on 19/7, third place on 22/7 and final on 23/7 (venue 4, neutral ground [inference]); category 10; tie-break 0x103 up to the semi-final and 3 in the final; no prizes;
- `cqp12_africana_tabela()` = 0x4012e0 (african_nations.cpp, thiscall, ret 0x10): rule: the African Nations Cup has four groups (stages −1 to 2, ids 0x3e8 to 0x3eb) of three rounds on 8, 12 and 16 February, with each group's fixed matches; the knockout (stage 3) in single matches: quarter-finals on 21/2 with fixed pairings, semi-final on 25/2, third place on 27/2 and final on 28/2 (venue 4); category 10; tie-break 0x303 up to the semi-final and 0x203 in the final; no prizes;
- `cqp12_confed_tabela()` = 0x58b560 (fifa_confed.cpp, thiscall, ret 0x10): rule: the Confederations Cup has two groups (stages −1 and 0, ids 0x3e8 and 0x3e9) of three rounds on 29 July, 1 and 4 August, with fixed matches; the knockout (stage 1) in single matches: semi-final on 6/8, third place on 8/8 and final on 9/8 (venue 4); category 10; tie-break 0x383 in the semi-final and 0x83 after;
- `cqp13_intertoto_tabela()` = 0x633300 (intertoto_cup.cpp, thiscall, ret 0x10): rule: the Intertoto Cup has 5 two-legged rounds: 1st drawn on 25/6 and played on 26/6 (40 teams, return 4 days later), 2nd on 3/7 (32 teams: 20 through and 12 new), 3rd on 17/7 (24: 16 and 8 new), semi-final on 28/7 and final on 10/8 (6 teams: the 3 winners go to the UEFA Cup [inference]); category 7 in the first three and 8 after; no prizes;
- `cqp13_intercontinental_tabela()` = 0x92b770 (world_club_cup.cpp, thiscall, ret 0x10): rule: the Intercontinental Cup (the European against the South American champion [inference]) is a single match on 1 December, drawn on 10 October, venue 4 (Tokyo [inference]), category 6, tie-break 3;
- `cqp13_interamericana_tabela()` = 0x632340 (inter_amer_cup.cpp, thiscall, ret 0x10): rule: the Inter-American Cup is two-legged from 26 January of the following year (7 days), drawn on 10 October, category 8, two-leg tie-break 3;
- `cqp13_concacaf_tabela()` = 0x4c0f00 (con_champ.cpp, thiscall, ret 0x10): rule: the CONCACAF Champions' Cup has 8 teams in single matches: quarter-finals drawn on 1/8 and played on 28/8, semi-final on 1/9 and final on 4/9; category 8, tie-break 3; no per-round prize (−1);
- `cqq10_uefa_tabela()` = 0x9025c0 (uefa_cup.cpp, thiscall, ret 0x10): rule: the UEFA Cup has, only when there are preliminary clubs (+0xb2, those with coefficient 10 or more [inference]), a preliminary round drawn on 28/6 and played on 6/7 (b teams); then: 1st qualifying round drawn on 28/7 and played on 10/8 (82 teams, 0x52 − b/2 new), 2nd on 14/9 (96 teams, 55 new), 1st round on 19/10 (48 teams), 2nd on 23/11 (32 teams, 8 new), 3rd on 1/3 of the following year, round of 16 on 15/3, quarter-finals on 5/4 and the single-match final on 16/5 (venue 4, prize of 1.5 million to the winner); two legs 14 days apart (7 in the 3rd round and the round of 16); category 7 in qualifying and 8 after;
- `cqq10_intertoto_inscritos()` = 0x6336c0 (intertoto_cup.cpp, thiscall, ret 0): rule: the Intertoto Cup takes the 60 clubs the European rules put in it (European cup +0x1db = the Intertoto), in club table order; each with class = its coefficient (+0x1df), and the list is sorted by class (the seeds); with fewer than 60, the error box;
- `cqq10_intertoto_lista()` = 0x633f70 (intertoto_cup.cpp, thiscall, ret 8): rule: the Intertoto club list (for the caller): all entrants, or only those still in (status 0 or 1) when asked;
- `cqq11_europeia_clubes()` = 0x57a500 (eur_super_cup.cpp, thiscall, ret 0): rule: the European Super Cup is between the Champions League winner and the Cup Winners' Cup winner in the 1999 edition (the last Cup Winners' Cup) and, in the others, the UEFA Cup winner; without either, the error box;
- `cqq11_asiatica_clubes()` = 0x410a50 (asia_super_cup.cpp, thiscall, ret 0): rule: the Asian Super Cup is between the Asian Club Championship winner and the Asian Cup Winners' Cup winner; if the same club won both, the Club Championship runner-up plays;
- `cqq11_c1_vagas()` = 0x63c9d0 (ita_c1_super.cpp, thiscall, ret 0): rule: the Serie C1 Super Cup starts with both places empty; the clubs enter at the end of the season [inference: the champions of the two C1 groups];
- `cqq12_oceania_tabela()` = 0x797230 (oceania_club_champ.cpp, thiscall, ret 0x10): rule: the Oceania Club Championship has two groups (stages −1 and 0, ids 0x41f and 0x420) of three rounds on 10, 12 and 14 January of the year after the cup year, with each group's fixed matches; the knockout (stage 1) in single matches: semi-final on 17/1, third place and final on 19/1 (venue 4 in the final); category 8; tie-break 0x283 in the semi-final and 0x83 after; no prizes;
- `cqq12_asia_clubes_times()` = 0x40b620 (asia_club_champ.cpp, thiscall, ret 0): rule: the Asian Club Championship has 16 teams, the qualified clubs (+0xb2) in the order they were picked, all with class and status 0;
- `cqq12_asia_recopa_times()` = 0x40d4d0 (asia_cup_winner.cpp, thiscall, ret 0): rule: the Asian Cup Winners' Cup has 16 teams, the qualified clubs (+0xb2) in the order they were picked, all with class and status 0;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (24):** 24 caught.
- **Real game comparing to 2002-01-01:** deferred to the audit's accumulated comparison (decision of 5 Nov 2026).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 1dedd9dc, original 404adc50 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 280's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 281`

**Fields touched:**
- league +0x1c the league above (index; negative at the top); +0x20 the league below (index in [0xadadfc]; negative at the bottom); reads (cadeia_de_acessos)
- competition +4 the record (+0 id); +0x40 year; club +0x80 reputation; fixture +0x1c/+0x20 the clubs, +0x3d/+0x3e [inference: penalty winner], +0x43/+0x44 and +0x4b/+0x4c the scores; reads (news)
- cup +0x40 cup year; value (signed word); reads (all four)
- cup round (0x68 bytes) +0..+0x22 and +0x5c..+0x67; value; writes (knockout and super cups)
- group round (0x41 bytes) +0 day of year, +2 relative year, +4 code, +5..+0xb empty match 0, +0x3d value; value; writes (groups)
- cup +0x40 cup year; value (signed word); reads (all three)
- cup round (0x68 bytes) +0..+0x22 (with the fixed pairings at +0x23 + 7 × slot) and +0x5c..+0x67; value; writes (knockout)
- group round (0x41 bytes) +0 day of year, +2 relative year, +4 code, +5 + 7 × match the matches, +0x3d value; value; writes (Asian groups)
- tournament +0x40 tournament year; value (signed word); reads (all three)
- cup round (0x68 bytes) +0..+0x22 (with the fixed pairings at +0x23 + 7 × slot) and +0x5c..+0x67; value; writes (knockout)
- group round (0x41 bytes) +0 day of year, +2 relative year, +4 code, +5 + 7 × match the matches, +0x3d value; value; writes (groups)
- cup +0x40 cup year; value (signed word); reads (all four)
- cup round (0x68 bytes) +0..+0x22, +0x23..+0x29 (pairing day 0, CONCACAF) and +0x5c..+0x67; value; writes
- cup +0x40 year, +0xb2 preliminary clubs (signed byte); value; reads (UEFA table)
- club +0x1db European cup (id), +0x1df European coefficient (byte) [inference]; id/value; reads (entrants)
- cup +0x3e entrants, +0xa7 list (6 bytes: club, +4 class, +5 status); value/pointer; writes (entrants) and reads (list)
- [0x9cf6f8] the Intertoto; id; reads
- cup +0x3e entrants, +0xa7 list (6 bytes: club, +4 class, +5 status); value/pointer; writes (all three)
- cup +0x40 year; value; reads (European Super Cup)
- [0x9cf6e8] Champions League, [0x9cf6ec] Cup Winners' Cup, [0x9cf6f0] UEFA Cup, [0x9cf64c]/[0x9cf650] the Asian cups; [0xae23d0] database competitions; id; reads
- cup +0x40 year; value (signed word); reads (Oceania)
- group round (0x41 bytes) and cup round (0x68 bytes); value; writes (Oceania)
- cup +0xb2 qualified clubs (club pointers); pointer; reads (Asia)
- cup +0x3e teams, +0xa7 table (6 bytes: club, +4 class, +5 status); value/pointer; writes (Asia)

**Of interest to:** CMRTE
