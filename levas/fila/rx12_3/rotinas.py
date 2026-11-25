# Os premios da Alemanha: o objeto, os catorze premios e o dia dos premios (award/germany_awards.cpp; leitor rx12,
# rodada 6, 25/11/2026). Usa os auxiliares dos premios das nacoes do sx05b_5 (motor.rs) e do sx03_2
# (england_awards.rs), ja publicados.
# Rodam com a Alemanha carregada: o construtor e o init na criacao do jogo, o daily_update todo dia (virtual); os
# premios em 6 de junho de 2002 (por isso ATE 2002-08-01) e a virada dos premios do mes no fim da temporada alema.
# Convencoes conferidas na desmontagem:
# - 0x5df650 thiscall(arquivo) ret 4: 0x768290; vtable 0x96b3b8; +0xe = nacao de KEY_NATION_GERMANY; +0x13 = 0xa;
#   +0x12 = 1; com arquivo, 0x76a280; MONTH_RATINGS 0x767af0(0) em operator new(0xf), nulo: caixa linha 0x50; senao
#   0x5df790(arquivo);
# - 0x5df790 thiscall(arquivo) ret 4: +8 = 14 antes do malloc(0x38); nulo: caixa linha 0x78; argumentos por emulacao
#   (rx09/emul_awards.py), conferidos a mao nos do mes: (7, ano, 4, ano + 1), o ano + 1 somado na dword da pilha;
# - 0x5dfd70 thiscall(hoje) ret 4: Date::Date 0x549f70 (6, 5, ano, -1); 0x769380 x5 (mascaras 1, 2, 4, 0x10, 0x40),
#   0x768cf0 x2 (premios 9 e 13), 0x769380 (premio 5, mascara -1), 0x768940 x2 (premios 8 e 12); no fim de temporada
#   0x7672e0 (ano de hoje) nos premios 7 e 11; 0x768310.
# Comparadores: construtor e init so a original e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('germany_awards_construct', 0x5df650, [0x6a, 0xff, 0x68, 0xe9, 0xa6, 0x95, 0x00]),
           ('germany_awards_init', 0x5df790, [0x6a, 0xff, 0x68, 0xc4, 0xa7, 0x95, 0x00]),
           ('germany_awards_daily_update', 0x5dfd70, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Alemanha',
        'the German awards',
        [('germany_awards_construct', 'GermanyAwards::GermanyAwards', '0x5df650', 'award/germany_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Alemanha: a base com a tabela virtual da classe, a Alemanha como nação, o id 0xa e o tipo 1; com arquivo, os prêmios salvos; senão as notas do mês e os catorze prêmios',
          'builds the German awards: the base with the class vtable, Germany as the nation, id 0xa and type 1; with a file, the saved awards; otherwise the month ratings and the fourteen awards'),
         ('germany_awards_init', 'GermanyAwards::init_awards', '0x5df790', 'award/germany_awards.cpp', 'thiscall, ret 4',
          'cria os catorze prêmios alemães: goleiro, líbero, defensor, meio-campista, atacante e jogador do ano; e, para a primeira e a segunda divisão, a equipe da semana, o jogador do mês (de agosto a maio), o técnico do ano e o artilheiro',
          'creates the fourteen German awards: goalkeeper, sweeper, defender, midfielder, striker and player of the year; and, for the first and second divisions, the team of the week, the player of the month (August to May), the manager of the year and the top goalscorer'),
         ('germany_awards_daily_update', 'GermanyAwards::daily_update', '0x5dfd70', 'award/germany_awards.cpp', 'thiscall, ret 4',
          'em 6 de junho, os prêmios do ano alemães: goleiro, líbero, defensor, meio-campista e atacante pelos três melhores das notas do ano por posição, os artilheiros das duas divisões, o jogador do ano e os técnicos do ano das duas divisões; no fim da temporada alemã, os dois prêmios do mês passam à temporada seguinte; depois, todo dia, a atualização da base',
          "on 6 June, the German year awards: goalkeeper, sweeper, defender, midfielder and striker from the year ratings' top three by position, both divisions' top goalscorers, the player of the year and both divisions' managers of the year; on Germany's season end day the two month awards move on; then, every day, the base daily update")])
CAMPOS = []
INTERESSA = []
