*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 18559 |
| chaves fixas cruas | 1432 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 10 |
| rotinas sem cabeçalho | 39 |
| linhas | 180560 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | linhas |
|---|---:|---:|---:|---:|---:|---:|
| `motor/transfer_manager.rs` | 1532 | 16 | 0 | 0 | 1 | 11257 |
| `motor/manager_manager.rs` | 965 | 3 | 0 | 0 | 0 | 5995 |
| `motor.rs` | 817 | 0 | 0 | 0 | 0 | 11957 |
| `motor/comp/league.rs` | 703 | 9 | 0 | 0 | 0 | 4143 |
| `motor/contract_manager.rs` | 696 | 13 | 0 | 2 | 0 | 4598 |
| `motor/match_screens.rs` | 654 | 2 | 0 | 0 | 0 | 6090 |
| `motor/match_eng.rs` | 628 | 7 | 0 | 0 | 1 | 10761 |
| `motor/player_regen.rs` | 602 | 7 | 0 | 0 | 0 | 3330 |
| `motor/shortlist_manager.rs` | 582 | 7 | 0 | 0 | 0 | 4292 |
| `motor/database.rs` | 571 | 46 | 0 | 2 | 0 | 9738 |
| `motor/match_pl.rs` | 530 | 5 | 0 | 0 | 0 | 9545 |
| `motor/national_teams.rs` | 530 | 29 | 0 | 0 | 0 | 3886 |
| `motor/comp/friendly.rs` | 519 | 15 | 0 | 0 | 0 | 3545 |
| `motor/injury.rs` | 495 | 0 | 0 | 0 | 0 | 3257 |
| `motor/comp/cup.rs` | 489 | 5 | 0 | 0 | 0 | 3435 |
| `motor/comp/leagues/fin_prm.rs` | 479 | 13 | 0 | 0 | 0 | 3195 |
| `motor/media.rs` | 478 | 1 | 0 | 0 | 4 | 3320 |
| `motor/staff_records.rs` | 388 | 1 | 0 | 0 | 0 | 2144 |
| `motor/club_records.rs` | 299 | 3 | 0 | 3 | 0 | 1790 |
| `motor/comp/fix_man.rs` | 260 | 10 | 0 | 0 | 0 | 2555 |
| `motor/match_man.rs` | 248 | 1 | 0 | 0 | 1 | 4264 |
| `motor/comp/stadium.rs` | 201 | 7 | 0 | 0 | 0 | 1297 |
| `motor/comp/eurocomp/european_cup.rs` | 191 | 4 | 0 | 0 | 0 | 1633 |
| `motor/news.rs` | 185 | 0 | 0 | 0 | 0 | 1572 |
| `motor/match_day.rs` | 179 | 8 | 0 | 0 | 0 | 1997 |
| `motor/player_stats.rs` | 175 | 0 | 0 | 0 | 0 | 1471 |
| `motor/comp/comp.rs` | 165 | 7 | 0 | 0 | 0 | 1118 |
| `motor/player_search.rs` | 156 | 0 | 0 | 0 | 0 | 1095 |
| `motor/comp/leagues/bra_reg_rio.rs` | 149 | 13 | 0 | 0 | 0 | 900 |
| `motor/scout_manager.rs` | 143 | 9 | 0 | 0 | 0 | 1304 |
| `motor/officials_manager.rs` | 139 | 0 | 0 | 0 | 0 | 969 |
| `motor/training_manager.rs` | 137 | 0 | 0 | 0 | 0 | 1444 |
| `motor/comp/worldcup/world_cup.rs` | 130 | 7 | 0 | 0 | 0 | 735 |
| `motor/comp/uefa_seeding.rs` | 127 | 7 | 0 | 0 | 0 | 1113 |
| `motor/comp/leagues/usa_mls.rs` | 125 | 1 | 0 | 0 | 0 | 810 |
| `motor/comp/leagues/ita_ser_a.rs` | 121 | 11 | 0 | 1 | 0 | 1966 |
| `motor/tactics.rs` | 117 | 6 | 0 | 0 | 0 | 1537 |
| `motor/match_events.rs` | 108 | 0 | 0 | 0 | 0 | 2384 |
| `motor/shared.rs` | 104 | 1 | 0 | 0 | 0 | 975 |
| `motor/comp/cups/ita_cup.rs` | 104 | 10 | 0 | 0 | 3 | 1114 |
