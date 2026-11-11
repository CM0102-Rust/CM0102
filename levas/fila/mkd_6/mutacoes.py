# Mutacoes (mkd_6). A rotina roda na criacao da carreira (o salario das listas iniciais, 0x83a1b0) e no contrato dos
# regens (set_default_wage 0x8626e0); o salario padrao entra nas listas e nas propostas, e muda quem cabe no teto de
# salarios de cada clube:
# - o salario do jogador pelo escore em dobro;
# - a oferta do clube pelo estatuto no lugar da media (salarios maiores para quem vale mais que a oferta);
# - sem o corte pelo desenvolvimento 2 da nacao (salarios maiores nas nacoes medias).
MUTACOES = [
    ('mkd6_salario_em_dobro',
     'let x = base * sq * read_f64(rdata::F64_1ENEG_08); // mkd6: the wage by the score',
     'let x = base * sq * 2.0 * read_f64(rdata::F64_1ENEG_08); // mutation: twice the wage',
     'mkd_6 default wage'),
    ('mkd6_oferta_do_clube',
     'v = ((v as f64 + y) * read_f64(rdata::F64_0_5)) as f32; // mkd6: the average with the club\'s offer',
     'v = y as f32; // mutation: the club\'s offer',
     'mkd_6 club\'s offer by status'),
    ('mkd6_sem_desenvolvimento',
     '2 => v = (v as f64 * read_f32(rdata::F32_0_75) as f64) as f32, // mkd6: the nation\'s development',
     '2 => {} // mutation: no development cut',
     'mkd_6 default wage'),
]
