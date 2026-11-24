# sbE: o que ficou para a próxima rodada

Leitor sbE, rodada 5, 24/11/2026. Estas rotinas da seção cx10 de `levas/reorganizacao/proximas/sobras.md` não viraram item nesta rodada: o pedido as deixava para o fim (a Regionalliga) e por último (a Itália), e o tempo acabou nos 13 itens `sbE_1`..`sbE_13`. Nenhuma está portada nem na fila.

## Regionalliga alemã (ger_regional.cpp): base NORMAL

Só existe com a Regionalliga carregada, que é divisão inferior (bit `LOWER_DIVISIONS_LOADED`): o `simular.py` fixa MINIMUM, então nenhuma roda no teste (0 chamadas na cobertura do mundo até 2002-08-01). Os itens levam `LIGAS = 'ENG BEL HOL GER DEN'`, `ATE = '2002-08-01'` e `BASE = 'NORMAL'`, como o `sbE_11`.

- **0x5dcbd0 `GerRegional::GerRegional`** (560 bytes): construtor; o molde é o `construct_nation_division`, com a tabela Norte, as estatísticas, os jogos e a fase Sul (0x5ddc50) no fim. Ver se o `open_nation_division` e o `build_new_nation_division` do `sbE_2` servem.
- **0x5dcef0 `GerRegional::create_schedule`** (2304 bytes): 34 rodadas, ou 38 na fase −1 em 2000; tirar as tabelas com o `emula.py` (o `tabela.py` do rascunho `sbE` aceita o ano).
- **0x5dd7f0 `GerRegional::create_table`** (512): a tabela Norte de 18 (19 em 2000) com os clubes da seção Norte (`club::RESERVE_DIVISION`, que o `sbE_3` grava).
- **0x5dd9f0 `GerRegional::set_up_rules`** (160): o molde `fra_first_apply_league_rules` com a 2. Bundesliga acima.
- **0x5dda90 `GerRegional::start_new_season`** (448): o molde `rebuild_season`/`top_division_turnover`.
- **0x5ddc50 `GerRegional::create_south_stage`** (928): os 18 da seção Sul num League_Stage na vaga 0 (o molde do `pol_first_create_second_group_stage` do `sbE_10`).
- **0x5ddff0 `GerRegional::get_club_array`** (320): a lista dos clubes da tabela Norte seguida dos da fase Sul.
- **0x5de130 `GerRegional::set_predicted_positions`** (368): ordena cada seção pela reputação e grava as posições previstas ímpares intercaladas.

## Itália: só em 2003, ou com a base NORMAL

- **0x6419e0 `ItaSerA::advance_stage`** (144) e **0x641a70 `ItaSerA::create_tiebreak_playoffs`** (848): o desempate (spareggio) de dois clubes empatados em pontos numa fronteira de zona; não houve empate até 2002-08-01. Pede `ATE` em 2003 e sorte (só roda com o empate).
- **0x646b20 `ItaSerB::advance_stage`** (144), **0x646bb0 `ItaSerB::create_tiebreak_playoff_stage`** (703) e **0x646e70 `ItaSerB::update_staff_records_for_status`** (480): o mesmo na Série B; o último passa os resultados à base e, no desempate, sobe o vencedor (o molde do `rus_prm_update_staff_records_for_status` do `cx13_12`).
- **0x6421c0 `ItaSerA::exchange_serie_d_clubs`** (592) e **0x642790 `ita_ser_a_regroup_serie_c2_clubs`** (240): a Série C2 e a Série D só existem com a base NORMAL (ver o cx11).
