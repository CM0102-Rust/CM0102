# The competition statistics texts: the team and player values, the record titles and the record texts:
# comp_screens.cpp (reader tel30b, the screens round, 21/11/2026; the four left out of tel30_11).
# Conventions checked in the disassembly (every push against esp):
# - 0x4a39c0 comp_stats_format_team_stat thiscall(ecx unused, entry, char code, char* out, int size, int with_detail)
#   ret 0x14 -> eax (called by 0x4956e0); jump table 0x4a3e94 (17 targets); 0x944c9f sprintf cdecl; 0x66f4e0
#   langlib_sprintf cdecl (the form letters, each into a 2-byte slot: W +0x12, X +0x16, D +0x14, L +0x10 of the frame;
#   the lea of the last three before their push). No x87.
# - 0x4a3ee0 Comp_Stats::format_player_stat thiscall(this, entry, char code, char* out, int size, int with_detail)
#   ret 0x14 -> eax (called by 0x495bc0, 0x7d9720); jump tables 0x4a47ec (the default text of 13..17) and 0x4a4800
#   (17 targets). x87: the club's games (0x4a4850 find_club_entry thiscall ret 4, 0x4a2ec0 team value stdcall ret 8 ->
#   st0, F32 0.0 at 0x968970 without a team; 17 keeps it as an f32 by fstp dword) x 35.0 (f32 0x968a7c) compared with
#   fild minutes by fcompp / test ah, 0x41 (games x 35 > minutes leaves the default); 13 / 16 fild x 100.0 (f32
#   0x968960) fidiv, then _ftol 0x945640 into "%d%%"; 14 / 15 fild / fidiv into a qword for "%3.1f"; 17 the games x
#   0.5 (f64 0x968968) against st0 of 0x4a3250 get_player_entry_value(this, entry, 1) thiscall ret 8, then fild /
#   fidiv into "%4.2f".
# - 0x4a5120 comp_stats_get_record_title thiscall(ecx unused, char kind, char* out, int size) ret 0xc; jump table
#   0x4a5464 (22 targets, checked against the strings); langlib_sprintf(out, title) only.
# - 0x4a5530 Comp_Stats::get_record_text thiscall(this, char view, char kind, Text*) ret 0xc -> eax of 0x4a5690
#   format_record_text thiscall(this, rec, kind dword, text) ret 0xc; 0x4afa40 Text::clear thiscall; 0x4ae660
#   Text::set_string thiscall(text, "-") ret 4; 0x945333 qsort(teams, count, 0x76, 0x4ad1a0) with the context at
#   0xadae10; 0x4a2ec0(first team, 2) -> st0 then _ftol; fld f32 +0x189, fcomp qword -1.0 (0x968948), test ah, 0x40
#   (equal or unordered) -> 0x4ad7f0 update_season_rating_record thiscall.
# Identical in cm0102-1080.exe (the whole routines).
# Depends on: tel26_1 (tel26_error_box), tel30_11 (comp_stats_label, comp_stats_club_games, the two value routines,
#   comp_stats_runs, COMP_STATS_SOURCE_FILE, COMP_STATS_F32_35).
# Comparers: the two formats and the title side by side over the buffer and the answer; get_record_text by
#   compare_text_writer (the copy into a local Text, after the original) and the answer. The error boxes the original
#   only.
# Defects of the original: none found (the form letters go into 2-byte slots: a translation longer than one letter
#   would run into the next slot).
MOTOR = True
TELAS = True
ROTINAS = [('comp_stats_format_team_stat', 0x4a39c0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('comp_stats_format_player_stat', 0x4a3ee0, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('comp_stats_get_record_title', 0x4a5120, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_stats_get_record_text', 0x4a5530, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
_A = 'comp_screens.cpp'
DOCS = ('os textos das estatísticas da competição: os valores de times e jogadores, os títulos e os textos dos recordes',
        'the competition statistics texts: the team and player values, the record titles and texts',
        [('comp_stats_format_team_stat', 'comp_stats_format_team_stat', '0x4a39c0', _A, 'thiscall',
          'o texto de uma estatística de time: "----" (2 a 5) ou "0" de saída; jogos, média de público (sem jogo fica "----"), a forma dos últimos cinco resultados em letras (V, X, E, D traduzidas), as sequências e as contagens; a 12 com o número entre parênteses quando pedido',
          'the text of a team statistic: "----" (2 to 5) or "0" to start; games, average attendance (with no game the "----" stays), the form of the last five results in letters (W, X, D, L translated), the runs and the counts; 12 with the bracketed number when asked'),
         ('comp_stats_format_player_stat', 'Comp_Stats::format_player_stat', '0x4a3ee0', _A, 'thiscall',
          'o texto de uma estatística de jogador: as contagens (jogos com as entradas do banco entre parênteses quando pedido) e as taxas (passes certos e chutes no alvo em %, desarmes e dribles por jogo com uma casa, nota média com duas) só para quem jogou 35 minutos por jogo do clube e os mínimos de cada uma; senão fica "---" ou "----"',
          "the text of a player statistic: the counts (appearances with the substitute ones in brackets when asked) and the rates (pass completion and shots on target in %, tackles and dribbles per game to one decimal, average rating to two) only for whoever played 35 minutes per club game and each one's minimums; otherwise \"---\" or \"----\" stays"),
         ('comp_stats_get_record_title', 'comp_stats_get_record_title', '0x4a5120', _A, 'thiscall',
          'o título do tipo de recorde (1 "Most Times Winner" a 22 "Oldest Player")', 'the title of the record type (1 "Most Times Winner" to 22 "Oldest Player")'),
         ('comp_stats_get_record_text', 'Comp_Stats::get_record_text', '0x4a5530', _A, 'thiscall',
          'o texto de um recorde (de sempre ou da temporada) num Text: "-" de saída; na temporada, a maior média de público ainda vazia sai do primeiro time da tabela ordenada por média, e a maior nota média ainda em -1 é recalculada, antes de formatar',
          "the text of a record (all-time or the season's) into a Text: \"-\" to start; for the season, the highest average attendance still empty comes from the first team of the table sorted by average, and the highest average rating still at -1 is recomputed, before formatting")])
