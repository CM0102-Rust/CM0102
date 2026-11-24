# Mutacoes (cx16_9), ENG ate 2005-12-31; acusadas pelos comparadores (lado a lado):
# - o tamanho dos grupos europeus com duas isentas sem o grupo 7 de seis (quem joga em cada grupo muda no sorteio);
# - a Copa do Mundo trocada pela competicao seguinte da tabela.
MUTACOES = [
    ('cx16_9_grupos_com_duas_isentas',
     '2 => if matches!(g, 2 | 4 | 7) { 6 } else { 5 }, // cx16_9: two exempt, three groups of six',
     '2 => if matches!(g, 2 | 4) { 6 } else { 5 }, // mutation: two groups of six',
     'cx16_9 european group size'),
    ('cx16_9_outra_copa',
     'read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b)) // cx16_9: the World Cup record',
     'read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b) + 0x6b) // mutation: the next record',
     'cx16_9 finals comp'),
]
