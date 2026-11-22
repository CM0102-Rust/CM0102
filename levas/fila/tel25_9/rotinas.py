# The printouts (printouts.cpp, part 5; reader tel25d, the screens round, 21/11/2026): a club's fixtures and results
# with the scorers, the writer of bit 8 of printouts_print_club_report (0x7d4460, tel25_8, which calls it by address).
# Uses tel25_7 (printout_write_heading/_titles/_row, printout_end_section, printout_column/_copy/_append,
# compare_printout_text), tel25_8 (printout_club_season, printout_short_date, printout_list_rewind), tel25_3
# (printout_text_length, PRINTOUT_LINE, PRINTOUT_PERIOD_*), tel26_1 (tel26_error_box) and tel22_1 (comp_lang_format,
# comp_crt_format): those go first.
# Conventions checked in the disassembly:
# - 0x7d69f0 cdecl(club, reserves, file) -> eax (called cdecl(club, reserves, file) by 0x7d4460); SEH frame, 0x13c8
#   of locals. Error box: line 0x2c2 (operator new failing). The pad_string calls are inlined (error path dead: the
#   buffer is a local).
# - 0x68d020 LLIST ctor / 0x68d040 dtor / 0x68d1b0 get_next thiscall on a local; the list rewound by copying +2 into
#   +0xa. 0x5acbe0 FIXTURE_MANAGER::find_fixtures thiscall([0xae2a58]; start day, start year, end day, end year
#   (words of 0xb74360/0xb74358, pushed with garbage high halves), club id ([club]), &list, -1, -2, reserves == 0
#   (sete al, garbage high bytes)) ret 0x24.
# - 0x944cf1 operator new(0xd7a); 0x9463af vector ctor iterator stdcall(obj+0x11f, 0x625, 2, 0x453c30, 0x453c10);
#   0x9462bb vector dtor iterator stdcall(obj+0x11f, 0x625, 2, 0x453c10); 0x944c94 operator delete cdecl. 0x453c30
#   only runs 0x54a110 (default date 0x28/0x7b4/1) on the 20 rows of 0x49 bytes at +0x71 (+0x14 each); 0x453c10 calls
#   an empty dtor (0x5a53c0): no global effect, nothing to restore between the original and the copy.
# - 0x549ef0 Date::Date thiscall(day = fixture +0x2a, year = +0x28) ret 8; 0x54a650 thiscall(date; buf, 0xc8, 0xff)
#   ret 0xc; 0x669630 cdecl(start year, club, buf, 0xc8); 0x53b900 database_is_national_team cdecl(club);
#   0x4afa80 cdecl(club, fixture) -> al; 0x4bf930 cdecl(fixture, 3, buf, 0xc8).
# - 0x729500 Match_Stats::fixture_entry_matches thiscall([0xb6b7c0]; slot +8, home id +0xc, away id +0x10, day +0x2a,
#   year +0x28) ret 0x14; 0x729600 Match_Stats::read_match_record thiscall([0xb6b7c0]; slot, obj) ret 8 (opens, reads
#   and closes match_stats_matches.tmp on each call: running twice is harmless).
# - 0x6db2d0 match_events_format_player_name cdecl(record, player byte, side byte, buf, 0xc8, 0, 0, 2): short form
#   off, so no random draw (garbage high bytes in the player and side pushes; the routine reads bytes).
# - 0x944c9f sprintf("%ld", attendance +0x34); 0x66f4e0 langlib_sprintf for the venue, result and scorer texts.
# - 0x9464f0 fprintf(file, text) cdecl, the text as the format.
# - 0x7d7b3e (an entry of the community map "in the middle") is the last inlined pad_string error box of this
#   routine, not a function: the whole 0x7d69f0..0x7d7c7e is hooked here.
# Defects of the original: none proven. Latent: the scorers buffer is 0x100 bytes on the stack, right before the
# name buffer; a scorers text of up to 50 characters plus ", " plus one entry over ~200 characters would run into the
# name (impossible with real names); the copy has room for it.
MOTOR = True
TELAS = True
ROTINAS = [('printouts_write_club_fixtures', 0x7d69f0, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as impressões (5): os jogos e resultados do clube, com os artilheiros',
        'the printouts (5): a club\'s fixtures and results, with the scorers',
        [('printouts_write_club_fixtures', 'printouts_write_club_fixtures', '0x7d69f0', 'printouts.cpp', 'cdecl',
          'escreve no arquivo da impressão os jogos do clube no período da impressão (find_fixtures pelo id do clube): por jogo, a data curta (8), o adversário (20), as três letras da nação dele quando o clube não é seleção e as nações diferem (3), o mando H, A ou N (3), a rodada da competição (30), o placar do lado do clube (depois da prorrogação quando houve; "P-P" adiado, "---" não jogado; 5); jogo disputado com o registro de estatísticas lido (fixture_entry_matches e read_match_record num objeto de partida de 0xd7a bytes montado com os dois times de 0x625) ganha o público (6) e os artilheiros do lado do clube (gols e pênaltis do lado dele, gols contra do outro, agrupados por jogador: "nome", "nome pen", "nome n", "nome n (1 pen)", "nome n pens", "nome n (p pens)", "nome og", "nome n ogs !", separados por vírgula), quebrando a linha quando passa de 50 caracteres, com recuo até a coluna; os outros jogos, "-----"; sem memória para o objeto, a caixa de erro 706 e 0',
          'writes into the printout file the club\'s fixtures over the printout\'s period (find_fixtures by the club\'s id): for each, the short date (8), the opposition (20), its nation\'s three letters when the club is not a national team and the nations differ (3), the venue H, A or N (3), the competition round (30), the score from the club\'s side (after extra time when there was some; "P-P" postponed, "---" not played; 5); a played match whose statistics record reads (fixture_entry_matches and read_match_record into a 0xd7a-byte match object built with its two 0x625 team blocks) gets the attendance (6) and the scorers of the club\'s side (its goals and penalties, the other side\'s own goals, grouped by player: "name", "name pen", "name n", "name n (1 pen)", "name n pens", "name n (p pens)", "name og", "name n ogs !", comma separated), breaking the line past 50 characters with an indent up to the column; the other fixtures, "-----"; without memory for the object, the 706 error box and 0')])
