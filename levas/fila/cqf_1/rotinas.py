# A oferta de transferência posta no vetor das ofertas e o empréstimo aceito pelo clube (transfer_manager.cpp):
# - 0x8be300 (TRANSFER_MANAGER::add_transfer_offer, 1741 bytes, quadro SEH só dos `new` da oferta e das listas, sem
#   destrutor no caminho normal): toda oferta nova passa por ela (os lances da IA 0x84a020, a oferta de empréstimo
#   0x8bed60, as telas): a vaga no vetor (a menor livre guardada em +0x85e, a do mesmo comprador pelo mesmo alvo, a
#   primeira vazia), o vetor que cresce 50, a cópia dos campos, o número de série, a lista das ofertas rivais pelo
#   jogador, o prazo da resposta (28 dias com humano, senão 0x8cc5b0) e a notícia. 1743 chamadas na carreira até
#   2002-06-01 (cobertura 17/10).
# - 0x8c0660 (TRANSFER_MANAGER::accept_loan_offer, 430 bytes, sem SEH): passageira, só pela tela de transferências
#   (0x8ef1e0) e pela notícia (0x784100): o clube aceita o pedido de empréstimo; a decisão do contrato, as notícias, as
#   listas de observação, o agendamento em 10 dias e o bit 0 do registro do jogador.
# Iguais no cm0102-1080.exe (as duas inteiras).
# Determinismo: os dois dwords que a 0x8be300 empurra para a 0x8c6220 têm só o byte baixo gravado (o resto é sobra de
# pilha); a 0x8c6220 (em Rust) lê os bytes, e a cópia passa os bytes estendidos com zero. As listas copiam os 4 bytes do
# índice das variáveis locais (a cópia passa o endereço de i32 locais). Nada mais lido sem valor.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x8be300: 0x84a728 (push 0; push &oferta; mov ecx, [0xde1f54]), 0x8bedf5 (push termos; push oferta;
#   mov ecx, esi), 0x8c0f10, 0x8c1229, 0x8bf1f4, 0x4e528c, 0x8efb53 -> thiscall(this, oferta, termos), ret 8.
# - quem chama a 0x8c0660: 0x784b5f e 0x784e1d (push 0; push vaga; mov ecx, [0xde1f54]), 0x8efce8 -> thiscall(this,
#   vaga, marcar), ret 8.
# - dentro da 0x8be300: 0x945667 push tamanho; push vetor; add esp, 8 -> cdecl(vetor, tamanho); 0x945501 push 4; push
#   cap; add esp, 8 -> cdecl(cap, 4); a caixa de memória (0x944cff 5 push, 0x90d130 4 push, 0x944c9f 3 push, add esp,
#   0x1c; 0x66f4e0 2 e 3 push; 0x5e8290 3 push, add esp, 0x18 -> cdecl), 0x5cc7d0 e 0x5a8f60 sem argumentos, 0x945934
#   push −1 (não volta); a caixa "Error" (aviso_em); 0x8d0f60 push oferta; mov ecx, ebx -> thiscall(this, oferta), ret 4;
#   0x944cf1 push 0x41 / 0xe; add esp, 4 -> cdecl; 0x8e5680 e 0x68d020 mov ecx, memória -> thiscall, ret 0; 0x417c10
#   push origem; push &temp; lea ecx, destino -> thiscall(destino, &temp, origem), ret 8; 0x68d2f0 push troca; mov ecx,
#   lista -> thiscall(lista, troca), ret 4; 0x68d0d0 push 4; push &índice; mov ecx, lista -> thiscall(lista, &índice,
#   4), ret 8; 0x4cf5a0 push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4; 0x600a50 push 0; push 0; push 1; push 0;
#   push clube; add esp, 0x14 -> cdecl(clube, 0, 1, 0, 0); 0x8cc5b0 push &fatias; push &dias; push oferta; mov ecx, ebx
#   -> thiscall(this, oferta, &dias, &fatias), ret 0xc; 0x8c6220 push fatias; push dias; push oferta; mov ecx, ebx ->
#   thiscall(this, oferta, dias, fatias), ret 0xc; 0x8e1780 push termos; push oferta; mov ecx, ebx -> thiscall, ret 8;
#   0x8d3090 push oferta; mov ecx, ebx -> thiscall, ret 4; 0x68d1b0 mov ecx, lista -> thiscall, ret 0; 0x7f75f0 push 0;
#   push pessoa; mov ecx, [0xb74448] -> thiscall(g, pessoa, 0), ret 8.
# - dentro da 0x8c0660: 0x4cc930 push oferta; mov ecx, [0xae1050] -> thiscall, ret 4; 0x8d76d0 push oferta; mov ecx,
#   ebp -> thiscall(this, oferta), ret 4; 0x8d8310 push 0; push oferta; mov ecx, ebp -> thiscall(this, oferta, 0), ret 8;
#   0x4cf5a0 como acima; 0x84aeb0 push pessoa; push clube; mov ecx, [ebp+0x856] -> thiscall(listas, clube, pessoa), ret
#   8; 0x68d1b0 mov ecx, lista; 0x8c6220 push 0; push 10; push oferta; mov ecx, ebp -> thiscall, ret 0xc.
# Comparadores: só a original nas duas (alocam e soltam ofertas e listas, agendam, entregam notícias). A da 0x8be300
# confere o plano: a vaga escolhida (com o vetor que cresce quando cheio) e o número de série gravados na oferta de
# entrada, e a série que sobe.
SAVE = False
ROTINAS = [('por_oferta_de_transferencia', 0x8be300, [0x6a, 0xff, 0x68, 0x6a, 0x3a, 0x96, 0x00]),
           ('aceitar_emprestimo', 0x8c0660, [0x8b, 0x44, 0x24, 0x08, 0x55])]
