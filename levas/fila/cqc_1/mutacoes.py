# Mutacoes (cqc_1). O fim de temporada das financas roda no dia anterior ao inicio da temporada de cada nacao-chave
# (40 vezes ate 2002-06-01), para cada clube da liga, e o saldo e lido pela carreira (verba, propostas, salarios,
# eventos financeiros da semana): o dividendo pela metade no saldo (quase todo clube paga), o dividendo do clube
# pequeno pelo presidente com 800 no lugar de 1000 (injury_word_0c abaixo de 4000 e saldo acima de 6000 x o indice do
# presidente) e os lugares sentados com 70 no lugar de 75 por lugar (dono do estadio com lugares livres). Contadores
# colados no efeito, depois das condicoes do trecho.
MUTACOES = [
    ('cqc1_dividendo_pela_metade',
     'core::ptr::write_unaligned(fin as *mut i64, cqc1_saldo(fin).wrapping_sub(v as i64)); // cqc1: o dividendo sai do saldo',
     'core::ptr::write_unaligned(fin as *mut i64, cqc1_saldo(fin).wrapping_sub((v / 2) as i64)); // mutacao: pela metade',
     'cqc_1 dividend'),
    ('cqc1_dividendo_pequeno_800',
     'v = r.wrapping_add(0x3e8).wrapping_mul(rep); // cqc1: o dividendo do pequeno pelo presidente',
     'v = r.wrapping_add(0x320).wrapping_mul(rep); // mutacao: 800 no lugar de 1000',
     'cqc_1 small club dividend by chairman'),
    ('cqc1_lugares_por_70',
     'charge_expense(fin, 0x8c, 0x12c, (v / 1000 + 1).wrapping_mul(50000).wrapping_add(v.wrapping_mul(75))); // cqc1: os lugares sentados',
     'charge_expense(fin, 0x8c, 0x12c, (v / 1000 + 1).wrapping_mul(50000).wrapping_add(v.wrapping_mul(70))); // mutacao: 70 por lugar',
     'cqc_1 seats'),
]
