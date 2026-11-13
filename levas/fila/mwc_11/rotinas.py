# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01.
# Convencoes conferidas na desmontagem: construtor thiscall ret 8 (ecx = this, ano e registro na pilha);
# set_up_rules e create_table thiscall ret 0; create_schedule thiscall ret 0x10. Dados extraidos da desmontagem
# por caminho (as gravacoes e chamadas na ordem), conferidos contra o tamanho do malloc.
# Depende dos ajudantes do mwc_1 (construct_nation_division, compare_nation_*).
# O calendario da Bundesliga tem uma tabela para 2001/02 e outra para as temporadas seguintes.
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_ger_first', 0x5d99b0, [0x6a, 0xff, 0x68, 0x89, 0xa5, 0x95, 0x00]),
           ('ger_first_set_up_rules', 0x5db400, [0x56, 0x8b, 0xf1, 0xba, 0x02, 0x00, 0x00, 0x00]),
           ('ger_first_create_schedule', 0x5d9c70, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Bundesliga: o construtor, as regras e o calendário',
        'the Bundesliga: the constructor, the rules and the schedule',
        [('construct_ger_first', 'GerFirst::GerFirst', '0x5d99b0', 'ger_first.cpp', 'thiscall, ret 8', 'regra: a Bundesliga nasce com a regra de disciplina 13, só a liga; sem estado gravado, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas', 'rule: the Bundesliga is born with discipline rule 13, only the league; with no saved state, the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('ger_first_set_up_rules', 'GerFirst::set_up_rules', '0x5db400', 'ger_first.cpp', 'thiscall, ret 0', 'regra: a Bundesliga: 2 turnos, 3 pontos por vitória e 1 por empate; sobem 0, 0 vão ao play-off de acesso, 0 ao de rebaixamento e caem 3; acima: nenhuma; abaixo: a segunda Bundesliga; desempates de códigos 1, 2, 3; 6 reservas e 3 substituições', 'rule: the Bundesliga: 2 rounds, 3 points for a win and 1 for a draw; 0 go up, 0 to the promotion play-off, 0 to the relegation play-off and 3 go down; above: none; below: `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION`; tie-breaker codes 1, 2, 3 (`+0xc5`..); 6 substitutes named and 3 substitutions'),
         ('ger_first_create_schedule', 'GerFirst::create_schedule', '0x5d9c70', 'ger_first.cpp', 'thiscall, ret 0x10', 'regra: o calendário de a Bundesliga: 34 rodadas, de 29/7 a 5/5; 30 com jogos em faixas de TV; em 2001/02 a sua tabela, nas seguintes outra', 'rule: the Bundesliga schedule: 34 rounds, from 29 Jul to 5 May; 30 with matches in TV slots; in 2001/02 its own table, in the following seasons another')])
