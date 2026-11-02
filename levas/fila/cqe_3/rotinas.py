# Os lances e os pedidos de empréstimo da IA pelas 40 vagas da lista de observação (shortlist_manager.cpp), o tema dos
# empréstimos (CM0102-Rust-Tweaks, docs/EMPRESTIMOS.md):
# - 0x84a020 (Shortlist_Manager::make_club_bids, 2978 bytes, quadro SEH: dois STAFF_CONTRACT locais sem destrutor e a
#   oferta local com o destrutor 0x8e5710 no caminho normal): pelo laço do dia (0x5cccd0), 210231 chamadas na carreira
#   até 2002-06-01 (cobertura 17/10). Uma rotina grande, sozinha no item (o brief da rodada permite).
# Igual no cm0102-1080.exe (inteira).
# Determinismo: os dois STAFF_CONTRACT e a oferta locais são preenchidos pelas mesmas originais (0x85e410, 0x85fc50,
# 0x8e5680, 0x8e5d60) a partir de memória zerada na cópia (na original, sobra de pilha); o motivo (palavra local) a
# original não inicializa e a 0x8ce990/0x8ced40 o gravam antes da leitura; os dwords que a original empurra com bytes
# altos de ponteiro (o estatuto da 0x85fc50, o cargo da 0x4cc490 e o tipo da 0x843e10 no empréstimo) vão iguais; os de
# bytes altos sem valor (o cargo da 0x85fc50, que vem do edx da 0x600a50, e o da 0x5968d0, do fnstsw) vão só com o byte
# que o chamado lê (char).
# Carreira Rust até 2002-06-01 numa cópia fora do projeto (o motor.rs da principal com a 266ª) com as três cqe_*: duas
# execuções iguais (impressão 6353f8540cc7e467, 2358193948 sorteios); até 2001-09-01 a impressão é a mesma da base sem os
# itens (c1c4ed632488da61, 319167747 sorteios). Jogo comparando até 2001-09-01: 0 divergências.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x84a020: 0x5cccd0 -> thiscall(this, clube), ret 4.
# - dentro: 0x54c5b0 push 7; push &data; mov ecx, 0xae2c90 -> thiscall, ret 8; 0x85e410 lea ecx, contrato -> thiscall,
#   ret 0; 0x53b900 push clube; add esp, 4 -> cdecl; 0x600a50 5 push; add esp, 0x14 -> cdecl; 0x53ca90 push; add esp, 4
#   -> cdecl; 0x5fd210 push técnico; mov ecx, [0xb63c98] -> thiscall, ret 4; 0x5fea50 push técnico; push 4; mov ecx,
#   [0xb63c98] -> thiscall, ret 8; 0x5a0590 push clube; mov ecx, [0xae2a38] -> thiscall, ret 4; 0x844540 push 0xb; push
#   clube; mov ecx, this -> thiscall, ret 8; 0x53c590 push 0; push 0; push clube; add esp, 0xc -> cdecl; 0x4cf5a0 push
#   pessoa; mov ecx, [0xae1050] -> thiscall, ret 4; 0x8cca10 push 0; push pessoa; mov ecx, [0xde1f54] -> thiscall, ret
#   8; 0x8c5450 o mesmo; 0x8ce990 push 0; push &motivo; push clube; push pessoa; mov ecx, [0xde1f54] -> thiscall, ret
#   0x10; 0x8cdc90 push clube; push pessoa; mov ecx, [0xde1f54] -> thiscall, ret 8; 0x8c5e40 push 0; push pessoa; mov
#   ecx, [0xde1f54] -> thiscall, ret 8; 0x8ced40 push &motivo; push 1; push clube atual; push pessoa; push &hoje+7; push
#   clube; mov ecx, [0xde1f54] -> thiscall, ret 0x18; 0x62ed70 push substituto; mov ecx, [0xb63d28] -> thiscall, ret 4;
#   0x945640 (_ftol) com o valor no ST0; 0x8cde70 push 0; push substituto; mov ecx, [0xde1f54] -> thiscall, ret 8;
#   0x85fc50 push 0; push 0; push cargo; push estatuto; push −1; push clube; push pessoa; lea ecx, contrato ->
#   thiscall, ret 0x1c; 0x5968d0 push estatuto; push pessoa; mov ecx, finanças (a 0x5a0590 entre os dois push) ->
#   thiscall, ret 8; 0x90cfc0 push n; add esp, 8 depois de dois -> cdecl; 0x8e5680 lea ecx, oferta -> thiscall, ret 0;
#   0x8e5d60 push &termos; push 0; push +7 da vaga; push &vaga; push clube; push pessoa; lea ecx, oferta -> thiscall,
#   ret 0x18; 0x843e10 push tipo; push 1; push clube; push pessoa; mov ecx, this -> thiscall, ret 0x10; 0x8be300 push 0;
#   push &oferta; mov ecx, [0xde1f54] -> thiscall, ret 8; 0x4cc490 push 0; push 1; push 0; push cargo; push +0x24;
#   push +0x20 da oferta; push termos; mov ecx, [0xae1050] -> thiscall, ret 0x1c; 0x4cc9e0 push id do vendedor; push id
#   da pessoa; mov ecx, [0xae1050] -> thiscall, ret 8; 0x4cf5f0 push vendedor; push pessoa; mov ecx, [0xae1050] ->
#   thiscall, ret 8; 0x849ce0 push 0; push 0; push pessoa; push vendedor; mov ecx, this -> thiscall, ret 0x10; 0x4cda70
#   push 2; push alto; push baixo (o fator em f64); push −1; push 0; push 0; push pessoa; mov ecx, [0xae1050] ->
#   thiscall, ret 0x1c; 0x944c94 push termos; add esp, 4 -> cdecl; 0x8e5710 lea ecx, oferta -> thiscall, ret 0; 0x8d0040
#   push 0; push pessoa; push clube; mov ecx, [0xde1f54] -> thiscall, ret 0xc; 0x842740 push clube; push pessoa; mov
#   ecx, this -> thiscall, ret 8.
# Comparador: só a original (sorteia, faz lances e pedidos, aloca ofertas e termos e mexe nas vagas).
SAVE = False
ROTINAS = [('lances_do_clube', 0x84a020, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('os lances e os pedidos de empréstimo da IA pelas 40 vagas da lista de observação',
        'the AI bids and loan requests over the 40 shortlist slots',
        [('lances_do_clube', 'Shortlist_Manager::make_club_bids', '0x84a020', 'shortlist_manager.cpp', 'thiscall, ret 4',
          'clube do computador com lista ativa: em cada vaga com alvo e sem oferta, as regras de cargo, contrato, chegada e oferta; na de compra, o valor da vaga (dobros pela marca 0x20 e pela liga, o substituto lesionado, a ficha e a oferta por ele) e o lance se sorteio(valor) passar de sorteio(2 × reputação + 1000), com a oferta de contrato e a renovação no vendedor; na de empréstimo, o empréstimo possível e o pedido (0x842740) se sorteio(+0xb) passar de sorteio(reputação / 10) ou de sorteio(2 × reputação)',
          'computer club with an active shortlist: in each slot with a target and no offer, the role, contract, reach and offer rules; in a purchase slot, the slot value (doubled by the 0x20 mark and the league, the injured player to replace, the record and the offer for him) and a bid if random(value) beats random(2 × reputation + 1000), with the contract offer and the renewal at the seller; in a loan slot, whether the loan is possible and the request (0x842740) if random(+0xb) beats random(reputation / 10) or random(2 × reputation)')])
CAMPOS = [('vaga da lista de observação (0x25 bytes, 40 por clube em [this+0xdc] + 9 × id) +0 cargo, +1 posição, +7, +0xb valor, +0xf alvo, +0x13 substituto, +0x17 oferta, +0x24 bits (0 empréstimo, 5 dobro); valor e id; lê e grava +0x17 e +0x24 (lances_do_clube)',
           'shortlist slot (0x25 bytes, 40 per club at [this+0xdc] + 9 × id) +0 role, +1 position, +7, +0xb value, +0xf target, +0x13 player to replace, +0x17 offer, +0x24 bits (0 loan, 5 double); value and id; reads and writes +0x17 and +0x24 (lances_do_clube)'),
          ('registro de 9 bytes do clube em [this+0xdc] +0 vagas, +4 ativa, +8 dias; ponteiro e valor; lê (lances_do_clube)',
           '9-byte club record at [this+0xdc] +0 slots, +4 active, +8 days; pointer and value; reads (lances_do_clube)'),
          ('clube +0x53 nação, +0x57 liga, +0x64, +0x80 reputação, +0xd3 técnico, +0x19f/+0x1b3/+0x1cf as listas da comissão; valor e ponteiro; lê (lances_do_clube)',
           'club +0x53 nation, +0x57 league, +0x64, +0x80 reputation, +0xd3 manager, +0x19f/+0x1b3/+0x1cf the staff lists; value and pointer; reads (lances_do_clube)'),
          ('pessoa +0x24/+0x28 empréstimo de saída, +0x39 clube, +0x61 ficha (+0x45 moral do jogador); registro de 79 bytes +0xb (bits 0x30 e 4 baixos); lesão +0x2c dias; contrato +0x4e bit 2, +0x4f estatuto; [0xae1080]; lê, grava [0xae1080] (lances_do_clube)',
           'person +0x24/+0x28 outgoing loan, +0x39 club, +0x61 record (+0x45 player morale); 79-byte record +0xb (bits 0x30 and low 4); injury +0x2c days; contract +0x4e bit 2, +0x4f status; [0xae1080]; reads, writes [0xae1080] (lances_do_clube)')]
INTERESSA = ['CMRTE', 'GPF2']
