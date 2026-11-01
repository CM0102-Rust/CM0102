# Os registros do campeao da liga e da copa e o registro do tecnico humano zerado (staff_records.cpp):
# - 0x86a800 (Staff_Records::add_cup_winners_record): pelas finais das copas (29 chamadores); 11 chamadas na carreira
#   automatica ate 2002-06-01 (simular.py --cobertura, motor ligado, 01/11/2026). A reputacao do clube, as notas do
#   tecnico, o registro dele, o moral do elenco e do contrato e a renovacao.
# - 0x868c50 (Staff_Records::add_league_champions_record): pelos campeoes das ligas (25 chamadores); 4 chamadas.
# - 0x86fc70 (Staff_Records::clear_human_manager_record): passageira (0 chamadas: so pela aposentadoria do humano e
#   pelo setup).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   As proprias: 0x86a800 thiscall (mov ecx, Staff_Records; 3 pushes: clube, jogo, partida; ret 0xc), 0x868c50 thiscall
#   (2 pushes: clube, comp; ret 8), 0x86fc70 thiscall (push pessoa; ret 4; o ecx e o Staff_Records: le [ecx+8]).
#   Nas duas primeiras: 0x58e340 thiscall (push entrada; push fim; push comeco; mov ecx, [edi] / [ecx+4]; ret 0xc),
#   0x53d980 cdecl (push 1; push nacao; push tecnico; add esp, 0xc), 0x945640 ftol, 0x945ff0 _CIpow (x87: base em st1,
#   expoente em st0; pow_do_jogo), 0x86f5b0 thiscall (push 1; push tecnico; mov ecx, edi; ret 8), 0x53b900 cdecl (push
#   clube; add esp, 4), 0x544710 cdecl (push 1; push delta; push p; add esp, 0xc), 0x4cf5f0 thiscall (push clube; push
#   p; mov ecx, [0xae1050]; ret 8), 0x54cdb0 thiscall (push +0x29; push +0x25; mov ecx, 0xae2c90; ret 8), 0x849ce0
#   thiscall (push 0; push 0; push p; push clube; mov ecx, [[0xde1f54]+0x856]; ret 0x10), 0x4cda70 thiscall (push 1;
#   push 0x3ff00000; push 0; push -1; push 0; push 0; push p; mov ecx, [0xae1050]: (p, 0, 0, -1, 1.0, 1)), 0x76d940 (push
#   comp; mov ecx, [0xb74318]; ret 4; o porte e stdcall e nao le o ecx), 0x774f30 thiscall (push comp; mov ecx,
#   [0xb74318]; ret 4), 0x75c390 thiscall (push marca; push comp; push clube; mov ecx, [0xb6b7c8]; ret 0xc), 0x7c1e30
#   thiscall (push 6,5f; push 1; push 2; push ficha; mov ecx, [0xb74350]; ret 0x10; volta no x87).
# Comparadores: as duas primeiras em ensaio (modelo das cqa_12): o registro do tecnico criado antes, a copia sem os
# historicos (arquivo), sem as noticias, sem acabar as convocacoes e sem a renovacao (os sorteios do moral sem contar),
# e a original a partir do mesmo estado; confere a reputacao, as notas, o registro do tecnico, o +0x7b e o bloco +0x69
# (liga), o moral da ficha e o do contrato dos 50 do elenco (fora os que a renovacao mexeria) e a data estatica da
# 0x54cdb0. A 0x86fc70 lado a lado pelos 0xd2 bytes do registro. Iguais no cm0102-1080.exe (as tres inteiras).
# Deterministicas: as entradas de historico saem com a data construida e os campos gravados (os bytes que a original
# nao grava so vao ao arquivo do historico; a copia grava zero, como as cpq_4/cqa_12); o pow e o da CRT do jogo.
SAVE = False
ROTINAS = [('campeao_da_liga_nos_registros', 0x868c50, [0x6a, 0xff, 0x68, 0x76, 0x2b, 0x96, 0x00]),
           ('campeao_da_copa_nos_registros', 0x86a800, [0x6a, 0xff, 0x68, 0x26, 0x2c, 0x96, 0x00]),
           ('limpar_registro_do_humano', 0x86fc70, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0x64, 0x23, 0xae, 0x00])]
