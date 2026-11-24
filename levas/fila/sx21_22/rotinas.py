# A Argentina: a volta a zero da contagem da segunda janela, a contagem das contratacoes e quem um clube argentino pode
# contratar (leitor sx21, rodada 5, 24/11/2026). O daily_update roda todo dia para as 28 regras (o zero em 13 de
# dezembro); a contagem e a regra, nas transferencias dos clubes argentinos: so com a Argentina carregada.
# A contagem fica no byte 0 da entrada de 16 bytes por clube de [[0xde1f54] + 0x91c] (season_loans_entry).
# Convencoes conferidas: 0x40a710 thiscall(data) ret 4; 0x40a810 thiscall(oferta) ret 4 (o clube comprador sem conferir
# a nacao nula); 0x40a860 thiscall(clube, jogador, vendedor, data, motivo) ret 0x14; 0x8e8e50 thiscall(data, saida)
# ret 8.
# Depende da sx21_20 (reset_window_signings, compare_window_signings_reset) e da sx21_1 (transfer_rules_can_sign_player,
# compare_rule_check): integrar junto ou depois delas.
# Comparadores: a tabela das contagens lado a lado; o byte do clube; a regra no retorno e no motivo.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ROTINAS = [
    ('argentina_rules_daily_update', 0x40a710, [0x6a, 0xff, 0x68, 0x28, 0x37, 0x95, 0x00]),
    ('argentina_rules_record_transfer', 0x40a810, [0x8b, 0x44, 0x24, 0x04, 0x56]),
    ('argentina_rules_can_sign_player', 0x40a860, [0x51, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x1c]),
]
DOCS = ('a Argentina: a volta a zero da contagem da segunda janela, a contagem das contratações e quem um clube argentino pode contratar',
        'Argentina: the reset of the second window count, the signing count and who an Argentine club may sign',
        [('argentina_rules_daily_update', 'argentina_rules_daily_update', '0x40a710', 'argentina_rules.cpp', 'thiscall, ret 4',
          'regra: na véspera da abertura da segunda janela (13 de dezembro), a contagem de contratações de cada clube argentino volta a zero',
          'rule: on the eve of the second window (13 December), the signing count of every Argentine club goes back to zero'),
         ('argentina_rules_record_transfer', 'argentina_rules_record_transfer', '0x40a810', 'argentina_rules.cpp', 'thiscall, ret 4',
          'regra: a contratação de um clube argentino soma 1 à contagem',
          'rule: an Argentine club signing adds 1 to the count'),
         ('argentina_rules_can_sign_player', 'argentina_rules_can_sign_player', '0x40a860', 'argentina_rules.cpp', 'thiscall, ret 0x14',
          'regra: fora das janelas não (9); na segunda janela, duas contratações no máximo (0xe); senão a regra de base',
          'rule: outside the windows no (9); in the second window, two signings at most (0xe); else the base rule')])
CAMPOS = []
INTERESSA = []
