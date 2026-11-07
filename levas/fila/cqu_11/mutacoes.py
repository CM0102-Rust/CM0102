# Mutacoes (cqu_11), na Recopa asiatica montada no comeco do jogo: o vice da Copa do Imperador (e da copa coreana) no
# lugar do campeao; 20 escolhas no embaralhamento dos clubes; os 16 da zona oeste de tras para a frente.
MUTACOES = [
    ('cqu11_vice_da_copa_do_imperador',
     '    let c = colocado(copa, 0); // cqu11: o campeão da copa',
     '    let c = colocado(copa, 1); // mutacao: o vice da copa',
     'cqu_11 clube da copa do imperador'),
    ('cqu11_20_escolhas',
     '0x28); // cqu11: 40 escolhas no embaralhamento',
     '0x14); // mutacao: 20 escolhas no embaralhamento',
     'cqu_11 sorteio dos clubes'),
    ('cqu11_zona_oeste_de_tras_para_frente',
     'cqu11_pd(times + i * 4, cqu11_d(cqu11_d(this + 0xb2) + 0x40 + i * 4)); // cqu11: o time i da zona oeste',
     'cqu11_pd(times + i * 4, cqu11_d(cqu11_d(this + 0xb2) + 0x40 + (n as usize - 1 - i) * 4)); // mutacao: de tras para a frente',
     'cqu_11 time da zona oeste'),
]
