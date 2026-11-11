# Mutacoes (mkd_7). A rotina roda na decisao das ofertas de contrato de quem tem ficha de comissao (0x4ce4e0), no
# contrato desejado (0x85fc50), nas vagas de tecnico (0x69f810) e nas ofertas da IA (0x840084); a vontade decide quem
# aceita o cargo:
# - a base para outro clube 3 em vez de 1,5 / 2,25 (mais gente aceita trocar de clube);
# - o jogador sem o x 10 para um cargo de comissao (menos jogadores viram comissao);
# - sem a diferenca das escalas de reputacao (clubes menores atraem como os maiores).
MUTACOES = [
    ('mkd7_base_tres',
     'score = if job == 5 || job == 0xc { 2.25 } else { 1.5 }; // mkd7: the base for another club',
     'score = 3.0; // mutation: the base 3',
     'mkd_7 job interest'),
    ('mkd7_jogador_sem_dez',
     'score *= read_f64(rdata::F64_10); // mkd7: a player taking a staff job',
     'score *= 1.0; // mutation: without the x 10',
     'mkd_7 job interest'),
    ('mkd7_sem_escalas',
     'let mut t = y * read_f64(job_interest_rdata::F64_0_0002222222222) + read_f64(rdata::F64_1); // mkd7: the scales\' difference',
     'let mut t = read_f64(rdata::F64_1) + 0.0 * y; // mutation: without the scales\' difference',
     'mkd_7 job interest'),
]
