# Os premios da Finlandia: o objeto, os catorze premios e o dia dos premios (award/finland_awards.cpp; leitor rx12,
# rodada 6, 25/11/2026). Usa NationAwardsClass, nation_awards_construct, nation_awards_build, AwardSpec,
# award_top_three_from_ratings, nation_awards_base_daily_update, is_season_end_day, compare_nation_awards_built/init
# (sx05b_5, no motor.rs) e nation_award, awards_day_is, award_manager_of_year, award_player_of_year,
# award_team_of_season e month_award_start_new_season (sx03_2, england_awards.rs): todos ja publicados.
# Rodam com a Finlandia carregada: o construtor e o init na criacao do jogo (AWARD_MANAGER), o daily_update todo dia
# (virtual); os premios em 15 de novembro de 2001 e a virada dos premios do mes no fim da temporada finlandesa.
# Convencoes conferidas na desmontagem:
# - 0x5a8f70 thiscall(arquivo) ret 4: NationAwards::NationAwards 0x768290; vtable 0x96a748; +0xe = nacao de
#   KEY_NATION_FINLAND; +0x13 = 8; +0x12 = 1; com arquivo, 0x76a280 (eax != 0: fim); MONTH_RATINGS 0x767af0(0) em
#   operator new(0xf); nulo: caixa linha 0x51; senao 0x5a90b0(arquivo);
# - 0x5a90b0 thiscall(arquivo) ret 4: +8 = 14 (word) antes do malloc(0x38); nulo: caixa linha 0x79; os argumentos dos
#   construtores tirados por emulacao (rascunho rx09/emul_awards.py) e conferidos a mao no 1o e nos do mes; o ano dos
#   premios do mes e a word de G_CURRENT_DATE_YEAR lida como dword da pilha (os bytes altos sao lixo; o construtor so
#   guarda a word);
# - 0x5a96e0 thiscall(hoje) ret 4: Date::Date 0x549f70 (0xf, 0xa, ano, -1); 0x769380 (hoje, premio, 7 filtros) ret
#   0x24; 0x768940 ret 0xc; 0x768cf0 ret 0x28; 0x7698f0 ret 8; fim de temporada pela tabela 0xb63d75 (0x669340);
#   MONTH_AWARD::start_new_season 0x7672e0 (ano + 1, word) nos premios 5, 6, 10, 11; 0x768310 (hoje) ret 4.
# Comparadores: construtor e init so a original (alocam) e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('finland_awards_construct', 0x5a8f70, [0x6a, 0xff, 0x68, 0x59, 0x94, 0x95, 0x00]),
           ('finland_awards_init', 0x5a90b0, [0x6a, 0xff, 0x68, 0x34, 0x95, 0x95, 0x00]),
           ('finland_awards_daily_update', 0x5a96e0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Finlândia',
        'the Finnish awards',
        [('finland_awards_construct', 'FinlandAwards::FinlandAwards', '0x5a8f70', 'award/finland_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Finlândia: a base com a tabela virtual da classe, a Finlândia como nação, o id 8 e o tipo 1; com arquivo, os prêmios salvos; senão as notas do mês e os catorze prêmios',
          'builds the Finnish awards: the base with the class vtable, Finland as the nation, id 8 and type 1; with a file, the saved awards; otherwise the month ratings and the fourteen awards'),
         ('finland_awards_init', 'FinlandAwards::init_awards', '0x5a90b0', 'award/finland_awards.cpp', 'thiscall, ret 4',
          'cria os catorze prêmios finlandeses: jogador, jovem, goleiro e técnico do ano; e, para a Premier Division e a First Division, o artilheiro, o jogador e o técnico do mês (de maio a outubro), a equipe do ano e a equipe da semana, cada um com as reputações nacional e mundial dele',
          'creates the fourteen Finnish awards: player, young player, goalkeeper and manager of the year; and, for the Premier Division and the First Division, the top goalscorer, the player and the manager of the month (May to October), the team of the year and the team of the week, each with its national and world reputations'),
         ('finland_awards_daily_update', 'FinlandAwards::daily_update', '0x5a96e0', 'award/finland_awards.cpp', 'thiscall, ret 4',
          'em 15 de novembro, os prêmios do ano finlandeses: jogador, jovem (até 21) e goleiro pelos três melhores das notas do ano, o técnico do ano pela Premier Division, e o artilheiro e a equipe da temporada de cada divisão; no fim da temporada da Finlândia, os quatro prêmios do mês passam ao ano seguinte; depois, todo dia, a atualização da base',
          "on 15 November, the Finnish year awards: player, young player (up to 21) and goalkeeper from the year ratings' top three, the manager of the year from the Premier Division, and each division's top goalscorer and team of the season; on Finland's season end day the four month awards move on to the next year; then, every day, the base daily update")])
CAMPOS = []
INTERESSA = []
