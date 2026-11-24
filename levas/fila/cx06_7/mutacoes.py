# Mutacoes (cx06_7), com as 8 ligas e ATE 2003-08-15 (a Copa das Confederacoes de 2003). Os comparadores rodam so a
# original: as mutacoes sao para a carreira.
# - o grupo A fica com as selecoes em ordem inversa nas linhas: os jogos fixos de cada rodada (pelas linhas) mudam de
#   adversario e de data;
# - o grupo B idem: as selecoes 7, 5, 3, 1;
# - as semifinais cruzam errado (1o do A contra 1o do B): outros confrontos em 06/08/2003.
MUTACOES = [
    ('cx06_7_grupo_a_invertido',
     '        let team = read_usize(read_usize(this + 0xee) + 8 * row as usize);',
     '        let team = read_usize(read_usize(this + 0xee) + 8 * (3 - row as usize)); // mutation',
     'cx06_7 confed group a'),
    ('cx06_7_grupo_b_invertido',
     '        write_usize(teams + 4 * k, read_usize(read_usize(this + 0xee) + 4 * (2 * k + 1)));',
     '        write_usize(teams + 4 * k, read_usize(read_usize(this + 0xee) + 4 * (2 * (3 - k) + 1))); // mutation',
     'cx06_7 confed group b'),
    ('cx06_7_semifinais_erradas',
     '    [a(1), b(0), a(0), b(1)]',
     '    [a(0), b(0), a(1), b(1)] // mutation',
     'cx06_7 confed semi-finals'),
]
