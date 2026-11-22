# As estatísticas das competições (Comp_Stats): a estatística de um jogador e as tabelas ordenadas de clubes e de
# jogadores (comp_screens.cpp; leitor tel41, rodada 2 das telas, 22/11/2026). Item de simulação: get_player_stat é
# chamada pela 0x6b6e06.. (4 chamadas), get_sorted_player_table pelas mídias e prêmios (0x768d66, 0x769123, 0x769947,
# 0x769b43, 0x7d99a9) e pela tela 0x49626e; get_sorted_team_table pela tela 0x49597c e pela 0x5d6382.
# Convenções conferidas na desmontagem: get_player_stat thiscall(stats, player, stat) ret 8 (st0);
# get_sorted_team_table thiscall(stats, stat, &count, max) ret 0xc; get_sorted_player_table thiscall(stats, stat,
# &count, max, groups) ret 0x10 (groups comparado como word, -1 = todos). Chamadas: 0x4a4a30 find_player_entry
# thiscall ret 4 (pelo endereço: o desvio leva à cópia, tel41_1); 0x4a2ec0 / 0x4a3250 thiscall(stats, entry, stat) ret
# 8 (o dword inteiro do argumento stat é repassado, como na original); malloc 0x944e46; qsort 0x945333;
# 0x4ae4f0 comp_stats_get_player_position_group cdecl -> ax (já em Rust).
# A cópia de cada entrada de clube na tabela é campo a campo (dword, bytes, as quatro sequências por run_record_assign
# 0x448170, os bytes +0x69..+0x75), como a original; a de jogador é rep movsd de 11 dwords + 1 byte (0x2d inteiros).
# Defeitos do original: na tabela de jogadores o corte dos zeros do fim usa a contagem inteira (não o max), e o
# filtro por grupo só corta depois: o bloco é alocado com a contagem toda (sobra memória, sem efeito).
ROTINAS = [('comp_stats_get_player_stat', 0x4a31a0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_stats_get_sorted_team_table', 0x4a4b20, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('comp_stats_get_sorted_player_table', 0x4a4dc0, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('as estatísticas das competições: a estatística de um jogador e as tabelas ordenadas de clubes e de jogadores',
        "the competition statistics: a player's statistic and the sorted club and player tables",
        [('comp_stats_get_player_stat', 'Comp_Stats::get_player_stat', '0x4a31a0', 'comp_screens.cpp', 'thiscall',
          'uma estatística de um jogador na competição: o valor da entrada dele; sem entrada, 0 (sem jogador, a caixa de comp_stats.CPP 2485)',
          "one statistic of a player in the competition: his entry's value; with no entry, 0 (with no player, the comp_stats.CPP 2485 box)"),
         ('comp_stats_get_sorted_team_table', 'Comp_Stats::get_sorted_team_table', '0x4a4b20', 'comp_screens.cpp', 'thiscall',
          'a tabela dos clubes por uma estatística, o melhor primeiro: no máximo max (todos quando negativo); fora as estatísticas 3, 4 e 5, os clubes do fim com 0 saem; um bloco novo com a cópia das entradas (quem chama libera)',
          'the club table by one statistic, best first: at most max (all when negative); except for statistics 3, 4 and 5 the clubs at the bottom with 0 leave; a new block with the copied entries (the caller frees it)'),
         ('comp_stats_get_sorted_player_table', 'Comp_Stats::get_sorted_player_table', '0x4a4dc0', 'comp_screens.cpp', 'thiscall',
          'a tabela dos jogadores por uma estatística, o melhor primeiro: no máximo max, só os dos grupos de posição pedidos (0xffff = todos) e nunca um com 0; na estatística 0x11 os clubes vão antes para a ordem dos ids',
          'the player table by one statistic, best first: at most max, only those of the asked position groups (0xffff = all) and never one with 0; for statistic 0x11 the clubs are first put in id order')])
