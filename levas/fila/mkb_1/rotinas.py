# O comprador da IA revendo o próprio lance depois da contraproposta do vendedor (transfer_manager.cpp), o mapa do
# mercado pedido pelo Rust Tweaks (levas/reorganizacao/mercado, leitor B, 11/11/2026):
# - 0x8c0810 (TRANSFER_MANAGER::review_ai_club_bid, 1840 bytes até a fronteira, 577 instruções; quadro SEH só com a
#   oferta local e o destrutor 0x8e5710): chamada pela process_due_offer 0x8c63a0 (oferta de transferência no estado 1
#   com comprador da IA, keep_existing 0), pela resposta automática das férias 0x784100, pela
#   handle_player_rejected_terms 0x8c28c0 e pelo diálogo de tirar jogador de troca 0x8f2870. 13 chamadas até
#   2002-06-01 na cobertura de 17/10 (docs/cobertura_simulacao.txt): rara, mas cada uma decide se o lance some ou volta.
# Igual no cm0102-1080.exe: a conferir pelo assinaturas.py na integração.
# Convenções conferidas nos pontos de chamada da original (desmontagem 0x8c0810..0x8c0f38):
# - quem chama: 0x8c685c push 0; push slot; mov ecx, TM -> thiscall(this, slot, keep_existing), ret 8.
# - dentro: 0x8e5680/0x8e5710 lea ecx, oferta -> thiscall, ret 0; 0x8c6220 push 0; push 1; push oferta -> thiscall,
#   ret 0xc; 0x417c10 push origem; push &saida; lea ecx, +0x12 -> thiscall, ret 8; 0x944cf1 push 0xe; add esp, 4 ->
#   cdecl; 0x68d020 mov ecx, nova -> thiscall, ret 0; 0x68d2f0 push lista; mov ecx, nova -> thiscall, ret 4; 0x849ce0
#   push 0; push (cargo != 11); push pessoa; push comprador; mov ecx, [TM+0x856] -> thiscall, ret 0x10; 0x8bfcb0 push 2;
#   push 0; push slot -> thiscall, ret 0xc; 0x840d80 push 1; push -1; push &saida; push vaga; push pessoa; push
#   comprador; mov ecx, [[0xde1f54]+0x856] -> thiscall, ret 0x18; 0x8e7b50 push 1 x4; push 0; lea ecx, oferta ->
#   thiscall, ret 0x14; 0x68d1b0 mov ecx, lista -> thiscall, ret 0; 0x4cf5a0 push pessoa; mov ecx, [0xae1050] ->
#   thiscall, ret 4; 0x4d1600 push 1; push 1; push 0; push pessoa; push clube; push comprador; mov ecx, [0xae1050] ->
#   thiscall, ret 0x18; 0x68d1f0 push item; mov ecx, lista -> thiscall, ret 4; 0x68d040 + 0x944c94 (cdecl); 0x843e10
#   push (cargo != 11); push 1; push comprador; push pessoa; mov ecx, [TM+0x856] -> thiscall, ret 0x10; 0x4cf5f0 push
#   vendedor; push pessoa -> thiscall, ret 8; 0x54cdb0 push +0x42; push +0x3e; mov ecx, 0xae2c90 -> thiscall, ret 8;
#   0x544600 push 0; push pessoa; add esp, 8 -> cdecl; 0x8676f0 mov ecx, contrato -> thiscall, ret 0; 0x600a50 5 push;
#   add esp, 0x14 -> cdecl; 0x4d0c60 push humano; push -1; push 0; push vendedor; push pessoa; mov ecx, [0xae1050] ->
#   thiscall, ret 0x14; 0x547d60/0x547d30 push vendedor; push pessoa; add esp, 8 -> cdecl; 0x90cfc0 cdecl; 0x867700
#   push 0; push 0x800000; mov ecx, contrato -> thiscall, ret 8; 0x8e7a70 push 1; push 0; push 1; push 0; lea ecx,
#   oferta -> thiscall, ret 0x10; 0x5a0590 push comprador; mov ecx, [0xae2a38] -> thiscall, ret 4; 0x598800 push 0;
#   push 1; push 0; push 0; push pessoa; mov ecx, finanças -> thiscall, ret 0x14 (i64 em edx:eax); 0x8be300 push 0;
#   push &oferta -> thiscall, ret 8.
# Determinismo: a oferta local vem do construtor 0x8e5680 sobre memória zerada na cópia (na original, sobra de pilha):
# o construtor escreve os campos que a rotina não copia; os nibbles altos de +0x30 e +0x31 ficam os do construtor
# nas duas. Os fatores f32 e as contas da prova do "jogador ouve o lance" em f64 (os produtos de um f32 por inteiro
# podem diferir do x87 estendido só numa fronteira de truncamento).
# Comparador: só a original (retira e submete ofertas, mexe nas vagas da lista e pode marcar o contrato).
SAVE = False
ROTINAS = [('review_ai_club_bid', 0x8c0810, [0x6a, 0xff, 0x68, 0x73, 0x3b, 0x96, 0x00])]
DOCS = ('o comprador da IA revendo o próprio lance depois da contraproposta do vendedor',
        "the AI buyer reviewing its own bid after the seller's counter offer",
        [('review_ai_club_bid', 'TRANSFER_MANAGER::review_ai_club_bid', '0x8c0810', 'transfer_manager.cpp', 'thiscall, ret 8',
          'o comprador da IA avalia o alvo pela vaga da lista dele (0x840d80); o preço é o valor ponderado da contraproposta mais o preço pedido de cada jogador de troca que caiba no valor (o que não cabe sai da oferta); preço acima do valor: retira o lance, marca a vaga e o jogador pode ficar inquieto (bit 0x800000 do contrato) se o lance passar de 17,5 milhões, de 1,5 × o nível dele (o valor para o vendedor escalado e vezes um fator de 0,9 a 2,5 pelas queixas, ambição, lealdade, gosto pelo clube e idade) ou, acima de 250 mil e do nível, com sorteio(temperamento + 1) = 0, desde que esteja no clube há mais de (lealdade + 42) × 5 + profissionalismo dias e o comprador tenha reputação 1250 acima do vendedor, ou ele esteja listado a pedido, ou com a marca 8 ou as queixas 0x14; dentro do valor: completa a taxa com o que os jogadores de troca tirados valiam (até o valor) e, se a verba de transferências cobre o total, refaz o lance (sem keep_existing); senão retira',
          "the AI buyer values the target from its shortlist slot (0x840d80); the price is the counter offer's weighted value plus the asking price of each exchange player that fits in the value (the others leave the offer); price above the value: the bid is withdrawn, the slot marked and the player may be unsettled (contract bit 0x800000) if the bid passes 17.5 million, 1.5 × his level (his value for the seller scaled and times a factor of 0.9 to 2.5 from grievances, ambition, loyalty, liking the club and age) or, above 250 thousand and the level, with random(temperament + 1) = 0, as long as he has been at the club more than (loyalty + 42) × 5 + professionalism days and the buyer's reputation is 1250 above the seller's, or he is listed by request, or with flag 8 or grievances 0x14; within the value: the fee is topped up with what the dropped exchange players were worth (up to the value) and, if the transfer budget covers the total, the bid is made again (without keep_existing); otherwise withdrawn")])
