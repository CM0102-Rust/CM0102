# The AI pre-season tournaments and tours (friendly.cpp, in comp\). Reader r5_C, round 4 (the big routines), 21/11/2026.
# - 0x5c3040 Friendly::plan_preseason_tournaments_and_tours (2976 bytes): 2 calls in the ENG career to 2002-06-01
#   (docs/cobertura_simulacao.txt), on the two pre-season planning days of July 2001 (Friendly::update_for_date 0x5c2460
#   -> plan_ai_preseason_friendlies 0x5c27a0). In 2001 only the tours run (the tournaments are skipped while the year is
#   2001): the tournament half first runs in the pre-season of 2002 (after the chain's date).
# Equal in cm0102-1080.exe (whole).
# Conventions checked in the disassembly (0x5c3040..0x5c3bd0) and at the caller (0x5c27a0: 3 pushes; ecx = this ->
# thiscall, ret 0xc):
# - 0x946464 (bsearch): 5 pushes; add esp, 0x14 -> cdecl; 0x54cab0 (Date::sub_weeks): lea ecx; 2 pushes -> thiscall;
# - 0x600a50 (HumanManager::is_human_club): 5 pushes; add esp, 0x14 -> cdecl(club, 1, 1, 0, 0);
# - 0x90cfc0 (utils_random): 1 push; add esp, 4 -> cdecl;
# - 0x5c4cb0 (find_friendly_opponent): 5 pushes (host, entries, count, band + 1, &group); ecx = this -> thiscall, ret 0x14;
# - 0x944cf1 (operator new 0xe) cdecl, 0x68d020 (LLIST::LLIST) thiscall, 0x68d0d0 (LLIST::add_tail: push 4; push
#   candidate; ecx = list) thiscall, ret 8;
# - 0x5c3be0 (create_ai_tournament): push kind (a dword whose high bytes are stack leftovers; the callee reads the
#   byte); push 4; push &group; ecx = this -> thiscall, ret 0xc;
# - 0x5c4680 (pick_tour_nation): push club; ecx = this -> thiscall, ret 4;
# - 0x5c5040 (find_tour_opponent): 5 pushes (candidate, sorted, count, band dword, nation); ecx = this -> thiscall, ret 0x14;
# - 0x945640 (_ftol) on band x 0.8 (fild; fmul [0x9689b0]);
# - 0x5c4040 (schedule_tour_matches): push n (dword with leftover high bytes; read as a byte); push &tour; push
#   candidate; ecx = this -> thiscall, ret 0xc.
# Defects of the original (DEFEITOS): the tournament half reads a byte counter never initialised before the first
# tournament (stack leftover; the copy starts it at 0) and bounds it by the low byte of the nation's reputation (as a
# signed char); with no opponent after 50 calls the null slot goes into the lists and the game crashes.
# Comparer: only the original runs (draws, lists, tournaments and tour friendlies cannot run twice); afterwards every
# opponents link must be mutual and every candidate's friendly count between 0 and 10.
SAVE = False
ROTINAS = [('plan_preseason_tournaments_and_tours', 0x5c3040, [0x6a, 0xff, 0x68, 0x24, 0x9f, 0x95, 0x00])]
DOCS = ('os torneios e as excursões de pré-temporada da IA',
        'the AI pre-season tournaments and tours',
        [('plan_preseason_tournaments_and_tours', 'Friendly::plan_preseason_tournaments_and_tours', '0x5c3040', 'friendly.cpp', 'thiscall, ret 0xc',
          'Na pré-temporada de cada liga escolhida (temporada ainda não começada e a 52 semanas do fim): a partir de 2002, um torneio por nação: entre os clubes da IA sem lista de datas e fora da Intertoto, o maior sorteio da faixa de reputação é a sede (e um sorteio de 10 tem de dar diferente de 0), que recebe três adversários de amistoso (até 50 tentativas cada, sem lista de datas), todos com todos na lista de adversários, numa copa ou minliga (meio a meio); defeito: o contador do laço não é iniciado antes do primeiro torneio e o limite é o byte baixo da reputação da nação. Todo ano, as excursões: cada clube da IA sem adversários, com liga, com até 4 amistosos e que passa num sorteio de 3, com uma nação de excursão, enfrenta clubes dela até ter 3 ou 4 amistosos (faixa = reputação / 500 + sorteio, × 0,8 a cada adversário, 10 abaixo de 1) e, com mais de um adversário, a excursão é marcada',
          'In the pre-season of each selected league (season not started yet and within 52 weeks of the end): from 2002 on, one tournament per nation: among the AI clubs without a date list and outside the Intertoto, the highest draw of the reputation band hosts (and a draw of 10 must be nonzero), taking three friendly opponents (up to 50 tries each, without a date list), all against all in the opponents lists, in a cup or a mini league (half and half); defect: the loop counter is not initialised before the first tournament and the bound is the low byte of the nation reputation. Every year, the tours: each AI club without opponents, with a league, with at most 4 friendlies and passing a draw of 3, given a tour nation, meets its clubs until it has 3 or 4 friendlies (band = reputation / 500 + draw, × 0.8 after each opponent, 10 below 1) and, with more than one opponent, the tour is scheduled')])
