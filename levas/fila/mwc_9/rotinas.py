# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01.
# Convencoes conferidas na desmontagem: construtor thiscall ret 8 (ecx = this, ano e registro na pilha);
# set_up_rules e create_table thiscall ret 0; create_schedule thiscall ret 0x10. Dados extraidos da desmontagem
# por caminho (as gravacoes e chamadas na ordem), conferidos contra o tamanho do malloc.
# Depende dos ajudantes do mwc_1 (construct_nation_division, compare_nation_*).
# As chaves holandesas trocadas pelo nome: a Eredivisie usa a chave KPN_TELECOMPETITION e a Eerste Divisie a TOTO_DIVISION.
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_hol_prm', 0x5f7610, [0x6a, 0xff, 0x68, 0xe9, 0xad, 0x95, 0x00]),
           ('hol_prm_set_up_rules', 0x5f8560, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03]),
           ('hol_prm_create_table', 0x5f8430, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Eredivisie: o construtor, as regras e a tabela',
        'the Eredivisie: the constructor, the rules and the table',
        [('construct_hol_prm', 'HolPrm::HolPrm', '0x5f7610', 'hol_prm.cpp', 'thiscall, ret 8', 'regra: a primeira divisão holandesa (Eredivisie) nasce com a regra de disciplina 15, só a liga; sem estado gravado, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas', 'rule: the Dutch Eredivisie is born with discipline rule 15, only the league; with no saved state, the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('hol_prm_set_up_rules', 'HolPrm::set_up_rules', '0x5f8560', 'hol_prm.cpp', 'thiscall, ret 0', 'regra: a primeira divisão holandesa (Eredivisie): 2 turnos, 3 pontos por vitória e 1 por empate; sobem 0, 0 vão ao play-off de acesso, 2 ao de rebaixamento e caem 1; acima: nenhuma; abaixo: a Eerste Divisie (chave Toto); desempates de códigos 1, 2, 0, 0; 7 reservas e 3 substituições', 'rule: the Dutch Eredivisie: 2 rounds, 3 points for a win and 1 for a draw; 0 go up, 0 to the promotion play-off, 2 to the relegation play-off and 1 go down; above: none; below: `KEY_CLUB_COMP_DUTCH_TOTO_DIVISION`; tie-breaker codes 1, 2, 0, 0 (`+0xc5`..); 7 substitutes named and 3 substitutions'),
         ('hol_prm_create_table', 'HolPrm::create_table', '0x5f8430', 'hol_prm.cpp', 'thiscall, ret 0', 'regra: a primeira divisão holandesa (Eredivisie) tem 18 clubes; fundo de prêmios pela cota de 50,000 por posição; a TV paga 1,000,000 a cada clube', 'rule: the Dutch Eredivisie has 18 clubs; prize fund by the share of 50,000 per place; TV pays 1,000,000 to each club')])
