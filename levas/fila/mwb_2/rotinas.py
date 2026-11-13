# A Copa da Escócia (sco_fa_cup.cpp): o construtor, a tabela das rodadas e os inscritos. Leitor mwb (12/11/2026), lista
# do mundo (Escócia e País de Gales). Depende do mwb_1 (construct_british_cup, club_division_id, put_cup_entrant):
# integrar junto ou depois dele.
# - 0x7ed310 (ScoFaCup::ScoFaCup): o molde das copas (fundo de prêmios de 1 milhão).
# - 0x7ed5d0 (ScoFaCup::create_round_table): 7 rodadas de novembro a maio, com replay até a semifinal.
# - 0x7edaf0 (ScoFaCup::collect_entrants): 50 inscritos (8 de fora da liga, Second e Third, First, Premier), com sorteios.
# Convenções conferidas: como as do mwb_1; nos inscritos 0x53b900, 0x4afbd0 (o 2º argumento como palavra), 0x4afb40 e
# 0x90cfc0 são cdecl.
# Comparadores: o construtor só a original, com o plano; a tabela lado a lado; os inscritos lado a lado com o estado do
# gerador (a cópia sem contar os sorteios).
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_sco_fa_cup', 0x7ed310, [0x6a, 0xff, 0x68, 0x09, 0x0b, 0x96, 0x00]),
           ('sco_fa_cup_create_round_table', 0x7ed5d0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('sco_fa_cup_collect_entrants', 0x7edaf0, [0x81, 0xec, 0x7c, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da Escócia: o construtor, as rodadas e os inscritos',
        'the Scottish Cup: the constructor, the rounds and the entrants',
        [('construct_sco_fa_cup', 'ScoFaCup::ScoFaCup', '0x7ed310', 'sco_fa_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Escócia nasce com 50 inscritos, 5 reservas e 3 substituições, a disciplina das copas escocesas (0x19) e um fundo de prêmios de 1 milhão; depois a árvore de confrontos, as estatísticas e as posições previstas',
          'rule: the Scottish Cup is created with 50 entrants, 5 substitutes and 3 substitutions, the discipline rule of the Scottish cups (0x19) and a prize fund of 1,000,000; then the fixture tree, the statistics and the predicted positions'),
         ('sco_fa_cup_create_round_table', 'ScoFaCup::create_round_table', '0x7ed5d0', 'sco_fa_cup.cpp', 'thiscall, ret 0x10',
          'regra: 7 rodadas: 1ª sorteada em 8/11 e jogada em 5/12 (16 clubes), 2ª 6/12 e 2/1 (20: 8 vencedores e 12 que entram), 3ª 3/1 e 29/1 (32: 10 vencedores e os 22 da Premier e da First), 4ª 30/1 e 19/2, quartas 20/2 e 11/3, semifinal 12/3 e 8/4 em campo neutro, final sorteada em 9/4 e jogada em 27/5 no estádio nacional; replay até a semifinal; o vencedor de cada rodada ganha 3 mil, nada, 6 mil, 10 mil, 20 mil, 70 mil e 150 mil',
          'rule: 7 rounds: the 1st drawn on 8/11 and played on 5/12 (16 clubs), the 2nd 6/12 and 2/1 (20: 8 winners and 12 entering), the 3rd 3/1 and 29/1 (32: 10 winners and the 22 of the Premier and the First), the 4th 30/1 and 19/2, quarter-finals 20/2 and 11/3, semi-finals 12/3 and 8/4 at a neutral ground, the final drawn on 9/4 and played on 27/5 at the national stadium; replays up to the semi-finals; the winner of each round gets 3,000, nothing, 6,000, 10,000, 20,000, 70,000 and 150,000'),
         ('sco_fa_cup_collect_entrants', 'ScoFaCup::collect_entrants', '0x7edaf0', 'sco_fa_cup.cpp', 'thiscall, ret 0',
          'regra: 50 inscritos: 8 clubes escoceses de fora da liga (ordenados pela reputação, os 8 sorteados entre os 24 primeiros), a Second e a Third (embaralhados junto com eles: entram nas duas primeiras rodadas), a First (com 20 trocas sorteadas entre os seus dez: entram na 2ª e na 3ª rodadas) e a Premier (na 3ª); devolve 1 com a lista cheia, 0 se faltar clube e -1 sem memória',
          'rule: 50 entrants: 8 Scottish non-league clubs (sorted by reputation, the 8 drawn among the first 24), the Second and the Third (shuffled together with them: they enter in the first two rounds), the First (with 20 random swaps among its ten: they enter in the 2nd and 3rd rounds) and the Premier (in the 3rd); returns 1 with the list full, 0 when clubs are missing and -1 out of memory')])
CAMPOS = [('copa +0x3e quantos times, +0xa7 tabela dos times (6 bytes: clube, classe, estado); grava (inscritos)',
           'cup +0x3e team count, +0xa7 team table (6 bytes: club, class, status); writes (entrants)')]
