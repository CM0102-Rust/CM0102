# A Copa do País de Gales (wel_cup.cpp): o construtor, a tabela das rodadas e os inscritos. Leitor mwb (12/11/2026),
# lista do mundo (Escócia e País de Gales). Depende do mwb_1 (construct_british_cup, club_division_id, put_cup_entrant):
# integrar junto ou depois dele.
# - 0x924420 (WelCup::WelCup): o molde das copas (sem gravar o banco; disciplina 0x28, fundo de 50 mil).
# - 0x9246e0 (WelCup::create_round_table): 5 rodadas de outubro a maio.
# - 0x924ab0 (WelCup::collect_entrants): até 14 clubes da divisão de baixo galesa e a Liga de Gales, 32 ao todo.
# Convenções conferidas: como as do mwb_1.
# Comparadores: o construtor só a original, com o plano; a tabela e os inscritos lado a lado.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_wel_cup', 0x924420, [0x6a, 0xff, 0x68, 0x99, 0x5c, 0x96, 0x00]),
           ('wel_cup_create_round_table', 0x9246e0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wel_cup_collect_entrants', 0x924ab0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a Copa do País de Gales: o construtor, as rodadas e os inscritos',
        'the Welsh Cup: the constructor, the rounds and the entrants',
        [('construct_wel_cup', 'WelCup::WelCup', '0x924420', 'wel_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa de Gales nasce com 32 inscritos, o banco padrão da copa, a disciplina 0x28 e um fundo de prêmios de 50 mil; depois a árvore de confrontos, as estatísticas e as posições previstas',
          "rule: the Welsh Cup is created with 32 entrants, the cup's default bench, discipline rule 0x28 and a prize fund of 50,000; then the fixture tree, the statistics and the predicted positions"),
         ('wel_cup_create_round_table', 'WelCup::create_round_table', '0x9246e0', 'wel_cup.cpp', 'thiscall, ret 0x10',
          'regra: 5 rodadas de jogo único: 1ª sorteada em 19/10 e jogada em 18/11 (os 32), 2ª 19/11 e 10/2, quartas 11/2 e 10/3, semifinal 11/3 e 7/4, final sorteada em 8/4 e jogada em 6/5; sem prêmio por rodada',
          'rule: 5 single-match rounds: the 1st drawn on 19/10 and played on 18/11 (all 32), the 2nd 19/11 and 10/2, quarter-finals 11/2 and 10/3, semi-finals 11/3 and 7/4, the final drawn on 8/4 and played on 6/5; no prize per round'),
         ('wel_cup_collect_entrants', 'WelCup::collect_entrants', '0x924ab0', 'wel_cup.cpp', 'thiscall, ret 0',
          'regra: 32 inscritos: até 14 clubes galeses da divisão de baixo e os clubes da Liga de Gales até completar, na ordem do banco; devolve 1 com a lista cheia, 0 se faltar clube e -1 sem memória',
          'rule: 32 entrants: up to 14 Welsh lower-division clubs and the League of Wales clubs to fill the list, in database order; returns 1 with the list full, 0 when clubs are missing and -1 out of memory')])
CAMPOS = [('copa +0x3e quantos times, +0xa7 tabela dos times (6 bytes: clube, classe, estado); grava (inscritos)',
           'cup +0x3e team count, +0xa7 team table (6 bytes: club, class, status); writes (entrants)')]
