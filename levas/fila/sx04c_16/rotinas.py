# Os premios da Espanha: o objeto, os oito premios e o dia dos premios (award/spain_awards.cpp; leitor sx04c, rodada
# 5, 23/11/2026). Usa os auxiliares do sx05b_5 e do sx03_2: integrar depois deles.
# Rodam com a Espanha carregada: construtor e init na criacao do jogo, daily_update todo dia; os premios em 16 de
# junho (2002-06-16: ATE 2002-08-01).
# Convencoes conferidas na desmontagem:
# - 0x858ea0 thiscall(arquivo) ret 4: vtable 0x96fdd8, [KEY_NATION_SPAIN], +0x13 = 0x12, +0x12 = 1; sem notas do mes;
# - 0x858f30 thiscall(arquivo) ret 4: +8 = 8, malloc(0x20) (nulo: linha 0x66); tecnico e jogador do ano sem
#   competicao; por divisao a equipe da semana (0/0), o artilheiro (opcao 1) e o goleiro do ano;
# - 0x8592c0 thiscall(hoje) ret 4: Date::Date (0x10, 5, ano, -1); 0x768940 (First Division, premio 0); 0x768cf0 nos
#   premios 3, 6 (estatistica 2), 4, 7 (posicoes 1, estatistica 0x12) e 1 (0x11); 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2002-08-01'
ROTINAS = [('spain_awards_construct', 0x858ea0, [0x6a, 0xff, 0x68, 0xb8, 0x28, 0x96, 0x00]),
           ('spain_awards_init', 0x858f30, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('spain_awards_daily_update', 0x8592c0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Espanha',
        'the Spanish awards',
        [('spain_awards_construct', 'SpainAwards::SpainAwards', '0x858ea0', 'award/spain_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Espanha: a base com a tabela virtual da classe, a Espanha, o id 0x12 e o tipo 1; com arquivo, os salvos; senão os oito prêmios',
          'builds the Spanish awards: the base with the class vtable, Spain, id 0x12 and type 1; with a file, the saved ones; otherwise the eight awards'),
         ('spain_awards_init', 'SpainAwards::init_awards', '0x858f30', 'award/spain_awards.cpp', 'thiscall, ret 4',
          'cria os oito prêmios: técnico e jogador espanhol do ano e, em cada divisão, a equipe da semana, o artilheiro e o goleiro do ano',
          'creates the eight awards: the Spanish manager and player of the year and, in each division, the team of the week, the top goalscorer and the goalkeeper of the year'),
         ('spain_awards_daily_update', 'SpainAwards::daily_update', '0x8592c0', 'award/spain_awards.cpp', 'thiscall, ret 4',
          'em 16 de junho, o técnico do ano (pela Primeira Divisão), os artilheiros e os goleiros do ano das duas divisões e o jogador do ano pelas estatísticas; depois, todo dia, a atualização da base',
          'on 16 June, the manager of the year (from the First Division), the top goalscorers and goalkeepers of the year of both divisions and the player of the year from the stats; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
