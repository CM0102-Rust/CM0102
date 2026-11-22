# As estatísticas das competições (1): as cópias dos recordes, as entradas de clube e de jogador, as ordenações e os
# recordes da temporada (comp_screens.cpp; leitor tel31, rodada das telas, 21/11/2026; grupo novo da varredura).
# Convenções conferidas na desmontagem: comp_run_record_assign, comp_season_records_assign thiscall ret 4;
# build_season_records_list thiscall(stats, &count) ret 4; add_club_entry thiscall ret 4; add_player_entry thiscall
# (stats, player, club) ret 8; compare_team_entries/compare_player_entries cdecl (ordens do qsort); update_most_titles
# e update_season_rating thiscall ret 0; comp_season_records_clear cdecl; get_player_position_group cdecl -> ax.
# Chamadas: 0x4a2ec0 comp_stats_get_team_entry_value e 0x4a3250 get_player_entry_value thiscall(stats, entry, byte)
# ret 8 (float em st0); 0x4a4850 find_club_entry thiscall ret 4; 0x4a22f0 restore_team_runs thiscall ret 4; 0x4a1fb0
# player_age_record_assign thiscall ret 4; 0x497910 LLIST::scalar_deleting_destructor thiscall ret 4; _ftol 0x945640.
# A cópia em linha de build_season_records_list (com 0x4a1f40/0x4a2050/0x4a1f70/0x4ab5b0 e goals_in_match) escreve os
# mesmos bytes que o operator= 0x4ab5f0 (conferido campo a campo): a cópia usa comp_season_records_assign nas duas.
# Rotinas inteiras iguais no cm0102-1080.exe. Quase todas rodam também fora das telas (a simulação grava as
# estatísticas a cada jogo: add_club_entry/add_player_entry, as ordenações, os recordes): o comparador pega desde a
# carreira automática no modo comparando.
# Defeitos do original: nenhum achado (o recorde de títulos fica com o primeiro a chegar à contagem).
TELAS = True
ROTINAS = [('comp_run_record_assign', 0x4ab5d0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('comp_season_records_assign', 0x4ab5f0, [0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14]),
           ('comp_stats_build_season_records_list', 0x4ab0c0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('comp_stats_add_club_entry', 0x4aceb0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_stats_add_player_entry', 0x4ad030, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_stats_compare_team_entries', 0x4ad1a0, [0x8b, 0x0d, 0x10, 0xae, 0xad, 0x00]),
           ('comp_stats_compare_player_entries', 0x4ad250, [0x8b, 0x0d, 0x10, 0xae, 0xad, 0x00]),
           ('comp_stats_update_most_titles_record', 0x4ad680, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('comp_stats_update_season_rating_record', 0x4ad7f0, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x19]),
           ('comp_season_records_clear', 0x4ad8c0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_stats_get_player_position_group', 0x4ae4f0, [0x8b, 0x4c, 0x24, 0x04, 0xb2, 0x14])]
DOCS = ('as estatísticas das competições (1): as cópias dos recordes, as entradas de clube e de jogador, as ordenações e os recordes da temporada',
        'the competition statistics (1): the record copies, the club and player entries, the sorts and the season records',
        [('comp_run_record_assign', 'comp_run_record_assign', '0x4ab5d0', 'comp_screens.cpp', 'thiscall',
          'copia um recorde de sequência das estatísticas: a sequência (jogos e datas) e o dword seguinte',
          'copies a run record of the statistics: the run (games and dates) and the dword after it'),
         ('comp_season_records_assign', 'comp_season_records_assign', '0x4ab5f0', 'comp_screens.cpp', 'thiscall',
          'copia os recordes de uma temporada campo a campo (o operator= do compilador)',
          "copies a season's records field by field (the compiler's operator=)"),
         ('comp_stats_build_season_records_list', 'Comp_Stats::build_season_records_list', '0x4ab0c0', 'comp_screens.cpp', 'thiscall',
          'a lista da tela de recordes: os recordes de sempre (ano 0), a temporada atual (com o recorde de pontos do líder e o de média de nota preenchidos quando faltam) e as temporadas arquivadas, da mais nova para a mais velha, até 500 linhas',
          "the records screen list: the all-time records (year 0), the current season (with the leader's points record and the rating record filled in when missing) and the archived seasons, newest first, up to 500 rows"),
         ('comp_stats_add_club_entry', 'Comp_Stats::add_club_entry', '0x4aceb0', 'comp_screens.cpp', 'thiscall',
          'uma entrada nova de clube nas estatísticas: zerada, com o id do clube e as sequências restauradas; a tabela cresce de 2 em 2 e fica desordenada',
          "a new club entry in the statistics: zeroed, with the club's id and its runs restored; the table grows two at a time and is marked unsorted"),
         ('comp_stats_add_player_entry', 'Comp_Stats::add_player_entry', '0x4ad030', 'comp_screens.cpp', 'thiscall',
          'uma entrada nova de jogador: zerada, com o id do jogador, o clube pelo qual jogou e a marca de nova; a tabela cresce de 22 em 22',
          "a new player entry: zeroed, with the player's id, the club he played for and the new mark; the table grows 22 at a time"),
         ('comp_stats_compare_team_entries', 'comp_stats_compare_team_entries', '0x4ad1a0', 'comp_screens.cpp', 'cdecl',
          'a ordem da tabela de clubes: pela estatística escolhida, maior primeiro (a 11, menor primeiro), e depois pelo id do clube',
          'the club table order: by the chosen statistic, highest first (statistic 11, lowest first), then by club id'),
         ('comp_stats_compare_player_entries', 'comp_stats_compare_player_entries', '0x4ad250', 'comp_screens.cpp', 'cdecl',
          'a ordem da tabela de jogadores: pela estatística escolhida, maior primeiro (a 0x12, menor primeiro), e depois pelo id; a média de nota só vale para quem jogou metade dos jogos do clube e as estatísticas por jogo (0xd a 0x10) para quem tem 35 vezes os jogos do clube em presenças',
          "the player table order: by the chosen statistic, highest first (0x12, lowest first), then by id; the average rating only counts players with half their club's games and the per-game statistics (0xd to 0x10) those with 35 times their club's games in appearances"),
         ('comp_stats_update_most_titles_record', 'Comp_Stats::update_most_titles_record', '0x4ad680', 'comp_screens.cpp', 'thiscall',
          'o recorde de mais títulos: o clube que mais venceu a competição na tabela do histórico (no empate fica o primeiro a chegar); devolve 1 quando o dono mudou',
          'the most titles record: the club that won the competition most often in the history table (on a tie the first to get there keeps it); returns 1 when the holder changed'),
         ('comp_stats_update_season_rating_record', 'Comp_Stats::update_season_rating_record', '0x4ad7f0', 'comp_screens.cpp', 'thiscall',
          'o recorde de média de nota da temporada: o primeiro da ordem pela média (0x11), com a nota, os jogos, o nome e o clube',
          "the season's average rating record: the top of the rating order (0x11), with the rating, the games, the name and the club"),
         ('comp_season_records_clear', 'comp_season_records_clear', '0x4ad8c0', 'comp_screens.cpp', 'cdecl',
          'esvazia os recordes de uma temporada (-1 em cada recorde, -1,0 na nota)',
          "empties a season's records (-1 in every record, -1.0 in the rating)"),
         ('comp_stats_get_player_position_group', 'comp_stats_get_player_position_group', '0x4ae4f0', 'comp_screens.cpp', 'cdecl',
          'o grupo de posição de um jogador para as estatísticas: goleiro (1), defensor (4), meio-campista (0x10) ou atacante (0x40), pelas posições com nota 20',
          "a player's position group for the statistics: goalkeeper (1), defender (4), midfielder (0x10) or attacker (0x40), by the positions rated 20")])
