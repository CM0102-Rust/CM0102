# Os premios da Irlanda do Norte: o objeto, os treze premios e o dia dos premios
# (award/northern_ireland_awards.cpp; leitor sx04c, rodada 5, 23/11/2026). Usa os auxiliares do sx05b_5, do sx03_2 e
# do sx04c_1 (award_player_of_year_from_world_ratings): integrar depois deles.
# Rodam com a Irlanda do Norte carregada: construtor e init na criacao do jogo, daily_update todo dia; os premios em
# 20 de maio (2002-05-20).
# Convencoes conferidas na desmontagem:
# - 0x792d80 thiscall(arquivo) ret 4: vtable 0x96df88, [KEY_NATION_NORTHERN_IRELAND], +0x13 = 0x19, +0x12 = 1; notas
#   do mes (linha 0x52);
# - 0x792ec0 thiscall(arquivo) ret 4: o ano local de [0xae2c92]; +8 = 0xd, malloc(0x34) (nulo: linha 0x7a); o jogador
#   do ano (sem competicao) e dois blocos de divisao (equipe, quatro do ano, tecnico do mes de agosto (7) do ano a
#   abril (3) do seguinte: `inc edx` sobre o dword do ano);
# - 0x793460 thiscall(hoje) ret 4: Date::Date (0x14, 4, ano, -1); 0x7683c0 (hoje, premio, [[0xad9c60]+0x10], 7
#   filtros) ret 0x28 com as notas +0x10 (nao as +0xc da Suecia); 0x768cf0 x3 e 0x768940 por divisao; o dia do fim
#   da temporada passa os premios 6 e 12; 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG IRL NIR'
ROTINAS = [('northern_ireland_awards_construct', 0x792d80, [0x6a, 0xff, 0x68, 0xd9, 0xe6, 0x95, 0x00]),
           ('northern_ireland_awards_init', 0x792ec0, [0x6a, 0xff, 0x68, 0xa6, 0xe7, 0x95, 0x00]),
           ('northern_ireland_awards_daily_update', 0x793460, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Irlanda do Norte',
        'the Northern Irish awards',
        [('northern_ireland_awards_construct', 'NorthernIrelandAwards::NorthernIrelandAwards', '0x792d80', 'award/northern_ireland_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Irlanda do Norte: a base com a tabela virtual da classe, a Irlanda do Norte, o id 0x19 e o tipo 1; com arquivo, os salvos; senão as notas do mês e os treze prêmios',
          'builds the Northern Irish awards: the base with the class vtable, Northern Ireland, id 0x19 and type 1; with a file, the saved ones; otherwise the month ratings and the thirteen awards'),
         ('northern_ireland_awards_init', 'NorthernIrelandAwards::init_awards', '0x792ec0', 'award/northern_ireland_awards.cpp', 'thiscall, ret 4',
          'cria os treze prêmios: o jogador norte-irlandês do ano e, em cada divisão, a equipe da semana, jogador, jovem, artilheiro e técnico do ano e o técnico do mês (de agosto a abril)',
          'creates the thirteen awards: the Northern Irish player of the year and, in each division, the team of the week, player, young player, top goalscorer and manager of the year and the manager of the month (August to April)'),
         ('northern_ireland_awards_daily_update', 'NorthernIrelandAwards::daily_update', '0x793460', 'award/northern_ireland_awards.cpp', 'thiscall, ret 4',
          'em 20 de maio, o jogador norte-irlandês do ano pelas notas mundiais e, em cada divisão, jogador, artilheiro e jovem (até 21) do ano pelas estatísticas e o técnico do ano; no dia do fim da temporada, os técnicos do mês passam à temporada seguinte; depois, todo dia, a atualização da base',
          'on 20 May, the Northern Irish player of the year from the world ratings and, in each division, player, top goalscorer and young player (up to 21) of the year from the stats and the manager of the year; on the season end day, the managers of the month move on; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
