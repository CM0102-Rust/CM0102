# Os premios da Australia: o objeto, os nove premios e o dia dos premios (award/australia_awards.cpp; leitor rx12,
# rodada 6, 25/11/2026). Usa os auxiliares dos premios das nacoes do sx05b_5 (motor.rs) e do sx03_2
# (england_awards.rs), ja publicados.
# Rodam com a Australia carregada (as 8 ligas): o construtor e o init na criacao do jogo, o daily_update todo dia; os
# premios em 20 de junho de 2002 (ATE 2002-08-01) e a virada dos premios do mes no fim da temporada da NSL.
# Convencoes conferidas na desmontagem:
# - 0x412540 thiscall(arquivo) ret 4: 0x768290; vtable 0x96781c; +0xe = nacao de KEY_NATION_AUSTRALIA; +0x13 = 0x18;
#   +0x12 = 1; com arquivo, 0x76a280; MONTH_RATINGS em operator new(0xf), nulo: caixa linha 0x4c; senao 0x412680;
# - 0x412680 thiscall(arquivo) ret 4: +8 = 9 antes do malloc(0x24); nulo: caixa linha 0x74; argumentos por emulacao
#   (rx09/emul_awards.py); os do mes (9, ano, 4, ano + 1);
# - 0x412ac0 thiscall(hoje) ret 4: Date::Date 0x549f70 (0x14, 5, ano, -1); 0x768cf0 x4 (premios 4 a 7), 0x768940
#   (premio 8); no fim de temporada 0x7672e0 (ano de hoje) nos premios 1, 2 e 3; 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2002-08-01'
ROTINAS = [('australia_awards_construct', 0x412540, [0x6a, 0xff, 0x68, 0x69, 0x3a, 0x95, 0x00]),
           ('australia_awards_init', 0x412680, [0x6a, 0xff, 0x68, 0xfe, 0x3a, 0x95, 0x00]),
           ('australia_awards_daily_update', 0x412ac0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Austrália',
        'the Australian awards',
        [('australia_awards_construct', 'AustraliaAwards::AustraliaAwards', '0x412540', 'award/australia_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Austrália: a base com a tabela virtual da classe, a Austrália como nação, o id 0x18 e o tipo 1; com arquivo, os prêmios salvos; senão as notas do mês e os nove prêmios',
          'builds the Australian awards: the base with the class vtable, Australia as the nation, id 0x18 and type 1; with a file, the saved awards; otherwise the month ratings and the nine awards'),
         ('australia_awards_init', 'AustraliaAwards::init_awards', '0x412680', 'award/australia_awards.cpp', 'thiscall, ret 4',
          'cria os nove prêmios australianos, todos da NSL: a equipe da semana; o técnico, o jogador e o jovem do mês (de outubro a maio); o jogador, o sub-21, o goleiro, o artilheiro e o técnico do ano',
          'creates the nine Australian awards, all of the NSL: the team of the week; the manager, the player and the young player of the month (October to May); the player, the under 21 player, the goalkeeper, the top goalscorer and the manager of the year'),
         ('australia_awards_daily_update', 'AustraliaAwards::daily_update', '0x412ac0', 'award/australia_awards.cpp', 'thiscall, ret 4',
          'em 20 de junho, os prêmios do ano da NSL pelas estatísticas da temporada: jogador, sub-21 (até 21), goleiro, artilheiro e técnico do ano; no fim da temporada da Austrália, os três prêmios do mês passam à temporada seguinte; depois, todo dia, a atualização da base',
          "on 20 June, the NSL year awards from the season's statistics: player, under 21 player (up to 21), goalkeeper, top goalscorer and manager of the year; on Australia's season end day the three month awards move on; then, every day, the base daily update")])
CAMPOS = []
INTERESSA = []
