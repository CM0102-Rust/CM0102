# Mutacoes (sx12_4), item de telas: a cadeia nao as roda (a carreira de ferias nao abre telas); provam o comparador numa
# sessao do Lucas jogando comparando (taticas, opcoes do jogo, oferecer contrato, elenco).
MUTACOES = [
    ('sx12_selecao_de_zonas',
     'active_tactic_slot_byte(this, manager, 0x18e3, (0x18a, 0x196)) // sx12_4: the zone selection',
     'active_tactic_slot_byte(this, manager, 0x18e7, (0x18a, 0x196)) // mutation: the previous zone selection',
     'sx12_4 zone selection'),
    ('sx12_velocidade_da_narracao',
     'write_u16(record + 0x9ad2, speed); // sx12_4: the commentary speed word',
     'write_u16(record + 0x9ad2, speed.wrapping_add(1)); // mutation: one step faster',
     'sx12_4 set commentary speed'),
    ('sx12_vista_do_elenco',
     'record + 0xa6d4 // sx12_4: the squad view',
     'record + 0xa6d5 // sx12_4: mutation: one byte off',
     'sx12_4 squad view'),
]
