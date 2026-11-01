# Os boatos de transferencia, a noticia de que o clube nao vende o jogador e a noticia do titulo (media.cpp).
# - 0x754a30 (Media::process_transfer_interest, 321 chamadas ate 2002-06-01 na cobertura de 17/10, uma por dia pela
#   0x5cdd8f): os registros de interesse vencidos saem; o sorteio(25) do boato com os clubes interessados, a noticia
#   0xfbc, a reacao do elenco (0x755340, ja em Rust na 264a), o aviso ao rival (0x758010, idem) e a resposta a historia
#   (0x741020, original); e a noticia de que o clube nao vende (0x587600). As mutacoes estao nela.
# - 0x587600 (news_create_not_for_sale_story; sem cobertura medida, so pela 0x754a30): passageira. Fora do media.cpp no
#   mapa (european_cup.cpp, sem outro chamador; nenhum agente tem esse arquivo nesta rodada).
# - 0x75c390 (Media::queue_trophy_praise_story; sem cobertura medida, pelos registros do titulo): passageira.
# Trocas: a 0x74b060 (post_team_media_story, 2,9 KB, tabela de saltos de 33 casos e varias entregas) e a 0x741020
# (apply_player_story_response, 33 originais com a tela) ficaram de fora pelo tamanho; as passageiras sao do mesmo fluxo.
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x754a30: thiscall (mov ecx, [0xb6b7c8] em 0x5cdd8f), 0 na pilha (ret). Dentro: 0x4cf5a0 thiscall (mov ecx,
#   [0xae1050]), 1; 0x4cf5f0 thiscall (mov ecx, [0xae1050]), 2 (pessoa, clube); 0x54c5b0 thiscall (lea ecx, a data do
#   registro), 2 (saida, 31); 0x8ce990 thiscall (mov ecx, [0xde1f54]), 4 (pessoa, clube, 0, 0); 0x90cfc0 cdecl, 1;
#   0x77bbe0 thiscall (lea ecx, a noticia), 2; 0x785790 thiscall (lea ecx, a noticia), 2; 0x600a50 cdecl (add esp,
#   0x14), 5; 0x735870 thiscall (mov ecx, ebp = this), 3 (tecnico do dono, pessoa, 0x1f); 0x546ab0 cdecl (add esp, 8),
#   2 (pessoa, dono); 0x5b7d20 cdecl (add esp, 4), 1, le ax; 0x544520 cdecl (add esp, 8), 2 (pessoa, 0); 0x8d1da0 com
#   ecx = [0xde1f54] e 4 na pilha sem add esp (ret 0x10: chamada como thiscall com esse ecx); 0x7861e0 thiscall (mov
#   ecx, [0xb7431c]), 2; 0x755340 thiscall (mov ecx, ebp), 5 (0x1f, pessoa, dono, ids, n); 0x758010 thiscall (mov ecx,
#   ebp), 4 (pessoa, clube, dono, resposta); 0x741020 thiscall (mov ecx, ebp), 4 (pessoa, dono, 0x1f, resposta);
#   0x587600 cdecl (add esp, 0x10), 4; 0x945333 cdecl (add esp, 0x10), 4 (vetor, n, 0x19, 0x757ea0).
#   0x587600: cdecl (add esp, 0x10 em 0x7552c6), 4. Dentro: 0x8d1da0 como acima (0xd8); 0x77bbe0, 0x785790; 0x90cfc0
#   cdecl, 1; 0x547810 cdecl (add esp, 8), 2 (jogador, presidente); 0x786780 thiscall (mov ecx, [0xb7431c]), 1;
#   0x7861e0 thiscall, 2.
#   0x75c390: thiscall (mov ecx, [0xb6b7c8] em 0x8693e2 e 0x86b000), 3 (ret 0xc). Dentro: 0x53b900 cdecl, 1; 0x77bbe0,
#   0x785790; 0x90cfc0 cdecl, 1; 0x757f30 cdecl (add esp, 4), 1; 0x7dab90 thiscall (mov ecx, [0xb74430]), 3 (clube,
#   veiculo, nacao ou -1); 0x54cdb0 thiscall (mov ecx, 0xae2c90), 2; 0x7861e0 thiscall, 2.
# Comparadores: os tres so a original (a 0x754a30 tira registros, sorteia e entrega noticias; as duas noticias alocam e
# entregam); as copias levam os contadores e se provam pela carreira e pelas mutacoes. Iguais no cm0102-1080.exe
# (conferido byte a byte).
SAVE = False
ROTINAS = [('boatos_de_transferencia', 0x754a30, [0x6a, 0xff, 0x68, 0xbb, 0xd8, 0x95, 0x00]),
           ('noticia_nao_vende', 0x587600, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_do_titulo', 0x75c390, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os boatos de transferência, a notícia de que o clube não vende o jogador e a notícia do título',
        'the transfer rumours, the news that the club will not sell the player and the title news',
        [('boatos_de_transferencia', 'Media::process_transfer_interest', '0x754a30', 'media.cpp', 'thiscall',
          'tira os interesses de transferência vencidos (31 dias) e, por sorteio, publica o boato que liga o jogador aos clubes interessados, com o pedido de transferência, a resposta do técnico da IA, a reação do elenco e o aviso aos rivais; o dono da IA pode avisar que não vende',
          'drops transfer interest older than 31 days and, by chance, publishes the rumour linking the player to the interested clubs, with the transfer request, the AI manager response, the squad reaction and the rival warning; the AI owner may say he will not sell'),
         ('noticia_nao_vende', 'news_create_not_for_sale_story', '0x587600', 'media.cpp', 'cdecl',
          'a notícia 0xfbe de que o clube não vende o jogador, com o presidente quando ele gosta do jogador; a todos os técnicos se o jogador é famoso, senão aos humanos interessados',
          'news item 0xfbe that the club will not sell the player, with the chairman when he likes the player; to every manager for a famous player, else to the interested human managers'),
         ('noticia_do_titulo', 'Media::queue_trophy_praise_story', '0x75c390', 'media.cpp', 'thiscall',
          'a notícia 0x2e da diretoria e da torcida elogiando o técnico pelo título, com o veículo e o comentarista, para o técnico com 180 dias ou mais no clube',
          'news item 0x2e with the board and fans praising the manager for the title, with the outlet and the pundit, for a manager with 180 days or more at the club')])
CAMPOS = [('[media+0xc] vetor dos interesses (25 bytes: +0 jogador, +4 clube, +8 data, +0x10 estado 0/1/2, +0x11, +0x15 valor), [media+0x14] conta; id/valor; lê e grava (boatos_de_transferencia)',
           '[media+0xc] interest array (25 bytes: +0 player, +4 club, +8 date, +0x10 state 0/1/2, +0x11, +0x15 value), [media+0x14] count; id/value; reads and writes (boatos_de_transferencia)'),
          ('pessoa +0x3e data, +0x59/+0x5b atributos, +0x5f humano, +0x61 ficha (+0xd reputação), +0x39 clube; valor/ponteiro; lê', 'person +0x3e date, +0x59/+0x5b attributes, +0x5f human, +0x61 player record (+0xd reputation), +0x39 club; value/pointer; reads'),
          ('contrato +0x3d felicidade, +0x49; valor; lê; [0xae1080] contrato consultado; ponteiro; grava', 'contract +0x3d happiness, +0x49; value; reads; [0xae1080] contract looked up; pointer; writes'),
          ('clube +0x53 nação, +0x57 divisão, +0x80 reputação, +0xbf presidente, +0xcf técnico (+0x3e data); ponteiro/valor; lê', 'club +0x53 nation, +0x57 division, +0x80 reputation, +0xbf chairman, +0xcf manager (+0x3e date); pointer/value; reads'),
          ('diretoria em [0xb67a44] (+6 confiança); valor; lê (noticia_do_titulo)', 'board at [0xb67a44] (+6 confidence); value; reads (noticia_do_titulo)')]
INTERESSA = ['CMRTE']