DOCS = ('a oferta de transferência posta no vetor das ofertas (a vaga, o vetor que cresce, a cópia, a série, as ofertas rivais ligadas, o prazo e a notícia) e o empréstimo aceito pelo clube',
        'the transfer offer placed in the offer array (the slot, the growing array, the copy, the serial number, linked rival offers, the deadline and the news) and the loan accepted by the club',
        [('por_oferta_de_transferencia', 'TRANSFER_MANAGER::add_transfer_offer', '0x8be300', 'transfer_manager.cpp', 'thiscall, ret 8',
          'põe a oferta numa vaga do vetor (a guardada, a do mesmo comprador pelo mesmo alvo ou a primeira vazia; cresce 50 quando cheio), copia os campos, dá o número de série, liga as ofertas rivais pelo jogador, agenda a resposta (28 dias com humano, senão o prazo da resposta) e entrega a notícia',
          'puts the offer in an array slot (the stored one, the same buyer\'s for the same target or the first empty one; grows by 50 when full), copies the fields, gives the serial number, links rival offers for the player, schedules the answer (28 days with a human, otherwise the reply deadline) and posts the news'),
         ('aceitar_emprestimo', 'TRANSFER_MANAGER::accept_loan_offer', '0x8c0660', 'transfer_manager.cpp', 'thiscall, ret 8',
          'só pelas telas: o clube aceita o pedido de empréstimo; a decisão do contrato, a notícia, e sem acordo o status 0xf, as listas de observação, o agendamento em 10 dias e o bit 0 do registro do jogador',
          'screens only: the club accepts the loan request; the contract decision, the news, and without agreement status 0xf, the shortlists, the 10-day scheduling and bit 0 of the player record')])
CAMPOS = [('gerência de transferências +0 vetor das ofertas, +4 capacidade, +8 conta, +0x85a próximo número de série, +0x85e menor vaga livre, +0x86e registro de 8 bytes por pessoa (+0 bit 0 com oferta, +1 ofertas pelo jogador, +2 ofertas em que está na troca), +0x856 listas de observação; ponteiro e valor; lê e grava (por_oferta_de_transferencia, aceitar_emprestimo)',
           'transfer manager +0 offer array, +4 capacity, +8 count, +0x85a next serial number, +0x85e lowest free slot, +0x86e 8-byte record per person (+0 bit 0 has offer, +1 offers for the player, +2 offers where he is in the exchange), +0x856 shortlists; pointer and value; reads and writes (por_oferta_de_transferencia, aceitar_emprestimo)'),
          ('oferta +0 lista das ofertas pelo jogador, +4 troca, +8 comprador, +0xc alvo, +0x10, +0x12 data, +0x1a fatia, +0x1b valor, +0x1f empréstimo, +0x20 vaga, +0x24 número de série, +0x28 ligada, +0x2c status, +0x2d, +0x2e, +0x2f, +0x30 e +0x31 nibbles baixos, +0x35, +0x3a, +0x40; id e valor; lê e grava (por_oferta_de_transferencia, aceitar_emprestimo)',
           'offer +0 list of offers for the player, +4 exchange, +8 buyer, +0xc target, +0x10, +0x12 date, +0x1a slice, +0x1b value, +0x1f loan, +0x20 slot, +0x24 serial number, +0x28 linked, +0x2c status, +0x2d, +0x2e, +0x2f, +0x30 and +0x31 low nibbles, +0x35, +0x3a, +0x40; id and value; reads and writes (por_oferta_de_transferencia, aceitar_emprestimo)'),
          ('pessoa +0x61 ficha de jogador; ponteiro; lê (por_oferta_de_transferencia)',
           'person +0x61 player record; pointer; reads (por_oferta_de_transferencia)'),
          ('contrato +4 clube, +0x35 tipo (6 bits); [0xae1080] o último contrato; id e ponteiro; lê, grava [0xae1080] (por_oferta_de_transferencia, aceitar_emprestimo)',
           'contract +4 club, +0x35 type (6 bits); [0xae1080] the last contract; id and pointer; reads, writes [0xae1080] (por_oferta_de_transferencia, aceitar_emprestimo)')]
INTERESSA = ['CMRTE']
