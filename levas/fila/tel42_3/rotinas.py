# The club history lists of the club screens (club_history.cpp; reader tel42, screens round 2, 22/11/2026): the
# league seasons and the cup runs of a club, the distinct competitions of each list, the current league record and the
# qsort comparison of the ids. Self-contained (its own club_history_lists constants): no order with tel42_1/tel42_2.
# Proven in the disassembly (prologue to the last ret; no jump tables; the SEH frames only guard the operator new of a
# list, no destructor to reproduce):
# - 0x443380 Club_History::get_league_history(club): thiscall, ret 4 -> LLIST* (called by the club history screen
#   builder 0x46afbc, already Rust: exe::club_history_get_league_history).
# - 0x443520 Club_History::get_cup_history(club): thiscall, ret 4 -> LLIST* (0x46b191).
# - 0x443770 Club_History::get_league_comp_ids(list): thiscall, ret 4 -> LLIST* of ints (0x46afe1); this not read.
# - 0x443a20 Club_History::get_cup_comp_ids(list): thiscall, ret 4 (0x46b1ba); byte for byte the same code as
#   0x443770 except the three error lines (0x298, 0x2a3, 0x2ce against 0x238, 0x243, 0x26e): one helper.
# - 0x444190 club_history_get_current_league_record(club, rec, &complete, &stage, reserve): cdecl, 5 arguments, returns
#   1 or 0 (called by 0x44341d, get_league_history, with (club, rec, &flag, 0, 0) and by the club screen 0x4550a4,
#   club_screen_build of tel01_2, with (club, rec, &complete, &stage, 1)).
# - 0x444500 compare_int_ids(a, b): cdecl (qsort of 0x443770/0x443a20).
# - Calls: FILE_LLIST::get_nodes 0x58f120 thiscall(list, first, last) ret 8 -> LLIST*; LLIST::LLIST 0x68d020,
#   ~LLIST 0x68d040, add_tail 0x68d0d0 thiscall(data, size) ret 8, get_next 0x68d1b0, get_prev 0x68d1d0; operator
#   new/delete; qsort 0x945333; Comp::find_club_progress 0x48cc70 thiscall(club, &stage, &round, &status) ret 0x10
#   (Rust); the competition objects' virtual +0xc (no argument, al) and +0x10 (1, 1; al); comp_util_is_league_division,
#   Friendly::is_friendly_competition, League::sort_table (thiscall(stage, 0)), Date::Date 0x549f70 thiscall(day,
#   month, year, -1) ret 0x10.
# Same in the cm0102-1080 exe (all six whole).
# Runs only from the screens: Club -> History (the league and cup tabs: the four lists) and the club screen (the
# current league record in the header "<pos> in <division>"); world coverage 0 for the five club_history ones.
# Comparers: the four lists side by side by content (the copy's list destroyed after); the current record side by side
# over the record and the two answers; compare_int_ids through compare_order!. Every one runs both sides: the table
# sorts they ask for give the same table twice [inference: League::sort_table is a total order ending on the club id].
# How to reach them in the game: any club -> Club -> History (league and cup history tabs); the club screen header of
# a club (the current league record; Brazilian clubs in February-June show the state league).
# Defects of the original kept: compare_int_ids returns 1 for any two different ids (qsort never orders the
# competitions: the order is whatever qsort leaves); get_league_history reads a completion flag that the current league
# record does not write when the club is found only in a stage it has not played yet (a stack slot never set: the
# copy takes 0, the record shown); in that path the record takes the club's DIVISION id even when the reserve (state)
# league was searched; a failed operator new of the first list crashes in LLIST::add_tail (this = 0).
TELAS = True
ROTINAS = [('club_history_get_league_history', 0x443380, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x6a, 0xff]),
           ('club_history_get_cup_history', 0x443520, [0x6a, 0xff, 0x68, 0x6c, 0x4f, 0x95, 0x00]),
           ('club_history_get_league_comp_ids', 0x443770, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x6a, 0xff]),
           ('club_history_get_cup_comp_ids', 0x443a20, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x6a, 0xff]),
           ('club_history_get_current_league_record', 0x444190, [0x6a, 0xff, 0x68, 0xcb, 0x4f, 0x95, 0x00]),
           ('compare_int_ids', 0x444500, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08])]
DOCS = ('as listas do histórico do clube das telas: temporadas de liga, campanhas de copa, as competições de cada lista e o registro da liga atual',
        'the club history lists of the screens: league seasons, cup runs, the competitions of each list and the current league record',
        [('club_history_get_league_history', 'Club_History::get_league_history', '0x443380', 'club_history.cpp', 'thiscall, ret 4',
          'monta a lista das temporadas de liga do clube, a mais nova primeiro: a temporada atual na frente se a liga ainda não acabou, depois as gravadas no arquivo, da última para a primeira',
          'builds the list of the club\'s league seasons, newest first: the current season in front when the league is not over, then the stored ones, from the last to the first'),
         ('club_history_get_cup_history', 'Club_History::get_cup_history', '0x443520', 'club_history.cpp', 'thiscall, ret 4',
          'monta a lista das campanhas de copa do clube: primeiro o progresso em cada copa em andamento (que não é liga, divisão nem amistoso) onde ele chegou a uma fase, depois as gravadas, da última para a primeira',
          'builds the list of the club\'s cup runs: first its progress in each running cup (not a league, a division nor a friendly) where it reached a stage, then the stored ones, from the last to the first'),
         ('club_history_get_league_comp_ids', 'Club_History::get_league_comp_ids', '0x443770', 'club_history.cpp', 'thiscall, ret 4',
          'junta as competições distintas de uma lista de temporadas de liga e as devolve numa lista nova, "ordenadas" por uma comparação que nunca ordena',
          'collects the distinct competitions of a league season list and returns them in a new list, "sorted" by a comparison that never orders'),
         ('club_history_get_cup_comp_ids', 'Club_History::get_cup_comp_ids', '0x443a20', 'club_history.cpp', 'thiscall, ret 4',
          'o mesmo para uma lista de campanhas de copa',
          'the same for a cup run list'),
         ('club_history_get_current_league_record', 'club_history_get_current_league_record', '0x444190', 'club_history.cpp', 'cdecl',
          'acha o clube na tabela atual da sua divisão (no Brasil, de fevereiro a junho, a estadual, quando pedido), da fase atual para trás, só fases-liga com mais de 7 rodadas (qualquer uma na Polônia; na Argentina só a liga), e preenche o registro com divisão, ano, posição e totais; diz também se a temporada acabou e em que fase achou',
          'finds the club in the current table of its division (in Brazil, February to June, the state league when asked), from the current stage back, only league stages with more than 7 rounds (any in Poland; in Argentina only the league), and fills the record with division, year, position and totals; also tells whether the season is over and in which stage it was found'),
         ('compare_int_ids', 'compare_int_ids', '0x444500', 'club_history.cpp', 'cdecl',
          'comparação do qsort que nunca ordena: 0 iguais, 1 diferentes',
          'qsort comparison that never orders: 0 equal, 1 different')])
