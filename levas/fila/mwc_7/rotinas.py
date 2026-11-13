# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01.
# Convencoes conferidas na desmontagem: construtor thiscall ret 8 (ecx = this, ano e registro na pilha);
# set_up_rules e create_table thiscall ret 0; create_schedule thiscall ret 0x10. Dados extraidos da desmontagem
# por caminho (as gravacoes e chamadas na ordem), conferidos contra o tamanho do malloc.
# Depende dos ajudantes do mwc_1 (construct_nation_division, compare_nation_*).
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_den_second', 0x553e80, [0x6a, 0xff, 0x68, 0x79, 0x7b, 0x95, 0x00]),
           ('den_second_set_up_rules', 0x554b60, [0x56, 0x8b, 0xf1, 0xba, 0x02, 0x00, 0x00, 0x00]),
           ('den_second_create_table', 0x554a40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a segunda divisão dinamarquesa: o construtor, as regras e a tabela',
        'the Danish Second Division: the constructor, the rules and the table',
        [('construct_den_second', 'DenSecond::DenSecond', '0x553e80', 'den_second.cpp', 'thiscall, ret 8', 'regra: a segunda divisão dinamarquesa nasce com a regra de disciplina 8, só a liga; sem estado gravado, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas', 'rule: the Danish Second Division is born with discipline rule 8, only the league; with no saved state, the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('den_second_set_up_rules', 'DenSecond::set_up_rules', '0x554b60', 'den_second.cpp', 'thiscall, ret 0', 'regra: a segunda divisão dinamarquesa: 2 turnos, 3 pontos por vitória e 1 por empate; sobem 3, 0 vão ao play-off de acesso, 0 ao de rebaixamento e caem 4; acima: a primeira dinamarquesa; abaixo: nenhuma; desempates de códigos 1, 2, 0; 4 reservas e 3 substituições', 'rule: the Danish Second Division: 2 rounds, 3 points for a win and 1 for a draw; 3 go up, 0 to the promotion play-off, 0 to the relegation play-off and 4 go down; above: `KEY_CLUB_COMP_DANISH_FIRST_DIVISION`; below: none; tie-breaker codes 1, 2, 0 (`+0xc5`..); 4 substitutes named and 3 substitutions'),
         ('den_second_create_table', 'DenSecond::create_table', '0x554a40', 'den_second.cpp', 'thiscall, ret 0', 'regra: a segunda divisão dinamarquesa tem 16 clubes; fundo de prêmios pela cota de 5,000 por posição; sem dinheiro de TV', 'rule: the Danish Second Division has 16 clubs; prize fund by the share of 5,000 per place; no TV money')])
