# The club history screen's tabs (club_screens.cpp, part 3; reader tel03, the screens round, 21/11/2026): the
# Competitions tab and the six season tabs.
# - 0x46adc0 club_history_draw_competitions_tab cdecl(top, bottom) ret 0, SEH, 0x818 bytes of locals (the view label
#   and a Text); 4717 bytes (the map's 1497 + the rest to 0x46c02d; 0x46bfc0 is not an entry, a false reference).
#   Split into named pieces: history_competitions_menu_list, history_competitions_league_table,
#   history_competitions_cup_runs, history_competitions_honours; the whole routine is hooked. Calls: 0x443380
#   get_league_history, 0x443770 get_league_comp_ids, 0x443520 get_cup_history, 0x443a20 get_cup_comp_ids (thiscall on
#   [0xad9da8], ret 4); 0x68d1b0 LLIST::get_next, 0x68d2e0 get_current; 0x55f990 Display::add_item thiscall (18 args)
#   ret 0x48; 0x4b0700 comp_util_is_league_division (called twice on a cup, as in the original); 0x669630
#   key_nation_format_club_season cdecl(ushort, club, buf, size); the competition object's virtual method +0x38
#   thiscall(buf, size, round, stage, flag, club) (the original pushes the round, the stage and the flag in the club
#   pointer's register: its high bytes go along, kept here); 0x90c310 utils_format_ordinal cdecl; 0x4c0140
#   comp_util_collect_club_honours cdecl(club, other, &array) returns the count; 0x5409b0 club_is_national_team,
#   0x53b3d0 database_get_national_team; 0x4afa40 Text::clear, 0x4ae660 set_string; 0x9452ca free; 0x403320
#   Area::set_scroll_row (inline Display::set_area_scroll_row).
# - 0x46c030/0x46c510/0x46c9e0/0x46ceb0/0x46d370/0x46d860 club_history_draw_{positions,attendances,results,sequences,
#   players,transfers}_tab cdecl(top, bottom) ret 0, SEH, 0x7f8/0x7f4 bytes of locals: one template of the original
#   (category 4..9, view param 7/0xa/0xd/0x10/0x13/0x16, scroll and area params after it; the period: param 2 for
#   positions, 2 for players, 1 for the rest; the default when the stored type does not apply: 3 for attendances, 7 for
#   results, the first that applies for players), here one helper (history_season_tab) with each tab's parameters.
#   Calls: 0x483920 club_record_type_to_category cdecl(type, 0), 0x449590 applies, 0x4490b0 type label, 0x450d50
#   Club_Records::build_history_list thiscall(club, &count) ret 8 (kept in params 5 and 6), 0x449810
#   club_records_format_record_text thiscall(club, record, type, Text*) ret 0x10.
# Kept from the original: the view label stays as stack garbage when no entry matches the view (empty here); the
#   colour words and the record type, period and row count go in dword slots whose high bytes are register or stack
#   garbage in the original (0 here: the callees read the low byte/word).
# Comparers: side by side by the display list (compare_match_screen_build); original only while the tab still builds
#   the lists or the array it keeps in the params (0x19..0x1c; 5).
# How to reach them: a club's History > Competitions (View: Honours, Domestic Leagues, a league, a cup; Germany's
#   national team for the West Germany honours), Positions, Attendances, Results, Sequences, Players, Transfers (each
#   with its View popup; a greyed type does not apply to the club).
TELAS = True
ROTINAS = [('club_history_draw_competitions_tab', 0x46adc0, [0x6a, 0xff, 0x68, 0xfb, 0x5b, 0x95, 0x00]),
           ('club_history_draw_positions_tab', 0x46c030, [0x6a, 0xff, 0x68, 0x1b, 0x5c, 0x95, 0x00]),
           ('club_history_draw_attendances_tab', 0x46c510, [0x6a, 0xff, 0x68, 0x3b, 0x5c, 0x95, 0x00]),
           ('club_history_draw_results_tab', 0x46c9e0, [0x6a, 0xff, 0x68, 0x5b, 0x5c, 0x95, 0x00]),
           ('club_history_draw_sequences_tab', 0x46ceb0, [0x6a, 0xff, 0x68, 0x7b, 0x5c, 0x95, 0x00]),
           ('club_history_draw_players_tab', 0x46d370, [0x6a, 0xff, 0x68, 0x9b, 0x5c, 0x95, 0x00]),
           ('club_history_draw_transfers_tab', 0x46d860, [0x6a, 0xff, 0x68, 0xbb, 0x5c, 0x95, 0x00])]
