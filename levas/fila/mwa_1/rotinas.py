# The FAI Senior Challenge Cup (ire_chal_cup.cpp, in cups\): the round table, the entrants and the season turnover.
# Whole-world front, round 2 (reader mwa, 12/11/2026), career with ENG IRL NIR to 2002-08-01.
# - 0x634360 (IreChalCup::create_round_table, 2 calls: the constructor at the start of the new game and the turnover);
#   the single-stage round table template (eng_cc_cup_create_single_stage_round_table), checked instruction by
#   instruction (the head against IreChalCup's sisters, the six rounds read by a symbolic reader of the pushes and
#   writes, which reproduces BraCup's table byte for byte).
# - 0x6347d0 (IreChalCup::collect_entrants, 2 calls).
# - 0x634a60 (IreChalCup::start_new_season, 1 call, at the Irish season turn [inference: June 2002]); the turnover
#   template start_new_cup_season (same.py against BraCup 0x427d10: only the entrant routine, the file and the lines).
# Conventions checked at the call sites: create_round_table is the virtual +0x3c (push out; push &extra; push &rounds;
# push stage; mov ecx -> thiscall, ret 0x10); collect_entrants (mov ecx, esi; call -> thiscall, ret 0, from the
# constructor 0x6340a0 and the turnover); start_new_season the virtual +8 (thiscall, ret 0). Inside: 0x944e46 malloc
# (push n; add esp, 4 -> cdecl), 0x90cfc0 utils_random (push n; add esp, 4 -> cdecl), 0x521e60/0x521eb0 (cdecl), the
# error box (show_error_at).
# Comparers: the round table side by side (compare_cup_round_table); the entrants side by side with the generator
# state (compare_irish_cup_entrants: club and status of the 42 entries, the class byte is not written by the
# original); the turnover only the original (it frees and draws), with the plan (the year up by one).
# Determinism: nothing reads stack, heap or an address as a value (the class byte left unwritten is not compared).
LIGAS = 'ENG IRL NIR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('ire_chal_cup_create_round_table', 0x634360, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ire_chal_cup_collect_entrants', 0x6347d0, [0x81, 0xec, 0x24, 0x05, 0x00, 0x00]),
           ('ire_chal_cup_start_new_season', 0x634a60, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da Irlanda (FAI Senior Challenge Cup): a tabela das rodadas, os inscritos e a virada da temporada',
        'the FAI Senior Challenge Cup: the round table, the entrants and the season turnover',
        [('ire_chal_cup_create_round_table', 'IreChalCup::create_round_table', '0x634360', 'ire_chal_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Irlanda tem seis rodadas em jogo único, com jogo-desempate: a primeira sorteada em 1/11 e jogada em 28/11 (20 clubes, 10 confrontos), a segunda sorteada em 6/12 e jogada em 9/1 do ano seguinte (32 clubes: os 10 vencedores e 22 que entram agora), depois 17/1 e 6/2 (16), 14/2 e 4/3 (quartas), 13/3 e 2/4 (semifinais) e a final sorteada em 3/4 e jogada em 30/4 em campo neutro [inferência: local 0x400], cada data levada ao seu dia da semana; desempate de ida e volta 3 [inferência]; categoria 4, e 6 na final; o prêmio do vencedor é uma parte do fundo (−1)',
          'rule: the FAI Senior Challenge Cup has six single-leg rounds with replays: the first drawn on 1/11 and played on 28/11 (20 clubs, 10 ties), the second drawn 6/12 and played 9/1 of the next year (32 clubs: the 10 winners and 22 entering now), then 17/1 and 6/2 (16), 14/2 and 4/3 (quarter-finals), 13/3 and 2/4 (semi-finals) and the final drawn 3/4 and played on 30/4 at a neutral ground [inference: venue 0x400], each moved to its weekday; two-leg tiebreak 3 [inference]; category 4, and 6 in the final; the tie winner\'s prize is a share of the prize fund (−1)'),
         ('ire_chal_cup_collect_entrants', 'IreChalCup::collect_entrants', '0x6347d0', 'ire_chal_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Irlanda tem 42 clubes: cinco sorteados de cada uma das quatro ligas regionais (Munster, Leinster, Ulster e Connacht, nessa ordem, nas vagas 0 a 19) e os 22 clubes da Premier e da First Division (vagas 20 a 41); o sorteio repete até cair num clube da liga da vez',
          'rule: the FAI Senior Challenge Cup takes 42 clubs: five drawn from each of the four regional senior leagues (Munster, Leinster, Ulster and Connacht, in that order, entries 0 to 19) and the 22 clubs of the Premier and First Divisions (entries 20 to 41); the draw repeats until it hits a club of the league in turn'),
         ('ire_chal_cup_start_new_season', 'IreChalCup::start_new_season', '0x634a60', 'ire_chal_cup.cpp', 'thiscall, ret 0',
          'regra: na virada a Copa da Irlanda é refeita: as tabelas da edição anterior são liberadas, as estatísticas arquivadas, o ano sobe um e os inscritos, as regras, a árvore dos confrontos e as posições previstas são feitos de novo',
          'rule: at the season turn the FAI Senior Challenge Cup is rebuilt: last edition\'s tables are freed, the statistics archived, the year goes up by one and the entrants, the rules, the tie tree and the predicted positions are made again')])
CAMPOS = [('copa +0xa7 tabela dos inscritos (6 bytes: clube, classe +4, estado +5), +0x3e quantos; ponteiro/valor; grava',
           'cup +0xa7 entrant table (6 bytes: club, class +4, status +5), +0x3e the count; pointer/value; writes')]
INTERESSA = []
