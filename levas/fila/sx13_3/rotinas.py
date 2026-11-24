# main.cpp (leitor sx13, rodada 5, 24/11/2026): copiar uma lista encadeada.
# - 0x68d2f0 LLIST::copy_from: thiscall(lista, origem) ret 4, sem devolução. Esvazia a lista com o LLIST::clear em
#   linha (o mesmo código do 0x68d080, já em Rust: manager_manager_clear, reaproveitado) e percorre a origem pelo
#   cursor dela (cursor = cabeça; a cada nó lê o item e o próximo; item nulo para o laço com o cursor nele; senão o
#   cursor anda e entra add_tail 0x68d0d0 thiscall(lista, item, tamanho) ret 8 com o tamanho do nó empurrado como
#   palavra: a metade alta do push é lixo, o add_tail lê só a palavra).
# - Chamadores: o gestor de transferências e as negociações (0x8be72e, 0x8becd4, 0x8c09b9, 0x8c68cd, 0x8e9c99,
#   0x8eeb05, 0x8eed4d, 0x8f30bc): roda na carreira ENG.
# Igual no cm0102-1080.exe. Tipo: simulação.
# Comparador: só a original (libera e aloca nós) e o plano lido da origem antes (os mesmos itens e tamanhos na mesma
# ordem, a cauda no último, o cursor da cópia nulo e o cursor da origem onde o percurso parou).
SAVE = False
ROTINAS = [('llist_assign', 0x68d2f0, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x46, 0x02])]
DOCS = ('a lista encadeada do jogo: copiar uma lista',
        'the game linked list: copying a list',
        [('llist_assign', 'LLIST::copy_from', '0x68d2f0', 'main.cpp', 'thiscall, ret 4',
          'esvazia a lista e acrescenta, em ordem, uma cópia de cada item da origem, percorrida pelo cursor dela a partir da cabeça; para no fim ou num nó de item nulo, onde o cursor da origem fica',
          "empties the list and appends, in order, a copy of every item of the source, walked by the source's cursor from its head; stops at the end or at a node with a null item, where the source cursor stays")])
CAMPOS = []
INTERESSA = []
