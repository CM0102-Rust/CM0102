# A rodada da copa (Cup.cpp, o que restava fora da fila): Cup::pay_round_prize_money (0x522240, 574 chamadas na
# carreira até 2002-06-01, pelo processamento da partida da copa 0x51a150: a FA Cup paga 10 mil ao vencedor de cada
# rodada, +0x60 = 0x2710 em 0x56f44a; a Copa da Liga paga do fundo, +0x60 = −1 em 0x56c6b5), Cup::set_home_teams_for_round
# (0x51e870, 133, pelo sorteio da rodada 0x51c410 e pela 0x51a150) e Cup::record_team_progress (0x521b30, 1.148, pela
# 0x51a150). Sem SEH, sem caixa (a 0x5a0590 só tem caixa no erro), sem sorteio. Iguais no cm0102-1080.exe (corpo
# inteiro). Os itens antigos cpb_2/cpb_3 (26/10, nunca integrados, com a escuta da cpb_1) tinham a 0x521b30, a 0x51e870
# e a 0x522240: este item as refaz sem a escuta, no critério de 31/10.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x48e3e0 (Comp::add_prize_money_news): 7 pushes (desfecho, partida, seguinte, −1, alto do valor, valor, clube);
#   ecx = a copa; sem add esp (ret 0x1c) -> thiscall de 7 argumentos.
# - 0x5a0590 (FinanceTable::get_club_finances): push valor (fica para a 0x5999a0); push clube; ecx = [0xae2a38]; sem
#   add esp -> thiscall(tabela, clube), ret 4.
# - 0x5999a0: ecx = o devolvido; o valor já na pilha -> thiscall(finanças, valor), ret 4.
# - virtual +0x90 (club_in_higher_division): push clube b; push clube a; ecx = a copa; sem add esp -> thiscall(copa,
#   a, b), ret 8.
# - virtual +0x68 (set_club_progress): 5 pushes (p3, p2, p1, comp +0x44, clube); ecx = a copa; sem add esp ->
#   thiscall(copa, clube, comp, p1, p2, p3), ret 0x14.
# - 0x4a4850 (no comparador da 0x521b30, como o da 0x68ac40): push clube; ecx = Comp_Stats -> thiscall, ret 4.
# Comparadores: a 0x522240 em ensaio (modelo CPV6_ENSAIO: a cópia sem a notícia, que aloca; confere o fundo +0x28 e os
# registros de finanças dos dois clubes); a 0x51e870 lado a lado nos confrontos percorridos; a 0x521b30 lado a lado nos
# 3 bytes de progresso do registro do clube no Comp_Stats da copa.
SAVE = False
ROTINAS = [('copa_premio_da_rodada', 0x522240, [0x83, 0xec, 0x08, 0x8b, 0x44, 0x24, 0x0c]),
           ('copa_mandantes_da_rodada', 0x51e870, [0x8b, 0x44, 0x24, 0x08, 0x83, 0xec, 0x08]),
           ('copa_progresso_do_time', 0x521b30, [0x66, 0x8b, 0x41, 0x3c, 0x53])]
DOCS = ('a rodada da copa: o prêmio da rodada, os mandantes da rodada e o progresso do time',
        'the cup round: the round prize money, the home teams of the round and the team progress',
        [('copa_premio_da_rodada', 'Cup::pay_round_prize_money', '0x522240', 'Cup.cpp', 'thiscall, ret 0x10',
          'paga o prêmio da rodada (+0x60 ao vencedor, +0x64 ao perdedor; com −1, do fundo +0x28 pelas rodadas que faltam e pelos confrontos, 3/4 e 1/4 quando os dois são −1): o vencedor com a notícia (0x48e3e0) e o dinheiro (0x5999a0), o perdedor só o dinheiro; o fundo perde os dois',
          'pays the round prize money (+0x60 to the winner, +0x64 to the loser; with -1, from the fund +0x28 over the remaining rounds and the ties, 3/4 and 1/4 when both are -1): the winner with the news item (0x48e3e0) and the money (0x5999a0), the loser only the money; the fund loses both'),
         ('copa_mandantes_da_rodada', 'Cup::set_home_teams_for_round', '0x51e870', 'Cup.cpp', 'thiscall, ret 8',
          'inverte mandante e visitante num confronto ou em todos os da rodada pela regra da rodada: o clube de divisão mais alta (virtual +0x90), o nível do time ou o estádio da copa',
          'swaps home and away in one tie or in every tie of the round by the round rule: the club in the higher division (virtual +0x90), the team level or the cup stadium'),
         ('copa_progresso_do_time', 'Cup::record_team_progress', '0x521b30', 'Cup.cpp', 'thiscall, ret 8',
          'quanto o time foi na copa, pelo desfecho e pela rodada (com ou sem a rodada do terceiro lugar), entregue à virtual +0x68 (set_club_progress) com a competição +0x44',
          'how far the team went in the cup, by outcome and round (with or without the third place round), handed to virtual +0x68 (set_club_progress) with competition +0x44')])
CAMPOS = [('Cup +0x28: o fundo de prêmios; valor; lê e grava (copa_premio_da_rodada)',
           'Cup +0x28: the prize fund; value; reads and writes (copa_premio_da_rodada)'),
          ('Cup +0x3c / +0xa3: as rodadas e a tabela delas (0x68 bytes: +7 tipo, +0xb regra, +0x1a confrontos, +0x5b progresso, +0x60/+0x64 prêmios); valor e ponteiro; lê (as três)',
           'Cup +0x3c / +0xa3: the rounds and their table (0x68 bytes: +7 type, +0xb rule, +0x1a ties, +0x5b progress, +0x60/+0x64 prizes); value and pointer; reads (all three)'),
          ('Cup +0xa7: os times (6 bytes: +0 clube, +4 nível, +5 desfecho); ponteiro; lê (as três)',
           'Cup +0xa7: the teams (6 bytes: +0 club, +4 level, +5 outcome); pointer; reads (all three)'),
          ('Cup +0xad: os confrontos de cada rodada (0x35 bytes: +0/+2 times, +0xa/+0xc, +0xe/+0x10); ponteiro; lê e grava (copa_mandantes_da_rodada)',
           'Cup +0xad: the ties of each round (0x35 bytes: +0/+2 teams, +0xa/+0xc, +0xe/+0x10); pointer; reads and writes (copa_mandantes_da_rodada)'),
          ('Cup +0x24 / +0x44: o estádio da copa (o id, comparado com o do estádio +0x69 do clube) e a competição passada à +0x68; id; lê (copa_mandantes_da_rodada, copa_progresso_do_time)',
           'Cup +0x24 / +0x44: the cup stadium (the id, compared with the club stadium +0x69) and the competition passed to +0x68; id; reads (copa_mandantes_da_rodada, copa_progresso_do_time)'),
          ('partida +0x30: a rodada da partida (0x136 troca o desfecho 1 por 3 na notícia); valor; lê (copa_premio_da_rodada)',
           'fixture +0x30: the fixture round (0x136 turns outcome 1 into 3 in the news); value; reads (copa_premio_da_rodada)'),
          ('finanças do clube (0x5a0590): saldo, +0xa8, +0x148; valor; soma pela 0x5999a0 (copa_premio_da_rodada)',
           'club finances (0x5a0590): balance, +0xa8, +0x148; value; adds through 0x5999a0 (copa_premio_da_rodada)')]
INTERESSA = ['CMRTE', 'SGE']
