# Mutações (cqn_1): as janelas destas nações valem na carreira de 8 ligas quando um clube das 8 ligas compra um jogador
# de um clube delas (a data da transferência anda pelas regras da nação do comprador e da do vendedor, 0x8cf0e0);
# a carreira começa em julho de 2001, no meio das janelas de verão:
# - Argentina: a janela de julho fecha em 25 de julho em vez de 6 de agosto (contador "cqn_1 argentina");
# - Portugal: a janela do meio do ano fecha em 1º de julho em vez de 1º de setembro (contador "cqn_1 portugal");
# - Coreia do Sul: fecha em janeiro (fechada de julho a novembro) (contador "cqn_1 coreia");
MUTACOES = [
    ('cqn1_argentina', '[0x01, 0x00, 0x03, 0x06, 0x07, 0x00], // cqn1: a Argentina', '[0x01, 0x00, 0x03, 0x19, 0x06, 0x00], // mutacao: a janela de julho fecha em 25 de julho em vez de 6 de agosto', 'cqn_1 argentina'),
    ('cqn1_portugal', '[0x13, 0x00, 0xff, 0x01, 0x08, 0x00], // cqn1: a Portugal', '[0x13, 0x00, 0xff, 0x01, 0x06, 0x00], // mutacao: a janela do meio do ano fecha em 1º de julho em vez de 1º de setembro', 'cqn_1 portugal'),
    ('cqn1_coreia', '[0x1b, 0x00, 0xff, 0x19, 0x06, 0x00], // cqn1: a Coreia do Sul', '[0x1b, 0x00, 0xff, 0x19, 0x00, 0x00], // mutacao: fecha em janeiro (fechada de julho a novembro)', 'cqn_1 coreia'),
]
