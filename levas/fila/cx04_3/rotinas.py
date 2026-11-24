# Os dados proprios da Copa da UEFA e a tabela do ranking da FIFA no jogo salvo (uefa_cup.cpp, fifa_rankings.cpp;
# leitor cx04, rodada 5, 24/11/2026). Item do SAVE (CARGA): a Copa da UEFA grava pela virtual 0x24 do .tmp da copa
# (Cup::save_cup_to_disk 0x5210f0); o ranking da FIFA tem o seu arquivo no arquivo do save (fifa_rankings.dat), gravado
# por 0x776fa7 e lido pelo construtor FIFA_RANKINGS::FIFA_RANKINGS (0x58cefb) quando ha arquivo (carga de um jogo).
# - 0x903340 UefaCup::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 (cdecl) de 1 byte de +0xb2; devolve o eax
#   do fwrite. Prologo mov eax, [esp+4]; add ecx, 0xb2 (10 bytes).
# - 0x58dae0 FIFA_RANKINGS::save_dat thiscall(arquivo) ret 4: zopen 0x9326e0 (cdecl(arquivo, nome, 1, 0x16, 4)), zwrite
#   0x932e10 (cdecl(origem, tamanho, n, f)): a tabela (n nacoes x 0x2c), por nacao a contagem (+0xa, 2) e o historico
#   (+4, contagem x 5), o mes (+4, 1) e a data (+5, 8); zclose 0x932b00 (cdecl(&f)), cujo eax devolve. Caixas nas linhas
#   0x299, 0x2a0, 0x2aa, 0x2b1, 0x2b9, 0x2c0. Prologo sub esp, 0x204 (6 bytes).
# - 0x58d7a0 FIFA_RANKINGS::load_dat thiscall(arquivo) ret 4 -> 0/1: zopen (modo 0), malloc 0x944e46 da tabela, zread
#   0x932c30 (cdecl) na ordem da gravacao, a contagem lida em +8 e copiada para +0xa, o historico alocado (contagem x 5)
#   ou zerado; zclose e 1. Caixas nas linhas 0x23a, 0x242, 0x249, 0x253, 0x261, 0x268, 0x276, 0x27d. Mesmo prologo.
# Defeito do original: nas caixas de erro o arquivo fica aberto (sem zclose); a gravacao devolve o eax da caixa (a
# copia devolve 0).
# Iguais no cm0102-1080.exe.
# Comparadores: as gravacoes so a original (a prova e a do save); a carga so a original, com o plano (tabela, contagem =
# capacidade e historico so com resultados).
SAVE = True
CARGA = True
ROTINAS = [('uefa_cup_save_extra_data', 0x903340, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xc1, 0xb2, 0x00, 0x00, 0x00]),
           ('fifa_rankings_save_dat', 0x58dae0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('fifa_rankings_load_dat', 0x58d7a0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('os dados próprios da Copa da UEFA e o ranking da FIFA no jogo salvo', "the UEFA Cup's own data and the FIFA ranking in the saved game",
        [('uefa_cup_save_extra_data', 'UefaCup::save_extra_data', '0x903340', 'uefa_cup.cpp', 'thiscall, ret 4',
          'grava no arquivo temporário da Copa da UEFA o número de clubes da rodada preliminar',
          'writes into the UEFA Cup temporary file the count of clubs in the preliminary round'),
         ('fifa_rankings_save_dat', 'FIFA_RANKINGS::save_dat', '0x58dae0', 'fifa_rankings.cpp', 'thiscall, ret 4',
          'grava o ranking da FIFA no fifa_rankings.dat do jogo salvo: a tabela das nações, a contagem e o histórico de resultados de cada uma, o mês da virada anual e a data da última atualização; erro de gravação: a caixa de erro (e o arquivo fica aberto, defeito do original)',
          'writes the FIFA ranking into fifa_rankings.dat of the saved game: the nations\' table, each one\'s results count and history, the month of the yearly rollover and the date of the last update; a write error: the error box (and the file is left open, a defect of the original)'),
         ('fifa_rankings_load_dat', 'FIFA_RANKINGS::load_dat', '0x58d7a0', 'fifa_rankings.cpp', 'thiscall, ret 4',
          'lê o ranking da FIFA do fifa_rankings.dat do jogo salvo, na ordem da gravação: a tabela das nações e, de cada uma, os resultados (o histórico alocado do tamanho exato, ou nenhum), o mês da virada e a data da última atualização; devolve 1, ou 0 depois da caixa de erro',
          'reads the FIFA ranking from fifa_rankings.dat of the saved game, in the order it was written: the nations\' table and, for each, its results (the history allocated to the exact size, or none), the rollover month and the date of the last update; returns 1, or 0 after the error box')])
CAMPOS = []
INTERESSA = []
