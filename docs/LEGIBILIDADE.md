*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 4876 |
| chaves fixas cruas | 1821 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 241 |
| linhas | 228568 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 587 | 0 | 0 | 0 | 0 | 104 | 6424 |
| `motor/match_screens.rs` | 233 | 4 | 0 | 0 | 0 | 6 | 5878 |
| `motor/manager_manager.rs` | 209 | 2 | 0 | 0 | 0 | 0 | 5140 |
| `motor/match_eng.rs` | 206 | 7 | 0 | 0 | 0 | 13 | 11686 |
| `motor/match_man.rs` | 173 | 1 | 0 | 0 | 0 | 1 | 4498 |
| `motor/match_pl.rs` | 171 | 5 | 0 | 0 | 0 | 7 | 10183 |
| `motor/national_teams.rs` | 136 | 33 | 0 | 0 | 0 | 2 | 3510 |
| `motor/transfer_manager.rs` | 132 | 29 | 0 | 0 | 0 | 3 | 10492 |
| `motor/injury.rs` | 119 | 0 | 0 | 0 | 0 | 4 | 3041 |
| `motor/database.rs` | 100 | 86 | 0 | 0 | 0 | 8 | 9344 |
| `motor/comp/cup.rs` | 96 | 8 | 0 | 0 | 0 | 3 | 3519 |
| `motor/formation.rs` | 85 | 0 | 0 | 0 | 0 | 0 | 1089 |
| `motor/comp/fix_man.rs` | 82 | 31 | 0 | 0 | 0 | 4 | 2591 |
| `motor/comp/league.rs` | 82 | 20 | 0 | 0 | 0 | 6 | 4132 |
| `motor/media.rs` | 80 | 2 | 0 | 0 | 0 | 0 | 3444 |
| `motor/shortlist_manager.rs` | 78 | 31 | 0 | 0 | 0 | 2 | 4242 |
| `motor/tactics.rs` | 76 | 0 | 0 | 0 | 0 | 2 | 1444 |
| `motor/player_search.rs` | 72 | 0 | 0 | 0 | 0 | 0 | 901 |
| `motor/contract_manager.rs` | 68 | 16 | 0 | 0 | 0 | 4 | 4240 |
| `motor/training_manager.rs` | 63 | 0 | 0 | 0 | 0 | 1 | 1319 |
| `motor/area.rs` | 59 | 0 | 0 | 0 | 0 | 0 | 353 |
| `motor/comp/uefa_seeding.rs` | 59 | 7 | 0 | 0 | 0 | 0 | 997 |
| `motor/match_day.rs` | 56 | 9 | 0 | 0 | 0 | 1 | 1899 |
| `motor/scout_manager.rs` | 55 | 18 | 0 | 0 | 0 | 0 | 1225 |
| `motor/human_manager.rs` | 54 | 3 | 0 | 0 | 0 | 0 | 772 |
| `motor/comp/eurocomp/european_cup.rs` | 54 | 9 | 0 | 0 | 0 | 1 | 1524 |
| `motor/player_regen.rs` | 47 | 6 | 0 | 0 | 0 | 2 | 3365 |
| `motor/comp/friendly.rs` | 47 | 26 | 0 | 0 | 0 | 1 | 3308 |
| `motor/comp/leagues/fin_prm.rs` | 45 | 24 | 0 | 0 | 0 | 1 | 3103 |
| `motor/player_stats.rs` | 44 | 8 | 0 | 0 | 0 | 1 | 1501 |
| `motor/shared.rs` | 43 | 2 | 0 | 0 | 0 | 1 | 591 |
| `motor/comp/stadium.rs` | 42 | 19 | 0 | 0 | 0 | 2 | 1362 |
| `motor/comp/worldcup/world_cup.rs` | 42 | 14 | 0 | 0 | 0 | 0 | 663 |
| `motor/officials_manager.rs` | 40 | 0 | 0 | 0 | 0 | 2 | 872 |
| `motor/comp/comp.rs` | 39 | 9 | 0 | 0 | 0 | 0 | 755 |
| `motor/db_files.rs` | 37 | 0 | 0 | 0 | 0 | 0 | 1282 |
| `motor/comp/cups/eng_cc_cup.rs` | 34 | 12 | 0 | 0 | 0 | 1 | 620 |
| `motor/comp/cups/ita_cup.rs` | 34 | 19 | 0 | 0 | 0 | 1 | 1155 |
| `motor/zipdir.rs` | 33 | 0 | 0 | 0 | 0 | 0 | 604 |
| `motor/awol.rs` | 32 | 0 | 0 | 0 | 0 | 0 | 896 |
