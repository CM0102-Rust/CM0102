# Mutacoes (rx13_15), pela carreira ENG com a base NORMAL ate 2002-08-01 (a virada de 2002 move os clubes; a temporada
# 2002/03 comeca com eles em agosto):
# - mais de um clube por divisao (sem a regra de um por divisao: sobem os tres mais fortes);
# - os rebaixados da Conference ficam nela (nenhuma descida).
MUTACOES = [
    ('rx13_15_sem_um_por_divisao',
     'if d != 0 && feeders.contains(&d) {',
     'if false && feeders.contains(&d) { // mutation',
     'rx13_15 conference feeders'),
    ('rx13_15_rebaixados_ficam',
     'while i < conference.club_count() as i32 && taken < 3 {',
     'while i < conference.club_count() as i32 && taken < 0 { // mutation',
     'rx13_15 conference feeders'),
]
