# A Recopa asiática: os 32 classificados das duas zonas, a fase da zona oeste e as posições previstas
# (asia_cup_winner.cpp, em eurocomp\). Frente das competições (Agente E, 06/11/2026, noite), carreira de 8 ligas (o Japão
# jogável: o campeão da Copa do Imperador entra; a copa é montada no começo do jogo e joga a temporada 2001/02).
# - 0x40d830 (AsiaCupWinner::select_qualified_clubs): as 32 vagas, sem os clubes da Liga dos Campeões. Duas mutações
#   (o vice da Copa do Imperador; 20 escolhas no embaralhamento).
# - 0x40ddf0 (AsiaCupWinner::create_zone_stage): o mata-mata da zona oeste. Uma mutação (de trás para a frente).
# - 0x40e3b0 (AsiaCupWinner::rank_teams): as posições previstas. Passageira.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são thiscall, ret 0 (mov ecx, esi; call, do construtor 0x40cbe0).
# Dentro: as mesmas da cqu_10 e mais 0x40cba0 (3 pushes; add esp, 0xc -> cdecl; devolve o byte do índice) e 0x40e7b0
# (push; add esp, 4 -> cdecl).
# Comparadores: só a original, com o plano (como a cqu_10; os classificados também conferem que nenhum joga a Liga dos
# Campeões asiática).
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas: comparação de 8 ligas até 2002-06-01 (os 12 itens do agente E) 0 div; Rust x2 iguais e detector igual (ab5806e63c96b235); mutações 3/3.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqu11_recopa_classificados', 0x40d830, [0xb8, 0x98, 0x15, 0x00, 0x00]),
           ('cqu11_recopa_fase_oeste', 0x40ddf0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('cqu11_recopa_previstas', 0x40e3b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Recopa asiática: os 32 classificados das duas zonas, a fase da zona oeste e as posições previstas',
        "the Asian Cup Winners' Cup: the 32 qualifiers of the two zones, the west zone stage and the predicted positions",
        [('cqu11_recopa_classificados', 'AsiaCupWinner::select_qualified_clubs', '0x40d830', 'asia_cup_winner.cpp', 'thiscall, ret 0', 'regra: a Recopa asiática tem 32 vagas em duas zonas de 16 (leste e oeste, como a Liga dos Campeões asiática); nenhum clube da Liga dos Campeões asiática entra; o último campeão da Recopa vai à vaga 0 ou 16; o campeão da Copa do Imperador (ou o vice) e o da copa coreana entram na disputa; depois os clubes asiáticos, pela reputação e embaralhados entre os 120 primeiros, dão uma vaga por nação', "rule: the Asian Cup Winners' Cup has 32 places in two zones of 16 (east and west, as in the Asian Club Championship); no Asian Club Championship club enters; the last Cup Winners' Cup winner takes place 0 or 16; the Emperor's Cup winner (or runner-up) and the Korean cup winner join the pool; then the Asian clubs, by reputation and shuffled among the first 120, give one place per nation"),
         ('cqu11_recopa_fase_oeste', 'AsiaCupWinner::create_zone_stage', '0x40ddf0', 'asia_cup_winner.cpp', 'thiscall, ret 0', 'regra: a fase 0 da Recopa asiática é um mata-mata com os 16 classificados da zona oeste, em ida e volta [inferência], com as rodadas da tabela da copa', "rule: stage 0 of the Asian Cup Winners' Cup is a knockout with the 16 west zone qualifiers, two-legged [inference], on the cup's round table"),
         ('cqu11_recopa_previstas', 'AsiaCupWinner::rank_teams', '0x40e3b0', 'asia_cup_winner.cpp', 'thiscall, ret 0', 'regra: as posições previstas da Recopa asiática: em cada zona, pela reputação, 1, 3, 5, 7, quatro 9 e oito 17', "rule: the Asian Cup Winners' Cup predicted positions: in each zone, by reputation, 1, 3, 5, 7, four 9ths and eight 17ths")])
CAMPOS = [('copa +0xb2 a lista das 32 vagas, +0xb6 a conta; ponteiro/valor; grava (classificados), lê (fase)', 'cup +0xb2 the 32-place list, +0xb6 its count; pointer/value; writes (qualifiers), reads (stage)'),
          ('[0xad9c5c] a Liga dos Campeões asiática (cache da 0x40e7b0); ponteiro; lê', '[0xad9c5c] the Asian Club Championship (0x40e7b0 cache); pointer; reads'),
          ('copa +0xc fases, +0x30 fase atual, +0xa7 tabela de times (6 bytes); ponteiro/valor; grava (fase), lê (previstas)', 'cup +0xc stages, +0x30 current stage, +0xa7 team table (6 bytes); pointer/value; writes (stage), reads (predicted)'),
          ('registro de estatística +0x72: a posição prevista do clube; valor; grava (previstas)', 'stats record +0x72: the club predicted position; value; writes (predicted)')]
INTERESSA = ['CMRTE']
