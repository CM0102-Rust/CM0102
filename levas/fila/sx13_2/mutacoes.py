# Mutacoes (sx13_2).
# - get_prev anda para a frente: quem percorre uma lista da cauda para a cabeca (registros de clube, premios,
#   calendarios, transferencias) trata so o ultimo item e para (o proximo da cauda e nulo): o que eles fazem muda.
# - remove_node nao recua a cauda: tirado o ultimo no (as partidas do fix_man), a cauda fica no no liberado e o proximo
#   add_tail pendura o no novo nele: o item se perde da lista.
# O destrutor da lista so libera memoria: sem mutacao (o comparador confere o plano).
MUTACOES = [
    ('sx13_2_anda_para_frente',
     '    list.set_cursor(node.prev()); // sx13_2: the cursor goes back',
     '    list.set_cursor(node.next()); // mutation: the cursor goes forward',
     'sx13_2 step back'),
    ('sx13_2_cauda_parada',
     '        list.set_tail(n.prev()); // sx13_2: the tail goes back',
     '        // mutation: the tail stays on the freed node',
     'sx13_2 tail unlinked'),
]
