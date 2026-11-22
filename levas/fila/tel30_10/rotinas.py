# The competition screen opener and the season records: comp_screens.cpp (reader tel30, the screens round,
# 21/11/2026; a new group from the disassembly sweep).
# Conventions checked in the disassembly (every push against esp):
# - 0x48f4a0 comp_screens_open_comp_screen cdecl(int* comp id, second, third, restore) ret 0 (called by 0x4965d0,
#   0x49eac0, 0x7643d0); the competition from [0xadadfc][id]; 0x7fe600 open_screen(0x48fcd0, 0x4965d0, 0, 0x4985e0, 0)
#   ret 0x14; 0x7ff010 / 0x7ff270 get_ / set_screen_param on 0xb74450.
# - 0x49f160 Comp_Season_Records::Comp_Season_Records thiscall(this) -> this, SEH: 0x54a110 Date::Date, 0x448670
#   run_record_construct, 0x450d40 record_holder_construct, 0x645800 FixedFixture::FixedFixture (all thiscall(this)),
#   0x49f390 / 0x49f3f0 (this item).
# - 0x49f2a0 / 0x49f2f0 / 0x49f340 / 0x49f390 / 0x49f3f0 thiscall(this) -> this, SEH (Date::Date on their dates).
# - 0x4a1f40 / 0x4a1f70 / 0x4a2050 thiscall(this, src) ret 4 -> this; 0x448490 record_holder_assign thiscall(dst,
#   src) ret 4.
# Called in the simulation too: the records constructors by Comp_Stats::Comp_Stats (0x49ee70), Comp_Season_Records also
#   by archive_season_and_reset (0x4a1c50) and the age records by 0x4a7f50; the assigns by 0x4a1c50 and 0x4ab0c0. The
#   chain's careers and its comparison run them (a TELAS batch still runs the careers).
# Identical in cm0102-1080.exe (the whole routines).
# Depends on: tel29_1 (current_screen_node).
# Comparers: the records and the assigns side by side over the object they build or copy (with the return value);
#   the opener the original only with the plan (the competition screen current, the competition and the caller's two
#   parameters, 0x2d = 1, 0x36 = -1).
# Defects of the original: none found.
MOTOR = True
TELAS = True
ROTINAS = [('comp_screens_open_comp_screen', 0x48f4a0, [0x81, 0xec, 0xf0, 0x00, 0x00, 0x00]),
           ('comp_season_records_comp_season_records', 0x49f160, [0x6a, 0xff, 0x68, 0xd4, 0x65, 0x95, 0x00]),
           ('comp_unbeaten_run_record_construct', 0x49f2a0, [0x6a, 0xff, 0x68, 0xeb, 0x65, 0x95, 0x00]),
           ('comp_winless_run_record_construct', 0x49f2f0, [0x6a, 0xff, 0x68, 0x0b, 0x66, 0x95, 0x00]),
           ('comp_goals_in_match_record_construct', 0x49f340, [0x6a, 0xff, 0x68, 0x2b, 0x66, 0x95, 0x00]),
           ('comp_youngest_player_record_construct', 0x49f390, [0x6a, 0xff, 0x68, 0x53, 0x66, 0x95, 0x00]),
           ('comp_oldest_player_record_construct', 0x49f3f0, [0x6a, 0xff, 0x68, 0x73, 0x66, 0x95, 0x00]),
           ('comp_tally_record_assign', 0x4a1f40, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('comp_discipline_record_assign', 0x4a1f70, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('comp_rating_record_assign', 0x4a2050, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c])]
_A = 'comp_screens.cpp'
DOCS = ('a abertura da tela da competição e os recordes da temporada',
        'the competition screen opener and the season records',
        [('comp_screens_open_comp_screen', 'comp_screens_open_comp_screen', '0x48f4a0', _A, 'cdecl',
          'abre a tela da competição: o parâmetro 0 recebe a competição, o 2 e o 3 os dados, e os outros os valores iniciais (aba 1, visão 0x11, ...); ao restaurar, os parâmetros 1, 0x1e e 0x23 da tela anterior voltam',
          'opens the competition screen: parameter 0 takes the competition, 2 and 3 the given values and the rest their defaults (tab 1, view 0x11, ...); when restoring, parameters 1, 0x1e and 0x23 of the previous screen come back'),
         ('comp_season_records_comp_season_records', 'Comp_Season_Records::Comp_Season_Records', '0x49f160', _A, 'thiscall',
          'constrói os recordes da temporada de uma competição: as datas, os registros de sequência, os detentores de recorde, a partida fixa e os registros do jogador mais novo e mais velho',
          "builds a competition's season records: the dates, the run records, the record holders, the fixed fixture and the youngest and oldest player records"),
         ('comp_unbeaten_run_record_construct', 'comp_unbeaten_run_record_construct', '0x49f2a0', _A, 'thiscall',
          'constrói um registro de invencibilidade (as duas datas)', 'builds an unbeaten run record (its two dates)'),
         ('comp_winless_run_record_construct', 'comp_winless_run_record_construct', '0x49f2f0', _A, 'thiscall',
          'constrói um registro de sequência sem vitória (as duas datas)', 'builds a winless run record (its two dates)'),
         ('comp_goals_in_match_record_construct', 'comp_goals_in_match_record_construct', '0x49f340', _A, 'thiscall',
          'constrói um registro de gols numa partida (as duas datas)', 'builds a goals in a match record (its two dates)'),
         ('comp_youngest_player_record_construct', 'comp_youngest_player_record_construct', '0x49f390', _A, 'thiscall',
          'constrói o registro do jogador mais novo (as três datas)', 'builds the youngest player record (its three dates)'),
         ('comp_oldest_player_record_construct', 'comp_oldest_player_record_construct', '0x49f3f0', _A, 'thiscall',
          'constrói o registro do jogador mais velho (as três datas)', 'builds the oldest player record (its three dates)'),
         ('comp_tally_record_assign', 'comp_tally_record_assign', '0x4a1f40', _A, 'thiscall',
          'copia um registro de contagem (o byte, o detentor, o dword e a palavra)', 'copies a tally record (the byte, the holder, the dword and the word)'),
         ('comp_discipline_record_assign', 'comp_discipline_record_assign', '0x4a1f70', _A, 'thiscall',
          'copia um registro de disciplina (os dois bytes, o detentor, o dword e a palavra)', 'copies a discipline record (the two bytes, the holder, the dword and the word)'),
         ('comp_rating_record_assign', 'comp_rating_record_assign', '0x4a2050', _A, 'thiscall',
          'copia um registro de nota (o dword, o detentor, o byte, o dword e a palavra)', 'copies a rating record (the dword, the holder, the byte, the dword and the word)')])
