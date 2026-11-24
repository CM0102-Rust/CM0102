# As rodadas da Copa da Dinamarca e da Copa da Liga alema e o mando na Copa da Alemanha (den_cup.cpp, ger_lge_cup.cpp,
# ger_cup.cpp; leitor cx02, rodada 5, 24/11/2026). Competicoes com a Dinamarca e a Alemanha; ATE 2002-08-01 (as
# set_up_rounds rodam na virada das copas, no meio de 2002).
# - 0x5516b0 DenCup::set_up_rounds (a virtual 0x8c da DenCup): premio 800.000, a tabela e depois +0x49 = 4 e +0x4a = 3.
# - 0x5dcb90 GerLgeCup::set_up_rounds: +0x49 = 6 e +0x4a = 3 antes, premio 500.000 e a tabela.
#   As duas usam set_up_prize_rounds do cx02_4 (integrar depois dele). Prologo push esi; mov esi, ecx; push 0.
# - 0x5d9930 GerCup::club_in_higher_division thiscall(a, b) ret 8 -> clube (a virtual 0x90 da Cup): os registros das
#   chaves 0x9cf54c (Primeira) e 0x9cf550 (Segunda) comparados como ponteiros com a divisao (+0x57) de cada clube.
#   Prologo mov eax, [0x9cf54c] (5 bytes, endereco absoluto).
# Iguais no cm0102-1080.exe.
# Comparadores: as rodadas lado a lado (cx02_4, com o banco); o clube devolvido (compare_value).
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('den_cup_set_up_rounds', 0x5516b0, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('ger_lge_cup_set_up_rounds', 0x5dcb90, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('ger_cup_club_in_higher_division', 0x5d9930, [0xa1, 0x4c, 0xf5, 0x9c, 0x00])]
DOCS = ('as rodadas da Copa da Dinamarca e da Copa da Liga alemã e o mando na Copa da Alemanha',
        'the Danish Cup and German League Cup rounds and the German Cup home advantage',
        [('den_cup_set_up_rounds', 'DenCup::set_up_rounds', '0x5516b0', 'den_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Dinamarca tem um prêmio total de 800.000, 4 reservas no banco e 3 substituições; monta a tabela de rodadas',
          'rule: the Danish Cup has an 800,000 prize fund, 4 substitutes named and 3 substitutions; builds the round table'),
         ('ger_lge_cup_set_up_rounds', 'GerLgeCup::set_up_rounds', '0x5dcb90', 'ger_lge_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Liga alemã tem 6 reservas no banco, 3 substituições e um prêmio total de 500.000; monta a tabela de rodadas',
          'rule: the German League Cup has 6 substitutes named, 3 substitutions and a 500,000 prize fund; builds the round table'),
         ('ger_cup_club_in_higher_division', 'GerCup::club_in_higher_division', '0x5d9930', 'ger_cup.cpp', 'thiscall, ret 8',
          'regra: na Copa da Alemanha joga em casa o clube de fora das duas divisões da Bundesliga quando o adversário é de uma delas; os dois de dentro ou os dois de fora: ninguém',
          'rule: in the German Cup the club from outside the two Bundesliga divisions plays at home when its opponent plays in one of them; both inside or both outside: nobody')])
CAMPOS = []
INTERESSA = []
