# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01.
# Convencoes conferidas na desmontagem: construtor thiscall ret 8 (ecx = this, ano e registro na pilha);
# set_up_rules e create_table thiscall ret 0; create_schedule thiscall ret 0x10. Dados extraidos da desmontagem
# por caminho (as gravacoes e chamadas na ordem), conferidos contra o tamanho do malloc.
# Depende dos ajudantes do mwc_1 (construct_nation_division, compare_nation_*).
# O construtor da Superliga passa antes da carga os clubes da Kvalifikationsraekken para a divisao de baixo generica.
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_den_prm', 0x5525e0, [0x6a, 0xff, 0x68, 0x29, 0x7b, 0x95, 0x00]),
           ('den_prm_set_up_rules', 0x553a00, [0x56, 0x8b, 0xf1, 0xba, 0x03, 0x00, 0x00, 0x00]),
           ('den_prm_create_table', 0x5538d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Superliga dinamarquesa: o construtor, as regras e a tabela',
        'the Danish Superliga: the constructor, the rules and the table',
        [('construct_den_prm', 'DenPrm::DenPrm', '0x5525e0', 'den_prm.cpp', 'thiscall, ret 8', 'regra: a Superliga dinamarquesa nasce com a regra de disciplina 8, só a liga; antes da carga, os clubes da Kvalifikationsrækken passam para a divisão de baixo genérica (`KEY_CLUB_COMP_A_LOWER_DIVISION`); sem estado gravado, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas', 'rule: the Danish Superliga is born with discipline rule 8, only the league; before the load, the Kvalifikationsraekken clubs move to the generic lower division (`KEY_CLUB_COMP_A_LOWER_DIVISION`); with no saved state, the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('den_prm_set_up_rules', 'DenPrm::set_up_rules', '0x553a00', 'den_prm.cpp', 'thiscall, ret 0', 'regra: a Superliga dinamarquesa: 3 turnos, 3 pontos por vitória e 1 por empate; sobem 0, 0 vão ao play-off de acesso, 0 ao de rebaixamento e caem 2; acima: nenhuma; abaixo: a primeira dinamarquesa; desempates de códigos 1, 2, 0; 4 reservas e 3 substituições', 'rule: the Danish Superliga: 3 rounds, 3 points for a win and 1 for a draw; 0 go up, 0 to the promotion play-off, 0 to the relegation play-off and 2 go down; above: none; below: `KEY_CLUB_COMP_DANISH_FIRST_DIVISION`; tie-breaker codes 1, 2, 0 (`+0xc5`..); 4 substitutes named and 3 substitutions'),
         ('den_prm_create_table', 'DenPrm::create_table', '0x5538d0', 'den_prm.cpp', 'thiscall, ret 0', 'regra: a Superliga dinamarquesa tem 12 clubes; fundo de prêmios pela cota de 50,000 por posição; a TV paga 100,000 a cada clube', 'rule: the Danish Superliga has 12 clubs; prize fund by the share of 50,000 per place; TV pays 100,000 to each club')])
