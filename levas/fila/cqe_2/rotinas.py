# A opção de troca da resposta da IA ao lance e o fim do empréstimo agendado (transfer_manager.cpp,
# contract_manager.cpp), o tema dos empréstimos (CM0102-Rust-Tweaks, docs/EMPRESTIMOS.md):
# - 0x8c8860 (TRANSFER_MANAGER::make_exchange_option, 1859 bytes, quadro SEH da LLIST local: 0x68d020/0x68d040 nos
#   dois caminhos): a opção de troca que a resposta da IA (0x8c8090) compara com a de dinheiro e a de cláusula: uma
#   oferta nova pelo preço, atração 50, e jogadores do comprador que o vendedor quer (a lista de observação do vendedor:
#   o alvo e as 25 vagas; o candidato precisa estar no comprador) com interesse ≥ 3,15 e nota acima de 0,9 da do
#   candidato, até 3; o valor de cada um sai do preço. 782 chamadas na carreira até 2002-06-01 (cobertura 17/10).
# - 0x4ccca0 (CONTRACT_MANAGER::schedule_loan_end, 548 bytes): passageira, só pela tela de transferências (0x8fbcc0): o
#   contrato de empréstimo acaba amanhã (tipo 7) e, com técnico humano no clube certo, a volta (0x4db0b0).
# Iguais no cm0102-1080.exe (as duas inteiras).
# Determinismo: o 7º byte do item da troca (0x68d0d0 copia 7 bytes) a original copia da pilha sem valor; a cópia grava
# zero. A posição, a nota base, as marcas e o estatuto ficam de uma vaga para a outra como na original (lá sem valor até
# a primeira vaga com ficha; candidato sem ficha não acontece); as marcas fora de 0..4 do alcance (que a 0x5408c0 não
# devolve) seriam o ebp anterior. O estatuto que vai à 0x846920 no alvo leva os bytes altos do ecx que a 0x8ce990
# deixou: a cópia passa só o byte (o parâmetro é char); nas vagas, os bytes altos de 0x25 × vaga, iguais.
# Carreira Rust até 2002-06-01 numa cópia fora do projeto (o motor.rs da principal com a 266ª) com as três cqe_*: duas
# execuções iguais (impressão 6353f8540cc7e467, 2358193948 sorteios); até 2001-09-01 a impressão é a mesma da base sem os
# itens (c1c4ed632488da61, 319167747 sorteios). Jogo comparando até 2001-09-01: 0 divergências.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x8c8860: 0x8c84a8 (push preço; push &atração; push &opção; push oferta; push alvo; push comprador;
#   mov ecx, ebx) -> thiscall(this, comprador, alvo, oferta, saída, atração, preço), ret 0x18.
# - quem chama a 0x4ccca0: 0x8fbcc0 -> thiscall(this, pessoa, clube), ret 8.
# - dentro da 0x8c8860: 0x4cf5a0 push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4; 0x68d020/0x68d040/0x68d080/
#   0x68d1b0 mov ecx, lista -> thiscall, ret 0; 0x4cf5f0 push clube; push pessoa; mov ecx, [0xae1050] -> thiscall, ret
#   8; 0x541f90 push 0; push &elenco; push comprador; add esp, 0x10 (com o push 0x41 da 0x944cf1) -> cdecl; 0x944cf1
#   push n; add esp -> cdecl; 0x8e5680 mov ecx, memória -> thiscall, ret 0; 0x8e78f0 push preço; push comprador; push
#   alvo; mov ecx, oferta -> thiscall, ret 0xc (sorteia 20 com a mesma liga +0x57); 0x945640 (_ftol) com o ST0;
#   0x5b7d20 push ficha; add esp, 4 -> cdecl; 0x5408c0 push técnico; push pessoa; add esp, 8 -> cdecl (al, movsx);
#   0x6e2f20 15 push; add esp, 0x3c -> cdecl; 0x4cf6d0 push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4; 0x4d1600
#   push 1; push 1; push 0; push pessoa; push 0; push comprador; mov ecx, [0xae1050] -> thiscall, ret 0x18; 0x8c5cf0
#   push 0; push 0; push pessoa; mov ecx, this -> thiscall, ret 0xc; 0x8c5e40 push &lista; push pessoa; mov ecx, this
#   -> thiscall, ret 8; 0x8c5fc0 push 0; push alvo; mov ecx, this -> thiscall, ret 8; 0x8ce990 push 1; push 0; push
#   vendedor; push pessoa; mov ecx, this -> thiscall, ret 0x10; 0x846920 push 0; push estatuto; push 0; push 1; push
#   vendedor; push pessoa; mov ecx, [this+0x856] -> thiscall, ret 0x18 (ST0, fcomp 3,15); 0x68d0d0 push 7; push &item;
#   mov ecx, troca -> thiscall, ret 8; 0x840d80 push 1; push −1; push &saída; push &vaga; push pessoa; push vendedor;
#   mov ecx, [[0xde1f54]+0x856] -> thiscall, ret 0x18.
# - dentro da 0x4ccca0: 0x54c5b0 push 1; push &data; mov ecx, 0xae2c90 -> thiscall, ret 8; 0x417c10 push data; push
#   &temp; lea ecx, contrato+0x2d -> thiscall, ret 8; 0x7875e0 push clube (ecx = [0xb7431c], não lido) -> stdcall, ret
#   4; 0x4db0b0 push clube; push pessoa; mov ecx, this -> thiscall, ret 8; na caixa, 0x944cff, 0x90d130, 0x944c9f e
#   0x5e8290 -> cdecl.
# Comparadores: a 0x8c8860 lado a lado a partir do mesmo estado de [0xae1080], *saída, *atração e do sorteador (a
# 0x8e78f0 sorteia; a cópia roda sem contar sorteios), com a devolução, a atração, [0xae1080] e a foto da oferta nova
# (os campos que a 0x8e5680 e a 0x8e78f0 preenchem e os itens da troca); a oferta da cópia é destruída e liberada. A
# 0x4ccca0 só a original (a volta do empréstimo entrega notícias e mexe no elenco).
SAVE = False
ROTINAS = [('opcao_de_troca', 0x8c8860, [0x6a, 0xff, 0x68, 0x07, 0x3e, 0x96, 0x00]),
           ('agendar_fim_do_emprestimo', 0x4ccca0, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00])]
