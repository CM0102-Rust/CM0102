# A virada da Série B e as posições finais da Série C1 B (ita_ser_b.cpp, ita_ser_c1b.cpp). Leitor mwf (12/11/2026),
# rodada 2 (o mundo até 2002-08-01; docs/cobertura_mundo.txt: a virada 1 vez, as posições 2 vezes).
# - 0x646960 (ItaSerB::start_new_season): corpo próprio: libera o vetor de fases sem destruir as fases e zera +0xc e
#   +0x2c; e grava +0x4c = 1 no fim da virada (como a Série A; as outras irmãs deixam 0): possível defeito do
#   original [inferência: o fim da temporada seguinte pode ser tomado como já encerrado].
# - 0x650050 (ItaSerC1b::record_final_positions, a virtual +0xa4 da C1 B): a League::record_final_positions (0x68aa30,
#   já em Rust) e depois o mata-mata do acesso (fase 0): o vencedor fica em 2o, o vice em 3o e os outros 4o, 5o...
# Convenções: virtual +0x08 / +0xa4 (mov ecx; call [eax+n], sem push) -> thiscall, ret 0.
# Comparadores: a virada só a original, com o ano +1; as posições lado a lado (só grava +0x5b..+0x5f dos clubes).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ita_ser_b_start_new_season', 0x646960, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ita_ser_c1b_record_final_positions', 0x650050, [0x51, 0x8b, 0x41, 0x0c, 0x33, 0xd2])]
DOCS = ('a virada da Série B e as posições finais da Série C1 B',
        'the Serie B season turnover and the Serie C1 B final positions',
        [('ita_ser_b_start_new_season', 'ItaSerB::start_new_season', '0x646960', 'ita_ser_b.cpp', 'thiscall, ret 0',
          'regra: a virada da temporada da Série B arquiva a temporada, limpa a tabela e as datas, descarta o vetor de fases, passa ao ano seguinte e refaz as regras, a tabela com os 20 clubes, os jogos e a ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros; marca a temporada como encerrada já no começo (como a Série A; possível defeito do original [inferência])',
          'rule: the Serie B season turnover archives the season, clears the table and the dates, drops the stage array, moves to the next year and rebuilds the rules, the table with the 20 clubs, the fixtures and the order; then the predicted positions, the board expectations and the referee renewal; it marks the season as closed right at the start (as Serie A does; a possible defect of the original [inference])'),
         ('ita_ser_c1b_record_final_positions', 'ItaSerC1b::record_final_positions', '0x650050', 'ita_ser_c1b.cpp', 'thiscall, ret 0',
          'regra: as posições finais da Série C1 B são as da tabela, mas o mata-mata do acesso as corrige: o vencedor fica em 2o, o vice em 3o e os outros em 4o, 5o..., na ordem da tabela do mata-mata',
          "rule: the Serie C1 B final places are the table's, but the promotion play-off overrides them: the winner 2nd, the runner-up 3rd and the others 4th, 5th..., in the order of the play-off's team table")])
CAMPOS = [('liga +0x4c (a Série B grava 1 no fim da virada), +0xc e +0x2c (Série B: zerados), +0x30, +0x40; valor/ponteiro; grava',
           'league +0x4c (Serie B writes 1 at the end of the turnover), +0xc and +0x2c (Serie B: zeroed), +0x30, +0x40; value/pointer; writes'),
          ('clube +0x5b (última divisão), +0x5f (última posição); valor; grava', 'club +0x5b (last division), +0x5f (last position); value; writes')]
