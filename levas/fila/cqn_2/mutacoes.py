# Mutações (cqn_2): as janelas destas nações valem na carreira de 8 ligas quando um clube das 8 ligas compra um jogador
# de um clube delas (a data da transferência anda pelas regras da nação do comprador e da do vendedor, 0x8cf0e0);
# a carreira começa em julho de 2001, no meio das janelas de verão:
# - Grécia: a janela do meio do ano fecha em junho em vez de agosto (contador "cqn_2 grecia");
# - Rússia: a janela do meio do ano fecha em julho em vez de agosto (contador "cqn_2 russia");
# - Bélgica: a segunda janela fecha em julho em vez de dezembro (contador "cqn_2 belgica");
MUTACOES = [
    ('cqn2_grecia', '[0x0b, 0x00, 0x06, 0x1d, 0x07, 0x00], // cqn2: a Grécia', '[0x0b, 0x00, 0x06, 0x1d, 0x05, 0x00], // mutacao: a janela do meio do ano fecha em junho em vez de agosto', 'cqn_2 grecia'),
    ('cqn2_russia', '[0x14, 0x01, 0xff, 0x04, 0x07, 0x00], // cqn2: a Rússia', '[0x14, 0x01, 0xff, 0x04, 0x06, 0x00], // mutacao: a janela do meio do ano fecha em julho em vez de agosto', 'cqn_2 russia'),
    ('cqn2_belgica', '[0x03, 0x01, 0xff, 0x1f, 0x0b, 0x00], // cqn2: a Bélgica', '[0x03, 0x01, 0xff, 0x1f, 0x06, 0x00], // mutacao: a segunda janela fecha em julho em vez de dezembro', 'cqn_2 belgica'),
]
