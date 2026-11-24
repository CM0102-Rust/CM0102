# Os textos e listas de competicao das telas (comp.cpp, comp_util.cpp, Cup.cpp; leitor cx01, rodada 5, 24/11/2026).
# Item de TELAS. Caminhos no jogo (o Lucas jogando comparando):
# - 0x48cab0 Comp::write_progress_text thiscall(saida, tamanho, fase, rodada, situacao, comp) ret 0x18 (slot 0x38;
#   club_history_draw_competitions_tab 0x46b4b9): a ficha do clube, historico, aba das competicoes. Dentro:
#   comp_util_get_parent_stage_id cdecl(fase); langlib_sprintf cdecl; comp_util_format_stage_round_name cdecl(fase,
#   rodada, saida, tamanho, 0); a caixa 0x1d5.
# - 0x48dfa0 Comp::get_club_label_text thiscall(saida, clube, fase) ret 0xc -> saida (slot 0x4c; nao achei chamada direta
#   pelo slot na varredura do exe: chamada por ponteiro [inferencia]): sprintf da saida vazia; League::get_table_row
#   0x6826d0 (push k; mov ecx -> thiscall, ret 4); utils_format_ordinal cdecl(posicao, -1); a caixa 0x607.
# - 0x522c50 Cup::update_projected_positions thiscall(clube) ret 4 -> 1 (slot 0x84; Comp_Stats::
#   get_club_projected_positions 0x4ae424, telas das estatisticas da competicao): o slot 0x68 (5 push; mov ecx).
# - 0x4bf930 comp_util_format_fixture_round_label cdecl(partida, forma, buf, tamanho) -> eax (proxima partida do clube,
#   encontros anteriores, linha das partidas do clube, painel da partida, impressao): 0x4bfa30 cdecl; as caixas 0x2587 e
#   0x258f.
# - 0x4c0140 comp_util_collect_club_honours cdecl(clube, outro, &lista) -> n e 0x4c02f0 compare_club_honours cdecl(a, b)
#   (o historico do clube, aba das competicoes): realloc cdecl; qsort cdecl(lista, n, 0x1a, 0x4c02f0); os globais
#   0xae0ff8/0xae0ffc com os dois clubes; a caixa 0x2703.
# - 0x4c0520 comp_util_sort_fixture_list cdecl(lista) -> 0/1 e 0x4c06c0 compare_fixtures_by_round_and_home_name cdecl(a,
#   b) (a lista de partidas da competicao: comp_screens_open_comp_fixtures e a aba das partidas): LLIST::get_next
#   0x68d1b0, clear 0x68d080, add_tail 0x68d0d0 (push 0x4f; push dado; mov ecx -> thiscall, ret 8); utils_string_compare
#   cdecl(a, b, 1, 0); as caixas 0x2782, 0x278d e 0x2796.
# - 0x48d500 Comp::build_match_date_list thiscall(&n) ret 4 -> lista e 0x48da90 comp_compare_fixture_list_entries
#   cdecl(a, b) (a aba do calendario da competicao; tambem o PLAYER_SEARCH::start_scout_assignment e o
#   SCOUT_MANAGER::collect_current_duty_comps): o slot 0x3c; Date::Date, Date::get_nearest_weekday 0x54a2d0
#   thiscall(dia_da_semana, &saida) ret 8, Date::add_days, Date::operator=; operator new/delete das datas; realloc;
#   qsort(lista, n, 0x14, 0x48da90); a etiqueta 0x4bfa30 nos buffers 0xde1f64 e 0xde234c; as caixas 0x3dd e 0x497.
# - 0x519d90 Cup::find_scheduled_fixture_for_tie thiscall(rodada, confronto) ret 8 -> partida (a aba das rodadas da
#   copa): FIXTURE_MANAGER::find_next_fixture (adiante 0, para tras); Date::sub_days 0x54c8e0; as caixas 0x3b9 e 0x3c1.
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: os textos lado a lado (a copia num buffer local do mesmo tamanho); as listas lado a lado (a copia monta
# a dela e a libera); as ordens pelo compare_order; a lista de partidas so a original (refaz a lista), conferindo a
# ordem; a partida do confronto pelo valor.
# Defeito do original (0x48d500): a lista tem 1000 entradas e nao e conferida; sem entradas, *n fica 1000 e a lista
# vaza.
SAVE = False
TELAS = True
ROTINAS = [('comp_write_progress_text', 0x48cab0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_get_club_label_text', 0x48dfa0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cup_update_projected_positions', 0x522c50, [0x8a, 0x51, 0x3e, 0x8b, 0x01]),
           ('comp_util_format_fixture_round_label', 0x4bf930, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_util_collect_club_honours', 0x4c0140, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00]),
           ('compare_club_honours', 0x4c02f0, [0x83, 0xec, 0x08, 0x8b, 0x44, 0x24, 0x0c]),
           ('comp_util_sort_fixture_list', 0x4c0520, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('compare_fixtures_by_round_and_home_name', 0x4c06c0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('comp_build_match_date_list', 0x48d500, [0x6a, 0xff, 0x68, 0x4d, 0x60, 0x95, 0x00]),
           ('comp_compare_fixture_list_entries', 0x48da90, [0x8b, 0x4c, 0x24, 0x04, 0x53]),
           ('cup_find_scheduled_fixture_for_tie', 0x519d90, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os textos e listas de competição das telas',
        'the competition texts and lists of the screens',
        [('comp_write_progress_text', 'Comp::write_progress_text', '0x48cab0', 'comp.cpp', 'thiscall, ret 0x18',
          'escreve o progresso do clube na competição: numa fase de grupos, "Classificado dos Grupos" ou "Fase de Grupos"; senão campeão, vice ou terceiro (sem fase nem rodada) ou o nome da fase e rodada',
          "writes the club's progress in the competition: in a group stage, \"Qualified From Groups\" or \"Group Stage\"; otherwise winners, runners-up or third (with no stage or round) or the stage and round name"),
         ('comp_get_club_label_text', 'Comp::get_club_label_text', '0x48dfa0', 'comp.cpp', 'thiscall, ret 0xc',
          'o rótulo curto de um clube: numa fase de liga a posição como ordinal; numa competição de clubes a sigla da divisão, numa de seleções a da nação',
          'the short label of a club: in a league stage the position as an ordinal; in a club competition the three-letter name of its division, in a national one that of its nation'),
         ('cup_update_projected_positions', 'Cup::update_projected_positions', '0x522c50', 'Cup.cpp', 'thiscall, ret 4',
          'na copa, a posição projetada do clube vai às estatísticas pela fase (atual e melhor 1, pior o número de times)',
          "in a cup, the club's projected position goes to the statistics by the stage (current and best 1, worst the team count)"),
         ('comp_util_format_fixture_round_label', 'comp_util_format_fixture_round_label', '0x4bf930', 'comp_util.cpp', 'cdecl',
          'o rótulo da competição, fase e rodada de uma partida; sem partida, buffer ou tamanho, ou sem competição, a caixa de erro e 0',
          "a fixture's competition, stage and round label; without a fixture, buffer or size, or without a competition, the error box and 0"),
         ('comp_util_collect_club_honours', 'comp_util_collect_club_honours', '0x4c0140', 'comp_util.cpp', 'cdecl',
          'junta as linhas do histórico em que o clube (ou o outro) foi campeão, vice ou terceiro e as ordena para a lista de títulos',
          'gathers the history rows in which the club (or the other one) was champion, runner-up or third and sorts them for the honours list'),
         ('compare_club_honours', 'compare_club_honours', '0x4c02f0', 'comp_util.cpp', 'cdecl',
          'a ordem da lista de títulos: continentais, depois nacionais (copas antes das ligas), depois a reputação e o id da competição, a colocação e a temporada',
          'the order of the honours list: continental, then national (cups before leagues), then the competition reputation and id, the place and the season'),
         ('comp_util_sort_fixture_list', 'comp_util_sort_fixture_list', '0x4c0520', 'comp_util.cpp', 'cdecl',
          'ordena a lista ligada de partidas (copia, ordena e refaz a lista)',
          'sorts the linked list of fixtures (copies, sorts and rebuilds the list)'),
         ('compare_fixtures_by_round_and_home_name', 'compare_fixtures_by_round_and_home_name', '0x4c06c0', 'comp_util.cpp', 'cdecl',
          'a ordem das partidas: fase, prefixo e número da rodada, nome da rodada e o nome do mandante',
          'the order of the fixtures: stage, round prefix and number, round name and the home club name'),
         ('comp_build_match_date_list', 'Comp::build_match_date_list', '0x48d500', 'comp.cpp', 'thiscall, ret 4',
          'a lista das datas de jogo de todas as fases da competição (as datas extras de cada rodada, a ida e a volta), ordenada por data',
          'the list of match dates of every stage of the competition (each round\'s extra days, both legs), sorted by date'),
         ('comp_compare_fixture_list_entries', 'comp_compare_fixture_list_entries', '0x48da90', 'comp.cpp', 'cdecl',
          'a ordem dessa lista: a data, a competição de maior reputação, o rótulo da fase e rodada',
          'the order of that list: the date, the competition of higher reputation, the stage and round label'),
         ('cup_find_scheduled_fixture_for_tie', 'Cup::find_scheduled_fixture_for_tie', '0x519d90', 'Cup.cpp', 'thiscall, ret 8',
          'a última partida já jogada entre os dois clubes de um confronto da copa, procurando para trás a partir de agora',
          'the last fixture already played between the two clubs of a cup tie, searching backwards from now')])
CAMPOS = [('competição +0x53 sigla de três letras [inferência]', 'competition +0x53 three-letter name [inference]')]
INTERESSA = []
