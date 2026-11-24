# As regras de transferencia de base (transfer_offer.cpp pelo mapa; leitor sx21, rodada 5, 24/11/2026): o construtor
# (de todas as 28 regras, na montagem do gestor de transferencias 0x8d26b0, uma vez na criacao do jogo), quem um clube
# pode contratar (slot 0x10, das nacoes sem regra propria: a Inglaterra entre elas) e de que oferta ele pode participar
# (slot 0xc). Rodam na carreira ENG.
# Convencoes conferidas: 0x8e84a0 thiscall(regra, nacao, continente, _, _) ret 0x14 (os dois ultimos nao sao lidos);
# 0x8e8560 thiscall(clube, jogador, _, data, motivo) ret 0x14; 0x8e8800 thiscall(oferta, clube, data, motivo) ret 0x10;
# 0x53d980 cdecl(pessoa, nacao, 1); 0x53d740 cdecl(pessoa, nacao); 0x8e8c40 thiscall(data, janela) ret 8;
# 0x68d1b0 thiscall ret 0.
# Comparadores: o construtor pelo compare_transfer_rules_ctor; as duas regras lado a lado no retorno e no motivo.
SAVE = False
ROTINAS = [
    ('construct_transfer_rules', 0x8e84a0, [0x8b, 0xc1, 0x56, 0x8a, 0x4c, 0x24, 0x08]),
    ('transfer_rules_can_sign_player', 0x8e8560, [0x83, 0xec, 0x08, 0x53, 0x55]),
    ('transfer_rules_can_make_offer', 0x8e8800, [0x83, 0xec, 0x08, 0x53, 0x55]),
]
DOCS = ('as regras de transferência de base: a montagem, quem um clube pode contratar e de que oferta pode participar',
        'the base transfer rules: building them, who a club may sign and which offers it may take part in',
        [('construct_transfer_rules', 'construct_transfer_rules', '0x8e84a0', 'transfer_offer.cpp', 'thiscall, ret 0x14',
          'regra: vtable 0x970458 e o número da regra (+9); com nação: o registro dela (+0xa), o continente dela (+0xe) e o alcance 1; senão, com continente: o registro do continente (+0xe) e o alcance 2; senão alcance 4; sempre a tabela de janelas e +0x15 zeradas, 50 estrangeiros (+0x13) e 30 de fora da UE (+0x14)',
          'rule: vtable 0x970458 and the rule number (+9); with a nation: its record (+0xa), its continent (+0xe) and scope 1; else with a continent: the continent record (+0xe) and scope 2; else scope 4; always the window table and +0x15 cleared, 50 foreign players (+0x13) and 30 non-EU (+0x14)'),
         ('transfer_rules_can_sign_player', 'transfer_rules_can_sign_player', '0x8e8560', 'transfer_offer.cpp', 'thiscall, ret 0x14',
          'regra: um estrangeiro não entra quando o elenco já tem o limite de estrangeiros (motivo 0x14; 0 para clube turco fora da primeira divisão); numa nação europeia, um estrangeiro de fora da UE não entra quando o elenco já tem o limite deles (0x15; na Espanha 4 na primeira, 3 na segunda e 0 nas outras); fora das janelas, motivo 9',
          'rule: a foreign player may not join when the squad already holds the foreign limit (reason 0x14; 0 for a Turkish club outside the top division); in a European nation, a non-EU foreigner may not join when the squad holds the non-EU limit (0x15; in Spain 4 in the first division, 3 in the second and 0 elsewhere); outside the windows, reason 9'),
         ('transfer_rules_can_make_offer', 'transfer_rules_can_make_offer', '0x8e8800', 'transfer_offer.cpp', 'thiscall, ret 0x10',
          'regra: os estrangeiros que a transferência traz (o alvo para o comprador, os jogadores de troca para o vendedor) somados aos do elenco não passam do limite (0x14); um jogador de troca estrangeiro dispensa a conta do comprador; o comprador fora das janelas: motivo 9',
          'rule: the foreign players the move brings (the target for the buyer, the exchange players for the seller) plus the squad\'s may not pass the limit (0x14); a foreign exchange player waives the buyer\'s count; a buyer outside the windows: reason 9')])
CAMPOS = []
INTERESSA = []