_TEMPORADA_PT = ('as categorias de recorde {c} no popup View (apagadas as que não valem para o clube no período), o tipo escolhido como título e uma linha por temporada do vetor de recordes do clube (montado uma vez e guardado nos parâmetros 5 e 6): a temporada e o recorde daquele tipo')
_TEMPORADA_EN = ('the record types of category {c} in the View popup (greyed out when they do not apply to the club in the period), the chosen type as the heading and one row per season of the club\'s record array (built once and kept in params 5 and 6): the season and the record of that type')
DOCS = ('as abas da história do clube: competições e as abas por temporada',
        'the club history tabs: competitions and the season tabs',
        [('club_history_draw_competitions_tab', 'club_history_draw_competitions_tab', '0x46adc0', 'club_screens.cpp', 'cdecl, SEH',
          'Desenha a aba Competições: o popup View com Honrarias, Ligas Nacionais e uma entrada por liga e por copa que o clube disputou (as listas guardadas nos parâmetros 0x19..0x1c), a vista escolhida (0x1d) como título (ligado à competição quando carregada); depois a tabela das ligas (todas ou uma divisão: temporada, posição, liga e os sete números), as campanhas numa copa (temporada, dois anos à frente nas eliminatórias da Copa e da Eurocopa, e a fase alcançada) ou as honrarias (cada primeiro, segundo ou terceiro lugar, com os da Alemanha Ocidental para a seleção alemã: competição, lugar e anos, até 8 por linha); a rolagem volta do parâmetro 0x1e e a área vai ao 0x1f.',
          'Draws the Competitions tab: the View popup with Honours, Domestic Leagues and one entry per league and per cup the club played (the lists kept in params 0x19..0x1c), the chosen view (0x1d) as the heading (linked to the competition when loaded); then the league table (all or one division: season, position, league and the seven numbers), the runs in a cup (season, two years on in the World Cup and European Championship qualifying, and the round reached) or the honours (every first, second or third place, with West Germany\'s for Germany\'s national team: competition, place and years, up to 8 a row); the scroll row comes back from param 0x1e and the area goes to 0x1f.'),
         ('club_history_draw_positions_tab', 'club_history_draw_positions_tab', '0x46c030', 'club_screens.cpp', 'cdecl, SEH',
          'Desenha a aba Posições: ' + _TEMPORADA_PT.format(c=4) + '; no período da aba Recordes (parâmetro 2), vista no parâmetro 7.',
          'Draws the Positions tab: ' + _TEMPORADA_EN.format(c=4) + '; in the Records tab\'s period (param 2), view in param 7.'),
         ('club_history_draw_attendances_tab', 'club_history_draw_attendances_tab', '0x46c510', 'club_screens.cpp', 'cdecl, SEH',
          'Desenha a aba Públicos: ' + _TEMPORADA_PT.format(c=5) + '; período 1, vista no parâmetro 0xa (o tipo 3 quando o guardado não vale).',
          'Draws the Attendances tab: ' + _TEMPORADA_EN.format(c=5) + '; period 1, view in param 0xa (type 3 when the stored one does not apply).'),
         ('club_history_draw_results_tab', 'club_history_draw_results_tab', '0x46c9e0', 'club_screens.cpp', 'cdecl, SEH',
          'Desenha a aba Resultados: ' + _TEMPORADA_PT.format(c=6) + '; período 1, vista no parâmetro 0xd (o tipo 7 quando o guardado não vale).',
          'Draws the Results tab: ' + _TEMPORADA_EN.format(c=6) + '; period 1, view in param 0xd (type 7 when the stored one does not apply).'),
         ('club_history_draw_sequences_tab', 'club_history_draw_sequences_tab', '0x46ceb0', 'club_screens.cpp', 'cdecl, SEH',
          'Desenha a aba Sequências: ' + _TEMPORADA_PT.format(c=7) + '; período 1, vista no parâmetro 0x10.',
          'Draws the Sequences tab: ' + _TEMPORADA_EN.format(c=7) + '; period 1, view in param 0x10.'),
         ('club_history_draw_players_tab', 'club_history_draw_players_tab', '0x46d370', 'club_screens.cpp', 'cdecl, SEH',
          'Desenha a aba Jogadores: ' + _TEMPORADA_PT.format(c=8) + '; período 2, vista no parâmetro 0x13 (o primeiro tipo que vale quando o guardado não vale).',
          'Draws the Players tab: ' + _TEMPORADA_EN.format(c=8) + '; period 2, view in param 0x13 (the first type that applies when the stored one does not).'),
         ('club_history_draw_transfers_tab', 'club_history_draw_transfers_tab', '0x46d860', 'club_screens.cpp', 'cdecl, SEH',
          'Desenha a aba Transferências: ' + _TEMPORADA_PT.format(c=9) + '; período 1, vista no parâmetro 0x16.',
          'Draws the Transfers tab: ' + _TEMPORADA_EN.format(c=9) + '; period 1, view in param 0x16.')])
