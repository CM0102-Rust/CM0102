# Mutacoes (cqq_13), nos registros dos clubes nas copas da Italia e no LDV Vans Trophy (78, 142 e 110 chamadas ate
# 2002-06-01 com as 8 ligas): os registros de staff vao ao historico e mexem na injury_word_0c e no moral (como os da
# cqi_10 e da cpz_4). O peso da classificacao nos grupos da Copa da Italia 0x14 em vez de 0x28; o da Copa da Serie C 0;
# o peso minimo da eliminacao no LDV 0x30 em vez de 1. Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqq13_peso_dos_grupos_da_copa_da_italia',
     '7, 0xa, 0x28, "cqq_13 classificado nos grupos da copa da italia")',
     '7, 0xa, 0x14, "cqq_13 classificado nos grupos da copa da italia") // mutacao: peso 0x14',
     'cqq_13 classificado nos grupos da copa da italia'),
    ('cqq13_peso_zero_na_copa_da_serie_c',
     '0xf, 0x14, 0x28, "cqq_13 classificado nos grupos da copa da serie c")',
     '0xf, 0x14, 0, "cqq_13 classificado nos grupos da copa da serie c") // mutacao: peso 0',
     'cqq_13 classificado nos grupos da copa da serie c'),
    ('cqq13_peso_da_eliminacao_no_trofeu',
     'let peso = scale_by_round(1, 0x50, cqq13_w(jogo + 0x34), 5); // cqq13: o peso da eliminação no LDV',
     'let peso = scale_by_round(0x30, 0x50, cqq13_w(jogo + 0x34), 5); // mutacao: minimo 0x30',
     'cqq_13 eliminado no trofeu'),
]
