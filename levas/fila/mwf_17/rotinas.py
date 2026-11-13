# A virada italiana, puxada pela Série A (ita_ser_a.cpp). Leitor mwf (12/11/2026), rodada 2 (o mundo até 2002-08-01;
# docs/cobertura_mundo.txt: 1 vez, na virada italiana de meados de 2002).
# - 0x641700 (ItaSerA::start_new_season): a cadeia (virtual +0xb0 com 1), a troca com a Série D e o reagrupamento da
#   C2 (com a C2 carregada) ou a troca sem a C2, o reagrupamento da C1; a Série A refeita (descarta o vetor de fases
#   sem destruí-las, +0x2c = 0, +0x30 = −1 e +0x4c = 1, como na Série B); o teto de divisão nos grupos mais baixos
#   carregados; as viradas da B, da C1 A, da C1 B (e da C2); previstas, expectativas, árbitros.
# Convenções: virtual +0x08 -> thiscall, ret 0; as rotinas da troca e dos reagrupamentos chamadas com mov ecx, esi e
# sem argumentos; 0x68a980 (4 pushes -> thiscall, ret 0x10).
# Comparador: só a original (move clubes e refaz as ligas), com o ano +1.
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ita_ser_a_start_new_season', 0x641700, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a virada italiana',
        'the Italian season turnover',
        [('ita_ser_a_start_new_season', 'ItaSerA::start_new_season', '0x641700', 'ita_ser_a.cpp', 'thiscall, ret 0',
          'regra: a virada italiana, puxada pela Série A: acesso e rebaixamento, a troca com o nível de baixo (a Série D e o reagrupamento da C2 com a C2 carregada; senão a troca com a C2), o reagrupamento da C1, a Série A refeita (ano seguinte, regras, 18 clubes, jogos, ordem), o teto de divisão nos grupos mais baixos carregados e as viradas da B, da C1 A, da C1 B (e da C2); depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros',
          'rule: the Italian turnover, led by Serie A: promotion and relegation, the exchange with the level below (Serie D and the C2 regrouping with Serie C2 loaded, otherwise the C2 exchange), the C1 regrouping, Serie A rebuilt (next year, rules, 18 clubs, fixtures, order), the division tier capped in the lowest loaded groups and the turnovers of Serie B, C1 A, C1 B (and C2); then the predicted positions, the board expectations and the referee renewal')])
CAMPOS = [('liga +0x4c (grava 1 no fim da virada), +0xc e +0x2c (zerados), +0x30, +0x40, +0xb1, +0xba; valor/ponteiro; grava',
           'league +0x4c (writes 1 at the end of the turnover), +0xc and +0x2c (zeroed), +0x30, +0x40, +0xb1, +0xba; value/pointer; writes')]
