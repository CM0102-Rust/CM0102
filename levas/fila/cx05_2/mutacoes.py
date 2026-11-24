# Mutacoes (cx05_2), carreira de 8 ligas ate 2004-11-01 (o sorteio da Copa da Asia e em 5 de junho de 2004).
# - os potes na ordem inversa: cada grupo fica com os mesmos potes, mas as vagas trocam de linha e os jogos fixos de
#   cada rodada mudam;
# - o grupo A le as sorteadas 4 a 7 (as do grupo B): o grupo A repete o grupo B e as seleções 0 a 3 nao jogam;
# - o embaralhamento das candidatas pega duas a mais: a escolha das finalistas muda.
MUTACOES = [
    ('cx05_2_potes_invertidos',
     '            let first = pot * 3; // cx05_2: the pot of the group\'s slot',
     '            let first = (3 - pot) * 3; // mutation'),
    ('cx05_2_grupo_a_do_b',
     'read_usize(read_usize(this + 0xee) + i as isize as usize * 4), 0, 0); // cx05_2: group A\'s team in the row',
     'read_usize(read_usize(this + 0xee) + (i as isize as usize + 4) * 4), 0, 0); // mutation'),
    ('cx05_2_embaralha_mais',
     '    let places = (read_u8(this + 0xf2).wrapping_sub(n as u8)) as i8 as i16; // cx05_2: the places after the hosts',
     '    let places = (read_u8(this + 0xf2).wrapping_sub(n as u8).wrapping_add(2)) as i8 as i16; // mutation'),
]
