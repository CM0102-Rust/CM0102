# O lance aceito pelo clube vendedor, a notícia da oferta feita e o empréstimo possível (transfer_manager.cpp):
# - 0x8c0080 (TRANSFER_MANAGER::accept_transfer_bid, 1501 bytes, quadro SEH sem destrutor no caminho normal: o
#   STAFF_CONTRACT local da 0x85e410): status 5, a recusa em se mudar no empréstimo (interesse < 0,75), os jogadores da
#   troca caros demais (salário desejado acima de 1,5 teto) saindo com o valor somado ao preço, e a oferta reagendada:
#   748 chamadas na carreira até 2002-06-01 (cobertura 17/10), pela oferta que vence hoje (0x8c63a0, cqd_1) e pela
#   resposta da IA (0x8c8090).
# - 0x8d3090 (TRANSFER_MANAGER::post_offer_made_news, 1167 bytes, quadro SEH sem destrutor no caminho normal: a notícia e
#   a data locais): a notícia 0xfa0 da oferta feita aos técnicos humanos interessados; passageira (só notícia): 1502
#   chamadas, pela oferta nova (0x8be300) e pela 0x8c63a0.
# - 0x8d0040 (TRANSFER_MANAGER::can_loan_player, 1263 bytes, quadro SEH sem destrutor: a data e os textos locais): o
#   clube pode pegar o jogador emprestado (clube atual, ficha, cargo, 0x8ce990, as regras das duas nações e o período
#   mês a mês pelo virtual +0x24): 309108 chamadas, pelas listas de observação da IA (0x83fe1f, 0x84aa2b; sem mensagem).
# Iguais no cm0102-1080.exe (as três inteiras). Determinismo: o STAFF_CONTRACT, a notícia e os vetores locais são
# preenchidos pelas mesmas originais; os dwords que a original empurra com bytes altos sem valor (o estatuto da 0x85fc50,
# o tipo da 0x843e10, o cargo da 0x5968d0) vão aqui só com o byte que o chamado lê (conferido: mov cl, [esp+0x284] na
# 0x85fc50; `tipo as u8` na 0x843e10; `cargo_arg as u8` na 0x5968d0), e o motivo da 0x846920 começa zerado como ela
# mesma grava (mov [eax], ebx em 0x846999).
# Carreira Rust até 2001-09-01 numa cópia fora do projeto (89ce07b + a 266ª): as impressões com estas rotinas caem no
# mesmo conjunto de estados (cf323a2c, cf816482, 6bc8050c) que a base sem elas dá de uma execução para outra (a base
# com a 266ª não sai igual entre execuções nesta janela); nenhum estado novo.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x8c0080: 0x8c6a40 (push 0; push vaga; mov ecx, edi) e 0x8c8090 -> thiscall(this, vaga, marcar), ret 8.
# - quem chama a 0x8d3090: 0x8c6a4c push oferta; mov ecx, edi -> thiscall(this, oferta), ret 4.
# - quem chama a 0x8d0040: 0x84aa21 (mov ecx, [0xde1f54]; push 0; push jogador; push clube), 0x83fe17 -> thiscall(this,
#   clube, jogador, mensagem), ret 0xc.
# - dentro da 0x8c0080: 0x85e410 lea ecx, contrato -> thiscall, ret 0; 0x4cf5a0 push pessoa; mov ecx, [0xae1050] ->
#   thiscall, ret 4; 0x84b580 push lista; push comprador; push alvo; mov ecx, [ebx+0x856] -> thiscall, ret 0xc; 0x7875e0
#   push clube (ecx = [0xb7431c], não lido) -> stdcall, ret 4; 0x8d76d0 push oferta; mov ecx, ebx -> thiscall, ret 4;
#   0x600a50 5 push; add esp, 0x14 -> cdecl; 0x849f00 push 0; push comprador; push alvo; mov ecx, [ebx+0x856] ->
#   thiscall, ret 0xc; 0x846920 push &motivo; push estatuto; push 0; push 0; push comprador; push alvo; mov ecx,
#   [ebx+0x856] -> thiscall, ret 0x18, devolve ST0 (fcomp [0x969040] = 0,75); 0x8d8310 push motivo; push oferta; mov ecx,
#   ebx -> thiscall, ret 8; 0x84aeb0 push jogador; push clube; mov ecx, [ebx+0x856] -> thiscall, ret 8; 0x8c6220 push
#   fatias; push dias; push oferta; mov ecx, ebx -> thiscall, ret 0xc; 0x8d0d90 push oferta (ecx não lido) -> stdcall,
#   ret 4; 0x849ce0 push 0; push tipo; push pessoa; push clube; mov ecx, [ebx+0x856] -> thiscall, ret 0x10; 0x843e10
#   push tipo; push 1; push clube; push pessoa; mov ecx, [ebx+0x856] -> thiscall, ret 0x10; 0x85fc50 push 0; push 0;
#   push 0xb; push estatuto; push −1; push vendedor; push pessoa; lea ecx, contrato -> thiscall, ret 0x1c; 0x5a0590 push
#   vendedor; mov ecx, [0xae2a38] -> thiscall, ret 4, com os 2 push de antes para a 0x5968d0 (push cargo; push pessoa;
#   mov ecx, finanças -> thiscall, ret 8); 0x4d0c60 push 0; push −1; push 0; push comprador; push pessoa; mov ecx,
#   [0xae1050] -> thiscall, ret 0x14; 0x8d8910 push valor; push pessoa; push oferta; mov ecx, ebx -> thiscall, ret 0xc;
#   0x68d1f0 push nó; mov ecx, lista -> thiscall, ret 4; 0x8c8090 push 1; push vaga; mov ecx, ebx -> thiscall, ret 8;
#   0x68d040 mov ecx, lista -> thiscall, ret 0; 0x944c94 push; add esp, 4 -> cdecl; 0x8d7cd0 push oferta; mov ecx, ebx
#   -> thiscall, ret 4; 0x68d1b0 mov ecx, lista -> thiscall, ret 0.
# - dentro da 0x8d3090: 0x4cf5a0 como acima (a segunda sem guardar em [0xae1080]); 0x8d1130 push 0x1e; push &vetor; push
#   0x3e; push oferta; mov ecx, ebp -> thiscall, ret 0x10; 0x77bbe0 push 0; push 0xfa0; lea ecx, notícia -> thiscall,
#   ret 8; 0x785790 push v; push k; lea ecx, notícia -> thiscall, ret 8; 0x8e7a70 push 1; push 0; push 1; push 0; mov
#   ecx, oferta -> thiscall, ret 0x10; 0x53b620 push byte; add esp, 4 -> cdecl; 0x5409b0 push seleção; add esp, 4 ->
#   cdecl; 0x4486c0 push 1; push oferta; mov ecx, [0xad9dac] -> thiscall, ret 8; 0x7861e0 push clube; push &notícia;
#   mov ecx, [0xb7431c] -> thiscall, ret 8; 0x54a110 lea ecx, data -> thiscall, ret 0; 0x8d0bb0 push 0; push &data (ecx
#   não lido) -> stdcall, ret 8.
# - dentro da 0x8d0040: 0x4cf5a0 como acima; 0x54a110 lea ecx, data -> thiscall, ret 0; 0x54cf70 mov ecx, 0xae2c90 ->
#   thiscall, ret 0; 0x4cf6d0 push jogador; mov ecx, [0xae1050] -> thiscall, ret 4; 0x53dba0 push 1; push 0x12c; push
#   texto; push jogador; add esp, 0x10 -> cdecl; 0x66f4e0 push ...; add esp, n -> cdecl com reticências; 0x8ce990 push
#   0; push &motivo; push clube; push jogador; mov ecx, esi -> thiscall, ret 0x10; 0x4cca30 push id; mov ecx, [0xae1050]
#   -> thiscall, ret 4; 0x6181d0 push 0xd; push 0x64; push texto; push nação; add esp, 0x10 -> cdecl; o virtual
#   [[regra]+0x20] push 0; push mensagem; push jogador; push clube; mov ecx, regra (4 dwords); 0x8d0590 push &fim_ano;
#   push &fim_mês; push &início_ano; push &início_mês; push jogador; push clube do contrato; push clube; mov ecx, this
#   (o [esp+0x14] guardado) -> thiscall, ret 0x1c; 0x54a130 push −1; push ano; push mês; push dia; lea ecx, data ->
#   thiscall, ret 0x10 (lê o byte do mês e a palavra do ano: and ecx, 0xffff); o virtual [[regra]+0x24] push 1; push
#   &data; push 0; push jogador; push clube atual; mov ecx, regra (5 dwords); 0x549f70 push −1; push fim_ano; push
#   fim_mês; push dia; lea ecx, local -> thiscall, ret 0x10.
# Comparadores: só a original na 0x8c0080 (muda o status, reagenda, solta a troca, chama a resposta da IA e aloca
# notícias) e na 0x8d3090 (só entrega a notícia); a 0x8d0040 lado a lado: a devolução, [0xae1080], o texto em
# [0xde1f64], a mensagem (0x100 bytes) e o byte do jogador em [[0xde1f54]+0x86e] (a réplica sem contar sorteios).
# Determinismo da 0x8d0040: a original lê os quatro meses e anos do período como dwords com sobra de pilha nos bytes que
# a 0x8d0590 não grava e empurra o mês com os bytes altos do ponteiro this; os chamados só leem o byte e a palavra.
SAVE = False
ROTINAS = [('aceitar_lance', 0x8c0080, [0x6a, 0xff, 0x68, 0x48, 0x3b, 0x96, 0x00]),
           ('noticia_da_oferta_feita', 0x8d3090, [0x6a, 0xff, 0x68, 0xc6, 0x42, 0x96, 0x00]),
           ('pode_emprestar', 0x8d0040, [0x6a, 0xff, 0x68, 0x1b, 0x40, 0x96, 0x00])]
