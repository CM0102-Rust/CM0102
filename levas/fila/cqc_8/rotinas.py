# A diretoria depois da partida: a confianca pelo resultado, a noticia da reacao e a avaliacao da federacao
# (manager_manager.cpp).
# - 0x695240 (MANAGER_MANAGER::update_board_confidence_after_match, 5.859 chamadas ate 2002-06-01 na cobertura de 17/10,
#   pela diretoria depois da partida 0x6951e0, ja em Rust): o ajuste pela disputa, pela reputacao (sem o jogo) ou pela
#   nota do time no jogo (x87, transcrito em f64 na ordem da original, as constantes lidas no exe), pelo placar, pelo
#   agregado, pelo classico/rival e pelas marcas; grava as quatro confiancas da linha da diretoria (+6/+8/+0xa/+0xc) e
#   decide a demissao (0x69aba0). As mutacoes estao nela.
# - 0x6a5fa0 (MANAGER_MANAGER::post_board_match_reaction_news, 10 chamadas): a noticia 0x138c da reacao (so para o
#   tecnico humano); com o sorteio(4) e os citados (0x6aaee0, ja em Rust na 263a).
# - 0x6a5a60 (MANAGER_MANAGER::post_fa_performance_news; sem cobertura medida): passageira (a noticia 0x1781).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x695240: thiscall (mov ecx, ebx nos dois pontos da 0x6951e0), 4 na pilha (ret 0x10). Dentro: 0x48f150 thiscall
#   (mov ecx, [[0xadadfc] + id * 4]), 1; 0x5c6710 thiscall (mov ecx, [0xae2c74]), 1; 0x53b900, 0x5409b0, 0x544820 cdecl
#   (add esp, 4), 1 (a 0x544820 le al); 0x53d980 cdecl (add esp, 0xc), 3 (pessoa, nacao, 1); 0x600a50 cdecl (add esp,
#   0x14), 5; 0x90cfc0 cdecl, 1; 0x6a9e60 thiscall (mov ecx, [esp+0x28] = this), 2 (clube, 1); 0x4b02e0 cdecl (add esp,
#   8), 2, le al; 0x6a69c0 thiscall (mov ecx, ebp = this), 15 (clube, adversario, competicao, marcas movsx bx, 0xad9c64,
#   -1, -1, oito zeros; ret 0x3c); 0x7875e0 thiscall (mov ecx, [0xb7431c]), 1; 0x6a5fa0 thiscall (mov ecx, ebp), 8;
#   0x74b060 thiscall (mov ecx, [0xb6b7c8]), 6 (clube, 0x3d ou 0x5c, 0, partida, 0, -1); 0x4a4850 thiscall (mov ecx,
#   [comp + 8]), 1; 0x7a7eb0 thiscall (mov ecx, [0xb7434c]), 1, le al; 0x6a5d10 thiscall (mov ecx, ebp), 1; 0x69aba0
#   thiscall (mov ecx, ebp), 2; 0x68d1b0 thiscall (mov ecx, [this+0x10]), 0; 0x68d1f0 thiscall, 1; 0x945640 _ftol.
#   0x6a5fa0: thiscall (mov ecx, ebp em 0x695ec2), 8 (ret 0x20). Dentro: 0x77bbe0 thiscall (lea ecx, a noticia), 2;
#   0x54a110 thiscall (lea ecx, o registro), 0; 0x785790 thiscall (lea ecx, a noticia), 2 (indice byte, valor);
#   0x6aaee0 thiscall (mov ecx, [esp+0x34] = this), 10 (partida, clube, ids, +8 da partida, &derrota, 5 saidas);
#   0x90cfc0 cdecl, 1; 0x7e4750 cdecl, 3 (partida, clube, registro) e 0x7e4de0 cdecl, 3 (registro, noticia, &indice)
#   (um add esp, 0x18 para as duas); 0x7861e0 thiscall (mov ecx, [0xb7431c]), 2 (noticia, clube).
#   0x6a5a60: thiscall (mov ecx, edi nos seis pontos da 0x6a5600..0x6a5a0c), 3 (ret 0xc). Dentro: 0x77bbe0 e 0x785790
#   como acima; 0x786090 thiscall (mov ecx, [0xb7431c]), 6 (noticia, selecao, 0, 0, 0, 0).
# Comparadores: os tres so a original (a 0x695240 chama o pedido a federacao, as noticias, as historias da midia, a
# demissao e tira item da lista; as duas noticias alocam e entregam); as copias levam os contadores e se provam pela
# carreira e pelas mutacoes. Iguais no cm0102-1080.exe (conferido byte a byte).
SAVE = False
ROTINAS = [('confianca_depois_da_partida', 0x695240, [0x83, 0xec, 0x2c, 0x8b, 0x44, 0x24, 0x3c]),
           ('noticia_da_reacao_da_diretoria', 0x6a5fa0, [0x6a, 0xff, 0x68, 0x16, 0xcf, 0x95, 0x00]),
           ('noticia_da_avaliacao_da_federacao', 0x6a5a60, [0x6a, 0xff, 0x68, 0xcb, 0xce, 0x95, 0x00])]
