# A T2K (library_without_asserts): o contexto de memoria (tsimem.c da T2K): a saida de erro, os buffers de rascunho e
# os blocos guardados (leitor rx03, rodada 6, 25/11/2026; o mapa em levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS: o texto desenhado e tela. No jogo rodam na carga de cada fonte (FONT: lib_ctx_create 0x5b0d3c, os
# blocos da fonte, do escalador e do fluxo) e no desenho de cada letra nova (os blocos do contorno, do conversor e do
# mapa de bits; os buffers de rascunho). Caminho no jogo: abrir o jogo comparando.
# Convencoes conferidas: todas cdecl (call direto, add esp do chamador): lib_ctx_longjmp_error (contexto, codigo),
# nunca volta; lib_ctx_create (&status) -> contexto; lib_ctx_acquire_scratch_buffer (contexto, tamanho, vaga);
# lib_ctx_reset_scratch_buffers, lib_ctx_destroy e lib_ctx_free_all_blocks (contexto); lib_mem_alloc (contexto,
# tamanho); lib_mem_realloc (contexto, bloco, tamanho); lib_mem_free (contexto, bloco). O malloc, o free e o realloc
# do CRT do jogo (0x944e46, 0x9452ca, 0x945667) pelos invólucros exe::crt_*; o longjmp 0x94791c pelo FN_CRT_LONGJMP do
# rx01_1.
# Nomes: 0x9443a0, 0x944480 e 0x9445d0 (no levantamento lib_mem_alloc, lib_mem_realloc, lib_mem_free) viram
# lib_ctx_alloc, lib_ctx_realloc e lib_ctx_free, porque o rx01_8 ja tem os invólucros lib_mem_alloc e lib_mem_free
# (pelo endereco do exe; o desvio os leva para ca).
# Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: so a original (alocam, liberam ou saltam: rodar duas vezes alocaria duas vezes), com o plano conferido
# onde e barato: o cabecalho, as guardas e a vaga do bloco novo; as vagas vazias do contexto novo; a vaga de rascunho.
# Prova sem o jogo: o emulador (rascunho rx03/check_rx03.py rx03_2), com o malloc/free/realloc do CRT trocados por um
# alocador de pilha (as vezes falhando) e o longjmp parado.
# Defeitos do original: lib_ctx_create sem memoria escreve as vagas de rascunho no endereco 0x50 (contexto nulo);
# lib_mem_alloc e lib_mem_free com contexto nulo marcam o erro no endereco 0x9c (violacao de acesso em vez do longjmp).
# Depende do rx01_1 (FN_CRT_LONGJMP).
SAVE = False
TELAS = True
ROTINAS = [('lib_ctx_longjmp_error', 0x944180, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('lib_ctx_create', 0x9441a0, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('lib_ctx_acquire_scratch_buffer', 0x944250, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('lib_ctx_reset_scratch_buffers', 0x9442c0, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('lib_ctx_destroy', 0x944300, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('lib_ctx_free_all_blocks', 0x944350, [0x55, 0x8b, 0x6c, 0x24, 0x08]),
           ('lib_ctx_alloc', 0x9443a0, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('lib_ctx_realloc', 0x944480, [0x8b, 0x44, 0x24, 0x08, 0x53]),
           ('lib_ctx_free', 0x9445d0, [0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9])]
_A = 'library_without_asserts'
DOCS = ('a T2K: o contexto de memória, a saída de erro, os buffers de rascunho e os blocos guardados',
        'the T2K: the memory context, the error exit, the scratch buffers and the guarded blocks',
        [('lib_ctx_longjmp_error', 'lib_ctx_longjmp_error', '0x944180', _A, 'cdecl',
          'regra: marca o contexto em erro (+0x9c = 0x5500aaff) e salta (longjmp) para o buffer de salto do contexto com o código; não volta',
          'rule: marks the context in error (+0x9c = 0x5500aaff) and longjmps to the context\'s jump buffer with the code; never returns'),
         ('lib_ctx_create', 'lib_ctx_create', '0x9441a0', _A, 'cdecl',
          'regra: status = 0; o contexto de 0xa4 bytes com as palavras mágicas, a tabela vazia de 0x1f5 blocos e as seis vagas de rascunho vazias (livres); sem memória, status = 0x2718 e contexto nulo (defeito: as vagas são limpas mesmo assim, no endereço 0x50)',
          'rule: status = 0; the 0xa4-byte context with its magic words, the empty table of 0x1f5 blocks and the six empty scratch slots (free); without memory, status = 0x2718 and a null context (defect: the slots are cleared anyway, at address 0x50)'),
         ('lib_ctx_acquire_scratch_buffer', 'lib_ctx_acquire_scratch_buffer', '0x944250', _A, 'cdecl',
          'regra: vaga livre: ela é tomada (fica ocupada) e devolvida, trocada antes por um bloco novo do tamanho pedido se for menor; vaga ocupada: um bloco novo comum',
          'rule: a free slot is taken (marked busy) and returned, first replaced by a new block of the asked size when smaller; a busy slot gives a plain new block'),
         ('lib_ctx_reset_scratch_buffers', 'lib_ctx_reset_scratch_buffers', '0x9442c0', _A, 'cdecl',
          'regra: libera os seis buffers de rascunho e esvazia as vagas (nulo, tamanho 0, livre)',
          'rule: frees the six scratch buffers and empties their slots (null, size 0, free)'),
         ('lib_ctx_destroy', 'lib_ctx_destroy', '0x944300', _A, 'cdecl',
          'regra: libera os buffers de rascunho, a tabela e o contexto; os outros blocos ficam',
          'rule: frees the scratch buffers, the table and the context; the other blocks stay'),
         ('lib_ctx_free_all_blocks', 'lib_ctx_free_all_blocks', '0x944350', _A, 'cdecl',
          'regra: a liberação de emergência depois de um erro: todo bloco ainda na tabela, a tabela e o contexto; nada com contexto nulo',
          'rule: the emergency release after an error: every block still in the table, the table and the context; nothing for a null context'),
         ('lib_ctx_alloc', 'lib_ctx_alloc', '0x9443a0', _A, 'cdecl',
          'regra: bloco guardado (cabeçalho de 16 bytes, 2 bytes de guarda no fim) registrado na primeira vaga livre da tabela a partir do ponto de busca, em volta; erros por longjmp: contexto nulo 0x271b (defeito: escreve no endereço 0x9c), sem memória 0x2718, tabela no limite 0x271c, sem vaga 0x2721',
          'rule: a guarded block (16-byte header, 2 guard bytes at the end) recorded in the first free table slot from the search point, wrapping; errors by longjmp: null context 0x271b (defect: writes at address 0x9c), no memory 0x2718, table at its limit 0x271c, no slot 0x2721'),
         ('lib_ctx_realloc', 'lib_ctx_realloc', '0x944480', _A, 'cdecl',
          'regra: muda o tamanho do bloco mantendo a vaga e reescreve o tamanho e as guardas; bloco nulo devolve nulo; erros: guarda quebrada 0x2719, contagem fora de 1..capacidade 0x271d, sem memória 0x271e (a vaga fica nula), vaga fora 0x271f',
          'rule: resizes the block keeping its slot and rewrites its size and guards; a null block returns null; errors: broken guard 0x2719, count outside 1..capacity 0x271d, no memory 0x271e (the slot is left null), slot outside 0x271f'),
         ('lib_ctx_free', 'lib_ctx_free', '0x9445d0', _A, 'cdecl',
          'regra: libera o bloco depois de conferir as guardas e a vaga; limpa a vaga e baixa a contagem; bloco nulo não faz nada; erros: contexto nulo 0x271b (escreve no endereço 0x9c), guarda quebrada 0x2719, vaga com outro bloco 0x271f',
          'rule: frees the block after checking its guards and its slot; clears the slot and lowers the count; a null block does nothing; errors: null context 0x271b (writes at address 0x9c), broken guard 0x2719, slot holding another block 0x271f')])
CAMPOS = []
INTERESSA = []
MUTACOES = []
