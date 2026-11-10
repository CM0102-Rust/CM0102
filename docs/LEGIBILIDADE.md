*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 4982 |
| chaves fixas cruas | 1821 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 474 |
| linhas | 228506 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 587 | 0 | 0 | 0 | 0 | 104 | 6406 |
| `motor/match_screens.rs` | 233 | 4 | 0 | 0 | 0 | 19 | 5877 |
| `motor/match_eng.rs` | 215 | 7 | 0 | 0 | 0 | 31 | 11689 |
| `motor/manager_manager.rs` | 212 | 2 | 0 | 0 | 0 | 9 | 5140 |
| `motor/match_man.rs` | 177 | 1 | 0 | 0 | 0 | 4 | 4492 |
| `motor/match_pl.rs` | 174 | 5 | 0 | 0 | 0 | 13 | 10196 |
| `motor/national_teams.rs` | 136 | 33 | 0 | 0 | 0 | 3 | 3510 |
| `motor/transfer_manager.rs` | 135 | 29 | 0 | 0 | 0 | 19 | 10492 |
| `motor/injury.rs` | 129 | 0 | 0 | 0 | 0 | 8 | 3041 |
| `motor/database.rs` | 102 | 86 | 0 | 0 | 0 | 14 | 9345 |
| `motor/media.rs` | 99 | 2 | 0 | 0 | 0 | 3 | 3444 |
| `motor/comp/cup.rs` | 96 | 8 | 0 | 0 | 0 | 7 | 3521 |
| `motor/formation.rs` | 85 | 0 | 0 | 0 | 0 | 0 | 1089 |
| `motor/comp/league.rs` | 83 | 20 | 0 | 0 | 0 | 15 | 4138 |
| `motor/shortlist_manager.rs` | 82 | 31 | 0 | 0 | 0 | 3 | 4243 |
| `motor/comp/fix_man.rs` | 82 | 31 | 0 | 0 | 0 | 5 | 2590 |
| `motor/tactics.rs` | 76 | 0 | 0 | 0 | 0 | 2 | 1444 |
| `motor/contract_manager.rs` | 74 | 16 | 0 | 0 | 0 | 6 | 4241 |
| `motor/player_search.rs` | 72 | 0 | 0 | 0 | 0 | 0 | 901 |
| `motor/training_manager.rs` | 65 | 0 | 0 | 0 | 0 | 1 | 1319 |
| `motor/player_regen.rs` | 60 | 6 | 0 | 0 | 0 | 5 | 3365 |
| `motor/area.rs` | 59 | 0 | 0 | 0 | 0 | 1 | 353 |
| `motor/comp/uefa_seeding.rs` | 59 | 7 | 0 | 0 | 0 | 0 | 997 |
| `motor/match_day.rs` | 56 | 9 | 0 | 0 | 0 | 7 | 1899 |
| `motor/human_manager.rs` | 55 | 3 | 0 | 0 | 0 | 5 | 772 |
| `motor/scout_manager.rs` | 55 | 18 | 0 | 0 | 0 | 2 | 1225 |
| `motor/comp/leagues/fin_prm.rs` | 55 | 24 | 0 | 0 | 0 | 6 | 3103 |
| `motor/comp/eurocomp/european_cup.rs` | 54 | 9 | 0 | 0 | 0 | 1 | 1524 |
| `motor/comp/friendly.rs` | 47 | 26 | 0 | 0 | 0 | 3 | 3314 |
| `motor/player_stats.rs` | 44 | 8 | 0 | 0 | 0 | 3 | 1501 |
| `motor/shared.rs` | 43 | 2 | 0 | 0 | 0 | 3 | 591 |
| `motor/comp/stadium.rs` | 42 | 19 | 0 | 0 | 0 | 5 | 1366 |
| `motor/comp/worldcup/world_cup.rs` | 42 | 14 | 0 | 0 | 0 | 0 | 663 |
| `motor/officials_manager.rs` | 40 | 0 | 0 | 0 | 0 | 3 | 872 |
| `motor/comp/comp.rs` | 39 | 9 | 0 | 0 | 0 | 4 | 755 |
| `motor/awol.rs` | 37 | 0 | 0 | 0 | 0 | 0 | 896 |
| `motor/db_files.rs` | 37 | 0 | 0 | 0 | 0 | 1 | 1282 |
| `motor/comp/cups/eng_cc_cup.rs` | 34 | 12 | 0 | 0 | 0 | 1 | 621 |
| `motor/comp/cups/ita_cup.rs` | 34 | 19 | 0 | 0 | 0 | 1 | 1155 |
| `motor/zipdir.rs` | 33 | 0 | 0 | 0 | 0 | 1 | 604 |
