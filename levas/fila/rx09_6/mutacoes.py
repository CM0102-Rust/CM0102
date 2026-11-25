# Mutacoes (rx09_6), com a Espanha e a Suecia ate 2002-09-01 e a base NORMAL (acusadas pelo comparador lado a lado):
# - o codigo do vencedor da repescagem 0x3c em vez de 0x3b (maio e junho de 2002);
# - nos grupos o melhor codigo 1 a 4 vira 2 em vez de 1 (o fim de cada rodada dos grupos).
MUTACOES = [
    ('rx09_6_repescagem_0x3c',
     'let code = |v: u8| if v == 1 { Some(0x3b) } else if v == lost { Some(0x3f) } else { None }; // rx09_6: the relegation play-off codes',
     'let code = |v: u8| if v == 1 { Some(0x3c) } else if v == lost { Some(0x3f) } else { None }; // mutation: 0x3c',
     'rx09_6 segunda b progress'),
    ('rx09_6_melhor_dos_grupos_2',
     'let b = if (1..=4).contains(&best) { 1 } else { best };',
     'let b = if (1..=4).contains(&best) { 2 } else { best }; // mutation',
     'rx09_6 segunda b progress'),
]
