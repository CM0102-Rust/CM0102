# Mutacoes. A vaga 0 (goleiro) com a mascara do libero (2) em vez do goleiro (1): todo clube do computador reavaliado
# deixa de procurar goleiro e passa a procurar libero (alvos, ofertas e transferencias mudam; comparando, acusa). Sem
# redistribuir a verba depois de reavaliar o elenco (a verba de transferencias do clube fica a antiga: ofertas mudam).
# O clube nao suspende o jogador (estado da entrada fica abaixo de 100: continua mirando o lesionado ou o regen e faz
# oferta por ele; comparando, acusa).
MUTACOES = [
    ('cpp_goleiro', 'cpp5_w(v + 1, 1); // o goleiro',
     'cpp5_w(v + 1, 2); // mutacao: o goleiro vira libero',
     'cpp_5 vagas da formacao'),
    ('cpp_verba', 'verba(this, clube); // a verba de transferências',
     'let _ = verba; // mutacao: sem redistribuir a verba',
     'cpp_5 refazer lista'),
    ('cpp_suspender', '*((e + 8) as *mut u8) = 0x64; // o clube suspende o jogador',
     '// mutacao: o clube nao suspende o jogador',
     'cpp_5 suspender clube'),
]
