# The Club World Championship: one entrant and the knockout stage (world_club_champ.cpp; reader rxg, round 6,
# 25/11/2026). Simulation with the 8 leagues and ATE 2003-08-15, as rx10_11..rx10_14 and rxf_6: the draw is on 5 Feb
# 2003 (pick_entrants, rxf_6, calls pick_entrant by address 12 times) and the knockout stage on 5 Aug 2003 (the
# advance_stage of rx10_11 calls 0x92a140 by address at stage 2). Same bytes in cm0102-1080.exe.
# - 0x92ae60 WorldClubChamp::pick_entrant thiscall(this, continent, nation, list, count, club comp) ret 0x14, returns
#   a club; the pushes conferred in pick_entrants (rxf_6). Prologue `mov eax, 0x2144` (5 bytes); the `call __chkstk`
#   right after it stays in the original (not in the prologue: no RELATIVAS). Last ret at 0x92b3f9.
# - 0x92a140 WorldClubChamp::create_knockout_stage thiscall(this) ret 0. Prologue push -1; push 0x965eec (7 bytes).
#   Last ret at 0x92a6aa.
# - Calls: comp_util_get_latest_placed_club 0x4afd80 cdecl(comp, place); comp_util_sort_and_shuffle_top_clubs 0x4afbd0
#   cdecl(list, n, min(n, 4)); comp_util_is_league_division 0x4b0700 cdecl(id); key_nation_continent_has_selected_nation
#   0x6699d0 cdecl(continent record, 0xc6 bytes); utils_random 0x90cfc0; League_Stage 0x68b100 (23 arguments, the
#   pushes conferred at 0x92a21c); League::sort_table 0x6827d0 thiscall(stage, 0); the vtable's slot 0 (delete, 1);
#   comp_vtable::CREATE_SCHEDULE (this, 2, &rounds, &id, 0); Cup_Stage 0x522e00 (4, list, rounds, comp, table, year,
#   2, 2, id, 0x14, 0, 0, 0, 0); Cup::set_up_round 0x51c800(stage, 0); FinanceTable::get_club_finance 0x5a0590 and
#   Finance::add_prize_money 0x5999a0(2500000).
# Defects of the original (DEFEITOS): the host nation's random search has no reputation floor: a host with fewer than 6
# clubs loops forever (the 16-bit threshold wraps); the host's champion is not checked for 0; create_knockout_stage reads
# the fourth semi-finalist from the list after freeing it (the copy reads it before) and writes through a null pointer
# when the scratch League_Stage's new fails.
# Comparers: only the original (random draw; allocation and payments), with the plans (a club of the table; stage 2).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('world_club_champ_pick_entrant', 0x92ae60, [0xb8, 0x44, 0x21, 0x00, 0x00]),
           ('world_club_champ_create_knockout_stage', 0x92a140, [0x6a, 0xff, 0x68, 0xec, 0x5e, 0x96, 0x00])]
_F = 'world_club_champ.cpp'
DOCS = ('o Mundial de Clubes: um participante e a fase eliminatória',
        'the Club World Championship: one entrant and the knockout stage',
        [('world_club_champ_pick_entrant', 'WorldClubChamp::pick_entrant', '0x92ae60', _F, 'thiscall, ret 0x14',
          'regra: o participante de um continente é o campeão do seu torneio de clubes (o vice quando o campeão já entrou), se o continente tem uma nação selecionada ou é 2001; senão um clube do continente de reputação alta, sorteado entre os 6 ou mais de reputação 5000 para baixo, de 1500 em 1500 (até -2000). O da sede é o campeão da primeira divisão da nação (depois o 2º e o 3º), se a liga dela está selecionada ou é 2001; senão um clube sorteado da nação, do mesmo jeito mas sem piso (defeito do original: uma sede com menos de 6 clubes trava o jogo)',
          "rule: a continent's entrant is the winner of its club championship (the runner-up when the winner is already in), when the continent has a selected nation or it is 2001; otherwise a high-reputation club of the continent, drawn among the 6 or more with reputation from 5000 down, 1500 at a time (to -2000). The host's is the champion of the nation's first division (then the 2nd and 3rd), when its league is selected or it is 2001; otherwise a club of the nation drawn the same way but with no floor (defect of the original: a host with fewer than 6 clubs hangs the game)"),
         ('world_club_champ_create_knockout_stage', 'WorldClubChamp::create_knockout_stage', '0x92a140', _F, 'thiscall, ret 0',
          'regra: os três campeões de grupo e o melhor segundo (os três segundos ordenados numa tabela só deles) vão a uma fase eliminatória de 4; os terceiros, os quartos e os segundos que saem recebem 2.500.000 cada',
          'rule: the three group winners and the best runner-up (the three runners-up ranked in a table of their own) go to a four-club knockout stage; the thirds, the fourths and the runners-up that go out get 2,500,000 each')])
CAMPOS = []
INTERESSA = []
