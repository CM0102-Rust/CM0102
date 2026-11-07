# Mutacoes (cqv_7), nos inscritos da Copa da Inglaterra (juntados uma vez, em 3 de setembro de 2001, antes da primeira
# rodada): sem as 60 trocas da primeira divisao (120 sorteios a menos e outra ordem de entrada); a segunda divisao antes
# da terceira (os clubes entram em outras rodadas); os de fora da liga sem a ordem pela reputacao (outros 38 clubes e
# sem os sorteios do embaralhamento).
MUTACOES = [
    ('cqv7_sem_as_trocas_da_primeira',
     'for _ in 0..0x3c { // cqv7: 60 trocas na primeira divisão',
     'for _ in 0..0 { // mutacao: sem as trocas',
     'cqv_7 primeira divisao embaralhada'),
    ('cqv7_segunda_antes_da_terceira',
     'cqv7_divisao(this, &mut k, 0x9cf5c8, &[a, b, x1, x2]); cqv7_divisao(this, &mut k, 0x9cf5c4, &[x1, x2]); // cqv7: a terceira e a segunda',
     'cqv7_divisao(this, &mut k, 0x9cf5c4, &[x1, x2]); cqv7_divisao(this, &mut k, 0x9cf5c8, &[a, b, x1, x2]); // mutacao: a segunda antes',
     'cqv_7 primeira divisao embaralhada'),
    ('cqv7_fora_da_liga_sem_reputacao',
     'embaralhar(v.as_mut_ptr() as usize + CQV7_FORA as usize * 4, nout as u32, vagas_fora as i16 as u16 as u32); // cqv7: os de fora da liga pela reputação',
     'let _ = embaralhar; // mutacao: sem a ordem pela reputacao',
     'cqv_7 primeira divisao embaralhada'),
]
