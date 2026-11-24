# main.cpp (leitor sx13, rodada 5, 24/11/2026): o nó da lista encadeada (LLIST_NODE) e a lista vazia (LLIST::LLIST).
# - 0x68cfc0 LLIST_NODE::LLIST_NODE: thiscall(nó, dado, tamanho) ret 8; o tamanho é lido como palavra (mov di, word
#   [esp+0x14]) e alargado com sinal (movsx) para o operator new 0x944cf1 (cdecl, 1) e para a cópia (rep movsd/movsb).
#   Grava +0xa e +6 = 0 antes do new, depois +4 = tamanho e +0 = cópia. Devolve o nó. Só o add_tail 0x68d0d0 a chama
#   (0x68d115, 0x68d15b).
# - 0x68d010 LLIST_NODE::~LLIST_NODE: thiscall(nó), ret 0: push [ecx]; call 0x944c94 (operator delete, cdecl, 1).
#   Prólogo mov eax, [ecx]; push eax; call rel32: RELATIVAS (0x68d010, 3) no rust/src/detours.rs (8 bytes, a chamada
#   inteira; instrução seguinte em 0x68d018).
# - 0x68d020 LLIST::LLIST: thiscall(lista), ret 0, devolve a lista (eax = ecx); +2, +6, +0xa = 0 e a palavra +0 = 0.
#   ~230 chamadores (listas locais e de objetos do jogo).
# Iguais no cm0102-1080.exe. Tipo: simulação (rodam o dia todo na carreira ENG).
# Comparadores: 0x68cfc0 só a original (aloca) e o plano (elos zerados, tamanho, cópia com os mesmos bytes); 0x68d010
# só a original (libera) e o nó intacto; 0x68d020 lado a lado no cabeçalho de 14 bytes.
SAVE = False
ROTINAS = [('llist_node_new', 0x68cfc0, [0x53, 0x56, 0x57, 0x66, 0x8b, 0x7c, 0x24, 0x14]),
           ('llist_node_drop', 0x68d010, [0x8b, 0x01, 0x50, 0xe8, 0x7c, 0x7c, 0x2b, 0x00]),
           ('llist_new', 0x68d020, [0x8b, 0xc1, 0x33, 0xc9, 0x89, 0x48, 0x02])]
DOCS = ('a lista encadeada do jogo: o nó e a lista vazia',
        'the game linked list: the node and the empty list',
        [('llist_node_new', 'LLIST_NODE::LLIST_NODE', '0x68cfc0', 'main.cpp', 'thiscall, ret 8',
          'o nó guarda a própria cópia do item: elos zerados, um bloco do tamanho pedido (palavra com sinal) do operator new, o tamanho na palavra +4 e os bytes copiados; devolve o nó. Defeito do original: a alocação não é conferida',
          "the node keeps its own copy of the item: links cleared, a block of the requested size (signed word) from operator new, the size in word +4 and the bytes copied; returns the node. Defect of the original: the allocation is not checked"),
         ('llist_node_drop', 'LLIST_NODE::~LLIST_NODE', '0x68d010', 'main.cpp', 'thiscall',
          'libera a cópia do item que o nó guarda (o ponteiro fica no nó)',
          "frees the item copy the node keeps (the pointer stays in the node)"),
         ('llist_new', 'LLIST::LLIST', '0x68d020', 'main.cpp', 'thiscall',
          'a lista começa vazia: cabeça, cauda e cursor nulos e a conta 0; devolve a lista',
          'the list starts empty: head, tail and cursor null and the count 0; returns the list')])
CAMPOS = []
INTERESSA = []
