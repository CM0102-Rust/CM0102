# The Asian Club Championship (asia_club_champ.cpp, in eurocomp\): the 32 qualifiers of the two zones, the west zone
# knockout and the predicted positions. Competitions front (reader cqx, 12/11/2026), 8-league career (Japan playable:
# the J1 clubs in the cup). Reuses cqu_10 (out on 07/11: the J1 runner-up mutation escaped; the other two were caught),
# rewritten in the current style over the Cup Winners' Cup helpers of the 285th batch (the same bodies:
# asia_cup_winner_zone_knockout_stage, asia_cup_winner_predicted_positions, checked instruction by instruction: only the
# source file and the box lines change).
# - 0x40b950 (AsiaClubChamp::select_qualified_clubs, 1 call: the constructor at the start of the new game);
# - 0x40bec0 (AsiaClubChamp::create_second_zone_cup_stage, 1 call);
# - 0x40c720 (AsiaClubChamp::rank_teams, 1 call).
# Conventions checked at the original's call sites (the constructor 0x40aae0, cqw_13): mov ecx, esi; call, no push ->
# thiscall, ret 0 (the three). Inside: 0x945501 (calloc: push 4; push 0x20; add esp, 8 -> cdecl), 0x4afd80 (push
# place; push comp; add esp, 8 -> cdecl), 0x4afbd0 (3 pushes; add esp, 0xc -> cdecl), the error box (show_error_at).
# Comparers: select_qualified_clubs and the stage, only the original (they allocate and draw) with the plan;
# rank_teams side by side: the original first (it adds the entries), then the copy over the same entries (it only
# rewrites the position byte), the 32 positions equal.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('asia_club_champ_select_qualified_clubs', 0x40b950, [0xb8, 0x94, 0x15, 0x00, 0x00]),
           ('asia_club_champ_create_second_zone_cup_stage', 0x40bec0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('asia_club_champ_rank_teams', 0x40c720, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Liga dos Campeões asiática: os 32 classificados das duas zonas, a fase da zona oeste e as posições previstas',
        'the Asian Club Championship: the 32 qualifiers of the two zones, the west zone stage and the predicted positions',
        [('asia_club_champ_select_qualified_clubs', 'AsiaClubChamp::select_qualified_clubs', '0x40b950', 'asia_club_champ.cpp', 'thiscall, ret 0',
          'regra: a Liga dos Campeões asiática tem 32 vagas em duas zonas de 16 (leste: Extremo Oriente e Sul da Ásia; oeste: Oriente Médio e Ásia Central); o último campeão entra na vaga 0 ou 16 pela zona da sua nação; o campeão da J1 (ou o vice, se o campeão já entrou) e o da liga coreana entram na disputa se as ligas estão selecionadas; depois os clubes asiáticos, ordenados pela reputação e embaralhados, dão uma vaga por nação na sua zona, enquanto houver vaga',
          "rule: the Asian Club Championship has 32 places in two zones of 16 (east: Far East and South Asia; west: Middle East and Central Asia); the last winner takes place 0 or 16 by its nation's zone; the J1 champion (or the runner-up, if the champion is already in) and the Korean league champion join the pool when those leagues are selected; then the Asian clubs, sorted by reputation and shuffled, give one place per nation in its zone while places remain"),
         ('asia_club_champ_create_second_zone_cup_stage', 'AsiaClubChamp::create_second_zone_cup_stage', '0x40bec0', 'asia_club_champ.cpp', 'thiscall, ret 0',
          'regra: a fase 0 da Liga dos Campeões asiática é um mata-mata com os 16 classificados da zona oeste (vagas 16 a 31), com as rodadas da tabela da copa',
          "rule: stage 0 of the Asian Club Championship is a knockout of the 16 west zone qualifiers (places 16 to 31), on the cup's round table"),
         ('asia_club_champ_rank_teams', 'AsiaClubChamp::rank_teams', '0x40c720', 'asia_club_champ.cpp', 'thiscall, ret 0',
          'regra: as posições previstas da Liga dos Campeões asiática: em cada zona, pela reputação, 1, 3, 5, 7, quatro 9 e oito 17',
          'rule: the Asian Club Championship predicted positions: in each zone, by reputation, 1, 3, 5, 7, four 9ths and eight 17ths')])
CAMPOS = [('copa +0xb2 a lista dos 32 classificados, +0xb6 quantos; ponteiro/valor; grava',
           'cup +0xb2 the list of the 32 qualifiers, +0xb6 the count; pointer/value; writes')]
INTERESSA = ['CMRTE']
