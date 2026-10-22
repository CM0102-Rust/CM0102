ROTINAS = [('noticias_nao_lidas', 0x786fe0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('gravar_marca_da_noticia', 0x786d30, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('clube_de_tecnico_humano', 0x7875e0, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('as notícias não lidas, a marca gravada na notícia da caixa e o clube de técnico humano',
        'the unread news, the slot written into the inbox news item and the human-managed club',
        [('noticias_nao_lidas', 'News::count_unread', '0x786fe0', 'news.cpp', 'thiscall',
          'as notícias não lidas da caixa do técnico humano, no total ou de uma categoria',
          "the unread news in a human manager's inbox, in total or for one category"),
         ('gravar_marca_da_noticia', 'News::set_item_slot', '0x786d30', 'news.cpp', 'thiscall',
          'grava um valor numa vaga da notícia da caixa do técnico humano e na cópia local dela',
          "writes a value into a slot of a news item in the human manager's inbox and into its local copy"),
         ('clube_de_tecnico_humano', 'news_club_has_human_manager', '0x7875e0', 'news.cpp', 'stdcall',
          'se um humano dirige o clube, sem contar as férias',
          'whether a human manager runs the club, ignoring holidays')])
