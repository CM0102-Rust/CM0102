# Mutações (cqm_2): as janelas destas nações valem na carreira de 8 ligas quando um clube das 8 ligas compra um jogador
# de um clube delas (a data da transferência anda pelas regras da nação do comprador e da do vendedor, 0x8cf0e0);
# a carreira começa em julho de 2001, no meio das janelas de verão:
# - Noruega: fecha em março (fechada de abril a agosto) (contador "cqm_2 norway");
# - Irlanda do Norte: abre em outubro (fechada de julho a setembro) (contador "cqm_2 northern_ireland");
# - País de Gales: abre em outubro (fechada de julho a setembro) (contador "cqm_2 wales");
MUTACOES = [
    ('cqm2_noruega', '[0x11, 0x00, 0xff, 0x1f, 0x07, 0x00], // cqm2: a Noruega', '[0x11, 0x00, 0xff, 0x1f, 0x02, 0x00], // mutacao: fecha em março (fechada de abril a agosto)', 'cqm_2 norway'),
    ('cqm2_irlanda_do_norte', '[0x10, 0x00, 0x06, 0x06, 0x04, 0x01], // cqm2: a Irlanda do Norte', '[0x10, 0x00, 0x06, 0x06, 0x09, 0x01], // mutacao: abre em outubro (fechada de julho a setembro)', 'cqm_2 northern_ireland'),
    ('cqm2_pais_de_gales', '[0x1a, 0x00, 0x00, 0x07, 0x04, 0x01], // cqm2: a País de Gales', '[0x1a, 0x00, 0x00, 0x07, 0x09, 0x01], // mutacao: abre em outubro (fechada de julho a setembro)', 'cqm_2 wales'),
]
