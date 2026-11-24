# Mutacoes (sx14b_3), pela prova do save (mutacoes.py --prova-save N): o .qst gravado muda.
# - os registros da diretoria gravados com 0x48 bytes: o man_conf.dat sai mais curto e torto;
# - os itens das saidas gravados com 0xe bytes em vez de 6.
MUTACOES = [
    ('sx14b_3_registros_menores',
     '    if exe::zwrite(read_usize(this + f::BOARD_RECORDS), 0x49, clubs, z) != read_u32(G_CLUB_COUNT) {',
     '    if exe::zwrite(read_usize(this + f::BOARD_RECORDS), 0x48, clubs, z) != read_u32(G_CLUB_COUNT) { // mutation',
     'sx14b_3 save man_conf.dat'),
    ('sx14b_3_saidas_sem_itens',
     '    man_conf_write_list(z, read_usize(this + f::DEPARTURE_LIST), 6, (0x256, 0x260, 0x266))?;',
     '    man_conf_write_list(z, read_usize(this + f::DEPARTURE_LIST), 0xe, (0x256, 0x260, 0x266))?; // mutation',
     'sx14b_3 save man_conf.dat'),
]
