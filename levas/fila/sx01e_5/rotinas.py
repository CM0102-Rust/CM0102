# Os escritores de texto de tres noticias da midia (media.cpp; leitor sx01e, rodada 5, 23/11/2026): a festa pela
# contratacao (0x75b9c0, noticia 0xbe5), o elogio ao tecnico pelo titulo ou taca (0x75c5a0, noticia (2, 0x2e)) e o
# agradecimento do jogador (0x733000, 0xbda). Cada um inteiro, de um desvio, em pedacos com nome. Chamados so pelo
# despacho das noticias (NewsItem::write_text 0x77bc80, tabela NEWS_WRITERS) quando a noticia e aberta: na carreira
# automatica so pelo modo CM_TEXTO_NOTICIAS (levas/reorganizacao/proximas/textos_noticias.md); sem ele, a prova e o
# Lucas abrindo as noticias jogando comparando. Depende do sx01e_3 (MediaText, NewsName...) e do sx01e_4
# (media_story_outlet).
# Fronteiras conferidas na desmontagem: 0x75b9c0..0x75c377 (ret 0xc em 0x75c1d1 e 0x75c374; tabela de salto 0x75c378 de
# 6 casos do veiculo); 0x75c5a0..0x75cf99 (ret 0xc em 0x75cf96; tabela do veiculo depois); 0x733000..0x7338d7 (ret 0xc
# em 0x733771 e 0x7338d4; depois as tabelas 0x7338d8/0x733920 (0..0x82, o assunto do corpo), 0x7339a4 (4 variantes) e
# 0x7339b4/0x7339d0 (o assunto da manchete)).
# Convencoes conferidas na desmontagem (cada push seguido ate a chamada):
# - as rotinas: thiscall(midia, noticia, corpo, Text), ret 0xc;
# - 0x785830 get_slot (na 0x75b9c0 o indice das vagas depois do porta-voz corre num byte: 11 ou 15); 0x53e140
#   cdecl(n1, n2, n3, buf, 0x190, estilo, id); 0x5461b0 write_position_name cdecl(posicao, buf, 0xc8, 1) -> al;
#   o veiculo como na 0x75d210 (sx01e_4); 0x66f4e0 langlib_sprintf e 0x944c9f sprintf cdecl; 0x4ae660 set_string;
#   0x53e990 cdecl(pessoa, n1, n2, n3, 0); 0x4ae8a0 add_link (0x7d5 clube, 0x7d0 competicao, 0x7d7 pessoa).
# - o agradecimento escreve no buffer compartilhado 0xde1f64; o assistente (vagas 13..16) so com a vaga 12 = 0x420e9,
#   senao o nome vazio (o byte 0xad9c64).
SAVE = False
# 24/11/2026: provado pela carreira automatica com o modo de textos (CM_TEXTOS=1 na cadeia); deixou de ser TELAS.
ROTINAS = [('write_signing_celebration_news_text', 0x75b9c0, [0xb8, 0x50, 0x20, 0x00, 0x00]),
           ('write_trophy_praise_news_text', 0x75c5a0, [0xb8, 0xd4, 0x41, 0x00, 0x00]),
           ('write_player_gratitude_news_text', 0x733000, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x08, 0x08, 0x00, 0x00])]
DOCS = ('os textos da festa pela contratacao, do elogio pelo titulo e do agradecimento do jogador',
        'the texts of the signing celebration, the trophy praise and the player\'s gratitude',
        [('write_signing_celebration_news_text', 'Media::write_signing_celebration_news_text', '0x75b9c0', 'media.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo da noticia 0xbe5, a recepcao a uma contratacao: o elenco (vaga 9 = 2, com o porta-voz das vagas 11..14 ou "Jake Vaughan"), a torcida (1, com a pessoa nomeada, a posicao e o veiculo) ou a tia do cafe; pela marca da pessoa e pela forma (vaga 5), com os nomes do clube e do estadio. Vinculos: os nomes do clube, o porta-voz e o jogador quando conferem. A manchete: a torcida comemora, os jogadores ou o porta-voz comemoram, ou a tia do cafe',
          'writes the headline or body of news item 0xbe5, the welcome to a new signing: the squad (slot 9 = 2, with the spokesman of slots 11..14 or "Jake Vaughan"), the fans (1, with the named person, the position and the outlet) or the tea lady; by the person\'s mark and the form (slot 5), with the club\'s and the stadium\'s names. Links: the club\'s names, the spokesman and the player when they match. The headline: the fans celebrate, the players or the spokesman celebrate, or the tea lady'),
         ('write_trophy_praise_news_text', 'Media::write_trophy_praise_news_text', '0x75c5a0', 'media.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo da noticia (2, 0x2e), o elogio ao tecnico pelo titulo ou pela taca: a diretoria (vaga 9 = 3) ou a torcida (1) com o clube e a competicao, e um segundo paragrafo pelas duas notas (vagas 10 e 13), os dias no clube (vaga 12), a marca (vaga 11) e a palavra +0x69 da competicao (primeira temporada, feito notavel, ascensao, continuidade), juntados por "%s\\n\\n%s"; senao a tia do cafe. Vinculos: os nomes do clube, os do tecnico quando conferem e o da competicao. A manchete: a torcida comemora, a diretoria satisfeita ou encantada (abaixo ou acima de 5250), ou a tia do cafe',
          'writes the headline or body of news item (2, 0x2e), the praise of the manager for a title or cup: the board (slot 9 = 3) or the supporters (1) with the club and the competition, and a second paragraph by the two ratings (slots 10 and 13), the days at the club (slot 12), the mark (slot 11) and the competition\'s word +0x69 (first season, remarkable feat, a rise, more of the same), joined by "%s\\n\\n%s"; otherwise the tea lady. Links: the club\'s names, the manager\'s when they match and the competition\'s. The headline: the fans celebrate, the board pleased or delighted (below or above 5250), or the tea lady'),
         ('write_player_gratitude_news_text', 'Media::write_player_gratitude_news_text', '0x733000', 'media.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ou o corpo da noticia 0xbda, o jogador agradece o apoio do tecnico contra as criticas da imprensa, no buffer compartilhado 0xde1f64: pelo assunto (vaga 5, ate 0x82), a marca (6) e a variante (7), reunioes particulares, o relato do assistente (vagas 13..16 com a marca 0x420e9 na 12) ou de alguem da comissao, a sua resposta a imprensa, o garoto ou a promessa do clube, a especulacao da imprensa, a reacao do tecnico. Vinculos: os dois nomes do jogador e do tecnico quando conferem e os do clube. A manchete: aceita a decisao (0x13), reage ao tecnico, o relatorio do assistente, reconhece o apoio ou agradece ao tecnico',
          'writes the headline or body of news item 0xbda, the player thanks the manager for his support against press criticism, into the shared buffer 0xde1f64: by the topic (slot 5, up to 0x82), the mark (6) and the variant (7), private meetings, the assistant\'s report (slots 13..16 with the mark 0x420e9 in slot 12) or a coaching staff member\'s, your answer to the press, the club\'s youngster or starlet, press speculation, the manager\'s reaction. Links: both names of the player and the manager when they match and the club\'s. The headline: he accepts the decision (0x13), reacts to the manager, the assistant\'s report, acknowledges support or thanks the manager')])
