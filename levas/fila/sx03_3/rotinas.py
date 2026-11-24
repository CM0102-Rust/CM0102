# As regras de transferencia da Europa (european_awards.cpp pelo mapa; leitor sx03, rodada 5, 23/11/2026): o
# construtor (uma vez, na montagem das regras do gestor de transferencias, 0x8d26fd), os limites de elenco padrao
# (slot 0x1c das regras sem limites proprios, a Europa entre elas) e quem nao pode jogar uma partida europeia (slot
# 0x18 da vtable 0x96a188). Rodam na carreira ENG (as copas europeias dos clubes ingleses).
# Convencoes conferidas: 0x583280 thiscall 5 args ret 0x14 (0x8e84a0 com os mesmos 5, na mesma ordem);
# 0x5832c0 thiscall(partida, saida) ret 8 (a partida nao e lida); 0x5832e0 thiscall(jogador, partida) ret 8, al;
# 0x8e8a60 thiscall(jogador, partida) ret 8, al; 0x4b0870 cdecl (Rust, round_below_1000).
# Ficaram fora da lista: 0x583470 (shared_noop_ret4, `ret 4` de 3 bytes, sem prologo de 5).
# Comparadores: o construtor por compare_transfer_rules_ctor (england_rules.rs); os limites lado a lado nos 42 bytes;
# a regra pelo codigo (al).
SAVE = False
ROTINAS = [
    ('construct_europe_rules', 0x583280, [0x8b, 0x44, 0x24, 0x14, 0x8b, 0x54, 0x24, 0x0c]),
    ('default_squad_restrictions', 0x5832c0, [0x57, 0x8b, 0x7c, 0x24, 0x0c]),
    ('europe_rules_get_ineligibility_code', 0x5832e0, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
]
DOCS = ('as regras de transferência da Europa: a montagem, os limites de elenco padrão e quem não pode jogar na Europa',
        'the Europe-wide transfer rules: building them, the default squad limits and who may not play in Europe',
        [('construct_europe_rules', 'construct_europe_rules', '0x583280', 'european_awards.cpp', 'thiscall, ret 0x14',
          'regra: a base das regras com os cinco argumentos, a vtable da Europa (0x96a188) e a contagem de janelas (+8) zerada',
          "rule: the base rules with the five arguments, the Europe vtable (0x96a188) and the window count (+8) cleared"),
         ('default_squad_restrictions', 'default_squad_restrictions', '0x5832c0', 'european_awards.cpp', 'thiscall, ret 8',
          'regra: sem limites de elenco: os 42 bytes do registro de limites ficam em 0xff',
          'rule: no squad limits: the 42 bytes of the restriction record are set to 0xff'),
         ('europe_rules_get_ineligibility_code', 'europe_rules_get_ineligibility_code', '0x5832e0', 'european_awards.cpp', 'thiscall, ret 8',
          'regra: a resposta da base para pessoa virtual, jogador com menos de 20 anos, a Supercopa, competição que não seja Intertoto, Copa UEFA, Liga dos Campeões ou Recopa, ou de outra temporada; na Intertoto, só a rodada 0x96 aplica a regra europeia; nas outras, as rodadas 0xe6 e 0xa0 dão 0 e as demais a regra europeia: 8 (inscrito por outro clube) quando a inscrição do jogador (+0x6d) não é 2',
          "rule: the base answer for a virtual person, a player under 20, the Super Cup, a competition other than the Intertoto, UEFA Cup, Champions League or Cup Winners' Cup, or of another season; in the Intertoto only round 0x96 applies the European rule; elsewhere rounds 0xe6 and 0xa0 give 0 and the others the European rule: 8 (cup-tied) when the player's registration (+0x6d) is not 2")])
CAMPOS = []
INTERESSA = []
