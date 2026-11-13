# A First Division escocesa (sco_first.cpp): o construtor, a tabela e as regras; os moldes das divisões escocesas e da
# Liga de Gales (construct_british_division, fill_british_division_table, scottish_division_rules, usados pelos itens
# mwb_6 a mwb_13: integrar este junto ou antes deles). Leitor mwb (12/11/2026), lista do mundo (Escócia e País de
# Gales).
# - 0x7edfc0 (ScoFirst::ScoFirst): disciplina 0x18; em 1999 uma fase a mais (o mata-mata do acesso).
# - 0x7eed30 (ScoFirst::create_table): 10 clubes, fundo de prêmios de 25 mil por posição.
# - 0x7eee50 (ScoFirst::set_up_rules): 4 turnos, 1 sobe, 2 descem (1999: 2 no mata-mata, ninguém desce).
# Convenções conferidas: o construtor thiscall, ret 8 (0x682200 e 0x6835c0 mov ecx -> thiscall, ret 0; 0x687b10 e
# 0x6827d0 push; mov ecx -> thiscall, ret 4; 0x944e46/0x944cf1 cdecl; 0x49ee70 push this; mov ecx -> thiscall, ret 4;
# 0x68a850 thiscall, ret 0); a tabela thiscall, ret 0 (0x687430 thiscall, ret 0x10; 0x682f70 thiscall, ret 4); as
# regras thiscall, ret 0 (a virtual +0x3c push 0; push &+0x3a; push &+0xa9; push -1 -> ret 0x10).
# Comparadores: o construtor e a tabela só a original (alocam), com o plano; as regras lado a lado nos bytes (a cópia
# sem a tabela de rodadas, ENG_PRM_RULES_DRY_RUN).
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_sco_first', 0x7edfc0, [0x6a, 0xff, 0x68, 0x59, 0x0b, 0x96, 0x00]),
           ('sco_first_create_table', 0x7eed30, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('sco_first_set_up_rules', 0x7eee50, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03])]
DOCS = ('a First Division escocesa: o construtor, a tabela e as regras',
        'the Scottish First Division: the constructor, the table and the rules',
        [('construct_sco_first', 'ScoFirst::ScoFirst', '0x7edfc0', 'sco_first.cpp', 'thiscall, ret 8',
          'regra: a First escocesa nasce com a disciplina 0x18 (a das ligas escocesas), em 1999 com uma fase a mais (o mata-mata do acesso), e monta regras, tabela, jogos, ordem, estatísticas e posições previstas',
          'rule: the Scottish First Division is created with discipline rule 0x18 (that of the Scottish leagues), in 1999 with one more stage (the promotion play-off), and builds the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('sco_first_create_table', 'ScoFirst::create_table', '0x7eed30', 'sco_first.cpp', 'thiscall, ret 0',
          'regra: a First tem 10 clubes, os da divisão no banco, na ordem do banco; fundo de prêmios de 25 mil por posição; sem dinheiro de TV',
          'rule: the First Division has 10 clubs, those of the division in the database, in database order; a prize fund of 25,000 per place; no TV money'),
         ('sco_first_set_up_rules', 'ScoFirst::set_up_rules', '0x7eee50', 'sco_first.cpp', 'thiscall, ret 0',
          'regra: 4 turnos, 3 pontos por vitória e 1 por empate; o campeão sobe para a Premier e os 2 últimos descem para a Second (em 1999: 2 vão ao mata-mata do acesso e ninguém desce); desempate saldo, gols pró e posição; 5 reservas e 3 substituições',
          'rule: 4 rounds, 3 points for a win and 1 for a draw; the champion goes up to the Premier and the bottom 2 go down to the Second (in 1999: 2 go to the promotion play-off and nobody goes down); tie-breaks goal difference, goals for and position; 5 substitutes and 3 substitutions')])
CAMPOS = [('liga +0x3e quantos clubes, +0xb1 tabela, +0x28 fundo de prêmios, +0xe1 modo do prêmio; grava (tabela)',
           'league +0x3e club count, +0xb1 table, +0x28 prize fund, +0xe1 prize mode; writes (table)'),
          ('liga +0x3c turnos, +0xc2..+0xc7 pontos e desempates, +0xbe..+0xc1 faixas, +0x1c/+0x20 divisões de cima e de baixo, +0x49/+0x4a banco, +0xba rodadas; grava (regras)',
           'league +0x3c legs, +0xc2..+0xc7 points and tie-breaks, +0xbe..+0xc1 bands, +0x1c/+0x20 upper and lower divisions, +0x49/+0x4a bench, +0xba rounds; writes (rules)')]
