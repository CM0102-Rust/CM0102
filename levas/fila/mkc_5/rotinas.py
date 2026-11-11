# O período do empréstimo e a regra de empréstimo da Irlanda do Norte, e o prazo dos empréstimos da Noruega
# (northern_ireland_rules.cpp, norway_rules.cpp), fechamento do mapa do mercado
# (levas/reorganizacao/mercado/fechamento_mkc.md, 11/11/2026). Pelas 29 tabelas virtuais das regras (achadas pelo slot
# +0x14 = 0x40a5f0, cada uma pelo construtor que a grava), nenhuma das três roda na carreira de 8 ligas:
# - 0x793870 (NORTHERN_IRELAND_RULES::get_loan_period): o virtual +0x28 só da Irlanda do Norte (0x96df9c);
# - 0x793900 (NORTHERN_IRELAND_RULES::can_loan_player): o virtual +0x20 da Dinamarca (0x969860), da Coreia (0x96cfec),
#   da Irlanda do Norte (0x96df9c) e do País de Gales (0x970a9c);
# - 0x794760 (NORWAY_RULES::can_loan_player): o virtual +0x20 da Noruega (0x96dfe4) e da Holanda (0x96ba88).
# A prova é pela comparação numa carreira com a liga norte-irlandesa e a norueguesa carregadas (LIGAS abaixo; o
# simular.py aceita --ligas com os códigos de 3 letras do nation.dat +0x53: NIR, NOR; também DEN, KOR, WAL, HOL
# exercitam a 0x793900 e a 0x794760).
# Iguais no cm0102-1080.exe (as três inteiras, conferido byte a byte).
# Convenções conferidas na desmontagem: 0x793870 thiscall ret 0x1c (três push; o clube em [esp+0x10], as saídas em
# [esp+0x18..0x24]); dentro 0x54cfb0 mov ecx, 0xae2c90 -> thiscall ret 0 e 0x4d45f0 push clube; mov ecx, [0xae1050]
# -> thiscall ret 4. 0x793900 thiscall ret 0x10 (o jogador em [esp+8]); dentro 0x4cf5a0 push jogador; mov ecx,
# [0xae1050] -> thiscall ret 4 (o resultado em [0xae1080]). 0x794760 thiscall ret 0x10; dentro 0x8e8e50 push 0; push
# 0xae2c90; ecx = regra -> thiscall ret 8; 0x6181d0 cdecl; 0x66f4e0 cdecl variádica.
# Comparadores: lado a lado; o período nas quatro saídas; as duas can_loan_player pelo comparador dos ingleses
# (retorno, motivo, [0xae1080] e texto).
# Determinismo: nada lê pilha ou heap como valor.
LIGAS = 'ENG FIN NOR NIR GER IRL'
SAVE = False
ROTINAS = [('northern_ireland_rules_get_loan_period', 0x793870, [0x53, 0x56, 0x57, 0xb9, 0x90, 0x2c, 0xae, 0x00]),
           ('northern_ireland_rules_can_loan_player', 0x793900, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00]),
           ('norway_rules_can_loan_player', 0x794760, [0x83, 0xec, 0x64, 0x56, 0x6a, 0x00])]
DOCS = ('o período do empréstimo e a regra de empréstimo da Irlanda do Norte (também Dinamarca, Coreia e País de Gales); o prazo dos empréstimos da Noruega (também Holanda)',
        'the Northern Irish loan period and loan rule (also Denmark, Korea and Wales); the Norwegian loan deadline (also Holland)',
        [('northern_ireland_rules_get_loan_period', 'NORTHERN_IRELAND_RULES::get_loan_period', '0x793870', 'northern_ireland_rules.cpp', 'thiscall, ret 0x1c',
          'regra: o empréstimo para um clube norte-irlandês vai do mês de hoje + 3 ao mês do fim da temporada da nação do clube (no ano do início se ainda está à frente dele, senão no ano seguinte a hoje); o início depois do fim vira o fim',
          'rule: the loan to a Northern Irish club runs from today\'s month + 3 to the season end month of the club\'s nation (in the start\'s year if still ahead of it, otherwise the year after today); a start after the end becomes the end'),
         ('northern_ireland_rules_can_loan_player', 'NORTHERN_IRELAND_RULES::can_loan_player', '0x793900', 'northern_ireland_rules.cpp', 'thiscall, ret 0x10',
          'regra: um clube da Dinamarca, da Coreia, da Irlanda do Norte ou do País de Gales sempre pode pegar emprestado (sem janela nem limite); só guarda o contrato principal do jogador',
          'rule: a club of Denmark, Korea, Northern Ireland or Wales can always take a player on loan (no window, no limit); it only keeps the player\'s main contract'),
         ('norway_rules_can_loan_player', 'NORWAY_RULES::can_loan_player', '0x794760', 'norway_rules.cpp', 'thiscall, ret 0x10',
          'regra: um clube da Noruega ou da Holanda só pega emprestado com uma janela da sua nação aberta hoje; fora dela, o motivo 9 e "The deadline for loans to ... clubs has passed."',
          'rule: a club of Norway or Holland only takes a player on loan with a window of its nation open today; outside it, reason 9 and "The deadline for loans to ... clubs has passed."')])
CAMPOS = [('regras de transferência +0xa (NATION_ID): a nação das regras, para o adjetivo do texto; ponteiro; lê',
           'transfer rules +0xa (NATION_ID): the rules\' nation, for the adjective of the text; pointer; reads'),
          ('[0xae1080] (G_CONTRACT_LOOKUP_RESULT): o contrato principal do jogador, guardado pela regra norte-irlandesa; escreve',
           '[0xae1080] (G_CONTRACT_LOOKUP_RESULT): the player\'s main contract, kept by the Northern Irish rule; writes')]
INTERESSA = ['Tweaks']
