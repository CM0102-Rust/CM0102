# Os jogos da rodada da copa levados às datas marcadas, o jogador que sai da troca e as permissões de trabalho da
# oferta (Cup.cpp, transfer_manager.cpp):
# - 0x51ebe0 (Cup::move_round_fixtures_to_set_dates, ~1,8 KB, quadro SEH sem destrutor no caminho normal): nas rodadas
#   com datas marcadas (bit 0x100 em +0xd), leva cada jogo da rodada ao dia da semana e à fatia da vaga dele (sai e
#   volta ao calendário), pela montagem da rodada 0x51c800 (montar_rodada_da_copa, em Rust; "cps_13 round matches"
#   na base): 68 chamadas na carreira automática até 2002-06-01 (docs/cobertura_simulacao.txt, 17/10).
# - 0x8c41c0 (TRANSFER_MANAGER::remove_exchange_player, 798 bytes, quadro SEH sem destrutor no caminho normal): tira o
#   jogador da lista de troca de uma oferta (ou das outras), soma o valor dele ao preço e a oferta segue; pela oferta
#   de contrato ao jogador (0x8c1e40), pela confirmação pelo clube (0x8c3310) e pela soltura dos jogadores (0x8c6ae0),
#   já em Rust: 882 chamadas na carreira.
# - 0x8c7690 (TRANSFER_MANAGER::check_offer_work_permits, 691 bytes, sem SEH): a permissão de trabalho do alvo e dos
#   jogadores da troca; recusada a do alvo, a oferta cai; pelo processamento da oferta (status 0xa, 0x8c6880) e pela
#   conclusão (0x8cbc40): 18 chamadas na carreira.
# Iguais no cm0102-1080.exe (as três inteiras). A 0x8d1130 (TRANSFER_MANAGER::collect_offer_news_recipients, 3,2 KB
# com x87, registro local para a 0x5407e0 e a lista da 0x842180) ficou de fora: só escolhe os destinatários das notícias
# (sem efeito na simulação para as mutações) e não coube nesta rodada.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x51ebe0: 0x51cab5 push rodada; push data; mov ecx, esi -> thiscall(this, data, rodada), ret 8.
# - quem chama a 0x8c41c0: 0x8c1f25 (push 0x16; push 1; push oferta; push pessoa; mov ecx, [0xde1f54]), 0x8c3403,
#   0x8c343f, 0x8c6b2e (push ...; push pessoa; mov ecx, ebp/ebx) -> thiscall(this, pessoa, oferta, só_esta, motivo),
#   ret 0x10.
# - quem chama a 0x8c7690: 0x8c6951 (push 0; push oferta; mov ecx, edi) e 0x8cbcb3 (push 1) -> thiscall(this, oferta,
#   quieto), ret 8.
# - dentro da 0x51ebe0: 0x77bbe0 push 0; push 0x14 (ou 3); lea ecx, notícia -> thiscall, ret 8; 0x68d020/0x68d040/
#   0x68d1b0 lea ecx, lista -> thiscall, ret 0; 0x5abd90 push lista; push tipo; push rodada; push id; push fatia; push
#   dia; push ano; mov ecx, [0xae2a58] -> thiscall, ret 0x1c; 0x785790 push v; push k; lea ecx -> thiscall, ret 8;
#   0x944cf1 push 8; add esp, 4 -> cdecl; 0x549ef0 push ano; push dia; mov ecx, bloco -> thiscall, ret 8; 0x54a110 lea
#   ecx -> thiscall, ret 0; 0x54a2d0 push destino; push dia da semana; mov ecx, bloco -> thiscall, ret 8; 0x7861e0 push
#   clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8; 0x5abc30 push &jogo; mov ecx, [0xae2a58] ->
#   thiscall, ret 4; 0x85c260 push +0x24; push &jogo; push regras da vaga; mov ecx, [0xdd7ef4] -> thiscall, ret 0xc;
#   0x5aa680 push 1; push &jogo; mov ecx, [0xae2a58] -> thiscall, ret 8; 0x944c94 push bloco; add esp, 4 -> cdecl;
#   0x7875e0 push clube; mov ecx, [0xb7431c] -> thiscall, ret 4.
# - dentro da 0x8c41c0: 0x4cf5a0 push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4; 0x68d1f0 push item; mov ecx,
#   lista -> thiscall, ret 4; 0x8de150 push motivo; push valor; push pessoa; push oferta; mov ecx, ebp -> thiscall, ret
#   0x10; 0x600a50 push 0; push 0; push 1; push 0; push clube; add esp, 0x14 -> cdecl; 0x8c8090 push 1; push &oferta;
#   mov ecx, ebp -> thiscall(this, vaga, 1), ret 8; 0x8c2f50 push oferta; mov ecx, ebp -> thiscall, ret 4; 0x8c5e40
#   push &lista; push pessoa; mov ecx, ebp -> thiscall, ret 8; 0x68d040 mov ecx, lista + 0x944c94 cdecl.
# - dentro da 0x8c7690: 0x8c7560 push 0; push clube; push pessoa; mov ecx, edi -> thiscall, ret 0xc; 0x4cc9e0 push id
#   do clube; push id da pessoa; mov ecx, [0xae1050] -> thiscall, ret 8; 0x8c7950 push 0; push 0; push proposta; push
#   nação; push pessoa; mov ecx, edi -> thiscall, ret 0x14; 0x8de790 push 0x13; push clube; push oferta; mov ecx, edi
#   -> thiscall, ret 0xc; 0x8c6db0 push oferta; mov ecx, edi -> (stdcall na cópia Rust, o ecx não é lido), ret 4;
#   0x84aeb0 push pessoa; push clube; mov ecx, [edi+0x856] -> thiscall, ret 8; 0x8c6220 push 0; push 0xa; push oferta;
#   mov ecx, edi -> thiscall, ret 0xc; 0x8e3130 push oferta; push clube; push pessoa; mov ecx, edi -> thiscall, ret
#   0xc; 0x8e35a0 push valor; push pessoa; push oferta; mov ecx, edi -> thiscall, ret 0xc; 0x68d1b0/0x68d1f0/0x68d040,
#   0x944c94, 0x4cf5a0 e 0x8c2f50 como acima.
# Comparadores: a 0x51ebe0 só a original (o calendário aloca e as notícias vão aos técnicos), com o plano: os jogos da
# copa na data e fatia (0x5abd90 numa lista provisória) ocupam as vagas na ordem, e a vaga de cada jogo na data da
# rodada fica 0xff (as outras não mudam); a 0x8c41c0 só a original (a oferta segue pela 0x8c2f50, que pode fechar a
# transferência e tirar outros jogadores das trocas, e o vendedor do computador pode responder pela 0x8c8090); a
# 0x8c7690 só a original, com o plano pelas consultas sem efeito (0x8c7560, 0x4cc9e0, 0x8c7950 sem sorteio): o alvo
# recusado devolve 0 com a oferta no status 0x10, senão 1.
SAVE = False
ROTINAS = [('copa_levar_jogos_as_datas', 0x51ebe0, [0x6a, 0xff, 0x68, 0x5a, 0x77, 0x95, 0x00]),
           ('tirar_jogador_da_troca', 0x8c41c0, [0x6a, 0xff, 0x68, 0x68, 0x3c, 0x96, 0x00]),
           ('permissoes_da_oferta', 0x8c7690, [0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14])]