DOCS = ('o lance aceito pelo clube vendedor (a recusa em se mudar no empréstimo, os jogadores da troca caros demais e a oferta reagendada), a notícia da oferta feita e o empréstimo possível',
        'the bid accepted by the selling club (the refusal to move on loans, exchange players too expensive and the rescheduled offer), the news of the offer made and whether a loan is possible',
        [('aceitar_lance', 'TRANSFER_MANAGER::accept_transfer_bid', '0x8c0080', 'transfer_manager.cpp', 'thiscall, ret 8',
          'o vendedor aceita o lance: status 5; no empréstimo, interesse do alvo abaixo de 0,75 dá a recusa em se mudar; os jogadores da troca com salário desejado acima de 1,5 teto do vendedor saem, com o valor somado ao preço; a oferta vai à fatia seguinte',
          'the seller accepts the bid: status 5; on loans, target interest below 0.75 gives the refusal to move; exchange players wanting more than 1.5 times the seller wage cap leave, with their value added to the fee; the offer goes to the next slice'),
         ('noticia_da_oferta_feita', 'TRANSFER_MANAGER::post_offer_made_news', '0x8d3090', 'transfer_manager.cpp', 'thiscall, ret 4',
          'a notícia 0xfa0 da oferta feita (alvo, clubes, valores, troca, contrato, janela) aos técnicos humanos interessados',
          'news 0xfa0 of an offer made (target, clubs, values, exchange, contract, window) to the interested human managers'),
         ('pode_emprestar', 'TRANSFER_MANAGER::can_loan_player', '0x8d0040', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o clube pode pegar o jogador emprestado: clube atual, ficha de contrato, cargo de jogador, se o clube chega a ele, a regra da nação do clube e a do clube atual mês a mês no período do empréstimo; o texto do motivo na mensagem',
          'whether the club can take the player on loan: current club, contract record, playing role, whether the club can reach him, the rule of the club nation and of the current club month by month over the loan period; the reason text in the message')])
