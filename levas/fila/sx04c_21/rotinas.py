# Os premios antigos da Irlanda: o objeto, os dezessete premios e o dia dos premios (award/old_ireland_awards.cpp;
# leitor sx04c, rodada 5, 23/11/2026). Usa os auxiliares do sx05b_5, do sx03_2 e do sx04c_1
# (award_player_of_year_from_world_ratings): integrar depois deles.
# NAO RODAM com a base do jogo (o AWARD_MANAGER 0x413a90 so monta a classe antiga com a chave nova em -1); 0 chamadas
# na cobertura do mundo. Simulacao, sem LIGAS e sem mutacoes.
# Convencoes conferidas na desmontagem:
# - 0x79ea00 thiscall(arquivo) ret 4: vtable 0x96e1bc, [KEY_NATION_REPUBLIC_OF_IRELAND], +0x13 = 0xc, +0x12 = 1; notas
#   do mes (linha 0x56);
# - 0x79eb40 thiscall(arquivo) ret 4: o ano local; +8 = 0x11, malloc(0x44) (nulo: linha 0x7e); tres premios do ano sem
#   competicao e dois blocos de sete (equipe da semana, jogador, jovem, chuteira, equipe do ano, tecnico do ano,
#   tecnico do mes de setembro (8) do ano a abril (3) do seguinte);
# - 0x79f250 thiscall(hoje) ret 4: Date::Date (1, 4, ano, -1); tres 0x7683c0 com as notas +0x10; por divisao 0x768cf0
#   x3, 0x7698f0 e 0x768940, este SEMPRE com [0x9cf80c] (a Premier Division; defeito na First Division, premio 15); no
#   dia do fim da temporada 0x7672e0 nos premios 9 e 16; 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
ROTINAS = [('old_ireland_awards_construct', 0x79ea00, [0x6a, 0xff, 0x68, 0x89, 0xee, 0x95, 0x00]),
           ('old_ireland_awards_init', 0x79eb40, [0x6a, 0xff, 0x68, 0x8e, 0xef, 0x95, 0x00]),
           ('old_ireland_awards_daily_update', 0x79f250, [0x83, 0xec, 0x10, 0x53, 0x56])]
DOCS = ('os prêmios antigos da Irlanda',
        'the old Irish awards',
        [('old_ireland_awards_construct', 'OldIrelandAwards::OldIrelandAwards', '0x79ea00', 'award/old_ireland_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios antigos da Irlanda (só com uma base de dados sem as chaves dos prêmios novos): a base com a tabela virtual da classe, a Irlanda, o id 0xc e o tipo 1; com arquivo, os salvos; senão as notas do mês e os dezessete prêmios',
          'builds the old Irish awards (only with a database lacking the new award keys): the base with the class vtable, Ireland, id 0xc and type 1; with a file, the saved ones; otherwise the month ratings and the seventeen awards'),
         ('old_ireland_awards_init', 'OldIrelandAwards::init_awards', '0x79eb40', 'award/old_ireland_awards.cpp', 'thiscall, ret 4',
          'cria os dezessete prêmios: jogador, jovem e personalidade irlandeses do ano e, em cada divisão, a equipe da semana, jogador e jovem do ano, a chuteira, a equipe do ano, o técnico do ano e o técnico do mês (de setembro a abril)',
          'creates the seventeen awards: the Irish player, young player and personality of the year and, in each division, the team of the week, player and young player of the year, the boot, the team of the year, the manager of the year and the manager of the month (September to April)'),
         ('old_ireland_awards_daily_update', 'OldIrelandAwards::daily_update', '0x79f250', 'award/old_ireland_awards.cpp', 'thiscall, ret 4',
          'em 1º de maio, jogador, jovem e personalidade irlandeses do ano pelas notas mundiais e, em cada divisão, jogador, jovem (até 23) e artilheiro do ano pelas estatísticas, a equipe da temporada e o técnico do ano (os dois pela Premier Division: defeito do original); no dia do fim da temporada, os técnicos do mês passam à seguinte; depois, todo dia, a atualização da base',
          'on 1 May, the Irish player, young player and personality of the year from the world ratings and, in each division, player, young player (up to 23) and top goalscorer of the year from the stats, the team of the season and the manager of the year (both from the Premier Division: defect of the original); on the season end day, the managers of the month move on; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