DOCS = ('a copa e as transferências: os jogos da rodada levados ao dia da semana e à fatia das vagas marcadas, o jogador que sai da troca de uma oferta e as permissões de trabalho da oferta',
        'the cup and transfers: the round fixtures moved to the weekday and time slot of the set slots, the player removed from an offer part exchange and the offer work permits',
        [('copa_levar_jogos_as_datas', 'Cup::move_round_fixtures_to_set_dates', '0x51ebe0', 'Cup.cpp', 'thiscall, ret 8',
          'nas rodadas de datas marcadas, cada jogo da rodada vai ao dia da semana e à fatia da sua vaga, com as notícias, o estádio e o calendário; a vaga usada fica vazia',
          'in rounds with set dates, each round fixture moves to the weekday and time slot of its slot, with the news, the stadium and the calendar; the used slot becomes empty'),
         ('tirar_jogador_da_troca', 'TRANSFER_MANAGER::remove_exchange_player', '0x8c41c0', 'transfer_manager.cpp', 'thiscall, ret 0x10',
          'tira o jogador da lista de troca da oferta (ou das outras ofertas dele), o valor dele passa ao preço, a notícia, a resposta do vendedor do computador e a oferta segue',
          'removes the player from the offer part exchange list (or from his other offers), his value goes to the price, the news, the computer seller reply and the offer goes on'),
         ('permissoes_da_oferta', 'TRANSFER_MANAGER::check_offer_work_permits', '0x8c7690', 'transfer_manager.cpp', 'thiscall, ret 8',
          'a permissão de trabalho do alvo no comprador (recusada: a oferta cai, status 0x10, reagendada em 10 dias, devolve 0) e a de cada jogador da troca no vendedor (recusado: sai da troca e o valor vai ao preço); as notícias e a oferta segue',
          'the work permit of the target at the buyer (refused: the offer collapses, status 0x10, rescheduled in 10 days, returns 0) and of each part exchange player at the seller (refused: he leaves the exchange and his value goes to the price); the news and the offer goes on')])
