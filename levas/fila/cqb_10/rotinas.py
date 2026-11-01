# A lista encadeada do jogo (LLIST): tirar o item pelo conteúdo, esvaziar e o item do cursor. Sem arquivo no mapa de
# funções (rótulo "-"), na vizinhança de manager_manager.cpp (0x68d010..0x68d370, antes do main.cpp de 0x68d370);
# nenhum outro agente as usa. A família de mídia, notícias e técnicos não tem mais rotina de simulação deste tamanho
# (ver o relatório).
# - 0x68d1f0 (LLIST::remove, ~77 chamadores, muitos já em Rust pelo endereço: faltas e ausências, ofertas de
#   contrato, vagas e ultimatos dos técnicos, amistosos, suspensões guardadas, ofertas guardadas): roda todo dia; as
#   mutações estão nela.
# - 0x68d080 (LLIST::clear, ~30 chamadores, entre eles a fila da transferência livre 0x8c18c0 em Rust, diária):
#   passageira.
# - 0x68d2e0 (LLIST::get_current, 18 chamadores, telas e listas): passageira.
# Convenções conferidas nos pontos de chamada da original (lição da 258a):
#   0x68d1f0: thiscall (mov ecx, a lista; ex. 0x41927e, 0x4ccaf9), 1 na pilha (ret 4), lê eax. Dentro: 0x68d010
#   thiscall (mov ecx, ebx = o nó), 0, ret 0 (libera o item: push [ecx], call 0x944c94, pop ecx); 0x944c94 free cdecl
#   (add esp, 4), 1.
#   0x68d080: thiscall (mov ecx, a lista; sem push), ret 0. Dentro: 0x68d010 e 0x944c94 como acima.
#   0x68d2e0: thiscall (mov ecx, a lista; ex. 0x416078, 0x46bae8), ret 0, lê eax.
# Comparadores: 0x68d1f0 só a original com o plano completo (o nó achado pela cópia antes; confere a devolução, a
# conta, a cabeça, a cauda, o cursor e os elos dos vizinhos: ela libera o nó e o item); 0x68d080 só a original
# (libera os nós) e confere o cabeçalho zerado; 0x68d2e0 pela devolução. Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('tirar_da_lista', 0x68d1f0, [0x8b, 0xc1, 0x53, 0x55, 0x56]),
           ('esvaziar_lista', 0x68d080, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x02]),
           ('item_do_cursor', 0x68d2e0, [0x8b, 0x41, 0x0a, 0x85, 0xc0])]
DOCS = ('a lista encadeada do jogo: tirar o item pelo conteúdo, esvaziar e o item do cursor',
        'the game linked list: removing an item by content, clearing and the cursor item',
        [('tirar_da_lista', 'LLIST::remove', '0x68d1f0', 'manager_manager.cpp', 'thiscall',
          'o primeiro nó cujo item tem os mesmos bytes do dado sai da lista (cabeça, cauda, conta e cursor consertados) e é liberado com o item; 1 se tirou',
          'the first node whose item has the same bytes as the data leaves the list (head, tail, count and cursor fixed) and is freed with its item; 1 when removed'),
         ('esvaziar_lista', 'LLIST::clear', '0x68d080', 'manager_manager.cpp', 'thiscall',
          'libera todos os nós e itens da lista e zera a conta, a cabeça, a cauda e o cursor',
          'frees every node and item of the list and zeroes the count, head, tail and cursor'),
         ('item_do_cursor', 'LLIST::get_current', '0x68d2e0', 'manager_manager.cpp', 'thiscall',
          'o item do nó do cursor, sem andar; sem cursor, 0',
          'the item of the cursor node, without moving; with no cursor, 0')])
CAMPOS = [('lista (14 bytes) +0 conta (palavra), +2 cabeça, +6 cauda, +0xa cursor; valor/ponteiro; lê e grava (tirar_da_lista, esvaziar_lista)',
           'list (14 bytes) +0 count (word), +2 head, +6 tail, +0xa cursor; value/pointer; reads and writes (tirar_da_lista, esvaziar_lista)'),
          ('nó (14 bytes) +0 item, +4 tamanho (palavra com sinal), +6 próximo, +0xa anterior; ponteiro/valor; lê e grava os elos',
           'node (14 bytes) +0 item, +4 size (signed word), +6 next, +0xa previous; pointer/value; reads and writes the links')]
INTERESSA = []
