# Mutacoes (mwf_19), na Copa da Liga coreana montada no comeco do jogo e na virada: a ordem dos dez clubes invertida
# depois de ordenar (as cabecas pares e impares trocam: os grupos A e B mudam; o comparador tambem a pega); as linhas
# do grupo A de tras para a frente (so a ordem inicial da tabela; o comparador da tabela a pega). A advance_stage fica
# sem mutacao: o unico efeito dela e montar o mata-mata ao sair da fase 0, e qualquer troca (outra fase, sem
# mata-mata) deixa a copa sem fase seguinte (a caixa de erro e a copa parada); o comparador confere a fase.
MUTACOES = [
    ('mwf19_ordem_invertida',
     'exe::crt_qsort(read_usize(this + 0xee), n, 4, 0x4b0780usize); // mwf19: by last season\'s position',
     'exe::crt_qsort(read_usize(this + 0xee), n, 4, 0x4b0780usize); { let l = read_usize(this + 0xee); for a in 0..(n as usize / 2) { let (x, y) = (read_usize(l + a * 4), read_usize(l + (n as usize - 1 - a) * 4)); write_usize(l + a * 4, y); write_usize(l + (n as usize - 1 - a) * 4, x); } } // mutation: reversed',
     'mwf_19 korean league cup clubs'),
    ('mwf19_grupo_a_de_tras_para_frente',
     'exe::league_init_table_row(this, i as u8 as u32, read_usize(slot), 0, 0); // mwf19: row i',
     'exe::league_init_table_row(this, (4 - i) as u8 as u32, read_usize(slot), 0, 0); // mutation: rows backwards',
     'mwf_19 korean league cup group a'),
]
