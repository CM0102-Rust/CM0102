# Os últimos nomes e as perguntas "a um amarelo da suspensão" dos órgãos (leitor sx18, rodada 5, 24/11/2026). Item de TELAS: rodam so quando a tela pede.
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
ROTINAS = [('rb_korea_get_name', 0x7e09e0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_northern_ireland_get_name', 0x7e0d60, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_norway_cup_get_name', 0x7e1080, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_australia_is_one_booking_from_ban', 0x7db8a0, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_denmark_is_one_booking_from_ban', 0x7dcf30, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_england_is_one_booking_from_ban', 0x7dd3a0, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_france_is_one_booking_from_ban', 0x7de260, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_greece_is_one_booking_from_ban', 0x7ded50, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_holland_is_one_booking_from_ban', 0x7df150, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_ireland_is_one_booking_from_ban', 0x7df910, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_italy_cup_is_one_booking_from_ban', 0x7dfd30, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_italy_league_is_one_booking_from_ban', 0x7e0160, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0])]
DOCS = ('os últimos nomes e as perguntas "a um amarelo da suspensão" dos órgãos',
        'the last names and the ruling bodies\' "one booking from a ban" questions',
        [('rb_korea_get_name', 'RbKorea::get_name', '0x7e09e0', 'ruling_body/rb_korea.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade sul-coreano no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xbb)',
          "writes the South Korean nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xbb)"),
         ('rb_northern_ireland_get_name', 'RbNorthernIreland::get_name', '0x7e0d60', 'ruling_body/rb_northern_ireland.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade norte-irlandês no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xc3)',
          "writes the Northern Irish nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xc3)"),
         ('rb_norway_cup_get_name', 'RbNorwayCup::get_name', '0x7e1080', 'ruling_body/rb_norway_cup.cpp', 'thiscall, ret 0xc',
          'escreve "Norwegian cup" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xa4)',
          'writes "Norwegian cup" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xa4)'),
         ('rb_australia_is_one_booking_from_ban', 'RbAustralia::is_one_booking_from_ban', '0x7db8a0', 'ruling_body/rb_australia.cpp', 'thiscall, ret 0x10',
          'diz se o próximo amarelo suspende o jogador pelas regras do órgão: com 4 amarelos ou um número par a partir de 6; as saídas, quando dadas, recebem 1 jogo e nenhum dia',
          "tells whether the next booking bans the player under the body's rules: at 4 bookings or an even count from 6; the outputs, when given, get one match and no days"),
         ('rb_denmark_is_one_booking_from_ban', 'RbDenmark::is_one_booking_from_ban', '0x7dcf30', 'ruling_body/rb_denmark.cpp', 'thiscall, ret 0x10',
          'diz se o próximo amarelo suspende o jogador pelas regras do órgão: quando 3 pontos a mais alcançam um limite (15, 30, depois de 14 em 14) que os pontos ainda não alcançaram; as saídas, quando dadas, recebem 1 jogo e nenhum dia',
          "tells whether the next booking bans the player under the body's rules: when 3 more points reach a threshold (15, 30, then every 14) the points are still under; the outputs, when given, get one match and no days"),
         ('rb_england_is_one_booking_from_ban', 'RbEngland::is_one_booking_from_ban', '0x7dd3a0', 'ruling_body/rb_england.cpp', 'thiscall, ret 0x10',
          'diz se o próximo amarelo suspende o jogador pelas regras do órgão: com 4, 9, 14 ou 19 amarelos, menos o 4º entre os dias 101 e 170 do ano; as saídas, quando dadas, recebem 1 jogo e nenhum dia',
          "tells whether the next booking bans the player under the body's rules: at 4, 9, 14 or 19 bookings, except the 4th between days 101 and 170 of the year; the outputs, when given, get one match and no days"),
         ('rb_france_is_one_booking_from_ban', 'RbFrance::is_one_booking_from_ban', '0x7de260', 'ruling_body/rb_france.cpp', 'thiscall, ret 0x10',
          'diz se o próximo amarelo suspende o jogador pelas regras do órgão: quando a janela das partidas com cartão, sem a mais antiga, já tem 2 ou mais; as saídas, quando dadas, recebem 1 jogo e nenhum dia',
          "tells whether the next booking bans the player under the body's rules: when the booking window, without the oldest match, already holds 2 or more; the outputs, when given, get one match and no days"),
         ('rb_greece_is_one_booking_from_ban', 'RbGreece::is_one_booking_from_ban', '0x7ded50', 'ruling_body/rb_greece.cpp', 'thiscall, ret 0x10',
          'diz se o próximo amarelo suspende o jogador pelas regras do órgão: quando o próximo amarelo é múltiplo de 3; as saídas, quando dadas, recebem 1 jogo e nenhum dia',
          "tells whether the next booking bans the player under the body's rules: when the next booking is a multiple of 3; the outputs, when given, get one match and no days"),
         ('rb_holland_is_one_booking_from_ban', 'RbHolland::is_one_booking_from_ban', '0x7df150', 'ruling_body/rb_holland.cpp', 'thiscall, ret 0x10',
          'diz se o próximo amarelo suspende o jogador pelas regras do órgão: com 2 amarelos; as saídas, quando dadas, recebem 1 jogo e nenhum dia',
          "tells whether the next booking bans the player under the body's rules: at 2 bookings; the outputs, when given, get one match and no days"),
         ('rb_ireland_is_one_booking_from_ban', 'RbIreland::is_one_booking_from_ban', '0x7df910', 'ruling_body/rb_ireland.cpp', 'thiscall, ret 0x10',
          'diz se o próximo amarelo suspende o jogador pelas regras do órgão: com 4, 7, 10 ou a cada 3 a partir de 13 amarelos; as saídas, quando dadas, recebem 1 jogo e nenhum dia',
          "tells whether the next booking bans the player under the body's rules: at 4, 7, 10 or every 3 from 13 bookings; the outputs, when given, get one match and no days"),
         ('rb_italy_cup_is_one_booking_from_ban', 'RbItalyCup::is_one_booking_from_ban', '0x7dfd30', 'ruling_body/rb_italy_cup.cpp', 'thiscall, ret 0x10',
          'diz se o próximo amarelo suspende o jogador pelas regras do órgão: com um número ímpar de amarelos; as saídas, quando dadas, recebem 1 jogo e nenhum dia',
          "tells whether the next booking bans the player under the body's rules: at an odd count of bookings; the outputs, when given, get one match and no days"),
         ('rb_italy_league_is_one_booking_from_ban', 'RbItalyLeague::is_one_booking_from_ban', '0x7e0160', 'ruling_body/rb_italy_league.cpp', 'thiscall, ret 0x10',
          'diz se o próximo amarelo suspende o jogador pelas regras do órgão: com 3, 6 ou um número par acima de 7; as saídas, quando dadas, recebem 1 jogo e nenhum dia',
          "tells whether the next booking bans the player under the body's rules: at 3, 6 or an even count above 7; the outputs, when given, get one match and no days")])
CAMPOS = []
INTERESSA = []
