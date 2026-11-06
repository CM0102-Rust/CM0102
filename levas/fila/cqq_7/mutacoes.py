# Mutacoes (cqq_7). Os play-offs das Series A e B sao montados uma vez cada na carreira de 8 ligas ate 2002-06-01 (no
# fim do Brasileiro de 2001): cada mutacao troca quem joga contra quem ou quem entra (outros jogos, outro campeao).
# O contador fica no corpo comum, logo depois da fase montada (a unica condicao antes e a memoria), com o nome da Serie.
MUTACOES = [
    ('cqq7_serie_a_chaves_trocadas',
     'cqq7_play_off(this, &[0, 3, 1, 2], 0x98aa30, 0x286, 0x29d, "cqq_7 serie a"); // cqq7: 1º × 4º, 2º × 3º Série A',
     'cqq7_play_off(this, &[0, 2, 1, 3], 0x98aa30, 0x286, 0x29d, "cqq_7 serie a"); // mutacao: 1º × 3º e 2º × 4º',
     'cqq_7 serie a'),
    ('cqq7_serie_b_chaves_trocadas',
     'cqq7_play_off(this, &[0, 3, 1, 2], 0x98ab64, 0x25f, 0x276, "cqq_7 serie b"); // cqq7: 1º × 4º, 2º × 3º Série B',
     'cqq7_play_off(this, &[0, 2, 1, 3], 0x98ab64, 0x25f, 0x276, "cqq_7 serie b"); // mutacao: 1º × 3º e 2º × 4º',
     'cqq_7 serie b'),
    ('cqq7_serie_a_quinto_no_lugar_do_quarto',
     'cqq7_play_off(this, &[0, 3, 1, 2], 0x98aa30, 0x286, 0x29d, "cqq_7 serie a"); // cqq7: 1º × 4º, 2º × 3º Série A',
     'cqq7_play_off(this, &[0, 4, 1, 2], 0x98aa30, 0x286, 0x29d, "cqq_7 serie a"); // mutacao: o 5º no lugar do 4º',
     'cqq_7 serie a'),
]
# Prova do agente (06/11, carreira Rust de 8 ligas ate 2002-06-01, base com os 9 itens): as tres ACUSADAS (os play-offs em 2001-335 e 2001-339; separam em 2001-339 e 2001-341).
