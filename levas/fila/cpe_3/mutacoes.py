# Mutacoes das pequenas da tabela da liga. A da ordem pelo confronto inverte a posicao guardada (+4): so roda nas ligas
# com a regra 4 como primeira e nos empates em pontos com jogos: PODE ESCAPAR (o comparando acusa no primeiro). A das
# datas inverte o ano (o bsearch da 0x685640, 6.533 chamadas, deixa de achar): deve acusar, se o que a 0x685640 decide
# pela data aparecer na carreira. A da linha do clube nunca acha (31 chamadas diretas): deve acusar se alguma roda na
# carreira sem tela. A de ter o clube diz que nao (so pela virtual +0x80): PODE ESCAPAR.
MUTACOES = [
    ('cpe_confronto_posicao', 'let dif = *((b + 4) as *const i8) as i32 - *((a + 4) as *const i8) as i32; // a posição do confronto',
     'let dif = *((a + 4) as *const i8) as i32 - *((b + 4) as *const i8) as i32; // mutacao: invertida'),
    ('cpe_datas_ano', 'if ya < yb { // o ano antes\n        return -1;',
     'if ya < yb { // mutacao: invertido\n        return 1;'),
    ('cpe_linha_do_clube', 'if cpe3_d(r) == clube { // a linha do clube\n                return r;',
     'if cpe3_d(r) == clube { // mutacao: nunca acha\n                return 0;'),
    ('cpe_liga_tem_o_clube', 'return (r != 0) as u32; // a liga tem o clube',
     'return (r == 0) as u32; // mutacao: nao tem'),
]
