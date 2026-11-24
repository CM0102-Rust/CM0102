# Mutacoes (sx22_1).
# - A noticia (adiada e abandonada, o auxiliar comum) vai so ao tecnico do mandante: a caixa de noticias do visitante
#   perde o item (acusada pela carreira se as noticias entram na impressao; ver o PRONTO).
# - A manchete da adiada sai no molde da abandonada: o comparador do escritor acusa toda manchete.
MUTACOES = [
    ('sx22_1_so_o_mandante',
     '    exe::news_send_to_club_manager(read_usize(G_NEWS), n, game.away_club()); // sx22_1: both managers',
     '    // mutation: only the home club\'s manager',
     'sx22_1 postponed news'),
    ('sx22_1_manchete_trocada',
     '        weather_news_headline(news, out, 0xad5220); // sx22_1: "<home> vs <away> <league> match postponed"',
     '        weather_news_headline(news, out, 0xad55f4); // mutation: the abandoned headline'),
]
