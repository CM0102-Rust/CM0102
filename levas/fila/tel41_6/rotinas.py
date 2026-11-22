# As estatísticas das competições no save: a carga (Comp_Stats::load, comp_screens.cpp; leitor tel41, rodada 2 das
# telas, 22/11/2026). Item do save (CARGA): chamada pela carga dos temporários das copas (Cup::load_from_disk
# 0x51fc00, gra_28) e das ligas (gra_29), ao carregar um save.
# Convenção conferida na desmontagem: thiscall(stats, file) ret 4 (0 sem arquivo, senão 1). Chamadas: fread 0x945bfe
# cdecl(buf, size, n, file); malloc 0x944e46; as caixas: show_error_box (sem arquivo) e, em toda leitura curta ou
# alocação falha, a caixa + game_fatal_cleanup + exit(-1) (comp_stats_fatal).
# As versões dos dados (G_GAME_DATA_VERSION 0xdd7110, valor absoluto): abaixo de 0x3e015 cada entrada de jogador vem
# com 0x1d bytes e as seis words +0x1d..+0x27 zeradas; acima de 0x3e007 vem o bloco +0x56a (0x64 bytes).
# Comparador: só a original (lê o arquivo aberto e aloca); a prova é a do save com a carga do save de férias.
# Defeitos do original: as tabelas antigas (clubes, jogadores, sequências) não são liberadas antes das novas; com
# capacidade 0 o ponteiro da tabela fica o que era; os bytes de ordem (+0x18, +0x25) não são gravados nem lidos.
SAVE = True
CARGA = True
ROTINAS = [('comp_stats_load', 0x4ab890, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as estatísticas das competições no save: a carga',
        'the competition statistics in the save: reading them',
        [('comp_stats_load', 'Comp_Stats::load', '0x4ab890', 'comp_screens.cpp', 'thiscall',
          'lê do temporário da competição na ordem da gravação: tabelas novas de clubes e de jogadores (nos dados antes da versão 0x3e015, entradas de jogador de 0x1d bytes com o resto zerado), o bloco +0x56a (só depois da versão 0x3e007), os recordes, as chaves, as bandeiras e as sequências guardadas; leitura curta para o jogo',
          "reads the competition's temporary file in the order it was written: new club and player tables (in data before version 0x3e015, player entries of 0x1d bytes with the rest zeroed), the block +0x56a (only after version 0x3e007), the records, the keys, the flags and the saved runs; a short read stops the game")])
