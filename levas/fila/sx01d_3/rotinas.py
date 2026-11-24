# O objeto da midia: construtor, destrutor e a carga do media.dat do save (Media::Media 0x72b1d0, Media::~Media 0x72b200,
# Media::load_media_dat 0x72b240, media.cpp; leitor sx01d, rodada 5, 23/11/2026). As tres rotinas inteiras
# (0x72b1d0..0x72b1fb, ret 4; 0x72b200..0x72b230, ret; 0x72b240..0x72b5e6, ret 4; o resto ate a proxima fronteira e
# enchimento).
# Chamadas: o construtor pela criacao do jogo (0x829c5d, sem save) e pela carga (0x82c9c2, com o save); o destrutor
# pela limpeza do jogo (0x82ffb4); a carga so pelo construtor (0x72b1f1).
# Convencoes conferidas na desmontagem:
# - 0x72b1d0 thiscall(midia, save) ret 4, devolve a midia; zera +0xc, +0x10, +0x14, +0, +4, +8 e chama a carga com o
#   save (o retorno nao e usado);
# - 0x72b200 thiscall(midia) ret; free(+0xc) e free(+0) (0x9452ca cdecl), cada ponteiro zerado;
# - 0x72b240 thiscall(midia, save) ret 4 -> al; 0x9326e0 zopen cdecl(save, "media.dat", 0, 0x16, 4); 0x932c30 zread
#   cdecl(buf, tamanho, quantos, z); 0x944e46 malloc cdecl; 0x945333 qsort cdecl(base, n, largura, 0x757ea0 /
#   0x757ee0); 0x417c10 Date::operator= thiscall(novo, &tmp, antigo); 0x932b00 zclose cdecl(&z); a caixa de erro:
#   0x944cff _splitpath, 0x90d130 cdecl(nome, ext, linha) cujos argumentos ficam na pilha para o sprintf (0x944c9f,
#   "v%s %s.%s %d" em 0xae24d0), 0x5e8290 ("Error", texto, 0) e [0xb67a34] = 0 (show_error_box, transfer_manager.rs).
# Defeito do original: nos erros de leitura o item fica aberto (sem zclose).
SAVE = True
CARGA = True  # a carga so roda ao carregar: a prova do save carrega o save de ferias com e sem a leva
ROTINAS = [('new_media', 0x72b1d0, [0x56, 0x8b, 0xf1, 0x8b, 0x4c, 0x24, 0x08]),
           ('drop_media', 0x72b200, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x0c]),
           ('load_media_dat', 0x72b240, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('o objeto da midia e a carga do media.dat',
        'the media object and the loading of media.dat',
        [('new_media', 'Media::Media', '0x72b1d0', 'media.cpp', 'thiscall, ret 4',
          'monta o objeto da midia: a tabela de interesses de transferencia e o registro de historias vazios (ponteiros, capacidades e contagens zerados) e, com um save, le os dois do media.dat (load_media_dat; a resposta nao e usada); devolve o objeto',
          'builds the media object: the transfer interest table and the story log empty (pointers, capacities and counts at 0) and, with a save, reads both from media.dat (load_media_dat; its answer is not used); returns the object'),
         ('drop_media', 'Media::~Media', '0x72b200', 'media.cpp', 'thiscall',
          'libera a tabela de interesses de transferencia e o registro de historias (free), zerando cada ponteiro (capacidades e contagens ficam)',
          'frees the transfer interest table and the story log (free), zeroing each pointer (capacities and counts stay)'),
         ('load_media_dat', 'Media::load_media_dat', '0x72b240', 'media.cpp', 'thiscall, ret 4',
          'le o media.dat do save: sem save ou sem o item, 0; a contagem dos interesses de transferencia (tambem a capacidade), os registros de 25 bytes ordenados (compare_id_pair_keys), a contagem do registro de historias e os registros de 23 bytes (de 22 bytes convertidos quando a versao dos dados do jogo, em valor absoluto, e menor que 0x3e037: o codigo da historia passa de byte a palavra) ordenados (compare_story_log_records); fecha o item e devolve 1. Leitura curta ou alocacao que falha mostra a caixa "Error" do media.cpp (linhas 0xa5, 0xb1, 0xb9, 0xc4, 0xcf, 0xd8, 0xdf, 0xf5) e devolve 0 com o item aberto',
          'reads media.dat from the save: without a save or the item, 0; the transfer interest count (also the capacity), the 25-byte records sorted (compare_id_pair_keys), the story log count and the 23-byte records (22-byte ones converted when the game data version, in absolute value, is below 0x3e037: the story code goes from a byte to a word) sorted (compare_story_log_records); closes the item and returns 1. A short read or a failed allocation shows the media.cpp "Error" box (lines 0xa5, 0xb1, 0xb9, 0xc4, 0xcf, 0xd8, 0xdf, 0xf5) and returns 0 with the item open')])
