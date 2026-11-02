# A vontade de uma pessoa de ir para um clube (ou de ficar no dela) e o construtor do contrato (shortlist_manager.cpp,
# staff_contracts.cpp), pedido do Lucas para a família dos empréstimos (referência para o CM0102-Rust-Tweaks,
# docs/EMPRESTIMOS.md):
# - 0x846920 (Shortlist_Manager::staff_interest_in_joining_club, 13248 bytes, 3881 instruções, quadro SEH sem
#   destrutor no caminho normal, x87 transcrito em f64 na ordem da original): a nota de vontade e os bits de motivo.
#   186136 chamadas na carreira automática até 2002-06-01 (docs/cobertura_simulacao.txt, 17/10), pelos lances, pelos
#   pedidos de empréstimo (0x842740, a nota mínima 3,5), pelas listas e pelos contratos.
# - 0x85e410 (STAFF_CONTRACT::STAFF_CONTRACT, 99 bytes): o construtor do contrato local (datas padrão e campos
#   zerados): passageira, chamada também pela própria 0x846920 no ramo do salário.
# Ficaram de fora as três originais que a 0x846920 chama: 0x864550 (o salário padrão, 8 KB) e 0x4a4850
# (Comp_Stats::find_club_entry: objetos locais com destrutor, a caixa de erro e um qsort com rotina de comparação
# global) seguem originais; a 0x85e410 entrou.
# Iguais no cm0102-1080.exe (as duas inteiras, conferido byte a byte).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x846920 (32 pontos): 0x8c8d2e (push motivo; push situação, um byte com lixo acima; mov ecx,
#   [edi+0x856]; push 0; push 1; push clube; push pessoa), 0x4ce73b (mov ecx, [ecx+0x856]; ...; push clube; push
#   pessoa), e as Rust 0x842740 / 0x83b6e0 / 0x8c1290 (thiscall com 6 argumentos) -> thiscall, ret 0x18 (ela termina
#   em ret 0x18, devolve no st0). O this é lido ([this+0xdc]: a tabela de 9 bytes por clube, a palavra +5). A situação
#   vale pelo byte nos saltos, mas o dword inteiro (com o lixo de quem chama) vai à 0x864550 e à 0x5968d0, como na
#   cópia.
# - quem chama a 0x85e410: 0x849914 (lea ecx, [esp+0xa0]; call; sem push) e as outras do contrato -> thiscall, ret 0.
# - dentro da 0x846920: 0x54a110 lea ecx -> thiscall, ret 0; 0x8cca10 push 0; push pessoa; mov ecx, [0xde1f54] ->
#   thiscall, ret 8; 0x4cf5a0 push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4; 0x4cf5f0 push clube; push pessoa;
#   mov ecx, [0xae1050] -> thiscall, ret 8; 0x54cdb0 push [0xae2c94]; push [0xae2c90]; lea ecx, data -> thiscall, ret 8;
#   0x945640 (_ftol, st0 truncado: ftol); 0x53d980 push 1; push nação; push pessoa; add esp, 0xc -> cdecl;
#   0x544590 / 0x544520 / 0x5444a0 / 0x547d30 / 0x547d60 push clube; push pessoa; add esp, 8 -> cdecl; 0x547810 /
#   0x547840 push pessoa; push técnico; add esp, 8 -> cdecl; 0x4b02e0 push atual; push clube; add esp, 8 -> cdecl;
#   0x54c8e0 push 0x96; push saída; mov ecx, 0xae2c90 -> thiscall, ret 8; 0x4ca470 push contrato; push pessoa; mov ecx,
#   [0xae1050] -> thiscall, ret 8; 0x600a50 push 0; push 0; push 1; push 0 (ou 1); push clube; add esp, 0x14 -> cdecl;
#   0x4a4850 push clube; mov ecx, [[0xadadfc] + 4·id] + 8 -> thiscall, ret 4; 0x5407e0 push 1; push pessoa; push
#   técnico; add esp, 0xc -> cdecl (só a palavra conta); 0x5a0590 push clube; mov ecx, [0xae2a38] -> thiscall, ret 4;
#   0x5986b0 mov ecx, finanças -> thiscall, ret 0; 0x7c6830 push 1; push idioma (byte; o resto do registrador é lixo, a
#   cópia passa o byte); push clube; push pessoa; mov ecx, [0xb74350] -> thiscall, ret 0x10; 0x4cbb60 push clube; push
#   pessoa; mov ecx, [0xae1050] -> thiscall, ret 8; 0x669bd0 push &data b; push &data a; push clube; add esp, 0xc ->
#   cdecl; 0x85e410 lea ecx, [esp+0xa0] -> thiscall, ret 0; 0x864550 push 0; push 0xb; push situação; push 0; push
#   clube; push pessoa; lea ecx, contrato -> thiscall, ret 0x18 (st0); 0x5968d0 push situação; push pessoa; mov ecx,
#   finanças -> thiscall, ret 8. As saltadas por tabela (0x849c58, 0x849c74, 0x849c90, 0x849ca4, 0x849cb8) conferidas.
# Comparadores: as duas lado a lado. A 0x846920 nos bits do f64 devolvido, nos motivos (com o ponteiro) e em [0xae1080]
# (a original e a 0x864550 gravam); as chamadas de dentro só leem (a 0x4a4850 reordena a tabela da competição, igual
# nas duas), sem sorteio. A 0x85e410 nos 0x4f bytes do contrato e na devolução.
# Determinismo: no ramo do próprio clube (clube = o do contrato) a original divide pelo float [base+0x58], que nesse
# caminho ninguém gravou (sobra da pilha); o resultado não é usado (o caminho segue direto para o fim), e a cópia divide
# por 1. As palavras de reputação guardadas em dwords com o resto do registrador (o ponteiro do clube em cima) são lidas
# só como palavra, como na original. Nada mais lê pilha ou heap como valor.
# Provas (01/11/2026, cópia fora do projeto: motor.rs da principal com a 266ª + os três itens cqe_4..6, 11 rotinas):
# - duas carreiras Rust até 2002-06-01 com o motor de partidas (como a cadeia) iguais entre si: impressão
#   6dec43875616a596, 19404 partidas, e uma terceira igual noutra pasta; até 2001-09-01 sem o motor também iguais;
# - jogo comparando (Inglaterra, sem o motor) até 2002-095: 0 divergências em 10 bilhões de comparações;
#   com Escócia, Itália, Espanha e Irlanda até 2002-014: 0 divergências destes itens (há 15 de duas rotinas que
#   não são deles: copa_criar_jogos_do_confronto e preencher_vaga_da_comissao, só nessas ligas);
# - as 9 mutações dos três itens, cada uma numa carreira Rust até 2002-06-01 com o motor: todas mudam a impressão.
# - contadores até 2002-06-01: base de outro clube 131072+, fator da reputação 131072+, salário 8192+.
SAVE = False
ROTINAS = [('cqe4_interesse', 0x846920, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('cqe4_construir_contrato', 0x85e410, [0x6a, 0xff, 0x68, 0xeb, 0x29, 0x96, 0x00])]
DOCS = ('a vontade de uma pessoa de ir para um clube (ou de ficar no dela), com os motivos, e o construtor do contrato',
        'the willingness of a person to join a club (or to stay at his own), with the reasons, and the contract constructor',
        [('cqe4_interesse', 'Shortlist_Manager::staff_interest_in_joining_club', '0x846920', 'shortlist_manager.cpp', 'thiscall, ret 0x18',
          'a nota de vontade (devolvida no st0) e os bits de motivo: a base 2,75 (3,5 no próprio clube, o contrato acabando, a idade sem clube), a reputação dos clubes no empréstimo, gostar ou não do clube e do técnico, rivais, nações e distância, a moral, a situação no elenco e a pedida, a reputação comparada (estádio, divisão, competição, técnico, finanças), a fluência no idioma, o tempo sem clube e, pedido o salário, o salário padrão contra o teto do clube',
          'the willingness score (returned in st0) and the reason bits: the 2.75 base (3.5 at his own club, the contract running out, the age with no club), the club reputations on loans, liking or disliking the club and the manager, rivals, nations and distance, morale, squad status and demands, the compared reputation (stadium, division, competition, manager, finances), language fluency, time without a club and, when a wage is asked, the default wage against the club wage cap'),
         ('cqe4_construir_contrato', 'STAFF_CONTRACT::STAFF_CONTRACT', '0x85e410', 'staff_contracts.cpp', 'thiscall, ret 0',
          'o contrato vazio: as datas +0x25 e +0x2d padrão, +4 e +0x4a = -1, +8, +0x3b, +0x40, +0x45, +0x49 e +0x4e = 0',
          'the empty contract: default dates at +0x25 and +0x2d, +4 and +0x4a = -1, +8, +0x3b, +0x40, +0x45, +0x49 and +0x4e = 0')])
CAMPOS = [('pessoa +0x18 idade, +0x1a nação, +0x22 jogos pela seleção (byte), +0x39 clube favorito, +0x3e/+0x40 data (fora do clube), +0x56/+0x57/+0x59/+0x5a/+0x5b atributos de caráter, +0x61 ficha (+5, +7, +9/+0xb/+0xd reputações, +0xf posição, +0x45 moral do jogador (i8)), +0x69; valor e ponteiro; lê (cqe4_interesse)',
           'person +0x18 age, +0x1a nation, +0x22 international caps (byte), +0x39 favourite club, +0x3e/+0x40 date (out of club), +0x56/+0x57/+0x59/+0x5a/+0x5b personality attributes, +0x61 player record (+5, +7, +9/+0xb/+0xd reputations, +0xf position, +0x45 player morale (i8)), +0x69; value and pointer; reads (cqe4_interesse)'),
          ('clube +0x53 nação (+0x71, +0x77 idioma, +0x7e, +0x85), +0x57 divisão (+0x69), +0x64, +0x69 estádio (+0x38 cidade: +0x23/+0x2b coordenadas, +0x33 importância), +0x73 público, +0x80 reputação, +0xb3/+0xb7/+0xbb rivais, +0xcf técnico (+0x1a nação, +0x3e/+0x40 data de chegada, +0x59, +0x69), +0x1db competição; valor e ponteiro; lê (cqe4_interesse)',
           'club +0x53 nation (+0x71, +0x77 language, +0x7e, +0x85), +0x57 division (+0x69), +0x64, +0x69 stadium (+0x38 city: +0x23/+0x2b coordinates, +0x33 importance), +0x73 attendance, +0x80 reputation, +0xb3/+0xb7/+0xbb rivals, +0xcf manager (+0x1a nation, +0x3e/+0x40 arrival date, +0x59, +0x69), +0x1db competition; value and pointer; reads (cqe4_interesse)'),
          ('contrato +4 clube, +8, +0xc, +0x25/+0x2d datas, +0x2f ano do fim, +0x35 tipo, +0x3b, +0x3d pedida, +0x40, +0x44 felicidade no contrato (happiness, i8 ±100), +0x45 marcas (1, 0x10, 0x20, 0x100, 0x161a8), +0x46 bit 1, +0x49 pedida em aberto, +0x4a, +0x4e bit 8, +0x4f situação (>> 4); valor; lê (cqe4_interesse), grava (cqe4_construir_contrato)',
           'contract +4 club, +8, +0xc, +0x25/+0x2d dates, +0x2f end year, +0x35 type, +0x3b, +0x3d demands, +0x40, +0x44 contract happiness (i8 ±100), +0x45 flags (1, 0x10, 0x20, 0x100, 0x161a8), +0x46 bit 1, +0x49 open demand, +0x4a, +0x4e bit 8, +0x4f squad status (>> 4); value; reads (cqe4_interesse), writes (cqe4_construir_contrato)'),
          ('[0xae1080] o último contrato consultado; ponteiro; grava (cqe4_interesse); [Shortlist_Manager +0xdc] tabela de 9 bytes por clube (+5 reputação); valor; lê; [0xb67a44] reputações vistas pelo técnico (0x49 bytes por clube: +6, +8, +0xa); valor; lê',
           '[0xae1080] the last looked-up contract; pointer; writes (cqe4_interesse); [Shortlist_Manager +0xdc] 9-byte table per club (+5 reputation); value; reads; [0xb67a44] reputations seen by the manager (0x49 bytes per club: +6, +8, +0xa); value; reads')]
INTERESSA = ['CMRTE']
