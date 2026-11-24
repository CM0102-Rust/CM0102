# news.cpp (leitor sx16, rodada 5, 24/11/2026): o armazem das noticias: construtor, destrutor e a carga do news.dat.
# - 0x785b20 News::News thiscall(news, save) ret 4 -> news: a verificacao do CD (GetLogicalDriveStringsA [0x9670f8],
#   GetDriveTypeA [0x967100], sprintf 0x987e00 com "cm0102.exe" letra a letra, 0x4c0aa0 file_find_first cdecl(nome,
#   busca, 0), 0x4c0c10 file_find_close cdecl(busca), a caixa "Insert CD" 0x5e8290 com 1, em laco); no cm0102-1080 o
#   jne do tipo de drive (0x785b5a) vira nop e o teste do achado (0x785b9e) vira xor: a copia le esses bytes do exe
#   que roda (game_cd_check, como o player_stats_check_cd). Depois 0x787780 com o save (pelo desvio) ou o armazem
#   vazio: +4 = 0, +0 = 0, +0xc = 16, +8 = malloc(0xc0) (sem memoria: a caixa fatal 0x1ef9, 0x5cc7d0, 0x5a8f60,
#   exit(-1)), as 16 caixas {-1, -1, 0}, +0x10 = 0. Chamado pela criacao do jogo (0x828b15, sem save) e pela carga
#   (0x82bb9b, com o save).
# - 0x785d00 News::~News thiscall(news) ret: free(+0) e zera +0, +4; free(+8) e zera +8, +0xc. Chamado pela limpeza do
#   jogo free_game_data (0x83032a).
# - 0x787780 News::load_from_zip thiscall(news, save) ret 4 -> eax: 0x9326e0 zopen cdecl(save, "news.dat", 0, 0x16,
#   4); 0x932c30 zread cdecl(buf, tamanho, quantos, z); 0x944e46 malloc; 0x932b00 zclose cdecl(&z); a caixa de erro
#   (sem save 0x2500, sem o item 0x2508: show_error_box, devolve 0) e as fatais (0x250f, 0x251b, 0x2523, 0x2531,
#   0x253a, 0x2542, 0x254a). Chamada so pelo construtor (0x785c18).
# game_cd_check fica neste item e e usado tambem pelo sx16_10 e sx16_11 (os construtores do OfficialsManager e do
# PLAYER_REGEN): estes vao depois deste.
# Defeito do original: um armazem cuja leitura falhou no meio guarda os buffers ja alocados (vazam; as falhas de
# leitura sao fatais, entao so o 0 de "sem o item" chega a isso, e sem alocar).
# Comparadores: so a original (CD, alocacao, leitura do save).
SAVE = True
CARGA = True  # a carga so roda ao carregar: a prova do save carrega o save de ferias com e sem a leva
ROTINAS = [('new_news', 0x785b20, [0x81, 0xec, 0xe0, 0x04, 0x00, 0x00]),
           ('drop_news', 0x785d00, [0x56, 0x8b, 0xf1, 0x8b, 0x06]),
           ('news_load_from_zip', 0x787780, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o armazém das notícias e a carga do news.dat',
        'the news store and the loading of news.dat',
        [('new_news', 'News::News', '0x785b20', 'news.cpp', 'thiscall, ret 4',
          'pede o CD do CM 01/02 até um drive de CD ter o cm0102.exe (nos exes sem CD a verificação passa); com um save, lê o armazém dele (load_from_zip); sem save ou se a leitura não deu, monta o armazém vazio: nenhuma notícia, 16 caixas de entrada vazias (início e posição -1) e o último id 0',
          'asks for the CM 01/02 CD until a CD drive holds cm0102.exe (the no-CD exes pass the check); with a save, reads the store from it (load_from_zip); without a save or when the reading fails, builds the empty store: no news, 16 empty inboxes (start and position -1) and the last id 0'),
         ('drop_news', 'News::~News', '0x785d00', 'news.cpp', 'thiscall',
          'libera as notícias e as caixas de entrada, zerando os ponteiros e as contagens',
          'frees the news items and the inboxes, zeroing the pointers and the counts'),
         ('news_load_from_zip', 'News::load_from_zip', '0x787780', 'news.cpp', 'thiscall, ret 4',
          'lê o news.dat do save: a contagem e as notícias (223 bytes), a contagem e as caixas de entrada (12 bytes) e o último id; sem save ou sem o item, a caixa de erro e 0; leitura curta ou memória que falta param o jogo',
          'reads news.dat from the save: the count and the news items (223 bytes), the count and the inboxes (12 bytes) and the last id; without a save or the item, the error box and 0; a short read or missing memory stops the game')])
