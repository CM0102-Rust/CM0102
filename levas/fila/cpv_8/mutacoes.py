# Mutacoes, todas no ritmo de recuperacao depois de cada partida (0x62c080, pela 0x6b5f00 e pela 0x84be20 em toda
# partida da carreira): o ritmo entra na recuperacao da condicao (0x62bd10, +0xc do registro de cada jogador), que pesa
# na escalacao e no motor das partidas seguintes. O divisor da condicao (sempre), a metade no jogo sem a marca +0x40 e o
# dobro do jovem com a marca: uma das duas ultimas roda em toda vaga, conforme o jogo (os contadores mostram quais).
MUTACOES = [
    ('cpv8_ritmo_divisor_90',
     '(c / 100).wrapping_add(((c as u32 & 0xffff_0000) | nivel as i16 as u16 as u32) as i32); // cpv8: o ritmo',
     '(c / 90).wrapping_add(((c as u32 & 0xffff_0000) | nivel as i16 as u16 as u32) as i32); // mutacao: divisor 90',
     'cpv_8 ritmo'),
    ('cpv8_um_terco_sem_marca',
     'v = (v as i16 as i32) / 2; // cpv8: sem a marca do jogo, a metade',
     'v = (v as i16 as i32) / 3; // mutacao: um terco',
     'cpv_8 metade'),
    ('cpv8_jovem_em_triplo',
     'v = v.wrapping_add(v); // cpv8: o jovem em dobro',
     'v = v.wrapping_add(v).wrapping_add(v); // mutacao: o triplo',
     'cpv_8 jovem'),
]
