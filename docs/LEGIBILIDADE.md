*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 16859 |
| chaves fixas cruas | 1495 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 11532 |
| linhas | 196619 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor/transfer_manager.rs` | 1280 | 16 | 0 | 0 | 0 | 853 | 10346 |
| `motor/match_eng.rs` | 645 | 7 | 0 | 0 | 0 | 582 | 11655 |
| `motor/comp/league.rs` | 640 | 10 | 0 | 0 | 0 | 219 | 3966 |
| `motor/match_screens.rs` | 635 | 2 | 0 | 0 | 0 | 327 | 5895 |
| `motor/manager_manager.rs` | 617 | 1 | 0 | 0 | 0 | 295 | 5125 |
| `motor.rs` | 614 | 0 | 0 | 0 | 0 | 106 | 10483 |
| `motor/match_pl.rs` | 538 | 5 | 0 | 0 | 0 | 376 | 10093 |
| `motor/player_regen.rs` | 520 | 6 | 0 | 0 | 0 | 176 | 3320 |
| `motor/comp/cup.rs` | 501 | 4 | 0 | 0 | 0 | 235 | 3328 |
| `motor/shortlist_manager.rs` | 487 | 16 | 0 | 0 | 0 | 303 | 4193 |
| `motor/contract_manager.rs` | 480 | 9 | 0 | 0 | 0 | 344 | 4210 |
| `motor/national_teams.rs` | 453 | 23 | 0 | 0 | 0 | 274 | 3514 |
| `motor/media.rs` | 432 | 1 | 0 | 0 | 0 | 192 | 3387 |
| `motor/comp/friendly.rs` | 410 | 14 | 0 | 0 | 0 | 196 | 3277 |
| `motor/database.rs` | 406 | 64 | 0 | 0 | 0 | 491 | 8853 |
| `motor/injury.rs` | 389 | 0 | 0 | 0 | 0 | 191 | 3009 |
| `motor/staff_records.rs` | 319 | 1 | 0 | 0 | 0 | 175 | 2273 |
| `motor/comp/leagues/fin_prm.rs` | 274 | 14 | 0 | 0 | 0 | 155 | 2975 |
| `motor/comp/fix_man.rs` | 259 | 16 | 0 | 0 | 0 | 204 | 2547 |
| `motor/match_man.rs` | 248 | 1 | 0 | 0 | 0 | 221 | 4562 |
| `motor/comp/stadium.rs` | 195 | 10 | 0 | 0 | 0 | 52 | 1299 |
| `motor/player_stats.rs` | 183 | 4 | 0 | 0 | 0 | 75 | 1425 |
| `motor/club_records.rs` | 175 | 3 | 0 | 0 | 0 | 92 | 1249 |
| `motor/match_day.rs` | 163 | 8 | 0 | 0 | 0 | 113 | 1930 |
| `motor/player_search.rs` | 149 | 0 | 0 | 0 | 0 | 58 | 910 |
| `motor/comp/eurocomp/european_cup.rs` | 148 | 5 | 0 | 0 | 0 | 103 | 1448 |
| `motor/compare/transfer_manager.rs` | 143 | 0 | 0 | 0 | 0 | 245 | 1666 |
| `motor/scout_manager.rs` | 137 | 9 | 0 | 0 | 0 | 85 | 1201 |
| `motor/comp/leagues/bra_reg_rio.rs` | 129 | 13 | 0 | 0 | 0 | 46 | 858 |
| `motor/officials_manager.rs` | 118 | 0 | 0 | 0 | 0 | 37 | 850 |
| `motor/comp/uefa_seeding.rs` | 118 | 5 | 0 | 0 | 0 | 63 | 1022 |
| `motor/comp/leagues/ita_ser_a.rs` | 114 | 8 | 0 | 0 | 0 | 71 | 1917 |
| `motor/training_manager.rs` | 109 | 0 | 0 | 0 | 0 | 40 | 1264 |
| `motor/comp/cups/ita_cup.rs` | 104 | 10 | 0 | 0 | 0 | 46 | 1159 |
| `motor/tactics.rs` | 103 | 0 | 0 | 0 | 0 | 128 | 1405 |
| `motor/discipline.rs` | 92 | 1 | 0 | 0 | 0 | 90 | 1072 |
| `motor/comp/comp.rs` | 92 | 8 | 0 | 0 | 0 | 50 | 730 |
| `motor/compare/comp/friendly.rs` | 91 | 0 | 0 | 0 | 0 | 68 | 686 |
| `motor/comp/leagues/usa_mls.rs` | 89 | 1 | 0 | 0 | 0 | 46 | 738 |
| `motor/comp/worldcup/world_cup.rs` | 87 | 7 | 0 | 0 | 0 | 36 | 620 |
