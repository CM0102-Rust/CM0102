# The Russian and Turkish cups: the constructors and the Russian round table (rus_cup.cpp, tur_cup.cpp). Reader mwd
# (12/11/2026), round 2 (the constructors 1 call each at career creation; the round table 2 calls).
# - 0x7e9210 RusCup::RusCup (the stage vector from +0x2c, always 0 here), 0x8fc3b0 TurCup::TurCup: the national cup
#   constructor template (construct_east_nation_cup, mwd_8), checked instruction by instruction against CroCup's 0x517480.
# - 0x7e9530 RusCup::create_round_table: six rounds (generated from the disassembly; all 16 fields in every round).
# Conventions: as in mwd_8. Comparers: the constructors only the original; the rounds side by side. Equal in
# cm0102-1080.exe.
# Depends on: construct_east_nation_cup / compare_east_nation_cup_ctor (mwd_8).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_rus_cup', 0x7e9210, [0x6a, 0xff, 0x68, 0xf9, 0x08, 0x96, 0x00]),
           ('rus_cup_create_round_table', 0x7e9530, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('construct_tur_cup', 0x8fc3b0, [0x6a, 0xff, 0x68, 0x79, 0x4f, 0x96, 0x00])]
DOCS = ('as copas da Rússia e da Turquia: construtores e as rodadas da russa', 'the Russian and Turkish cups: constructors and the Russian rounds',
        [('construct_rus_cup', 'RusCup::RusCup', '0x7e9210', 'rus_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Rússia ao criar a carreira: a regra de disciplina 0x27, 7 reservas e 3 substituições e, sem estado gravado, sem vetor de fases (+0x2c é 0), os 48 inscritos, o fundo de prêmios de 800 mil, as seis rodadas, a árvore de jogos, as estatísticas e as posições previstas',
          'rule: the Russian Cup at career creation: discipline rule 0x27, 7 substitutes and 3 substitutions and, with no saved state, no stage vector (+0x2c is 0), the 48 entrants, an 800,000 prize fund, the six rounds, the fixture tree, the statistics and the predicted positions'),
         ('rus_cup_create_round_table', 'RusCup::create_round_table', '0x7e9530', 'rus_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Rússia tem seis rodadas de jogo único: a primeira (32 times) sorteada em 23/6 e jogada em 30/6; a segunda (16 novos) 10/7 e 12/10; as oitavas 13/10 e 13/11; as quartas 14/11 e 4/4 do ano seguinte; as semifinais 5/4 e 3/5; a final 4/5 e 21/5 no local 4 [inferência: campo neutro]; uma parte do fundo por rodada vencida',
          'rule: the Russian Cup has six single-leg rounds: the first (32 teams) drawn 23/6, played 30/6; the second (16 new) 10/7 and 12/10; the round of 16 13/10 and 13/11; the quarter-finals 14/11 and 4/4 of the next year; the semi-finals 5/4 and 3/5; the final 4/5 and 21/5 at venue 4 [inference: a neutral ground]; a share of the fund per round won'),
         ('construct_tur_cup', 'TurCup::TurCup', '0x8fc3b0', 'tur_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Turquia ao criar a carreira: a regra de disciplina 0x26, 7 reservas e 3 substituições e, sem estado gravado, os 64 inscritos, o fundo de prêmios de 600 mil, as sete rodadas, a árvore de jogos, as estatísticas e as posições previstas',
          'rule: the Turkish Cup at career creation: discipline rule 0x26, 7 substitutes and 3 substitutions and, with no saved state, the 64 entrants, a 600,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted positions')])
CAMPOS = [('copa +4, +0x40, +0xab, +0x44, +0x30, +0x2c, +0xc, +0x42, +0x49, +0x4a, +0x50, +0xb1, +0x28, +0xa3, +8; a rodada de copa (0x68 bytes)', 'cup +4, +0x40, +0xab, +0x44, +0x30, +0x2c, +0xc, +0x42, +0x49, +0x4a, +0x50, +0xb1, +0x28, +0xa3, +8; the cup round (0x68 bytes)')]
