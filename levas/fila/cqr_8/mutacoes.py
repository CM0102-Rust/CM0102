# Mutacoes (cqr_8), na fase de cada grupo das copas italianas (2001-000; jogam em agosto de 2001): os times do grupo
# na ordem inversa (mando e datas); os times nao saem da lista (os grupos seguintes repetem clubes); a vitoria vale o
# ponto do empate na liga do grupo (outro campeao de grupo).
MUTACOES = [
    ('cqr8_times_em_ordem_inversa',
     'cqr8_pd(times + k as usize * 4, c); // cqr8: o time na vaga k do grupo',
     'cqr8_pd(times + (cqr8_w(this + 0x3e) as i32 - 1 - k) as usize * 4, c); // mutacao',
     'cqr_8 time do grupo da copa da italia'),
    ('cqr8_time_fica_na_lista',
     '                cqr8_pd(cqr8_d(this + 0xee) + o, 0);\n',
     '                // mutacao: fica na lista\n',
     'cqr_8 time do grupo da copa da serie c'),
    ('cqr8_vitoria_vale_empate',
     '            cqr8_b(this + 0xc2) as u32,',
     '            cqr8_b(this + 0xc3) as u32, // mutacao',
     'cqr_8 time do grupo da copa da italia'),
]
