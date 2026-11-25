# Mutacoes (rx13_16), pela carreira ENG ESP SWE com a base NORMAL ate 2002-09-01 (as viradas de 2002 movem os clubes; a
# temporada seguinte comeca com eles):
# - os times B sobem mesmo com o time principal na Segunda B;
# - os grupos da Segunda B sem a segunda ordenacao (os 60 do norte ficam pela latitude);
# - os doze clubes guardados nao sobem para a Segunda Divisao sueca.
MUTACOES = [
    ('rx13_16_time_b_sobe',
     'if partner != 0 && is_a == 0 {',
     'if partner != 0 && is_a == 0 && false { // mutation',
     'rx13_16 segunda b exchange'),
    ('rx13_16_grupos_so_latitude',
     'exe::crt_qsort(list[20..].as_mut_ptr() as usize, 60, 4, CLUB_LONGITUDE_COMPARE); // rx13_16: the northern 60 by longitude',
     '// mutation: no longitude sort',
     'rx13_16 segunda b groups'),
    ('rx13_16_doze_ficam',
     'for k in 0..0xc {\n        let id = read_u32(second.addr() + 0xee + k * 4); // the twelve promoted club ids',
     'for k in 0..0 {\n        let id = read_u32(second.addr() + 0xee + k * 4); // mutation',
     'rx13_16 swedish second division moves'),
]
