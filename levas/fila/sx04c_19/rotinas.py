# Os premios antigos da Franca: o objeto, os catorze premios e os dias dos premios (award/old_france_awards.cpp;
# leitor sx04c, rodada 5, 23/11/2026). Usa os auxiliares do sx05b_5 (award_player_of_year_from_ratings, com as notas
# +0xc) e do sx03_2: integrar depois deles.
# NAO RODAM com a base do jogo (o AWARD_MANAGER 0x413965 so monta a classe antiga com a chave nova em -1); 0 chamadas
# na cobertura do mundo. Simulacao, sem LIGAS e sem mutacoes.
# Convencoes conferidas na desmontagem:
# - 0x79dcf0 thiscall(arquivo) ret 4: vtable 0x96e194, [KEY_NATION_FRANCE], +0x13 = 9, +0x12 = 1; sem notas do mes;
# - 0x79dd80 thiscall(arquivo) ret 4: +8 = 0xe, malloc(0x38) (nulo: linha 0x6b); seis premios do ano sem competicao
#   (tipo 1 no tecnico e no sub-23, 0 nos outros) e dois blocos de divisao (equipe da semana 0/0 e tres do ano, tipo 1);
# - 0x79e330 thiscall(hoje) ret 4: Date::Date (0x14, 0xb, ano, -1): quatro 0x7683c0 com as notas [[0xad9c60]+0xc]
#   (lidas uma vez em ebx); Date::Date (6, 5, ano, -1): 0x768940 (First, premio 0) e sete 0x768cf0; 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
ROTINAS = [('old_france_awards_construct', 0x79dcf0, [0x6a, 0xff, 0x68, 0x18, 0xed, 0x95, 0x00]),
           ('old_france_awards_init', 0x79dd80, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('old_france_awards_daily_update', 0x79e330, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios antigos da França',
        'the old French awards',
        [('old_france_awards_construct', 'OldFranceAwards::OldFranceAwards', '0x79dcf0', 'award/old_france_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios antigos da França (só com uma base de dados sem as chaves dos prêmios novos): a base com a tabela virtual da classe, a França, o id 9 e o tipo 1; com arquivo, os salvos; senão os catorze prêmios',
          'builds the old French awards (only with a database lacking the new award keys): the base with the class vtable, France, id 9 and type 1; with a file, the saved ones; otherwise the fourteen awards'),
         ('old_france_awards_init', 'OldFranceAwards::init_awards', '0x79dd80', 'award/old_france_awards.cpp', 'thiscall, ret 4',
          'cria os catorze prêmios: técnico, jogador, estrangeiro, sub-23, mais popular e mais impopular do ano e, em cada divisão, a equipe da semana, o jogador escolhido pelos jogadores, o jogador e o goleiro do ano',
          'creates the fourteen awards: manager, player, foreign player, under-23, most popular and most unpopular player of the year and, in each division, the team of the week, the players\' player, the player and the goalkeeper of the year'),
         ('old_france_awards_daily_update', 'OldFranceAwards::daily_update', '0x79e330', 'award/old_france_awards.cpp', 'thiscall, ret 4',
          'em 20 de dezembro, jogador, estrangeiro, mais popular e mais impopular do ano pelas notas do ano; em 6 de junho, o técnico do ano e os prêmios das divisões pelas estatísticas; depois, todo dia, a atualização da base',
          'on 20 December, player, foreign player, most popular and most unpopular player of the year from the year ratings; on 6 June, the manager of the year and the division awards from the stats; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
