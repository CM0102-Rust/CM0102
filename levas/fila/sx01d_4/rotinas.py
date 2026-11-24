# A gravacao do media.dat no save (Media::save_media_dat 0x72b5f0, media.cpp; leitor sx01d, rodada 5, 23/11/2026). A
# rotina inteira (0x72b5f0..0x72b806, ret 4 em 0x72b7e9 e 0x72b804; o resto ate 0x72b810 e enchimento).
# Chamada pela gravacao do jogo (0x82f2db).
# Convencoes conferidas na desmontagem: thiscall(midia, save) ret 4 -> al; 0x9326e0 zopen cdecl(save, "media.dat", 1,
# 0x16, 4); 0x932e10 zwrite cdecl(buf, tamanho, quantos, z); 0x932b00 zclose cdecl(&z); a caixa de erro como na carga
# (show_error_box, transfer_manager.rs: _splitpath, 0x90d130 com os argumentos reaproveitados pelo sprintf, "Error",
# [0xb67a34] = 0).
# Defeito do original: nos erros de escrita o item fica aberto (sem zclose).
SAVE = True
ROTINAS = [('save_media_dat', 0x72b5f0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a gravacao do media.dat',
        'the saving of media.dat',
        [('save_media_dat', 'Media::save_media_dat', '0x72b5f0', 'media.cpp', 'thiscall, ret 4',
          'grava o media.dat no save: a contagem dos interesses de transferencia, os registros de 25 bytes, a contagem do registro de historias e os registros de 23 bytes; fecha o item e devolve 1. Sem save (linha 0x112), sem o item (0x11a) ou com escrita curta (0x121, 0x129, 0x130, 0x138) mostra a caixa "Error" do media.cpp e devolve 0 com o item aberto',
          'writes media.dat into the save: the transfer interest count, the 25-byte records, the story log count and the 23-byte records; closes the item and returns 1. Without a save (line 0x112), without the item (0x11a) or on a short write (0x121, 0x129, 0x130, 0x138) shows the media.cpp "Error" box and returns 0 with the item open')])
