# As estatísticas das competições (Comp_Stats): achar a entrada de um clube e a de um jogador e a estatística de um
# clube (comp_screens.cpp; leitor tel41, rodada 2 das telas, 22/11/2026). Item de simulação: rodam a cada partida
# (record_match_result procura a entrada do clube), nas ordenações (a 0x4ad250 chama a 0x4a4850), na confiança da
# diretoria e na IA (get_club_stat tem 25 chamadores).
# Convenções conferidas na desmontagem: find_club_entry thiscall(stats, club) ret 4; find_player_entry thiscall(stats,
# player) ret 4; get_club_stat thiscall(stats, club, stat) ret 8, o valor em st0. Chamadas: qsort 0x945333 e bsearch
# 0x946464 (cdecl), 0x4a2ec0 comp_stats_get_team_entry_value thiscall(stats, entry, stat) ret 8.
# A chave do bsearch da 0x4a4850 é uma entrada inteira de 0x76 bytes na pilha (os construtores de data 0x54a110 e de
# sequência 0x448670 e o destrutor vazio 0x5a53c0 só tocam a pilha; a ordem lê só o id): a cópia usa um local zerado com
# o id. O +0x566 é a entrada provisória que get_club_projected_positions (0x4ae300, tel41_4) empresta à projeção.
# Prólogo: a 0x4a4850 termina depois de push -1; push 0x956788 (fronteira); as outras, sub esp / mov eax, [esp+4].
# Sem RELATIVAS. Defeitos do original: nenhum.
ROTINAS = [('comp_stats_find_club_entry', 0x4a4850, [0x6a, 0xff, 0x68, 0x88, 0x67, 0x95, 0x00]),
           ('comp_stats_find_player_entry', 0x4a4a30, [0x81, 0xec, 0x30, 0x02, 0x00, 0x00]),
           ('comp_stats_get_club_stat', 0x4a2e10, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as estatísticas das competições: a entrada de um clube e a de um jogador e a estatística de um clube',
        "the competition statistics: a club's and a player's entry and a club's statistic",
        [('comp_stats_find_club_entry', 'Comp_Stats::find_club_entry', '0x4a4850', 'comp_screens.cpp', 'thiscall',
          'a entrada de um clube nas estatísticas: a entrada provisória da projeção (+0x566) quando há uma; sem clube a caixa de comp_stats.CPP 3297 e 0; senão a tabela de clubes posta na ordem dos ids (quando não está) e a busca binária pelo id',
          "a club's entry in the statistics: the projection's temporary entry (+0x566) when there is one; with no club the comp_stats.CPP 3297 box and 0; otherwise the club table put in id order (when it is not) and the binary search by id"),
         ('comp_stats_find_player_entry', 'Comp_Stats::find_player_entry', '0x4a4a30', 'comp_screens.cpp', 'thiscall',
          'a entrada de um jogador nas estatísticas: sem jogador a caixa de comp_stats.CPP 3342 e 0; senão a tabela de jogadores posta na ordem dos ids (quando não está) e a busca binária pelo id',
          "a player's entry in the statistics: with no player the comp_stats.CPP 3342 box and 0; otherwise the player table put in id order (when it is not) and the binary search by id"),
         ('comp_stats_get_club_stat', 'Comp_Stats::get_club_stat', '0x4a2e10', 'comp_screens.cpp', 'thiscall',
          'uma estatística de um clube na competição: o valor da entrada dele; sem entrada, 0 (sem clube, a caixa de comp_stats.CPP 2274)',
          "one statistic of a club in the competition: its entry's value; with no entry, 0 (with no club, the comp_stats.CPP 2274 box)")])
