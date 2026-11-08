# Mutacoes (cqw_11), na eliminatoria da Oceania que o jogo de 2001 monta ja no comeco: a Nova Caledonia na contest
# (outra selecao nos grupos); as dez sem a ordem pela injury_word_0c (os grupos pares e impares mudam); a final com o
# campeao do segundo grupo mandando primeiro.
MUTACOES = [
    ('cqw11_com_a_nova_caledonia',
     'let fora = id == cqw11_d(0x9cf530) as i32 || id == cqw11_d(0x9cf41c) as i32; // cqw11: sem a Nova Caledônia e a Papua-Nova Guiné',
     'let fora = id == cqw11_d(0x9cf41c) as i32; // mutacao: com a Nova Caledonia',
     'cqw_11 selecao da eliminatoria'),
    ('cqw11_sem_ordenar',
     '    ordenar(cqw11_d(this + 0xef), k as u32, 4, 0x4b07d0); // cqw11: as dez pela reputação',
     '    let _ = ordenar; // mutacao: sem ordenar',
     'cqw_11 selecao da eliminatoria'),
    ('cqw11_final_invertida',
     '    cqw11_pd(v, cqw11_d(linha(this, 0))); // cqw11: o campeão do primeiro grupo manda primeiro\n    cqw11_pd(v + 4, cqw11_d(linha(cqw11_d(cqw11_d(this + 0xc)), 0)));',
     '    cqw11_pd(v + 4, cqw11_d(linha(this, 0))); // mutacao: invertida\n    cqw11_pd(v, cqw11_d(linha(cqw11_d(cqw11_d(this + 0xc)), 0)));',
     'cqw_11 final da oceania'),
]
