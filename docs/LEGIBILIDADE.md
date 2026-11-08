*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 4949 |
| chaves fixas cruas | 1821 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 1198 |
| linhas | 210864 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 598 | 0 | 0 | 0 | 0 | 105 | 10670 |
| `motor/match_screens.rs` | 240 | 4 | 0 | 0 | 0 | 62 | 6040 |
| `motor/manager_manager.rs` | 231 | 2 | 0 | 0 | 0 | 18 | 5240 |
| `motor/match_eng.rs` | 173 | 7 | 0 | 0 | 0 | 89 | 11743 |
| `motor/transfer_manager.rs` | 144 | 29 | 0 | 0 | 0 | 81 | 10707 |
| `motor/national_teams.rs` | 143 | 33 | 0 | 0 | 0 | 25 | 3581 |
| `motor/match_pl.rs` | 141 | 5 | 0 | 0 | 0 | 28 | 10229 |
| `motor/injury.rs` | 132 | 0 | 0 | 0 | 0 | 19 | 3111 |
| `motor/media.rs` | 118 | 2 | 0 | 0 | 0 | 17 | 3477 |
| `motor/contract_manager.rs` | 113 | 16 | 0 | 0 | 0 | 30 | 4341 |
| `motor/shortlist_manager.rs` | 105 | 31 | 0 | 0 | 0 | 20 | 4339 |
| `motor/match_man.rs` | 100 | 1 | 0 | 0 | 0 | 22 | 4570 |
| `motor/comp/cup.rs` | 96 | 8 | 0 | 0 | 0 | 15 | 3554 |
| `motor/comp/fix_man.rs` | 91 | 31 | 0 | 0 | 0 | 21 | 2675 |
| `motor/comp/leagues/fin_prm.rs` | 85 | 24 | 0 | 0 | 0 | 9 | 3112 |
| `motor/comp/league.rs` | 81 | 20 | 0 | 0 | 0 | 31 | 4172 |
| `motor/database.rs` | 77 | 86 | 0 | 0 | 0 | 42 | 9395 |
| `motor/player_search.rs` | 73 | 0 | 0 | 0 | 0 | 5 | 913 |
| `motor/formation.rs` | 72 | 0 | 0 | 0 | 0 | 2 | 1100 |
| `motor/player_regen.rs` | 67 | 6 | 0 | 0 | 0 | 15 | 3419 |
| `motor/training_manager.rs` | 65 | 0 | 0 | 0 | 0 | 1 | 1320 |
| `motor/comp/worldcup/world_cup.rs` | 62 | 14 | 0 | 0 | 0 | 3 | 672 |
| `motor/area.rs` | 59 | 0 | 0 | 0 | 0 | 4 | 354 |
| `motor/comp/uefa_seeding.rs` | 59 | 7 | 0 | 0 | 0 | 5 | 1014 |
| `motor/scout_manager.rs` | 56 | 18 | 0 | 0 | 0 | 10 | 1251 |
| `motor/match_day.rs` | 55 | 9 | 0 | 0 | 0 | 13 | 1945 |
| `motor/comp/eurocomp/european_cup.rs` | 55 | 9 | 0 | 0 | 0 | 8 | 1543 |
| `motor/comp/friendly.rs` | 54 | 26 | 0 | 0 | 0 | 30 | 3406 |
| `motor/comp/stadium.rs` | 50 | 19 | 0 | 0 | 0 | 16 | 1376 |
| `motor/human_manager.rs` | 49 | 3 | 0 | 0 | 0 | 7 | 782 |
| `motor/player_stats.rs` | 47 | 8 | 0 | 0 | 0 | 10 | 1516 |
| `motor/comp/comp.rs` | 39 | 9 | 0 | 0 | 0 | 2 | 751 |
| `motor/staff_records.rs` | 38 | 2 | 0 | 0 | 0 | 17 | 2405 |
| `motor/awol.rs` | 37 | 0 | 0 | 0 | 0 | 2 | 915 |
| `motor/db_files.rs` | 37 | 0 | 0 | 0 | 0 | 1 | 1353 |
| `motor/comp/cups/eng_cc_cup.rs` | 34 | 12 | 0 | 0 | 0 | 3 | 628 |
| `motor/comp/cups/ita_cup.rs` | 34 | 19 | 0 | 0 | 0 | 3 | 1170 |
| `motor/match_events.rs` | 33 | 0 | 0 | 0 | 0 | 6 | 1930 |
| `motor/zipdir.rs` | 33 | 0 | 0 | 0 | 0 | 3 | 631 |
| `motor/comp/fifa_rankings.rs` | 32 | 0 | 0 | 0 | 0 | 4 | 393 |
