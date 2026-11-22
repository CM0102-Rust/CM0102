# The printouts (printouts.cpp, part 3; reader tel25c, the screens round, 21/11/2026): a competition's statistics
# table and its league tables, the two writers of printouts_print_comp_report (0x7d7c80, tel25_3), and the shared
# helpers of every printout writer (fprintf, columns, the side-by-side comparer over the text). Uses tel26_1
# (tel26_error_box), tel22_1 (comp_lang_format, comp_crt_format) and tel25_3 (printouts_pad_string, PRINTOUT_LINE,
# PRINTOUT_NEWLINE, printout_text_length...): those go first; tel25_8 uses this one.
# Conventions checked in the disassembly:
# - 0x7d9720 cdecl(comp, table, file) -> eax (called as cdecl(comp, table, file) by 0x7d7c80); the table compared as a
#   byte, passed whole to the Comp_Stats methods. Error boxes: lines 0x5b6 and 0x5be.
# - 0x7d8120 cdecl(comp, file) -> eax; error box line 0x483.
# - 0x9464f0 is fprintf(file, format, ...) (cdecl), not fputs: the original passes each text as the format.
# - 0x6697c0 cdecl(year, comp, buf, 0xc8); 0x4a2b10 thiscall(stats; stat, buf, size) ret 0xc; 0x4a4dc0
#   thiscall(stats; stat, &count, 0x1e, -1) ret 0x10 -> malloc'd 0x2d-byte rows (staff id at +4), freed with 0x9452ca;
#   0x4a3ee0 thiscall(stats; row, stat, buf, size, 1) ret 0x14; 0x90c310 cdecl(n, -1) -> text; 0x53dba0
#   cdecl(staff, buf, 0xc8, style); 0x48d200 Comp::get_stage, 0x6827d0 League::sort_table(stage, 0) and 0x6826d0
#   League::get_table_row are Rust already (called directly); 0x4b4850 cdecl(out, 0xc8, word, 0) (the original
#   pushes the word with a garbage high half); 0x549ef0 Date::Date thiscall(day, year) ret 8.
# - The pad_string calls are inlined in both (the error path dead: the buffer is a local).
# Defects of the original: none found here. Found in tel25_3 (reported, not changed here): its printout_put calls
# 0x9464f0 as (text, file), but 0x9464f0 is fprintf(file, format): the copy of print_comp_report would pass a text
# as the FILE*.
MOTOR = True
TELAS = True
ROTINAS = [('printouts_write_comp_stat_table', 0x7d9720, [0x81, 0xec, 0xe0, 0x0a, 0x00, 0x00]),
           ('printouts_write_league_tables', 0x7d8120, [0xb8, 0xe4, 0x20, 0x00, 0x00])]
DOCS = ('as impressões (3): a tabela de estatísticas de uma competição e as tabelas de classificação',
        'the printouts (3): a competition\'s statistics table and its league tables',
        [('printouts_write_comp_stat_table', 'printouts_write_comp_stat_table', '0x7d9720', 'printouts.cpp', 'cdecl',
          'escreve no arquivo da impressão os 30 primeiros jogadores de uma estatística da competição: cabeçalho com a temporada e o nome da estatística, colunas Pos, Jogador, Clube (ou Nação nas competições de seleções, tipo 4), Jogos e a estatística (Gls, Asts, Av R, MoM ou ???); sem linhas, "No stats available"; sem o objeto da competição ou sem as estatísticas, as caixas de erro 1462 e 1470 e 0',
          'writes into the printout file the leading 30 players of a competition statistic: a heading with the season and the statistic\'s name, the columns Pos, Player, Club (or Nation in national-team competitions, kind 4), Apps and the statistic (Gls, Asts, Av R, MoM or ???); without rows, "No stats available"; without the competition object or its statistics, the 1462 and 1470 error boxes and 0'),
         ('printouts_write_league_tables', 'printouts_write_league_tables', '0x7d8120', 'printouts.cpp', 'cdecl',
          'escreve as tabelas de classificação de uma competição, uma por fase de liga (com o nome da fase quando há mais de uma): ordena a tabela antes (sort_table), marca campeão, classificados, promovidos, repescagem e rebaixados, traça uma linha onde as vagas mudam e mostra jogos, vitórias, empates (ou pênaltis, ou shoot-out), derrotas, gols pró e contra em casa e fora e os pontos; fase sem sorteio ainda, "Draw not yet made"; sem o objeto da competição, a caixa de erro 1155 e 0',
          'writes a competition\'s league tables, one per league stage (with the stage\'s name when there are several): sorts the table first (sort_table), marks the champions, the qualified, the promoted, the play-offs and the relegated, draws a line where the places change and shows played, won, drawn (or penalties, or shoot-outs), lost, for and against at home and away and the points; a stage not drawn yet, "Draw not yet made"; without the competition object, the 1155 error box and 0')])
