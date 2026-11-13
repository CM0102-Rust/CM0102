# The Turkish Cup rounds and the Greek Super Cup (tur_cup.cpp, gre_super.cpp). Reader mwd (12/11/2026), round 2 (the
# round tables 2 calls each: creation and the turnover; the constructor 1 call).
# - 0x8fc670 TurCup::create_round_table: seven rounds (generated from the disassembly; all 16 fields in every round).
# - 0x5ec040 GreSuper::GreSuper: the national cup constructor template without prize fund nor rank_teams
#   (construct_east_nation_cup, mwd_8), checked instruction by instruction against CroCup's 0x517480.
# - 0x5ec2f0 GreSuper::create_round_table: one round, the 16 fields plus the word +0x11 = 0 (outside the bytes the
#   comparer reads).
# Conventions: as in mwd_8. Comparers: the round tables side by side (compare_cup_round_table); the constructor only the
# original. Equal in cm0102-1080.exe.
# Depends on: construct_east_nation_cup / compare_east_nation_cup_ctor (mwd_8). super_cup_round_table is reused by mwd_21.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('tur_cup_create_round_table', 0x8fc670, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('construct_gre_super', 0x5ec040, [0x6a, 0xff, 0x68, 0x79, 0xaa, 0x95, 0x00]),
           ('gre_super_create_round_table', 0x5ec2f0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as rodadas da Copa da Turquia e a Supercopa da Grécia', 'the Turkish Cup rounds and the Greek Super Cup',
        [('tur_cup_create_round_table', 'TurCup::create_round_table', '0x8fc670', 'tur_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Turquia tem sete rodadas de jogo único: a primeira (36 times) sorteada em 17/10 e jogada em 25/10; a segunda (10 novos) 31/10 e 8/11; a terceira (18 novos) 5/12 e 13/12; as oitavas 9/1 e 17/1 do ano seguinte, nada aos vencedores e 3 mil a cada perdedor; as quartas 10/1 e 31/1; as semifinais 6/2 e 14/2; a final 15/2 e 2/5 no local 4 [inferência: campo neutro]',
          'rule: the Turkish Cup has seven single-leg rounds: the first (36 teams) drawn 17/10, played 25/10; the second (10 new) 31/10 and 8/11; the third (18 new) 5/12 and 13/12; the round of 16 9/1 and 17/1 of the next year, nothing to the winners and 3,000 to each loser; the quarter-finals 10/1 and 31/1; the semi-finals 6/2 and 14/2; the final 15/2 and 2/5 at venue 4 [inference: a neutral ground]'),
         ('construct_gre_super', 'GreSuper::GreSuper', '0x5ec040', 'gre_super.cpp', 'thiscall, ret 8',
          'regra: a Supercopa da Grécia ao criar a carreira: a regra de disciplina 0x20, 7 reservas e 3 substituições e, sem estado gravado, os dois clubes (o campeão da liga e o da copa), a rodada, a árvore de jogos e as estatísticas; sem fundo de prêmios nem posições previstas',
          'rule: the Greek Super Cup at career creation: discipline rule 0x20, 7 substitutes and 3 substitutions and, with no saved state, its two clubs (the league champion and the cup winner), the round, the fixture tree and the statistics; no prize fund nor predicted positions'),
         ('gre_super_create_round_table', 'GreSuper::create_round_table', '0x5ec2f0', 'gre_super.cpp', 'thiscall, ret 0x10',
          'regra: a Supercopa da Grécia é um jogo único entre 2 clubes, sorteado em 5/7 e jogado em 2/8 no local 4 [inferência: campo neutro], desempate 3, categoria 6, sem prêmios',
          'rule: the Greek Super Cup is a single match between 2 clubs, drawn on 5/7 and played on 2/8 at venue 4 [inference: a neutral ground], tie rule 3, category 6, no prizes')])
CAMPOS = [('copa +4, +0x40, +0x50, +0xa3, +8; a rodada de copa (0x68 bytes, +0x11 incluída)', 'cup +4, +0x40, +0x50, +0xa3, +8; the cup round (0x68 bytes, +0x11 included)')]
