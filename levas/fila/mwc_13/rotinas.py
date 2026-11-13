# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01.
# create_round_table thiscall ret 0x10 (ecx = a copa; fase, &rodadas, &extra, &saida). As rodadas extraidas da
# desmontagem pelo caminho de sucesso: cada campo gravado conferido (tamanho e deslocamento) contra o
# fra_cup_write_cup_round; nenhuma gravacao a mais. Usa single_stage_round_table e compare_cup_round_table (ja no
# motor); nao depende do mwc_1. A mutacao muda a data de um jogo em 7 dias (o comparador lado a lado a pega).
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('hol_super_create_round_table', 0x5f8ad0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ger_cup_create_round_table', 0x5d8ee0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as tabelas de rodadas da Supercopa holandesa e da Copa da Alemanha',
        'the round tables of the Dutch Super Cup and the German Cup',
        [('hol_super_create_round_table', 'HolSuper::create_round_table', '0x5f8ad0', 'hol_super.cpp', 'thiscall, ret 0x10', 'regra: a Supercopa da Holanda tem 1 rodada(s): código 0: sorteio 5/7, jogo 8/8, 2 times (2 novos)', 'rule: the Dutch Super Cup has 1 round(s): code 0: draw 5 Jul, match 8 Aug, 2 teams (2 new)'),
         ('ger_cup_create_round_table', 'GerCup::create_round_table', '0x5d8ee0', 'ger_cup.cpp', 'thiscall, ret 0x10', 'regra: a Copa da Alemanha tem 6 rodada(s): código 10: sorteio 10/7, jogo 26/8, 64 times (64 novos); código 20: sorteio 27/8, jogo 11/10, 32 times (0 novos); código 30: sorteio 12/11, jogo 29/11, 16 times (0 novos); código 120: sorteio 30/11, jogo 20/12, 8 times (0 novos); código 130: sorteio 21/12, jogo 7/2 +1, 4 times (0 novos); código 150: sorteio 8/2 +1, jogo 11/5 +1, 2 times (0 novos); em 2001/02 a final é em 11/5, nas seguintes em 26/5', 'rule: the German Cup has 6 round(s): code 10: draw 10 Jul, match 26 Aug, 64 teams (64 new); code 20: draw 27 Aug, match 11 Oct, 32 teams (0 new); code 30: draw 12 Nov, match 29 Nov, 16 teams (0 new); code 120: draw 30 Nov, match 20 Dec, 8 teams (0 new); code 130: draw 21 Dec, match 7 Feb (+1 year), 4 teams (0 new); code 150: draw 8 Feb (+1 year), match 11 May (+1 year), 2 teams (0 new); in 2001/02 the final is on 11 May, in the following seasons on 26 May')])
