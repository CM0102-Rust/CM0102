# Mutacoes (sbA_1): todas mudam o saldo dos clubes ingleses, que pesa nas compras e vendas da carreira ENG ate
# 2002-06-01. As mudancas de estadio de 2004 ficam sem mutacao (nao rodam ate a data).
MUTACOES = [
    ('sbA_1_juros_dobrados',
     '        let v = ftol(b as f64 * read_f64(rdata::F64_0_0025));',
     '        let v = ftol(b as f64 * read_f64(rdata::F64_0_0025) * 4.0); // mutation'),
    ('sbA_1_treino_caro',
     '        (t - 2).wrapping_mul(1000)',
     '        (t - 2).wrapping_mul(9000) // mutation'),
    ('sbA_1_folha_sem_pagar',
     '    sub_i32_from_balance(fin, wages);',
     '    let _ = wages; // mutation'),
    ('sbA_1_custos_dobrados',
     '        ftol((rep + 125) as f64 * f)',
     '        ftol((rep + 125) as f64 * f * 6.0) // mutation'),
]
