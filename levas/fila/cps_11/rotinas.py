# O calendário guardado (fix_man.cpp): FIXTURE_MANAGER::find_stored_fixture (0x5ac7a0, 330 chamadas na cobertura de
# 17/10), FIXTURE_MANAGER::insert_fixture (0x5ab230, 123) e FIXTURE_MANAGER::remove_fixture (0x5abc30, 468). As três
# acham o bloco do ano do jogo pela corrente do calendário (o seguinte de outro ano vai ao ano pedido pela 0x5aa2d0;
# sem seguinte, um bloco novo pela 0x944cf1 + 0x5a9cf0) e a lista do dia, fatia e nação-chave (0x48dd70); depois
# procuram a cópia de 0x4f bytes iguais, acrescentam uma cópia (0x68d0d0) ou tiram a primeira igual (0x68d1f0). O SEH
# das três só cobre o bloco novo durante o construtor (sem destrutor a reproduzir). O ano empurrado à 0x5aa2d0 leva sobra
# de registrador na metade alta (ela compara a palavra; caminho só da virada do ano): a cópia passa o ano limpo
# (comentado). Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira e a confirmação no jogo real comparando.
# Comparadores: a busca pela devolução; guardar e tirar com a lista achada antes pela cópia, a original faz e confere a
# conta, a cauda nova e a devolução esperada.
SAVE = False
ROTINAS = [('jogo_guardado_igual', 0x5ac7a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('guardar_jogo_no_calendario', 0x5ab230, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('tirar_jogo_do_calendario', 0x5abc30, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o calendário guardado: achar a cópia guardada de um jogo, guardar um jogo e tirar um jogo',
        'the stored calendar: finding the stored copy of a fixture, storing a fixture and removing a fixture',
        [('jogo_guardado_igual', 'FIXTURE_MANAGER::find_stored_fixture', '0x5ac7a0', 'fix_man.cpp', 'thiscall, ret 4',
          'devolve a cópia guardada cujos 0x4f bytes são iguais aos do jogo, na lista do dia, fatia e nação-chave dele no bloco do ano; 0 se não há',
          "returns the stored copy whose 0x4f bytes match the fixture's, in its day, slice and key-nation list of the year block; 0 if none"),
         ('guardar_jogo_no_calendario', 'FIXTURE_MANAGER::insert_fixture', '0x5ab230', 'fix_man.cpp', 'thiscall, ret 4',
          'acha (ou cria) o bloco do ano do jogo e acrescenta uma cópia dele à lista do dia, fatia e nação-chave',
          "finds (or creates) the fixture's year block and appends a copy of it to the day, slice and key-nation list"),
         ('tirar_jogo_do_calendario', 'FIXTURE_MANAGER::remove_fixture', '0x5abc30', 'fix_man.cpp', 'thiscall, ret 4',
          'tira da lista do dia, fatia e nação-chave do jogo o primeiro item com os mesmos bytes; devolve 1 se tirou',
          "removes the first item with the same bytes from the fixture's day, slice and key-nation list; returns 1 if removed")])
CAMPOS = [('jogo (0x4f bytes) +0x14: a competição; +0x28 o ano (palavra); +0x2a o dia (palavra); +0x3f a fatia; ponteiro e valor; lê (as três)',
           'fixture (0x4f bytes) +0x14: the competition; +0x28 the year (word); +0x2a the day (word); +0x3f the slice; pointer and value; reads (all three)'),
          ('bloco do ano do calendário (0x1184 bytes) +0: as listas (dia × 3 + fatia) de vetores por nação-chave; +0x112c o bloco seguinte; +0x1130 o ano (palavra); ponteiro e valor; lê e grava o seguinte (as três)',
           'calendar year block (0x1184 bytes) +0: the lists (day × 3 + slice) of per-key-nation arrays; +0x112c next block; +0x1130 the year (word); pointer and value; reads and writes the next one (all three)'),
          ('lista (LLIST) +0: a conta (palavra); +2 o começo; +6 a cauda; +0xa o cursor; nó +0 o dado, +4 o tamanho, +6 o seguinte; valor e ponteiro; lê, anda o cursor, acrescenta e tira (as três)',
           'list (LLIST) +0: the count (word); +2 head; +6 tail; +0xa cursor; node +0 data, +4 size, +6 next; value and pointer; reads, moves the cursor, appends and removes (all three)')]
INTERESSA = ['SGE']
