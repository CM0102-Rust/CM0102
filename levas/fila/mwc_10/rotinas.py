# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01.
# Convencoes conferidas na desmontagem: construtor thiscall ret 8 (ecx = this, ano e registro na pilha);
# set_up_rules e create_table thiscall ret 0; create_schedule thiscall ret 0x10. Dados extraidos da desmontagem
# por caminho (as gravacoes e chamadas na ordem), conferidos contra o tamanho do malloc.
# Depende dos ajudantes do mwc_1 (construct_nation_division, compare_nation_*).
# A tabela da Eerste Divisie nao tem fundo nem TV: a mutacao troca as linhas 0 e 1 (so o comparador a pega).
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_hol_first', 0x5f5490, [0x6a, 0xff, 0x68, 0x29, 0xad, 0x95, 0x00]),
           ('hol_first_set_up_rules', 0x5f6340, [0x53, 0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('hol_first_create_table', 0x5f6240, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Eerste Divisie: o construtor, as regras e a tabela',
        'the Eerste Divisie: the constructor, the rules and the table',
        [('construct_hol_first', 'HolFirst::HolFirst', '0x5f5490', 'hol_first.cpp', 'thiscall, ret 8', 'regra: a segunda divisão holandesa (Eerste Divisie) nasce com a regra de disciplina 15, 6 fases além da liga; sem estado gravado, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas', 'rule: the Dutch Eerste Divisie is born with discipline rule 15, 6 stages besides the league (the stage vector, malloc of 24); with no saved state, the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('hol_first_set_up_rules', 'HolFirst::set_up_rules', '0x5f6340', 'hol_first.cpp', 'thiscall, ret 0', 'regra: a segunda divisão holandesa (Eerste Divisie): 2 turnos, 3 pontos por vitória e 1 por empate; sobem 1, 0 vão ao play-off de acesso, 0 ao de rebaixamento e caem 0; acima: a Eredivisie (chave KPN); abaixo: nenhuma; desempates de códigos 1, 2, 0; 7 reservas e 3 substituições; zera os campos próprios da classe +0xee, +0xef, +0xf0, +0xf1 [inferência: os vencedores dos períodos]', 'rule: the Dutch Eerste Divisie: 2 rounds, 3 points for a win and 1 for a draw; 1 go up, 0 to the promotion play-off, 0 to the relegation play-off and 0 go down; above: `KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`; below: none; tie-breaker codes 1, 2, 0 (`+0xc5`..); 7 substitutes named and 3 substitutions; zeroes the class fields `+0xee`, `+0xef`, `+0xf0`, `+0xf1` [inference: the period winners]'),
         ('hol_first_create_table', 'HolFirst::create_table', '0x5f6240', 'hol_first.cpp', 'thiscall, ret 0', 'regra: a segunda divisão holandesa (Eerste Divisie) tem 18 clubes, sem fundo de prêmios nem dinheiro de TV', 'rule: the Dutch Eerste Divisie has 18 clubs, with no prize fund and no TV money')])
