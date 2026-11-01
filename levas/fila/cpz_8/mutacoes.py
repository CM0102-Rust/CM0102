# Mutacoes (cpz_8), todas nos jogos vizinhos do clube (0x5ae9d0, 821 chamadas ate 2002-06-01 na cobertura de 17/10,
# a cada jogo juntado ao calendario): o jogo de dois dias antes vai dois dias antes (e nao um) e o de dois dias
# depois vai dois dias depois (e nao um), com a data nova conferida pela 0x5ad780: o jogo cai noutra data (ou fica
# onde estava, se a data nova tiver outro jogo) e o calendario muda (os jogos e as datas entram na impressao do banco
# e nos sorteios das partidas). A marca +0x43 invertida move os jogos de dois dias antes que a original deixa e deixa os
# que ela move.
MUTACOES = [
    ('cpz8_antecipa_dois_dias', '                    let r = cpz8_menos(d.as_ptr() as usize, v.as_mut_ptr(), 1); // cpz8: um dia antes',
     '                    let r = cpz8_menos(d.as_ptr() as usize, v.as_mut_ptr(), 2); // mutacao: dois dias antes',
     'cpz_8 jogo antecipado'),
    ('cpz8_adia_dois_dias', '        let r = cpz8_mais(d.as_ptr() as usize, v.as_mut_ptr(), 1); // cpz8: um dia depois',
     '        let r = cpz8_mais(d.as_ptr() as usize, v.as_mut_ptr(), 2); // mutacao: dois dias depois',
     'cpz_8 jogo adiado'),
    ('cpz8_marca_invertida', '                if f2[0x43] == 0xff { // cpz8: o jogo de dois dias antes sem a marca',
     '                if f2[0x43] != 0xff { // mutacao: a marca invertida',
     'cpz_8 jogo dois dias antes'),
]
