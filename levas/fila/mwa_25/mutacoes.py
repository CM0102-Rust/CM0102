# Mutacoes (mwa_25): o vice do primeiro grupo no mata-mata em vez do vencedor (outro clube nas quartas de novembro);
# a primeira faixa de 6 sem o embaralhamento (outros grupos e outra sequencia do gerador); a primeira rodada dos grupos
# 7 dias antes (13/9).
MUTACOES = [
    ('mwa25_vice_no_mata_mata', "write_usize(teams, read_usize(this.table())); // mwa25: the first group's winner", 'write_usize(teams, read_usize(this.table() + league_table_row::SIZE)); // mutation: the runner-up', 'mwa_25 irish league cup knockout'),
    ('mwa25_sem_embaralhar_a_faixa_a', '    exe::comp_util_shuffle_pointer_array(v, 6);\n    exe::comp_util_shuffle_pointer_array(v + 0x18, 6);', '    exe::comp_util_shuffle_pointer_array(v + 0x18, 6); // mutation: band A not shuffled', 'mwa_25 irish league cup entrants'),
    ('mwa25_grupos_7_dias_antes', '[(0x14, 8, 2, 2), (0x18, 8, 6, 1), (0x1b, 8, 2, 2)]; // mwa25: the first group matchday', '[(0xd, 8, 2, 2), (0x18, 8, 6, 1), (0x1b, 8, 2, 2)]; // mutation: 13 September', 'mwa_25 irish league cup groups'),
]
