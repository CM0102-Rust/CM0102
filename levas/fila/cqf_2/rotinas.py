# Os termos aceitos pelo jogador e o empréstimo concluído na hora (transfer_manager.cpp), e a notícia do jogador posto
# na lista de transferências (contract_manager.cpp):
# - 0x8c2180 (TRANSFER_MANAGER::handle_player_accepted_terms, 1852 bytes, quadro SEH da data local sem destrutor): o
#   jogador (ou um jogador da troca) aceitou os termos do contrato (0x4cf413). Fora do empréstimo: a notícia da resposta
#   (0x8d8e90) e o seguimento da oferta (0x8c2f50). No empréstimo: o elenco cheio (mais de 50) ou a regra da nação
#   (vtable +0x20 do objeto de regras) recusam (0x8e23d0, status 0x10, 5 dias); senão o empréstimo na hora: termos,
#   contrato assinado, histórico, números, o registro da transferência passada (0x8cb810), status 0x14 e 5 dias. 1123
#   chamadas na carreira até 2002-06-01 (cobertura 17/10).
# - 0x4d71b0 (CONTRACT_MANAGER::post_transfer_listed_news, 1469 bytes, quadro SEH da lista local): a notícia 0xbea do
#   jogador posto na lista de transferências, com o preço (o pedido 0x4d1600 vezes um fator sorteado, arredondado) e
#   os até 3 interessados mais fortes; sorteia duas vezes (0x90cfc0 com 2 e 0x29). 340 chamadas na carreira.
# Iguais no cm0102-1080.exe (as duas inteiras).
# Determinismo: na 0x8c2180, a resposta (2) que vai à 0x8d8e90 num dword com os bytes altos de sobra da pilha (a
# 0x8d8e90 lê o byte; a cópia passa 2); nos caminhos de erro (sem troca ou sem o nó da pessoa) os clubes A e B a original
# deixa sem valor, a cópia 0 (não acontece); o registro local da transferência passada é preenchido inteiro (0x25 bytes).
# Na 0x4d71b0, o lugar da última linha da notícia vai num dword com os bytes altos do preço (a 0x785790 lê o byte).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x8c2180: 0x4cf413 (5 push; mov ecx, [0xde1f54]) -> thiscall(this, pessoa, vaga, contrato, tipo,
#   noticia), ret 0x14.
# - quem chama a 0x4d71b0: 0x4d8777 (push clube; push jogador; mov ecx, ebp), 0x784893 e 0x8423da (mov ecx,
#   [0xae1050]), 0x4dd0bd -> thiscall(this, jogador, clube), ret 8.
# - dentro da 0x8c2180: a caixa "Error" (aviso_em); 0x4cf5a0 push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4;
#   0x68d1b0 mov ecx, lista -> thiscall, ret 0; 0x8d8e90 8 push; mov ecx, ebp -> thiscall(this, pessoa, A, B, 2,
#   oferta, 0, noticia, 0), ret 0x20; 0x8c2f50 push oferta; mov ecx, ebp -> thiscall, ret 4; 0x53c590 push 0; push 0;
#   push A; add esp, 0xc -> cdecl (movsx al); a regra: mov ecx, regra; push &motivo; push 0; push alvo; push comprador;
#   call [vtable+0x20] -> thiscall, ret 0x10, e push oferta; call [vtable+4] ou [+8] -> thiscall, ret 4; 0x8e23d0 push
#   motivo; push oferta; mov ecx, ebp -> thiscall, ret 8; 0x8c6db0 push oferta; mov ecx, ebp -> thiscall, ret 4;
#   0x84aeb0 push pessoa; push clube; mov ecx, [ebp+0x856] -> thiscall, ret 8; 0x8c6220 push 0; push 5; push oferta;
#   mov ecx, ebp -> thiscall, ret 0xc; 0x8e1f10 push contrato; push oferta; mov ecx, ebp -> thiscall, ret 8; 0x4d01e0
#   push contrato; mov ecx, [0xae1050] -> thiscall, ret 4; 0x86da20 push A; push B; push pessoa; mov ecx, [0xdd7f00] ->
#   thiscall, ret 0xc; 0x842080 push A; push pessoa; mov ecx, [ebp+0x856] -> thiscall, ret 8 (al); 0x843e10 push 0;
#   push 5; push A; push pessoa -> thiscall, ret 0x10; 0x5563f0 push alvo; mov ecx, [0xae28f0] -> thiscall, ret 4;
#   0x85a850 push 0; push A; push alvo; mov ecx, [0xdd7ef0] -> thiscall, ret 0xc; 0x540980 push 1; push pessoa (cdecl,
#   add esp, 0x1c com a 0x600a50 push 0; push 0; push 1; push 0; push A); 0x85add0 push 0; push A; mov ecx, [0xdd7ef0]
#   -> thiscall, ret 8; 0x85a470 push pessoa; push A -> thiscall, ret 8; 0x4196a0 push pessoa; mov ecx, [0xad9c6c] ->
#   thiscall, ret 4; 0x54a110 lea ecx, data -> thiscall, ret 0; 0x417c10 push 0xae2c90; push &temp; lea ecx, data ->
#   thiscall, ret 8; 0x8cb810 push &registro; mov ecx, ebp -> thiscall, ret 4; 0x842b00 push 1; push pessoa; mov ecx,
#   [ebp+0x856] -> thiscall, ret 8.
# - dentro da 0x4d71b0: 0x4cf5f0 push clube; push jogador; ecx = this -> thiscall, ret 8; 0x68d020 / 0x68d040 lea ecx,
#   lista -> thiscall, ret 0; 0x77bbe0 push 0; push 0xbea; lea ecx, notícia -> thiscall, ret 8; 0x785790 push valor; push
#   lugar; lea ecx, notícia -> thiscall, ret 8; 0x5b7d20 push ficha; add esp, 4 -> cdecl (movsx ax); 0x90cfc0 push 2 /
#   0x29; add esp, 4 -> cdecl; 0x5444a0 push clube; push jogador (cdecl, add esp, 0x1c com a 0x600a50); 0x4d1600 push 1;
#   push 1; push 0; push jogador; push 0; push clube; mov ecx, this -> thiscall, ret 0x18; 0x945640 _ftol; 0x842180
#   push 0; push 1; push 1; push &lista; push jogador; mov ecx, [[0xde1f54]+0x856] -> thiscall, ret 0x14; 0x5a0590 push
#   clube; mov ecx, [0xae2a38] -> thiscall, ret 4; 0x598800 push 0; push 1; push 0; push 0; push 0; mov ecx, finanças ->
#   thiscall, ret 0x14 (edx:eax); 0x8d1da0 push 0x1e; push &clubes; push 0x21a; push jogador; mov ecx, [0xde1f54] -> thiscall, ret 0x10 (al; a
#   0x8d1da0 faz mov esi, ecx: o mapa da comunidade a dá como stdcall, e a cópia a chamava assim e caía no modo Rust);
#   0x7861e0 push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8.
# Comparadores: só a original nas duas (a 0x8c2180 entrega notícias, assina contrato, conclui e registra o
# empréstimo; a 0x4d71b0 sorteia e entrega a notícia, que aloca).
# Provas (02/11, cópia fora do projeto com o motor.rs da principal = 268ª + cqc_6, mais cqf_1..3 e cqg_1): jogo comparando
# até 2001-09-01 em ENG (0 divergências, nenhuma linha DIVERGENCIA) e em ENG ESP ITA FRA (0 do total; uma linha
# DIVERGENCIA da aviso_ao_rival_de_posicao, a mesma com a DLL sem os itens: anterior a eles); carreira Rust até 2002-06-01
# duas vezes iguais e o detector de lixo igual (impressão 825918943f68aa97, 473125915 sorteios, a mesma da base sem os itens).
SAVE = False
ROTINAS = [('termos_aceitos', 0x8c2180, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_de_jogador_listado', 0x4d71b0, [0x6a, 0xff, 0x68, 0x16, 0x6f, 0x95, 0x00])]
DOCS = ('os termos aceitos pelo jogador (a notícia da resposta e o seguimento da oferta; no empréstimo, a recusa pelo elenco cheio ou pela regra da nação, ou a conclusão na hora com o registro da transferência) e a notícia do jogador posto na lista de transferências (o preço sorteado e os interessados mais fortes)',
        'the terms accepted by the player (the reply news and the offer follow-up; for loans, refusal for a full squad or by the nation rule, or completion at once with the transfer record) and the news of a player put on the transfer list (the drawn price and the strongest interested clubs)',
        [('termos_aceitos', 'TRANSFER_MANAGER::handle_player_accepted_terms', '0x8c2180', 'transfer_manager.cpp', 'thiscall, ret 0x14',
          'marca o acordo do alvo (ou do jogador da troca); fora do empréstimo, a notícia da resposta e o seguimento da oferta; no empréstimo, elenco acima de 50 ou a regra da nação recusam, senão conclui na hora (contrato, histórico, números, registro, status 0x14 em 5 dias)',
          'marks the agreement of the target (or the exchange player); outside loans, the reply news and the offer follow-up; for loans, a squad above 50 or the nation rule refuse, otherwise completes at once (contract, history, numbers, record, status 0x14 in 5 days)'),
         ('noticia_de_jogador_listado', 'CONTRACT_MANAGER::post_transfer_listed_news', '0x4d71b0', 'contract_manager.cpp', 'thiscall, ret 8',
          'a notícia 0xbea do jogador na lista de transferências: o preço (o valor, ou o pedido vezes um fator sorteado, arredondado) e de 500 mil para cima os até 3 interessados mais fortes; entregue aos clubes humanos interessados',
          'news 0xbea of a player on the transfer list: the price (the value, or the asking price times a drawn factor, rounded) and from 500 thousand up the up to 3 strongest interested clubs; posted to the interested human clubs')])
CAMPOS = [('oferta +0 lista das ofertas pelo jogador, +4 troca (nós de 7 bytes: +0 id, +4 acordo, +6 tipo), +8 comprador, +0xc alvo, +0x10 acordo do alvo, +0x11 tipo, +0x1f empréstimo, +0x2c status, +0x2e; id e valor; lê e grava (termos_aceitos)',
           'offer +0 list of offers for the player, +4 exchange (7-byte nodes: +0 id, +4 agreement, +6 type), +8 buyer, +0xc target, +0x10 target agreement, +0x11 type, +0x1f loan, +0x2c status, +0x2e; id and value; reads and writes (termos_aceitos)'),
          ('gerência de transferências +0 vetor das ofertas, +4 capacidade, +0x856 listas de observação, +0x86e registro de 8 bytes por pessoa (bit 0), +0x8ac os 28 objetos de regras por nação (+0xa a nação); ponteiro e valor; lê, grava o bit 0 (termos_aceitos)',
           'transfer manager +0 offer array, +4 capacity, +0x856 shortlists, +0x86e 8-byte record per person (bit 0), +0x8ac the 28 per-nation rule objects (+0xa the nation); pointer and value; reads, writes bit 0 (termos_aceitos)'),
          ('pessoa +0 id, +4/+8/+0xc ponteiros com o id em +0x33, +0x52 valor, +0x61 ficha de jogador, +0x6d situação (2 vira 0x10); id e valor; lê, grava +0x6d (termos_aceitos, noticia_de_jogador_listado)',
           'person +0 id, +4/+8/+0xc pointers with the id at +0x33, +0x52 value, +0x61 player record, +0x6d status (2 becomes 0x10); id and value; reads, writes +0x6d (termos_aceitos, noticia_de_jogador_listado)'),
          ('clube +0x53 nação, +0x80 reputação, +0xcf comissão (+0 id, +4/+8/+0xc com o id em +0x33); id e valor; lê (termos_aceitos, noticia_de_jogador_listado)',
           'club +0x53 nation, +0x80 reputation, +0xcf board (+0 id, +4/+8/+0xc with the id at +0x33); id and value; reads (termos_aceitos, noticia_de_jogador_listado)'),
          ('contrato +4 clube, +0x35 tipo (6 bits), +0x45, +0x4f nibble alto; id e valor; lê (termos_aceitos, noticia_de_jogador_listado)',
           'contract +4 club, +0x35 type (6 bits), +0x45, +0x4f high nibble; id and value; reads (termos_aceitos, noticia_de_jogador_listado)')]
INTERESSA = ['CMRTE']
