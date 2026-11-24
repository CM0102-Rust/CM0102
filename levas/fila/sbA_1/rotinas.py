# O passe diario das financas (fin_prm.cpp pelo mapa, finance.CPP pelos asserts; leitor sbA, rodada 5, 24/11/2026).
# Carreira ENG (sem LIGAS). Roda todo dia, pelo laco diario do jogo.
# - 0x59b9c0 FinanceTable::daily_update thiscall(hoje) ret 4: nada antes da data de inicio do jogo; em 16/05/2004 as
#   mudancas de estadio (Brighton para Falmer, Everton para Kings Dock; so a partir de 2004, sem mutacao); por clube ate
#   a primeira selecao: o clube sem livros (sem divisao, ou liga nao escolhida e reputacao < 5000, ou a divisao-chave
#   0x9cf764) tem o saldo preso numa faixa pelo status profissional (+0x64); o com livros paga a semana no domingo (dia
#   da semana 6: folha 0x598370, mercadoria 0x59b7e0, carnes 0x59b4c0/0x599a20); no ultimo dia do mes o fechamento do
#   mes (inquietacao 0x74ae60 e 0x867700, estadio, centro de treino, bilheteria, juros, custos e patrocinio, plc,
#   manutencao, eventos 0x59ed70 com 1 e a virada do bloco do mes); nos outros dias, os clubes em crise (+0x165 1 ou 2)
#   rodam os eventos 0x59ed70, com semana em 1 de 10.
# Convencoes conferidas: Date::Date 0x549f70 (thiscall, 4 args), Date::add_days 0x54c5b0 (thiscall(&saida, 1)),
# Date::get_month 0x54cfb0, Date::get_weekday 0x54a3f0, club_get_linked_b_team 0x540a50 (cdecl 3), utils_random
# 0x90cfc0 (cdecl), _ftol 0x945640, staff_is_disliked_by/favoured_by 0x547840/0x547810 (cdecl(tecnico, pessoa)),
# get_contract_for_club 0x4cf5f0 (thiscall(pessoa, clube)), apply_event_flags 0x867700 (thiscall(0x400000, 0)),
# find_fixtures 0x5acbe0 (thiscall, 9 args, ret 0x24). Prologo push -1; push 0x959057 (7 bytes, sem relativa). Igual
# no cm0102-1080.exe (inteira).
# Defeito do original: em 16/05/2004 a mudanca para Kings Dock procura os jogos do Southampton ([0x9d0260]), nao os do
# Everton: os jogos em casa do Everton ficam no estadio velho.
# Comparador: so a original (a rotina manda noticias, mexe em contratos e roda os eventos de todos os clubes); no fim de
# mes confere que o ultimo clube que o passe do mes alcanca ficou com o bloco do mes (+0x70, 0x50 bytes) zerado.
# O FINANCE_SIZE (0x167) fica aqui; o sbA_2 o usa (integrar este antes).
SAVE = False
ROTINAS = [('finance_daily_update', 0x59b9c0, [0x6a, 0xff, 0x68, 0x57, 0x90, 0x95, 0x00])]
DOCS = ('o passe diário das finanças',
        'the daily finance pass',
        [('finance_daily_update', 'FinanceTable::daily_update', '0x59b9c0', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 4',
          'o passe diário das finanças de todos os clubes: o saldo dos clubes sem livros fica numa faixa pelo status profissional (2: ±250 mil com reputação acima de 2500, senão ±25 mil; 3: de −10 mil a 2500; os outros ±2,25 milhões, e passando do limite um sorteio de 5 pode virar o saldo para o outro lado, 175 mil ou −175 mil); no domingo os clubes com livros pagam a folha, recebem a mercadoria e vendem carnês até a temporada começar; no fim do mês cada clube com livros paga as coberturas e o aquecimento do estádio, o centro de treino por nível, os juros (0,25% ao mês sobre saldo positivo; 1,2%, 1,8% ou 2,4% sobre o negativo, pela reputação) e os custos da diretoria, recebe o patrocínio, a renda de plc e, numa divisão sem objeto de competição, a bilheteria, paga a manutenção do estádio pela capacidade e pelo estado financeiro, roda os eventos do mês e vira o bloco do mês; nos outros dias os clubes em crise rodam os eventos, com semana em 1 de 10; em 16/05/2004 o Brighton vai para Falmer e o Everton para Kings Dock',
          "every club's daily finance pass: the balance of a club without books stays in a band by its professional status (2: ±250k with reputation above 2500, else ±25k; 3: from −10k to 2500; others ±2.25m, and past the edge a draw of 5 may swing it to the other side, 175k or −175k); on Sundays clubs with books pay wages, take the merchandising and sell season tickets until the season starts; at the month end each club with books pays the stadium's covered stands and heating, the training ground by level, the interest (0.25% a month on a positive balance; 1.2%, 1.8% or 2.4% on a negative one, by reputation) and the board's costs, receives the sponsorship, the plc income and, in a division without a competition object, the gate, pays the ground maintenance by capacity and financial state, runs the month's events and rolls the month block over; on other days clubs in crisis run their events, with a week 1 time in 10; on 16/05/2004 Brighton moves to Falmer and Everton to the Kings Dock")])
CAMPOS = [('financas (+0x84/+0x124 juros pagos, +0xb0/+0x150 juros recebidos, +0xa4/+0x144 patrocinio, +0x20 o bloco do mes anterior) [inferencia pelo passe do mes]; escreve (finance_daily_update)',
           'finance (+0x84/+0x124 interest paid, +0xb0/+0x150 interest received, +0xa4/+0x144 sponsorship, +0x20 last month block) [inference from the month pass]; writes (finance_daily_update)')]
INTERESSA = []
