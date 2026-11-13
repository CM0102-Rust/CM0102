# A Third Division escocesa (sco_third.cpp): o construtor, a tabela e as regras. Depende do mwb_5 (construct_british_division,
# fill_british_division_table, scottish_division_rules): integrar junto ou depois dele. Leitor mwb (12/11/2026), lista
# do mundo (Escócia e País de Gales).
# - 0x7f41f0 (ScoThird::ScoThird): o molde das divisões (disciplina 0x18, sem fase a mais).
# - 0x7f4e20 (ScoThird::create_table): 10 clubes, sem fundo de prêmios nem TV (devolve 1 só com a tabela cheia).
# - 0x7f4f20 (ScoThird::set_up_rules): 4 turnos, 2 sobem.
# Convenções conferidas: como as do mwb_5.
# Comparadores: o construtor e a tabela só a original (alocam), com o plano; as regras lado a lado nos bytes.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_sco_third', 0x7f41f0, [0x6a, 0xff, 0x68, 0xe9, 0x0c, 0x96, 0x00]),
           ('sco_third_create_table', 0x7f4e20, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('sco_third_set_up_rules', 0x7f4f20, [0x56, 0x8b, 0xf1, 0xb0, 0x02])]
DOCS = ('a Third Division escocesa: o construtor, a tabela e as regras',
        'the Scottish Third Division: the constructor, the table and the rules',
        [('construct_sco_third', 'ScoThird::ScoThird', '0x7f41f0', 'sco_third.cpp', 'thiscall, ret 8',
          'regra: a Third escocesa nasce com a disciplina 0x18 (a das ligas escocesas), sem fase além da liga, e monta regras, tabela, jogos, ordem, estatísticas e posições previstas',
          'rule: the Scottish Third Division is created with discipline rule 0x18 (that of the Scottish leagues), with no stage beyond the league, and builds the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('sco_third_create_table', 'ScoThird::create_table', '0x7f4e20', 'sco_third.cpp', 'thiscall, ret 0',
          'regra: a Third tem 10 clubes, os da divisão no banco, na ordem do banco; sem fundo de prêmios nem dinheiro de TV',
          'rule: the Third Division has 10 clubs, those of the division in the database, in database order; no prize fund and no TV money'),
         ('sco_third_set_up_rules', 'ScoThird::set_up_rules', '0x7f4f20', 'sco_third.cpp', 'thiscall, ret 0',
          'regra: 4 turnos, 3 pontos por vitória e 1 por empate; os 2 primeiros sobem para a Second e ninguém desce (é a última divisão); desempate saldo, gols pró e posição; 5 reservas e 3 substituições',
          'rule: 4 rounds, 3 points for a win and 1 for a draw; the top 2 go up to the Second and nobody goes down (it is the last division); tie-breaks goal difference, goals for and position; 5 substitutes and 3 substitutions')])
