ROTINAS = [('apagar_repetidos_das_convocacoes', 0x76c060, [0x51, 0x53, 0x55, 0x32, 0xc0]),
           ('vaga_do_escalado', 0x8972c0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('escolher_comentarista', 0x72b810, [0x51, 0x53, 0x55, 0x8b, 0x6c, 0x24, 0x14])]
DOCS = ('as convocações repetidas das seleções, a vaga do escalado e o comentarista ex-internacional',
        'the repeated national squad entries, the selected player slot and the former international pundit',
        [('apagar_repetidos_das_convocacoes', 'national_teams_remove_duplicate_squad_entries', '0x76c060', 'national_teams.cpp', 'stdcall',
          'apaga as convocações repetidas da seleção principal e da seleção B da nação',
          'clears repeated squad entries from the senior and B national sides of a nation'),
         ('vaga_do_escalado', 'Tactics::find_selection_slot', '0x8972c0', 'tactics.cpp', 'thiscall',
          'a vaga (0..19) da escalação do clube em que o jogador está, ou 0xff',
          'the club selection slot (0..19) holding the player, or 0xff'),
         ('escolher_comentarista', 'Media::pick_former_international_pundit', '0x72b810', 'media.cpp', 'thiscall',
          'sorteia um ex-internacional (37 anos ou mais, 50 jogos pela seleção) para comentar, pesando a reputação da nação e os jogos',
          'draws a former international (37+, 50+ caps) as pundit, weighted by nation reputation and caps')])
