# Mutações (cqm_1): as janelas destas nações valem na carreira de 8 ligas quando um clube das 8 ligas compra um jogador
# de um clube delas (a data da transferência anda pelas regras da nação do comprador e da do vendedor, 0x8cf0e0);
# a carreira começa em julho de 2001, no meio das janelas de verão:
# - Holanda: abre em outubro (fechada de julho a setembro) (contador "cqm_1 netherlands");
# - Croácia: abre em outubro (fechada de julho a setembro) (contador "cqm_1 croatia");
# - Irlanda: abre em outubro (fechada de julho a setembro) (contador "cqm_1 ireland");
MUTACOES = [
    ('cqm1_holanda', '[0x0c, 0x00, 0x06, 0x08, 0x05, 0x01], // cqm1: a Holanda', '[0x0c, 0x00, 0x06, 0x08, 0x09, 0x01], // mutacao: abre em outubro (fechada de julho a setembro)', 'cqm_1 netherlands'),
    ('cqm1_croacia', '[0x05, 0x00, 0xff, 0x01, 0x05, 0x01], // cqm1: a Croácia', '[0x05, 0x00, 0xff, 0x01, 0x09, 0x01], // mutacao: abre em outubro (fechada de julho a setembro)', 'cqm_1 croatia'),
    ('cqm1_irlanda', '[0x0d, 0x00, 0xff, 0x0a, 0x04, 0x01], // cqm1: a Irlanda', '[0x0d, 0x00, 0xff, 0x0a, 0x09, 0x01], // mutacao: abre em outubro (fechada de julho a setembro)', 'cqm_1 ireland'),
]
