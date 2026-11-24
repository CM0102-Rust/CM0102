# Os premios da Italia: o objeto, os 25 ou 40 premios e o dia dos premios (award/italy_awards.cpp; leitor sx04c,
# rodada 5, 23/11/2026). Traz tambem os auxiliares que os outros itens sx04c usam: AWARD_MANAGER_WORLD_RATINGS e
# award_player_of_year_from_world_ratings (as notas +0x10 do AWARD_MANAGER), awards_file_from_version (o arquivo dos
# premios que entraram no save numa versao posterior) e award_month_best_player (o jogador do mes que a Russia e a
# Escocia dao em linha). Usa NationAwardsClass, nation_awards_construct, nation_awards_build, AwardSpec,
# FN_NATION_AWARDS_PLAYER_OF_YEAR_FROM_RATINGS, nation_awards_base_daily_update e compare_nation_awards_built/init
# (sx05b_5) e nation_award, awards_day_is, awards_lower_divisions, award_manager_of_year, award_player_of_year e
# award_team_of_season (sx03_2): integrar depois dos dois e antes dos outros sx04c.
# Rodam com a Italia carregada: o construtor e o init na criacao do jogo (AWARD_MANAGER::create_nation_awards), o
# daily_update todo dia (virtual +4); os premios em 6 de junho (2002-06-06: ATE 2002-08-01).
# Convencoes conferidas na desmontagem (cada push seguido ate a chamada; as listas dos construtores tiradas por um
# script e conferidas a mao na primeira, na ultima e nas do arquivo dependente da versao):
# - 0x65fe30 thiscall(arquivo) ret 4: 0x768290; vtable 0x96c958; +0xe = nacao de [KEY_NATION_ITALY]; +0x13 = 0xd;
#   +0x12 = 1; com arquivo 0x76a280 (eax), senao (ou falhando) 0x65fec0(arquivo);
# - 0x65fec0 thiscall(arquivo) ret 4: +8 = 0x28 com o bit 4 de nacao+0x11c (divisoes inferiores), senao 0x19; malloc
#   0x944e46(+8 x 4) (nulo: show_error_at linha 0x9b); 25 premios e, com o bit relido, mais 15 (Serie C2 A, B, C);
#   os artilheiros (opcao 1; indices 5, 14, 19, 24, 29, 34, 39) recebem o arquivo so se |[0xdd7110]| >= 0x3e007
#   (a variavel local zerada antes do primeiro premio); o indice vem de esi, incrementado so quando o operator new
#   da certo (sem memoria os indices seguintes andariam um a menos: nao modelado, como nos itens sx05b);
# - 0x660ec0 thiscall(hoje) ret 4: Date::Date 0x549f70 (6, 5, ano, -1); 0x768940 (hoje, premio, [chave]) ret 0xc;
#   0x768cf0 (chave, hoje, premio, 7 filtros) ret 0x28; 0x7698f0 (chave, premio) ret 8; 0x768310 (hoje) ret 4.
# Comparadores: construtor e init so a original (alocam) e o objeto conferido; daily_update so a original.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2002-08-01'
ROTINAS = [('italy_awards_construct', 0x65fe30, [0x6a, 0xff, 0x68, 0xf8, 0xbd, 0x95, 0x00]),
           ('italy_awards_init', 0x65fec0, [0x6a, 0xff, 0x68, 0x40, 0xc0, 0x95, 0x00]),
           ('italy_awards_daily_update', 0x660ec0, [0x83, 0xec, 0x10, 0x56, 0x57])]
DOCS = ('os prêmios da Itália',
        'the Italian awards',
        [('italy_awards_construct', 'ItalyAwards::ItalyAwards', '0x65fe30', 'award/italy_awards.cpp', 'thiscall, ret 4',
          'monta os prêmios da Itália: a base com a tabela virtual da classe, a Itália como nação, o id 0xd e o tipo 1; com arquivo, os prêmios salvos; senão cria os prêmios',
          'builds the Italian awards: the base with the class vtable, Italy as the nation, id 0xd and type 1; with a file, the saved awards; otherwise creates the awards'),
         ('italy_awards_init', 'ItalyAwards::init_awards', '0x65fec0', 'award/italy_awards.cpp', 'thiscall, ret 4',
          'cria 25 prêmios (40 com as divisões inferiores): na Série A a equipe da semana e a do ano, técnico, jogador, jovem, artilheiro, defensor, goleiro, italiano e estrangeiro do ano; na B, C1 A e C1 B (e nos três grupos da C2) a equipe da semana e a do ano, técnico, jogador e artilheiro; os artilheiros só vêm do arquivo se o save for da versão 0x3e007 em diante',
          'creates 25 awards (40 with the lower divisions): in Serie A the team of the week and of the year, manager, player, young player, top goalscorer, defender, goalkeeper, Italian and foreign player of the year; in B, C1 A and C1 B (and the three C2 groups) the team of the week and of the year, manager, player and top goalscorer; the top goalscorers come from the file only when the save is version 0x3e007 or later'),
         ('italy_awards_daily_update', 'ItalyAwards::daily_update', '0x660ec0', 'award/italy_awards.cpp', 'thiscall, ret 4',
          'em 6 de junho, o técnico do ano de cada divisão, os prêmios de jogador da Série A pelas estatísticas (jogador, jovem até 23, defensor, goleiro, artilheiro, italiano e estrangeiro), jogador e artilheiro das outras divisões e a equipe da temporada de cada uma (os grupos da C2 só com as divisões inferiores); depois, todo dia, a atualização da base',
          'on 6 June, the manager of the year of each division, the Serie A player awards from the stats (player, young player up to 23, defender, goalkeeper, top goalscorer, Italian and foreign player), player and top goalscorer of the other divisions and the team of the season of each (the C2 groups only with the lower divisions); then, every day, the base daily update')])
CAMPOS = []
INTERESSA = []
