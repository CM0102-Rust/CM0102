# The FIFA Club World Championship (world_club_champ.cpp, in eurocomp\): the round tables, the rules and the day.
# Competitions front (reader cqx, 12/11/2026), 8-league career. The constructor (0x9291a2) puts the competition in the
# first even year from 2003 on, so the draw (5 February of that year) never comes in the test career: create_schedule
# and set_up_rules run once (the constructor, at the start of the new game) and update_for_date every day (2,064 calls
# to 2002-06-01), always on the early return. None of the three changes the career up to June 2002: the proof is the
# comparers (side by side), through the confirmation in comparison mode (mutacoes.py --confirmar 2001-08-20).
# - 0x929390 (WorldClubChamp::create_schedule): the generic engine of the national-team tournaments (build_phase_table,
#   GoldcupStagePlan; the operations taken one by one from the disassembly);
# - 0x929bb0 (WorldClubChamp::set_up_rules);
# - 0x92a6b0 (WorldClubChamp::update_for_date): the GoldCup::update_for_date mould (0x5e2120) with this cup's routines.
# Conventions checked at the original's call sites: create_schedule is the virtual +0x3c (push 0; push &extra; push
# &rounds; push stage; mov ecx -> thiscall, ret 0x10); set_up_rules from the constructor (0x92928d: mov ecx, esi; call
# -> thiscall, ret 0); update_for_date the virtual +0x14 (push full; push today; mov ecx -> thiscall, ret 8). Inside:
# 0x68a160 (9 pushes) and 0x68a1c0 (7 pushes), 0x521e60 (7) and 0x521eb0 (9), add esp after -> cdecl; 0x549f70 (4
# pushes; lea ecx, local -> thiscall, ret 0x10); 0x549ef0 (2 pushes; lea ecx -> thiscall, ret 8); 0x92a8b0, 0x929aa0,
# 0x929e60 (mov ecx, esi; call, no push -> thiscall, ret 0); 0x6835c0 (thiscall, ret 0), 0x6827d0 (push 0 ->
# thiscall, ret 4), 0x6847c0 (push full; push today -> thiscall, ret 8).
# Comparers: create_schedule side by side (compare_phase_table); set_up_rules side by side over the same object (the
# copy's table freed, the original's put back); update_for_date side by side before the draw date (only reads), only
# the original from it on.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('world_club_champ_create_schedule', 0x929390, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('world_club_champ_set_up_rules', 0x929bb0, [0x83, 0xec, 0x08, 0xb0, 0x02]),
           ('world_club_champ_update_for_date', 0x92a6b0, [0x6a, 0xff, 0x68, 0x0b, 0x5f, 0x96, 0x00])]
DOCS = ('o Mundial de Clubes da FIFA: as tabelas das rodadas, as regras e o dia',
        'the FIFA Club World Championship: the round tables, the rules and the day',
        [('world_club_champ_create_schedule', 'WorldClubChamp::create_schedule', '0x929390', 'world_club_champ.cpp', 'thiscall, ret 0x10',
          'regra: o Mundial de Clubes tem três grupos de três rodadas, em turno único, em 29 de julho, 2 e 5 de agosto (fases −1, 0 e 1), cada rodada com os dois jogos fixos do grupo; depois o mata-mata (fase 2) em jogo único: semifinais em 9 de agosto, final e terceiro lugar em 12 de agosto; categoria 0x17 (mundiais de clubes); prêmios de 6 e 5 milhões na final, 4 e 3 milhões no terceiro lugar',
          'rule: the Club World Championship has three groups of three rounds, single round robin, on 29 July, 2 and 5 August (stages −1, 0 and 1), each round with the group\'s two fixed matches; then the knockout (stage 2) in single matches: semi-finals on 9 August, final and third place on 12 August; category 0x17 (club world cups); prizes of 6 and 5 million in the final, 4 and 3 million for third place'),
         ('world_club_champ_set_up_rules', 'WorldClubChamp::set_up_rules', '0x929bb0', 'world_club_champ.cpp', 'thiscall, ret 0',
          'regra: os grupos do Mundial de Clubes são em turno único, 3 pontos a vitória e 1 o empate, categoria 0x17, 7 reservas e 3 substituições; o sorteio é em 5 de fevereiro do ano da competição (no dia da semana 5), o primeiro ano par a partir de 2003: nunca antes de junho de 2002',
          'rule: the Club World Championship groups are single round robin, 3 points a win and 1 a draw, category 0x17, 7 substitutes and 3 changes; the draw is on 5 February of the competition year (on weekday 5), the first even year from 2003 on: never before June 2002'),
         ('world_club_champ_update_for_date', 'WorldClubChamp::update_for_date', '0x92a6b0', 'world_club_champ.cpp', 'thiscall, ret 8',
          'regra: o dia do Mundial de Clubes: antes da data do sorteio, nada (nem o dia da liga); no dia do sorteio, uma vez, os participantes, as posições previstas, a tabela do primeiro grupo, os jogos, a ordem da tabela e as fases dos outros grupos; depois, o dia da liga',
          "rule: the Club World Championship's day: before the draw date nothing (not even the league day); on the draw day, once, the entrants, the predicted positions, the first group's table, the fixtures, the table order and the other groups' stages; then the league day")])
CAMPOS = [('liga +0xb6 o dia do sorteio, +0xb8 o ano relativo, +0xba a tabela das rodadas, +0x45 ativa, +0xee/+0xf2 a lista sorteada; valor/ponteiro; grava',
           'league +0xb6 the draw day, +0xb8 the relative year, +0xba the round table, +0x45 active, +0xee/+0xf2 the drawn list; value/pointer; writes')]
INTERESSA = ['CMRTE']
