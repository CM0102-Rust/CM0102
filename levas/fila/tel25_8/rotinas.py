# The printouts (printouts.cpp, part 4; reader tel25c, the screens round, 21/11/2026): a club's printout (the Print of
# the club screen, club_screen_input 0x4671f0, still original) with its squad section, the squad statistics and the
# transfers. Uses tel25_7 (the printout helpers and compare_printout_text), tel25_3, tel26_1 and tel22_1: those go
# first. Left out: 0x7d69f0 printouts_write_club_fixtures (see PRONTO.txt), still original, called by address.
# Conventions checked in the disassembly:
# - 0x7d60c0 and 0x7d5280 cdecl(club, reserves, file) -> eax (the reserves argument unused); 0x7d4460 cdecl(club,
#   reserves, kind, file, print) -> eax (kind tested as a byte: 1 squad, 2 0x7d5280, 4 0x7d60c0, 8 0x7d69f0, each
#   called cdecl(club, reserves, file)). Error boxes: 0x58, 0x71, 0xaf.
# - 0x8cc030 TRANSFER_MANAGER::get_past_transfers thiscall(start month, start year, end month, end year, &array,
#   &count) ret 0x18 (the months from 0x54cfb0 Date::get_month, al; the years as words, the original pushing garbage
#   high halves); records of 0x25 bytes; qsort 0x945333(array, count, 0x25, 0x7da130); free 0x9452ca.
# - 0x669630 cdecl(year, club, buf, 0xc8); 0x54a650 thiscall(date; buf, 0xc8, 0xff) ret 0xc; 0x53e140 cdecl(first,
#   second, common, buf, 0xc8, 1, staff); 0x43f460 thiscall(&cash; 5) ret 4 -> edx:eax; 0x441c10 cdecl(buf, size,
#   i64, 5); 0x450aa0 thiscall(&cash; buf, 0xc8, 5) ret 0xc.
# - 0x7c14b0 thiscall(g_player_stats; scope, buf, 0x7d0) ret 0xc; 0x7c25b0 thiscall(g_player_stats; player, scope,
#   stat, buf, 0xc8, a, b, c) ret 0x20 (the three options per statistic in PRINTOUT_SQUAD_STATS).
# - 0x85add0 thiscall(squad manager; club, 0); 0x85a760 thiscall(squad manager; staff, club) -> al (called twice);
#   0x4cf5a0 / 0x4cf6d0 thiscall(contract manager; staff) ret 4 (the loan contract asked up to three times);
#   0x85e750 thiscall(contract; buf, 0, style) ret 0xc; 0x85e8f0 thiscall(contract; buf) ret 4; 0x542100 cdecl(club,
#   0, buf, 0xc8, 3, 1); 0x4849c0 cdecl(club, &llist); 0x68d2e0 LLIST peek, 0x68d1b0 get_next; the list rewound by
#   copying the head (+2) into the cursor (+0xa).
# - 0x7d4460's squad columns call 0x7da070 (except the squad number, inlined); the others inline it.
# Defects of the original: none found.
MOTOR = True
TELAS = True
ROTINAS = [('printouts_write_club_transfers', 0x7d60c0, [0x6a, 0xff, 0x68, 0xf6, 0xfc, 0x95, 0x00]),
           ('printouts_write_club_squad_stats', 0x7d5280, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('printouts_print_club_report', 0x7d4460, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as impressões (4): a impressão do clube, as estatísticas do elenco e as transferências',
        'the printouts (4): a club\'s printout, its squad statistics and its transfers',
        [('printouts_write_club_transfers', 'printouts_write_club_transfers', '0x7d60c0', 'printouts.cpp', 'cdecl',
          'escreve as transferências do clube no período da impressão, por data: primeiro as compras (data, jogador, de onde, valor) e depois as vendas (para onde); sem clube de origem "Free Transfer", sem destino "Released"; sem valor, "Exch", "Loan", "Bos" ou "Free"; troca com valor, "<valor> p/x"; lado sem linhas, "None"; seleção nacional não escreve nada e devolve 0',
          'writes the club\'s transfers in the printout\'s period, by date: first the ones in (date, player, from, fee) and then the ones out (to); no selling club "Free Transfer", no buyer "Released"; without a fee, "Exch", "Loan", "Bos" or "Free"; an exchange with a fee, "<fee> p/x"; a side without rows, "None"; a national team writes nothing and returns 0'),
         ('printouts_write_club_squad_stats', 'printouts_write_club_squad_stats', '0x7d5280', 'printouts.cpp', 'cdecl',
          'escreve as estatísticas do elenco por nome: número, nome e nove estatísticas do jogador (jogos, gols, sofridos, pênaltis, assistências, amarelos, vermelhos, melhor em campo e nota média) no escopo do clube (6) ou da seleção (5)',
          'writes the squad statistics by name: number, name and nine statistics of the player (apps, goals, conceded, penalties, assists, yellow and red cards, man of the match and the average rating) in the club\'s scope (6) or the national team\'s (5)'),
         ('printouts_print_club_report', 'printouts_print_club_report', '0x7d4460', 'printouts.cpp', 'cdecl',
          'imprime um clube num arquivo (por padrão ~printout.txt): fixa o período desde o fim da última temporada da nação (dia 0 do ano para seleções e nações sem temporada dividida) até um ano depois, o cabeçalho com o nome e a data, o elenco (bit 1: número, nome, posições, nação, nascimento, idade, jogos e gols pela seleção, salário, fim do contrato e valor; o contrato de empréstimo quando o principal é de outro clube), as estatísticas (bit 2), as transferências (bit 4) e os jogos (bit 8); com impressão, cada linha vai para a impressora (um espaço sozinho = quebra de página); o arquivo temporário é apagado; sem clube ou sem abrir o arquivo, as caixas de erro 88, 113 e 175 e 0',
          'prints a club into a file (by default ~printout.txt): sets the period from the nation\'s last season end (day 0 of the year for national teams and nations without a split season) to a year later, the heading with the name and the date, the squad (bit 1: number, name, positions, nation, birth, age, caps and international goals, wages, contract end and value; the loan contract when the main one is another club\'s), the statistics (bit 2), the transfers (bit 4) and the fixtures (bit 8); when printing, each line goes to the printer (a lone space = page break); the temporary file is removed; no club or no file, the 88, 113 and 175 error boxes and 0')])
