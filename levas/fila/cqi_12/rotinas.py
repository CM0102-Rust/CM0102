# As regras da fase de grupos das copas: a Copa da Itália, a Copa da Itália da Série C e a Copa dos Campeões do Brasil
# (ita_cup.cpp, ita_c_cup.cpp, bra_champ_cup.cpp; o mesmo slot, a virtual +0x8c set_up_rules, em três copas). Frente
# das competições (Agente D, 03/11/2026), carreira de 8 ligas.
# - 0x63f350 (ItaCup::set_up_rules, 135 bytes; 1 chamada até 2002-06-01 com as 8 ligas, na montagem da copa): turno
#   único, 3/1 pontos, categoria 4, desempate (4, 1, 2, 0), um classificado por grupo, prêmios 800.000. É a rotina de
#   duas mutações.
# - 0x63d6e0 (ItaCCup::set_up_rules, 135 bytes; 1 chamada): as mesmas regras, prêmios 300.000. Uma mutação.
# - 0x426230 (BraChampCup::set_up_rules, 139 bytes; 2 chamadas): turno único, 3/1, categoria 0x17, desempate (3, 1, 2,
#   4), dois classificados, os bits de regra 8, sem prêmio.
# A cópia é um ajudante comum (cqi12_regras) com as regras de cada copa; cada endereço tem o seu Prologue. Diferenças
# byte a byte conferidas: a 0x63d6e0 só muda o imediato do prêmio (0x493e0 × 0xc3500); a 0x426230 grava +0xc4 = 0x17,
# +0xc8 = 4, +0xc5 = 3, +0xbe = 2, +0xd9 = 8 e não grava +0x28.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual +0x8c (mov ecx, copa; sem push -> thiscall, ret 0;
# devolvem 1). Dentro: a virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push −1 (ecx = −1 da original); mov ecx, this
# -> thiscall, ret 0x10), cuja devolução vai para +0xba.
# Comparadores: só a original (a virtual +0x3c aloca a tabela das rodadas), com o plano: os campos de regra e a
# devolução 1.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas (03-04/11/2026, cópia fora do projeto: motor.rs da principal com a 272a + os 9 itens cqi_10..12,
# cqj_10..12, cqk_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01, de novo depois do aviso do agente A
#   (o histórico nas regiões da cqi_11), também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 82940ff23e3d1c6d; todos os contadores das mutações rodaram;
# - mutações: as três ACUSADAS.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqi12_copa_italia_regras', 0x63f350, [0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x04]),
           ('cqi12_copa_c_regras', 0x63d6e0, [0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x04]),
           ('cqi12_copa_dos_campeoes_regras', 0x426230, [0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x03])]
DOCS = ('as regras da fase de grupos da Copa da Itália, da Copa da Série C e da Copa dos Campeões do Brasil',
        'the group stage rules of the Italian Cup, the Serie C Cup and the Brazilian Champions\' Cup',
        [('cqi12_copa_italia_regras', 'ItaCup::set_up_rules', '0x63f350', 'ita_cup.cpp', 'thiscall, ret 0',
          'regra: os grupos da Copa da Itália são de turno único, 3 pontos a vitória e 1 o empate; o desempate pelos códigos 4, 1 e 2 [inferência: confronto direto, saldo, gols pró]; um classificado por grupo (o primeiro); a categoria 4 (copa nacional) em cada jogo; sem acesso nem rebaixamento; fundo de prêmios de 800.000',
          'rule: the Italian Cup groups are single round robin, 3 points a win and 1 a draw; tiebreak by codes 4, 1 and 2 [inference: head-to-head, goal difference, goals for]; one qualifier per group (the winner); category 4 (national cup) on each match; no promotion or relegation; prize fund of 800,000'),
         ('cqi12_copa_c_regras', 'ItaCCup::set_up_rules', '0x63d6e0', 'ita_c_cup.cpp', 'thiscall, ret 0',
          'regra: os grupos da Copa da Itália da Série C seguem as regras da Copa da Itália (turno único, 3/1, desempate 4, 1, 2, um classificado por grupo); fundo de prêmios de 300.000',
          'rule: the Serie C Italian Cup groups follow the Italian Cup rules (single round robin, 3/1, tiebreak 4, 1, 2, one qualifier per group); prize fund of 300,000'),
         ('cqi12_copa_dos_campeoes_regras', 'BraChampCup::set_up_rules', '0x426230', 'bra_champ_cup.cpp', 'thiscall, ret 0',
          'regra: o grupo da Copa dos Campeões do Brasil é de turno único, 3 pontos a vitória e 1 o empate; desempate pelos códigos 3, 1, 2 e 4 [inferência: vitórias, saldo, gols pró, confronto direto]; dois classificados; os bits de regra 8 (quem garante as vagas vai à fase seguinte); a categoria 23 em cada jogo (a das finais de campeões); sem fundo de prêmios',
          'rule: the Brazilian Champions\' Cup group is single round robin, 3 points a win and 1 a draw; tiebreak by codes 3, 1, 2 and 4 [inference: wins, goal difference, goals for, head-to-head]; two qualifiers; rule bits 8 (those who secure the places go to the next stage); category 23 on each match (the champions\' finals one); no prize fund')])
CAMPOS = [('liga do grupo +0xc2/+0xc3 pontos por vitória/empate, +0xc4 categoria do jogo, +0xc5..+0xc8 desempates, +0xbe..+0xc1 vagas (acesso, play-offs, rebaixamento), +0x1c/+0x20 ligas de acesso/queda (−1), +0x3c turnos, +0x42 tipo (1 clubes), +0x28 fundo de prêmios, +0xd9 bits de regra, +0xba calendário; valor/ponteiro; grava (as três)',
           'group league +0xc2/+0xc3 points per win/draw, +0xc4 match category, +0xc5..+0xc8 tiebreaks, +0xbe..+0xc1 places (promotion, play-offs, relegation), +0x1c/+0x20 promotion/relegation leagues (−1), +0x3c rounds, +0x42 type (1 clubs), +0x28 prize fund, +0xd9 rule bits, +0xba schedule; value/pointer; writes (all three)')]
INTERESSA = ['CMRTE']
