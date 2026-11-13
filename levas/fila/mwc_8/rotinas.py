# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01.
# Convencoes conferidas na desmontagem: construtor thiscall ret 8 (ecx = this, ano e registro na pilha);
# set_up_rules e create_table thiscall ret 0; create_schedule thiscall ret 0x10. Dados extraidos da desmontagem
# por caminho (as gravacoes e chamadas na ordem), conferidos contra o tamanho do malloc.
# Depende dos ajudantes do mwc_1 (construct_nation_division, compare_nation_*).
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_ger_second', 0x5de2e0, [0x6a, 0xff, 0x68, 0x99, 0xa6, 0x95, 0x00]),
           ('ger_second_set_up_rules', 0x5df3e0, [0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00]),
           ('ger_second_create_table', 0x5df2b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a segunda Bundesliga: o construtor, as regras e a tabela',
        'the 2. Bundesliga: the constructor, the rules and the table',
        [('construct_ger_second', 'GerSecond::GerSecond', '0x5de2e0', 'ger_second.cpp', 'thiscall, ret 8', 'regra: a segunda Bundesliga nasce com a regra de disciplina 13, só a liga; sem estado gravado, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas', 'rule: the 2. Bundesliga is born with discipline rule 13, only the league; with no saved state, the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('ger_second_set_up_rules', 'GerSecond::set_up_rules', '0x5df3e0', 'ger_second.cpp', 'thiscall, ret 0', 'regra: a segunda Bundesliga: 2 turnos, 3 pontos por vitória e 1 por empate; sobem 3, 0 vão ao play-off de acesso, 0 ao de rebaixamento e caem 4; acima: a Bundesliga; abaixo: a Regionalliga, se a divisão de baixo da Alemanha é jogável (bit 4), senão nenhuma; desempates de códigos 1, 2, 3; 6 reservas e 3 substituições', "rule: the 2. Bundesliga: 2 rounds, 3 points for a win and 1 for a draw; 3 go up, 0 to the promotion play-off, 0 to the relegation play-off and 4 go down; above: `KEY_CLUB_COMP_GERMAN_FIRST_DIVISION`; below: `KEY_CLUB_COMP_GERMAN_REGIONAL` when the German lower division is playable (bit 4 of the nation's `LEAGUE_SELECTED`), else none; tie-breaker codes 1, 2, 3 (`+0xc5`..); 6 substitutes named and 3 substitutions"),
         ('ger_second_create_table', 'GerSecond::create_table', '0x5df2b0', 'ger_second.cpp', 'thiscall, ret 0', 'regra: a segunda Bundesliga tem 18 clubes; fundo de prêmios pela cota de 50,000 por posição; a TV paga 1,300,000 a cada clube', 'rule: the 2. Bundesliga has 18 clubs; prize fund by the share of 50,000 per place; TV pays 1,300,000 to each club')])