CAMPOS = [('copa +0xa3 (rodadas de 0x68 bytes) +3 dia, +5 ano, +6 fatia, +7 código, +0xd bit 0x100 datas marcadas, +0x21 mãos, +0x23.. vagas de 7 bytes (+0 regras do estádio, +4 dia da semana, +5 fatia, +6 tipo; 0xff vazia); valor; lê e grava (copa_levar_jogos_as_datas: a vaga usada fica 0xff)',
           'cup +0xa3 (0x68-byte rounds) +3 day, +5 year, +6 slot, +7 code, +0xd bit 0x100 set dates, +0x21 legs, +0x23.. 7-byte slots (+0 stadium rules, +4 weekday, +5 time slot, +6 type; 0xff empty); value; reads and writes (copa_levar_jogos_as_datas: the used slot becomes 0xff)'),
          ('partida (0x4f bytes no calendário) +0, +4 estádio, +0xc/+0x10 ids, +0x1c/+0x20 clubes, +0x28 ano, +0x2a dia, +0x30/+0x32, +0x3b tipo, +0x3f fatia, +0x42 tipo da liga; valor; lê e grava (copa_levar_jogos_as_datas)',
           'fixture (0x4f bytes in the calendar) +0, +4 stadium, +0xc/+0x10 ids, +0x1c/+0x20 clubs, +0x28 year, +0x2a day, +0x30/+0x32, +0x3b type, +0x3f time slot, +0x42 league type; value; reads and writes (copa_levar_jogos_as_datas)'),
          ('oferta de transferência +4 lista da troca (ids de 4 bytes), +8 comprador, +0xc alvo, +0x1b preço, +0x2c status (0x10 caída pela permissão), +0x2e; id e valor; lê e grava (tirar_jogador_da_troca, permissoes_da_oferta)',
           'transfer offer +4 part exchange list (4-byte ids), +8 buyer, +0xc target, +0x1b price, +0x2c status (0x10 collapsed by the permit), +0x2e; id and value; reads and writes (tirar_jogador_da_troca, permissoes_da_oferta)'),
          ('gerência de transferências +0 vetor das ofertas, +0x86e registro de 8 bytes por pessoa (+2 ofertas em que está na troca), +0x856 gerente de listas; valor; lê e grava (tirar_jogador_da_troca, permissoes_da_oferta)',
           'transfer manager +0 offer array, +0x86e 8-byte record per person (+2 offers where he is in the exchange), +0x856 shortlist manager; value; reads and writes (tirar_jogador_da_troca, permissoes_da_oferta)'),
          ('pessoa +0x52 valor na troca; clube +0x53 nação; [0xae1080] o último contrato consultado; valor e ponteiro; lê (tirar_jogador_da_troca, permissoes_da_oferta), grava [0xae1080]',
           'person +0x52 exchange value; club +0x53 nation; [0xae1080] the last contract looked up; value and pointer; reads (tirar_jogador_da_troca, permissoes_da_oferta), writes [0xae1080]')]
INTERESSA = ['CMRTE']
