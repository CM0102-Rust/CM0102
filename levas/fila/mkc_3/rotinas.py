# O período do empréstimo das regras galesa e russa e o prazo dos empréstimos da regra turca (wales_rules.cpp,
# russia_rules.cpp, turkey_rules.cpp), fechamento do mapa do mercado (levas/reorganizacao/mercado/fechamento_mkc.md,
# 11/11/2026). O mapa (secao_c, §1) dava as três como "fora: não rodam nas 8 ligas"; as tabelas virtuais das 29 regras
# (achadas pelo slot +0x14 = 0x40a5f0, cada uma pelo construtor que a grava) mostram que rodam, sim, nas 8 ligas:
# - 0x90f1a0 (WALES_RULES::get_loan_period): o virtual +0x28 de 11 nações, entre elas a França (tabela 0x96ae48) e a
#   Austrália (0x967830); pela 0x8d0590 com a nação do clube que pega emprestado.
# - 0x7ec8e0 (RUSSIA_RULES::get_loan_period): o virtual +0x28 do Brasil (0x968838), da Polônia e da Rússia.
# - 0x902240 (TURKEY_RULES::can_loan_player): o virtual +0x20 da Turquia (0x97075c) e dos EUA (0x970a38); pela
#   0x8d0040 e pela 0x8c2180.
# Iguais no cm0102-1080.exe (as três inteiras, conferido byte a byte).
# Convenções conferidas na desmontagem: os dois períodos thiscall ret 0x1c (os sete argumentos da 0x8d0590; o clube
# em [esp+0xc] depois de dois push, as saídas em [esp+0x14..0x20]); dentro 0x54cfb0 mov ecx, 0xae2c90 -> thiscall
# ret 0 e 0x4d45f0 push clube; mov ecx, [0xae1050] -> thiscall ret 4. 0x902240 thiscall ret 0x10; dentro 0x8e8e50
# push 0; push 0xae2c90; ecx = regra -> thiscall ret 8; 0x4cf5a0 push jogador; mov ecx, [0xae1050] -> thiscall ret 4
# (o resultado em [0xae1080]); 0x6181d0 push 2; push 0x64; push buffer; push nação; add esp, 0x10 -> cdecl; 0x66f4e0
# cdecl variádica.
# Comparadores: lado a lado; os períodos nas quatro saídas; a turca pelo comparador dos can_loan_player ingleses
# (retorno, motivo, [0xae1080] e texto).
# Determinismo: nada lê pilha ou heap como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('wales_rules_get_loan_period', 0x90f1a0, [0x56, 0x57, 0xb9, 0x90, 0x2c, 0xae, 0x00]),
           ('russia_rules_get_loan_period', 0x7ec8e0, [0x56, 0x57, 0xb9, 0x90, 0x2c, 0xae, 0x00]),
           ('turkey_rules_can_loan_player', 0x902240, [0x83, 0xec, 0x64, 0x56, 0x6a, 0x00])]
DOCS = ('o período do empréstimo das regras galesa (França, Austrália e mais 9) e russa (Brasil, Polônia, Rússia); o prazo dos empréstimos da regra turca (Turquia e EUA)',
        'the loan period of the Welsh rules (France, Australia and 9 more) and of the Russian rules (Brazil, Poland, Russia); the loan deadline of the Turkish rules (Turkey and the USA)',
        [('wales_rules_get_loan_period', 'WALES_RULES::get_loan_period', '0x90f1a0', 'wales_rules.cpp', 'thiscall, ret 0x1c',
          'regra: o empréstimo para um clube de Austrália, Bélgica, Croácia, França, Alemanha, Grécia, Holanda, Irlanda, Coreia, Turquia ou País de Gales começa e termina no mês do fim da temporada da nação do clube (este ano se ainda está à frente do mês seguinte, senão no ano seguinte): início = fim, que o can_loan_player aceita na hora',
          'rule: the loan to a club of Australia, Belgium, Croatia, France, Germany, Greece, Holland, Ireland, Korea, Turkey or Wales starts and ends in the season end month of the club\'s nation (this year if still ahead of next month, otherwise next year): start = end, which can_loan_player accepts at once'),
         ('russia_rules_get_loan_period', 'RUSSIA_RULES::get_loan_period', '0x7ec8e0', 'russia_rules.cpp', 'thiscall, ret 0x1c',
          'regra: o empréstimo para um clube do Brasil, da Polônia ou da Rússia vai do mês de hoje + 3 ao mês do fim da temporada da nação do clube (no ano do início se ainda está à frente dele, senão no ano seguinte a hoje); defeito: começando no ano seguinte (outubro a dezembro) com a temporada acabando em janeiro ou fevereiro, o fim fica antes do início',
          'rule: the loan to a club of Brazil, Poland or Russia runs from today\'s month + 3 to the season end month of the club\'s nation (in the start\'s year if still ahead of it, otherwise the year after today); defect: starting next year (October to December) with the season ending in January or February, the end lands before the start'),
         ('turkey_rules_can_loan_player', 'TURKEY_RULES::can_loan_player', '0x902240', 'turkey_rules.cpp', 'thiscall, ret 0x10',
          'regra: um clube da Turquia ou dos EUA só pega emprestado com uma janela da sua nação aberta hoje (EUA: 7 de janeiro a 15 de agosto); fora dela, o motivo 9 e "The deadline for loans to ... clubs has passed."',
          'rule: a club of Turkey or the USA only takes a player on loan with a window of its nation open today (USA: 7 January to 15 August); outside it, reason 9 and "The deadline for loans to ... clubs has passed."')])
CAMPOS = [('regras de transferência +0xa (NATION_ID): a nação das regras, para o adjetivo do texto; ponteiro; lê',
           'transfer rules +0xa (NATION_ID): the rules\' nation, for the adjective of the text; pointer; reads'),
          ('[0xae1080] (G_CONTRACT_LOOKUP_RESULT): o contrato principal do jogador, guardado pela turca com a janela aberta; escreve',
           '[0xae1080] (G_CONTRACT_LOOKUP_RESULT): the player\'s main contract, kept by the Turkish rule with the window open; writes')]
INTERESSA = ['Tweaks']
