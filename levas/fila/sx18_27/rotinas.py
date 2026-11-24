# As últimas perguntas "a um amarelo da suspensão" dos órgãos (leitor sx18, rodada 5, 24/11/2026). Item de TELAS: rodam so quando a tela pede.
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
ROTINAS = [('rb_japan_cup_is_one_booking_from_ban', 0x7e0450, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_korea_is_one_booking_from_ban', 0x7e0aa0, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0]),
           ('rb_northern_ireland_is_one_booking_from_ban', 0x7e0e20, [0x8b, 0x44, 0x24, 0x0c, 0x85, 0xc0]),
           ('rb_norway_cup_is_one_booking_from_ban', 0x7e1120, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0])]
DOCS = ('as últimas perguntas "a um amarelo da suspensão" dos órgãos',
        'the last "one booking from a ban" questions of the ruling bodies',
        [('rb_japan_cup_is_one_booking_from_ban', 'RbJapanCup::is_one_booking_from_ban', '0x7e0450', 'ruling_body/rb_japan_cup.cpp', 'thiscall, ret 0x10',
          'diz se o próximo amarelo suspende o jogador pelas regras do órgão: com um número ímpar de amarelos; as saídas, quando dadas, recebem 1 jogo e nenhum dia',
          "tells whether the next booking bans the player under the body's rules: at an odd count of bookings; the outputs, when given, get one match and no days"),
         ('rb_korea_is_one_booking_from_ban', 'RbKorea::is_one_booking_from_ban', '0x7e0aa0', 'ruling_body/rb_korea.cpp', 'thiscall, ret 0x10',
          'diz se o próximo amarelo suspende o jogador pelas regras do órgão: com 2, 5, 8... amarelos (de 3 em 3, que não é a regra coreana: defeito do original); as saídas, quando dadas, recebem 1 jogo e nenhum dia',
          "tells whether the next booking bans the player under the body's rules: at 2, 5, 8... bookings (every three, which is not the Korean rule: a defect of the original); the outputs, when given, get one match and no days"),
         ('rb_northern_ireland_is_one_booking_from_ban', 'RbNorthernIreland::is_one_booking_from_ban', '0x7e0e20', 'ruling_body/rb_northern_ireland.cpp', 'thiscall, ret 0x10',
          'diz se o próximo amarelo suspende o jogador pelas regras do órgão: quando os pontos estão a menos de 4 de 20, 32, 42 ou 51 e devolve a suspensão de 1, 2, 4 ou 6 jogos; as saídas, quando dadas, recebem 1 jogo e nenhum dia',
          "tells whether the next booking bans the player under the body's rules: when the points are within 4 of 20, 32, 42 or 51, giving the ban of 1, 2, 4 or 6 matches; the outputs, when given, get one match and no days"),
         ('rb_norway_cup_is_one_booking_from_ban', 'RbNorwayCup::is_one_booking_from_ban', '0x7e1120', 'ruling_body/rb_norway_cup.cpp', 'thiscall, ret 0x10',
          'diz se o próximo amarelo suspende o jogador pelas regras do órgão: com 1 amarelo; as saídas, quando dadas, recebem 1 jogo e nenhum dia',
          "tells whether the next booking bans the player under the body's rules: at 1 booking; the outputs, when given, get one match and no days")])
CAMPOS = []
INTERESSA = []
