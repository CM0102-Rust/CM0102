# A Franca: a volta a zero das contagens da janela restrita, a contagem das contratacoes e quem um clube frances pode
# contratar (leitor sx21, rodada 5, 24/11/2026). O daily_update roda todo dia para as 28 regras (o zero em 31 de
# agosto); a contagem e a regra, nas transferencias dos clubes franceses: so com a Franca carregada.
# As contagens ficam na tabela de 16 bytes por clube de [[0xde1f54] + 0x91c] (season_loans_entry, a mesma das contas de
# emprestimo da Inglaterra): byte 0 as compras, byte 1 as contratacoes livres.
# Convencoes conferidas: 0x5c1710 thiscall(data) ret 4 (o mesmo corpo do 0x40a710 da Argentina, zerando dois bytes);
# 0x5c17d0 thiscall(oferta) ret 4; 0x5c1830 thiscall(clube, jogador, vendedor, data, motivo) ret 0x14; 0x8e8e50
# thiscall(data, saida) ret 8; 0x40a7d0 thiscall(saida, ano) ret 8; 0x54c8e0 thiscall(saida, n) ret 8.
# Ajudantes comuns: reset_window_signings e compare_window_signings_reset (usados pela Argentina, sx21_22).
# Depende da sx21_1 (transfer_rules_can_sign_player, compare_rule_check): integrar junto ou depois dela.
# Comparadores: a tabela das contagens lado a lado; a entrada do clube; a regra no retorno e no motivo.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [
    ('france_rules_daily_update', 0x5c1710, [0x6a, 0xff, 0x68, 0xd8, 0x9c, 0x95, 0x00]),
    ('france_rules_record_transfer', 0x5c17d0, [0x8b, 0x54, 0x24, 0x04, 0x56]),
    ('france_rules_can_sign_player', 0x5c1830, [0x51, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x1c]),
]
DOCS = ('a França: a volta a zero das contagens da janela restrita, a contagem das contratações e quem um clube francês pode contratar',
        'France: the reset of the restricted window counts, the signing count and who a French club may sign',
        [('france_rules_daily_update', 'france_rules_daily_update', '0x5c1710', 'france_rules.cpp', 'thiscall, ret 4',
          'regra: na véspera da abertura da segunda janela (31 de agosto), as duas contagens (compras e livres) de cada clube francês voltam a zero',
          'rule: on the eve of the second window (31 August), both counts (purchases and free signings) of every French club go back to zero'),
         ('france_rules_record_transfer', 'france_rules_record_transfer', '0x5c17d0', 'france_rules.cpp', 'thiscall, ret 4',
          'regra: a contratação de um clube francês soma 1 às livres (pré-contrato) ou às compras',
          'rule: a French club signing adds 1 to the free signings (pre-contract) or to the purchases'),
         ('france_rules_can_sign_player', 'france_rules_can_sign_player', '0x5c1830', 'france_rules.cpp', 'thiscall, ret 0x14',
          'regra: fora das janelas não (9); na janela restrita (número 1), uma contratação livre (0xf) e uma compra (0x10) no máximo; senão a regra de base',
          'rule: outside the windows no (9); in the restricted window (number 1), one free signing (0xf) and one purchase (0x10) at most; else the base rule')])
CAMPOS = []
INTERESSA = []
