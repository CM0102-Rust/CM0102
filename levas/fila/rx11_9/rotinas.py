# Os prêmios irlandeses: o objeto, os seus dezoito prêmios e o dia dos prêmios (award/ireland_awards.cpp; leitor rx11,
# rodada 6, 25/11/2026). Rodam com a Irlanda carregada: construtor e init na criação do jogo, daily_update todo dia; os
# prêmios em 1o de maio (2002-05-01): LIGAS = 'ENG IRL NIR', como o sx04c_5. Usa o molde comum do sx05b_5, o
# award_player_of_year_from_world_ratings do sx04c_1 e o nation_awards_construct_from_version do rx11_7: integrar
# depois deles (INTEGRAR_ANTES.txt).
# - 0x63b640 IrelandAwards::IrelandAwards thiscall(arquivo) ret 4: NationAwards 0x768290, vtable 0x96c17c,
#   [KEY_NATION_REPUBLIC_OF_IRELAND], +0x13 = 0xc, +0x12 = 1; com arquivo e a versão [0xdd7110] (absoluta) >= 0x3e028,
#   NationAwards::load 0x76a280 (lido: fim); senão MONTH_RATINGS 0x767af0(0) (nulo: caixa 0x57 e fim) e init_awards
#   0x63b790(arquivo). Prólogo push -1; push imm32 (7 bytes).
# - 0x63b790 IrelandAwards::init_awards thiscall(arquivo) ret 4: +8 = 0x12, malloc(0x48) (nulo: caixa 0x7f); os 18
#   prêmios (argumentos extraídos por script e conferidos à mão; os do mês de setembro (8) a abril (3)). Prólogo push
#   -1; push imm32 (7 bytes).
# - 0x63bf10 IrelandAwards::daily_update thiscall(hoje) ret 4: em 1/5 (Date(1, 4, ano, -1)) três 0x7683c0 com as notas
#   mundiais do AWARD_MANAGER (+0x10), por divisão 0x768cf0 x3, 0x7698f0 e 0x768940; no fim da temporada da nação
#   (0x669340 e a tabela 0xb63d75) 0x7672e0 nos prêmios 3, 10 e 17; por fim 0x768310. Prólogo sub esp, 0x10; push
#   ebx; push esi (5 bytes).
# Iguais no cm0102-1080.exe. Comparadores: construtor e init só a original com o plano do molde; daily_update só a
# original.
SAVE = False
LIGAS = 'ENG IRL NIR'
ROTINAS = [('ireland_awards_construct', 0x63b640, [0x6a, 0xff, 0x68, 0x79, 0xb6, 0x95, 0x00]),
           ('ireland_awards_init', 0x63b790, [0x6a, 0xff, 0x68, 0x8c, 0xb7, 0x95, 0x00]),
           ('ireland_awards_daily_update', 0x63bf10, [0x83, 0xec, 0x10, 0x53, 0x56])]
DOCS = ('os prêmios irlandeses: o objeto, os seus dezoito prêmios e o dia dos prêmios',
        'the Irish awards: the object, its eighteen awards and the awards day',
        [('ireland_awards_construct', 'IrelandAwards::IrelandAwards', '0x63b640', 'award/ireland_awards.cpp', 'thiscall, ret 4',
          'regra: os prêmios irlandeses são da República da Irlanda, com as notas do mês; de um jogo salvo na versão 0x3e028 ou depois são lidos, senão criados',
          'rule: the Irish awards object is built for the Republic of Ireland with month ratings; from a saved game of version 0x3e028 or later it loads them, otherwise it creates its awards'),
         ('ireland_awards_init', 'IrelandAwards::init_awards', '0x63b790', 'award/ireland_awards.cpp', 'thiscall, ret 4',
          'regra: a Irlanda tem dezoito prêmios: o jogador, o jovem e a personalidade do ano e o jogador do mês (de setembro a abril); depois, em cada divisão, a equipe da semana, o jogador, o jovem, a chuteira (de ouro na Premier, de bronze na Primeira), a equipe do ano, o técnico do ano e o técnico do mês',
          'rule: Ireland has eighteen awards: the player, young player and personality of the year and the player of the month (September to April); then for each division the team of the week, the player, the young player, the boot (golden for the Premier, bronze for the First), the team of the year, the manager of the year and the manager of the month'),
         ('ireland_awards_daily_update', 'IrelandAwards::daily_update', '0x63bf10', 'award/ireland_awards.cpp', 'thiscall, ret 4',
          'regra: em 1o de maio saem o jogador, o jovem (até 23) e a personalidade do ano pelas notas mundiais e, em cada divisão, o jogador, o jovem (até 23) e a chuteira pelas estatísticas, a equipe da temporada e o técnico do ano; no fim da temporada da nação os três prêmios do mês passam à temporada seguinte; depois, todo dia, a atualização da base',
          "rule: on 1 May the player, young player (up to 23) and personality of the year come from the world ratings and, in each division, the player, the young player (up to 23) and the boot from the stats, the team of the season and the manager of the year; on the nation's season end day the three month awards move on; then, every day, the base update")])
CAMPOS = []
INTERESSA = []
