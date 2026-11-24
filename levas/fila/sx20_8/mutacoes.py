# Mutacoes (sx20_8), item de SAVE: a prova do save grava o .qst com e sem a leva (e o free_game_data do .qst roda os
# dois destrutores depois da gravacao; liberar memoria nao deixa rastro no .qst, por isso nao ha mutacao deles):
# - as listas de posicao gravadas com 8 bytes por entrada (o item muda);
# - as vagas dos clubes gravadas so ate a 39a.
MUTACOES = [
    ('sx20_8_listas_curtas',
     'if !write(read_usize(this + lists + 4 * k), 0xc, read_u32(this + f::POSITION_LIST_COUNTS + 4 * k), 0x3db) { // sx20_8: the position lists',
     'if !write(read_usize(this + lists + 4 * k), 8, read_u32(this + f::POSITION_LIST_COUNTS + 4 * k), 0x3db) { // mutation: 8-byte entries',
     'sx20_8 save shortlists'),
    ('sx20_8_vagas_a_menos',
     'if slots != 0 && !write(slots, SHORTLIST_SLOT_SIZE as u32, SHORTLIST_SLOTS as u32, 0x3f3) {',
     'if slots != 0 && !write(slots, SHORTLIST_SLOT_SIZE as u32, SHORTLIST_SLOTS as u32 - 1, 0x3f3) {',
     'sx20_8 save shortlists'),
]
