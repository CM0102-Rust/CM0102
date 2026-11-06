# Os play-offs das Séries A e B (bra_nat_first.cpp, bra_nat_second.cpp; agente C da frente das competições,
# 05/11/2026): a fase seguinte (advance_stage, slot 11) e o play-off dos 4 primeiros (create_play_off_stage) das duas
# Séries. A e B têm o mesmo molde, conferido instrução a instrução (só mudam o fonte e as linhas das caixas).
# Cada uma roda uma vez na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt), no fim do Brasileiro
# de 2001.
# - 0x42a950 BraNatFirst::advance_stage e 0x42a9e0 BraNatFirst::create_play_off_stage (Série A);
# - 0x42cdd0 BraNatSecond::advance_stage e 0x42ce60 BraNatSecond::create_play_off_stage (Série B).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - o play-off, da fase seguinte: call direto com o ecx da liga, sem push -> thiscall, ret 0;
# - 0x944e46 (malloc): push 0x10; call; add esp, 4 -> cdecl; 0x944cf1 (operator new): push 0xb2; call; add esp, 4 -> cdecl;
# - a virtual +0x3c (create_round_table): push 0; push &extra; push &rodadas; push 0; mov ecx, esi -> thiscall, ret 0x10;
# - 0x522e00 (Cup_Stage::Cup_Stage): 15 pushes; mov ecx, eax -> thiscall, ret 0x3c (aqui com n = 4, o 13º argumento 0
#   e as sementes nulas; a cópia passa as words limpas);
# - 0x9452ca (free): push t; call; add esp, 4 e push lista; call; add esp, 4 -> cdecl;
# - 0x51c800 (Cup::set_up_round): push 0; mov ecx, [[+0xc]] -> thiscall, ret 4;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparadores: só a original, com o plano (a fase seguinte: +0x30 + 1; o play-off aloca e sorteia: os 4 clubes na
# lista da fase, como conjunto, +0x3e = 4, +4 e +0x30 = 0; o modelo da cqn_9).
# Determinismo: a lista vem da tabela; nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqq7_fase_seguinte_da_serie_a', 0x42a950, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqq7_play_off_da_serie_a', 0x42a9e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('cqq7_fase_seguinte_da_serie_b', 0x42cdd0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqq7_play_off_da_serie_b', 0x42ce60, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os play-offs das Séries A e B: a fase seguinte e o mata-mata dos 4 primeiros',
        'the play-offs of Series A and B: the next stage and the knockout of the top 4',
        [('cqq7_fase_seguinte_da_serie_a', 'BraNatFirst::advance_stage', '0x42a950', 'bra_nat_first.cpp', 'thiscall, ret 0',
          'A Série A tem os pontos corridos (a fase −1) e o play-off (a fase 0): quando os pontos corridos acabam, a fase passa a 0 e o play-off é montado; depois da última fase é erro (a caixa)',
          'Series A has the round robin (stage −1) and the play-off (stage 0): when the round robin ends, the stage becomes 0 and the play-off is built; past the last stage it is an error (the box)'),
         ('cqq7_play_off_da_serie_a', 'BraNatFirst::create_play_off_stage', '0x42a9e0', 'bra_nat_first.cpp', 'thiscall, ret 0',
          'O play-off da Série A: os 4 primeiros dos pontos corridos, 1º × 4º e 2º × 3º, num mata-mata (sem sementes: a ordem da lista) [inferência: semifinais e final em ida e volta] que decide o campeão brasileiro',
          'The Series A play-off: the top 4 of the round robin, 1st v 4th and 2nd v 3rd, in a knockout (no seeds: the list order) [inference: two-legged semi-finals and final] that decides the Brazilian champion'),
         ('cqq7_fase_seguinte_da_serie_b', 'BraNatSecond::advance_stage', '0x42cdd0', 'bra_nat_second.cpp', 'thiscall, ret 0',
          'A Série B tem os pontos corridos (a fase −1) e o play-off (a fase 0): quando os pontos corridos acabam, a fase passa a 0 e o play-off é montado; depois da última fase é erro (a caixa)',
          'Series B has the round robin (stage −1) and the play-off (stage 0): when the round robin ends, the stage becomes 0 and the play-off is built; past the last stage it is an error (the box)'),
         ('cqq7_play_off_da_serie_b', 'BraNatSecond::create_play_off_stage', '0x42ce60', 'bra_nat_second.cpp', 'thiscall, ret 0',
          'O play-off da Série B: os 4 primeiros dos pontos corridos, 1º × 4º e 2º × 3º, num mata-mata (sem sementes) que decide o campeão da Série B [inferência: os acessos à Série A saem dele]',
          'The Series B play-off: the top 4 of the round robin, 1st v 4th and 2nd v 3rd, in a knockout (no seeds) that decides the Series B champion [inference: the promotions to Series A come from it]')])
CAMPOS = [('liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; lê', 'league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; reads'),
          ('liga +0xc: as fases (vetor de ponteiros); ponteiro; grava a primeira', 'league +0xc: the stages (pointer array); pointer; writes the first'),
          ('liga +0x2c: o número de fases; valor; lê (fase seguinte)', 'league +0x2c: the number of stages; value; reads (next stage)'),
          ('liga +0x30: a fase atual (−1 = pontos corridos); valor; lê e grava', 'league +0x30: the current stage (−1 = round robin); value; reads and writes'),
          ('liga +0x40: o ano da temporada; valor; lê', 'league +0x40: the season year; value; reads'),
          ('fase de copa +0xa7: a lista dos times (6 bytes, +0 o clube); ponteiro; lê (comparador)', 'cup stage +0xa7: the team list (6 bytes, +0 the club); pointer; reads (comparator)')]
INTERESSA = []
