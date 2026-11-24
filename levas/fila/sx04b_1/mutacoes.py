# Mutacoes (sx04b_1):
# - o sorteio do modelo entre 2 e nao 3 (o ensaio do comparador sorteia outro limite: rehearse_news acusa);
# - a manchete por % 5 (outro modelo de manchete; acusado pelo compare_text_writer);
# - os dois modelos do corpo com descricao trocados (acusado pelo compare_text_writer).
MUTACOES = [
    ('sx04b_1_sorteio_de_dois',
     'exe::newsitem_set_slot(item, 0xa, exe::utils_random(3) as u32); // sx04b_1: the template draw',
     'exe::newsitem_set_slot(item, 0xa, exe::utils_random(2) as u32); // mutation: two templates',
     'sx04b_1 winner news'),
    ('sx04b_1_manchete_por_cinco',
     'match (winner / (draw as i32 + 1)) % 6 {',
     'match (winner / (draw as i32 + 1)) % 5 { // mutation',
     'sx04b_1 headline'),
    ('sx04b_1_modelos_trocados',
     'if draw == 1 { // sx04b_1: the club-first template',
     'if draw == 2 { // mutation: the templates swapped',
     'sx04b_1 body with the description'),
]
