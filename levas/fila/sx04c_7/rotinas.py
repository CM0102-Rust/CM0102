# Os premios da Noruega: o objeto, os onze premios e o dia dos premios (award/norway_awards.cpp; leitor sx04c, rodada
# 5, 23/11/2026). Usa os auxiliares do sx05b_5 (award_new, AwardSpec...), do sx03_2 e do sx04c_1
# (awards_file_from_version): integrar depois deles.
# Rodam com a Noruega carregada: construtor e init na criacao do jogo, daily_update todo dia; os premios em 20 de
# novembro (2001-11-20); os premios do mes viram no dia do fim da temporada da nacao.
# Convencoes conferidas na desmontagem (o init inteiro a mao: tem dois caminhos):
# - 0x793940 thiscall(arquivo) ret 4: vtable 0x96dfd0, [KEY_NATION_NORWAY], +0x13 = 0xf, +0x12 = 1; notas do mes
#   (linha 0x50);
# - 0x793a80 thiscall(arquivo) ret 4: o ano local; +8 = 0xb, malloc(0x2c) (nulo: linha 0x7a); a variavel do arquivo
#   dos premios do mes zerada se |[0xdd7110]| < 0x3e007; cinco YEAR_AWARD (tipo 0, sem competicao, arquivo cru) e a
#   equipe da Premier (anos 0, modo 1, arquivo cru); depois, se |versao| < 0x3e009 e o arquivo dos premios do mes nao
#   e nulo, dois YEAR_AWARD de enchimento (chave 0, reputacoes 0) lidos do arquivo e apagados na hora pela virtual 0
#   (bandeiras 1) e os dois premios do mes da divisao sem arquivo; senao os dois premios do mes com esse arquivo; a
#   equipe da First Division com o arquivo cru; o mesmo para a First Division (a condicao relida); os indices vem de
#   esi e andam tambem nos enchimentos (8, 9, 10, 13, 14 no save antigo); a 0x794220 do mapa e um pedaco desta;
# - 0x7942c0 thiscall(hoje) ret 4: Date::Date (0x14, 0xa, ano, -1); cinco 0x769380 (mascaras 0, 1, 6, 0x18, 0x40,
#   modo 0x11, a Noruega no 4o filtro); no dia do fim da temporada, 0x7672e0 com o ano + 1 (inc cx) nos premios 6, 7,
#   9 e 10; 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido (o init sem a conferencia dos indices no save
# antigo); daily_update so a original.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ROTINAS = [('norway_awards_construct', 0x793940, [0x6a, 0xff, 0x68, 0xe9, 0xe7, 0x95, 0x00]),
           ('norway_awards_init', 0x793a80, [0x6a, 0xff, 0x68, 0x0a, 0xe9, 0x95, 0x00]),
           ('norway_awards_daily_update', 0x7942c0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Noruega',
        'the Norwegian awards',
        [('norway_awards_construct', 'NorwayAwards::NorwayAwards', '0x793940', 'award/norway_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Noruega: a base com a tabela virtual da classe, a Noruega, o id 0xf e o tipo 1; com arquivo, os salvos; senão as notas do mês e os onze prêmios',
          'builds the Norwegian awards: the base with the class vtable, Norway, id 0xf and type 1; with a file, the saved ones; otherwise the month ratings and the eleven awards'),
         ('norway_awards_init', 'NorwayAwards::init_awards', '0x793a80', 'award/norway_awards.cpp', 'thiscall, ret 4',
          'cria os onze prêmios: jogador, goleiro, defensor, meio-campista e atacante do ano, e em cada divisão a equipe da semana e os técnico e jogador do mês (de abril a outubro); os prêmios do mês só vêm do arquivo se o save for da versão 0x3e007 em diante, e um save anterior a 0x3e009 traz dois prêmios de enchimento antes de cada par, lidos e descartados',
          'creates the eleven awards: player, goalkeeper, defender, midfielder and attacker of the year, and in each division the team of the week and the manager and player of the month (April to October); the month awards come from the file only when the save is version 0x3e007 or later, and a save before 0x3e009 holds two placeholder awards before each pair, read and discarded'),
         ('norway_awards_daily_update', 'NorwayAwards::daily_update', '0x7942c0', 'award/norway_awards.cpp', 'thiscall, ret 4',
          'em 20 de novembro, jogador, goleiro, defensor, meio-campista e atacante do ano pelos três melhores das notas do ano por posição; no dia do fim da temporada, os quatro prêmios do mês passam ao ano seguinte; depois, todo dia, a atualização da base',
          'on 20 November, player, goalkeeper, defender, midfielder and attacker of the year from the year ratings top three by position; on the season end day, the four month awards move on to the next year; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