DOCS = ('a diretoria depois da partida: a confiança pelo resultado, a notícia da reação e a avaliação da federação',
        'the board after the match: the confidence from the result, the reaction news and the FA assessment',
        [('confianca_depois_da_partida', 'MANAGER_MANAGER::update_board_confidence_after_match', '0x695240', 'manager_manager.cpp', 'thiscall',
          'o ajuste da confiança da diretoria depois do jogo: a disputa, a diferença de reputação ou a nota do time e dos adversários no jogo, o placar, o agregado, o clássico e o rival, o peso da competição; grava as quatro confianças (entre 1 e 10000, com pisos pela fase), dá as histórias da mídia, o protesto da torcida e decide se pensa em demitir o técnico',
          'the board confidence adjustment after the match: the rivalry, the reputation gap or the team and opponent ratings in the match, the score, the aggregate, the derby and rival, the competition weight; writes the four confidence values (1 to 10000, with floors by stage), raises the media stories and the fan protest, and decides whether to consider sacking the manager'),
         ('noticia_da_reacao_da_diretoria', 'MANAGER_MANAGER::post_board_match_reaction_news', '0x6a5fa0', 'manager_manager.cpp', 'thiscall',
          'a notícia 0x138c da reação da diretoria ao jogo para o técnico do clube: as marcas, os clubes, os placares, os jogadores citados e o registro da partida',
          'news item 0x138c with the board reaction to the match for the club manager: the flags, the clubs, the scores, the named players and the match record'),
         ('noticia_da_avaliacao_da_federacao', 'MANAGER_MANAGER::post_fa_performance_news', '0x6a5a60', 'manager_manager.cpp', 'thiscall',
          'a notícia 0x1781 da avaliação da federação sobre o técnico da seleção numa competição, a partir do começo do jogo',
          'news item 0x1781 with the FA assessment of the national team manager in a competition, from the game start')])
CAMPOS = [('diretoria (0x49 bytes em [this] + id × 0x49) +6/+8/+0xa/+0xc confianças; valor; lê e grava (confianca_depois_da_partida); +0xe marcas (0x400, 4); valor; lê; +0x16; valor; grava (−250 com a história 0x3d)',
           'board (0x49 bytes at [this] + id × 0x49) +6/+8/+0xa/+0xc confidence; value; reads and writes (confianca_depois_da_partida); +0xe flags (0x400, 4); value; reads; +0x16; value; writes (−250 with story 0x3d)'),
          ('[this+0x10] lista dos ultimatos (+0 conta, +2 cabeça, +0xa cursor); valor/ponteiro; lê e grava (confianca_depois_da_partida)',
           '[this+0x10] ultimatum list (+0 count, +2 head, +0xa cursor); value/pointer; reads and writes (confianca_depois_da_partida)'),
          ('partida +0 competição, +0x14 competição, +0x18 seleção, +0x1c/+0x20 clubes, +0x30 fase, +0x32, +0x38 marcas, +0x3a tipo, +0x40, +0x43..+0x4c placares e agregado, +8; id/ponteiro/valor; lê',
           'fixture +0 competition, +0x14 competition, +0x18 national side, +0x1c/+0x20 clubs, +0x30 stage, +0x32, +0x38 flags, +0x3a kind, +0x40, +0x43..+0x4c scores and aggregate, +8; id/pointer/value; reads'),
          ('jogo (registro da partida) +0x4c, +0x4d, lados de 0x625: +0x127/+0x128, linhas de 0x49 bytes em +0x190 (id) e +0x1c0..+0x1c4 (notas, idade); valor; lê',
           'match record +0x4c, +0x4d, 0x625-byte sides: +0x127/+0x128, 0x49-byte rows at +0x190 (id) and +0x1c0..+0x1c4 (ratings, age); value; reads'),
          ('clube +0x53 nação, +0x57 divisão, +0x69 ficha (+0x38 cidade: +0x23/+0x2b coordenadas), +0x80 reputação, +0xb3/+0xb7/+0xbb rivais, +0xcf técnico (+0x5d, +0x69 ficha +0x1d), +0x1db liga; ponteiro/valor; lê',
           'club +0x53 nation, +0x57 division, +0x69 record (+0x38 city: +0x23/+0x2b coordinates), +0x80 reputation, +0xb3/+0xb7/+0xbb rivals, +0xcf manager (+0x5d, +0x69 record +0x1d), +0x1db league; pointer/value; reads'),
          ('competição +0x69 força; valor; lê; linha da tabela da liga +0x72 posição de perigo, +0x74; valor; lê', 'competition +0x69 strength; value; reads; league table row +0x72 danger position, +0x74; value; reads'),
          ('pessoa +0x1a nação, +0x61 ficha (+9, +0xb, +0xd reputações); valor/ponteiro; lê', 'person +0x1a nation, +0x61 player record (+9, +0xb, +0xd reputations); value/pointer; reads'),
          ('[0xae2d14] estado do clube (6 bytes, +4); valor; lê', '[0xae2d14] club state (6 bytes, +4); value; reads')]
INTERESSA = ['CMRTE']
