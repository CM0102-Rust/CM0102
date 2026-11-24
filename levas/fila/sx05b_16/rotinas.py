# Os premios dos EUA: o objeto, os sete premios da MLS e o dia dos premios (award/usa_awards.cpp; leitor sx05b, rodada
# 5, 23/11/2026). Usa os construtores e chamadas comuns dos premios das nacoes (sx05b_5) e nation_award, awards_day_is,
# award_manager_of_year, award_player_of_year, award_team_of_season e month_award_start_new_season (sx03_2): integrar
# depois dos dois.
# Rodam com os EUA carregados: o construtor e o init na criacao do jogo (AWARD_MANAGER 0x413cf9), o daily_update todo
# dia; os premios em 25 de novembro (2001-11-25, dentro da janela). LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'.
# Convencoes conferidas na desmontagem (as listas de argumentos dos construtores tiradas por script e conferidas a mao
# no premio do mes e no primeiro do ano):
# - 0x906980 thiscall(arquivo) ret 4: como o sueco, vtable 0x970830, [KEY_NATION_UNITED_STATES], +0x13 = 0x14, +0x12 =
#   1; sem carregar, MONTH_RATINGS por operator new 0x944cf1(0xf) e 0x767af0(0) em +0xa (nulo: show_error_at linha
#   0x48 e sem premios); depois 0x906ac0(arquivo);
# - 0x906ac0 thiscall(arquivo) ret 4: o ano local de [0xae2c92] (word; o original empilha o dword com a palavra alta do
#   que estava na pilha: o MONTH_AWARD 0x766e10 so le a palavra); +8 = 7, malloc(0x1c) (nulo: linha 0x70); o premio do
#   mes (0xa2 bytes, 12 argumentos: de marco (3) a outubro (0xa) do ano), cinco do ano e a equipe;
# - 0x906e10 thiscall(hoje) ret 4: Date::Date 0x549f70 (0x19, 0xa, ano, -1); 0x768940, quatro 0x768cf0 (goleiro
#   posicoes 1, defensor 6, estreante idade 0x15, artilheiro estatistica 2), 0x7698f0; o dia do fim da temporada
#   (key_nation_index_of_nation 0x669340, o word +0x15 do registro de 0x48 bytes em 0xb63d60, dia com sinal contra
#   word sem sinal) passa o premio do mes a temporada seguinte (0x7672e0(ano)); 0x768310.
# Comparadores: construtor e init so a original (alocam) e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('usa_awards_construct', 0x906980, [0x6a, 0xff, 0x68, 0xd9, 0x52, 0x96, 0x00]),
           ('usa_awards_init', 0x906ac0, [0x6a, 0xff, 0x68, 0x52, 0x53, 0x96, 0x00]),
           ('usa_awards_daily_update', 0x906e10, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios dos EUA',
        'the USA awards',
        [('usa_awards_construct', 'UsaAwards::UsaAwards', '0x906980', 'award/usa_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios dos EUA: a base com a tabela virtual da classe, os EUA, o id 0x14 e o tipo 1; com arquivo, os salvos; senão as notas do mês (sem memória, a caixa de erro e nenhum prêmio) e os sete prêmios',
          'builds the USA awards: the base with the class vtable, the United States, id 0x14 and type 1; with a file, the saved ones; otherwise the month ratings (out of memory, the error box and no awards) and the seven awards'),
         ('usa_awards_init', 'UsaAwards::init_awards', '0x906ac0', 'award/usa_awards.cpp', 'thiscall, ret 4',
          'cria os sete prêmios da MLS: jogador do mês (de março a outubro do ano corrente), técnico, artilheiro, goleiro, defensor e estreante do ano, e a seleção do campeonato',
          'creates the seven MLS awards: player of the month (March to October of the current year), manager, scoring champion, goalkeeper, defender and rookie of the year, and the all-pro team'),
         ('usa_awards_daily_update', 'UsaAwards::daily_update', '0x906e10', 'award/usa_awards.cpp', 'thiscall, ret 4',
          'em 25 de novembro, os prêmios da MLS: técnico do ano, goleiro, defensor e estreante (até 21 anos) pelas notas da temporada, o artilheiro e a seleção; no dia do fim da temporada da nação, o prêmio do mês passa para a temporada seguinte; depois, todo dia, a atualização da base',
          "on 25 November, the MLS awards: manager of the year, goalkeeper, defender and rookie (up to 21) by the season's ratings, the scoring champion and the all-pro team; on the nation's season end day, the month award moves to the next season; then, every day, the base daily update")])
CAMPOS = []
INTERESSA = []
