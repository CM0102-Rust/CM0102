# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01.
# Convencoes conferidas na desmontagem: construtor thiscall ret 8 (ecx = this, ano e registro na pilha);
# set_up_rules e create_table thiscall ret 0; create_schedule thiscall ret 0x10. Dados extraidos da desmontagem
# por caminho (as gravacoes e chamadas na ordem), conferidos contra o tamanho do malloc.
# Depende dos ajudantes do mwc_1 (construct_nation_division, compare_nation_*).
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_den_first', 0x5516f0, [0x6a, 0xff, 0x68, 0xd9, 0x7a, 0x95, 0x00]),
           ('den_first_set_up_rules', 0x5523b0, [0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00]),
           ('den_first_create_table', 0x552290, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a primeira divisão dinamarquesa: o construtor, as regras e a tabela',
        'the Danish First Division: the constructor, the rules and the table',
        [('construct_den_first', 'DenFirst::DenFirst', '0x5516f0', 'den_first.cpp', 'thiscall, ret 8', 'regra: a primeira divisão dinamarquesa nasce com a regra de disciplina 8, só a liga; sem estado gravado, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas', 'rule: the Danish First Division is born with discipline rule 8, only the league; with no saved state, the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('den_first_set_up_rules', 'DenFirst::set_up_rules', '0x5523b0', 'den_first.cpp', 'thiscall, ret 0', 'regra: a primeira divisão dinamarquesa: 2 turnos, 3 pontos por vitória e 1 por empate; sobem 2, 0 vão ao play-off de acesso, 0 ao de rebaixamento e caem 3; acima: a Superliga; abaixo: a segunda dinamarquesa; desempates de códigos 1, 2, 0; 4 reservas e 3 substituições', 'rule: the Danish First Division: 2 rounds, 3 points for a win and 1 for a draw; 2 go up, 0 to the promotion play-off, 0 to the relegation play-off and 3 go down; above: `KEY_CLUB_COMP_DANISH_PREMIER_DIVISION`; below: `KEY_CLUB_COMP_DANISH_SECOND_DIVISION`; tie-breaker codes 1, 2, 0 (`+0xc5`..); 4 substitutes named and 3 substitutions'),
         ('den_first_create_table', 'DenFirst::create_table', '0x552290', 'den_first.cpp', 'thiscall, ret 0', 'regra: a primeira divisão dinamarquesa tem 16 clubes; fundo de prêmios pela cota de 10,000 por posição; sem dinheiro de TV', 'rule: the Danish First Division has 16 clubs; prize fund by the share of 10,000 per place; no TV money')])