CAMPOS = [('oferta +0 (zerado na cópia local), +4 jogadores de troca (LLIST), +8 comprador, +0xc alvo, +0x11 cargo, +0x12 data, +0x1b taxa, +0x28 negociação (−1), +0x2c estado (0x10 sem negociação), +0x30 e +0x31 nibbles; lê e grava (review_ai_club_bid)',
           'offer +0 (zeroed in the local copy), +4 exchange players (LLIST), +8 buyer, +0xc target, +0x11 job, +0x12 date, +0x1b fee, +0x28 negotiation (−1), +0x2c status (0x10 without negotiation), +0x30 and +0x31 nibbles; reads and writes (review_ai_club_bid)'),
          ('TRANSFER_MANAGER +0xc negociações, +0x10 limite, +0x856 Shortlist_Manager; ponteiro e valor; lê (review_ai_club_bid)',
           'TRANSFER_MANAGER +0xc negotiations, +0x10 bound, +0x856 Shortlist_Manager; pointer and value; reads (review_ai_club_bid)'),
          ('pessoa +0x18 idade, +0x3e/+0x42 data de chegada ao clube, +0x57 ambição, +0x59 lealdade, +0x5b profissionalismo, +0x5d temperamento; contrato +4 clube, +0x35 tipo, +0x45 queixas (0x101, 0x14); clube +0x80 reputação; lê (review_ai_club_bid)',
           'person +0x18 age, +0x3e/+0x42 date joined club, +0x57 ambition, +0x59 loyalty, +0x5b professionalism, +0x5d temperament; contract +4 club, +0x35 type, +0x45 grievances (0x101, 0x14); club +0x80 reputation; reads (review_ai_club_bid)')]
INTERESSA = ['CMRTE']
