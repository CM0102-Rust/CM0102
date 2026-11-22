# The scout's working day (player_search.cpp; reader r8_scout, round 4, the big routines, 21/11/2026).
# - 0x7b3e20 PLAYER_SEARCH::run_scout_day (7616 bytes, 0x7b3e20..0x7b5bdc, thiscall, ret 0, `and esp, -8` frame with
#   SEH): called by the scouts' day (scout_manager.rs, 0x7f8750) for each of the human managers' lists; 3220 calls in
#   the ENG career to 2002-06-01, all returning early (no report: see mutacoes.py). Split into named pieces:
#   scout_day (the day), scout_day_keeps (the filter), scout_day_remove_result, scout_day_report (one player's report
#   0x138b), scout_day_final_report (the summary 0x138a, scout_day_best_players, scout_day_shortlisted,
#   scout_day_player_flags), scout_day_next_day; ScoutDayRun (deliveries and fog of war, real or dry).
# Identical in cm0102.exe and cm0102-1080.exe.
# Conventions checked in the disassembly (every push against esp of the moment):
# - 0x53e990 database_staff_matches_names cdecl(scout, [+0x174], [+0x178], [+0x17c], +0x184);
# - 0x77bbe0 NewsItem::NewsItem thiscall(item, kind, 0) ret 8; 0x785790 set_slot thiscall(item, index byte (dword with
#   garbage above), value) ret 8; 0x7e5160 cdecl(name ref, item, &index); 0x7e4920 cdecl(person, 0x18-byte buffer);
#   0x785d40 deliver_to_manager thiscall([0xb7431c], &item, manager, 0) ret 0xc; no destructor is called;
# - 0x7b1520 (tidies the results in place, frees an empty array) thiscall(this, 0) ret 4;
# - 0x5afde0 thiscall([0xae2a60], manager) and 0x5b09d0 thiscall(map, player, scout) ret 8;
# - 0x5407e0 cdecl(player, scout, 0) -> ax; 0x4cf5a0 thiscall([0xae1050], player), the answer stored in [0xae1080]
#   (not in the best-players fill, 0x7b50ba); 0x7a7eb0 thiscall([0xb7434c], player) -> al;
# - 0x417c10 date operator= thiscall(dst, &tmp, src); 0x945333 qsort / 0x9452ca free / 0x944e46 malloc cdecl;
#   0x90cfc0 random cdecl;
# - 0x8c7560 thiscall([0xde1f54], player, club, flag) ret 0xc; 0x8c7950 thiscall([0xde1f54], player, nation, 0, 0, 1)
#   ret 0x14; 0x5b7d20 cdecl -> ax; 0x7bd9d0 cdecl(player, scout, &word36, &word34); 0x546ab0 cdecl(player, club);
# - 0x7c1e30 get_stat thiscall([0xb74350], sheet, 6, which, 6.5f) ret 0x10 (x87); 0x7c1950 thiscall([0xb74350],
#   sheet) -> al; 0x54cdb0 thiscall(0xae2c90, day dword, leap dword) -> eax; 0x62c400 thiscall([0xb63d28], person) -> al;
# - 0x549100, 0x5492c0, 0x549200, 0x549470, 0x5495b0, 0x5494d0, 0x549690, 0x549770, 0x549940, 0x549850, 0x5493c0
#   cdecl(club) -> al and 0x549a30 cdecl(club, club) -> al (Database.cpp, queue items dbo_1..dbo_4);
# - 0x7ae9c0 start_scout_assignment thiscall(this, nation, region (only the byte is read), competition, base,
#   opposition) ret 0x14; 0x7aee80 thiscall(this) (ends the assignment); 0x54c5b0 thiscall(date, &out, days) ret 8.
# DEFECTS of the original kept: the summary without a target club sends 14 zeros where the target branch sends 15
# answers; the final report asks the work permit for the club left in [esp+0x30] (the club of the last reported
# player's run, or the count of flagged results when nobody was reported, which is then 0); the same-club run reports
# past the day's number.
# Simplification: the local date the original constructs at the start (0x54a110 on [esp+0x158]) is never used: not
# built.
# Comparer: dry run first (copy of the result array by the game's malloc, news kept, fog of war records saved, no
# restart/end of the assignment), then the object, records, generator and contract lookup go back and the original
# runs; compared: the return, each delivered news item in the inbox ring (kind and data), the fog of war records and,
# without restart/end, the object (minus the array pointer), the array, the generator and the contract lookup. Risk:
# the delivery's automatic reply (manager on holiday) runs only in the original.
SAVE = False
ROTINAS = [('run_scout_day', 0x7b3e20, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('o dia de trabalho do olheiro',
        "the scout's working day",
        [('run_scout_day', 'PLAYER_SEARCH::run_scout_day', '0x7b3e20', 'player_search.cpp', 'thiscall',
          'Só na lista de tipo 4 com olheiro (nome ainda confere) e alguma tarefa (nação, região, competição, base ou adversário); antes da data de fim, nada (1). No dia: o primeiro dia é a notícia 0x1389. Depois: a tarefa dura 1 dia (competição ou adversário) ou 8; os resultados são arrumados; cada um mostra o jogador à névoa de guerra do técnico e só fica se servir (o olheiro tem clube e o jogador ficha; com o adversário, sempre; senão a nação dele (do clube ou a própria) tem de ser a pedida e da região pedida; na base, menor de 20 sem clube, da região do clube do olheiro ou valendo mais de 40000; da nação do olheiro, reputação para ele abaixo de 4000; nunca do clube do olheiro nem prestes a se aposentar); os marcados são os com o bit 0. Quantos relatar hoje: antes do último dia (75..124)% dos resultados / (dias × 100), no último todos os marcados, no mínimo 1 e no máximo os marcados; cada um é um marcado ao acaso, com a notícia 0x138b (nota 4 ou mais), e logo os outros marcados do mesmo clube. No último dia o resumo 0x138a (com o clube-alvo: os dois melhores e 15 respostas sobre o clube; sem: 14 zeros), entregue havendo resultados, senão a tarefa recomeça; sem o adversário, a tarefa termina. Na metade, a notícia 0x138a com a contagem. O próximo dia: 2 dias depois da data da competição, ou em 2..8 dias; um dia a mais feito',
          "Only on a type-4 list with a scout (whose name still matches) and some assignment (nation, region, competition, base or opposition); before the end date, nothing (1). On the day: the first day is news item 0x1389. Then: the assignment lasts 1 day (competition or opposition) or 8; the results are tidied; each shows the player to the manager's fog of war and stays only if it fits (the scout has a club and the player a sheet; with the opposition, always; otherwise his nation (his club's or his own) has to be the assigned one and of the assigned region; for the base, under 20 without a club, from the region of the scout's club or worth more than 40000; from the scout's nation, reputation for him below 4000; never of the scout's club nor about to retire); the flagged ones have bit 0. How many to report today: before the last day (75..124)% of the results / (days × 100), on the last day all the flagged, at least 1 and at most the flagged; each is a random flagged one, with news item 0x138b (grade 4 or more), followed by the other flagged ones of the same club. On the last day the summary 0x138a (with the target club: the two best players and 15 answers about the club; without: 14 zeros), delivered when there are results, otherwise the assignment starts again; without the opposition, the assignment ends. Halfway, news item 0x138a with the count. The next day: 2 days after the competition's date, or in 2..8 days; one more day done")])
