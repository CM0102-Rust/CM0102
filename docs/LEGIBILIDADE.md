*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 18567 |
| chaves fixas cruas | 1494 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| linhas | 189477 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | linhas |
|---|---:|---:|---:|---:|---:|---:|
| `motor/transfer_manager.rs` | 1485 | 16 | 0 | 0 | 0 | 11801 |
| `motor/manager_manager.rs` | 882 | 1 | 0 | 0 | 0 | 5550 |
| `motor.rs` | 818 | 0 | 0 | 0 | 0 | 12264 |
| `motor/comp/league.rs` | 731 | 10 | 0 | 0 | 0 | 4465 |
| `motor/contract_manager.rs` | 671 | 9 | 0 | 0 | 0 | 4773 |
| `motor/match_screens.rs` | 667 | 2 | 0 | 0 | 0 | 6375 |
| `motor/match_eng.rs` | 660 | 7 | 0 | 0 | 0 | 11368 |
| `motor/player_regen.rs` | 651 | 7 | 0 | 0 | 0 | 3723 |
| `motor/shortlist_manager.rs` | 586 | 16 | 0 | 0 | 0 | 4500 |
| `motor/database.rs` | 571 | 64 | 0 | 0 | 0 | 10114 |
| `motor/comp/cup.rs` | 568 | 4 | 0 | 0 | 0 | 3785 |
| `motor/match_pl.rs` | 542 | 5 | 0 | 0 | 0 | 9839 |
| `motor/media.rs` | 524 | 1 | 0 | 0 | 0 | 3706 |
| `motor/comp/friendly.rs` | 519 | 14 | 0 | 0 | 0 | 3809 |
| `motor/national_teams.rs` | 509 | 23 | 0 | 0 | 0 | 3939 |
| `motor/injury.rs` | 492 | 0 | 0 | 0 | 0 | 3447 |
| `motor/comp/leagues/fin_prm.rs` | 475 | 14 | 0 | 0 | 0 | 3265 |
| `motor/staff_records.rs` | 461 | 1 | 0 | 0 | 0 | 2589 |
| `motor/comp/fix_man.rs` | 284 | 16 | 0 | 0 | 0 | 2815 |
| `motor/match_man.rs` | 267 | 1 | 0 | 0 | 0 | 4527 |
| `motor/club_records.rs` | 234 | 3 | 0 | 0 | 0 | 1626 |
| `motor/player_stats.rs` | 233 | 4 | 0 | 0 | 0 | 1772 |
| `motor/comp/stadium.rs` | 224 | 10 | 0 | 0 | 0 | 1446 |
| `motor/comp/eurocomp/european_cup.rs` | 190 | 5 | 0 | 0 | 0 | 1705 |
| `motor/match_day.rs` | 183 | 8 | 0 | 0 | 0 | 2046 |
| `motor/player_search.rs` | 174 | 0 | 0 | 0 | 0 | 1073 |
| `motor/scout_manager.rs` | 160 | 9 | 0 | 0 | 0 | 1443 |
| `motor/comp/leagues/bra_reg_rio.rs` | 149 | 13 | 0 | 0 | 0 | 950 |
| `motor/training_manager.rs` | 140 | 0 | 0 | 0 | 0 | 1542 |
| `motor/officials_manager.rs` | 131 | 0 | 0 | 0 | 0 | 990 |
| `motor/comp/worldcup/world_cup.rs` | 130 | 7 | 0 | 0 | 0 | 771 |
| `motor/comp/leagues/usa_mls.rs` | 126 | 1 | 0 | 0 | 0 | 928 |
| `motor/discipline.rs` | 122 | 1 | 0 | 0 | 0 | 1374 |
| `motor/comp/uefa_seeding.rs` | 122 | 5 | 0 | 0 | 0 | 1127 |
| `motor/comp/leagues/ita_ser_a.rs` | 120 | 8 | 0 | 0 | 0 | 1967 |
| `motor/comp/leagues/bra_nat_third.rs` | 118 | 0 | 0 | 0 | 0 | 689 |
| `motor/tactics.rs` | 117 | 0 | 0 | 0 | 0 | 1475 |
| `motor/awol.rs` | 105 | 0 | 0 | 0 | 0 | 982 |
| `motor/shared.rs` | 104 | 1 | 0 | 0 | 0 | 977 |
| `motor/comp/cups/ita_cup.rs` | 104 | 10 | 0 | 0 | 0 | 1163 |
