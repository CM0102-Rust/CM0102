# Mutações da 326a leva (itens gra_22). Uso: python mutacoes.py mutacoes\leva326.py --carreira 2002-06-01
MUTACOES = [
    ('gra22_19_trocas', 'for _ in 0..0x14 { // gra22: 20 swaps per pot', 'for _ in 0..0x13 { // mutation: 19 swaps per pot', 'gra_22 pot drawn'),
    ('gra22_vaga_do_proprio_grupo', 'let cap = exe::eurochampqual_get_group_team_count(this, g.wrapping_sub(1) as u8 as u32); // gra22: the capacity asked for g - 1', 'let cap = exe::eurochampqual_get_group_team_count(this, g as u8 as u32); // mutation: the group own room', 'gra_22 pot drawn'),
]
