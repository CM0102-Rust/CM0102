*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 11432 |
| chaves fixas cruas | 1495 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 3278 |
| linhas | 197112 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor/transfer_manager.rs` | 720 | 16 | 0 | 0 | 0 | 208 | 10313 |
| `motor.rs` | 614 | 0 | 0 | 0 | 0 | 106 | 10771 |
| `motor/match_screens.rs` | 601 | 2 | 0 | 0 | 0 | 105 | 5895 |
| `motor/match_eng.rs` | 425 | 7 | 0 | 0 | 0 | 131 | 11655 |
| `motor/manager_manager.rs` | 416 | 1 | 0 | 0 | 0 | 107 | 5115 |
| `motor/comp/cup.rs` | 341 | 4 | 0 | 0 | 0 | 50 | 3328 |
| `motor/match_pl.rs` | 334 | 5 | 0 | 0 | 0 | 56 | 10093 |
| `motor/comp/league.rs` | 319 | 10 | 0 | 0 | 0 | 57 | 3966 |
| `motor/staff_records.rs` | 271 | 1 | 0 | 0 | 0 | 74 | 2273 |
| `motor/national_teams.rs` | 259 | 23 | 0 | 0 | 0 | 89 | 3514 |
| `motor/injury.rs` | 249 | 0 | 0 | 0 | 0 | 45 | 3009 |
| `motor/comp/fix_man.rs` | 242 | 16 | 0 | 0 | 0 | 65 | 2540 |
| `motor/comp/friendly.rs` | 238 | 14 | 0 | 0 | 0 | 74 | 3277 |
| `motor/contract_manager.rs` | 236 | 9 | 0 | 0 | 0 | 113 | 4198 |
| `motor/shortlist_manager.rs` | 189 | 16 | 0 | 0 | 0 | 78 | 4193 |
| `motor/media.rs` | 175 | 1 | 0 | 0 | 0 | 78 | 3387 |
| `motor/player_stats.rs` | 170 | 4 | 0 | 0 | 0 | 19 | 1425 |
| `motor/club_records.rs` | 164 | 3 | 0 | 0 | 0 | 23 | 1249 |
| `motor/comp/leagues/fin_prm.rs` | 159 | 14 | 0 | 0 | 0 | 54 | 2975 |
| `motor/match_man.rs` | 151 | 1 | 0 | 0 | 0 | 63 | 4562 |
| `motor/match_day.rs` | 139 | 8 | 0 | 0 | 0 | 34 | 1930 |
| `motor/comp/stadium.rs` | 138 | 10 | 0 | 0 | 0 | 30 | 1299 |
| `motor/database.rs` | 130 | 64 | 0 | 0 | 0 | 173 | 8853 |
| `motor/player_regen.rs` | 124 | 6 | 0 | 0 | 0 | 49 | 3284 |
| `motor/scout_manager.rs` | 120 | 9 | 0 | 0 | 0 | 23 | 1201 |
| `motor/officials_manager.rs` | 109 | 0 | 0 | 0 | 0 | 11 | 850 |
| `motor/compare/transfer_manager.rs` | 107 | 0 | 0 | 0 | 0 | 61 | 1666 |
| `motor/player_search.rs` | 103 | 0 | 0 | 0 | 0 | 17 | 910 |
| `motor/tactics.rs` | 100 | 0 | 0 | 0 | 0 | 40 | 1405 |
| `motor/comp/eurocomp/european_cup.rs` | 100 | 5 | 0 | 0 | 0 | 29 | 1448 |
| `motor/comp/cups/ita_cup.rs` | 98 | 10 | 0 | 0 | 0 | 4 | 1095 |
| `motor/discipline.rs` | 90 | 1 | 0 | 0 | 0 | 23 | 1072 |
| `motor/compare/comp/friendly.rs` | 89 | 0 | 0 | 0 | 0 | 32 | 686 |
| `motor/comp/comp.rs` | 82 | 8 | 0 | 0 | 0 | 16 | 730 |
| `motor/formation.rs` | 74 | 0 | 0 | 0 | 0 | 3 | 1064 |
| `motor/comp/leagues/bra_reg_rio.rs` | 72 | 11 | 0 | 0 | 0 | 10 | 781 |
| `motor/comp/leagues/ita_ser_a.rs` | 67 | 2 | 0 | 0 | 0 | 20 | 1698 |
| `motor/comp/worldcup/world_cup.rs` | 62 | 7 | 0 | 0 | 0 | 11 | 620 |
| `motor/area.rs` | 59 | 0 | 0 | 0 | 0 | 4 | 331 |
| `motor/training_manager.rs` | 59 | 0 | 0 | 0 | 0 | 4 | 1264 |
