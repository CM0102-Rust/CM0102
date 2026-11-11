# Encerrar um empréstimo na França e na Itália e o prazo dos empréstimos na Austrália (france_rules.cpp,
# italy_rules.cpp, australia_rules.cpp), leitor C do mapa do mercado (levas/reorganizacao/mercado/secao_c.md,
# 11/11/2026). Item de competições: só rodam com essas nações jogáveis, na carreira de 8 ligas.
# - 0x5c1ed0 (FRANCE_RULES::can_end_loan) e 0x6618f0 (ITALY_RULES::can_end_loan): o virtual +0x24, pela 0x8d0530
#   (tela do empréstimo, pedido de empréstimo da IA 0x842740) e pela 0x8d0040 (cada mês do empréstimo do jogador de
#   um clube francês ou italiano, com flag 1). É a regra que o can_loan_player usa para recusar "This loan deal
#   cannot go ahead".
# - 0x412dd0 (AUSTRALIA_RULES::can_loan_player): o virtual +0x20 pela 0x8d0040 e pela 0x8c2180 (o clube australiano
#   pegando emprestado).
# Ficam de fora (não rodam em nenhuma das 8 ligas da auditoria): Finlândia 0x5a9a80/0x5a9af0/0x5a9ba0, Noruega
# 0x794760, Turquia 0x902240, Irlanda do Norte 0x793870/0x793900, Rússia 0x7ec8e0, País de Gales 0x90f1a0, Alemanha
# 0x5e0210, Irlanda 0x63c530/0x63c4e0 (mapeadas na secao_c.md).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas na desmontagem: as duas can_end_loan thiscall ret 0x14 (push flag; push data; push mensagem;
# push jogador; push clube; mov ecx, regra); a 0x412dd0 thiscall ret 0x10. Dentro: 0x8e8e50 push 0; push data; ecx =
# regra -> thiscall ret 8; 0x8e8c40 push janela; push data; mov ecx, esi -> thiscall ret 8; 0x54cfb0 mov ecx, data ->
# thiscall ret 0; 0x6181d0 push forma; push 0x64; push buffer; push nação; add esp, 0x10 -> cdecl; 0x66f4e0 cdecl
# variádica.
# Comparadores: lado a lado no retorno e no texto da mensagem (só escrita; a cópia começa com o primeiro byte zerado e
# no fim volta o texto da original); a australiana pelo comparador dos can_loan_player ingleses (retorno, motivo,
# [0xae1080] e texto).
# Determinismo: nada lê pilha ou heap como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('france_rules_can_end_loan', 0x5c1ed0, [0x8b, 0x44, 0x24, 0x10, 0x83, 0xec, 0x64]),
           ('italy_rules_can_end_loan', 0x6618f0, [0x83, 0xec, 0x64, 0x56, 0x57]),
           ('australia_rules_can_loan_player', 0x412dd0, [0x83, 0xec, 0x64, 0x56, 0x6a, 0x00])]
DOCS = ('encerrar um empréstimo na França e na Itália; o prazo dos empréstimos na Austrália',
        'ending a loan in France and Italy; the Australian loan deadline',
        [('france_rules_can_end_loan', 'FRANCE_RULES::can_end_loan', '0x5c1ed0', 'france_rules.cpp', 'thiscall, ret 0x14',
          'regra: o empréstimo com um clube francês só termina (devolvido ou chamado de volta) numa data dentro de uma janela francesa; fora dela, "Players cannot be sent back/recalled until after the French transfer deadline"',
          'rule: a loan with a French club only ends (sent back or recalled) on a date inside a French window; outside it, "Players cannot be sent back/recalled until after the French transfer deadline"'),
         ('italy_rules_can_end_loan', 'ITALY_RULES::can_end_loan', '0x6618f0', 'italy_rules.cpp', 'thiscall, ret 0x14',
          'regra: o empréstimo com um clube italiano termina em junho (na conferência do pedido de empréstimo) ou numa data dentro das duas primeiras janelas italianas (a terceira, só de jogadores livres, não conta)',
          'rule: a loan with an Italian club ends in June (when the loan request is checked) or on a date inside the first two Italian windows (the third, free agents only, does not count)'),
         ('australia_rules_can_loan_player', 'AUSTRALIA_RULES::can_loan_player', '0x412dd0', 'australia_rules.cpp', 'thiscall, ret 0x10',
          'regra: o clube australiano só pega emprestado com a janela australiana aberta hoje (11 de junho a 17 de abril); fora dela, o motivo 9 e "The deadline for loans to Australian clubs has passed."',
          'rule: an Australian club only takes a player on loan with the Australian window open today (11 June to 17 April); outside it, reason 9 and "The deadline for loans to Australian clubs has passed."')])
CAMPOS = [('regras de transferência +4: a tabela das janelas (12 bytes cada); ponteiro; lê (a italiana só as duas primeiras)',
           'transfer rules +4: the window table (12 bytes each); pointer; reads (the Italian one only the first two)'),
          ('staff +0x39 (club_job): o clube do jogador, para escolher "devolver" ou "chamar de volta"; ponteiro; lê',
           'staff +0x39 (club_job): the player\'s club, to choose "send back" or "recall"; pointer; reads')]
INTERESSA = ['Tweaks']
