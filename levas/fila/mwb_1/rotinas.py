# A Copa Challenge escocesa (sco_chal_cup.cpp): o construtor, a tabela das rodadas e os inscritos. Leitor mwb (12/11/2026),
# lista do mundo (Escócia e País de Gales).
# - 0x7ec980 (ScoChalCup::ScoChalCup): o molde de construtor das copas escocesas e da galesa (construct_british_cup,
#   usado também pelos itens mwb_2 e mwb_3: integrar este junto ou antes deles).
# - 0x7ecc40 (ScoChalCup::create_round_table): 5 rodadas de jogo único de julho a outubro.
# - 0x7ed010 (ScoChalCup::collect_entrants): os 30 clubes da First, Second e Third escocesas.
# Convenções conferidas: o construtor é thiscall, ret 8 (dentro: 0x518640 mov ecx -> thiscall, ret 0; 0x51fc00 push 1;
# mov ecx -> thiscall, ret 4; a virtual +0x3c push 0; push &+0x3a; push &+0x3c; push -1 -> ret 0x10; 0x518790 e
# 0x5223a0 mov ecx -> thiscall, ret 0; 0x944cf1 cdecl; 0x49ee70 push this; mov ecx -> thiscall, ret 4); a tabela é
# thiscall, ret 0x10 (0x521e60/0x521eb0 cdecl); os inscritos thiscall, ret 0 (0x944e46 cdecl).
# Comparadores: o construtor só a original (aloca), com o plano; a tabela e os inscritos lado a lado.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_sco_chal_cup', 0x7ec980, [0x6a, 0xff, 0x68, 0xb9, 0x0a, 0x96, 0x00]),
           ('sco_chal_cup_create_round_table', 0x7ecc40, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('sco_chal_cup_collect_entrants', 0x7ed010, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa Challenge escocesa: o construtor, as rodadas e os inscritos',
        'the Scottish Challenge Cup: the constructor, the rounds and the entrants',
        [('construct_sco_chal_cup', 'ScoChalCup::ScoChalCup', '0x7ec980', 'sco_chal_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa Challenge nasce com os 30 clubes das três divisões de baixo, 5 reservas e 3 substituições, a disciplina das copas escocesas (0x19) e um fundo de prêmios de 200 mil; depois a árvore de confrontos, as estatísticas e as posições previstas',
          'rule: the Challenge Cup is created with the 30 clubs of the three lower divisions, 5 substitutes and 3 substitutions, the discipline rule of the Scottish cups (0x19) and a prize fund of 200,000; then the fixture tree, the statistics and the predicted positions'),
         ('sco_chal_cup_create_round_table', 'ScoChalCup::create_round_table', '0x7ecc40', 'sco_chal_cup.cpp', 'thiscall, ret 0x10',
          'regra: 5 rodadas de jogo único: 1ª sorteada em 28/6 e jogada em 10/7 (28 clubes), 2ª 12/7 e 17/7 (16: os 14 vencedores e mais 2), quartas 19/7 e 24/7, semifinal 26/7 e 31/7, final sorteada em 2/9 e jogada em 14/10 em campo neutro; sem prêmio por rodada',
          'rule: 5 single-match rounds: the 1st drawn on 28/6 and played on 10/7 (28 clubs), the 2nd 12/7 and 17/7 (16: the 14 winners and 2 more), quarter-finals 19/7 and 24/7, semi-finals 26/7 and 31/7, the final drawn on 2/9 and played on 14/10 at a neutral ground; no prize per round'),
         ('sco_chal_cup_collect_entrants', 'ScoChalCup::collect_entrants', '0x7ed010', 'sco_chal_cup.cpp', 'thiscall, ret 0',
          'regra: entram os 30 clubes da First, Second e Third escocesas, na ordem do banco (a Premier não joga); devolve 1 com a lista cheia, 0 se faltar clube e -1 sem memória',
          'rule: the 30 clubs of the Scottish First, Second and Third Divisions enter, in database order (the Premier does not play); returns 1 with the list full, 0 when clubs are missing and -1 out of memory')])
CAMPOS = [('copa +0xab rodada atual, +0x44, +0x30, +0x2c, +0x42, +0x49/+0x4a banco, +0x50 disciplina, +0xb1, +0x28 fundo de prêmios, +0xa3 tabela das rodadas, +0x3c/+0x3a; valor/ponteiro; grava (construtor)',
           'cup +0xab current round, +0x44, +0x30, +0x2c, +0x42, +0x49/+0x4a bench, +0x50 discipline, +0xb1, +0x28 prize fund, +0xa3 round table, +0x3c/+0x3a; value/pointer; writes (constructor)'),
          ('copa +0x3e quantos times, +0xa7 tabela dos times (6 bytes: clube, classe, estado); grava (inscritos)',
           'cup +0x3e team count, +0xa7 team table (6 bytes: club, class, status); writes (entrants)')]
