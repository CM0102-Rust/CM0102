ROTINAS = [('entrada_da_aposentadoria', 0x7a7ed0, [0x8b, 0x41, 0x04, 0x8b, 0x49, 0x08]),
           ('registrar_competicoes_do_amistoso', 0x5ca030, [0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('vaga_da_noticia', 0x785830, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a entrada da aposentadoria, as competições do amistoso e a vaga da notícia',
        'the retirement entry, the friendly competitions and the news slot',
        [('entrada_da_aposentadoria', 'PLAYER_REGEN::find_retirement_entry', '0x7a7ed0', 'player_regen.cpp', 'thiscall',
          'a entrada da pessoa na lista das aposentadorias (por lesão ou já anunciada)',
          'the person\'s entry in the retirement list (by injury or already announced)'),
         ('registrar_competicoes_do_amistoso', 'Friendly::register_comp_objects', '0x5ca030', 'friendly.cpp', 'thiscall',
          'grava os objetos dos amistosos e dos torneios na tabela das competições',
          'stores the friendly and tournament objects in the competition table'),
         ('vaga_da_noticia', 'NewsItem::get_slot', '0x785830', 'news.cpp', 'thiscall',
          'o valor de uma das 50 vagas da notícia',
          'the value of one of the 50 news item slots')])
