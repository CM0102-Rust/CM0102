# Os premios da Dinamarca: o objeto, os dez premios e os dias dos premios (award/denmark_awards.cpp; leitor rx12,
# rodada 6, 25/11/2026). Usa os auxiliares dos premios das nacoes do sx05b_5 (motor.rs) e do sx03_2
# (england_awards.rs), ja publicados.
# Rodam com a Dinamarca carregada: o construtor e o init na criacao do jogo, o daily_update todo dia; o jogador do ano
# em 20 de dezembro de 2001 e os premios das divisoes em 6 de junho de 2002 (ATE 2002-08-01).
# Convencoes conferidas na desmontagem:
# - 0x554dc0 thiscall(arquivo) ret 4: 0x768290; vtable 0x96984c; +0xe = nacao de KEY_NATION_DENMARK; +0x13 = 6;
#   +0x12 = 1; com arquivo, 0x76a280; senao 0x554e50(arquivo); sem notas do mes;
# - 0x554e50 thiscall(arquivo) ret 4: +8 = 10 antes do malloc(0x28); nulo: caixa linha 0x67; argumentos por emulacao
#   (rx09/emul_awards.py);
# - 0x5552a0 thiscall(hoje) ret 4: Date::Date 0x549f70 (0x14, 0xb, ano, -1): 0x7683c0 (hoje, premio 0, [[0xad9c60]
#   +0xc], DEN, -1, -1, -1, -1, 0, -1) ret 0x28; Date::Date (6, 5, ano, -1): 0x768cf0 x6 (premios 2, 3, 5, 6, 8, 9);
#   0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('denmark_awards_construct', 0x554dc0, [0x6a, 0xff, 0x68, 0xb8, 0x7b, 0x95, 0x00]),
           ('denmark_awards_init', 0x554e50, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('denmark_awards_daily_update', 0x5552a0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Dinamarca',
        'the Danish awards',
        [('denmark_awards_construct', 'DenmarkAwards::DenmarkAwards', '0x554dc0', 'award/denmark_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Dinamarca: a base com a tabela virtual da classe, a Dinamarca como nação, o id 6 e o tipo 1; com arquivo, os prêmios salvos; senão os dez prêmios',
          'builds the Danish awards: the base with the class vtable, Denmark as the nation, id 6 and type 1; with a file, the saved awards; otherwise the ten awards'),
         ('denmark_awards_init', 'DenmarkAwards::init_awards', '0x554e50', 'award/denmark_awards.cpp', 'thiscall, ret 4',
          'cria os dez prêmios dinamarqueses: o jogador do ano; e, para cada uma das três divisões, a equipe da semana, o jogador e o jovem do ano',
          'creates the ten Danish awards: the player of the year; and, for each of the three divisions, the team of the week, the player and the young player of the year'),
         ('denmark_awards_daily_update', 'DenmarkAwards::daily_update', '0x5552a0', 'award/denmark_awards.cpp', 'thiscall, ret 4',
          'em 20 de dezembro, o jogador do ano pelas notas do ano dos dinamarqueses; em 6 de junho, pelas estatísticas da temporada de cada divisão, o jogador e o jovem (até 21) do ano; depois, todo dia, a atualização da base',
          "on 20 December, the player of the year from the Danes' year ratings; on 6 June, from each division's season statistics, the player and the young player (up to 21) of the year; then, every day, the base daily update")])
CAMPOS = []
INTERESSA = []
