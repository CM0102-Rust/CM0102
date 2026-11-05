# Mutações (cqm_3): as janelas destas nações valem na carreira de 8 ligas quando um clube das 8 ligas compra um jogador
# de um clube delas (a data da transferência anda pelas regras da nação do comprador e da do vendedor, 0x8cf0e0);
# a carreira começa em julho de 2001, no meio das janelas de verão:
# - Turquia: abre em outubro (fechada de julho a setembro) (contador "cqm_3 turquia");
# - Polônia: abre em outubro (fechada de julho a setembro) (contador "cqm_3 polonia");
# - Finlândia: fecha em abril (fechada de abril a novembro) (contador "cqm_3 finlandia");
MUTACOES = [
    ('cqm3_turquia', '[0x18, 0x00, 0xff, 0x01, 0x05, 0x01], // cqm3: a Turquia', '[0x18, 0x00, 0xff, 0x01, 0x09, 0x01], // mutacao: abre em outubro (fechada de julho a setembro)', 'cqm_3 turquia'),
    ('cqm3_polonia', '[0x12, 0x00, 0xff, 0x01, 0x06, 0x01], // cqm3: a Polônia', '[0x12, 0x00, 0xff, 0x01, 0x09, 0x01], // mutacao: abre em outubro (fechada de julho a setembro)', 'cqm_3 polonia'),
    ('cqm3_finlandia', '[0x08, 0x00, 0xff, 0x0f, 0x07, 0x00], // cqm3: a Finlândia', '[0x08, 0x00, 0xff, 0x0f, 0x03, 0x00], // mutacao: fecha em abril (fechada de abril a novembro)', 'cqm_3 finlandia'),
]
