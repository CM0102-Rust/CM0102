# Mutacoes (cqu_11), na Recopa asiatica montada no comeco do jogo: o vice da Copa do Imperador (e da copa coreana) no
# lugar do campeao; 20 escolhas no embaralhamento dos clubes; os 16 da zona oeste de tras para a frente.
MUTACOES = [
    ('cqu11_vice_da_copa_do_imperador',
     '    let c = exe::comp_util_get_latest_placed_club(cup_record, 0); // cqu11: the cup winner',
     '    let c = exe::comp_util_get_latest_placed_club(cup_record, 1); // mutation: the cup runner-up',
     'cqu_11 emperors cup club'),
    ('cqu11_20_escolhas',
     '0x28); // cqu11: 40 picks in the shuffle',
     '0x14); // mutation: 20 picks in the shuffle',
     'cqu_11 club draw'),
    ('cqu11_zona_oeste_de_tras_para_frente',
     'write_usize(teams + i * 4, read_usize(read_usize(this + 0xb2) + 0x40 + i * 4)); // cqu11: team i of the west zone',
     'write_usize(teams + i * 4, read_usize(read_usize(this + 0xb2) + 0x40 + (n as usize - 1 - i) * 4)); // mutation: backwards',
     'cqu_11 west zone team'),
]
