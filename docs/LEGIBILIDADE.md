*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 2063 |
| chaves fixas cruas | 1104 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 1 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 239 |
| linhas | 273172 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 1011 | 5 | 0 | 1 | 0 | 106 | 61295 |
| `motor/match_pl.rs` | 129 | 0 | 0 | 0 | 0 | 7 | 9585 |
| `motor/match_man.rs` | 122 | 1 | 0 | 0 | 0 | 1 | 4263 |
| `motor/match_screens.rs` | 88 | 0 | 0 | 0 | 0 | 6 | 5483 |
| `motor/match_eng.rs` | 82 | 0 | 0 | 0 | 0 | 13 | 10847 |
| `motor/fields.rs` | 51 | 0 | 0 | 0 | 0 | 0 | 3376 |
| `motor/national_teams.rs` | 41 | 1 | 0 | 0 | 0 | 2 | 3097 |
| `motor/tactics.rs` | 33 | 0 | 0 | 0 | 0 | 2 | 1279 |
| `motor/manager_manager.rs` | 26 | 0 | 0 | 0 | 0 | 0 | 4723 |
| `motor/database.rs` | 20 | 4 | 0 | 0 | 0 | 8 | 8282 |
| `motor/injury.rs` | 20 | 0 | 0 | 0 | 0 | 4 | 2777 |
| `motor/comp/cup.rs` | 19 | 3 | 0 | 0 | 0 | 2 | 3050 |
| `motor/scout_manager.rs` | 18 | 9 | 0 | 0 | 0 | 0 | 1093 |
| `motor/player_regen.rs` | 16 | 1 | 0 | 0 | 0 | 2 | 3067 |
| `motor/comp/friendly.rs` | 16 | 3 | 0 | 0 | 0 | 1 | 2961 |
| `motor/shortlist_manager.rs` | 15 | 2 | 0 | 0 | 0 | 2 | 3917 |
| `motor/human_manager.rs` | 13 | 3 | 0 | 0 | 0 | 0 | 657 |
| `motor/news.rs` | 13 | 0 | 0 | 0 | 0 | 0 | 919 |
| `motor/training_manager.rs` | 13 | 0 | 0 | 0 | 0 | 1 | 1157 |
| `motor/comp/eurocomp/european_cup.rs` | 13 | 1 | 0 | 0 | 0 | 1 | 1315 |
| `motor/player_stats.rs` | 12 | 0 | 0 | 0 | 0 | 1 | 1347 |
| `motor/comp/leagues/fin_prm.rs` | 12 | 0 | 0 | 0 | 0 | 1 | 2837 |
| `motor/formation.rs` | 11 | 0 | 0 | 0 | 0 | 0 | 968 |
| `motor/media.rs` | 10 | 0 | 0 | 0 | 0 | 0 | 3112 |
| `motor/comp/cups/eng_cc_cup.rs` | 10 | 6 | 0 | 0 | 0 | 1 | 535 |
| `motor/comp/worldcup/world_cup.rs` | 10 | 0 | 0 | 0 | 0 | 0 | 581 |
| `motor/contract_manager.rs` | 9 | 3 | 0 | 0 | 0 | 4 | 3743 |
| `motor/player_search.rs` | 9 | 0 | 0 | 0 | 0 | 0 | 815 |
| `motor/comp/cups/jap_j_cup.rs` | 9 | 12 | 0 | 0 | 0 | 1 | 436 |
| `motor/zipdir.rs` | 8 | 0 | 0 | 0 | 0 | 0 | 534 |
| `motor/comp/fifa_rankings.rs` | 8 | 0 | 0 | 0 | 0 | 0 | 339 |
| `motor/compare/comp/leagues/bra_nat_third.rs` | 8 | 0 | 0 | 0 | 0 | 0 | 220 |
| `motor/officials_manager.rs` | 7 | 0 | 0 | 0 | 0 | 2 | 764 |
| `motor/transfer_manager.rs` | 7 | 0 | 0 | 0 | 0 | 3 | 9228 |
| `motor/comp/leagues/bra_nat_third.rs` | 7 | 0 | 0 | 0 | 0 | 0 | 468 |
| `motor/staff_records.rs` | 6 | 0 | 0 | 0 | 0 | 0 | 2111 |
| `motor/comp/league.rs` | 6 | 0 | 0 | 0 | 0 | 5 | 3796 |
| `motor/compare/match_pl.rs` | 6 | 0 | 0 | 0 | 0 | 0 | 561 |
| `motor/transfer/england_rules.rs` | 6 | 0 | 0 | 0 | 0 | 0 | 291 |
| `motor/match_day.rs` | 5 | 0 | 0 | 0 | 0 | 1 | 1772 |
