# Os mata-matas da Copa Africana de Nações e da Recopa asiática (african_nations.cpp, em intercomp\, e
# asia_cup_winner.cpp, em eurocomp\). Frente das competições (Agente E, 06/11/2026), carreira de 8 ligas (a Copa
# Africana de 2002 e a Recopa asiática de 2001 ficam dentro da janela).
# - 0x402090 (AfricanNations::create_knockout_stage): as quartas cruzadas. Duas mutações (outros cruzamentos).
# - 0x40dfa0 (AsiaCupWinner::advance_stage): a fase seguinte (passageira).
# - 0x40e030 (AsiaCupWinner::create_finals_stage): os 4 vivos das duas zonas, o par do meio cruzado. Uma mutação (sem
#   cruzar).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as quartas e as finais são thiscall, ret 0 (call direto das fases seguintes);
# a fase seguinte é a virtual +0x28 (thiscall, ret 0). Dentro: 0x6826d0 (push p; mov ecx, grupo -> thiscall, ret 4),
# a virtual +0x3c (4 pushes; mov ecx -> thiscall, ret 0x10), 0x522e00 (15 pushes; mov ecx, novo -> thiscall, ret
# 0x3c), 0x51c800 e 0x51c410 (push 0; mov ecx, fase -> thiscall, ret 4), 0x944e46, 0x944cf1, 0x9452ca e a caixa.
# Comparadores: só a original (alocam e montam ou sorteiam a rodada), com o plano: a fase nova com os cruzados (lidos
# antes) e +0x30; a fase seguinte confere +0x30.
# Determinismo: nada lê pilha, heap ou endereço como valor (com menos de 4 vivos a original deixa vagas com o que o
# malloc deu; nesta carreira são 4).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqt13_africana_quartas', 0x402090, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('cqt13_recopa_asiatica_fase_seguinte', 0x40dfa0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqt13_recopa_asiatica_finais', 0x40e030, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os mata-matas da Copa Africana de Nações e da Recopa asiática: as quartas, a fase seguinte e as finais',
        "the African Nations Cup and Asian Cup Winners' Cup knockouts: the quarter-finals, the next stage and the finals",
        [('cqt13_africana_quartas', 'AfricanNations::create_knockout_stage', '0x402090', 'african_nations.cpp', 'thiscall, ret 0', 'regra: as quartas da Copa Africana cruzam os grupos: 1º A × 2º B, 1º C × 2º D, 2º C × 1º D e 1º B × 2º A, numa fase de copa (a fase 3) com a primeira rodada montada na hora', 'rule: the African Nations Cup quarter-finals cross the groups: A winner × B runner-up, C winner × D runner-up, C runner-up × D winner and B winner × A runner-up, in a cup stage (stage 3) with the first round set up at once'),
         ('cqt13_recopa_asiatica_fase_seguinte', 'AsiaCupWinner::advance_stage', '0x40dfa0', 'asia_cup_winner.cpp', 'thiscall, ret 0', 'regra: a Recopa asiática avança uma fase quando a atual acaba; depois dos mata-matas das duas zonas (ao chegar a 1) monta as finais; passar da última fase é erro', "rule: the Asian Cup Winners' Cup moves on one stage when the current one ends; after the two zones' knockouts (on reaching 1) it builds the finals; going past the last stage is an error"),
         ('cqt13_recopa_asiatica_finais', 'AsiaCupWinner::create_finals_stage', '0x40e030', 'asia_cup_winner.cpp', 'thiscall, ret 0', 'regra: as finais da Recopa asiática têm os 4 vivos dos mata-matas das duas zonas, com o par do meio cruzado (os dois de cada zona não se enfrentam na semifinal), numa fase de copa (a fase 1) com a primeira rodada sorteada na hora', "rule: the Asian Cup Winners' Cup finals have the 4 survivors of the two zones' knockouts, with the middle pair crossed (the two of each zone do not meet in the semi-final), in a cup stage (stage 1) with the first round drawn at once")])
CAMPOS = [('copa +0xb1 tabela do grupo A, +0xc fases ([0..2] os grupos B a D, [3] as quartas); ponteiro; lê e grava (quartas)', 'cup +0xb1 group A table, +0xc stages ([0..2] groups B to D, [3] the quarter-finals); pointer; reads and writes (quarter-finals)'),
          ('copa +0xa7 tabela do mata-mata da zona 1 (6 bytes: +5 situação, 1 = vivo), +0x3e times, +0xc fases ([0] zona 2, [1] as finais); ponteiro/valor; lê e grava (finais)', 'cup +0xa7 zone 1 knockout table (6 bytes: +5 status, 1 = alive), +0x3e teams, +0xc stages ([0] zone 2, [1] the finals); pointer/value; reads and writes (finals)'),
          ('copa +0x2c fases, +0x30 fase atual; valor; lê e grava', 'cup +0x2c stages, +0x30 current stage; value; reads and writes')]
INTERESSA = ['CMRTE']