DOCS = ('a opção de troca da resposta da IA ao lance (os jogadores do comprador que o vendedor quer, pelo interesse e pela nota) e o fim do empréstimo agendado',
        'the exchange option of the AI response to a bid (the buyer players the seller wants, by interest and rating) and the scheduled loan end',
        [('opcao_de_troca', 'TRANSFER_MANAGER::make_exchange_option', '0x8c8860', 'transfer_manager.cpp', 'thiscall, ret 0x18',
          'monta a opção de troca: oferta nova pelo preço com atração 50; para o alvo e as 25 vagas da lista do vendedor, jogadores do comprador na mesma posição, livres de ofertas e trocas, com interesse de pelo menos 3,15 e nota acima de 0,9 da do candidato entram na troca (até 3): a atração sobe 20 ou 5 e o valor deles sai do preço',
          'builds the exchange option: a new offer at the price with attraction 50; for the target and the 25 slots of the seller shortlist, buyer players in the same position, free of offers and exchanges, with interest of at least 3.15 and rating above 0.9 of the candidate join the exchange (up to 3): attraction rises 20 or 5 and their value leaves the price'),
         ('agendar_fim_do_emprestimo', 'CONTRACT_MANAGER::schedule_loan_end', '0x4ccca0', 'contract_manager.cpp', 'thiscall, ret 8',
          'só pela tela: o contrato de empréstimo da pessoa acaba amanhã (tipo 7) e, com técnico humano no clube que interessa, a volta antecipada',
          'screens only: the person loan contract ends tomorrow (type 7) and, with a human manager at the relevant club, the early return')])
CAMPOS = [('oferta nova +4 troca (itens de 7 bytes: id, 0, estatuto), +0x1b preço, +0x20 índice; valor; grava (opcao_de_troca)',
           'new offer +4 exchange (7-byte items: id, 0, status), +0x1b fee, +0x20 index; value; writes (opcao_de_troca)'),
          ('vaga da lista de observação +0xf alvo, +0x24 estatuto (bits 1..4); registro de 9 bytes +0 vagas, +4 ativa; valor; lê (opcao_de_troca)',
           'shortlist slot +0xf target, +0x24 status (bits 1..4); 9-byte record +0 slots, +4 active; value; reads (opcao_de_troca)'),
          ('contrato +0x35 tipo, +0x4f estatuto (4 bits altos); registro de 79 bytes +0xb bits 0x30; pessoa +0x39 clube, +0x61 ficha; valor e ponteiro; lê (opcao_de_troca)',
           'contract +0x35 type, +0x4f status (high 4 bits); 79-byte record +0xb bits 0x30; person +0x39 club, +0x61 record; value and pointer; reads (opcao_de_troca)'),
          ('registro de 79 bytes +0 contrato, +4 contrato de empréstimo; contrato +0x2d fim, +0x35 tipo (bits 3..5 = 0, baixos 7); [0xae1080]; índice e valor; lê e grava (agendar_fim_do_emprestimo)',
           '79-byte record +0 contract, +4 loan contract; contract +0x2d end, +0x35 type (bits 3..5 = 0, low 7); [0xae1080]; index and value; reads and writes (agendar_fim_do_emprestimo)')]
INTERESSA = ['CMRTE']
