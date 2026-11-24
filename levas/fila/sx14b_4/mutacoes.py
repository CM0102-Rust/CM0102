# Mutacoes (sx14b_4), de TELAS: a carreira automatica nao roda nenhuma destas; servem para provar o comparador numa
# sessao do Lucas (lado a lado nos rotulos, na disponibilidade e no bloqueio da liberacao).
# - o rotulo do pedido de adiamento troca pelo da reclamacao do arbitro;
# - o bloqueio da liberacao nao corta o humor dos favoritos.
MUTACOES = [
    ('sx14b_4_rotulo_trocado',
     '        2 => story_lang!(out, 0xa11558), // "Request match postponement"',
     '        2 => story_lang!(out, 0xa1153c), // mutation',
     'sx14b_4 FA request label'),
    ('sx14b_4_bloqueio_sem_favoritos',
     '    row.set_favourites_mood(row.favourites_mood().wrapping_sub(750));',
     '    // mutation: the favourites mood stays',
     'sx14b_4 board blocks release'),
]
