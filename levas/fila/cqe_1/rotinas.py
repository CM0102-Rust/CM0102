# A resposta da IA ao lance e a negociação de empréstimo copiada da oferta (transfer_manager.cpp), o tema dos
# empréstimos (CM0102-Rust-Tweaks, docs/EMPRESTIMOS.md):
# - 0x8c8090 (TRANSFER_MANAGER::ai_respond_to_bid, 1711 bytes, quadro SEH sem destrutor: a data da vaga local): o
#   vendedor do computador responde ao lance. O lance de empréstimo (+0x1f = 1) vai direto para a recusa do empréstimo
#   (0x8bfa30), sem olhar salário, período nem jogador; o resto pela regra do vendedor (0x8cd240), o técnico humano, o
#   piso (0x4d1600), as outras ofertas, a troca (0x840d80, 0x843800) e as três opções (dinheiro 0x8c8740, troca 0x8c8860,
#   cláusula 0x8c8fd0): aceita (opção × 0,85 abaixo do pedido), contrapropõe (× 0,75) ou recusa (motivo 10 com o total
#   muito abaixo do valor do jogador). 791 chamadas na carreira até 2002-06-01 (cobertura 17/10).
# - 0x8bee50 (TRANSFER_MANAGER::add_loan_negotiation, 527 bytes, quadro SEH da oferta local): passageira, só pela tela
#   de transferências (0x8fb5e0): a cópia da oferta (sem as listas) com o contrato pelo atual (0x85e590) vira negociação.
# Iguais no cm0102-1080.exe (as duas inteiras).
# Determinismo: a vaga local que a 0x8c8090 passa à 0x840d80 (e a cópia de oferta da 0x8bee50) a original deixa com
# sobra de pilha fora dos campos que grava; a cópia zera (a 0x840d80 lê dela só a posição, o tipo 4 de +0x24 e o +0xf,
# este comparado com um jogador da troca, que nunca é do vendedor).
# Carreira Rust até 2002-06-01 numa cópia fora do projeto (o motor.rs da principal com a 266ª) com as três cqe_*: duas
# execuções iguais (impressão 6353f8540cc7e467, 2358193948 sorteios); até 2001-09-01 a impressão é a mesma da base sem os
# itens (c1c4ed632488da61, 319167747 sorteios). Jogo comparando até 2001-09-01: 0 divergências.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x8c8090: 0x8c0080 (push 1; push vaga; mov ecx, ebx), 0x8c63a0 (push 0; push vaga) e as Rust
#   0x8c28c0/0x8c41c0 -> thiscall(this, vaga, ensaio), ret 8.
# - quem chama a 0x8bee50: 0x8fb5e0 -> thiscall(this, oferta, a2, a3, a4, a5), ret 0x14.
# - dentro da 0x8c8090: 0x54a110 lea ecx, data -> thiscall, ret 0; 0x4cf5a0 push alvo; mov ecx, [0xae1050] -> thiscall,
#   ret 4; 0x8c6220 push 0; push 1; push oferta; mov ecx, this -> thiscall, ret 0xc; 0x8bfa30 push 0; push 0; push vaga;
#   mov ecx, this -> thiscall, ret 0xc; 0x8cd240 push 0; push 0; push oferta (ecx = this, não lido) -> stdcall, ret 0xc;
#   0x8c0080 push 0; push vaga; mov ecx, this -> thiscall, ret 8; 0x600a50 5 push; add esp, 0x14 -> cdecl; 0x53ca90 push
#   clube; add esp, 4 -> cdecl; 0x5fd210 push técnico; mov ecx, [0xb63c98] -> thiscall, ret 4; 0x5fea50 push técnico;
#   push 0; mov ecx, [0xb63c98] -> thiscall, ret 8 (devolve al); 0x8bf680 push motivo; push 0/1; push vaga; mov ecx,
#   this -> thiscall, ret 0xc; 0x4d1600 push 1; push 1; push &motivo; push alvo; push comprador; push vendedor; mov ecx,
#   [0xae1050] -> thiscall, ret 0x18; 0x68d1b0 mov ecx, lista -> thiscall, ret 0; 0x8e7b50 5 push; mov ecx, oferta ->
#   thiscall, ret 0x14; 0x8e7a70 push 1; push 0; push 0; push 1; mov ecx, oferta -> thiscall, ret 0x10; 0x5b7d20 push
#   ficha; add esp, 4 -> cdecl (só o al é testado); 0x840d80 push 0; push −1; push &saída; push &vaga; push jogador; push
#   vendedor; mov ecx, [[0xde1f54]+0x856] -> thiscall, ret 0x18; 0x843800 push 1; push valor; push vendedor; push jogador;
#   mov ecx, [this+0x856] -> thiscall, ret 0x10; 0x8c8740 e 0x8c8fd0 push preço; push &atração; push &opção; push alvo;
#   push comprador; mov ecx, this -> thiscall, ret 0x14; 0x8c8860 o mesmo com push oferta antes do alvo -> thiscall, ret
#   0x18; 0x8be9d0 push 0; push melhor; push oferta; mov ecx, this -> thiscall, ret 0xc; 0x8e5710 mov ecx, opção ->
#   thiscall, ret 0; 0x944c94 push; add esp, 4 -> cdecl; na caixa, 0x944cff (5 push), 0x90d130 (3 push), 0x944c9f (3
#   push) e 0x5e8290 (3 push), add esp, 0x14 e 0x24 -> cdecl.
# - dentro da 0x8bee50: 0x85e410 lea ecx, contrato -> thiscall, ret 0; 0x8e5680 lea ecx, cópia -> thiscall, ret 0;
#   0x417c10 push +0x12 da oferta; push &temp; lea ecx, cópia+0x12 -> thiscall, ret 8; 0x85e590 push a5; push a4; push
#   a3; push a2; push clube; push alvo; lea ecx, contrato -> thiscall, ret 0x18; 0x944cf1 push 0xe; add esp, 4 -> cdecl;
#   0x68d020 mov ecx, memória -> thiscall, ret 0; 0x68d0d0 push 0x50; push &contrato; mov ecx, [this+0x946] -> thiscall,
#   ret 8; 0x8be9d0 push 0; push &cópia; push oferta; mov ecx, this -> thiscall, ret 0xc; 0x8e5710 lea ecx, cópia ->
#   thiscall, ret 0.
# Comparadores: só a original nas duas (aceitam, recusam, contrapropõem, reagendam, mexem nas listas de observação e
# alocam e soltam ofertas e negociações). A prova da 0x8c8090 é a carreira Rust igual à da base e as mutações.
SAVE = False
ROTINAS = [('resposta_da_ia_ao_lance', 0x8c8090, [0x6a, 0xff, 0x68, 0xab, 0x3d, 0x96, 0x00]),
           ('negociacao_de_emprestimo', 0x8bee50, [0x6a, 0xff, 0x68, 0xee, 0x3a, 0x96, 0x00])]
