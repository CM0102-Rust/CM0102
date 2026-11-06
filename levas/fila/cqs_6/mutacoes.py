# Mutacoes (cqs_6), na final do Trofeu da Liga (montada em 2002-070, jogada em abril de 2002): os dois finalistas com o
# mando trocado; a final sem o sorteio da rodada (sem mando nem data); a final no ano seguinte (fora da carreira).
MUTACOES = [
    ('cqs6_finalistas_trocados', '        cqs6_pd(l, c);\n    }\n    if let Some(c) = cqs6_vivo(cqs6_d(cqs6_d(cqs6_d(this + 0xc)) + 0xa7), cqs6_w(this + 0x3e) as i32, 1) { // cqs6: o finalista das chaves\n        cqs6_pd(l + 4, c);', '        cqs6_pd(l + 4, c); // mutacao: o mando trocado\n    }\n    if let Some(c) = cqs6_vivo(cqs6_d(cqs6_d(cqs6_d(this + 0xc)) + 0xa7), cqs6_w(this + 0x3e) as i32, 1) { // cqs6: o finalista das chaves\n        cqs6_pd(l, c);', 'cqs_6 final do trofeu'),
    ('cqs6_final_sem_sorteio', 'sortear(f, 0); // cqs6: o sorteio da final', 'let _ = sortear; // mutacao', 'cqs_6 final do trofeu'),
    ('cqs6_final_no_ano_seguinte', 'cqs6_w(this + 0x40) as u16 as u32, 1, 1, // cqs6: o ano da final', '(cqs6_w(this + 0x40) + 1) as u16 as u32, 1, 1, // mutacao', 'cqs_6 final do trofeu'),
]
