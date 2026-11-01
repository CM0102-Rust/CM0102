# O dia das transferências, a oferta que vence hoje e as restrições da oferta (transfer_manager.cpp):
# - 0x8c5880 (TRANSFER_MANAGER::daily_update, 1130 bytes, quadro SEH sem destrutor: as datas locais): a fatia do dia
#   das transferências, pelo laço do dia (0x5cd3ee): as ofertas vencidas reagendadas no dia múltiplo de 7, o balde da
#   fatia processado, na fatia 2 a base, as prestações, as regras das nações e, no dia 100, o +0x1e das pessoas
#   (o clube que a nação aceita como de formação): 966 chamadas na carreira até 2002-06-01 (cobertura 17/10).
# - 0x8c63a0 (TRANSFER_MANAGER::process_due_offer, 1852 bytes, quadro SEH com as listas locais da 0x68d020/0x68d040,
#   chamadas na mesma ordem): o despacho da oferta pelo status +0x2c: 4053 chamadas.
# - 0x8cfc90 (TRANSFER_MANAGER::check_offer_restrictions, 931 bytes, quadro SEH com a lista local): data de conclusão,
#   veto, orçamento × 1,1, vaga no elenco e a regra da nação: 2897 chamadas (pela 0x8c2f50, 0x8c34e0 e 0x8cbbe0, em
#   Rust, e pela notícia 0x784100).
# Iguais no cm0102-1080.exe (as três inteiras). Determinismo: nenhuma lê memória sem valor, endereço de pilha ou de
# heap como valor; as datas e listas locais são criadas pelas mesmas originais. Carreira Rust até 2001-09-01 numa cópia
# fora do projeto (89ce07b + a 266ª): as impressões das execuções com estas rotinas caem no mesmo conjunto de três
# estados (cf323a2c, cf816482, 6bc8050c) que a base sem elas e que uma DLL só com a notícia 0x8d3090 também dá: a
# variação entre execuções vem da base (a 266ª em correção), não destas rotinas.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x8c5880: 0x5cd3e2 mov ecx, [0xde1f54]; push dl (fatia, bytes altos com sobra: a cópia lê só o byte);
#   push 0xae2c90 -> thiscall(this, hoje, fatia), ret 8.
# - quem chama a 0x8c63a0: 0x8c5a2c push vaga; mov ecx, esi -> thiscall(this, vaga), ret 4.
# - quem chama a 0x8cfc90: 0x8c3238 (push 1; push &motivo; push decide; push oferta; mov ecx, esi), 0x8cbcdc (mov ecx,
#   edi; push clube; push oferta, com os outros dois antes) -> thiscall(this, oferta, decide, motivo, sem_elenco),
#   ret 0x10.
# - dentro da 0x8c5880: 0x417c10 push origem; push saída; mov ecx, destino -> thiscall, ret 8; 0x8c6bc0 push hoje;
#   mov ecx, esi -> thiscall, ret 4; 0x8c6220 push 0; push 1; push oferta; mov ecx, esi -> thiscall, ret 0xc; 0x8c18c0,
#   0x8caee0, 0x8c3eb0, 0x8c7330, 0x8cbbe0 mov ecx, esi -> thiscall, ret 0; 0x68d1b0 e 0x68d080 lea ecx, balde ->
#   thiscall, ret 0; 0x8c63a0 como acima; 0x54c5b0 push 1; push saída; mov ecx, data -> thiscall, ret 8; 0x54cfb0 mov
#   ecx, data -> thiscall, ret 0; o virtual [[regra]] push hoje; mov ecx, regra -> thiscall; 0x549ef0 push ano; push
#   0x64; lea ecx, local -> thiscall, ret 8; 0x53d740 push clube; push pessoa; add esp, 8 -> cdecl.
# - dentro da 0x8c63a0: 0x68d020/0x68d040 lea ecx, lista -> thiscall, ret 0; 0x68d2f0 push lista; lea ecx, local ->
#   thiscall, ret 4; 0x4cf5a0 push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4; 0x600a50 5 push; add esp, 0x14 ->
#   cdecl(clube, 0, 1, 0, 0); 0x8c6220 como acima (0xe, 0xa ou 0x1c dias); 0x84aeb0 push jogador; push clube; mov ecx,
#   [edi+0x856] -> thiscall, ret 8; 0x84af00 8 dwords (com o ecx = [edi+0x856] que ela não lê) -> stdcall(vendedor,
#   alvo, i64, comprador, i64, empréstimo), ret 0x20; os 4 últimos dwords da segunda chamada sobram dos 9 empurrados
#   para a 0x8e7b50 (mov ecx, oferta -> thiscall com 5 argumentos, ret 0x14); 0x8bfa30 e 0x8bf680 push 0; push 0;
#   push vaga; mov ecx, edi -> thiscall, ret 0xc; 0x4cc930 push oferta; mov ecx, [0xae1050] -> thiscall, ret 4;
#   0x8d76d0, 0x8d0f60, 0x8c2f50, 0x8d3090 push oferta; mov ecx, edi -> thiscall, ret 4; 0x8c0f40 push vaga; mov ecx,
#   edi -> thiscall, ret 4; 0x8d8310, 0x8c7690, 0x8c34e0 push 0; push oferta; mov ecx, edi -> thiscall, ret 8; 0x8c0080,
#   0x8c8090, 0x8c0810 push 0; push vaga; mov ecx, edi -> thiscall, ret 8; 0x8d0d90 push oferta; mov ecx, edi (não lido)
#   -> stdcall, ret 4; 0x8c1e40 push oferta; push pessoa; push clube; mov ecx, edi -> thiscall, ret 0xc; 0x7875e0 push
#   clube; mov ecx, [0xb7431c] (não lido) -> stdcall, ret 4.
# - dentro da 0x8cfc90: 0x54a110 lea ecx, local -> thiscall, ret 0; 0x8cee70 push &data; push oferta; mov ecx, ebp ->
#   thiscall, ret 8; 0x53b620 push byte (bytes altos com sobra; ela lê o byte); add esp, 4 -> cdecl; 0x8cd240 push 0;
#   push 0; push oferta (com o ecx, não lido) -> stdcall, ret 0xc; 0x8cdb80 6 push; mov ecx, ebp -> thiscall, ret 0x18;
#   0x5a0590 push comprador; mov ecx, [0xae2a38] -> thiscall, ret 4, e os 5 push de antes (alvo, 0, 0, 0, 0) ficam para
#   a 0x598800 (mov ecx, finanças -> thiscall, ret 0x14, devolve edx:eax); 0x43f4e0 push 4; lea ecx, dinheiro ->
#   thiscall, ret 4 (cdq depois: int); 0x8e7a70 push 0; push 1; push 1; push 0; mov ecx, oferta -> thiscall, ret 0x10;
#   0x600a50 como acima; 0x68d020/0x68d1b0/0x68d040 lea ecx, lista; 0x8420e0 push &lista; push comprador; mov ecx,
#   [ebp+0x856] -> thiscall, ret 8; 0x8cdd80 push 0; push comprador; push pessoa; mov ecx, ebp -> thiscall, ret 0xc;
#   0x8cd450 push motivo; push sem_elenco; push oferta; push clube; mov ecx, ebp -> thiscall, ret 0x10; o virtual
#   [[regra]+0xc] push motivo; push 0xae2c90; push decide; push oferta; mov ecx, regra -> thiscall.
# Comparadores: a 0x8c5880 só a original (processa as ofertas), conferindo o que ela garante: a fatia +0x855, o começo
# +0x84c e a base +0x84d e, no dia 100 da fatia 2, o +0x1e de cada pessoa que cumpre a condição (o laço é o último
# passo); a 0x8c63a0 e a 0x8cfc90 só a original (cada ramo reagenda, conclui ou tira a oferta; a vaga no elenco pode
# liberar um jogador).
SAVE = False
ROTINAS = [('dia_das_transferencias', 0x8c5880, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('processar_oferta_vencida', 0x8c63a0, [0x6a, 0xff, 0x68, 0x00, 0x3d, 0x96, 0x00]),
           ('restricoes_da_oferta', 0x8cfc90, [0x6a, 0xff, 0x68, 0xf8, 0x3f, 0x96, 0x00])]
DOCS = ('o dia das transferências: a fatia do dia (as ofertas vencidas, o balde da fatia, a base, as prestações, as regras das nações e a naturalização no dia 100), a oferta que vence hoje pelo status e as restrições da oferta (data, veto, orçamento, vaga no elenco e a regra da nação)',
        'the transfer day: the day slice (overdue offers, the slice bucket, the base date, instalments, nation rules and the naturalisation on day 100), the offer due today by its status and the offer restrictions (date, veto, budget, squad room and the nation rule)',
        [('dia_das_transferencias', 'TRANSFER_MANAGER::daily_update', '0x8c5880', 'transfer_manager.cpp', 'thiscall, ret 8',
          'uma fatia do dia das transferências: avisos das janelas, ofertas vencidas reagendadas, o balde da fatia processado; na fatia 2 a base passa a amanhã, as prestações no fim do mês, as regras das nações; no dia 100 o +0x1e das pessoas não comunitárias há anos no clube',
          'one slice of the transfer day: window notices, overdue offers rescheduled, the slice bucket processed; in slice 2 the base date moves to tomorrow, instalments at month end, nation rules; on day 100 the +0x1e of non-EU people at the club for years'),
         ('processar_oferta_vencida', 'TRANSFER_MANAGER::process_due_offer', '0x8c63a0', 'transfer_manager.cpp', 'thiscall, ret 4',
          'despacha a oferta que vence hoje pelo status: resposta do vendedor, aceitação, contrato ao jogador, permissões, conclusão, saída ou nova rodada da oferta',
          'dispatches the offer due today by its status: seller response, acceptance, player contract, permits, completion, removal or a new round of the offer'),
         ('restricoes_da_oferta', 'TRANSFER_MANAGER::check_offer_restrictions', '0x8cfc90', 'transfer_manager.cpp', 'thiscall, ret 0x10',
          'confere data de conclusão, veto da diretoria, orçamento (1,1 vez, mais as ofertas da lista de observação), vaga no elenco e a regra da nação; 0 libera, 1 ou 2 barram',
          'checks completion date, board veto, budget (1.1 times, plus the shortlist offers), squad room and the nation rule; 0 allows, 1 or 2 block')])
CAMPOS = [('gerência de transferências +0x84c começo dos baldes, +0x84d/+0x84f a base, +0x855 a fatia, +0x18 baldes (listas de 0xe bytes), +0x86e registros de 8 bytes (bit 0), +0x8ac as 28 regras (+0xa nação); valor; lê e grava (dia_das_transferencias, processar_oferta_vencida), lê (restricoes_da_oferta)',
           'transfer manager +0x84c bucket start, +0x84d/+0x84f the base date, +0x855 the slice, +0x18 buckets (0xe-byte lists), +0x86e 8-byte records (bit 0), +0x8ac the 28 rules (+0xa nation); value; reads and writes (dia_das_transferencias, processar_oferta_vencida), reads (restricoes_da_oferta)'),
          ('oferta +0 lista de ofertas, +4 troca, +8 comprador, +0xc alvo, +0x10, +0x11 tipo, +0x12/+0x14 data, +0x1a fatia, +0x1f empréstimo, +0x2c status, +0x2e, +0x2f, +0x30; id e valor; lê e grava (processar_oferta_vencida), lê (dia_das_transferencias, restricoes_da_oferta)',
           'offer +0 offer list, +4 exchange, +8 buyer, +0xc target, +0x10, +0x11 type, +0x12/+0x14 date, +0x1a slice, +0x1f loan, +0x2c status, +0x2e, +0x2f, +0x30; id and value; reads and writes (processar_oferta_vencida), reads (dia_das_transferencias, restricoes_da_oferta)'),
          ('pessoa (0x6e bytes) +0x1e segunda nacionalidade (a naturalização do dia 100), +0x39 jogador (+0x53 clube), +0x3e/+0x40 data, +0x61; ponteiro e valor; lê e grava +0x1e (dia_das_transferencias)',
           'person (0x6e bytes) +0x1e second nationality (the day-100 naturalisation), +0x39 player (+0x53 club), +0x3e/+0x40 date, +0x61; pointer and value; reads and writes +0x1e (dia_das_transferencias)'),
          ('[0xae1080] o último contrato; [0x9cfa10] a nação; [0x9cf488], [0x9cf434], [0x9cf360], [0x9cf2fc], [0x9cf448] clubes; ponteiro e id; grava [0xae1080], lê o resto',
           '[0xae1080] the last contract; [0x9cfa10] the nation; [0x9cf488], [0x9cf434], [0x9cf360], [0x9cf2fc], [0x9cf448] clubs; pointer and id; writes [0xae1080], reads the rest')]
INTERESSA = ['CMRTE']
