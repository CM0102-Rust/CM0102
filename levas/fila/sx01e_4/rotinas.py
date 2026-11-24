# Os escritores de texto de tres noticias da midia (media.cpp; leitor sx01e, rodada 5, 23/11/2026): o gesto obsceno
# (0x7572a0, noticia 0xbcb), a insatisfacao com a venda de um colega (0x75d210, 0xbe4) e a expulsao (0x759a10, 0xbe6).
# Cada um inteiro, de um desvio, em pedacos com nome. Chamados so pelo despacho das noticias (NewsItem::write_text
# 0x77bc80, tabela NEWS_WRITERS) quando a noticia e aberta: na carreira automatica so pelo modo de teste
# CM_TEXTO_NOTICIAS (levas/reorganizacao/proximas/textos_noticias.md); sem ele, a prova e o Lucas abrindo as noticias
# jogando comparando. Depende do sx01e_3 (MediaText, NewsName, compare_media_text_writer...); NewsMatch e
# append_shootout deste item servem ao seguinte.
# Fronteiras conferidas na desmontagem: 0x7572a0..0x757a45 (ret 0xc em 0x757a42; o resto e enchimento);
# 0x75d210..0x75da2e (ret 0xc em 0x75da2b; tabela de salto 0x75da30 de 6 casos do veiculo); 0x759a10..0x75a333 (ret
# 0xc em 0x75a330).
# Convencoes conferidas na desmontagem (cada push seguido ate a chamada):
# - as rotinas: thiscall(midia, noticia, corpo, Text), ret 0xc;
# - 0x54a110 Date::Date(registro); 0x7e4fa0 read_match_record_from_news cdecl(noticia, &k, registro) (k = 6 na
#   0x7572a0, 21 na 0x759a10; a vaga seguinte e lida por k); 0x944c9f sprintf cdecl(buf, "%d-%d", maior, menor) na
#   0x7572a0; 0x66f4e0 langlib_sprintf cdecl(buf, 0x9c116c, a, b) na 0x759a10 (a ordem pelo lado do clube);
#   0x729500 Match_Stats::fixture_entry_matches thiscall([0xb6b7c0], +0x1b, +0x11, +0x15, +0, +2 do registro);
#   0x4ae8a0 add_link (0x7d5 clube, 0x7d0 competicao, 0x7d3 placar com +0x1b, 0x7d7 pessoa, 0x7da o jogador na
#   manchete do gesto); 0x90c310 utils_format_ordinal cdecl(n, -1) -> texto copiado (strcpy) para o quadro;
#   0x7dad20 PunditList::get_pundit_name thiscall([0xb74430], tipo, comentarista, buf, &genero) ret 0x10; 0x6181d0
#   cdecl(nacao, buf, 0x78, forma); 0x66f4d0 langlib_get_language; o gesto escreve no buffer compartilhado 0xde1f64
#   (a frase dos penaltis num local e anexada com strcat).
# Defeitos do original: na 0x759a10 a derrota do clube do jogador como segundo clube do registro vira empate (resultado
# 0); a resposta em segunda pessoa e escolhida quando o proprio jogador (e nao o seu tecnico) e o tecnico humano da
# tela; resposta acima de 3 junta o buffer da resposta nunca escrito (MEDIA_TEXT_UNSET); sem o outro clube (vaga 10 =
# -1) o original usa um registrador velho (a copia usa nulo; a 0x759610 nunca grava -1 ali). Na 0x75d210 o veiculo
# (pundit ou jornal) e escrito e nenhum modelo o usa.
SAVE = False
TELAS = True
ROTINAS = [('write_obscene_gesture_news_text', 0x7572a0, [0x6a, 0xff, 0x68, 0x1b, 0xd9, 0x95, 0x00]),
           ('media_write_sale_unrest_news_text', 0x75d210, [0xb8, 0x10, 0x21, 0x00, 0x00]),
           ('write_player_sent_off_news_text', 0x759a10, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os textos do gesto obsceno, da insatisfacao com uma venda e da expulsao',
        'the texts of the obscene gesture, the unrest at a sale and the sending off',
        [('write_obscene_gesture_news_text', 'Media::write_obscene_gesture_news_text', '0x7572a0', 'media.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo da noticia 0xbcb, o jogador filmado fazendo gestos obscenos para a torcida, no buffer compartilhado 0xde1f64: o placar com o maior primeiro, a competicao e o adversario do registro da partida; pela forma (vaga 5) "durante o empate/vitoria/derrota ... provocou a torcida" ou "foi flagrado pelas cameras ..."; no empate, a frase dos penaltis (ganhou ou perdeu nos penaltis, pela vaga seguinte ao registro). Vinculos: os nomes do clube e do adversario, a competicao, o placar (quando a partida e achada nas estatisticas) e os dois nomes do jogador. A manchete "flagrado pelas cameras" ou "suspenso pelo gesto" pelo resto de (vaga 5 + 1) x id por 7, com o vinculo 0x7da',
          'writes the headline or body of news item 0xbcb, the player filmed making obscene gestures to the fans, into the shared buffer 0xde1f64: the score with the higher goals first, the competition and the opponent from the match record; by the form (slot 5) "during the draw/win/defeat ... incited fury" or "has been captured on camera ..."; for a draw, the shoot-out sentence (won or lost on penalties, by the slot after the record). Links: the club\'s and the opponent\'s names, the competition, the score (when the match statistics find the match) and the player\'s two names. The headline "caught on camera" or "banned for gesture" by (slot 5 + 1) x id mod 7, with link 0x7da'),
         ('media_write_sale_unrest_news_text', 'media_write_sale_unrest_news_text', '0x75d210', 'media.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo da noticia 0xbe4, o jogador insatisfeito com a venda de um colega: antes, o veiculo (o comentarista em casa, ou o jornal, radio, revista, jornal esportivo ou site, com a nacionalidade fora), que nenhum modelo usa; sem a forma de historia (vaga 17 diferente de 2) a frase da tia do cafe; senao, pelo caso, os jogadores que nao comentam, o boato de descontentamento com a decisao do tecnico, o jogador que critica o tecnico depois da compra ou o jogador preocupado com a saida do colega, com os nomes dos clubes vendedor e comprador. Vinculos: os nomes dos dois clubes e os do jogador; na manchete os nomes curtos do jogador e do vendido',
          'writes the headline or body of news item 0xbe4, the player unhappy at a team mate\'s sale: first the outlet (the pundit at home, or the newspaper, radio station, magazine, sports paper or website, with the nationality abroad), which no template uses; without the story form (slot 17 other than 2) the tea lady\'s line; otherwise, by case, the players who decline to comment, the rumours of discontent at the manager\'s decision, the player criticising the manager after the purchase or the player worried by his colleague\'s departure, with the selling and buying clubs\' names. Links: both clubs\' names and the player\'s; in the headline the short names of the player and of the sold player'),
         ('write_player_sent_off_news_text', 'Media::write_player_sent_off_news_text', '0x759a10', 'media.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo da noticia 0xbe6, a expulsao de um jogador: o resultado e o placar do registro da partida pelo lado do clube; pela forma (vaga 8) e o resultado (empate, vitoria, derrota) a historia com os clubes, o placar, o adversario e os cartoes vermelhos (vaga 11, ou o seu ordinal) e os tecnicos; com resposta (vaga 20), a resposta (em segunda pessoa quando o jogador e o tecnico humano da tela, senao o comentario do tecnico), juntadas por "%s\\n\\n%s". Vinculos: os nomes dos dois clubes, os nomes do jogador, do seu tecnico, do outro jogador e do outro tecnico quando conferem. A manchete "perde a cabeca" ou "deveria ser contido" pela vaga 8. Defeitos do original: a derrota como segundo clube do registro vira empate; a segunda pessoa compara o jogador (e nao o tecnico) ao tecnico humano; resposta acima de 3 junta um buffer nunca escrito',
          'writes the headline or body of news item 0xbe6, a player\'s sending off: the result and score from the match record from the club\'s side; by the form (slot 8) and the result (draw, win, defeat) the story with the clubs, the score, the opponent and the red cards (slot 11, or its ordinal) and the managers; with a response (slot 20), the reply (second person when the player is the human manager on screen, otherwise the manager\'s comment), joined by "%s\\n\\n%s". Links: both clubs\' names, the names of the player, his manager, the other player and the other manager when they match. The headline "lashes out" or "should be put on a leash" by slot 8. Defects of the original: a defeat as the record\'s second club reads as a draw; the second person compares the player (not the manager) with the human manager; a response above 3 joins a buffer never written')])
