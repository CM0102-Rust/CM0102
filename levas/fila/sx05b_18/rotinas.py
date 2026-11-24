# Os premios do Pais de Gales: o objeto, os seis premios e o dia dos premios (award/wales_awards.cpp; leitor sx05b,
# rodada 5, 23/11/2026). Usa os construtores e chamadas comuns dos premios das nacoes (sx05b_5) e nation_award,
# awards_day_is, award_manager_of_year, award_player_of_year e month_award_start_new_season (sx03_2): integrar depois
# dos dois.
# Rodam com Gales carregado: o construtor e o init na criacao do jogo (AWARD_MANAGER 0x413f61), o daily_update todo
# dia; os premios em 11 de maio (2002-05-11, dentro da janela). LIGAS = 'ENG SCO WAL'.
# Convencoes conferidas na desmontagem (as listas de argumentos dos construtores tiradas por script e conferidas a mao
# na equipe e no premio do mes):
# - 0x90ead0 thiscall(arquivo) ret 4: como o dos EUA, vtable 0x970a88, [KEY_NATION_WALES], +0x13 = 0x1c, +0x12 = 1; as
#   notas do mes (linha 0x49);
# - 0x90ec10 thiscall(arquivo) ret 4: o ano local de [0xae2c92] (word); +8 = 6, malloc(0x18) (nulo: linha 0x71); a
#   equipe, quatro do ano e o tecnico do mes (MONTH_AWARD 0x766e10 de agosto (7) do ano a abril (3) do ano seguinte:
#   `lea ecx, [edx + 1]` sobre o dword do ano; o construtor so le a palavra);
# - 0x90ef10 thiscall(hoje) ret 4: Date::Date 0x549f70 (0xb, 4, ano, -1); tres 0x768cf0 (jogador estatistica 0x11,
#   artilheiro 2, jovem idade 0x15) e 0x768940; o dia do fim da temporada passa o premio do mes (+0x14) a temporada
#   seguinte; 0x768310.
# Comparadores: construtor e init so a original (alocam) e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG SCO WAL'
ROTINAS = [('wales_awards_construct', 0x90ead0, [0x6a, 0xff, 0x68, 0xc9, 0x55, 0x96, 0x00]),
           ('wales_awards_init', 0x90ec10, [0x6a, 0xff, 0x68, 0x34, 0x56, 0x96, 0x00]),
           ('wales_awards_daily_update', 0x90ef10, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios do País de Gales',
        'the Welsh awards',
        [('wales_awards_construct', 'WalesAwards::WalesAwards', '0x90ead0', 'award/wales_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios de Gales: a base com a tabela virtual da classe, Gales, o id 0x1c e o tipo 1; com arquivo, os salvos; senão as notas do mês (sem memória, a caixa de erro e nenhum prêmio) e os seis prêmios',
          'builds the Welsh awards: the base with the class vtable, Wales, id 0x1c and type 1; with a file, the saved ones; otherwise the month ratings (out of memory, the error box and no awards) and the six awards'),
         ('wales_awards_init', 'WalesAwards::init_awards', '0x90ec10', 'award/wales_awards.cpp', 'thiscall, ret 4',
          'cria os seis prêmios galeses da Premier Division: a equipe da semana, jogador, jovem, artilheiro e técnico do ano, e o técnico do mês (de agosto do ano corrente a abril do seguinte)',
          'creates the six Welsh Premier Division awards: the team of the week, player, young player, top goalscorer and manager of the year, and the manager of the month (August of the current year to April of the next)'),
         ('wales_awards_daily_update', 'WalesAwards::daily_update', '0x90ef10', 'award/wales_awards.cpp', 'thiscall, ret 4',
          'em 11 de maio, os prêmios galeses: jogador, artilheiro e jovem (até 21 anos) do ano e o técnico do ano; no dia do fim da temporada da nação, o prêmio do mês passa para a temporada seguinte; depois, todo dia, a atualização da base',
          "on 11 May, the Welsh awards: player, top goalscorer and young player (up to 21) of the year and the manager of the year; on the nation's season end day, the month award moves to the next season; then, every day, the base daily update")])
CAMPOS = []
INTERESSA = []