DOCS = ('a resposta da IA ao lance (o lance de empréstimo recusado de saída, o piso, as opções de dinheiro, troca e cláusula, o aceite, a contraproposta e a recusa) e a negociação de empréstimo copiada da oferta',
        'the AI response to a bid (loan bids refused outright, the floor fee, the cash, exchange and clause options, acceptance, counter offer and refusal) and the loan negotiation copied from the offer',
        [('resposta_da_ia_ao_lance', 'TRANSFER_MANAGER::ai_respond_to_bid', '0x8c8090', 'transfer_manager.cpp', 'thiscall, ret 8',
          'o vendedor do computador responde ao lance: o de empréstimo é recusado sem olhar mais nada; sem regra que o faça aceitar, o piso e as outras ofertas, o valor da troca, as opções de dinheiro, troca e cláusula; a opção × 0,85 abaixo do pedido aceita, a melhor × 0,75 abaixo vira contraproposta, senão recusa',
          'the computer seller answers a bid: loan bids are refused without looking at anything else; unless a rule makes it accept, the floor fee and the other offers, the exchange value, the cash, exchange and clause options; an option worth less than the bid / 0.85 accepts, the best one under the bid / 0.75 becomes a counter offer, otherwise refuses'),
         ('negociacao_de_emprestimo', 'TRANSFER_MANAGER::add_loan_negotiation', '0x8bee50', 'transfer_manager.cpp', 'thiscall, ret 0x14',
          'só pela tela: a cópia da oferta sem as listas, com o contrato montado pelos termos novos do empréstimo, vira negociação; devolve o índice da cópia',
          'screens only: the offer copy without its lists, with the contract built from the new loan terms, becomes a negotiation; returns the copy index')])
CAMPOS = [('oferta +0 lista das ofertas pelo jogador, +4 troca, +8 comprador, +0xc alvo, +0x1f empréstimo, +0x20 índice, +0x2c status, +0x2e, +0x30 nibble baixo, +0x31 tipo (4 bits); id e valor; lê e grava +0x2c (resposta_da_ia_ao_lance), lê tudo (negociacao_de_emprestimo)',
           'offer +0 list of offers for the player, +4 exchange, +8 buyer, +0xc target, +0x1f loan, +0x20 index, +0x2c status, +0x2e, +0x30 low nibble, +0x31 type (4 bits); id and value; reads and writes +0x2c (resposta_da_ia_ao_lance), reads all (negociacao_de_emprestimo)'),
          ('pessoa +0x52 valor, +0x61 ficha de jogador; valor e ponteiro; lê (resposta_da_ia_ao_lance)',
           'person +0x52 value, +0x61 player record; value and pointer; reads (resposta_da_ia_ao_lance)'),
          ('contrato +4 clube, +0x35 tipo (6 bits); [0xae1080] o último contrato; valor e ponteiro; lê, grava [0xae1080] (resposta_da_ia_ao_lance)',
           'contract +4 club, +0x35 type (6 bits); [0xae1080] the last contract; value and pointer; reads, writes [0xae1080] (resposta_da_ia_ao_lance)'),
          ('gerência de transferências +0 vetor das ofertas, +0x856 a lista de observação, +0x946 a lista dos contratos de negociação; ponteiro; lê (resposta_da_ia_ao_lance), lê e grava +0x946 (negociacao_de_emprestimo)',
           'transfer manager +0 offer array, +0x856 the shortlist manager, +0x946 the negotiation contract list; pointer; reads (resposta_da_ia_ao_lance), reads and writes +0x946 (negociacao_de_emprestimo)')]
INTERESSA = ['CMRTE']