DOCS = ('os registros do campeão da liga e da copa e o registro do técnico humano zerado',
        'the league and cup champions records and the cleared human manager record',
        [('campeao_da_liga_nos_registros', 'Staff_Records::add_league_champions_record', '0x868c50', 'staff_records.cpp', 'thiscall',
          'o campeão da liga: a reputação do clube (até peso × 500), o +0x7b e o bloco +0x69, as notas e o registro do técnico (peso × 5000), os históricos (tipo 5), o moral do elenco pela média do jogador (acima do limiar: +20 e +100 no contrato com a renovação; senão +10 e +50) e o elogio da mídia',
          'the league champion: the club reputation (up to weight × 500), the +0x7b and the +0x69 block, the manager\'s ratings and record (weight × 5000), the histories (type 5), the squad morale by the player\'s average (above the threshold: +20 and +100 on the contract with the renewal; else +10 and +50) and the media praise'),
         ('campeao_da_copa_nos_registros', 'Staff_Records::add_cup_winners_record', '0x86a800', 'staff_records.cpp', 'thiscall',
          'o campeão da copa: a reputação do clube, as notas do lado dele na partida, o registro do técnico (peso^4 × 0,5), os históricos (tipo 0), o moral do elenco (+15 e +100 no contrato, com a renovação) enquanto a reputação não passa de peso × 400, o fim das convocações e o elogio da mídia',
          'the cup winner: the club reputation, its side\'s ratings in the match, the manager\'s record (weight^4 × 0.5), the histories (type 0), the squad morale (+15 and +100 on the contract, with the renewal) while the reputation stays up to weight × 400, the end of the call-ups and the media praise'),
         ('limpar_registro_do_humano', 'Staff_Records::clear_human_manager_record', '0x86fc70', 'staff_records.cpp', 'thiscall',
          'o registro do técnico humano (um dos 16; senão a própria pessoa, como na original) zerado, com −1 nas vagas',
          'the human manager record (one of the 16; else the person itself, as in the original) cleared, with −1 in the slots')])
CAMPOS = [('clube +0x80 reputação; valor; lê e grava (campeao_da_liga_nos_registros, campeao_da_copa_nos_registros)',
           'club +0x80 reputation; value; reads and writes (campeao_da_liga_nos_registros, campeao_da_copa_nos_registros)'),
          ('clube +0x57, +0x73, +0x7b e o bloco +0x69 (+0x44); valor; lê e grava (campeao_da_liga_nos_registros)',
           'club +0x57, +0x73, +0x7b and the +0x69 block (+0x44); value; reads and writes (campeao_da_liga_nos_registros)'),
          ('partida +0x188/+0x18a/+0x18c notas do lado (0x625 por lado), +0x190 escalação (20 × 0x49 bytes); valor; lê e grava (campeao_da_copa_nos_registros)',
           'fixture +0x188/+0x18a/+0x18c side ratings (0x625 per side), +0x190 lineup (20 × 0x49 bytes); value; reads and writes (campeao_da_copa_nos_registros)'),
          ('registro de não jogador do técnico +8/+0xa/+0xc notas; valor; grava (campeao_da_liga_nos_registros)',
           'manager\'s non-player record +8/+0xa/+0xc ratings; value; writes (campeao_da_liga_nos_registros)'),
          ('registro do técnico (0xd2 bytes, Staff_Records +8) +0x8a/+0x8c títulos, +0xb6/+0xba/+0xbe/+0xca pontos; valor; grava (as duas) e zera (limpar_registro_do_humano)',
           'manager record (0xd2 bytes, Staff_Records +8) +0x8a/+0x8c titles, +0xb6/+0xba/+0xbe/+0xca points; value; writes (both) and clears (limpar_registro_do_humano)'),
          ('contrato +0x44 moral, +0x4f situação (>> 4), +0x25 data; valor; lê e grava (as duas)',
           'contract +0x44 morale, +0x4f status (>> 4), +0x25 date; value; reads and writes (both)'),
          ('competição +0x69 peso; objeto [0xadadfc] + 4 × id: +0x3c/+0x3e (limiar), +0x42 fase; valor; lê (as duas)',
           'competition +0x69 weight; object [0xadadfc] + 4 × id: +0x3c/+0x3e (threshold), +0x42 phase; value; reads (both)')]
INTERESSA = ['CMRTE']
