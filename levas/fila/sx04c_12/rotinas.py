# Os premios da Russia: o objeto, os sete premios e o dia dos premios (award/russia_awards.cpp; leitor sx04c, rodada
# 5, 23/11/2026). Usa os auxiliares do sx05b_5 e do sx03_2: integrar depois deles.
# Rodam com a Russia carregada: construtor e init na criacao do jogo, daily_update todo dia; os premios em 30 de
# novembro (2001-11-30).
# Convencoes conferidas na desmontagem:
# - 0x7ec210 thiscall(arquivo) ret 4: vtable 0x96f32c, [KEY_NATION_RUSSIA], +0x13 = 0x1b, +0x12 = 1; SEM notas do mes
#   (o on_month_change 0x7ec6c0 nunca roda: defeito);
# - 0x7ec2a0 thiscall(arquivo) ret 4: o ano local (o dword com o alto do quadro, em ebp depois do push ebx); +8 = 7,
#   malloc(0x1c) (nulo: linha 0x69); o jogador do ano (tipo 0, sem competicao), e por divisao duas equipes (anos 0,
#   modo 3) e o jogador do mes (abril (3) a novembro (0xa) do mesmo ano);
# - 0x7ec600 thiscall(hoje) ret 4: Date::Date (0x1e, 0xa, ano, -1); 0x769380 (mascara 0, modo 0x11, a Russia no 4o
#   filtro), 0x7698f0 Premier (premio 2) e First (premio 5); sem a virada dos premios do mes; 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ROTINAS = [('russia_awards_construct', 0x7ec210, [0x6a, 0xff, 0x68, 0xf8, 0x09, 0x96, 0x00]),
           ('russia_awards_init', 0x7ec2a0, [0x6a, 0xff, 0x68, 0x72, 0x0a, 0x96, 0x00]),
           ('russia_awards_daily_update', 0x7ec600, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Rússia',
        'the Russian awards',
        [('russia_awards_construct', 'RussiaAwards::RussiaAwards', '0x7ec210', 'award/russia_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Rússia: a base com a tabela virtual da classe, a Rússia, o id 0x1b e o tipo 1, sem notas do mês; com arquivo, os salvos; senão os sete prêmios',
          'builds the Russian awards: the base with the class vtable, Russia, id 0x1b and type 1, with no month ratings; with a file, the saved ones; otherwise the seven awards'),
         ('russia_awards_init', 'RussiaAwards::init_awards', '0x7ec2a0', 'award/russia_awards.cpp', 'thiscall, ret 4',
          'cria os sete prêmios: o jogador russo do ano e, em cada divisão, a equipe da semana, a do ano e o jogador do mês (de abril a novembro)',
          'creates the seven awards: the Russian player of the year and, in each division, the team of the week, the team of the year and the player of the month (April to November)'),
         ('russia_awards_daily_update', 'RussiaAwards::daily_update', '0x7ec600', 'award/russia_awards.cpp', 'thiscall, ret 4',
          'em 30 de novembro, o jogador russo do ano pelos três melhores das notas do ano e as equipes da temporada das duas divisões; depois, todo dia, a atualização da base',
          'on 30 November, the Russian player of the year from the year ratings top three and the teams of the season of both divisions; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
