# A oferta de transferencia: o construtor vazio, a soma das clausulas e a situacao das conversas de contrato de um
# jogador da oferta (transfer_offer.cpp; leitor sx21, rodada 5, 24/11/2026). Rodam nas ofertas da carreira ENG (o
# construtor a cada oferta nova ou carregada; os outros dois na avaliacao e no andamento das ofertas).
# Convencoes conferidas: 0x8e5680 thiscall ret 0 (eax = this; 0x54a110 thiscall ret 0 no +0x12); 0x8e7ef0 thiscall
# ret 0; 0x8e7f10 thiscall(id) ret 4, al; 0x68d1b0 thiscall ret 0.
# Comparadores: o construtor lado a lado nas listas e na data; a soma pelo valor; a situacao (al) e o byte +0x10 lado a
# lado.
SAVE = False
ROTINAS = [
    ('construct_transfer_offer', 0x8e5680, [0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x12]),
    ('transfer_offer_get_clause_total', 0x8e7ef0, [0x8b, 0x51, 0x35, 0x33, 0xc0]),
    ('transfer_offer_get_player_talks_status', 0x8e7f10, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
]
DOCS = ('a oferta de transferência: o construtor vazio, a soma das cláusulas e a situação das conversas de um jogador da oferta',
        "a transfer offer: the empty constructor, the clause total and a player's contract talks status",
        [('construct_transfer_offer', 'construct_transfer_offer', '0x8e5680', 'transfer_offer.cpp', 'thiscall, ret 0',
          'regra: a data da oferta (+0x12) construída, a lista de ofertas rivais (+0) e a de jogadores de troca (+4) zeradas',
          "rule: the offer's date (+0x12) built, the rival offer list (+0) and the exchange player list (+4) cleared"),
         ('transfer_offer_get_clause_total', 'transfer_offer_get_clause_total', '0x8e7ef0', 'transfer_offer.cpp', 'thiscall, ret 0',
          'regra: a cláusula de jogos (+0x35) mais a de seleção (+0x3a), cada uma só se positiva',
          'rule: the appearance clause (+0x35) plus the international clause (+0x3a), each only when positive'),
         ('transfer_offer_get_player_talks_status', 'transfer_offer_get_player_talks_status', '0x8e7f10', 'transfer_offer.cpp', 'thiscall, ret 4',
          'regra: o alvo: com a oferta retirada (situação 3) as conversas viram 4 (fracassadas), e devolve +0x10; um jogador de troca: o byte +4 da entrada dele; fora da oferta: 4',
          'rule: the target: with the offer withdrawn (status 3) the talks become 4 (collapsed), and +0x10 is returned; an exchange player: his entry byte +4; not in the offer: 4')])
CAMPOS = []
INTERESSA = []
