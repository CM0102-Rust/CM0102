# Os nomes dos orgaos disciplinares (liga espanhola a Gales, e o da base) (vtable +0x10, get_name; rb_*.cpp, ruling_body.cpp; leitor sx19, rodada 5,
# 24/11/2026). Item de TELAS: rodam so quando a tela pede (0 chamadas na carreira do mundo ate 2002-08-01): o texto do
# ambito da suspensao, pedido por RulingBody::get_ban_text (0x7e88d0, via Discipline::get_slot_text 0x55d3c0) na ficha
# do jogador (0x87618a). Usa o molde do sx18_24 (RulingBodyName, ruling_body_name, compare_ruling_body_name):
# integrar depois dele. Junto do sx19_15 (mesma leva).
# Caminho no jogo (para o Lucas provar): um jogo com a liga do orgao carregada; a ficha de um jogador suspenso ou com
# cartoes nessa liga (a linha da suspensao/disciplina mostra o nome do orgao).
# Convencoes conferidas na desmontagem: thiscall(buf, tamanho, forma) ret 0xc, o retorno nao e lido (0x7e8a4d);
# tamanho < 20 (com sinal): a caixa de erro da versao (arquivo e linha do .cpp do orgao, 0xb67a34 = 0) e nada mais;
# senao 0x66f4e0 langlib_sprintf(buf, texto), 0x6181d0 adjetivo de nacionalidade (nacao, buf, tamanho, forma) ou
# 0x616f20 nome da regiao (codigo, buf, tamanho, 4).
# A da base (0x7e8da0) so mostra a caixa de erro (linha 0x4a6, sem conferir o tamanho; todos os orgaos tem o seu
# nome): o comparador so roda a original, e ela fica sem mutacao (abriria a caixa).
SAVE = False
TELAS = True
ROTINAS = [('rb_spain_league_get_name', 0x7e3230, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_sweden_cup_get_name', 0x7e35b0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_sweden_league_get_name', 0x7e3920, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_turkey_cup_get_name', 0x7e3cc0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_turkey_league_get_name', 0x7e4010, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_usa_get_name', 0x7e42f0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rb_wales_get_name', 0x7e4650, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ruling_body_get_name', 0x7e8da0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os nomes dos órgãos disciplinares (liga espanhola a Gales, e o da base)',
        "the names of the ruling bodies (the Spanish league to Wales, and the base's)",
        [('rb_spain_league_get_name', 'RbSpainLeague::get_name', '0x7e3230', 'ruling_body/rb_spain_league.cpp', 'thiscall, ret 0xc',
          'escreve "Spanish league" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xb7)',
          'writes "Spanish league" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xb7)'),
         ('rb_sweden_cup_get_name', 'RbSwedenCup::get_name', '0x7e35b0', 'ruling_body/rb_sweden_cup.cpp', 'thiscall, ret 0xc',
          'escreve "Swedish cup" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xc5)',
          'writes "Swedish cup" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xc5)'),
         ('rb_sweden_league_get_name', 'RbSwedenLeague::get_name', '0x7e3920', 'ruling_body/rb_sweden_league.cpp', 'thiscall, ret 0xc',
          'escreve "Swedish league" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xbc)',
          'writes "Swedish league" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xbc)'),
         ('rb_turkey_cup_get_name', 'RbTurkeyCup::get_name', '0x7e3cc0', 'ruling_body/rb_turkey_cup.cpp', 'thiscall, ret 0xc',
          'escreve "Turkish cup" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xb7)',
          'writes "Turkish cup" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xb7)'),
         ('rb_turkey_league_get_name', 'RbTurkeyLeague::get_name', '0x7e4010', 'ruling_body/rb_turkey_league.cpp', 'thiscall, ret 0xc',
          'escreve "Turkish league" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xb7)',
          'writes "Turkish league" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xb7)'),
         ('rb_usa_get_name', 'RbUsa::get_name', '0x7e42f0', 'ruling_body/rb_usa.cpp', 'thiscall, ret 0xc',
          'escreve "United States" no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xcc)',
          'writes "United States" into the buffer as the body\'s name; a buffer under 20 bytes: the version error box (line 0xcc)'),
         ('rb_wales_get_name', 'RbWales::get_name', '0x7e4650', 'ruling_body/rb_wales.cpp', 'thiscall, ret 0xc',
          'escreve o adjetivo de nacionalidade galês no buffer como o nome do órgão; buffer de menos de 20 bytes: a caixa de erro da versão (linha 0xb9)',
          "writes the Welsh nationality adjective into the buffer as the body's name; a buffer under 20 bytes: the version error box (line 0xb9)"),
         ('ruling_body_get_name', 'RulingBody::get_name', '0x7e8da0', 'ruling_body/ruling_body.cpp', 'thiscall, ret 0xc',
          'a versão da base: cada órgão tem o seu nome, então a base só mostra a caixa de erro da versão (linha 0x4a6) e não escreve nada',
          'the base version: every body has its own name, so the base only shows the version error box (line 0x4a6) and writes nothing')])
CAMPOS = []
INTERESSA = []
