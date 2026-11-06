# As três divisões da MLS (usa_mls.cpp; agente C da frente das competições, 06/11/2026): a tabela da primeira divisão
# e as fases da segunda e da terceira. A MLS de 2001 tem doze clubes em três divisões de quatro (os códigos 0x413,
# 0x47b e 0x414 da usa_mls_get_club_division 0x909ec0); a liga é a primeira divisão e as outras duas são as fases 0 e
# 1, que correm junto na temporada regular. Na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt),
# cada uma roda duas vezes: na criação da carreira (a MLS de 2001) e na virada para 2002.
# - 0x908560 UsaMls::create_table: a tabela de quatro da primeira divisão.
# - 0x908fc0 UsaMls::create_second_division_stage e 0x9091e0 UsaMls::create_third_division_stage: o mesmo molde
#   (conferido instrução a instrução: só mudam o código, a vaga, as linhas das caixas e o +0x30 gravado).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - a tabela, do construtor (0x907083) e da virada (0x908eac): mov ecx, esi; call, sem push -> thiscall, ret 0;
# - as divisões, de UsaMls::create_season_fixtures (0x9087c2, 0x9087c9): mov ecx, esi; call -> thiscall, ret 0;
# - 0x909ec0 (usa_mls_get_club_division): push clube; call; add esp, 4 -> cdecl; devolve a palavra do código;
# - 0x687430 (League::init_table_row): push 0; push 0; push clube; push eax (a linha no byte baixo, o código em cima);
#   mov ecx, edi -> thiscall, ret 0x10 (só lê o byte: a cópia passa a linha limpa);
# - 0x944e46 (malloc): push n; call; add esp, 4 -> cdecl; 0x944cf1 (operator new): push 0xee; add esp, 4 -> cdecl;
# - a virtual +0x3c (create_round_table): push 0; push &extra; push &rodadas; push −3; mov ecx, esi -> thiscall, ret 0x10;
# - 0x68b100 (League_Stage::League_Stage): 22 pushes; mov ecx, eax -> thiscall, ret 0x58 (a liga, +0x3e, o vetor, +0x3c,
#   +4, a tabela, as rodadas, +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano, k, o extra, +0x51, 1, +0xab, +0xd9, +0xa7, 0, 2;
#   a original empurra os words e bytes com lixo nos bytes altos: a cópia passa os valores limpos);
# - 0x9452ca (free): push; call (duas vezes); add esp, 8 -> cdecl;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparadores: só a original (alocam), com o plano feito antes: os clubes de cada código na ordem do banco (a tabela:
# as 4 linhas na ordem e a devolução; as divisões: os clubes na tabela da fase, como conjunto, +0x3a = o código e
# +0x30 = k).
# Determinismo: os clubes vêm do banco, na ordem dele; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqs1_tabela_da_mls', 0x908560, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqs1_segunda_divisao_da_mls', 0x908fc0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('cqs1_terceira_divisao_da_mls', 0x9091e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as três divisões da MLS: a tabela da primeira e as fases da segunda e da terceira',
        'the three MLS divisions: the first division table and the second and third division stages',
        [('cqs1_tabela_da_mls', 'UsaMls::create_table', '0x908560', 'usa_mls.cpp', 'thiscall, ret 0',
          'A tabela da MLS é a da primeira divisão: os quatro clubes da competição com o código de divisão 0x413, na ordem do banco; dá certo se achou os quatro',
          'The MLS table is the first division one: the four clubs of the competition with division code 0x413, in database order; succeeds if it found all four'),
         ('cqs1_segunda_divisao_da_mls', 'UsaMls::create_second_division_stage', '0x908fc0', 'usa_mls.cpp', 'thiscall, ret 0',
          'A segunda divisão da MLS (código 0x47b): uma fase de pontos corridos com os quatro clubes dela (na ordem do banco), os mesmos turnos, pontos, critérios de desempate e vagas da liga, na fase 0; os três campeões de divisão e os cinco melhores dos outros vão ao play-off',
          'The second MLS division (code 0x47b): a round-robin stage with its four clubs (in database order), the same rounds, points, tie-breakers and places as the league, as stage 0; the three division winners and the five best of the rest go to the play-off'),
         ('cqs1_terceira_divisao_da_mls', 'UsaMls::create_third_division_stage', '0x9091e0', 'usa_mls.cpp', 'thiscall, ret 0',
          'A terceira divisão da MLS (código 0x414): o mesmo molde da segunda, na fase 1; a fase atual fica 1, de onde a fase seguinte passa ao play-off',
          'The third MLS division (code 0x414): the same template as the second, as stage 1; the current stage becomes 1, from which the next stage goes to the play-off')])
CAMPOS = [('clube +0x57: a competição do clube (o registro); ponteiro; lê', 'club +0x57: the club competition (the record); pointer; reads'),
          ('[0xae23b4] / [0xae235c]: os clubes do banco (0x245 bytes) / quantos; lê', '[0xae23b4] / [0xae235c]: the database clubs (0x245 bytes) / how many; reads'),
          ('liga +0x3e: a quantidade de clubes; valor; grava 4 (tabela), lê', 'league +0x3e: the club count; value; writes 4 (table), reads'),
          ('liga +0xb1: a tabela; ponteiro; grava (tabela)', 'league +0xb1: the table; pointer; writes (table)'),
          ('liga +0xc: as fases (vetor de ponteiros); ponteiro; grava as vagas 0 e 1', 'league +0xc: the stages (pointer array); pointer; writes slots 0 and 1'),
          ('liga +0x30: a fase atual; valor; grava 0 e 1', 'league +0x30: the current stage; value; writes 0 and 1'),
          ('liga +0x3c, +0xa7, +0xab, +0xbe, +0xc2..+0xc5, +0xd9, +0x40, +0x51, +4: turnos, jogos por time, vagas, pontos, categoria, desempate, bits de regra, ano, importância, registro; valor; lê', 'league +0x3c, +0xa7, +0xab, +0xbe, +0xc2..+0xc5, +0xd9, +0x40, +0x51, +4: rounds, games per club, places, points, category, tie-breakers, rule bits, year, importance, record; value; reads'),
          ('fase +0x3a: o código da fase principal; valor; grava o código da divisão', 'stage +0x3a: the main stage code; value; writes the division code')]
INTERESSA = []
