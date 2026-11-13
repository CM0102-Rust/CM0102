# The Croatian leagues: the tie-break play-off (cro_a1.cpp, cro_a2b.cpp). Reader mwd (12/11/2026), round 2 (1 call each,
# at the end of the league in May 2002).
# - 0x514c60 CroA1::create_playoff_stage, 0x517210 CroA2b::create_tiebreak_playoff_stage: the same 624-byte template
#   (only the file and the box line, 0x24b and 0x23a).
# Conventions: thiscall, ret 0; the virtual +0x94 (thiscall, ret 0) and +0x3c (push 0; push &extra; push &rounds; push
# 0 -> thiscall, ret 0x10); 0x944cf1 (cdecl); 0x522e00 (15 pushes; mov ecx, new -> thiscall, ret 0x3c); 0x9452ca
# (cdecl); 0x51c800 (push 0 -> thiscall, ret 4).
# Comparer: only the original runs (it builds a Cup_Stage); the pair count and the stage's list checked against the
# plan read from the table before the call.
# Note: the scan narrows the best/worst possible positions of the level rows (+0x39/+0x3a) and marks the pairs with
# status 1. Equal in cm0102-1080.exe.
# Depends on: TIEBREAK_PAIRS (mwd_1), croatian_tiebreak_round (mwd_4, through the virtual schedule).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('cro_a1_create_playoff_stage', 0x514c60, [0x6a, 0xff, 0x68, 0x9e, 0x73, 0x95, 0x00]),
           ('cro_a2b_create_tiebreak_playoff_stage', 0x517210, [0x6a, 0xff, 0x68, 0x9e, 0x74, 0x95, 0x00])]
_R = 'no fim da liga, os clubes empatados em pontos com o clube logo abaixo (nenhum dos dois rebaixado) jogam um jogo-desempate de uma rodada (fase 0, em maio): cada par de linhas empatadas entra no mata-mata e as posições que ainda podem alcançar são estreitadas; sem par empatado, a temporada acaba sem desempate'
_E = 'at the end of the league, the clubs level on points with the club right below them (neither relegated) play a one-round tie-break play-off (stage 0, in May): each pair of level rows goes into the play-off and the positions they can still reach are narrowed; with no level pair, the season ends without tie-breaks'
DOCS = ('as ligas croatas: o jogo-desempate', 'the Croatian leagues: the tie-break play-off',
        [('cro_a1_create_playoff_stage', 'CroA1::create_playoff_stage', '0x514c60', 'cro_a1.cpp', 'thiscall, ret 0', 'regra: Primeira Divisão croata: ' + _R, 'rule: Croatian First Division: ' + _E),
         ('cro_a2b_create_tiebreak_playoff_stage', 'CroA2b::create_tiebreak_playoff_stage', '0x517210', 'cro_a2b.cpp', 'thiscall, ret 0', 'regra: grupo sul da Segunda Divisão croata: ' + _R, 'rule: Croatian Second Division south group: ' + _E)])
CAMPOS = [('liga +0xee pares, +0x2c, +0x30, +0xe6, +0xc, +0xc5; linha da tabela +0x19 pontos, +0x37, +0x39, +0x3a', 'league +0xee pairs, +0x2c, +0x30, +0xe6, +0xc, +0xc5; table row +0x19 points, +0x37, +0x39, +0x3a')]
