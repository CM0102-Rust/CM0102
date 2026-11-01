# O dia do calendário, a lista dos jogos do dia e a carga dos jogos do ano (fix_man.cpp):
# - 0x5aaf00 (FIXTURE_MANAGER::daily_update, 807 bytes, quadro SEH sem destrutor no caminho normal): os jogos de
#   reservas (dias 2 e 3), a fila das notícias de jogo aos técnicos, os amistosos de ontem entre clubes sem técnico
#   humano tirados das listas (dias 5 e 6) e os rótulos das rodadas pela lista das conferências das pernas; pelo laço do
#   dia 0x5cc850: 1098 chamadas na carreira automática até 2002-06-01 (docs/cobertura_simulacao.txt, 17/10).
# - 0x5abf10 (FIXTURE_MANAGER::get_fixture_list, 333 bytes, quadro SEH sem destrutor no caminho normal): a lista dos
#   jogos de um ano, dia, fatia e nação-chave, trocando ou criando o bloco do outro ano; 14 milhões de chamadas (a mais
#   quente do calendário: sem contador nela).
# - 0x5ac3b0 (FIXTURE_MANAGER::load_fixtures, ~1 KB): lê o fixtures_<ano>.tmp de CM3_TEMP para as listas do bloco,
#   pelo construtor do bloco (0x5a9cf0) e pela troca do ano (0x5aa2d0): 6 chamadas; as trocas do bloco do outro ano
#   (gravar 2002, carregar 2003 e voltar) recarregam os jogos de 2002 que a carreira joga.
# Iguais no cm0102-1080.exe (as três inteiras). A 0x5a9cf0 (o construtor do bloco) ficou de fora: o exe de trabalho
# tem a troca do CD nela (0x5a9d65, 0x5a9da7), lógica diferente.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x5aaf00: 0x5cc979 e 0x5ccadd (mov ecx, [0xae2a58]; call; sem push) e o salto de cauda 0x5ce0a1 (mov ecx,
#   [0xae2a58]; jmp) -> thiscall, ret 0 (ela termina em ret).
# - quem chama a 0x5abf10: 0x5ac9bb (push nação; push fatia; push dia; push ano; mov ecx, [esp+0x28]) e outras 13 ->
#   thiscall, ret 0x10 (ela termina em ret 0x10).
# - quem chama a 0x5ac3b0: 0x5a9ffb e 0x5aa5e1 (mov ecx, bloco; sem push) -> thiscall, ret 0.
# - dentro da 0x5aaf00: 0x54a3f0 mov ecx, 0xae2c90 -> thiscall, ret 0; 0x54c5b0 / 0x54c8e0 push n; push saída; mov ecx,
#   0xae2c90 -> thiscall, ret 8; 0x7e6380 push &data; mov ecx, [0xb74434] -> thiscall, ret 4; 0x68d1b0 / 0x68d1d0 mov
#   ecx, lista -> thiscall, ret 0; 0x785830 push k; mov ecx, notícia -> thiscall, ret 4; 0x7861e0 push clube; push
#   notícia; mov ecx, [0xb7431c] -> thiscall, ret 8; 0x68d040 mov ecx, lista -> thiscall, ret 0; 0x944c94 push lista;
#   add esp, 4 -> cdecl; 0x600a50 push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl; 0x68d280 push nó;
#   mov ecx, lista -> thiscall, ret 4; 0x522c70 push +0xb; push +7; push eax (palavra da rodada); ecx = a copa (ficou do
#   cálculo) -> thiscall, ret 0xc (ela faz mov esi, ecx e lê a palavra); 0x68d080 mov ecx, 0xae2a48 -> thiscall, ret 0.
# - dentro da 0x5abf10: 0x5aa2d0 push ano; ecx = [bloco +0x112c] -> thiscall, ret 4; 0x944cf1 push 0x1184; add esp, 4
#   -> cdecl; 0x5a9cf0 push 0; push bloco; push ano; mov ecx, novo -> thiscall, ret 0xc; a caixa: 0x944cff (5 pushes,
#   add esp, 0x14), 0x90d130, 0x944c9f, 0x5e8290 (add esp, 0x24) -> cdecl (aviso_em).
# - dentro da 0x5ac3b0: 0x944c9f push ano; push "fixtures_%d.tmp"; push nome; add esp, 0xc -> cdecl; 0x90bde0 push nome;
#   push "CM3_TEMP"; add esp, 8 -> cdecl (o "rb" empurrado antes fica para a 0x945b00); 0x945b00 push caminho; ("rb");
#   add esp, 8 -> cdecl; 0x945bfe push arquivo; push 1; push tamanho; push destino; add esp, 0x10 -> cdecl; 0x48dd70
#   push &jogo; push competição; add esp, 8 -> cdecl (o push 0x4f e o push &jogo antes ficam para a 0x68d0d0); 0x68d0d0
#   (0x4f, &jogo); ecx = a lista -> thiscall, ret 8; 0x945a09 push arquivo; add esp, 4 -> cdecl; os erros fatais pela
#   0x5cc7d0, 0x5a8f60 e 0x945934(-1) (erro_em).
# Comparadores: a 0x5aaf00 só a original (as notícias, os nós liberados e os jogos de reservas não rodam duas vezes), com
# o que ela garante (a fila das notícias some e a lista das conferências fica vazia); a 0x5abf10 as duas com o ano no
# bloco ou no outro bloco (só leem; a mesma lista), senão só a original (troca ou cria o bloco); a 0x5ac3b0 só a
# original (lê o arquivo e aloca os nós), com o nome gravado em +0x1133.
# Determinismo: nada lê sobra de pilha; os registros locais (as datas, o jogo de 0x4f bytes) são inteiros gravados
# antes do uso (o jogo pelo fread).
SAVE = False
ROTINAS = [('calendario_dia', 0x5aaf00, [0x6a, 0xff, 0x68, 0x70, 0x96, 0x95, 0x00]),
           ('calendario_lista', 0x5abf10, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('calendario_carregar_ano', 0x5ac3b0, [0x81, 0xec, 0x54, 0x02, 0x00, 0x00])]
DOCS = ('o calendário: o dia do calendário (reservas, notícias de jogo, amistosos de ontem, rótulos das rodadas), a lista dos jogos do dia e a carga dos jogos do ano',
        'the fixture manager: the daily update (reserves, fixture news, yesterday\'s friendlies, round labels), the fixture list of a day and the loading of the year\'s fixtures',
        [('calendario_dia', 'FIXTURE_MANAGER::daily_update', '0x5aaf00', 'fix_man.cpp', 'thiscall, ret 0',
          'nos dias 2 e 3 os jogos de reservas de daqui a 3 dias (e de amanhã no dia 3); a fila das notícias de jogo aos técnicos dos dois clubes; nos dias 5 e 6 os amistosos de ontem entre clubes sem técnico humano saem das listas; os rótulos das rodadas das copas da lista das conferências das pernas',
          'on days 2 and 3 the reserve fixtures 3 days ahead (and tomorrow on day 3); the queued fixture news to both clubs\' managers; on days 5 and 6 yesterday\'s friendlies between clubs without a human manager leave the lists; the cup round labels from the leg check list'),
         ('calendario_lista', 'FIXTURE_MANAGER::get_fixture_list', '0x5abf10', 'fix_man.cpp', 'thiscall, ret 0x10',
          'a lista dos jogos do ano, dia, fatia e nação-chave, trocando o bloco do outro ano para o ano pedido ou criando-o',
          'the fixture list of a year, day, slice and key nation, switching the other year block to the requested year or creating it'),
         ('calendario_carregar_ano', 'FIXTURE_MANAGER::load_fixtures', '0x5ac3b0', 'fix_man.cpp', 'thiscall, ret 0',
          'lê o fixtures_<ano>.tmp: cada jogo de 0x4f bytes ganha os ponteiros dos clubes, da competição e do estádio e entra na lista do dia, fatia e nação-chave',
          'reads fixtures_<year>.tmp: each 0x4f-byte fixture gets its club, competition and stadium pointers and goes into the list of its day, slice and key nation')])
CAMPOS = [('calendário (bloco do ano, 0x1184 bytes) +0..+0x1127 as listas de 366 dias × 3 fatias (vetor por nação-chave), +0x1128 a fila das notícias de jogo, +0x112c o bloco do outro ano, +0x1130 o ano, +0x1133 o nome do arquivo; ponteiro e valor; lê e grava (calendario_dia, calendario_lista, calendario_carregar_ano)',
           'fixture manager (year block, 0x1184 bytes) +0..+0x1127 the lists of 366 days × 3 slices (vector by key nation), +0x1128 the fixture news queue, +0x112c the other year block, +0x1130 the year, +0x1133 the file name; pointer and value; reads and writes (calendario_dia, calendario_lista, calendario_carregar_ano)'),
          ('partida (0x4f bytes) +0 competição (id → +0x14), +4 estádio (id → +0x18), +0xc/+0x10 clubes (ids → +0x1c/+0x20), +0x2a dia, +0x3a tipo, +0x3f fatia; id, ponteiro e valor; grava os ponteiros (calendario_carregar_ano), lê (calendario_dia)',
           'fixture (0x4f bytes) +0 competition (id → +0x14), +4 stadium (id → +0x18), +0xc/+0x10 clubs (ids → +0x1c/+0x20), +0x2a day, +0x3a type, +0x3f slot; id, pointer and value; writes the pointers (calendario_carregar_ano), reads (calendario_dia)'),
          ('[0xae2a48] a lista das conferências das pernas (+4 fase, +5 rodada, +7/+0xb os clubes); valor; lê e esvazia (calendario_dia)',
           '[0xae2a48] the leg check list (+4 stage, +5 round, +7/+0xb the clubs); value; reads and empties (calendario_dia)'),
          ('[0x9cf684] a competição dos amistosos; id; lê (calendario_dia)',
           '[0x9cf684] the friendlies competition; id; reads (calendario_dia)')]
INTERESSA = ['SGE']
