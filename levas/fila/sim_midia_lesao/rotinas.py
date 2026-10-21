ROTINAS = [('historia_punivel_do_jogador', 0x7357c0, [0x8b, 0x44, 0x24, 0x04, 0x53, 0x55]),
           ('escolher_tipo_de_lesao', 0x62d140, [0x51, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x18]),
           ('categoria_da_noticia', 0x785670, [0x8b, 0x01, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a história punível da imprensa, o tipo de lesão sorteado e a categoria da notícia',
        'the finable media story, the drawn injury type and the news category',
        [('historia_punivel_do_jogador', 'Media::get_finable_story_kind', '0x7357c0', 'media.cpp', 'thiscall',
          'o tipo da história da imprensa sobre o jogador quando é de indisciplina, senão 0',
          'the kind of the media story about the player when it is a misbehaviour kind, else 0'),
         ('escolher_tipo_de_lesao', 'Injury::choose_injury_type', '0x62d140', 'injury.cpp', 'thiscall',
          'sorteia o tipo de lesão da parte do corpo e da gravidade, mantendo a lesão atual ou preferindo a recorrente',
          'draws the injury type for the body part and severity, keeping the current injury or preferring the recurring one'),
         ('categoria_da_noticia', 'NewsItem::get_category', '0x785670', 'news.cpp', 'thiscall',
          'a categoria da notícia pelo milhar do tipo',
          'the news category by the thousands range of its type')])
