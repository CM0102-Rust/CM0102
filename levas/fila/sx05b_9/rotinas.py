# A gravacao do estado do AWOL no awol.dat (awol.cpp; leitor sx05b, rodada 5, 23/11/2026). Usa AWOL_LISTS, AWOL_CPP e
# AWOL_DAT (sx05b_8): integrar depois dele.
# Roda ao gravar o jogo (0x82ef13, save_game_data). Item de save (SAVE).
# Convencoes conferidas na desmontagem: thiscall(save) ret 4, devolve al; zopen 0x9326e0 cdecl(save, "awol.dat", 1,
# 0x16, 4); zwrite 0x932e10 cdecl(origem, tamanho, n, arquivo); a conta de cada lista e o word com sinal do LLIST
# gravado como dword (movsx); o iterador rebobinado ([lista+0xa] = [lista+2]) e LLIST::get_next 0x68d1b0 thiscall; a
# caixa "Error" (show_error_at) e depois zclose 0x932b00 em cada falha com o arquivo aberto (linhas 0x141 a 0x192);
# sem save (0x132) ou sem arquivo (0x13a), so a caixa.
# Comparador: so a original (o arquivo do save nao se grava duas vezes); a prova do save compara os .qst.
SAVE = True
ROTINAS = [('awol_save', 0x418980, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('a gravação do estado do AWOL',
        'saving the AWOL state',
        [('awol_save', 'AWOL::save', '0x418980', 'awol.cpp', 'thiscall, ret 4',
          'grava o awol.dat como a carga o lê: a conta de pessoas, a tabela de marcas e cada uma das três listas (a conta e os registros de 12, 13 e 17 bytes, do começo); sem save ou sem arquivo, a caixa "Error" e 0; qualquer falha depois de abrir mostra a caixa, fecha e devolve 0; senão fecha e devolve 1',
          'writes awol.dat as the load reads it: the staff count, the flag table and each of the three lists (the count and the 12, 13 and 17-byte records, from the head); without a save or a file, the "Error" box and 0; any failure after opening shows the box, closes and returns 0; otherwise it closes and returns 1')])
CAMPOS = []
INTERESSA = ['SGE', 'CMRTE']
