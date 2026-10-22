ROTINAS = [('partida_das_vagas_da_noticia', 0x4b0010, [0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14]),
           ('resultado_da_eliminatoria_na_noticia', 0x4b0890, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('dia_da_rodada_da_copa', 0x521f10, [0x66, 0x8b, 0x4c, 0x24, 0x0c])]
DOCS = ('a partida guardada na notícia, o resultado da eliminatória para a notícia e o dia da rodada da copa',
        'the match stored in the news item, the cup tie result for the news and the cup round match day',
        [('partida_das_vagas_da_noticia', 'comp_util_read_fixture_from_news_slots', '0x4b0010', 'comp_util.cpp', 'cdecl',
          'refaz a partida a partir de 18 vagas seguidas da notícia e liga os clubes, o estádio e a competição',
          'rebuilds the fixture from eighteen consecutive news slots and resolves its clubs, stadium and competition'),
         ('resultado_da_eliminatoria_na_noticia', 'comp_util_classify_tie_result_for_news', '0x4b0890', 'comp_util.cpp', 'cdecl',
          'a categoria do resultado da eliminatória para o clube, pela margem final, o mando e a diferença de reputação',
          "the category of the cup tie result for the club, from the final margin, the side and the reputation gap"),
         ('dia_da_rodada_da_copa', 'cup_set_round_match_day', '0x521f10', 'Cup.cpp', 'cdecl',
          'grava um dos oito dias da rodada da copa: o mando, o dia da semana, o horário e o código',
          'stores one of the eight cup round match days: the venue flags, the weekday, the time slot and the code')])
