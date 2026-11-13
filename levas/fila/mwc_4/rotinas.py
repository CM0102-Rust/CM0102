# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01.
# Convencoes conferidas na desmontagem: construtor thiscall ret 8 (ecx = this, ano e registro na pilha);
# set_up_rules e create_table thiscall ret 0; create_schedule thiscall ret 0x10. Dados extraidos da desmontagem
# por caminho (as gravacoes e chamadas na ordem), conferidos contra o tamanho do malloc.
# Depende dos ajudantes do mwc_1 (construct_nation_division, compare_nation_*).
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_bel_second', 0x41f030, [0x6a, 0xff, 0x68, 0xf9, 0x3f, 0x95, 0x00]),
           ('bel_second_set_up_rules', 0x41fea0, [0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x03]),
           ('bel_second_create_table', 0x41fd70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a segunda divisão belga: o construtor, as regras e a tabela',
        'the Belgian Second Division: the constructor, the rules and the table',
        [('construct_bel_second', 'BelSecond::BelSecond', '0x41f030', 'bel_second.cpp', 'thiscall, ret 8', 'regra: a segunda divisão belga nasce com a regra de disciplina 4, o banco (4 reservas, 3 substituições), 4 fases além da liga; sem estado gravado, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas', 'rule: the Belgian Second Division is born with discipline rule 4, the bench (4 substitutes named, 3 substitutions), 4 stages besides the league (the stage vector, malloc of 16); with no saved state, the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('bel_second_set_up_rules', 'BelSecond::set_up_rules', '0x41fea0', 'bel_second.cpp', 'thiscall, ret 0', 'regra: a segunda divisão belga: 2 turnos, 3 pontos por vitória e 1 por empate; sobem 1, 0 vão ao play-off de acesso, 1 ao de rebaixamento e caem 2; acima: a primeira belga; abaixo: a terceira belga; desempates de códigos 3, 1, 2, 0; zera os campos próprios da classe +0xf6, +0xf7, +0xf8 [inferência: os vencedores dos períodos]; bits de regra +0xd9 = 0x43', 'rule: the Belgian Second Division: 2 rounds, 3 points for a win and 1 for a draw; 1 go up, 0 to the promotion play-off, 1 to the relegation play-off and 2 go down; above: `KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION`; below: `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`; tie-breaker codes 3, 1, 2, 0 (`+0xc5`..); zeroes the class fields `+0xf6`, `+0xf7`, `+0xf8` [inference: the period winners]; rule bits `+0xd9` = 0x43'),
         ('bel_second_create_table', 'BelSecond::create_table', '0x41fd70', 'bel_second.cpp', 'thiscall, ret 0', 'regra: a segunda divisão belga tem 18 clubes; fundo de prêmios pela cota de 10,000 por posição; a TV paga 40,000 a cada clube (antes do fundo)', 'rule: the Belgian Second Division has 18 clubs; prize fund by the share of 10,000 per place; TV pays 40,000 to each club (before the fund)')])
