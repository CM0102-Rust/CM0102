# A Second Division escocesa (sco_second.cpp): o construtor, a tabela e as regras. Depende do mwb_5 (construct_british_division,
# fill_british_division_table, scottish_division_rules): integrar junto ou depois dele. Leitor mwb (12/11/2026), lista
# do mundo (Escócia e País de Gales).
# - 0x7f3250 (ScoSecond::ScoSecond): o molde das divisões (disciplina 0x18, sem fase a mais).
# - 0x7f3e80 (ScoSecond::create_table): 10 clubes, sem fundo de prêmios nem TV (devolve 1 só com a tabela cheia).
# - 0x7f3f80 (ScoSecond::set_up_rules): 4 turnos, 2 sobem.
# Convenções conferidas: como as do mwb_5.
# Comparadores: o construtor e a tabela só a original (alocam), com o plano; as regras lado a lado nos bytes.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_sco_second', 0x7f3250, [0x6a, 0xff, 0x68, 0x99, 0x0c, 0x96, 0x00]),
           ('sco_second_create_table', 0x7f3e80, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('sco_second_set_up_rules', 0x7f3f80, [0x56, 0x8b, 0xf1, 0xb2, 0x03])]
DOCS = ('a Second Division escocesa: o construtor, a tabela e as regras',
        'the Scottish Second Division: the constructor, the table and the rules',
        [('construct_sco_second', 'ScoSecond::ScoSecond', '0x7f3250', 'sco_second.cpp', 'thiscall, ret 8',
          'regra: a Second escocesa nasce com a disciplina 0x18 (a das ligas escocesas), sem fase além da liga, e monta regras, tabela, jogos, ordem, estatísticas e posições previstas',
          'rule: the Scottish Second Division is created with discipline rule 0x18 (that of the Scottish leagues), with no stage beyond the league, and builds the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('sco_second_create_table', 'ScoSecond::create_table', '0x7f3e80', 'sco_second.cpp', 'thiscall, ret 0',
          'regra: a Second tem 10 clubes, os da divisão no banco, na ordem do banco; sem fundo de prêmios nem dinheiro de TV',
          'rule: the Second Division has 10 clubs, those of the division in the database, in database order; no prize fund and no TV money'),
         ('sco_second_set_up_rules', 'ScoSecond::set_up_rules', '0x7f3f80', 'sco_second.cpp', 'thiscall, ret 0',
          'regra: 4 turnos, 3 pontos por vitória e 1 por empate; os 2 primeiros sobem para a First e os 2 últimos descem para a Third (em 1999 ninguém descia); desempate saldo, gols pró e posição; 5 reservas e 3 substituições',
          'rule: 4 rounds, 3 points for a win and 1 for a draw; the top 2 go up to the First and the bottom 2 go down to the Third (in 1999 nobody went down); tie-breaks goal difference, goals for and position; 5 substitutes and 3 substitutions')])
