# Mutacoes (cx15_5), carreira das 8 ligas ate 2002-06-01. Nenhuma abre caixa.
# - o Leste e o Central trocados: as tabelas das divisoes da MLS e o calendario por divisao mudam;
# - os play-offs pelo menor numero de pontos primeiro: outros cruzamentos nas quartas de final.
MUTACOES = [
    ('cx15_5_leste_central_trocados',
     '    (0x413, [0x9d0390, 0x9d0394, 0x9d0384, 0x9d0380]),\n    (0x47b, [0x9d03a4, 0x9d0388, 0x9d03a0, 0x9d037c]),',
     '    (0x47b, [0x9d0390, 0x9d0394, 0x9d0384, 0x9d0380]), // mutation\n    (0x413, [0x9d03a4, 0x9d0388, 0x9d03a0, 0x9d037c]),',
     'cx15_5 mls division of a club'),
    ('cx15_5_menos_pontos_primeiro',
     '    read_i16(b + 4) as i32 - read_i16(a + 4) as i32\n}',
     '    read_i16(a + 4) as i32 - read_i16(b + 4) as i32 // mutation\n}',
     'cx15_5 play-off entries by points'),
]
