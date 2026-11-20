# Mutacoes (mwa_26): o terceiro do primeiro grupo no mata-mata em vez do vice (outro clube nas quartas de 31/10); os
# grupos A e B trocados (os mesmos clubes em outros grupos: outros jogos); a primeira rodada dos grupos 7 dias antes
# (22/8).
MUTACOES = [
    ('mwa26_terceiro_no_mata_mata', "write_usize(teams + 4, read_usize(this.table() + league_table_row::SIZE)); // mwa26: the first group's runner-up", 'write_usize(teams + 4, read_usize(this.table() + 2 * league_table_row::SIZE)); // mutation: the third', 'mwa_26 northern irish league cup knockout'),
    ('mwa26_grupos_a_e_b_trocados', 'Some((0, 5)) } // mwa26: the group A band\n                else if id == read_usize(KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_B) { Some((5, 0xa)) }', 'Some((5, 0xa)) } // mutation: groups A and B swapped\n                else if id == read_usize(KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_B) { Some((0, 5)) }', 'mwa_26 northern irish league cup entrants'),
    ('mwa26_grupos_7_dias_antes', '[(0x1d, 7, 2, 2), (0xc, 8, 2, 2), (0x1a, 8, 2, 2), (0xa, 9, 2, 2), (0x18, 9, 2, 2)]; // mwa26: the group matchdays', '[(0x16, 7, 2, 2), (0xc, 8, 2, 2), (0x1a, 8, 2, 2), (0xa, 9, 2, 2), (0x18, 9, 2, 2)]; // mutation: 22 August', 'mwa_26 northern irish league cup groups'),
]
