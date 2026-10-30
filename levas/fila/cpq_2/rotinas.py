# A federação e a diretoria (manager_manager.cpp): avaliar a seleção na competição (MANAGER_MANAGER::
# review_national_team_performance 0x6a5610, pela NationalTeams::clear_competition_for_comp_clubs 0x775420 no fim da
# competição de cada seleção), o clube em apuro financeiro (is_club_in_financial_trouble 0x69eb50, pelo motor de lances:
# os cantos da torcida, 0x6c6430 e na cópia Rust da narração) e a faixa da confiança da diretoria (get_board_confidence_band
# 0x690b70, pelo texto do relatório 0x690d80). Sem SEH, sem leitura de lixo; a 0x6a5610 tem a tabela de saltos em
# 0x6a5a3c (notas −3..3). Iguais no cm0102-1080.exe. Comparadores: a 0x6a5610 só a original (notícia, sorteio, saída
# alocada); as outras duas pelo valor.
SAVE = False
ROTINAS = [('avaliar_a_selecao_na_competicao', 0x6a5610, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('clube_em_apuro_financeiro', 0x69eb50, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('faixa_da_confianca_da_diretoria', 0x690b70, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x09])]
DOCS = ('a federação e a diretoria: avaliar a seleção na competição, o clube em apuro financeiro e a faixa da confiança da diretoria',
        'the FA and the board: reviewing the national team in a competition, the club in financial trouble and the board confidence band',
        [('avaliar_a_selecao_na_competicao', 'MANAGER_MANAGER::review_national_team_performance', '0x6a5610', 'manager_manager.cpp', 'thiscall, ret 8',
          'a confiança da federação pela nota do técnico na competição (−3..3, virtual +0x74), a notícia da avaliação e, conforme o tempo no cargo, a reputação e sorteios, a saída do técnico ou a federação pensando em demiti-lo (0x69aba0)',
          'the FA confidence from the manager grade in the competition (−3..3, virtual +0x74), the assessment story and, by time in the job, reputation and draws, the manager departure or the FA considering the sacking (0x69aba0)'),
         ('clube_em_apuro_financeiro', 'MANAGER_MANAGER::is_club_in_financial_trouble', '0x69eb50', 'manager_manager.cpp', 'thiscall, ret 4',
          'clube dirigível, fora da seleção, com a confiança da diretoria em +8 da linha abaixo de 3000 e saldo negativo',
          'manageable club, not a national team, with board confidence at row +8 under 3000 and a negative balance'),
         ('faixa_da_confianca_da_diretoria', 'MANAGER_MANAGER::get_board_confidence_band', '0x690b70', 'manager_manager.cpp', 'thiscall, ret 4',
          'a faixa 0..6 da confiança da diretoria (limites 3000, 3500, 4275, 6500, 8500, 9500)',
          'the 0..6 band of the board confidence (limits 3000, 3500, 4275, 6500, 8500, 9500)')])
CAMPOS = [('linha da diretoria [this] + id×73: +6 confiança (da federação na seleção), +8 confiança financeira, +0xe; valores; lê e grava +6 (avaliar_a_selecao_na_competicao), lê +6 (faixa_da_confianca_da_diretoria) e +8 (clube_em_apuro_financeiro)',
           'board row [this] + id×73: +6 confidence (the FA one for national teams), +8 financial confidence, +0xe; values; reads and writes +6 (avaliar_a_selecao_na_competicao), reads +6 (faixa_da_confianca_da_diretoria) and +8 (clube_em_apuro_financeiro)'),
          ('MANAGER_MANAGER +4: a lista das saídas (6 bytes: 1, motivo 6 ou 0, id); ponteiro; acrescenta (avaliar_a_selecao_na_competicao)',
           'MANAGER_MANAGER +4: the departure list (6 bytes: 1, reason 6 or 0, id); pointer; appends (avaliar_a_selecao_na_competicao)'),
          ('clube +0x80: reputação; valor; lê (avaliar_a_selecao_na_competicao)',
           'club +0x80: reputation; value; reads (avaliar_a_selecao_na_competicao)'),
          ('registro de finanças (0x5a0590 de [0xae2a38]) +0: saldo em 64 bits; valor; lê (clube_em_apuro_financeiro)',
           'finance record (0x5a0590 of [0xae2a38]) +0: 64-bit balance; value; reads (clube_em_apuro_financeiro)')]
INTERESSA = ['CMRTE']
