# Mutacoes (cx16_2), ENG ate 2002-08-01; as tabelas nao voltam a carreira ate agosto (as rodadas sao de 2004), entao
# sao acusadas pelos comparadores (lado a lado):
# - a regra de sorteio 0x200 da campanha de 2000 vira 0x100 (o mata-mata da criacao do jogo);
# - os grupos da America Central com duas rodadas sem isentas (a virada, campanha de 2004);
# - o rotulo dos grupos europeus o do grupo seguinte (a criacao e a virada; o comparador confere as saidas).
MUTACOES = [
    ('cx16_2_regra_de_sorteio',
     "let draw = if year == 0x7d0 { 0x200 } else { 0 }; // cx16_2: the 2000 campaign's draw rule",
     "let draw = if year == 0x7d0 { 0x100 } else { 0 }; // mutation: another draw rule",
     'cx16_2 concacaf qualifying round table'),
    ('cx16_2_grupos_curtos',
     'let short = (others == 1 && f == others) || others == 2; // cx16_2: two rounds when Central America has exempt nations',
     'let short = (others == 1 && f == others) || others != 2; // mutation: short groups without exempt nations',
     'cx16_2 concacaf qualifying round table'),
    ('cx16_2_europa_rotulo',
     'let label = (f as i16).wrapping_add(0x3e9); // cx16_2: the European group label',
     'let label = (f as i16).wrapping_add(0x3ea); // mutation: the next group label',
     'cx16_2 european qualifying round table'),
]
