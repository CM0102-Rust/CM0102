# Mutacoes (sx09b_5). As disputas provavelmente nao acontecem na carreira de ferias ENG ate 2002-06-01: as duas so
# seriam acusadas com uma disputa (pelo comparador e pela carreira: o jogador livre muda de clube). Se escaparem, e
# por isso (a marca fica 0 em todos).
# - a audiencia que liberaria o jogador o mantem (sem a noticia de liberacao);
# - o clube da disputa nunca e achado.
MUTACOES = [
    ('sx09b_audiencia_sem_noticia',
     'post(this, person.addr(), club.addr(), 2, 0, 0); // sx09b_5: the outcome of a freed player',
     '// mutation: no news',
     'sx09b_5 hearing frees the player'),
    ('sx09b_clube_da_disputa',
     'return read_i32(e + 4); // +4: the club id of the dispute',
     'return -1; // mutation',
     'sx09b_5 dispute club found'),
]
