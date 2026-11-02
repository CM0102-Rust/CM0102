# Mutacoes (cqf_10), no olheiro na partida observada (0x7b5be0, 50770 chamadas ate 2002-06-01): o relatorio (0x7b62a0)
# sorteia; as mutacoes mudam quem e avaliado ou quando a lista volta a observar, e a carreira pega pelos sorteios.
MUTACOES = [
    ('cqf10_nota_minima_5',
     'if (nota as i8) < 0xf { // cqf10: a nota mínima',
     'if (nota as i8) < 0x5 { // mutacao: nota minima 5',
     'cqf_10 avaliar jogador'),
    ('cqf10_reputacao_2000',
     '&& reputacao(p, olheiro, 0) >= 0xfa0 { // cqf10: reputação vista',
     '&& reputacao(p, olheiro, 0) >= 0x7d0 { // mutacao: reputacao 2000',
     'cqf_10 avaliar jogador'),
    ('cqf10_comeco_em_tres_dias',
     'let amanha = mais(0xae2c90, tmp.as_mut_ptr() as usize, 1); // cqf10: amanhã',
     'let amanha = mais(0xae2c90, tmp.as_mut_ptr() as usize, 3); // mutacao: tres dias',
     'cqf_10 relatorio'),
]
