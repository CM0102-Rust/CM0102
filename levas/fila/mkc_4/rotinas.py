# As regras de empréstimo da Finlândia (finland_rules.cpp), fechamento do mapa do mercado
# (levas/reorganizacao/mercado/fechamento_mkc.md, 11/11/2026). As três só servem à Finlândia (tabela virtual 0x96a75c;
# nenhuma outra das 29 tabelas das regras aponta para elas): não rodam na carreira de 8 ligas. A prova é pela
# comparação numa carreira com a liga finlandesa carregada (LIGAS abaixo; o simular.py aceita --ligas com os códigos de
# 3 letras do nation.dat +0x53: FIN).
# - 0x5a9a80 (FINLAND_RULES::get_loan_period): o virtual +0x28 pela 0x8d0590 (clube finlandês pegando emprestado).
# - 0x5a9af0 (FINLAND_RULES::can_loan_player): o virtual +0x20 pela 0x8d0040 e pela 0x8c2180.
# - 0x5a9ba0 (FINLAND_RULES::can_end_loan): o virtual +0x24 pela 0x8d0530 (telas 0x8fa341 com flag 1 e 0x8fbcf9 com
#   flag 0; pedido de empréstimo da IA 0x8429ae, flag 1) e pela 0x8d0040 (cada mês do empréstimo, flag 1).
# Iguais no cm0102-1080.exe (as três inteiras, conferido byte a byte).
# Convenções conferidas na desmontagem: 0x5a9a80 thiscall ret 0x1c; 0x5a9af0 thiscall ret 0x10 (o clube em
# [esp+0x6c] depois de sub esp, 0x64; push esi); 0x5a9ba0 thiscall ret 0x14 com quadro SEH sem destrutor (as datas
# locais). Dentro: 0x54cfb0 mov ecx, 0xae2c90 -> thiscall ret 0; 0x4d45f0 push clube; mov ecx, [0xae1050] -> thiscall
# ret 4; 0x8e8e50 push 0; push 0xae2c90; ecx = regra -> thiscall ret 8; 0x600a50 push 0; push 0; push 1; push 1;
# push clube; add esp, 0x14 -> cdecl; 0x549f70 push -1; push ano (só a palavra baixa é lida: and ecx, 0xffff);
# push mês; push dia; lea ecx, local -> thiscall ret 0x10; 0x6181d0 cdecl; 0x66f4e0 cdecl variádica.
# Comparadores: lado a lado; o período nas quatro saídas; a can_loan_player pelo comparador dos ingleses (retorno,
# motivo, [0xae1080] e texto); a can_end_loan no retorno e no texto da mensagem (a cópia começa com o primeiro byte
# zerado e no fim volta o texto da original).
# Determinismo: nada lê pilha ou heap como valor (as datas locais são preenchidas pela 0x549f70).
LIGAS = 'ENG FIN NOR NIR GER IRL'
SAVE = False
ROTINAS = [('finland_rules_get_loan_period', 0x5a9a80, [0x56, 0x57, 0xb9, 0x90, 0x2c, 0xae, 0x00]),
           ('finland_rules_can_loan_player', 0x5a9af0, [0x83, 0xec, 0x64, 0x56, 0x6a, 0x00]),
           ('finland_rules_can_end_loan', 0x5a9ba0, [0x6a, 0xff, 0x68, 0x68, 0x95, 0x95, 0x00])]
DOCS = ('as regras de empréstimo da Finlândia: o período, o prazo e o outono sem devolução',
        'the Finnish loan rules: the period, the deadline and the autumn without returns',
        [('finland_rules_get_loan_period', 'FINLAND_RULES::get_loan_period', '0x5a9a80', 'finland_rules.cpp', 'thiscall, ret 0x1c',
          'regra: o empréstimo para um clube finlandês vai do mês seguinte ao mês do fim da temporada da nação do clube, neste ano se esse mês ainda está à frente do início, senão no ano seguinte (o mesmo resultado da regra base)',
          'rule: the loan to a Finnish club runs from next month to the season end month of the club\'s nation, this year if that month is still ahead of the start, otherwise next year (the same result as the base rule)'),
         ('finland_rules_can_loan_player', 'FINLAND_RULES::can_loan_player', '0x5a9af0', 'finland_rules.cpp', 'thiscall, ret 0x10',
          'regra: o clube finlandês só pega emprestado com a janela finlandesa aberta hoje (16 de novembro a 15 de agosto; fora dela, o motivo 9 e "The deadline for loans to Finnish clubs has passed."); o clube da IA, além disso, só entre os dias 90 e 300 do ano (1º de abril a 28 de outubro), recusando sem motivo nem texto',
          'rule: a Finnish club only takes a player on loan with the Finnish window open today (16 November to 15 August; outside it, reason 9 and "The deadline for loans to Finnish clubs has passed."); an AI club, moreover, only between days 90 and 300 of the year (1 April to 28 October), refused with no reason and no text'),
         ('finland_rules_can_end_loan', 'FINLAND_RULES::can_end_loan', '0x5a9ba0', 'finland_rules.cpp', 'thiscall, ret 0x14',
          'regra: o empréstimo com um clube finlandês não termina (nem devolvido, nem chamado de volta) numa data estritamente entre 18 de agosto e 15 de novembro do ano da data: "This player cannot be sent back/recalled until 15th November"',
          'rule: a loan with a Finnish club does not end (neither sent back nor recalled) on a date strictly between 18 August and 15 November of the date\'s year: "This player cannot be sent back/recalled until 15th November"')])
CAMPOS = [('regras de transferência +0xa (NATION_ID): a nação das regras, para o adjetivo do texto; ponteiro; lê',
           'transfer rules +0xa (NATION_ID): the rules\' nation, for the adjective of the text; pointer; reads'),
          ('staff +0x39 (club_job): o clube do jogador, para escolher "devolver" ou "chamar de volta"; ponteiro; lê',
           'staff +0x39 (club_job): the player\'s club, to choose "send back" or "recall"; pointer; reads'),
          ('[0xae2c90] (G_CURRENT_DATE): o dia do ano de hoje (palavra com sinal, 0 = 1º de janeiro); lê',
           '[0xae2c90] (G_CURRENT_DATE): today\'s day of the year (signed word, 0 = 1 January); reads')]
INTERESSA = ['Tweaks']
