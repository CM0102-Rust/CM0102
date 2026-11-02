# A Copa do Mundo: o sorteio dos grupos, o sorteio feito no dia e o dia da Copa (world_cup.cpp):
# - 0x92e940 (WorldCup::draw_groups, 1728 bytes, sem SEH): a lista do sorteio (32 × 5 bytes): os cabeças de chave nos
#   grupos 1..8 (o 1 e o 3 fixos, os outros sorteados entre as vagas 2..7), os outros 24 em grupos sorteados com o limite
#   de (k / 8) + 1 times; na Copa de 2002, depois disso, o sorteio real (as 32 nações por chave) e a tabela das nações
#   [+0x14] com a situação (6; 2 a detentora, 1 as sedes). Roda uma vez na carreira automática até 2002-06-01, em
#   27/12/2001 (a data +0xb6/+0xb8 gravada pela 0x92d200: dia 0x1b do mês 0xb, ano anterior).
#   O LAÇO DA QUEDA DE 27/12/2001 (contornado pelo Alan_v2 em 0x92ea55): o sorteio dos cabeças de chave dos grupos 2 e
#   4..8 repete 0x90cfc0(6) + 2 enquanto a vaga sorteada (+0xf2 + 5k) for 0xff. A seed_teams (0x92f230) volta sem fazer
#   nada na Copa de 2002 (o teste +0x40 = 2002 no começo dela); [inferência] numa carreira começada antes de 2001 (o ano
#   do jogo mudado por patch), a Copa de 2002 vem da start_new_season (0x92d330) e não do banco de dados, as vagas 2..7
#   ficam 0xff e o laço não termina: o jogo para em 27/12/2001 sem erro (o relato do fórum: "Start a game in the year
#   1997 ... when you reach 27/12/2001 stop running"). A cópia repete o laço como a original (sem limite).
# - 0x92e7f0 (WorldCup::run_group_draw, 336 bytes): o sorteio do dia: a lista, o grupo 1 na própria Copa (0x92d0f0), os
#   jogos dele (0x6835c0), a tabela ordenada (0x6827d0), as fases dos grupos 2..8 (0x92d540), as posições previstas
#   (método +0x5c) e os cartões zerados (0x7e9180). Uma vez, em 27/12/2001.
# - 0x92f1a0 (WorldCup::update_for_date, 144 bytes, método +0x14): o dia da Copa: a semeadura em 15/12, o sorteio em
#   27/12 (sem tabela ainda) e o dia de liga (0x6847c0, repassando a devolução): todo dia da carreira.
# Iguais no cm0102-1080.exe (as três inteiras). Chamam originais não portadas: 0x945501 (calloc), 0x9452ca (free), a
# caixa (0x944cff, 0x90d130, 0x944c9f, 0x5e8290), 0x92d0f0, 0x92d540, 0x92f680 e 0x92f230 (as três primeiras na cqf_5,
# a última na cqf_6) e 0x7e9180; o resto já em Rust (0x90cfc0, 0x53b3d0, 0x6835c0, 0x6827d0, 0x6847c0).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x92e940: 0x92e7fa (mov ecx, esi; sem push) -> thiscall, ret 0; devolve a lista no eax.
# - quem chama a 0x92e7f0: 0x92f20e (mov ecx, esi; sem push) -> thiscall, ret 0.
# - quem chama a 0x92f1a0: o método +0x14 da vtable 0x9712f0 (o dia das competições: push completo; push &data; mov
#   ecx, comp; call [vt+0x14], como a liga_dia) -> thiscall, ret 8.
# - dentro da 0x92e940: 0x945501 push 5; push 0x20; add esp, 8 -> cdecl (calloc); 0x90cfc0 push 6 (ou 8); add esp, 4
#   -> cdecl; 0x53b3d0 push nação; add esp (acumulado, 0x40) -> cdecl; a caixa (aviso_em).
# - dentro da 0x92e7f0: 0x92e940 mov ecx -> thiscall; 0x92d0f0 push lista; mov ecx -> thiscall, ret 4; 0x6835c0 mov ecx
#   -> thiscall, ret 0; 0x6827d0 push 0; mov ecx -> thiscall, ret 4; 0x92d540 push lista; push g; mov ecx -> thiscall,
#   ret 8; 0x9452ca push; add esp, 4 -> cdecl; o método +0x5c mov ecx, esi -> thiscall; 0x7e9180 push 0; mov ecx,
#   [[0xae28f0] + 4 × (char)+0x50] -> thiscall, ret 4.
# - dentro da 0x92f1a0: 0x92f230 e 0x92e7f0 mov ecx, esi -> thiscall, ret 0; 0x6847c0 push completo; push hoje; mov
#   ecx, esi -> thiscall, ret 8.
# Comparadores: a 0x92e940 lado a lado (o estado do sorteador, as vagas +0xee, +0x36, [0xb67a34] e, em 2002, a tabela
# [+0x14]; a cópia sem contar os sorteios; as duas listas alocadas iguais byte a byte, a da cópia liberada); a 0x92e7f0 e
# a 0x92f1a0 só a original (o sorteio, os jogos, as notícias e o dia de liga não rodam duas vezes), conferindo o que
# garantem: o sorteio deixa +0x45 = 1, as sete fases e +0x30 = 6; o dia do sorteio deixa a tabela +0xb1.
# Determinismo: nada lê pilha, heap ou endereço como valor; os contadores dos grupos (8 bytes na pilha) começam zerados
# como na original.
SAVE = False
ROTINAS = [('mundial_sortear_grupos', 0x92e940, [0x81, 0xec, 0x1c, 0x02, 0x00, 0x00]),
           ('mundial_sorteio_dos_grupos', 0x92e7f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('mundial_dia', 0x92f1a0, [0x53, 0x8b, 0x5c, 0x24, 0x0c])]
DOCS = ('a Copa do Mundo: o sorteio dos grupos (com o laço da queda de 27/12/2001), o sorteio feito no dia e o dia da Copa',
        'the World Cup: the group draw (with the 27/12/2001 hang loop), the draw run on its day and the World Cup day',
        [('mundial_sortear_grupos', 'WorldCup::draw_groups', '0x92e940', 'world_cup.cpp', 'thiscall, ret 0',
          'a lista do sorteio: os cabeças de chave nos grupos 1..8 (sorteados entre as vagas de pé: o laço que trava o jogo em 27/12/2001 quando nenhuma está de pé), os outros 24 em grupos sorteados com limite; em 2002, o sorteio real e a tabela das nações com a detentora e as sedes',
          'the draw list: the seeds in groups 1..8 (drawn among the slots still standing: the loop that hangs the game on 27/12/2001 when none is), the other 24 in drawn groups with a limit; in 2002, the real draw and the nation table with the holder and the hosts'),
         ('mundial_sorteio_dos_grupos', 'WorldCup::run_group_draw', '0x92e7f0', 'world_cup.cpp', 'thiscall, ret 0',
          'o sorteio do dia: a lista, o grupo 1 na própria Copa com os jogos e a tabela ordenada, as fases dos grupos 2..8, as posições previstas e os cartões zerados',
          'the draw day: the list, group 1 in the World Cup itself with its fixtures and sorted table, the group 2..8 stages, the predicted positions and the bookings reset'),
         ('mundial_dia', 'WorldCup::update_for_date', '0x92f1a0', 'world_cup.cpp', 'thiscall, ret 8',
          'o dia da Copa: a semeadura na data das cabeças de chave, o sorteio na data dele (sem tabela ainda) e o dia de liga',
          'the World Cup day: the seeding on the seeding date, the draw on its date (no table yet) and the league day')])
CAMPOS = [('copa do mundo +0x14 tabela das nações (6 bytes: +0 clube, +4 situação: 6, 2 detentora, 1 sede), +0x36 quantas (word), +0x38 o máximo, +0x40 ano, +0x45 sorteio feito, +0x50 índice do órgão (char), +0xb1 tabela do grupo 1, +0xb6/+0xb8 data do sorteio (dia do ano, ano relativo), +0x18e/+0x190 data das cabeças de chave; valor e ponteiro; lê e grava (mundial_sortear_grupos, mundial_sorteio_dos_grupos, mundial_dia)',
           'world cup +0x14 nation table (6 bytes: +0 club, +4 status: 6, 2 holder, 1 host), +0x36 count (word), +0x38 maximum, +0x40 year, +0x45 draw done, +0x50 ruling body index (char), +0xb1 group 1 table, +0xb6/+0xb8 draw date (day of year, relative year), +0x18e/+0x190 seeding date; value and pointer; reads and writes (mundial_sortear_grupos, mundial_sorteio_dos_grupos, mundial_dia)'),
          ('copa do mundo +0xee as 8 vagas dos cabeças de chave e +0x116 os 24 outros (5 bytes: +0 id do clube, +4 pote; 0xff = já sorteada); id e valor; lê e grava (mundial_sortear_grupos)',
           'world cup +0xee the 8 seed slots and +0x116 the other 24 (5 bytes: +0 club id, +4 pot; 0xff = already drawn); id and value; reads and writes (mundial_sortear_grupos)'),
          ('lista do sorteio (32 × 5 bytes, alocada): +0 id do clube da seleção, +4 grupo 1..8; id e valor; grava (mundial_sortear_grupos)',
           'draw list (32 × 5 bytes, allocated): +0 national team club id, +4 group 1..8; id and value; writes (mundial_sortear_grupos)'),
          ('[0x9cf218..0x9cf51c] chaves das nações (g_key_nation_*), [0xae23a8] tabela de nações (0x122 bytes), [0xae23b4] tabela de clubes (0x245 bytes); id e ponteiro; lê (mundial_sortear_grupos)',
           '[0x9cf218..0x9cf51c] nation keys (g_key_nation_*), [0xae23a8] nation table (0x122 bytes), [0xae23b4] club table (0x245 bytes); id and pointer; reads (mundial_sortear_grupos)')]
INTERESSA = ['CMRTE', 'GPF2']
