# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01.
# Convencoes conferidas na desmontagem: construtor thiscall ret 8 (ecx = this, ano e registro na pilha);
# set_up_rules e create_table thiscall ret 0; create_schedule thiscall ret 0x10. Dados extraidos da desmontagem
# por caminho (as gravacoes e chamadas na ordem), conferidos contra o tamanho do malloc.
# Traz os ajudantes comuns dos construtores, das regras e das tabelas destas ligas.
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_bel_first', 0x41dd40, [0x6a, 0xff, 0x68, 0xa9, 0x3f, 0x95, 0x00]),
           ('bel_first_set_up_rules', 0x41e9e0, [0x56, 0x8b, 0xf1, 0xb9, 0x02, 0x00, 0x00, 0x00]),
           ('bel_first_create_table', 0x41e8b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a primeira divisão belga: o construtor, as regras e a tabela',
        'the Belgian First Division: the constructor, the rules and the table',
        [('construct_bel_first', 'BelFirst::BelFirst', '0x41dd40', 'bel_first.cpp', 'thiscall, ret 8', 'regra: a primeira divisão belga nasce com a regra de disciplina 4, o banco (7 reservas, 3 substituições), só a liga; sem estado gravado, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas', 'rule: the Belgian First Division is born with discipline rule 4, the bench (7 substitutes named, 3 substitutions), only the league; with no saved state, the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('bel_first_set_up_rules', 'BelFirst::set_up_rules', '0x41e9e0', 'bel_first.cpp', 'thiscall, ret 0', 'regra: a primeira divisão belga: 2 turnos, 3 pontos por vitória e 1 por empate; sobem 0, 0 vão ao play-off de acesso, 0 ao de rebaixamento e caem 2; acima: nenhuma; abaixo: a segunda belga; desempates de códigos 3, 1, 2, 0; bits de regra +0xd9 = 0x43', 'rule: the Belgian First Division: 2 rounds, 3 points for a win and 1 for a draw; 0 go up, 0 to the promotion play-off, 0 to the relegation play-off and 2 go down; above: none; below: `KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION`; tie-breaker codes 3, 1, 2, 0 (`+0xc5`..); rule bits `+0xd9` = 0x43'),
         ('bel_first_create_table', 'BelFirst::create_table', '0x41e8b0', 'bel_first.cpp', 'thiscall, ret 0', 'regra: a primeira divisão belga tem 18 clubes; fundo de prêmios pela cota de 20,000 por posição; a TV paga 50,000 a cada clube', 'rule: the Belgian First Division has 18 clubs; prize fund by the share of 20,000 per place; TV pays 50,000 to each club')])
