# Os premios do Japao: o objeto, os oito premios e o dia dos premios (award/japan_awards.cpp; leitor sx04c, rodada 5,
# 23/11/2026). Usa os auxiliares do sx05b_5 e do sx03_2 (como o sx04c_1): integrar depois deles.
# Rodam com o Japao carregado: construtor e init na criacao do jogo, daily_update todo dia; os premios em 20 de
# dezembro (2001-12-20).
# Convencoes conferidas na desmontagem:
# - 0x666690 thiscall(arquivo) ret 4: como o da Italia; vtable 0x96cce8, [KEY_NATION_JAPAN], +0x13 = 0xe, +0x12 = 1;
# - 0x666720 thiscall(arquivo) ret 4: +8 = 8, malloc(0x20) (nulo: linha 0x66); quatro YEAR_AWARD (tipo 0, sem
#   competicao: -1) e quatro TEAM_AWARD (anos 0; modo 1 nas equipes da semana); o 1o com o indice 0 literal;
# - 0x666ab0 thiscall(hoje) ret 4: Date::Date 0x549f70 (0x14, 0xb, ano, -1); tres 0x768cf0 da J1 (jogador 0x11,
#   artilheiro 2, jovem idade 0x15), 0x768940 (tecnico da J1), 0x7698f0 J1 (premio 5) e J2 (premio 7), 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('japan_awards_construct', 0x666690, [0x6a, 0xff, 0x68, 0x68, 0xc2, 0x95, 0x00]),
           ('japan_awards_init', 0x666720, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('japan_awards_daily_update', 0x666ab0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios do Japão',
        'the Japanese awards',
        [('japan_awards_construct', 'JapanAwards::JapanAwards', '0x666690', 'award/japan_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios do Japão: a base com a tabela virtual da classe, o Japão, o id 0xe e o tipo 1; com arquivo, os salvos; senão os oito prêmios',
          'builds the Japanese awards: the base with the class vtable, Japan, id 0xe and type 1; with a file, the saved ones; otherwise the eight awards'),
         ('japan_awards_init', 'JapanAwards::init_awards', '0x666720', 'award/japan_awards.cpp', 'thiscall, ret 4',
          'cria os oito prêmios japoneses: técnico, jogador, artilheiro e jovem do ano, e as equipes da semana e do ano da J1 e da J2',
          'creates the eight Japanese awards: manager, player, top goalscorer and young player of the year, and the teams of the week and of the year of the J1 and J2 Leagues'),
         ('japan_awards_daily_update', 'JapanAwards::daily_update', '0x666ab0', 'award/japan_awards.cpp', 'thiscall, ret 4',
          'em 20 de dezembro, pelas estatísticas da J1 o jogador, o artilheiro e o jovem (até 21) do ano, o técnico do ano da J1 e as equipes da temporada da J1 e da J2; depois, todo dia, a atualização da base',
          'on 20 December, from the J1 League stats the player, the top goalscorer and the young player (up to 21) of the year, the J1 manager of the year and the teams of the season of the J1 and J2 Leagues; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
