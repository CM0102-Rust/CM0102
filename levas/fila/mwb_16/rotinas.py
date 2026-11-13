# A Copa da Liga escocesa (sco_lge_cup.cpp): o construtor, a tabela das rodadas e a atualização do dia. Leitor mwb
# (12/11/2026), lista do mundo (Escócia e País de Gales). Só usa o que já está no motor (free_season_tables,
# call_cup_virtual, eng_cc_cup_create_single_stage_round_table, compare_cup_round_table).
# - 0x7ef9e0 (ScoLgeCup::ScoLgeCup): o molde das copas escocesas sem os inscritos nem as posições previstas (vêm na
#   atualização do dia); 40 vagas vazias (calloc), fundo de 1 milhão.
# - 0x7efcb0 (ScoLgeCup::create_round_table): 6 rodadas de julho a março; os clubes nas copas europeias (+0xb2) só
#   entram na 3ª (as contas do molde da Copa da Liga inglesa, com outros números; os de 1999 à parte).
# - 0x7f0230 (ScoLgeCup::update_for_date): roda todo dia (2308 vezes); refaz a copa uma vez por temporada, no primeiro
#   dia com resultados em que o ano do jogo chega ao da copa (em 2001 logo no começo da carreira; em 2002 depois da
#   virada).
# Convenções conferidas: o construtor thiscall, ret 8 (como os do mwb_1; 0x945501 push 6; push 0x28 -> cdecl); a tabela
# thiscall, ret 0x10; a do dia thiscall, ret 8 (0x7f0400 mov ecx -> thiscall, ret 0; as virtuais +0x8c, +0x94, +0x5c
# thiscall, ret 0; 0x51f890 com os mesmos dois argumentos, ret 8).
# Comparadores: o construtor só a original (aloca), com o plano; a tabela lado a lado; a do dia só a original (refaz,
# sorteia e joga), com o plano do +0x45.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_sco_lge_cup', 0x7ef9e0, [0x6a, 0xff, 0x68, 0xc9, 0x0b, 0x96, 0x00]),
           ('sco_lge_cup_create_round_table', 0x7efcb0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('sco_lge_cup_update_for_date', 0x7f0230, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da Liga escocesa: o construtor, as rodadas e o dia',
        'the Scottish League Cup: the constructor, the rounds and the day update',
        [('construct_sco_lge_cup', 'ScoLgeCup::ScoLgeCup', '0x7ef9e0', 'sco_lge_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Liga escocesa nasce com 40 vagas vazias, 5 reservas e 3 substituições, a disciplina das copas escocesas (0x19) e um fundo de prêmios de 1 milhão; os inscritos e as posições previstas só vêm quando a temporada começa',
          'rule: the Scottish League Cup is created with 40 empty places, 5 substitutes and 3 substitutions, the discipline rule of the Scottish cups (0x19) and a prize fund of 1,000,000; the entrants and the predicted positions only come when the season starts'),
         ('sco_lge_cup_create_round_table', 'ScoLgeCup::create_round_table', '0x7efcb0', 'sco_lge_cup.cpp', 'thiscall, ret 0x10',
          'regra: 6 rodadas de jogo único: 1ª sorteada em 10/7 e jogada em 31/7, 2ª 1/8 e 17/8, 3ª 19/8 e 22/9, quartas 23/9 e 1/12, semifinal 2/12 e 16/2 em campo neutro, final sorteada em 17/2 e jogada em 19/3 no estádio nacional; os clubes nas copas europeias só entram na 3ª (de 2000 em diante: 2b + 20 na 1ª rodada, (16 - b) x 2 na 2ª); categoria 4 nas três primeiras, 6 depois; sem prêmio por rodada',
          'rule: 6 single-match rounds: the 1st drawn on 10/7 and played on 31/7, the 2nd 1/8 and 17/8, the 3rd 19/8 and 22/9, quarter-finals 23/9 and 1/12, semi-finals 2/12 and 16/2 at a neutral ground, the final drawn on 17/2 and played on 19/3 at the national stadium; the clubs in the European cups only enter in the 3rd (from 2000: 2b + 20 in the 1st round, (16 - b) x 2 in the 2nd); category 4 in the first three, 6 after; no prize per round'),
         ('sco_lge_cup_update_for_date', 'ScoLgeCup::update_for_date', '0x7f0230', 'sco_lge_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Liga escocesa é refeita uma vez por temporada, no primeiro dia com resultados em que o ano do jogo chega ao da copa: libera as tabelas velhas e monta os inscritos, as rodadas, a árvore de confrontos e as posições previstas; depois, todo dia, a atualização da copa (sorteios e jogos nas datas da tabela)',
          'rule: the Scottish League Cup is rebuilt once per season, on the first day with results when the game year reaches the cup year: the old tables are freed and the entrants, the rounds, the fixture tree and the predicted positions are built; then, every day, the cup update (draws and matches on the table dates)')])
