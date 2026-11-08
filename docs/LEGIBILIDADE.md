*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 5749 |
| chaves fixas cruas | 1821 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 1337 |
| linhas | 207932 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 614 | 0 | 0 | 0 | 0 | 105 | 10669 |
| `motor/match_screens.rs` | 240 | 4 | 0 | 0 | 0 | 76 | 6202 |
| `motor/manager_manager.rs` | 231 | 2 | 0 | 0 | 0 | 23 | 5367 |
| `motor/match_eng.rs` | 199 | 7 | 0 | 0 | 0 | 99 | 11902 |
| `motor/transfer_manager.rs` | 161 | 29 | 0 | 0 | 0 | 103 | 11073 |
| `motor/match_pl.rs` | 152 | 5 | 0 | 0 | 0 | 35 | 10343 |
| `motor/national_teams.rs` | 143 | 33 | 0 | 0 | 0 | 27 | 3690 |
| `motor/injury.rs` | 133 | 0 | 0 | 0 | 0 | 22 | 3143 |
| `motor/contract_manager.rs` | 129 | 16 | 0 | 0 | 0 | 32 | 4427 |
| `motor/media.rs` | 118 | 2 | 0 | 0 | 0 | 22 | 3564 |
| `motor/shortlist_manager.rs` | 105 | 31 | 0 | 0 | 0 | 25 | 4413 |
| `motor/match_man.rs` | 101 | 1 | 0 | 0 | 0 | 25 | 4628 |
| `motor/database.rs` | 98 | 86 | 0 | 0 | 0 | 47 | 9456 |
| `motor/comp/cup.rs` | 96 | 8 | 0 | 0 | 0 | 16 | 3590 |
| `motor/comp/fix_man.rs` | 91 | 31 | 0 | 0 | 0 | 22 | 2730 |
| `motor/comp/leagues/fin_prm.rs` | 85 | 24 | 0 | 0 | 0 | 9 | 3140 |
| `motor/comp/league.rs` | 82 | 20 | 0 | 0 | 0 | 32 | 4234 |
| `motor/player_search.rs` | 76 | 0 | 0 | 0 | 0 | 5 | 935 |
| `motor/formation.rs` | 72 | 0 | 0 | 0 | 0 | 2 | 1115 |
| `motor/player_regen.rs` | 67 | 6 | 0 | 0 | 0 | 18 | 3477 |
| `motor/training_manager.rs` | 65 | 0 | 0 | 0 | 0 | 1 | 1346 |
| `motor/comp/worldcup/world_cup.rs` | 62 | 14 | 0 | 0 | 0 | 3 | 689 |
| `motor/area.rs` | 59 | 0 | 0 | 0 | 0 | 4 | 361 |
| `motor/comp/uefa_seeding.rs` | 59 | 7 | 0 | 0 | 0 | 6 | 1038 |
| `motor/scout_manager.rs` | 56 | 18 | 0 | 0 | 0 | 13 | 1295 |
| `motor/match_day.rs` | 55 | 9 | 0 | 0 | 0 | 17 | 2030 |
| `motor/comp/eurocomp/european_cup.rs` | 55 | 9 | 0 | 0 | 0 | 8 | 1570 |
| `motor/comp/friendly.rs` | 54 | 26 | 0 | 0 | 0 | 34 | 3461 |
| `motor/comp/stadium.rs` | 50 | 19 | 0 | 0 | 0 | 16 | 1386 |
| `motor/human_manager.rs` | 49 | 3 | 0 | 0 | 0 | 7 | 792 |
| `motor/player_stats.rs` | 47 | 8 | 0 | 0 | 0 | 10 | 1534 |
| `motor/compare/comp/leagues/bra_nat_third.rs` | 47 | 0 | 0 | 0 | 0 | 2 | 248 |
| `motor/compare/comp/worldcup/world_cup.rs` | 41 | 0 | 0 | 0 | 0 | 1 | 202 |
| `motor/comp/comp.rs` | 39 | 9 | 0 | 0 | 0 | 2 | 762 |
| `motor/staff_records.rs` | 38 | 2 | 0 | 0 | 0 | 17 | 2429 |
| `motor/compare/comp/intercomp/goldcup.rs` | 38 | 0 | 0 | 0 | 0 | 1 | 155 |
| `motor/awol.rs` | 37 | 0 | 0 | 0 | 0 | 3 | 936 |
| `motor/db_files.rs` | 37 | 0 | 0 | 0 | 0 | 1 | 1357 |
| `motor/compare/comp/eurocomp/european_cup.rs` | 37 | 0 | 0 | 0 | 0 | 2 | 358 |
| `motor/compare/comp/eurocomp/oceania_club_champ.rs` | 35 | 0 | 0 | 0 | 0 | 0 | 147 |