CAMPOS = [('oferta +0 lista, +4 troca (nós com +5 estatuto), +8 comprador, +0xc alvo, +0x11 tipo, +0x1b preço, +0x20, +0x24, +0x2c status, +0x2d, +0x2e, +0x2f, +0x30, +0x31; id e valor; lê e grava (aceitar_lance), lê (noticia_da_oferta_feita)',
           'offer +0 list, +4 exchange (nodes with +5 status), +8 buyer, +0xc target, +0x11 type, +0x1b fee, +0x20, +0x24, +0x2c status, +0x2d, +0x2e, +0x2f, +0x30, +0x31; id and value; reads and writes (aceitar_lance), reads (noticia_da_oferta_feita)'),
          ('gerência de transferências +0x86e registros de 8 bytes (bit 0 do +0, contagem +2), +0x856 a lista de observação; valor; grava (aceitar_lance)',
           'transfer manager +0x86e 8-byte records (bit 0 of +0, count +2), +0x856 the shortlist manager; value; writes (aceitar_lance)'),
          ('entrada da lista de observação +0 cargo, +0x17 índice da oferta; valor; lê e grava (aceitar_lance)',
           'shortlist entry +0 role, +0x17 offer index; value; reads and writes (aceitar_lance)'),
          ('regras de nação (as 28 em +0x8ac) +0xa nação, virtuais +0x20 e +0x24; pessoa +0x3d cargo; clube +0x38 nome, +0x52, +0x53 nação; ponteiro e valor; lê (pode_emprestar)',
           'nation rules (the 28 at +0x8ac) +0xa nation, virtuals +0x20 and +0x24; person +0x3d role; club +0x38 name, +0x52, +0x53 nation; pointer and value; reads (pode_emprestar)'),
          ('contrato +0x2d/+0x2f fim, +0x35 tipo; [0xae1080] o último contrato; valor e ponteiro; lê (noticia_da_oferta_feita), grava [0xae1080] (as duas)',
           'contract +0x2d/+0x2f end, +0x35 type; [0xae1080] the last contract; value and pointer; reads (noticia_da_oferta_feita), writes [0xae1080] (both)')]
INTERESSA = ['CMRTE']
