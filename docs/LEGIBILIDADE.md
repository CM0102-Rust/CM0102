*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 15458 |
| chaves fixas cruas | 1495 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 3286 |
| linhas | 196892 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor/transfer_manager.rs` | 1100 | 16 | 0 | 0 | 0 | 209 | 10346 |
| `motor/match_eng.rs` | 645 | 7 | 0 | 0 | 0 | 131 | 11655 |
| `motor/match_screens.rs` | 635 | 2 | 0 | 0 | 0 | 105 | 5895 |
| `motor/manager_manager.rs` | 617 | 1 | 0 | 0 | 0 | 107 | 5125 |
| `motor.rs` | 614 | 0 | 0 | 0 | 0 | 106 | 10483 |
| `motor/match_pl.rs` | 538 | 5 | 0 | 0 | 0 | 56 | 10093 |
| `motor/comp/league.rs` | 466 | 10 | 0 | 0 | 0 | 57 | 3966 |
| `motor/national_teams.rs` | 453 | 23 | 0 | 0 | 0 | 89 | 3514 |
| `motor/shortlist_manager.rs` | 447 | 16 | 0 | 0 | 0 | 78 | 4193 |
| `motor/player_regen.rs` | 445 | 6 | 0 | 0 | 0 | 49 | 3320 |
| `motor/contract_manager.rs` | 439 | 9 | 0 | 0 | 0 | 113 | 4210 |
| `motor/media.rs` | 432 | 1 | 0 | 0 | 0 | 78 | 3387 |
| `motor/database.rs` | 406 | 64 | 0 | 0 | 0 | 173 | 8853 |
| `motor/comp/cup.rs` | 403 | 4 | 0 | 0 | 0 | 50 | 3328 |
| `motor/injury.rs` | 339 | 0 | 0 | 0 | 0 | 45 | 3009 |
| `motor/staff_records.rs` | 319 | 1 | 0 | 0 | 0 | 74 | 2273 |
| `motor/comp/friendly.rs` | 298 | 14 | 0 | 0 | 0 | 74 | 3277 |
| `motor/comp/leagues/fin_prm.rs` | 265 | 14 | 0 | 0 | 0 | 54 | 2975 |
| `motor/comp/fix_man.rs` | 259 | 16 | 0 | 0 | 0 | 65 | 2547 |
| `motor/match_man.rs` | 248 | 1 | 0 | 0 | 0 | 63 | 4562 |
| `motor/comp/stadium.rs` | 195 | 10 | 0 | 0 | 0 | 30 | 1299 |
| `motor/player_stats.rs` | 183 | 4 | 0 | 0 | 0 | 19 | 1425 |
| `motor/club_records.rs` | 175 | 3 | 0 | 0 | 0 | 23 | 1249 |
| `motor/match_day.rs` | 163 | 8 | 0 | 0 | 0 | 34 | 1930 |
| `motor/compare/transfer_manager.rs` | 143 | 0 | 0 | 0 | 0 | 61 | 1666 |
| `motor/scout_manager.rs` | 137 | 9 | 0 | 0 | 0 | 23 | 1201 |
| `motor/officials_manager.rs` | 118 | 0 | 0 | 0 | 0 | 11 | 850 |
| `motor/comp/uefa_seeding.rs` | 118 | 5 | 0 | 0 | 0 | 9 | 1022 |
| `motor/player_search.rs` | 113 | 0 | 0 | 0 | 0 | 17 | 910 |
| `motor/comp/leagues/bra_reg_rio.rs` | 113 | 13 | 0 | 0 | 0 | 11 | 858 |
| `motor/training_manager.rs` | 109 | 0 | 0 | 0 | 0 | 4 | 1264 |
| `motor/comp/cups/ita_cup.rs` | 104 | 10 | 0 | 0 | 0 | 4 | 1159 |
| `motor/tactics.rs` | 103 | 0 | 0 | 0 | 0 | 40 | 1405 |
| `motor/comp/eurocomp/european_cup.rs` | 100 | 5 | 0 | 0 | 0 | 29 | 1448 |
| `motor/discipline.rs` | 92 | 1 | 0 | 0 | 0 | 23 | 1072 |
| `motor/comp/comp.rs` | 92 | 8 | 0 | 0 | 0 | 16 | 730 |
| `motor/compare/comp/friendly.rs` | 91 | 0 | 0 | 0 | 0 | 32 | 686 |
| `motor/compare/match_eng.rs` | 85 | 0 | 0 | 0 | 0 | 9 | 1045 |
| `motor/comp/leagues/ita_ser_a.rs` | 83 | 8 | 0 | 0 | 0 | 20 | 1917 |
| `motor/formation.rs` | 74 | 0 | 0 | 0 | 0 | 3 | 1064 |
