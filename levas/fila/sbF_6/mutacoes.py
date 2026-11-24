# Mutacoes (sbF_6), carreira com a Holanda ate 2002-08-01. Nenhuma abre caixa.
# - o grupo de tres clubes na fase 17 e nao na 18: os grupos mudam (o comparador acusa: o original poe 4 clubes na
#   fase 17; a carreira muda os grupos de agosto);
# - o mando do sorteio do mata-mata invertido: quem joga em casa na 1a rodada (setembro) muda (a carreira acusa; o
#   comparador confere so o conjunto).
MUTACOES = [
    ('sbF_6_grupo_de_tres_na_17',
     'let last = g == 0x12; // sbF6: the last group has three clubs',
     'let last = g == 0x11; // mutation',
     'sbF_6 dutch cup group stage'),
    ('sbF_6_mando_invertido',
     'let (home, away) = if exe::utils_random(2) != 0 { (a, b) } else { (b, a) };',
     'let (home, away) = if exe::utils_random(2) != 0 { (b, a) } else { (a, b) }; // mutation',
     'sbF_6 dutch cup knockout'),
]
