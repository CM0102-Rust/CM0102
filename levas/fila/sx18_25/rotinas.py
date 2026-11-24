# Os nomes dos órgãos disciplinares (França a Noruega) (leitor sx18, rodada 5, 24/11/2026). Item de TELAS: rodam so quando a tela pede.
# get_name (vtable +0x10): o texto do ambito da suspensao, pedido por RulingBody::get_ban_text (0x7e88d0, via
# Discipline::get_slot_text 0x55d3c0) na ficha do jogador (0x87618a): caminho no jogo: a ficha de um jogador suspenso
# ou com cartoes (a linha da suspensao/disciplina), com a liga do orgao carregada. is_one_booking_from_ban (vtable
# +0x14): pela RulingBody::check_one_booking_from_ban (0x7e8d60, via Discipline::is_one_booking_from_ban 0x55d360) no
# elenco do clube (0x484eb0, o aviso "a um amarelo da suspensao") e pela get_ban_text na ficha do jogador.
# Convencoes conferidas: get_name thiscall(buf, tamanho, forma) ret 0xc, o retorno nao e lido (0x7e8a4d);
# is_one_booking_from_ban thiscall(entrada, &jogos, &dias, competicao) ret 0x10, devolve 0/1 (lido em 0x7e8d8b).
# get_name: tamanho < 20 (com sinal) mostra a caixa da versao (show_error_at) e nada mais; 0x66f4e0 langlib_sprintf,
# 0x6181d0 adjetivo de nacionalidade (nacao, buf, tamanho, forma), 0x6177f0 adjetivo do continente, 0x616f20 nome da
# regiao (2, buf, tamanho, 4). Comparadores: a copia num buffer local primeiro, depois a original; texto e saidas.
SAVE = False
TELAS = True
ROTINAS = [('rb_finland_league_get_name', 0x7ddc20, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_france_get_name', 0x7de1a0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_germany_cup_get_name', 0x7de580, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_germany_league_get_name', 0x7de8c0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_greece_get_name', 0x7dec90, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_holland_get_name', 0x7df090, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_international_get_name', 0x7df320, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_ireland_get_name', 0x7df850, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_italy_cup_get_name', 0x7dfc90, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_italy_league_get_name', 0x7e00c0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_japan_cup_get_name', 0x7e03b0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_japan_league_get_name', 0x7e0690, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os nomes dos órgãos disciplinares (França a Noruega)',
        "the ruling bodies' names (France to Norway)",
        [('rb_finland_league_get_name', 'RbFinlandLeague::get_name', '0x7ddc20', 'ruling_body/rb_finland_league.cpp', 'thiscall, ret 0xc',
          'escreve "Finnish league" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xb4)',
          'writes "Finnish league" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xb4)'),
         ('rb_france_get_name', 'RbFrance::get_name', '0x7de1a0', 'ruling_body/rb_france.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade francês no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xe6)',
          "writes the French nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xe6)"),
         ('rb_germany_cup_get_name', 'RbGermanyCup::get_name', '0x7de580', 'ruling_body/rb_germany_cup.cpp', 'thiscall, ret 0xc',
          'escreve "German cup" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xba)',
          'writes "German cup" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xba)'),
         ('rb_germany_league_get_name', 'RbGermanyLeague::get_name', '0x7de8c0', 'ruling_body/rb_germany_league.cpp', 'thiscall, ret 0xc',
          'escreve "German league" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xba)',
          'writes "German league" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xba)'),
         ('rb_greece_get_name', 'RbGreece::get_name', '0x7dec90', 'ruling_body/rb_greece.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade grego no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xbc)',
          "writes the Greek nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xbc)"),
         ('rb_holland_get_name', 'RbHolland::get_name', '0x7df090', 'ruling_body/rb_holland.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade holandês no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xbc)',
          "writes the Dutch nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xbc)"),
         ('rb_international_get_name', 'RbInternational::get_name', '0x7df320', 'ruling_body/rb_international.cpp', 'thiscall, ret 0xc',
          'escreve "International" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0x9f)',
          'writes "International" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0x9f)'),
         ('rb_ireland_get_name', 'RbIreland::get_name', '0x7df850', 'ruling_body/rb_ireland.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade irlandês no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xe3)',
          "writes the Irish nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xe3)"),
         ('rb_italy_cup_get_name', 'RbItalyCup::get_name', '0x7dfc90', 'ruling_body/rb_italy_cup.cpp', 'thiscall, ret 0xc',
          'escreve "Italian cup" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xcb)',
          'writes "Italian cup" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xcb)'),
         ('rb_italy_league_get_name', 'RbItalyLeague::get_name', '0x7e00c0', 'ruling_body/rb_italy_league.cpp', 'thiscall, ret 0xc',
          'escreve "Italian league" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xd2)',
          'writes "Italian league" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xd2)'),
         ('rb_japan_cup_get_name', 'RbJapanCup::get_name', '0x7e03b0', 'ruling_body/rb_japan_cup.cpp', 'thiscall, ret 0xc',
          'escreve "Japanese cup" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xb5)',
          'writes "Japanese cup" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xb5)'),
         ('rb_japan_league_get_name', 'RbJapanLeague::get_name', '0x7e0690', 'ruling_body/rb_japan_league.cpp', 'thiscall, ret 0xc',
          'escreve "Japanese league" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xb7)',
          'writes "Japanese league" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xb7)')])
CAMPOS = []
INTERESSA = []
