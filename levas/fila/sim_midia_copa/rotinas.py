ROTINAS = [('noticia_com_resposta_2', 0x735780, [0x56, 0x8b, 0x71, 0x08, 0x33, 0xd2]),
           ('tem_noticia', 0x735840, [0x56, 0x8b, 0x71, 0x08, 0x33, 0xd2]),
           ('importancia_da_partida', 0x5221f0, [0x8a, 0x51, 0x51, 0x53, 0x8a, 0x59, 0x3c])]
DOCS = ('as notícias do jogador na mídia e a importância da partida de copa',
        'the player\'s media stories and the cup match importance',
        [('noticia_com_resposta_2', 'story_with_response_two', '0x735780', 'media.cpp', 'thiscall',
          'o jogador tem uma notícia com a resposta 2 no registro da mídia',
          'whether the player has a story with response 2 in the media log'),
         ('tem_noticia', 'has_story', '0x735840', 'media.cpp', 'thiscall',
          'o jogador tem alguma notícia no registro da mídia',
          'whether the player has any story in the media log'),
         ('importancia_da_partida', 'cup_match_importance', '0x5221f0', 'Cup.cpp', 'thiscall',
          'a importância da partida da copa pela fase e a diferença do placar da ida',
          'the cup match importance by round and the first-leg score difference')])
