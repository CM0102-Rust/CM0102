# Os premios da Polonia: o objeto, os dez premios e o dia dos premios (award/poland_awards.cpp; leitor sx04c, rodada
# 5, 23/11/2026). Usa os auxiliares do sx05b_5 e do sx03_2: integrar depois deles.
# Rodam com a Polonia carregada: construtor e init na criacao do jogo, daily_update todo dia; os premios em 30 de maio
# (2002-05-30).
# Convencoes conferidas na desmontagem:
# - 0x7cd130 thiscall(arquivo) ret 4: vtable 0x96e608, [KEY_NATION_POLAND], +0x13 = 0x15, +0x12 = 1; SEM notas do mes
#   (o construtor curto, como o da Italia): o on_month_change da Polonia (0x7cd790) nunca roda (defeito);
# - 0x7cd1c0 thiscall(arquivo) ret 4: o ano local; +8 = 0xa, malloc(0x28) (nulo: linha 0x6b); dois blocos de divisao
#   (equipe 0/0, jogador, jovem, tecnico do ano, tecnico do mes de julho (6) do ano a maio (4) do seguinte);
# - 0x7cd630 thiscall(hoje) ret 4: Date::Date (0x1e, 4, ano, -1); por divisao 0x768cf0 x2 (0x11; jovem 0x15) e
#   0x768940; no dia do fim da temporada 0x7672e0 nos premios 4 e 9; 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ROTINAS = [('poland_awards_construct', 0x7cd130, [0x6a, 0xff, 0x68, 0xe8, 0xf8, 0x95, 0x00]),
           ('poland_awards_init', 0x7cd1c0, [0x6a, 0xff, 0x68, 0x8c, 0xf9, 0x95, 0x00]),
           ('poland_awards_daily_update', 0x7cd630, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Polônia',
        'the Polish awards',
        [('poland_awards_construct', 'PolandAwards::PolandAwards', '0x7cd130', 'award/poland_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Polônia: a base com a tabela virtual da classe, a Polônia, o id 0x15 e o tipo 1, sem notas do mês; com arquivo, os salvos; senão os dez prêmios',
          'builds the Polish awards: the base with the class vtable, Poland, id 0x15 and type 1, with no month ratings; with a file, the saved ones; otherwise the ten awards'),
         ('poland_awards_init', 'PolandAwards::init_awards', '0x7cd1c0', 'award/poland_awards.cpp', 'thiscall, ret 4',
          'cria os dez prêmios: em cada divisão a equipe da semana, jogador, jovem e técnico do ano e o técnico do mês (de julho a maio)',
          'creates the ten awards: in each division the team of the week, player, young player and manager of the year and the manager of the month (July to May)'),
         ('poland_awards_daily_update', 'PolandAwards::daily_update', '0x7cd630', 'award/poland_awards.cpp', 'thiscall, ret 4',
          'em 30 de maio, em cada divisão o jogador e o jovem (até 21) do ano pelas estatísticas e o técnico do ano; no dia do fim da temporada, os técnicos do mês passam à temporada seguinte; depois, todo dia, a atualização da base',
          'on 30 May, in each division the player and the young player (up to 21) of the year from the stats and the manager of the year; on the season end day, the managers of the month move on; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
