# A oferta na Franca e a contratacao e a oferta na Italia (leitor sx21, rodada 5, 24/11/2026): rodam nas transferencias
# dos clubes franceses e italianos, so com essas ligas carregadas.
# Convencoes conferidas: 0x5c1930 e 0x661630 thiscall(oferta, clube, data, motivo) ret 0x10; 0x661520 thiscall(clube,
# jogador, vendedor, data, motivo) ret 0x14; 0x8e8e50 thiscall(data, saida) ret 8; 0x8e8c40 thiscall(data, janela)
# ret 8; 0x4cf5a0 thiscall(pessoa) ret 4 (sem guardar em 0xae1080).
# Ajudante comum: date_in_window (usado pelos sx21 seguintes).
# Depende da sx21_1 (transfer_rules_can_sign_player, transfer_rules_can_make_offer, compare_rule_check): integrar junto
# ou depois dela.
# Comparadores: a regra no retorno, no motivo e em G_CONTRACT_LOOKUP_RESULT, lado a lado.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [
    ('france_rules_can_make_offer', 0x5c1930, [0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14]),
    ('italy_rules_can_sign_player', 0x661520, [0x51, 0x53, 0x55, 0x56, 0x57]),
    ('italy_rules_can_make_offer', 0x661630, [0x53, 0x55, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14]),
]
DOCS = ('a oferta na França e a contratação e a oferta na Itália',
        'the French offer check and the Italian signing and offer checks',
        [('france_rules_can_make_offer', 'france_rules_can_make_offer', '0x5c1930', 'france_rules.cpp', 'thiscall, ret 0x10',
          'regra: na janela restrita, o comprador com uma compra (0x10) ou uma livre (0xf, pré-contrato) já feitas, ou o vendedor cujos jogadores de troca passem de uma compra (0x10), não pode; fora das janelas 9; senão a regra de base',
          'rule: in the restricted window, a buyer with a purchase (0x10) or a free signing (0xf, pre-contract) already made, or a seller whose exchange players would pass one purchase (0x10), may not; outside the windows 9; else the base rule'),
         ('italy_rules_can_sign_player', 'italy_rules_can_sign_player', '0x661520', 'italy_rules.cpp', 'thiscall, ret 0x14',
          'regra: nas duas primeiras janelas a regra de base; na terceira só jogador livre ou de saída pela lei Bosman; fora delas 9',
          'rule: in the first two windows the base rule; in the third only a free agent or a player leaving on a Bosman; outside them 9'),
         ('italy_rules_can_make_offer', 'italy_rules_can_make_offer', '0x661630', 'italy_rules.cpp', 'thiscall, ret 0x10',
          'regra: o comprador (e o vendedor com troca) só dentro das janelas (9); na terceira janela o comprador só com pré-contrato (9); senão a regra de base',
          'rule: the buyer (and a seller with exchange players) only inside the windows (9); in the third window the buyer only with a pre-contract (9); else the base rule')])
CAMPOS = []
INTERESSA = []
