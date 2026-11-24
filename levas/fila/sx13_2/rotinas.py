# main.cpp (leitor sx13, rodada 5, 24/11/2026): liberar a lista, andar para trás e tirar um nó dado.
# - 0x68d040 LLIST::~LLIST: thiscall(lista), ret 0. Do mesmo molde do LLIST::clear 0x68d080 (já em Rust,
#   manager_manager_clear) sem zerar o cabeçalho: cada nó, a partir da cabeça, 0x68d010 (thiscall, nó) e 0x944c94
#   (operator delete, cdecl, 1); a cabeça anda. ~400 chamadores, também por jmp nos desenroladores de exceção
#   (0x955...-0x965...) e duas referências absolutas (0x963921, 0x9639a1).
# - 0x68d1d0 LLIST::get_prev: thiscall(lista), ret 0: o item do nó do cursor, e o cursor vai ao anterior (+0xa do nó);
#   sem cursor, 0. ~40 chamadores (registros de clube, prêmios, calendários, transferências...).
# - 0x68d280 LLIST::remove_node: thiscall(lista, nó) ret 4, devolve 1. Só o fix_man 0x5aaf00 a chama (0x5ab151, já em
#   Rust: fix_man_daily_update; chega a esta pelo exe). Lê o cursor antes de baixar a conta; o cursor no nó anda ao
#   próximo (o nó ainda vivo). O "test esi, esi" antes de liberar fica (o nó nulo já teria caído antes).
# Iguais no cm0102-1080.exe. Tipo: simulação (carreira ENG).
# Comparadores: 0x68d040 só a original e o plano (cabeça nula, o resto do cabeçalho intacto); 0x68d1d0 pela devolução e
# o cursor (salvo e restaurado, molde do next_list_item 0x68d1b0); 0x68d280 só a original com o plano completo.
SAVE = False
ROTINAS = [('llist_free_nodes', 0x68d040, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x02]),
           ('llist_prev_item', 0x68d1d0, [0x8b, 0x51, 0x0a, 0x85, 0xd2]),
           ('llist_unlink', 0x68d280, [0x8b, 0x41, 0x02, 0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('a lista encadeada do jogo: liberar, andar para trás e tirar um nó',
        'the game linked list: freeing, walking backwards and unlinking a node',
        [('llist_free_nodes', 'LLIST::~LLIST', '0x68d040', 'main.cpp', 'thiscall',
          'libera todos os nós a partir da cabeça (a cópia do item e o nó); a cabeça termina nula e a conta, a cauda e o cursor ficam como estavam',
          'frees every node from the head (the item copy and the node); the head ends null and the count, tail and cursor stay as they were'),
         ('llist_prev_item', 'LLIST::get_prev', '0x68d1d0', 'main.cpp', 'thiscall',
          'o item do nó do cursor, e o cursor volta ao nó anterior; sem cursor, 0',
          'the item of the cursor node, and the cursor moves back to the previous node; with no cursor, 0'),
         ('llist_unlink', 'LLIST::remove_node', '0x68d280', 'main.cpp', 'thiscall, ret 4',
          'tira um nó dado: a cabeça ou o elo do anterior, a cauda ou o elo do próximo, a conta menos 1, o cursor no nó anda ao próximo; libera o item e o nó; devolve 1',
          'unlinks a given node: the head or the previous link, the tail or the next link, the count minus 1, a cursor on the node moves to the next; frees the item and the node; returns 1')])
CAMPOS = []
INTERESSA = []
