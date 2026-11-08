# Mutacoes (com o ramo de cada uma), todas em can_club_bid_for_player, que as listas de alvos dos clubes chamam
# todo dia (os outros dois so rodam com trocas e com o elenco no limit). O jogador com oferta em negociacao fica
# livre para outros clubes (as listas de alvos e as ofertas seguintes mudam). O jogador com contrato acertado tambem
# (e perde a marca 0x40). O jogador com transferencia futura acertada tambem. Os ramos dizem se a carreira passou por
# eles.
MUTACOES = [
    ('cpr2_negociacao_livre', 'return (0x12, 0); // já em negociação',
     'return (0, 1); // mutacao: pode mesmo em negociacao',
     'cpr_2 offer under negotiation'),
    ('cpr2_acerto_livre', 'return (0x21, 0); // contrato acertado',
     'let _ = 0x21; // mutacao: o acerto nao segura',
     'cpr_2 contract agreed'),
    ('cpr2_futura_livre', 'return (0x20, 0); // transferência futura acertada',
     'return (0, 1); // mutacao: pode mesmo com a futura',
     'cpr_2 future transfer'),
]
