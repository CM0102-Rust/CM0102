# Comparar dois registros de partida por gols, uma duração em palavras e a oferta de empréstimo nova
# (record_utils.cpp, transfer_offer.cpp; leitor mwg, rodada 2 do mundo inteiro, 12/11/2026). match_record_has_more_goals
# roda 129.577 vezes até 2002-08-01 no mundo inteiro (cobertura_mundo.txt; os recordes dos clubes, club_records 0x4454ce
# e companhia, e as estatísticas das competições, 0x4a0a14), format_duration_text 169 (textos de lesão e de prazo) e
# init_loan_offer 4.182 (as ofertas de empréstimo do mercado, transfer_manager.rs, Rust). Rodam na carreira só com a
# Inglaterra: LIGAS = 'ENG'.
# Convenções conferidas na desmontagem (0x7e4be0..0x7e4c95, 0x7e5780..0x7e5b06, 0x8e7810..0x8e78e7):
# - match_record_has_more_goals: cdecl(a, b) -> eax 0/1; format_duration_text: cdecl(dias, saída, tamanho, aproximado)
#   -> eax 0/1; init_loan_offer: thiscall(oferta, pessoa, clube), ret 8;
# - os textos por 0x66f4e0 langlib_sprintf (cdecl, variádica: saída, formato, número);
# - divisões: /7 (0x92492493 add sar 2), /30 (0x88888889 add sar 4), /365 (0xb38cf9b1 add sar 8), truncando para zero;
# - as caixas de erro (linhas 0x107 e 0x288 de record_utils.cpp, 0xa7a088; 0x485 de transfer_offer.cpp, 0xa9e318).
# Defeito do original: match_record_has_more_goals devolve 0 quando os gols em casa de a mais os gols fora de b somam 0
# (mistura os dois registros no teste; o resto compara os totais de a e de b); mantido.
# Comparadores: match_record_has_more_goals pelo valor; format_duration_text lado a lado sobre a saída (até 256
# bytes); init_loan_offer lado a lado sobre os 0x41 bytes da oferta.
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('match_record_has_more_goals', 0x7e4be0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('format_duration_text', 0x7e5780, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('init_loan_offer', 0x8e7810, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('comparar registros de partida por gols, uma duração em palavras e a oferta de empréstimo nova',
        'comparing match records by goals, a duration in words and a new loan offer',
        [('match_record_has_more_goals', 'match_record_has_more_goals', '0x7e4be0', 'record_utils.cpp', 'cdecl',
          'se o primeiro registro de partida tem mais gols no total que o segundo: registro faltando, a caixa de erro (linha 0x107) e 0; quando os gols em casa do primeiro mais os gols fora do segundo somam 0, 0 (o original mistura os dois registros nesse teste); senão os gols em casa + fora do primeiro acima dos do segundo',
          'whether the first match record has more goals in all than the second: a missing record, the error box (line 0x107) and 0; when the first one\'s home goals plus the second one\'s away goals make 0, 0 (the original mixes the two records in this check); otherwise the first one\'s home + away goals above the second one\'s'),
         ('format_duration_text', 'format_duration_text', '0x7e5780', 'record_utils.cpp', 'cdecl',
          'escreve um número de dias em palavras: saída com menos de 12 bytes, a caixa de erro (linha 0x288) e 0; menos de um dia, texto vazio; aproximado: 1 "um dia", menos de 6 "alguns dias", menos de 9 "cerca de uma semana", menos de 14 "cerca de dez dias", menos de 28 "cerca de N semanas" (dias / 7), menos de 40 "cerca de um mês", menos de 365 "cerca de N meses" (dias / 30 + 1), menos de 450 "cerca de um ano", menos de 630 "cerca de 18 meses", menos de 910 "cerca de 2 anos", senão "cerca de N anos" (dias / 365); exato: menos de 14 "1 dia" ou "N dias", menos de 43 "1 semana" ou "N semanas" (dias / 7), senão "1 mês" ou "N meses" ((dias − 15) / 30 + 1); textos traduzidos; devolve 1',
          'writes a number of days in words: an output under 12 bytes, the error box (line 0x288) and 0; under a day, an empty text; approximate: 1 "one day", under 6 "a few days", under 9 "about a week", under 14 "about ten days", under 28 "about N weeks" (days / 7), under 40 "about a month", under 365 "about N months" (days / 30 + 1), under 450 "about a year", under 630 "about 18 months", under 910 "about 2 years", otherwise "about N years" (days / 365); exact: under 14 "1 day" or "N days", under 43 "1 week" or "N weeks" (days / 7), otherwise "1 month" or "N months" ((days − 15) / 30 + 1); translated texts; returns 1'),
         ('init_loan_offer', 'TRANSFER_OFFER::init_loan_offer', '0x8e7810', 'transfer_offer.cpp', 'thiscall, ret 8',
          'zera uma oferta para o empréstimo de uma pessoa a um clube: sem a pessoa ou o clube, a caixa de erro (linha 0x485); senão o jogador não sabe dela, sem lista de troca, cargo 11, os ids do clube comprador e da pessoa, sem valor, as cláusulas vazias com 10 jogos (de clube e de seleção), série e negociação −1, sem percentual de venda nem parcelas, estado 0, sem termos, rodada de resposta nem pré-contrato, tipo empréstimo (1) e o meio byte baixo do momento da conclusão zerado',
          "resets an offer for a loan of a person to a club: without the person or the club, the error box (line 0x485); otherwise the player is not aware, no exchange list, job 11, the buying club's and the person's ids, no fee, empty clauses with 10 games (club and international), serial and negotiation −1, no sell-on or instalments, status 0, no terms, response round or pre-contract, a loan (1) and the completion timing's low nibble cleared")])
INTERESSA = ['Tweaks']
