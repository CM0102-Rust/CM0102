# A Liga dos Campeões asiática: os 32 classificados das duas zonas, a fase da zona oeste e as posições previstas
# (asia_club_champ.cpp, em eurocomp\). Frente das competições (Agente E, 06/11/2026, noite), carreira de 8 ligas (o Japão
# jogável: o campeão da J1 entra; a copa é montada no começo do jogo e joga a temporada 2001/02 dentro da janela).
# - 0x40b950 (AsiaClubChamp::select_qualified_clubs): as 32 vagas. Duas mutações (o vice da J1; 20 escolhas no
#   embaralhamento).
# - 0x40bec0 (AsiaClubChamp::create_second_zone_cup_stage): o mata-mata da zona oeste. Uma mutação (de trás para a
#   frente).
# - 0x40c720 (AsiaClubChamp::rank_teams): as posições previstas. Passageira (sem mutação: só a projeção da copa).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são thiscall, ret 0 (mov ecx, esi; call, do construtor 0x40aae0).
# Dentro: 0x945501 (push 4; push 0x20; add esp, 8 -> cdecl), 0x4afd80 (push lugar; push comp; add esp, 8 -> cdecl),
# 0x4afbd0 (3 pushes; add esp, 0xc -> cdecl), 0x944e46, 0x944cf1 e 0x9452ca (push; add esp, 4 -> cdecl), a virtual
# +0x3c (4 pushes; mov ecx -> thiscall, ret 0x10), 0x522e00 (15 pushes; mov ecx, novo -> thiscall, ret 0x3c), 0x945333
# (4 pushes; add esp, 0x20 em dobro -> cdecl), 0x4a2540 (push posição; push clube; mov ecx, [+8] -> thiscall, ret 8) e
# a caixa (aviso_em).
# Comparadores: só a original (alocam, sorteiam, acrescentam registros), com o plano: os classificados confere as
# zonas das nações e nenhuma repetida; a fase confere os 16 times da zona oeste na tabela dela e +0x30; as previstas
# conferem as posições gravadas pela reputação (empates podem trocar).
# Determinismo: nada lê pilha, heap ou endereço como valor (a lista local de 1.200 clubes só é lida até a conta).
# Provas: em curso.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqu10_asia_classificados', 0x40b950, [0xb8, 0x94, 0x15, 0x00, 0x00]),
           ('cqu10_asia_fase_oeste', 0x40bec0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('cqu10_asia_previstas', 0x40c720, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Liga dos Campeões asiática: os 32 classificados das duas zonas, a fase da zona oeste e as posições previstas',
        'the Asian Club Championship: the 32 qualifiers of the two zones, the west zone stage and the predicted positions',
        [('cqu10_asia_classificados', 'AsiaClubChamp::select_qualified_clubs', '0x40b950', 'asia_club_champ.cpp', 'thiscall, ret 0', 'regra: a Liga dos Campeões asiática tem 32 vagas em duas zonas de 16 (leste: do Extremo Oriente e do Sul da Ásia; oeste: do Oriente Médio e da Ásia Central); o último campeão entra na vaga 0 ou 16 pela zona da sua nação; o campeão da J1 (ou o vice, se o campeão já entrou) e o da liga coreana entram na disputa se as ligas existem; depois os clubes asiáticos, ordenados pela reputação e embaralhados entre os 120 primeiros, dão uma vaga por nação na sua zona, enquanto houver vaga', "rule: the Asian Club Championship has 32 places in two zones of 16 (east: Far East and South Asia; west: Middle East and Central Asia); the last winner takes place 0 or 16 by its nation's zone; the J1 champion (or the runner-up, if the champion is already in) and the Korean league champion join the pool when those leagues exist; then the Asian clubs, sorted by reputation and shuffled among the first 120, give one place per nation in its zone while places remain"),
         ('cqu10_asia_fase_oeste', 'AsiaClubChamp::create_second_zone_cup_stage', '0x40bec0', 'asia_club_champ.cpp', 'thiscall, ret 0', 'regra: a fase 0 da Liga dos Campeões asiática é um mata-mata com os 16 classificados da zona oeste (vagas 16 a 31), em ida e volta [inferência: o 2 do construtor], com as rodadas da tabela da copa', "rule: stage 0 of the Asian Club Championship is a knockout with the 16 west zone qualifiers (places 16 to 31), two-legged [inference: the constructor's 2], on the cup's round table"),
         ('cqu10_asia_previstas', 'AsiaClubChamp::rank_teams', '0x40c720', 'asia_club_champ.cpp', 'thiscall, ret 0', 'regra: as posições previstas da Liga dos Campeões asiática: em cada zona, pela reputação, 1, 3, 5, 7, quatro 9 e oito 17', 'rule: the Asian Club Championship predicted positions: in each zone, by reputation, 1, 3, 5, 7, four 9ths and eight 17ths')])
CAMPOS = [('copa +0xb2 a lista das 32 vagas (ponteiros de clube), +0xb6 a conta; ponteiro/valor; grava (classificados), lê (fase)', 'cup +0xb2 the 32-place list (club pointers), +0xb6 its count; pointer/value; writes (qualifiers), reads (stage)'),
          ('[0xae23a8] + 0x122 × nação, +0x11c: a nação tem a liga [inferência]; valor; lê (classificados)', '[0xae23a8] + 0x122 × nation, +0x11c: the nation has its league [inference]; value; reads (qualifiers)'),
          ('clube +0x53 nação, nação +0x71 continente, clube +0x80 reputação; ponteiro/valor; lê', 'club +0x53 nation, nation +0x71 continent, club +0x80 reputation; pointer/value; reads'),
          ('copa +0xc fases ([0] a da zona oeste), +0x30 fase atual, +0xa7 tabela de times (6 bytes); ponteiro/valor; grava (fase), lê (previstas)', 'cup +0xc stages ([0] the west zone one), +0x30 current stage, +0xa7 team table (6 bytes); pointer/value; writes (stage), reads (predicted)'),
          ('registro de estatística +0x72: a posição prevista do clube; valor; grava (previstas, 0x4a2540)', 'stats record +0x72: the club predicted position; value; writes (predicted, 0x4a2540)')]
INTERESSA = ['CMRTE']
