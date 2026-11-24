# Os premios de Portugal: o objeto, os doze premios e o dia dos premios (award/portugal_awards.cpp; leitor sx04c,
# rodada 5, 23/11/2026). Usa os auxiliares do sx05b_5 e do sx03_2: integrar depois deles.
# Rodam com Portugal carregado: construtor e init na criacao do jogo, daily_update todo dia; os premios em 6 de junho
# (2002-06-06: ATE 2002-08-01).
# Convencoes conferidas na desmontagem:
# - 0x7d3630 thiscall(arquivo) ret 4: vtable 0x96e9ac, [KEY_NATION_PORTUGAL], +0x13 = 0x10, +0x12 = 1; sem notas do mes;
# - 0x7d36c0 thiscall(arquivo) ret 4: +8 = 0xc, malloc(0x30) (nulo: linha 0x68); dois blocos (equipe da semana 0/0,
#   jogador, mais vezes melhor em campo, artilheiro (opcao 1), equipe do ano, tecnico do ano);
# - 0x7d3be0 thiscall(hoje) ret 4: Date::Date (6, 5, ano, -1); por divisao 0x768cf0 x3 (0xc, 2, 0x11), 0x7698f0 e
#   0x768940; 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('portugal_awards_construct', 0x7d3630, [0x6a, 0xff, 0x68, 0x78, 0xfb, 0x95, 0x00]),
           ('portugal_awards_init', 0x7d36c0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('portugal_awards_daily_update', 0x7d3be0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios de Portugal',
        'the Portuguese awards',
        [('portugal_awards_construct', 'PortugalAwards::PortugalAwards', '0x7d3630', 'award/portugal_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios de Portugal: a base com a tabela virtual da classe, Portugal, o id 0x10 e o tipo 1; com arquivo, os salvos; senão os doze prêmios',
          'builds the Portuguese awards: the base with the class vtable, Portugal, id 0x10 and type 1; with a file, the saved ones; otherwise the twelve awards'),
         ('portugal_awards_init', 'PortugalAwards::init_awards', '0x7d36c0', 'award/portugal_awards.cpp', 'thiscall, ret 4',
          'cria os doze prêmios: em cada divisão a equipe da semana, jogador, o de mais prêmios de melhor em campo, artilheiro, a equipe do ano e o técnico do ano',
          'creates the twelve awards: in each division the team of the week, player, most man of the match awards, top goalscorer, team of the year and manager of the year'),
         ('portugal_awards_daily_update', 'PortugalAwards::daily_update', '0x7d3be0', 'award/portugal_awards.cpp', 'thiscall, ret 4',
          'em 6 de junho, em cada divisão o de mais prêmios de melhor em campo, o artilheiro e o jogador do ano pelas estatísticas, a equipe da temporada e o técnico do ano; depois, todo dia, a atualização da base',
          'on 6 June, in each division the most man of the match awards, the top goalscorer and the player of the year from the stats, the team of the season and the manager of the year; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
