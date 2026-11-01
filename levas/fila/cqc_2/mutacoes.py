# Mutacoes (cqc_2). O teto salarial roda a cada proposta de contrato ou de transferencia da IA e a cada folha salarial
# (que refaz o teto por jogador +0x14 das financas), e o valor sai nos salarios oferecidos que a carreira grava: a folha
# e a verba por semana divididas por 48 no lugar de 52 (sem pessoa: a folha salarial), a reputacao ao quadrado x 1,5
# (com pessoa, quando o contrato atual vale menos) e o presidente com 0,8 no lugar de 0,9 (clube com presidente e
# pessoa de reputacao perto da do clube). Contadores colados no efeito, depois das condicoes do trecho.
MUTACOES = [
    ('cqc2_folha_por_48',
     'let menos = cqc2_i(fin + 0xc).wrapping_sub(t.wrapping_add(verba) / 52); // cqc2: a folha e a verba por semana',
     'let menos = cqc2_i(fin + 0xc).wrapping_sub(t.wrapping_add(verba) / 48); // mutacao: por 48',
     'cqc_2 teto pela tabela'),
    ('cqc2_reputacao_vezes_1_5',
     'ftol(repf * repf * fator * f64_em(0x96a5c0) + cqc2_i(fin + 0x10) as f64) // cqc2: a reputação ao quadrado',
     'ftol(repf * repf * fator * f64_em(0x96a5c0) * 1.5 + cqc2_i(fin + 0x10) as f64) // mutacao: x 1,5',
     'cqc_2 teto pela reputacao'),
    ('cqc2_presidente_0_8',
     's = ftol(((cqc2_b(pres + 0x57) as i8 as i32) as f64 * f64_em(0x967898) + f64_em(0x9689d8)) * s as f64); // cqc2: o presidente',
     's = ftol(((cqc2_b(pres + 0x57) as i8 as i32) as f64 * f64_em(0x967898) + f64_em(0x9689b0)) * s as f64); // mutacao: 0,8',
     'cqc_2 presidente'),
]
