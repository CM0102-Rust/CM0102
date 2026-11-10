*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 5090 |
| chaves fixas cruas | 1821 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 855 |
| linhas | 228316 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 587 | 0 | 0 | 0 | 0 | 104 | 6406 |
| `motor/match_screens.rs` | 236 | 4 | 0 | 0 | 0 | 43 | 5874 |
| `motor/match_eng.rs` | 227 | 7 | 0 | 0 | 0 | 75 | 11690 |
| `motor/manager_manager.rs` | 214 | 2 | 0 | 0 | 0 | 12 | 5142 |
| `motor/match_pl.rs` | 203 | 5 | 0 | 0 | 0 | 24 | 10196 |
| `motor/match_man.rs` | 181 | 1 | 0 | 0 | 0 | 10 | 4481 |
| `motor/national_teams.rs` | 142 | 33 | 0 | 0 | 0 | 15 | 3512 |
| `motor/transfer_manager.rs` | 137 | 29 | 0 | 0 | 0 | 40 | 10395 |
| `motor/injury.rs` | 132 | 0 | 0 | 0 | 0 | 15 | 3044 |
| `motor/media.rs` | 105 | 2 | 0 | 0 | 0 | 6 | 3444 |
| `motor/database.rs` | 102 | 86 | 0 | 0 | 0 | 30 | 9355 |
| `motor/comp/cup.rs` | 96 | 8 | 0 | 0 | 0 | 9 | 3505 |
| `motor/comp/fix_man.rs` | 88 | 31 | 0 | 0 | 0 | 12 | 2591 |
| `motor/shortlist_manager.rs` | 86 | 31 | 0 | 0 | 0 | 11 | 4244 |
| `motor/formation.rs` | 85 | 0 | 0 | 0 | 0 | 2 | 1089 |
| `motor/comp/league.rs` | 83 | 20 | 0 | 0 | 0 | 22 | 4132 |
| `motor/contract_manager.rs` | 77 | 16 | 0 | 0 | 0 | 16 | 4246 |
| `motor/tactics.rs` | 76 | 0 | 0 | 0 | 0 | 3 | 1446 |
| `motor/player_search.rs` | 73 | 0 | 0 | 0 | 0 | 2 | 901 |
| `motor/training_manager.rs` | 65 | 0 | 0 | 0 | 0 | 1 | 1321 |
| `motor/player_regen.rs` | 61 | 6 | 0 | 0 | 0 | 6 | 3367 |
| `motor/area.rs` | 59 | 0 | 0 | 0 | 0 | 4 | 353 |
| `motor/comp/uefa_seeding.rs` | 59 | 7 | 0 | 0 | 0 | 2 | 997 |
| `motor/match_day.rs` | 58 | 9 | 0 | 0 | 0 | 10 | 1899 |
| `motor/comp/leagues/fin_prm.rs` | 58 | 24 | 0 | 0 | 0 | 10 | 3104 |
| `motor/human_manager.rs` | 55 | 3 | 0 | 0 | 0 | 6 | 772 |
| `motor/scout_manager.rs` | 55 | 18 | 0 | 0 | 0 | 4 | 1224 |
| `motor/comp/eurocomp/european_cup.rs` | 55 | 9 | 0 | 0 | 0 | 2 | 1514 |
| `motor/comp/friendly.rs` | 50 | 26 | 0 | 0 | 0 | 20 | 3296 |
| `motor/comp/stadium.rs` | 50 | 19 | 0 | 0 | 0 | 15 | 1368 |
| `motor/player_stats.rs` | 46 | 8 | 0 | 0 | 0 | 8 | 1502 |
| `motor/shared.rs` | 44 | 2 | 0 | 0 | 0 | 3 | 591 |
| `motor/comp/worldcup/world_cup.rs` | 42 | 14 | 0 | 0 | 0 | 0 | 665 |
| `motor/officials_manager.rs` | 40 | 0 | 0 | 0 | 0 | 4 | 872 |
| `motor/comp/comp.rs` | 40 | 9 | 0 | 0 | 0 | 4 | 756 |
| `motor/awol.rs` | 37 | 0 | 0 | 0 | 0 | 0 | 896 |
| `motor/db_files.rs` | 37 | 0 | 0 | 0 | 0 | 1 | 1346 |
| `motor/comp/cups/eng_cc_cup.rs` | 34 | 12 | 0 | 0 | 0 | 1 | 620 |
| `motor/comp/cups/ita_cup.rs` | 34 | 19 | 0 | 0 | 0 | 2 | 1155 |
| `motor/zipdir.rs` | 33 | 0 | 0 | 0 | 0 | 2 | 604 |
