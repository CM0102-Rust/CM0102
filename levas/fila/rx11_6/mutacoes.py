# Mutacoes (rx11_6), pela carreira ENG ate 2002-06-01 (os premios ingleses nascem na criacao do jogo; os do mes de
# agosto a maio e os do ano no fim da temporada; a reputacao ganha com um premio muda o mercado):
# - os premios do mes so a partir de setembro (o de agosto de 2001 nao sai);
# - o jogador do ano dos jogadores sem reputacao (8000/5000 -> 0/0).
# O construtor fica sem mutacao propria (o molde comum do sx05b_5, provado pelas outras nacoes); o comparador confere a
# vtable, o id, o tipo e a contagem.
MUTACOES = [
    ('rx11_6_mes_de_setembro',
     'AwardSpec::Month([n, i + j, k(key), rep.0, rep.1, comp, 7, y, 4, y + 1, f, players]);',
     'AwardSpec::Month([n, i + j, k(key), rep.0, rep.1, comp, 8, y, 4, y + 1, f, players]); // mutation',
     'rx11_6 english awards built'),
    ('rx11_6_jogador_do_ano_sem_reputacao',
     'k(KEY_STAFF_COMP_ENGLISH_PLAYERS_PLAYER_OF_THE_YEAR), 8000, 5000,',
     'k(KEY_STAFF_COMP_ENGLISH_PLAYERS_PLAYER_OF_THE_YEAR), 0, 0, // mutation\n       ',
     'rx11_6 english awards built'),
]
