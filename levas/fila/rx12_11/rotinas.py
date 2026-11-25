# Os premios da Grecia: o objeto, os dez premios e o dia dos premios (award/greece_awards.cpp; leitor rx12, rodada 6,
# 25/11/2026). Usa os auxiliares dos premios das nacoes do sx05b_5 (motor.rs; FN_NATION_AWARDS_PLAYER_OF_YEAR_FROM_
# RATINGS e pub(crate) la) e do sx03_2 (england_awards.rs), ja publicados.
# Rodam com a Grecia carregada: o construtor e o init na criacao do jogo, o daily_update todo dia; os premios em 30
# de maio de 2002.
# Convencoes conferidas na desmontagem:
# - 0x5ec850 thiscall(arquivo) ret 4: 0x768290; vtable 0x96b770; +0xe = nacao de KEY_NATION_GREECE; +0x13 = 0x16;
#   +0x12 = 1; com arquivo, 0x76a280; senao 0x5ec8e0(arquivo); sem notas do mes;
# - 0x5ec8e0 thiscall(arquivo) ret 4: +8 = 10 antes do malloc(0x28); nulo: caixa linha 0x68; argumentos por emulacao
#   (rx09/emul_awards.py);
# - 0x5ecd30 thiscall(hoje) ret 4: Date::Date 0x549f70 (0x1e, 4, ano, -1); 0x7683c0 (hoje, premio 0, [[0xad9c60]
#   +0x10] (as notas do ano do mundo, nao as +0xc das outras nacoes), GRE, -1, -1, -1, -1, 0, -1) ret 0x28; 0x769380
#   (hoje, premio 1, GRE, -1, GRE, -1, -1, 0, 0x11) ret 0x24; por divisao 0x768cf0 (0x11), 0x768940, 0x768cf0 (2);
#   0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('greece_awards_construct', 0x5ec850, [0x6a, 0xff, 0x68, 0xb8, 0xaa, 0x95, 0x00]),
           ('greece_awards_init', 0x5ec8e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('greece_awards_daily_update', 0x5ecd30, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Grécia',
        'the Greek awards',
        [('greece_awards_construct', 'GreeceAwards::GreeceAwards', '0x5ec850', 'award/greece_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Grécia: a base com a tabela virtual da classe, a Grécia como nação, o id 0x16 e o tipo 1; com arquivo, os prêmios salvos; senão os dez prêmios',
          'builds the Greek awards: the base with the class vtable, Greece as the nation, id 0x16 and type 1; with a file, the saved awards; otherwise the ten awards'),
         ('greece_awards_init', 'GreeceAwards::init_awards', '0x5ec8e0', 'award/greece_awards.cpp', 'thiscall, ret 4',
          'cria os dez prêmios gregos: o jogador grego e o estrangeiro do ano; e, para cada uma das duas divisões, a equipe da semana, o jogador, o técnico e o artilheiro do ano',
          'creates the ten Greek awards: the Greek and the foreign player of the year; and, for each of the two divisions, the team of the week, the player, the manager and the top goalscorer of the year'),
         ('greece_awards_daily_update', 'GreeceAwards::daily_update', '0x5ecd30', 'award/greece_awards.cpp', 'thiscall, ret 4',
          'em 30 de maio, o jogador grego do ano pelas notas do ano do mundo (as dos jogadores acima de 2000 de reputação), o estrangeiro do ano pelos três melhores das notas do ano, e, pelas estatísticas da temporada de cada divisão, o jogador, o técnico e o artilheiro do ano; depois, todo dia, a atualização da base',
          "on 30 May, the Greek player of the year from the world year ratings (the players above 2000 of reputation), the foreign player of the year from the year ratings' top three, and, from each division's season statistics, the player, the manager and the top goalscorer of the year; then, every day, the base daily update")])
CAMPOS = []
INTERESSA = []
