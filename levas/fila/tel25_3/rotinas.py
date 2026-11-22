# The printouts (printouts.cpp, part 1; reader tel25, the screens round, 21/11/2026): padding a text, the club
# transfers printout's order by date and a competition's printout (the Print popup of the competition screen,
# comp_screens_handle_comp_screen_click 0x4965d0, tel22_4). Uses tel26_1 (tel26_error_box) and tel22_1
# (comp_lang_format): those go first.
# Conventions checked in the disassembly:
# - 0x7da070 cdecl(text, width) -> eax; the error box line 0x640 (1600).
# - 0x7da130 cdecl(a, b) -> eax over the records themselves (+0x1d the date); 0x456770 Date::operator< and 0x456740
#   Date::operator> thiscall(lo, hi) ret 8.
# - 0x7d7c80 cdecl(comp, kind, file, print) -> eax: 0x669470 key_nation_index_of_comp cdecl -> al; 0x549ef0 Date::Date
#   thiscall(day, year) ret 8 (words); 0x417c10 Date::operator= thiscall(out, src) ret 8; 0x945b00 fopen, 0x945a09
#   fclose, 0x9464f0 fputs(text, file), 0x94700c fgets(buf, 0xc8, file), 0x94643a remove (cdecl); 0x54ba20
#   get_long_date_string thiscall(buf, 0xc8, 0xff) ret 0xc on [0xae2c90]; 0x66f4e0 langlib_sprintf(dest, fmt, 0x64,
#   name, date); 0x7d8120 printouts_write_league_tables cdecl(comp, file); 0x7d9720
#   printouts_write_comp_stat_table cdecl(comp, table, file); ScreenManager 0x8033e0 begin_print_job thiscall(0x8c)
#   ret 4, 0x803710 add_print_line thiscall(line) ret 4, 0x803930 add_print_page_break, 0x803a90 print_job, 0x8035b0
#   end_print_job thiscall ret 0. The error boxes: lines 0x3e8, 0x401 and 0x446.
# - The original passes the dates' words to Date::Date with garbage high halves (a stack slot, the entry pointer's
#   high word); Date::Date reads words only, the copy passes clean words.
# Defects of the original: none found.
MOTOR = True
TELAS = True
ROTINAS = [('printouts_pad_string', 0x7da070, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('printouts_compare_by_date', 0x7da130, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('printouts_print_comp_report', 0x7d7c80, [0x81, 0xec, 0xd8, 0x02, 0x00, 0x00])]
DOCS = ('as impressões (1): completar um texto, a ordem das transferências por data e a impressão de uma competição',
        'the printouts (1): padding a text, the transfer order by date and a competition\'s printout',
        [('printouts_pad_string', 'printouts_pad_string', '0x7da070', 'printouts.cpp', 'cdecl',
          'completa um texto com espaços até a largura, ou o corta nela; sem texto ou largura menor que 1, a caixa de erro 1600 e 0',
          'pads a text with spaces up to the width, or cuts it there; no text or a width below 1: error box 1600 and 0'),
         ('printouts_compare_by_date', 'printouts_compare_by_date', '0x7da130', 'printouts.cpp', 'cdecl',
          'ordena as transferências da impressão do clube pela data (+0x1d), da mais antiga para a mais nova',
          'sorts the club printout\'s transfers by date (+0x1d), earliest first'),
         ('printouts_print_comp_report', 'printouts_print_comp_report', '0x7d7c80', 'printouts.cpp', 'cdecl',
          'imprime uma competição: o período desde o fim da última temporada da nação, o cabeçalho, as tabelas (bit 1) e as estatísticas (bits 2, 4, 8, 0x10: tabelas 2, 5, 0x11, 0xc) num arquivo (por padrão ~printout.txt); com impressão, cada linha vai para a impressora (um espaço sozinho = quebra de página); o arquivo temporário é apagado',
          'prints a competition: the period since the nation\'s last season end, the heading, the tables (bit 1) and the statistics (bits 2, 4, 8, 0x10: tables 2, 5, 0x11, 0xc) into a file (by default ~printout.txt); when printing, each line goes to the printer (a lone space = page break); the temporary file is removed')])
