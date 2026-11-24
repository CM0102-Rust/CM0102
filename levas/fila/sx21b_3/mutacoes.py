# Mutacoes (sx21b_3). get_id_block roda em toda partida (pelos jogadores virtuais das duas equipes); as outras quando
# um clube tem virtuais. Caem pelos comparadores lado a lado (e pela carreira, a do bloco: outros registros virtuais).
MUTACOES = [
    ('sx21b_3_bloco_49',
     '(slot as i32).wrapping_mul(50) + away // sx21b_3: 50 records a fixture',
     '(slot as i32).wrapping_mul(49) + away // mutation: 49',
     'sx21b_3 fixture block'),
    ('sx21b_3_vaga_mantida',
     'write_u32((this as isize + virtual_fixture_fns::VIEWED_CLUBS as isize + slot * 4) as usize, 0); // sx21b_3: the slot is freed',
     'let _ = slot; // mutation: the slot is kept',
     'sx21b_3 virtual players removed'),
    ('sx21b_3_bloco_24',
     'if read_i32(s).wrapping_sub(staff_id) < 0x19 { // sx21b_3: already in the block',
     'if read_i32(s).wrapping_sub(staff_id) < 0x18 { // mutation: 24',
     'sx21b_3 virtual player moved'),
]
