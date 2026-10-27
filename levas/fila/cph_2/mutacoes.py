# Mutacoes do lugar no elenco e das trocas. A do proprio clube troca principal e reservas e a do time B troca 3 e 4:
# devem acusar (o lugar decide escalacao e trocas; o comparando acusa na primeira chamada). As das trocas deixam o time
# B (3) e o clube com time B (4) de fora: o comparador decide pelo proprio lugar e manda esses casos so para a
# original, entao so a carreira acusa: PODEM ESCAPAR se nenhum time B trocar jogador ate 2002-06-01.
MUTACOES = [
    ('cph_lugar_reservas', 'return if *((pessoa + 0x5e) as *const u8) & 1 != 0 { 2 } else { 1 }; // principal ou reservas',
     'return if *((pessoa + 0x5e) as *const u8) & 1 != 0 { 1 } else { 2 }; // mutacao: trocados'),
    ('cph_lugar_marca', 'if marca == 0 { 4 } else { 3 }',
     'if marca == 0 { 3 } else { 4 } // mutacao: trocados'),
    ('cph_subir', 'cph2_mover(this, pessoa, clube, [1, 3]) // reservas ou time B',
     'cph2_mover(this, pessoa, clube, [1, 1]) // mutacao: sem o time B'),
    ('cph_descer', 'cph2_mover(this, pessoa, clube, [2, 4]) // principal ou clube com time B',
     'cph2_mover(this, pessoa, clube, [2, 2]) // mutacao: sem o clube com time B'),
]
