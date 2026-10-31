# As estatísticas do jogador (player_stats.cpp): a estatística de uma categoria e as notas combinadas
# (PlayerStats::get_stat 0x7c1e30, pela escolha da escalação match_man_score_player_for_role 0x6e2f20, pelo
# MATCH_PLAYER::setup 0x6eaea0, pela reputação do clube 0x545e90 e pela força do jogador 0x546ab0, já em Rust; também
# o dia da comissão game_daily_staff_update 0x5ce2d0, a transferência 0x8c9720 e os registros do clube 0x444930), a
# ordem dos idiomas (player_stats_compare_language_entries 0x7c6d40, o qsort e o bsearch da fluência 0x7c6830 e dos
# idiomas acrescentados, tirados e melhorados) e a ordem da história por clube (player_stats_compare_history_by_club
# 0x7c6d90, o qsort da carga das estatísticas 0x7bfd50; passageira). Chamadas originais pelo endereço: 0x7c3920 (o
# registro da categoria, cpq_10) e 0x7c2150 (a estatística do registro, já em Rust). Iguais no cm0102-1080.exe. As
# famílias player.cpp, person*.cpp, city*.cpp e non_player*.cpp não existem no mapa da comunidade: player_stats.cpp
# não é de nenhum outro agente desta rodada. Comparadores: pela devolução (os bits do x87 na estatística); a categoria
# fora de 1..6 (caixa da 0x7c3920) e as combinadas sem o registro 4 ou 5 (locais sem valor na original) só a original.
SAVE = False
ROTINAS = [('estatistica_da_categoria', 0x7c1e30, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('ordem_dos_idiomas', 0x7c6d40, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('ordem_da_historia_por_clube', 0x7c6d90, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('as estatísticas do jogador: a estatística de uma categoria e as notas combinadas, a ordem dos idiomas e a ordem da história por clube',
        'the player statistics: the statistic of a category and the combined ratings, the order of the languages and the order of the history by club',
        [('estatistica_da_categoria', 'PlayerStats::get_stat', '0x7c1e30', 'player_stats.cpp', 'thiscall',
          'a estatística pedida do registro da categoria (sem registro, o padrão para a nota e a média), ou as notas combinadas das categorias 2 a 5',
          'the requested statistic of the category record (without a record, the default for the rating and the average), or the combined ratings of categories 2 to 5'),
         ('ordem_dos_idiomas', 'player_stats_compare_language_entries', '0x7c6d40', 'player_stats.cpp', 'cdecl',
          'a ordem dos registros de idioma: pelo id da pessoa e pelo idioma, com 0xff valendo qualquer idioma',
          'the order of the language records: by person id and by language, with 0xff matching any language'),
         ('ordem_da_historia_por_clube', 'player_stats_compare_history_by_club', '0x7c6d90', 'player_stats.cpp', 'cdecl',
          'a ordem da história do jogador pelo dword +0xa e depois pelo +4',
          'the order of the player history by the dword +0xa and then by +4')])
CAMPOS = [('pessoa +0: id; valor; lê (estatistica_da_categoria); [0xae236c] número de pessoas com estatística; lê',
           'person +0: id; value; reads (estatistica_da_categoria); [0xae236c] number of people with statistics; reads'),
          ('registro da categoria (0x20 bytes) +0: jogos; +0xe: total (nota); lê (estatistica_da_categoria)',
           'category record (0x20 bytes) +0: games; +0xe: total (rating); reads (estatistica_da_categoria)'),
          ('registro de idioma (6 bytes) +0: id da pessoa; +4: idioma; lê (ordem_dos_idiomas)',
           'language record (6 bytes) +0: person id; +4: language; reads (ordem_dos_idiomas)'),
          ('registro da história +4 e +0xa; valor; lê (ordem_da_historia_por_clube)',
           'history record +4 and +0xa; value; reads (ordem_da_historia_por_clube)')]
INTERESSA = ['CMScout']
