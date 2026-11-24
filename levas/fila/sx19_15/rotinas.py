# Os nomes dos orgaos disciplinares (Noruega a copa espanhola) (vtable +0x10, get_name; rb_*.cpp; leitor sx19, rodada 5,
# 24/11/2026). Item de TELAS: rodam so quando a tela pede (0 chamadas na carreira do mundo ate 2002-08-01): o texto do
# ambito da suspensao, pedido por RulingBody::get_ban_text (0x7e88d0, via Discipline::get_slot_text 0x55d3c0) na ficha
# do jogador (0x87618a). Usa o molde do sx18_24 (RulingBodyName, ruling_body_name, compare_ruling_body_name):
# integrar depois dele. O sx19_16 e da mesma leva.
# Caminho no jogo (para o Lucas provar): um jogo com a liga do orgao carregada; a ficha de um jogador suspenso ou com
# cartoes nessa liga (a linha da suspensao/disciplina mostra o nome do orgao).
# Convencoes conferidas na desmontagem: thiscall(buf, tamanho, forma) ret 0xc, o retorno nao e lido (0x7e8a4d);
# tamanho < 20 (com sinal): a caixa de erro da versao (arquivo e linha do .cpp do orgao, 0xb67a34 = 0) e nada mais;
# senao 0x66f4e0 langlib_sprintf(buf, texto), 0x6181d0 adjetivo de nacionalidade (nacao, buf, tamanho, forma) ou
# 0x616f20 nome da regiao (codigo, buf, tamanho, 4).
SAVE = False
TELAS = True
ROTINAS = [('rb_norway_league_get_name', 0x7e12f0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_oceania_get_name', 0x7e1570, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_poland_get_name', 0x7e1950, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_portugal_get_name', 0x7e1cf0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_russia_get_name', 0x7e20b0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_scotland_cup_get_name', 0x7e23a0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_scotland_league_get_name', 0x7e2890, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_south_america_get_name', 0x7e2b90, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_spain_cup_get_name', 0x7e2ee0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os nomes dos órgãos disciplinares (Noruega a copa espanhola)',
        'the names of the ruling bodies (Norway to the Spanish cup)',
        [('rb_norway_league_get_name', 'RbNorwayLeague::get_name', '0x7e12f0', 'ruling_body/rb_norway_league.cpp', 'thiscall, ret 0xc',
          'escreve "Norwegian league" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xa7)',
          'writes "Norwegian league" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xa7)'),
         ('rb_oceania_get_name', 'RbOceania::get_name', '0x7e1570', 'ruling_body/rb_oceania.cpp', 'thiscall, ret 0xc',
          'escreve o nome da região 0xa (Oceania) no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0x9f)',
          "writes the name of region 0xa (Oceania) into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0x9f)"),
         ('rb_poland_get_name', 'RbPoland::get_name', '0x7e1950', 'ruling_body/rb_poland.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade polonês no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xc0)',
          "writes the Polish nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xc0)"),
         ('rb_portugal_get_name', 'RbPortugal::get_name', '0x7e1cf0', 'ruling_body/rb_portugal.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade português no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xbc)',
          "writes the Portuguese nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xbc)"),
         ('rb_russia_get_name', 'RbRussia::get_name', '0x7e20b0', 'ruling_body/rb_russia.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade russo no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xb7)',
          "writes the Russian nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xb7)"),
         ('rb_scotland_cup_get_name', 'RbScotlandCup::get_name', '0x7e23a0', 'ruling_body/rb_scotland_cup.cpp', 'thiscall, ret 0xc',
          'escreve "Scottish cup" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xb7)',
          'writes "Scottish cup" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xb7)'),
         ('rb_scotland_league_get_name', 'RbScotlandLeague::get_name', '0x7e2890', 'ruling_body/rb_scotland_league.cpp', 'thiscall, ret 0xc',
          'escreve "Scottish league" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0x135)',
          'writes "Scottish league" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0x135)'),
         ('rb_south_america_get_name', 'RbSouthAmerica::get_name', '0x7e2b90', 'ruling_body/rb_south_america.cpp', 'thiscall, ret 0xc',
          'escreve o nome da região 0xc (América do Sul) no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0x9f)',
          "writes the name of region 0xc (South America) into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0x9f)"),
         ('rb_spain_cup_get_name', 'RbSpainCup::get_name', '0x7e2ee0', 'ruling_body/rb_spain_cup.cpp', 'thiscall, ret 0xc',
          'escreve "Spanish cup" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xb7)',
          'writes "Spanish cup" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xb7)')])
CAMPOS = []
INTERESSA = []
