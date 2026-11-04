# As regras dos grupos continentais de clubes: a Libertadores, a Copa Merconorte e o Campeonato de Clubes da Oceania (o
# mesmo slot, a virtual +0x8c set_up_rules; conmebol_liber.cpp, con_merc_cup.cpp, oceania_club_champ.cpp). Frente das
# competições (Agente D, 03/11/2026), carreira de 8 ligas (Brasil e Austrália jogáveis).
# - 0x4c3890 (ConmebolLiber::set_up_rules, 226 bytes; 2 chamadas até 2002-06-01 com as 8 ligas): os grupos da
#   Libertadores. Rotina de duas mutações.
# - 0x4c1ed0 (ConMercCup::set_up_rules, 221 bytes; 2 chamadas): os grupos da Copa Merconorte.
# - 0x7979b0 (OceaniaClubChamp::set_up_rules, 214 bytes; 1 chamada): os grupos do Campeonato de Clubes da Oceania. Uma
#   mutação.
# As três têm quadro SEH só pela Date local (sem destrutor; a original grava o estado 0 do quadro): a cópia não monta o
# quadro. O porte é um ajudante comum (cqj11_regras) com a tabela de campos de cada copa; cada endereço tem o seu
# Prologo. Diferenças byte a byte conferidas: a Merconorte não grava +0x49/+0x4a e tem +0xbe = 1 e a data (1, 7); a da
# Oceania grava +0x43, +0xd9 = 0x28, +0xc4 = 8, +0xc7 = 4, turno único, a data (20, 7, dia da semana 5) e não grava
# +0x1c, +0x20, +0x42 nem o prêmio.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual +0x8c (mov ecx, copa; sem push -> thiscall, ret 0;
# devolvem 1). Dentro: 0x549f70 (Date::Date: push semana; push ano; push mês; push dia; lea ecx, local -> thiscall, ret
# 0x10; o ano vem de +0x40 num registrador com lixo nos 16 bits altos na da Oceania: o construtor lê a palavra, a cópia
# passa limpo) e a virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10).
# Comparadores: só a original (a virtual +0x3c aloca a tabela das rodadas), com o plano: os campos, a data em +0xb6
# (calculada antes numa Date local pela própria 0x549f70), +0xb8 = 0, o prêmio e a devolução 1.
# Determinismo: a Date local começa zerada na cópia; o construtor grava o que a rotina lê dela (a primeira palavra).
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
ROTINAS = [('cqj11_libertadores_regras', 0x4c3890, [0x6a, 0xff, 0x68, 0x78, 0x6a, 0x95, 0x00]),
           ('cqj11_merconorte_regras', 0x4c1ed0, [0x6a, 0xff, 0x68, 0xc8, 0x69, 0x95, 0x00]),
           ('cqj11_oceania_clubes_regras', 0x7979b0, [0x6a, 0xff, 0x68, 0xe8, 0xea, 0x95, 0x00])]
DOCS = ('as regras dos grupos da Libertadores, da Copa Merconorte e do Campeonato de Clubes da Oceania',
        'the group rules of the Copa Libertadores, the Copa Merconorte and the Oceania Club Championship',
        [('cqj11_libertadores_regras', 'ConmebolLiber::set_up_rules', '0x4c3890', 'conmebol_liber.cpp', 'thiscall, ret 0',
          'regra: os grupos da Libertadores são de ida e volta (dois turnos), 3 pontos a vitória e 1 o empate, desempate pelos códigos 1, 2 e 3 [inferência: saldo, gols pró, vitórias]; dois classificados por grupo; clubes de várias nações; categoria 7 em cada jogo; 7 reservas no banco e 3 substituições; começa no dia 15 do mês 0 do ano da competição [inferência: 15 de janeiro, meses contados de 0]; fundo de prêmios de 2.000.000',
          'rule: the Libertadores groups are home and away (double round robin), 3 points a win and 1 a draw, tiebreak by codes 1, 2 and 3 [inference: goal difference, goals for, wins]; two qualifiers per group; clubs from several nations; category 7 on each match; 7 substitutes on the bench and 3 changes; starts on day 15 of month 0 of the competition year [inference: 15 January, months counted from 0]; prize fund of 2,000,000'),
         ('cqj11_merconorte_regras', 'ConMercCup::set_up_rules', '0x4c1ed0', 'con_merc_cup.cpp', 'thiscall, ret 0',
          'regra: os grupos da Copa Merconorte são de ida e volta, 3/1 pontos, desempate 1, 2, 3, um classificado por grupo, categoria 7, começo no dia 1 do mês 7 [inferência: 1º de agosto], prêmios de 2.000.000',
          'rule: the Copa Merconorte groups are home and away, 3/1 points, tiebreak 1, 2, 3, one qualifier per group, category 7, starts on day 1 of month 7 [inference: 1 August], prize fund of 2,000,000'),
         ('cqj11_oceania_clubes_regras', 'OceaniaClubChamp::set_up_rules', '0x7979b0', 'oceania_club_champ.cpp', 'thiscall, ret 0',
          'regra: os grupos do Campeonato de Clubes da Oceania são de turno único, 3/1 pontos, desempate pelos códigos 1, 2 e 4, dois classificados por grupo (os de cima seguem, os outros estão eliminados: bits de regra 0x28), categoria 8 (a das fases decisivas), 3 reservas no banco e 3 substituições; começa no dia 20 do mês 7 levado a um dia da semana 5 [inferência]; sem fundo de prêmios',
          'rule: the Oceania Club Championship groups are single round robin, 3/1 points, tiebreak by codes 1, 2 and 4, two qualifiers per group (the top ones go on, the others are out: rule bits 0x28), category 8 (the deciding stages one), 3 substitutes on the bench and 3 changes; starts on day 20 of month 7 moved to weekday 5 [inference]; no prize fund')])
CAMPOS = [('liga do grupo +0xc2/+0xc3 pontos, +0xc4 categoria, +0xc5..+0xc8 desempates, +0xbe..+0xc1 vagas, +0x3c turnos, +0x42 tipo (2 clubes internacionais), +0x43, +0x49 reservas no banco, +0x4a substituições, +0x28 prêmios, +0xd9 bits de regra, +0xb6 a data de começo (a primeira palavra da Date), +0xb8, +0xba calendário; valor/ponteiro; grava (as três)',
           'group league +0xc2/+0xc3 points, +0xc4 category, +0xc5..+0xc8 tiebreaks, +0xbe..+0xc1 places, +0x3c rounds, +0x42 type (2 international clubs), +0x43, +0x49 bench substitutes, +0x4a changes, +0x28 prize fund, +0xd9 rule bits, +0xb6 start date (the first word of the Date), +0xb8, +0xba schedule; value/pointer; writes (all three)'),
          ('competição +0x40 ano; valor; lê (as três)', 'competition +0x40 year; value; reads (all three)')]
INTERESSA = ['CMRTE']
