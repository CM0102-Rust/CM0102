# rx08: o que ficou de fora da lista (rodada 6, 25/11/2026)

O leitor rx08 escreveu os itens rx08_1 a rx08_10 (30 rotinas). As 19 rotinas abaixo não foram escritas: sete saem de vez
(decisões já tomadas ou entradas que não se desviam) e doze ficam para a próxima rodada de telas. Nenhuma está em
`rust/src` nem em outro item da fila (conferido em 25/11/2026).

## Fora de vez

| Endereço | Nome | Por quê |
|---|---|---|
| 0x4481b0 | season_history_entry_destruct | Só roda no desenrolar de uma exceção (o quadro SEH de quem monta um registro de temporada): quatro destrutores vazios 0x5a53c0. Decisão do sx06 (sobras.md). |
| 0x450a70 | Date::get_day | `mov ax, [ecx]; ret`: prólogo de 4 bytes, menos que os 5 do desvio. |
| 0x450a80 | Date::get_year | `mov ax, [ecx+2]; ret`: os 5 bytes são a rotina inteira com o `ret`; leitor de palavra minúsculo (decisão do sx06). |
| 0x487c50 | coach_init_static_global | O registro no `atexit` (`push 0x487c60; call 0x94627a`): a chamada relativa cai nos bytes do desvio; inicializador estático. |
| 0x487c60 | coach_exit_static_global | Corpo vazio (`ret`). |
| 0x91b760, 0x91db60 | WcEuropeLeague::load_2002_campaign, add_fixed_result | Código morto: a campanha de 2002 nunca roda com o jogo começando em 2001 (o construtor leva o ano ao ciclo de 2004); a mesma decisão do cx16 (sobras.md) e do cx17_7 para as sul-americanas. Só um exe com o ano de início mudado as rodaria. |
| 0x7c3a40 | player_stats_get_apps_for_club | Já é porta do pós-jogo (`POS_PROLOGUES` em `rust/src/motor/match_day.rs`, `gate_apps_for_club`): um segundo desvio no mesmo prólogo precisa de uma exceção no `instalar`, como a `LLIST::add_tail` (sobras.md, sx13). Decisão do cps_9. |

## Para a próxima rodada (TELAS, não escritas por tempo)

Todas rodam só nas telas (a cobertura do mundo viu 0 chamadas).

- **club_records.cpp:** 0x449810 `club_records_format_record_text` (11,8 KB, o texto de cada um dos 32 tipos com vínculos; vai inteira num item, em pedaços por tipo, com o `compare_text_writer`; quem chama já é Rust no rx08_9, `club_records_get_record_text`, pelo exe); 0x450d50 `Club_Records::build_history_list` (1,2 KB, até 500 conjuntos de recordes por ano: os de sempre, os da temporada e cada temporada arquivada do arquivo de histórico +0); 0x451200 `season_history_entry_assign` (cópia campo a campo de um conjunto de 0x38b bytes pelos ajudantes de cópia já em Rust no club_records.rs).
- **coach.cpp (o relatório do treinador):** 0x487c70 `coach_post_report_news` (a notícia de 23 vagas), 0x487f60 `write_coach_report_news_text` (12 KB, texto), 0x48af30 `coach_get_player_career_stage` (192 bytes: 1 abaixo de 18, 2 abaixo de 24, 4 veterano, 8 rodado, 16 estabelecido), 0x48aff0 `coach_assess_player` (2,6 KB, as duas palavras de veredito) e 0x48ba10 `coach_get_assessment_days` (496 bytes). Caminho: ficha do jogador → pedir relatório ao treinador.
- **player_stats.cpp:** 0x7c2750 `PlayerStats::format_record_field` (thiscall(registro, campo, buf, tamanho, subs, apps, zero) ret 0x1c; quadro alinhado `and esp, -8`; buf nulo ou < 0x33: caixa; o buf recebe "----" no campo 0x11 ou "-"; registro nulo: caixa 0x811; tabela de saltos 0x7c3014 para os 17 campos; campo 1 com subs e o byte +1: "%d (%d)" (byte +0 menos +1, e +1); "%d / %d" com apps; zero só com `zero`) e 0x7c3060 `format_totals_field` (o mesmo leiaute sobre os totais inteiros). Quem chama já é Rust no rx08_8 (`format_player_stat`, pelo exe).
- **manager_manager.cpp (os tratadores da diretoria):** 0x69ca00 `apply_for_job`, 0x6a8810 `handle_board_request` (2,9 KB), 0x6ab800 `submit_ban_appeal`, 0x6aca50 `decide_ultimatum_response` (3 KB) e 0x6ad610 `handle_manager_ultimatum`. Notas da 0x6ad610 (lida na desmontagem, não escrita): thiscall(tecnico, clube, codigo) ret 0xc, quadro SEH só da data local (0x54a110); sem técnico, sem `+0x69` do técnico, sem clube, sem `+0x57` ou sem `+0x69` do clube: nada. O registro do ultimato local de 0x1a bytes: o id do clube, o id do técnico, a data de hoje + 1 (0x54c5b0 sobre 0xae2c90, depois 0x417c10), o código; o boato: reputação do técnico (`[+0x69]+0xa`) > 6750, ou sorteio(6750) + 2000 abaixo dela; com boato, `post_ultimatum_response_news` (rx08_10) com (código, 1, 1, 0) ou (código, 1, 2, 0) conforme a tabela 0xae2d14 (6 bytes por clube, +4), a confiança da diretoria (registro de 0x49 bytes, +8 > 5250) e os dias desde a chegada (0x54cdb0 > 1000, ou < 180 com reputação > 6500); depois `decide_ultimatum_response` 0x6aca50 e a notícia (código, resultado, 3, ...); resultado 2: a saída na lista +4 (6 bytes; o motivo 0xc, 0xd, 0xe ou 0xf pela reputação, pelos dias e pela marca) com o bit 2 do +2 do registro e, com presidente (`+0xbf`), o sorteio do desafeto (0x547af0); 3 e 4: a confiança (+6 do registro) menos 1250 ou 500, entre 1 e 10000.

