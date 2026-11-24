# Mutacoes (sx04b_8), com LIGAS = 'ENG SWE FIN NOR POR' (os premios da Suecia em 2001-12-01):
# - a nota sem o peso da reputacao (outros vencedores: o comparador acusa o registro; a reputacao dos premiados muda);
# - exige cinco jogos a mais (outros vencedores ou nenhum);
# - a ordem decrescente invertida (compare_order! acusa; outros vencedores).
MUTACOES = [
    ('sx04b_8_sem_peso',
     'write_f32(c + STATS_CANDIDATE_SCORE, (weight as f64 * read_f32(c + STATS_CANDIDATE_SCORE) as f64) as f32); // sx04b_8: weighed by reputation',
     'let _ = weight; // mutation: no reputation weight',
     'sx04b_8 statistics candidate'),
    ('sx04b_8_cinco_jogos_a_mais',
     'if (read_i16(c + STATS_CANDIDATE_GAMES) as i32) < least { // sx04b_8: enough games',
     'if (read_i16(c + STATS_CANDIDATE_GAMES) as i32) < least + 5 { // mutation: five games more',
     'sx04b_8 statistics candidate'),
    ('sx04b_8_ordem_invertida',
     'if above { // sx04b_8: the higher score first',
     'if below { // mutation: the lower score first',
     'sx04b_8 scores ordered'),
]
