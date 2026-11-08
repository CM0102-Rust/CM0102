*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 16850 |
| chaves fixas cruas | 1494 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| linhas | 191328 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | linhas |
|---|---:|---:|---:|---:|---:|---:|
| `motor/transfer_manager.rs` | 1423 | 16 | 0 | 0 | 0 | 11766 |
| `motor.rs` | 810 | 0 | 0 | 0 | 0 | 12272 |
| `motor/comp/league.rs` | 705 | 10 | 0 | 0 | 0 | 4518 |
| `motor/match_screens.rs` | 663 | 2 | 0 | 0 | 0 | 6445 |
| `motor/match_eng.rs` | 650 | 7 | 0 | 0 | 0 | 11596 |
| `motor/manager_manager.rs` | 637 | 1 | 0 | 0 | 0 | 5565 |
| `motor/comp/cup.rs` | 563 | 4 | 0 | 0 | 0 | 3896 |
| `motor/player_regen.rs` | 558 | 7 | 0 | 0 | 0 | 3770 |
| `motor/match_pl.rs` | 536 | 5 | 0 | 0 | 0 | 10000 |
| `motor/contract_manager.rs` | 528 | 9 | 0 | 0 | 0 | 4851 |
| `motor/shortlist_manager.rs` | 512 | 16 | 0 | 0 | 0 | 4511 |
| `motor/comp/friendly.rs` | 501 | 14 | 0 | 0 | 0 | 3868 |
| `motor/national_teams.rs` | 490 | 23 | 0 | 0 | 0 | 4053 |
| `motor/media.rs` | 447 | 1 | 0 | 0 | 0 | 3739 |
| `motor/injury.rs` | 445 | 0 | 0 | 0 | 0 | 3490 |
| `motor/database.rs` | 413 | 64 | 0 | 0 | 0 | 10221 |
| `motor/staff_records.rs` | 346 | 1 | 0 | 0 | 0 | 2650 |
| `motor/comp/leagues/fin_prm.rs` | 282 | 14 | 0 | 0 | 0 | 3289 |
| `motor/comp/fix_man.rs` | 275 | 16 | 0 | 0 | 0 | 2889 |
| `motor/match_man.rs` | 248 | 1 | 0 | 0 | 0 | 4623 |
| `motor/club_records.rs` | 228 | 3 | 0 | 0 | 0 | 1642 |
| `motor/player_stats.rs` | 216 | 4 | 0 | 0 | 0 | 1773 |
| `motor/comp/stadium.rs` | 206 | 10 | 0 | 0 | 0 | 1458 |
| `motor/comp/eurocomp/european_cup.rs` | 185 | 5 | 0 | 0 | 0 | 1740 |
| `motor/match_day.rs` | 176 | 8 | 0 | 0 | 0 | 2080 |
| `motor/player_search.rs` | 159 | 0 | 0 | 0 | 0 | 1072 |
| `motor/scout_manager.rs` | 157 | 9 | 0 | 0 | 0 | 1455 |
| `motor/comp/leagues/bra_reg_rio.rs` | 146 | 13 | 0 | 0 | 0 | 963 |
| `motor/training_manager.rs` | 130 | 0 | 0 | 0 | 0 | 1542 |
| `motor/comp/worldcup/world_cup.rs` | 128 | 7 | 0 | 0 | 0 | 778 |
| `motor/comp/leagues/usa_mls.rs` | 123 | 1 | 0 | 0 | 0 | 943 |
| `motor/officials_manager.rs` | 120 | 0 | 0 | 0 | 0 | 1000 |
| `motor/comp/uefa_seeding.rs` | 119 | 5 | 0 | 0 | 0 | 1126 |
| `motor/comp/leagues/bra_nat_third.rs` | 116 | 0 | 0 | 0 | 0 | 694 |
| `motor/comp/leagues/ita_ser_a.rs` | 116 | 8 | 0 | 0 | 0 | 2011 |
| `motor/discipline.rs` | 112 | 1 | 0 | 0 | 0 | 1398 |
| `motor/tactics.rs` | 110 | 0 | 0 | 0 | 0 | 1497 |
| `motor/comp/cups/ita_cup.rs` | 104 | 10 | 0 | 0 | 0 | 1217 |
| `motor/shared.rs` | 97 | 1 | 0 | 0 | 0 | 988 |
| `motor/comp/comp.rs` | 96 | 8 | 0 | 0 | 0 | 848 |
