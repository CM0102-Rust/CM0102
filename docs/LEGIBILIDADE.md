*[English version](LEGIBILIDADE.en.md).*

# Placar de legibilidade

Gerado por `python engenharia-reversa/legibilidade.py`. O que ainda atrapalha quem lê o motor de fora (`rust/src/motor.rs` e `rust/src/motor/`); cada rodada da etapa 4 da reorganização deve baixar esses números.

## Total

| Medida | Quantidade |
|---|---:|
| deslocamentos crus | 1854 |
| chaves fixas cruas | 1821 |
| chamadas por endereço | 0 |
| cópias de acesso à memória | 0 |
| rotinas sem cabeçalho | 0 |
| locais/parâmetros em português | 238 |
| linhas | 235359 |

## Por arquivo (os 40 piores, pelos deslocamentos crus)

| Arquivo | deslocamentos crus | chaves fixas cruas | chamadas por endereço | cópias de acesso à memória | rotinas sem cabeçalho | locais/parâmetros em português | linhas |
|---|---:|---:|---:|---:|---:|---:|---:|
| `motor.rs` | 587 | 0 | 0 | 0 | 0 | 104 | 6430 |
| `motor/match_screens.rs` | 127 | 4 | 0 | 0 | 0 | 6 | 5883 |
| `motor/match_man.rs` | 122 | 1 | 0 | 0 | 0 | 1 | 4498 |
| `motor/match_eng.rs` | 93 | 7 | 0 | 0 | 0 | 13 | 11686 |
| `motor/national_teams.rs` | 60 | 33 | 0 | 0 | 0 | 2 | 3512 |
| `motor/match_pl.rs` | 56 | 5 | 0 | 0 | 0 | 7 | 10183 |
| `motor/fields.rs` | 40 | 0 | 0 | 0 | 0 | 0 | 3283 |
| `motor/manager_manager.rs` | 31 | 2 | 0 | 0 | 0 | 0 | 5143 |
| `motor/scout_manager.rs` | 31 | 18 | 0 | 0 | 0 | 0 | 1225 |
| `motor/tactics.rs` | 31 | 0 | 0 | 0 | 0 | 2 | 1444 |
| `motor/comp/cup.rs` | 30 | 8 | 0 | 0 | 0 | 2 | 3519 |
| `motor/transfer_manager.rs` | 27 | 29 | 0 | 0 | 0 | 3 | 10492 |
| `motor/injury.rs` | 23 | 0 | 0 | 0 | 0 | 4 | 3043 |
| `motor/training_manager.rs` | 23 | 0 | 0 | 0 | 0 | 1 | 1319 |
| `motor/comp/friendly.rs` | 23 | 26 | 0 | 0 | 0 | 1 | 3308 |
| `motor/comp/eurocomp/european_cup.rs` | 23 | 9 | 0 | 0 | 0 | 1 | 1524 |
| `motor/database.rs` | 21 | 86 | 0 | 0 | 0 | 8 | 9344 |
| `motor/human_manager.rs` | 21 | 3 | 0 | 0 | 0 | 0 | 773 |
| `motor/player_regen.rs` | 18 | 6 | 0 | 0 | 0 | 2 | 3374 |
| `motor/shortlist_manager.rs` | 17 | 31 | 0 | 0 | 0 | 2 | 4248 |
| `motor/comp/league.rs` | 17 | 20 | 0 | 0 | 0 | 6 | 4132 |
| `motor/comp/cups/eng_cc_cup.rs` | 17 | 12 | 0 | 0 | 0 | 1 | 620 |
| `motor/news.rs` | 16 | 0 | 0 | 0 | 0 | 0 | 1058 |
| `motor/comp/leagues/bra_nat_third.rs` | 13 | 0 | 0 | 0 | 0 | 0 | 554 |
| `motor/player_stats.rs` | 12 | 8 | 0 | 0 | 0 | 1 | 1501 |
| `motor/comp/leagues/fin_prm.rs` | 12 | 24 | 0 | 0 | 0 | 1 | 3104 |
| `motor/comp/worldcup/world_cup.rs` | 12 | 14 | 0 | 0 | 0 | 0 | 663 |
| `motor/formation.rs` | 11 | 0 | 0 | 0 | 0 | 0 | 1089 |
| `motor/media.rs` | 10 | 2 | 0 | 0 | 0 | 0 | 3444 |
| `motor/comp/leagues/ita_ser_a.rs` | 10 | 3 | 0 | 0 | 0 | 6 | 1746 |
| `motor/contract_manager.rs` | 9 | 16 | 0 | 0 | 0 | 4 | 4241 |
| `motor/comp/cups/jap_j_cup.rs` | 9 | 24 | 0 | 0 | 0 | 1 | 502 |
| `motor/comp/eurocomp/intertoto_cup.rs` | 9 | 8 | 0 | 0 | 0 | 0 | 393 |
| `motor/match_day.rs` | 8 | 9 | 0 | 0 | 0 | 1 | 1899 |
| `motor/match_official.rs` | 8 | 0 | 0 | 0 | 0 | 0 | 696 |
| `motor/zipdir.rs` | 8 | 0 | 0 | 0 | 0 | 0 | 604 |
| `motor/compare/comp/leagues/bra_nat_third.rs` | 8 | 0 | 0 | 0 | 0 | 0 | 252 |
| `motor/transfer/england_rules.rs` | 8 | 3 | 0 | 0 | 0 | 0 | 332 |
| `motor/officials_manager.rs` | 7 | 0 | 0 | 0 | 0 | 2 | 873 |
| `motor/player_search.rs` | 7 | 0 | 0 | 0 | 0 | 0 | 903 |
