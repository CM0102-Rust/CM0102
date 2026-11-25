# Os prêmios franceses: o objeto, os seus 21 prêmios e os dias dos prêmios (award/france_awards.cpp; leitor rx11,
# rodada 6, 25/11/2026). Rodam com a França carregada: construtor e init na criação do jogo, daily_update todo dia; os
# prêmios em 20 de dezembro (2001) e 6 de junho (2002): LIGAS com a França e ATE = '2002-08-01' (o molde do sx04c_16).
# Usa o molde comum do sx05b_5: integrar depois dele (INTEGRAR_ANTES.txt). O rx11_9 usa o
# nation_awards_construct_from_version daqui.
# - 0x5c0980 FranceAwards::FranceAwards thiscall(arquivo) ret 4: NationAwards 0x768290, vtable 0x96ae34,
#   [KEY_NATION_FRANCE], +0x13 = 9, +0x12 = 1; com arquivo e a versão dos dados [0xdd7110] (valor absoluto) >= 0x3e028,
#   NationAwards::load 0x76a280 (lido: fim); senão (sem notas do mês) init_awards 0x5c0a20(arquivo). Prólogo push -1;
#   push imm32 (7 bytes).
# - 0x5c0a20 FranceAwards::init_awards thiscall(arquivo) ret 4: +8 = 0x15, malloc(0x54) (nulo: caixa 0x74); os 21
#   prêmios (argumentos extraídos por script e conferidos à mão). Prólogo mov eax, fs:[0] (6 bytes).
# - 0x5c1260 FranceAwards::daily_update thiscall(hoje) ret 4: em 20/12 (Date 0x549f70(0x14, 0xb, ano, -1)) quatro
#   0x7683c0 com as notas do ano do AWARD_MANAGER (+0xc); em 6/6 0x768940, o sub-23 e por divisão 0x768cf0 x3 e
#   0x7698f0; por fim 0x768310. Prólogo sub esp, 0x10; push esi; push edi (5 bytes).
# Iguais no cm0102-1080.exe. Comparadores: construtor e init só a original com o plano do molde; daily_update só a
# original.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2002-08-01'
ROTINAS = [('france_awards_construct', 0x5c0980, [0x6a, 0xff, 0x68, 0x68, 0x9b, 0x95, 0x00]),
           ('france_awards_init', 0x5c0a20, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('france_awards_daily_update', 0x5c1260, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios franceses: o objeto, os seus 21 prêmios e os dias dos prêmios',
        'the French awards: the object, its 21 awards and the awards days',
        [('france_awards_construct', 'FranceAwards::FranceAwards', '0x5c0980', 'award/france_awards.cpp', 'thiscall, ret 4',
          'regra: os prêmios franceses são da França; de um jogo salvo na versão 0x3e028 ou depois são lidos, senão criados',
          'rule: the French awards object is built for France; from a saved game of version 0x3e028 or later it loads them, otherwise it creates its awards'),
         ('france_awards_init', 'FranceAwards::init_awards', '0x5c0a20', 'award/france_awards.cpp', 'thiscall, ret 4',
          'regra: a França tem 21 prêmios: o técnico do ano e cinco jogadores do ano (jogador, estrangeiro, sub-23, o mais popular e o mais impopular); depois, em cada divisão (Primeira, Segunda, National), a equipe da semana, a equipe do ano, o jogador dos jogadores, o jogador e o goleiro do ano',
          "rule: France has 21 awards: the manager of the year and five players of the year (player, foreign player, under 23, most popular and most unpopular); then for each division (First, Second, National) the team of the week, the team of the year, the players' player, the player and the goalkeeper of the year"),
         ('france_awards_daily_update', 'FranceAwards::daily_update', '0x5c1260', 'award/france_awards.cpp', 'thiscall, ret 4',
          'regra: em 20 de dezembro saem o jogador, o estrangeiro, o mais popular e o mais impopular do ano pelas notas do ano; em 6 de junho o técnico do ano e o sub-23 (da Primeira Divisão) e, em cada divisão, o jogador dos jogadores, o goleiro e o jogador do ano pelas estatísticas e a equipe da temporada; depois, todo dia, a atualização da base',
          "rule: on 20 December the player, foreign player, most popular and most unpopular player of the year come from the year ratings; on 6 June the manager of the year and the under 23 player (from the First Division) and, for each division, the players' player, the goalkeeper and the player of the year from the stats and the team of the season; then, every day, the base update")])
CAMPOS = []
INTERESSA = []
