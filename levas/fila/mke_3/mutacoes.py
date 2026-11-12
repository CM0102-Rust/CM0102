# Mutacoes (mke_3), no contrato inicial de cada pessoa (0x85e960; 8.431 chamadas ate 2002-06-01: todos os contratos
# no inicio do jogo, os regens que assinam e os tecnicos contratados). O comparador e lado a lado (o contrato inteiro),
# entao qualquer mudanca no contrato e acusada ja no inicio; na carreira:
# - a multa da clausula de rebaixamento x 0,5 em vez de x 0,85 (lealdade < 15): o preco de saida de quem cai muda
#   (contador "mke_3 relegation clause");
# - o inicio do contrato do banco de dados volta 4 anos em vez de 3 (24 a 29 anos): muda o tempo de casa, as faixas de
#   renovacao e a lei Bosman.
MUTACOES = [
    ('mke3_multa_rebaixamento',
     'ftol(v as f64 * read_f64(rdata::F64_0_85)) // mke3: the relegation fee × 0.85',
     'ftol(v as f64 * read_f64(rdata::F64_0_5)) // mutation: the relegation fee × 0.5',
     'mke_3 relegation clause'),
    ('mke3_inicio_quatro_anos',
     'write_u16(sy, read_u16(G_CURRENT_DATE + 2).wrapping_sub(3)); // mke3: the database start goes back 3 years',
     'write_u16(sy, read_u16(G_CURRENT_DATE + 2).wrapping_sub(4)); // mutation: 4 years',
     'mke_3 relegation clause'),
]
