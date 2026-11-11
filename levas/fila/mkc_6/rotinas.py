# Encerrar um empréstimo na Alemanha e na Irlanda e a conta de empréstimos da temporada na Irlanda
# (germany_rules.cpp, ireland_rules.cpp), fechamento do mapa do mercado (levas/reorganizacao/mercado/fechamento_mkc.md,
# 11/11/2026). Pelas 29 tabelas virtuais das regras (achadas pelo slot +0x14 = 0x40a5f0, cada uma pelo construtor que a
# grava), as três só servem à própria nação e não rodam na carreira de 8 ligas:
# - 0x5e0210 (GERMANY_RULES::can_end_loan): o virtual +0x24 da Alemanha (0x96b3cc); pela 0x8d0530 (telas 0x8fa341 com
#   flag 1 e 0x8fbcf9 com flag 0; pedido de empréstimo da IA 0x8429ae, flag 1) e pela 0x8d0040 (flag 1).
# - 0x63c530 (IRELAND_RULES::can_end_loan): o virtual +0x24 da Irlanda (0x96c190); os mesmos chamadores.
# - 0x63c4e0 (IRELAND_RULES::record_loan): o virtual +0x8 da Irlanda, na assinatura do empréstimo. A tabela irlandesa
#   usa como daily_update (+0) a inglesa 0x57a020 (zera a conta em 20 de junho; já em Rust).
# A prova é pela comparação numa carreira com as ligas alemã e irlandesa carregadas (LIGAS abaixo; o simular.py aceita
# --ligas com os códigos de 3 letras do nation.dat +0x53: GER, IRL). A 0x5e0210 só recusa com flag 0, que só a tela de
# encerrar o empréstimo (0x8fbcc0) passa: numa carreira de férias ela só devolve 1; a mutação dela (recusar também com
# flag) muda a carreira pela 0x8d0040 e pelo pedido da IA.
# Iguais no cm0102-1080.exe (as três inteiras, conferido byte a byte).
# Convenções conferidas na desmontagem: as duas can_end_loan thiscall ret 0x14 (a flag em [esp+0x14] na entrada da
# alemã; push flag; push data; push mensagem; push jogador; push clube; mov ecx, regra); a 0x63c530 chama a 0x8e8e50
# com o ecx da entrada (push 0; push data). A 0x63c4e0 thiscall ret 4 (a oferta em [esp+4]). Dentro: 0x6181d0 push
# forma; push 0x64; push buffer; push nação; add esp, 0x10 -> cdecl; 0x66f4e0 cdecl variádica.
# Comparadores: lado a lado; as can_end_loan no retorno e no texto da mensagem (a cópia começa com o primeiro byte
# zerado e no fim volta o texto da original); a record_loan no byte da conta do clube.
# Determinismo: nada lê pilha ou heap como valor.
LIGAS = 'ENG FIN NOR NIR GER IRL'
SAVE = False
ROTINAS = [('germany_rules_can_end_loan', 0x5e0210, [0x8b, 0x44, 0x24, 0x14, 0x83, 0xec, 0x64]),
           ('ireland_rules_can_end_loan', 0x63c530, [0x8b, 0x44, 0x24, 0x10, 0x83, 0xec, 0x64]),
           ('ireland_rules_record_loan', 0x63c4e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0xb4, 0x23, 0xae, 0x00])]
DOCS = ('encerrar um empréstimo na Alemanha e na Irlanda; a conta de empréstimos da temporada na Irlanda',
        'ending a loan in Germany and Ireland; the Irish season loan count',
        [('germany_rules_can_end_loan', 'GERMANY_RULES::can_end_loan', '0x5e0210', 'germany_rules.cpp', 'thiscall, ret 0x14',
          'regra: o empréstimo entre clubes alemães não termina antes da hora pela tela (flag 0): "Players on loan from other German clubs cannot be sent back until the end of the loan." para quem pegou emprestado, "Players on loan to other German clubs cannot be recalled." para o dono; com a flag (a conferência do empréstimo e o pedido da IA), sempre pode',
          'rule: a loan between German clubs cannot end early through the screen (flag 0): "Players on loan from other German clubs cannot be sent back until the end of the loan." for the borrower, "Players on loan to other German clubs cannot be recalled." for the owner; with the flag (the loan check and the AI request), it always can'),
         ('ireland_rules_can_end_loan', 'IRELAND_RULES::can_end_loan', '0x63c530', 'ireland_rules.cpp', 'thiscall, ret 0x14',
          'regra: o empréstimo com um clube irlandês só termina (devolvido ou chamado de volta) numa data dentro da janela irlandesa (10 de maio a 31 de janeiro); fora dela, "Players cannot be sent back/recalled ... until after the Irish transfer deadline"',
          'rule: a loan with an Irish club only ends (sent back or recalled) on a date inside the Irish window (10 May to 31 January); outside it, "Players cannot be sent back/recalled ... until after the Irish transfer deadline"'),
         ('ireland_rules_record_loan', 'IRELAND_RULES::record_loan', '0x63c4e0', 'ireland_rules.cpp', 'thiscall, ret 4',
          'regra: cada empréstimo assinado por um clube irlandês soma 1 na conta da temporada dele (o limite de 5 da regra irlandesa; zerada em 20 de junho)',
          'rule: each loan signed by an Irish club adds 1 to its season count (the Irish rule\'s limit of 5; zeroed on 20 June)')])
CAMPOS = [('staff +0x39 (club_job): o clube do jogador, a nação dele (a alemã) e "devolver" ou "chamar de volta"; ponteiro; lê',
           'staff +0x39 (club_job): the player\'s club, its nation (the German rule) and "send back" or "recall"; pointer; reads'),
          ('[[0xde1f54]+0x91c] + id·16 (SEASON_LOAN_COUNTS): o byte 0, empréstimos do clube na temporada; soma 1',
           '[[0xde1f54]+0x91c] + id·16 (SEASON_LOAN_COUNTS): byte 0, the club\'s loans this season; adds 1')]
INTERESSA = ['Tweaks']
