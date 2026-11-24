# Mutacoes (sx05b_11). O AWOL corta reputacoes, cria antipatias e marca a volta (a carreira acusa quando acontece);
# a noticia e ensaiada pelo comparador. O AWOL e raro na carreira ENG ate 2002-06-01: as mutacoes podem sair por nao
# rodar (o rotulo do covered! diz).
# - o corte de reputacao ate 80%;
# - a volta sempre em 21 dias;
# (a entrega da noticia nao se compara: a original nao a expoe; sem mutacao dela.)
MUTACOES = [
    ('sx05b_11_corte_dobrado',
     '    let draw = exe::utils_random(0x29);',
     '    let draw = exe::utils_random(0x29) * 2; // mutation',
     'sx05b_11 player goes awol'),
    ('sx05b_11_volta_fixa',
     '    let days = if kind as u8 == 4 { exe::utils_random(0x1f) + 2 } else { exe::utils_random(0x96) + 0x15 };',
     '    let days = if kind as u8 == 4 { exe::utils_random(0x1f) + 2 } else { exe::utils_random(0x96) * 0 + 0x15 }; // mutation',
     'sx05b_11 player goes awol'),
]
