# Mutacoes (cqu_10), na Liga dos Campeoes asiatica montada no comeco do jogo (as fases jogam na temporada 2001/02): o
# vice da J1 (e da liga coreana) no lugar do campeao; 20 escolhas no embaralhamento dos clubes (os sorteios mudam e os
# escolhidos de cada nacao tambem); os 16 da zona oeste de tras para a frente (os confrontos da fase 0 mudam).
MUTACOES = [
    ('cqu10_vice_da_j1',
     '    let mut c = colocado(liga, 0); // cqu10: o campeão da liga',
     '    let mut c = colocado(liga, 1); // mutacao: o vice da liga',
     'cqu_10 clube da j1'),
    ('cqu10_20_escolhas',
     '0x28); // cqu10: 40 escolhas no embaralhamento',
     '0x14); // mutacao: 20 escolhas no embaralhamento',
     'cqu_10 sorteio dos clubes'),
    ('cqu10_zona_oeste_de_tras_para_frente',
     'cqu10_pd(times + i * 4, cqu10_d(cqu10_d(this + 0xb2) + 0x40 + i * 4)); // cqu10: o time i da zona oeste',
     'cqu10_pd(times + i * 4, cqu10_d(cqu10_d(this + 0xb2) + 0x40 + (n as usize - 1 - i) * 4)); // mutacao: de tras para a frente',
     'cqu_10 time da zona oeste'),
]
