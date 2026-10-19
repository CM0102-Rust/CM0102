# Batch 173 — closing the engine (7): the squad rule and the player scores

**Routines:**
- `cumprir_regra_do_elenco()` = 0x6e8910 (match_man.cpp, cdecl): applies the squad rule (foreigners, non-EU or age): swaps restricted players for the candidates with the smallest score loss;
- `nota_fora_da_partida()` = 0x6e6340 (match_man.cpp, cdecl): the player's score for a position outside a match (transfers and scouting): position, form, perceived ability, contract, age, scout knowledge;
- `nota_para_a_vaga()` = 0x6e2f20 (match_man.cpp, cdecl): the player's score for a lineup slot: like the off-match score, plus the second position, in-match stamina, tactics, veterans and holidays;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (12):** 8 caught. `r_banco` escaped: the bench phase of the squad rule (0x6e8910) had no restricted bench players in the English league (goes to the broad career). `r_idade` escaped: the age rule (type 3) never had a player exactly at the limit. `n_veterano` escaped: the contract adjustment (flag 0x2000000) never met a veteran (over 34) with no games. `v_ferias` escaped: the holiday bonus of 0x6e2f20 only runs with a human manager's holiday list. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 172's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 173`
