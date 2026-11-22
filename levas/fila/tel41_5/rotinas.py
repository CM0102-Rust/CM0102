# As estatísticas das competições no save: a gravação (Comp_Stats::save, comp_screens.cpp; leitor tel41, rodada 2
# das telas, 22/11/2026). Item do save: chamada pela gravação dos temporários das copas e das ligas
# (Cup::save_cup_to_disk 0x5210f0 e o par da liga; docs/MAPA_SAVE.md §6), no meio do .tmp de cada competição.
# Convenção conferida na desmontagem: thiscall(stats, file) ret 4 (0 sem arquivo, senão 1). Chamadas: fwrite 0x945ce6
# cdecl(buf, size, n, file); free 0x9452ca; as caixas: show_error_box (sem arquivo, devolve 0) e, em toda gravação
# curta, a caixa + game_fatal_cleanup 0x5cc7d0 + 0x5a8f60 + exit(-1) (comp_stats_fatal).
# A ordem (linhas da caixa): +0x10, +0x14, clubes (6201), +0x1d, +0x21, jogadores (6226), +0x56a 0x64, +0x26 0x269,
# +0x28f 0x269, +0x4f8, +0x4fc, +0x500 (2), +0x506 0x5c, +0x562, as sequências guardadas: +4, +8 e capacidade × 0x1c.
# Comparador: só a original (grava arquivo); a prova é a do save (o .tmp da competição vai no .qst).
# Defeitos do original: as sequências guardadas são gravadas com a capacidade como contagem (save_team_runs deixa
# capacidade = contagem; restore_team_runs baixa só a contagem, então um save no meio da temporada grava registros já
# devolvidos: a carga os relê sem efeito [inferência]).
SAVE = True
ROTINAS = [('comp_stats_save', 0x4ac4e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as estatísticas das competições no save: a gravação',
        'the competition statistics in the save: writing them',
        [('comp_stats_save', 'Comp_Stats::save', '0x4ac4e0', 'comp_screens.cpp', 'thiscall',
          'grava no temporário da competição, em ordem: a tabela de clubes (capacidade, contagem, entradas de 0x76), a de jogadores (entradas de 0x2d), o bloco +0x56a, os dois conjuntos de recordes, as chaves do arquivo, a contagem de fins de temporada, as bandeiras de recorde e as sequências guardadas (liberadas antes quando a contagem é 0); gravação curta para o jogo',
          "writes into the competition's temporary file, in order: the club table (capacity, count, 0x76-byte entries), the player table (0x2d-byte entries), the block +0x56a, both record sets, the archive keys, the season end count, the record flags and the saved runs (freed first when the count is 0); a short write stops the game")])
