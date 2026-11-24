# Mutacoes (sx04b_3):
# - as partidas nao vao para as notas do mes (sem jogador nem jovem do mes: a reputacao dos vencedores muda, a carreira
#   acusa);
# - a nota da lista curta pela soma do modo seguinte (o comparador acusa as entradas; a decisao da lista e em junho);
# - exige um candidato a mais do que cabe (a lista nao e preenchida nem anunciada; o comparador acusa).
MUTACOES = [
    ('sx04b_3_sem_notas_do_mes',
     'f(ratings, fixture, record); // sx04b_3: the month ratings',
     'let _ = (f, fixture, record); // mutation: no month ratings',
     'sx04b_3 match to the month ratings'),
    ('sx04b_3_nota_do_modo_seguinte',
     '0..=2 => entry.rating_sum(mode as usize),',
     '0..=2 => entry.rating_sum((mode as usize + 1) % 3), // mutation',
     'sx04b_3 shortlist filled'),
    ('sx04b_3_um_a_mais',
     'if found < n { // sx04b_3: enough candidates',
     'if found < n + 1 { // mutation: one more',
     'sx04b_3 player of the year from ratings'),
]
