# As respostas de base das regras de transferencia (argentina_rules.cpp pelo mapa; leitor sx21, rodada 5, 24/11/2026):
# os slots 0x14 (nenhuma nacao o troca), 0x20 (pode emprestar) e 0x24 (o emprestimo pode acabar) das nacoes sem regra
# propria. O 0x24 roda na carreira ENG (a Inglaterra nao tem prazo de emprestimo: as devolucoes e os chamados de
# volta); os outros dois, onde as regras de base forem perguntadas. Corpos de "mov eax, 1; ret N" de 5 bytes de
# prologo (cada um no seu endereco), mantidos porque o desvio faz sentido: a resposta de todas essas nacoes.
# Convencoes conferidas: 0x40a5f0 thiscall ret 0xc; 0x40a600 thiscall ret 0x10; 0x40a610 thiscall ret 0x14.
# Comparadores: o valor; o 0x24 pelo rules_compare_can_end_loan (france_rules.rs).
SAVE = False
ROTINAS = [
    ('transfer_rules_rule_slot_5', 0x40a5f0, [0xb8, 0x01, 0x00, 0x00, 0x00]),
    ('transfer_rules_can_loan_player', 0x40a600, [0xb8, 0x01, 0x00, 0x00, 0x00]),
    ('transfer_rules_can_end_loan', 0x40a610, [0xb8, 0x01, 0x00, 0x00, 0x00]),
]
DOCS = ('as respostas de base das regras de transferência: o slot 5, o empréstimo e o fim do empréstimo',
        'the default answers of the transfer rules: slot 5, the loan and the end of a loan',
        [('transfer_rules_rule_slot_5', 'transfer_rules_rule_slot_5', '0x40a5f0', 'argentina_rules.cpp', 'thiscall, ret 0xc',
          'regra: sempre 1 (nenhuma nação troca o slot 0x14)',
          'rule: always 1 (no country overrides slot 0x14)'),
         ('transfer_rules_can_loan_player', 'transfer_rules_can_loan_player', '0x40a600', 'argentina_rules.cpp', 'thiscall, ret 0x10',
          'regra: todo empréstimo pode (1), sem mensagem nem motivo',
          'rule: every loan is allowed (1), with no message nor reason'),
         ('transfer_rules_can_end_loan', 'transfer_rules_can_end_loan', '0x40a610', 'argentina_rules.cpp', 'thiscall, ret 0x14',
          'regra: o emprestado pode ser chamado de volta ou devolvido a qualquer hora (1)',
          'rule: a loaned player may be recalled or sent back at any time (1)')])
CAMPOS = []
INTERESSA = []
