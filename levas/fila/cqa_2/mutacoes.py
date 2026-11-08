# Mutacoes (cqa_2). As tres na resposta ao pedido de amistoso (0x5c5f40, 464 chamadas na carreira ate 2002-06-01,
# pela atualizacao diaria dos amistosos 0x5c2460: os amistosos de pre-temporada que a IA marca em julho e agosto de
# 2001). A decisao (aceita ou recusa) e o mandante do amistoso aceito mudam o calendario dos clubes (jogos a mais ou
# a menos, renda, cansaco e moral dos elencos), que a carreira le. Os contadores ficam depois das condicoes, colados
# no efeito: a decisao, o fator da distancia (clubes com cidade do estadio) e a partida do amistoso aceito.
MUTACOES = [
    ('cqa2_convidado_com_o_dobro_da_folga',
     'let aceita = ra as f64 >= ro as f64 / f || f * read_f64_at(0x96a690) >= ro.wrapping_sub(ra) as f64; // cqa2: aceita pela reputação',
     'let aceita = ra as f64 >= ro as f64 / (f * 2.0) || f * read_f64_at(0x96a690) * 2.0 >= ro.wrapping_sub(ra) as f64; // mutacao: o dobro da folga',
     'cqa_2 decisao do amistoso'),
    ('cqa2_sem_o_fator_da_distancia',
     'f *= v; // cqa2: o fator da distância',
     'f *= 1.0 + 0.0 * v; // mutacao: sem o fator da distancia',
     'cqa_2 distancia do amistoso'),
    ('cqa2_mandante_trocado',
     '        cqa2_pd(pp + 0x1c, c2); // cqa2: o mandante do amistoso\n        cqa2_pd(pp + 0xc, cqa2_d(c2));\n        cqa2_pd(pp + 0x20, c3);\n        cqa2_pd(pp + 0x10, cqa2_d(c3));',
     '        cqa2_pd(pp + 0x1c, c3); // mutacao: o outro clube manda o amistoso\n        cqa2_pd(pp + 0xc, cqa2_d(c3));\n        cqa2_pd(pp + 0x20, c2);\n        cqa2_pd(pp + 0x10, cqa2_d(c2));',
     'cqa_2 amistoso aceito'),
]
