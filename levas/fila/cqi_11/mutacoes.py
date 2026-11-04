# Mutacoes (cqi_11), nos classificados da Copa dos Campeoes da CONCACAF (0x4c11a0, 2 chamadas ate 2002-06-01 com as
# 8 ligas, pela atualizacao diaria da copa): mudam quem entra (jogos de clubes da MLS, liga jogavel da carreira) ou a
# contagem dos sorteios (o embaralhamento da 0x4afbd0 sorteia 3 x k vezes), que a impressao pega no dia.
MUTACOES = [
    ('cqi11_terceiro_mexicano',
     'pd(t(3), l.mex[1]); // cqi11: o segundo mexicano',
     'pd(t(3), l.mex[2]); // mutacao: o terceiro mexicano no lugar do segundo',
     'cqi_11 classificados'),
    ('cqi11_quatro_centro_americanos_no_sorteio',
     'ordenar(l.centro.as_mut_ptr() as usize, l.n[1], 3); // cqi11: três centro-americanos',
     'ordenar(l.centro.as_mut_ptr() as usize, l.n[1], 4); // mutacao: embaralha para quatro',
     'cqi_11 classificados'),
    ('cqi11_sempre_o_vice_da_mls',
     'if cqi11_d(t(0)) == cqi11_d(t(1)) { // cqi11: o campeão das duas dá a vaga ao vice da MLS',
     'if true { // mutacao: a vaga do campeao da MLS sempre vai ao vice',
     'cqi_11 campeao da mls'),
]
