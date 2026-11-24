# A gravacao do news.dat, do notes.dat e do retire.dat (news.cpp, notes.cpp, player_regen.cpp; leitor sx16, rodada 5,
# 24/11/2026). As tres chamadas pela gravacao do jogo (0x82eaa9, 0x82efb7, 0x82f289; save_game 0x82e..), cada uma com o
# conteiner do save; thiscall, ret 4.
# - 0x787c30 News::save_to_zip -> eax: 0x9326e0 zopen cdecl(save, "news.dat", 1, 0x16, 4); 0x932e10 zwrite
#   cdecl(buf, tamanho, quantos, z); 0x932b00 zclose cdecl(&z). Sem save, a caixa 0x2565 e 0; o resto fatal (0x256d,
#   0x2574, 0x257f, 0x2588, 0x2590, 0x2598: caixa, 0x5cc7d0, 0x5a8f60, exit(-1)). As noticias so com contagem.
# - 0x794f00 Notes::save_notes -> al: as 16 vagas, cada lista com a contagem (palavra com sinal em dword) e os
#   registros (0x114 e 0x10c) pelo 0x68d1b0 get_next; caixas 0xb2, 0xba (sem fechar), 0xc4, 0xce, 0xd4, 0xdd, 0xe7,
#   0xed (com zclose) e 0.
# - 0x7ad7c0 PLAYER_REGEN::save_retire_dat, sem resposta (eax = o do zclose; quem chama nao usa): as quatro tabelas
#   (+4/+8 0x1a bytes, +0x10/+0x14 0xc, +0x1c/+0x20 0x10, +0x3c/+0x40 0x10), a contagem e os registros (so com
#   contagem); sem save 0x1a46, sem o item 0x1a4e; nas falhas de escrita a caixa e o zclose, e segue escrevendo.
# Defeito do original (retire.dat): depois de uma falha de escrita ele fecha o item e continua escrevendo nele (mais
# caixas) e fecha de novo no fim.
# Comparadores: so a original (escrevem no conteiner do save).
SAVE = True
ROTINAS = [('news_save_to_zip', 0x787c30, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('notes_save_notes', 0x794f00, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('save_retire_dat', 0x7ad7c0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a gravação do news.dat, do notes.dat e do retire.dat',
        'saving news.dat, notes.dat and retire.dat',
        [('news_save_to_zip', 'News::save_to_zip', '0x787c30', 'news.cpp', 'thiscall, ret 4',
          'grava o news.dat: a contagem e as notícias (223 bytes, só se houver), a contagem e as caixas de entrada (12 bytes) e o último id; sem save, a caixa de erro e 0; não abrir o item ou escrita curta param o jogo',
          'writes news.dat: the count and the news items (223 bytes, only when there are any), the count and the inboxes (12 bytes) and the last id; without a save, the error box and 0; not opening the item or a short write stops the game'),
         ('notes_save_notes', 'Notes::save_notes', '0x794f00', 'notes.cpp', 'thiscall, ret 4',
          'grava o notes.dat: para cada uma das 16 vagas, a contagem e as notas (0x114 bytes) da primeira lista e a contagem e os registros (0x10c bytes) da segunda; falhas mostram a caixa de erro, fecham o item e devolvem 0',
          "writes notes.dat: for each of the 16 slots, the count and the notes (0x114 bytes) of the first list and the count and the records (0x10c bytes) of the second; failures show the error box, close the item and answer 0"),
         ('save_retire_dat', 'PLAYER_REGEN::save_retire_dat', '0x7ad7c0', 'player_regen.cpp', 'thiscall, ret 4',
          'grava o retire.dat: as aposentadorias pendentes (26 bytes), a fila de regens (12), os registros de não jogador de reserva (16) e as fichas de jogador livres (16), cada tabela com a contagem antes; uma falha de escrita mostra a caixa, fecha o item e continua (defeito)',
          'writes retire.dat: the pending retirements (26 bytes), the regen queue (12), the spare non-player records (16) and the free player data records (16), each table with its count first; a failed write shows the box, closes the item and goes on (defect)')])
