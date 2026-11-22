# The competition statistics: the carried runs, the match totals, the labels and the values: comp_screens.cpp (reader
# tel30, the screens round, 21/11/2026; a new group from the disassembly sweep).
# Conventions checked in the disassembly (every push against esp):
# - 0x4a2090 Comp_Stats::save_team_runs thiscall(this) ret 0 (called by archive_season_and_reset 0x4a1c50); 0x944e46
#   malloc; 0x9452ca free; 0x945333 qsort cdecl(base, n, 0x1c, 0x8b4e00); the error box like tel26_error_box with
#   comp_stats.CPP (0x99ca94) line 0x5f7 (no exit).
# - 0x4a22f0 Comp_Stats::restore_team_runs thiscall(this, team) ret 4 (called by add_club_entry 0x4aceb0): 0x946464
#   bsearch cdecl(&key, base, n, 0x1c, 0x8b4e00); 0x549ef0 Date::Date thiscall(date, day, year) ret 8; 0x417c10
#   Date::operator= thiscall(dst, scratch, src) ret 8.
# - 0x4a25f0 Comp_Stats::accumulate_match_player_totals thiscall(this, fixture) ret 4 -> eax (called by
#   record_match_result 0x49f570, every match).
# - 0x4a2810 / 0x4a2b10 cdecl(char code, char* out, int size) -> eax; jump tables 0x4a2ac8 / 0x4a2dc8 (17 targets,
#   checked against the strings); 0x66f4e0 langlib_sprintf.
# - 0x4a2ec0 comp_stats_get_team_entry_value stdcall(entry, char code) ret 8 -> st0 (fild / fidiv; 0.0 from the f32 at
#   0x968970); jump table 0x4a3150 (19 targets).
# - 0x4a3250 Comp_Stats::get_player_entry_value thiscall(this, entry, char code) ret 8 -> st0; jump table 0x4a3970
#   (18 targets); 0x4a4850 Comp_Stats::find_club_entry thiscall(this, club) ret 4; 0x4a2ec0 called with ecx = this
#   (unused); fmul dword 35.0 (0x968a7c) / 100.0 (0x968960), fmul qword 0.5 (0x968968), fcompp with test ah, 0x41.
# - 0x4a5010 cdecl(char code, char* out, int size) -> eax; 0x4a54c0 Comp_Stats::is_record_type_shown thiscall(this,
#   char kind) ret 4: 0x5c6710 Friendly::is_friendly_competition thiscall([0xae2c74], comp) ret 4, 0x4b0700
#   comp_util_is_league_division cdecl(id).
# Called in the simulation too: save/restore (the season turnover), accumulate (every match), the team value (by
#   0x4a19f0 and the Comp_Stats tables): the chain's careers and its comparison run them.
# Identical in cm0102-1080.exe (the whole routines).
# Depends on: tel26_1 (tel26_error_box).
# Left for a later round (not in this item, still original): comp_stats_format_team_stat 0x4a39c0 (1304 bytes),
#   Comp_Stats::format_player_stat 0x4a3ee0 (2404), comp_stats_get_record_title 0x4a5120 (924) and
#   Comp_Stats::get_record_text 0x4a5530 (340: it sorts the team table and calls 0x4ad7f0 and 0x4a5690).
# Comparers: accumulate side by side over the totals; the labels side by side over the buffer and the answer; the two
#   value routines by the value's bits; is_record_type_shown by value; save and restore the original only (they
#   allocate / free) with the plan (one sorted saved run per team; the four run lengths from the club's saved run). The
#   error boxes the original only.
# Defects of the original: none found.
MOTOR = True
TELAS = True
ROTINAS = [('comp_stats_save_team_runs', 0x4a2090, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_stats_restore_team_runs', 0x4a22f0, [0x83, 0xec, 0x2c, 0x53, 0x8b, 0xd9]),
           ('comp_stats_accumulate_match_player_totals', 0x4a25f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_stats_get_team_stat_label', 0x4a2810, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_stats_get_player_stat_label', 0x4a2b10, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_stats_get_team_entry_value', 0x4a2ec0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('comp_stats_get_player_entry_value', 0x4a3250, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('comp_stats_get_record_period_label', 0x4a5010, [0x8b, 0x4c, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_stats_is_record_type_shown', 0x4a54c0, [0x56, 0x8b, 0xf1, 0x8b, 0x86, 0x02, 0x05, 0x00, 0x00])]
_A = 'comp_screens.cpp'
DOCS = ('as estatísticas da competição: as sequências guardadas, os totais da partida, os nomes e os valores das estatísticas',
        'the competition statistics: the saved runs, the match totals, the statistic names and values',
        [('comp_stats_save_team_runs', 'Comp_Stats::save_team_runs', '0x4a2090', _A, 'thiscall',
          'guarda as quatro sequências de cada time antes de a tabela de times sair: um registro de 0x1c bytes por time (o id do clube e as doze palavras das sequências), ordenado pelo id; sem memória, a caixa de erro',
          "saves every team's four runs before the team table goes: one 0x1c-byte record per team (the club id and the twelve words of the runs), sorted by id; out of memory, the error box"),
         ('comp_stats_restore_team_runs', 'Comp_Stats::restore_team_runs', '0x4a22f0', _A, 'thiscall',
          'devolve a um time novo as sequências guardadas do seu clube (o tamanho e a data de início de cada uma, a data atual como a última), e solta o arranjo quando o último é devolvido',
          "gives a new team back its club's saved runs (each one's length and start date, the current date as the latest), and frees the array when the last one is given back"),
         ('comp_stats_accumulate_match_player_totals', 'Comp_Stats::accumulate_match_player_totals', '0x4a25f0', _A, 'thiscall',
          'soma os números dos jogadores de uma partida jogada aos totais da competição (os dois lados, 20 linhas cada, os reservas só quando entraram); partida já contada dá 0',
          "adds a played match's player figures to the competition totals (both sides, 20 rows each, the substitutes only when they came on); a match already counted gives 0"),
         ('comp_stats_get_team_stat_label', 'comp_stats_get_team_stat_label', '0x4a2810', _A, 'cdecl',
          'o nome da estatística de time (1 "Games" a 17 "Red Cards")', 'the name of the team statistic (1 "Games" to 17 "Red Cards")'),
         ('comp_stats_get_player_stat_label', 'comp_stats_get_player_stat_label', '0x4a2b10', _A, 'cdecl',
          'o nome da estatística de jogador (1 "Appearances" a 17 "Average Rating")', 'the name of the player statistic (1 "Appearances" to 17 "Average Rating")'),
         ('comp_stats_get_team_entry_value', 'comp_stats_get_team_entry_value', '0x4a2ec0', _A, 'stdcall',
          'o valor de uma estatística de time: jogos, média de público, as três formas, as sequências e as contagens',
          'the value of a team statistic: games, average attendance, the three forms, the runs and the counts'),
         ('comp_stats_get_player_entry_value', 'Comp_Stats::get_player_entry_value', '0x4a3250', _A, 'thiscall',
          'o valor de uma estatística de jogador: as contagens, e as taxas (passes, desarmes e dribles por jogo, chutes no alvo, nota média) só para quem jogou o bastante (minutos de 35 × os jogos do clube e os mínimos de cada uma)',
          "the value of a player statistic: the counts, and the rates (passes, tackles and dribbles per game, shots on target, average rating) only for whoever played enough (minutes of 35 × the club's games and each one's minimums)"),
         ('comp_stats_get_record_period_label', 'comp_stats_get_record_period_label', '0x4a5010', _A, 'cdecl',
          'o nome do período dos recordes (1 "All Time", 2 "Current")', 'the name of the records period (1 "All Time", 2 "Current")'),
         ('comp_stats_is_record_type_shown', 'Comp_Stats::is_record_type_shown', '0x4a54c0', _A, 'thiscall',
          'se um tipo de recorde aparece: nunca num amistoso; o tipo 2 só em divisão de liga; o tipo 8 não em competição do tipo 4',
          'whether a record type is shown: never for a friendly; type 2 only for a league division; type 8 not for a competition of kind 4')])
