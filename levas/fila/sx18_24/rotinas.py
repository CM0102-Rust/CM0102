# Os nomes dos órgãos disciplinares (Argentina a Finlândia) (leitor sx18, rodada 5, 24/11/2026). Item de TELAS: rodam so quando a tela pede.
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
ROTINAS = [('rb_argentina_get_name', 0x7db2a0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_asia_get_name', 0x7db510, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_australia_get_name', 0x7db7e0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_belgium_cup_get_name', 0x7dbc00, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_belgium_league_get_name', 0x7dbfe0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_brazil_national_get_name', 0x7dc320, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_brazil_regional_get_name', 0x7dc660, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_croatia_get_name', 0x7dca20, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_denmark_get_name', 0x7dce70, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_england_get_name', 0x7dd2e0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_europe_get_name', 0x7dd5a0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_finland_cup_get_name', 0x7dd8f0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os nomes dos órgãos disciplinares (Argentina a Finlândia)',
        "the ruling bodies' names (Argentina to Finland)",
        [('rb_argentina_get_name', 'RbArgentina::get_name', '0x7db2a0', 'ruling_body/rb_argentina.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade argentino no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0x9f)',
          "writes the Argentine nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0x9f)"),
         ('rb_asia_get_name', 'RbAsia::get_name', '0x7db510', 'ruling_body/rb_asia.cpp', 'thiscall, ret 0xc',
          'escreve o nome da região 2 (a Ásia), na forma 4 no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0x9f)',
          "writes the name of region 2 (Asia), in form 4 into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0x9f)"),
         ('rb_australia_get_name', 'RbAustralia::get_name', '0x7db7e0', 'ruling_body/rb_australia.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade australiano no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xad)',
          "writes the Australian nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xad)"),
         ('rb_belgium_cup_get_name', 'RbBelgiumCup::get_name', '0x7dbc00', 'ruling_body/rb_belgium_cup.cpp', 'thiscall, ret 0xc',
          'escreve "Belgian cup" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xc2)',
          'writes "Belgian cup" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xc2)'),
         ('rb_belgium_league_get_name', 'RbBelgiumLeague::get_name', '0x7dbfe0', 'ruling_body/rb_belgium_league.cpp', 'thiscall, ret 0xc',
          'escreve "Belgian league" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xc5)',
          'writes "Belgian league" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xc5)'),
         ('rb_brazil_national_get_name', 'RbBrazilNational::get_name', '0x7dc320', 'ruling_body/rb_brazil_national.cpp', 'thiscall, ret 0xc',
          'escreve "Brazilian national" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xbe)',
          'writes "Brazilian national" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xbe)'),
         ('rb_brazil_regional_get_name', 'RbBrazilRegional::get_name', '0x7dc660', 'ruling_body/rb_brazil_regional.cpp', 'thiscall, ret 0xc',
          'escreve "Brazilian regional" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xbe)',
          'writes "Brazilian regional" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xbe)'),
         ('rb_croatia_get_name', 'RbCroatia::get_name', '0x7dca20', 'ruling_body/rb_croatia.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade croata no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xbc)',
          "writes the Croatian nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xbc)"),
         ('rb_denmark_get_name', 'RbDenmark::get_name', '0x7dce70', 'ruling_body/rb_denmark.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade dinamarquês no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xf1)',
          "writes the Danish nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xf1)"),
         ('rb_england_get_name', 'RbEngland::get_name', '0x7dd2e0', 'ruling_body/rb_england.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade inglês no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xdb)',
          "writes the English nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xdb)"),
         ('rb_europe_get_name', 'RbEurope::get_name', '0x7dd5a0', 'ruling_body/rb_europe.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo do continente europeu no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0x9f)',
          "writes the European continent adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0x9f)"),
         ('rb_finland_cup_get_name', 'RbFinlandCup::get_name', '0x7dd8f0', 'ruling_body/rb_finland_cup.cpp', 'thiscall, ret 0xc',
          'escreve "Finnish cup" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xb4)',
          'writes "Finnish cup" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xb4)')])
CAMPOS = []
INTERESSA = []
