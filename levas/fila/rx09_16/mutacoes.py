# Mutacoes (rx09_16), com a Inglaterra ate 2005-12-31 (a campanha asiatica de 2006: sorteio em marco de 2005):
# - sem isentos, o grupo 1 com tres selecoes (a campanha de 2006 nao tem isentos asiaticos: a Alemanha sedia e o Brasil
#   e o campeao): o comparador lado a lado acusa e o sorteio muda;
# - 19 trocas em vez de 20 no embaralhamento: o gerador de sorteios fica uma chamada atras (a carreira acusa);
# - as linhas do primeiro grupo de tras para a frente: o comparador da tabela acusa.
MUTACOES = [
    ('rx09_16_grupo_1_de_tres',
     '        0 => false,\n        1 => g == 1,\n        2 => g == 3 || g == 6, // rx09_16: two exempt, groups 3 and 6 of three',
     '        0 => g == 1, // mutation\n        1 => g == 1,\n        2 => g == 3 || g == 6,',
     'rx09_16 asian first round draw'),
    ('rx09_16_dezenove_trocas',
     '    for _ in 0..0x14 {\n        let a = exe::utils_random(0xa) as u8 as i8 as isize as usize;',
     '    for _ in 0..0x13 { // mutation: 19 swaps\n        let a = exe::utils_random(0xa) as u8 as i8 as isize as usize;',
     'rx09_16 asian first round draw'),
    ('rx09_16_linhas_invertidas',
     'exe::league_init_table_row(l, k as u32, read_usize(draw + 4 * k as i8 as usize), 0, 0); // rx09_16: the k-th team of the first group',
     'exe::league_init_table_row(l, k as u32, read_usize(draw + 4 * (l.club_count() as usize - 1 - k as usize)), 0, 0); // mutation: reversed',
     'rx09_16 asian first group table'),
]
