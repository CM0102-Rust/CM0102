# Mutacoes (cpw_3), todas na oferta de emprestimo pelo jogador da lista (0x842740, 312 chamadas ate 2002-06-01 na
# cobertura de 17/10, pela avaliacao das listas dos clubes 0x84a020; "cpq_8 loan offer" passa de 256): o
# interesse minimo em 3,0 (e nao 3,5) faz ofertas de emprestimo que a original recusa; contar do inicio com interesse de
# 2,0 (e nao 2,75) muda o mes da volta do emprestimo (a data em que o jogador volta); o limit de 6 semanas a partir de
# hoje (e nao 3) recusa os emprestimos curtos. As tres mudam os emprestimos feitos e os elencos.
MUTACOES = [
    ('cpw3_interesse_3', '    if !(f1 >= read_f64_at(0x968e38)) { // cpw3: o interesse mínimo',
     '    if !(f1 >= 3.0) { // mutacao: o minimo em 3,0',
     'cpw_3 loan interest'),
    ('cpw3_inicio_2', '    if f2 >= read_f64_at(0x9690a8) { // cpw3: o interesse para contar do início',
     '    if f2 >= 2.0 { // mutacao: 2,0',
     'cpw_3 count from the start'),
    ('cpw3_seis_semanas', '    let r = mais_semanas(0xae2c90, &mut t, 3); // cpw3: as 3 semanas a partir de hoje',
     '    let r = mais_semanas(0xae2c90, &mut t, 6); // mutacao: 6 semanas',
     'cpw_3 three weeks'),
]
