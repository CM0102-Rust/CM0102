# A Premier Division escocesa (sco_prm.cpp): o construtor, a tabela e as regras. Depende do mwb_5 (construct_british_division,
# fill_british_division_table): integrar junto ou depois dele. Leitor mwb (12/11/2026), lista do mundo (Escócia e País
# de Gales).
# - 0x7f0b80 (ScoPrm::ScoPrm): o molde das divisões, com estádio de 10 mil lugares sentados e, no save carregado de uma
#   temporada em 4 turnos fora de 1999, a comparação da divisão (split) de volta (+0xa3 = 0x7f3220).
# - 0x7f2820 (ScoPrm::create_table): 12 clubes (10 em 1999), fundo de prêmios de 50 mil por posição e 500 mil de TV a cada
#   clube.
# - 0x7f2970 (ScoPrm::set_up_rules): 3 turnos, o último desce; depois do calendário, o número de rodadas vira 33 (as de
#   antes da divisão).
# Convenções conferidas: como as do mwb_5 (0x683010 push 0; push 0x7a120; mov ecx -> thiscall, ret 8).
# Comparadores: o construtor e a tabela só a original (alocam e pagam), com o plano; as regras lado a lado nos bytes
# (com o +0xa9 = 33 fora de 1999).
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_sco_prm', 0x7f0b80, [0x6a, 0xff, 0x68, 0x19, 0x0c, 0x96, 0x00]),
           ('sco_prm_create_table', 0x7f2820, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('sco_prm_set_up_rules', 0x7f2970, [0x53, 0x56, 0x8b, 0xf1, 0x57])]
DOCS = ('a Premier Division escocesa: o construtor, a tabela e as regras',
        'the Scottish Premier Division: the constructor, the table and the rules',
        [('construct_sco_prm', 'ScoPrm::ScoPrm', '0x7f0b80', 'sco_prm.cpp', 'thiscall, ret 8',
          'regra: a Premier escocesa nasce exigindo estádio de 10 mil lugares sentados, com a disciplina 0x18, e monta regras, tabela, jogos, ordem, estatísticas e posições previstas',
          'rule: the Scottish Premier Division is created requiring a stadium of 10,000 seated places, with discipline rule 0x18, and builds the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('sco_prm_create_table', 'ScoPrm::create_table', '0x7f2820', 'sco_prm.cpp', 'thiscall, ret 0',
          'regra: a Premier tem 12 clubes (10 em 1999), os da divisão no banco, na ordem do banco; fundo de prêmios de 50 mil por posição e 500 mil de TV para cada clube',
          'rule: the Premier Division has 12 clubs (10 in 1999), those of the division in the database, in database order; a prize fund of 50,000 per place and 500,000 of TV money to every club'),
         ('sco_prm_set_up_rules', 'ScoPrm::set_up_rules', '0x7f2970', 'sco_prm.cpp', 'thiscall, ret 0',
          'regra: 3 turnos (33 jogos de cada clube) e depois a divisão em dois grupos de seis; 3 pontos por vitória e 1 por empate; o último desce para a First (em 1999: 10 clubes, 4 turnos, o último no mata-mata do rebaixamento); desempate saldo, gols pró e posição; 5 reservas e 3 substituições',
          'rule: 3 rounds (33 matches per club) and then the split into two groups of six; 3 points for a win and 1 for a draw; the bottom club goes down to the First (in 1999: 10 clubs, 4 rounds, the bottom club to a relegation play-off); tie-breaks goal difference, goals for and position; 5 substitutes and 3 substitutions')])
