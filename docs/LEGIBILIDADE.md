*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 18441 |
| chaves fixas cruas | 1494 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| linhas | 186998 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | linhas |
|---|---:|---:|---:|---:|---:|---:|
| `motor/transfer_manager.rs` | 1476 | 16 | 0 | 0 | 0 | 11451 |
| `motor/manager_manager.rs` | 879 | 1 | 0 | 0 | 0 | 5388 |
| `motor.rs` | 818 | 0 | 0 | 0 | 0 | 12264 |
| `motor/comp/league.rs` | 731 | 10 | 0 | 0 | 0 | 4445 |
| `motor/contract_manager.rs` | 667 | 9 | 0 | 0 | 0 | 4710 |
| `motor/match_screens.rs` | 667 | 2 | 0 | 0 | 0 | 6375 |
| `motor/match_eng.rs` | 660 | 7 | 0 | 0 | 0 | 11368 |
| `motor/player_regen.rs` | 651 | 7 | 0 | 0 | 0 | 3676 |
| `motor/shortlist_manager.rs` | 584 | 16 | 0 | 0 | 0 | 4431 |
| `motor/database.rs` | 571 | 64 | 0 | 0 | 0 | 10036 |
| `motor/comp/cup.rs` | 567 | 4 | 0 | 0 | 0 | 3760 |
| `motor/match_pl.rs` | 542 | 5 | 0 | 0 | 0 | 9839 |
| `motor/media.rs` | 520 | 1 | 0 | 0 | 0 | 3634 |
| `motor/comp/friendly.rs` | 519 | 14 | 0 | 0 | 0 | 3773 |
| `motor/national_teams.rs` | 508 | 23 | 0 | 0 | 0 | 3920 |
| `motor/injury.rs` | 492 | 0 | 0 | 0 | 0 | 3398 |
| `motor/comp/leagues/fin_prm.rs` | 475 | 14 | 0 | 0 | 0 | 3203 |
| `motor/staff_records.rs` | 461 | 1 | 0 | 0 | 0 | 2530 |
| `motor/comp/fix_man.rs` | 284 | 16 | 0 | 0 | 0 | 2787 |
| `motor/match_man.rs` | 267 | 1 | 0 | 0 | 0 | 4527 |
| `motor/club_records.rs` | 234 | 3 | 0 | 0 | 0 | 1595 |
| `motor/player_stats.rs` | 233 | 4 | 0 | 0 | 0 | 1757 |
| `motor/comp/stadium.rs` | 224 | 10 | 0 | 0 | 0 | 1431 |
| `motor/comp/eurocomp/european_cup.rs` | 189 | 5 | 0 | 0 | 0 | 1691 |
| `motor/match_day.rs` | 183 | 8 | 0 | 0 | 0 | 2046 |
| `motor/player_search.rs` | 174 | 0 | 0 | 0 | 0 | 1060 |
| `motor/scout_manager.rs` | 160 | 9 | 0 | 0 | 0 | 1428 |
| `motor/comp/leagues/bra_reg_rio.rs` | 146 | 13 | 0 | 0 | 0 | 934 |
| `motor/training_manager.rs` | 140 | 0 | 0 | 0 | 0 | 1499 |
| `motor/officials_manager.rs` | 131 | 0 | 0 | 0 | 0 | 971 |
| `motor/comp/worldcup/world_cup.rs` | 130 | 7 | 0 | 0 | 0 | 771 |
| `motor/comp/leagues/usa_mls.rs` | 125 | 1 | 0 | 0 | 0 | 919 |
| `motor/discipline.rs` | 122 | 1 | 0 | 0 | 0 | 1365 |
| `motor/comp/uefa_seeding.rs` | 122 | 5 | 0 | 0 | 0 | 1107 |
| `motor/comp/leagues/ita_ser_a.rs` | 119 | 8 | 0 | 0 | 0 | 1949 |
| `motor/comp/leagues/bra_nat_third.rs` | 118 | 0 | 0 | 0 | 0 | 689 |
| `motor/tactics.rs` | 117 | 0 | 0 | 0 | 0 | 1472 |
| `motor/awol.rs` | 105 | 0 | 0 | 0 | 0 | 975 |
| `motor/shared.rs` | 104 | 1 | 0 | 0 | 0 | 977 |
| `motor/comp/cups/ita_cup.rs` | 104 | 10 | 0 | 0 | 0 | 1150 |
