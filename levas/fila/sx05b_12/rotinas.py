# Os premios da Suecia: o objeto, os dez premios e o dia dos premios (award/sweden_awards.cpp; leitor sx05b, rodada
# 5, 23/11/2026). Usa NationAwardsClass, nation_awards_construct, nation_awards_build, AwardSpec,
# award_player_of_year_from_ratings, award_top_three_from_ratings, nation_awards_base_daily_update e
# compare_nation_awards_built/init (sx05b_5) e nation_award, awards_day_is, award_manager_of_year e
# award_player_of_year (sx03_2, premios da Inglaterra): integrar depois dos dois.
# Rodam com a Suecia carregada: o construtor e o init na criacao do jogo (o AWARD_MANAGER 0x413cac), o daily_update
# todo dia (virtual); os premios em 1o de dezembro (2001-12-01, dentro da janela). LIGAS = 'ENG SWE FIN NOR POR'.
# Convencoes conferidas na desmontagem (cada push seguido ate a chamada; as listas dos argumentos dos construtores
# tiradas da desmontagem por um script e conferidas a mao na primeira e na ultima):
# - 0x893720 thiscall(arquivo) ret 4: NationAwards::NationAwards 0x768290; vtable 0x970354; +0xe = [0xae23a8] + 0x122 x
#   [KEY_NATION_SWEDEN]; +0x13 = 0x13; +0x12 = 1; com arquivo, NationAwards::load_from_disk 0x76a280 (eax); senao (ou
#   falhando) 0x8937b0(arquivo);
# - 0x8937b0 thiscall(arquivo) ret 4: +8 = 10 (word) antes do malloc 0x944e46(0x28) em +4 (nulo: show_error_at linha
#   0x68); cada premio por operator new 0x944cf1 (0x2b ou 0x25d; nulo deixa 0) e YEAR_AWARD 0x92fe80 (11 argumentos)
#   ou TEAM_AWARD 0x8af520 (10); o 1o premio tem o indice 0 literal e o tipo 1 (esi);
# - 0x893bf0 thiscall(hoje) ret 4: Date::Date 0x549f70 (1, 11, ano (word), -1); o ano e o dia comparados como words;
#   NationAwards: 0x768940 (hoje, premio, [chave]) ret 0xc, 0x7683c0 (10 argumentos, as notas [[0xad9c60]+0xc] lidas
#   antes) ret 0x28, 0x769380 (9) ret 0x24, 0x768cf0 (10) ret 0x28 e 0x768310 (hoje) ret 4.
# Comparadores: construtor e init so a original (alocam) e o objeto conferido (vtable, nacao, tipo, conta; cada premio
# com a nacao e o indice); daily_update so a original (os premios nao se dao duas vezes).
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ROTINAS = [('sweden_awards_construct', 0x893720, [0x6a, 0xff, 0x68, 0x58, 0x32, 0x96, 0x00]),
           ('sweden_awards_init', 0x8937b0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('sweden_awards_daily_update', 0x893bf0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Suécia',
        'the Swedish awards',
        [('sweden_awards_construct', 'SwedenAwards::SwedenAwards', '0x893720', 'award/sweden_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Suécia: a base com a tabela virtual da classe, a Suécia como nação, o id 0x13 e o tipo 1; com arquivo, os prêmios salvos; senão os dez prêmios',
          'builds the Swedish awards: the base with the class vtable, Sweden as the nation, id 0x13 and type 1; with a file, the saved awards; otherwise the ten awards'),
         ('sweden_awards_init', 'SwedenAwards::init_awards', '0x8937b0', 'award/sweden_awards.cpp', 'thiscall, ret 4',
          'cria os dez prêmios suecos: técnico do ano, jogador, jogador da torcida, goleiro, defensor, meio-campista e atacante do ano, a equipe da semana da Premier Division, o artilheiro dela e a equipe da semana da First Division, cada um com as reputações nacional e mundial dele',
          'creates the ten Swedish awards: manager of the year, player, fans\' player, goalkeeper, defender, midfielder and forward of the year, the Premier Division team of the week, its top goalscorer and the First Division team of the week, each with its national and world reputations'),
         ('sweden_awards_daily_update', 'SwedenAwards::daily_update', '0x893bf0', 'award/sweden_awards.cpp', 'thiscall, ret 4',
          'em 1º de dezembro, os prêmios do ano suecos: o técnico (pela Premier Division), o jogador e o da torcida pelas notas do ano, goleiro, defensor, meio-campista e atacante pelos três melhores das notas por posição, e o artilheiro da Premier Division; depois, todo dia, a atualização da base',
          'on 1 December, the Swedish year awards: the manager (from the Premier Division), the player and the fans\' player from the year ratings, goalkeeper, defender, midfielder and forward from the ratings\' top three by position, and the Premier Division top goalscorer; then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
