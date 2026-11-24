# Mutacoes (sx09b_4, telas: a carreira automatica nao abre a tela; para provar o comparador numa sessao do Lucas).
# - um amarelo so vira "<n> yellow cards" (o motivo mais comum: 1 jogo por amarelos);
# - a forma do ambito trocada por 0 no texto de N jogos.
MUTACOES = [
    ('sx09b_um_amarelo',
     '1 if n == 1 => story_lang!(out, 0x9c3528), // sx09b_4: "Yellow card"',
     '1 if n == 0x7fff => story_lang!(out, 0x9c3528), // mutation',
     'sx09b_4 ban of matches'),
    ('sx09b_forma_zero',
     'story_lang!(out, 0x9c32c0, matches, g, g, matches, sp); // sx09b_4: "<n> match <scope> ban"',
     'story_lang!(out, 0x9c32c0, matches, 0, 0, matches, sp); // mutation',
     'sx09b_4 ban of matches'),
]
