# As regras dos cartoes do orgao portugues e o orgao da liga sueca (construtor e regras dos cartoes) (rb_portugal.cpp,
# rb_sweden_league.cpp; leitor sx19, rodada 5, 24/11/2026). Usa as pecas do sx18_1, do sx19_1 (stepped_card_verdict,
# own_rules) e do sx19_5 (PORTUGAL_RULING_BODY): integrar depois deles. O sx19_7 usa sweden_card_verdict deste.
# So rodam com as ligas carregadas (Portugal e Suecia selecionados): as regras nas partidas das ligas com cartao (a
# Allsvenskan joga de abril a outubro de 2002: ATE 2002-08-01).
# Convencoes conferidas na desmontagem:
# - 0x7e1ab0 thiscall(linha do registro, partida) ret 8: vermelho (+0x2d == 1): 0x7e8e10 make_points_ban(no, linha, 1);
#   com suspensao, o no comeca amanha (0x54c5b0 add_days de hoje, 1), regras 17 ff ff ff, a partida em +0x20, e vai a
#   lista do orgao (0x68d0d0); depois 1 jogo (motivo 2, quantidade 1). Amarelo: +1 na entrada; a cada quinto mais 1
#   jogo (motivo 1, quantidade 5). A partir de hoje, regras 17 ff ff ff;
# - 0x7e3650 thiscall(arquivo) ret 4: 0x7e7760, vtable 0x96eeb4, +4 = 0, +5 = 0x1c, +6 = 0xc, +0x17 Suecia (0x9cf4a8),
#   +0x1b Europa, +0x1f = 0x1c, +0x20 = 0x1d, +0x21 = 1, +0x22 = 0;
# - 0x7e36a0 thiscall(linha do registro, partida) ret 8: vermelho: o no pendente (amanha, regras 1c ff ff ff) e 1 jogo
#   (motivo 2, quantidade 1); amarelo: +1; no 3 mais 1 jogo (quantidade 3, sete bl + 2) e de 5 em diante a cada impar
#   mais 1 (quantidade 2); motivo 1; a partir de hoje, regras 1c ff ff ff.
# Defeito do original: no no pendente, a quantidade e as marcas (+0x11..+0x13) com lixo da pilha (a copia grava 0); os
# bits de lixo das marcas da suspensao, como nos outros orgaos.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('rb_portugal_process_match_cards', 0x7e1ab0, [0x6a, 0xff, 0x68, 0x26, 0x04, 0x96, 0x00]),
           ('rb_sweden_league_construct', 0x7e3650, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_sweden_league_process_match_cards', 0x7e36a0, [0x6a, 0xff, 0x68, 0xf1, 0x05, 0x96, 0x00])]
DOCS = ('as regras dos cartões do órgão português e o órgão da liga sueca',
        'the Portuguese card rules and the Swedish league ruling body',
        [('rb_portugal_process_match_cards', 'RbPortugal::process_match_cards', '0x7e1ab0', 'ruling_body/rb_portugal.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras portuguesas: o vermelho põe na lista do órgão uma suspensão por pontos a partir de amanhã (quando a falta dá uma) e dá 1 jogo; cada partida com amarelo soma 1 e a cada quinto dá mais 1 jogo; a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the Portuguese rules: a red card queues on the body\'s list a points ban from tomorrow (when the offence brings one) and bans for 1 match; each match with a booking adds 1 and every fifth bans for 1 more match; from today, with its news'),
         ('rb_sweden_league_construct', 'RbSwedenLeague::RbSwedenLeague', '0x7e3650', 'ruling_body/rb_sweden_league.cpp', 'thiscall, ret 4',
          'monta o órgão da liga sueca sobre a base: tabela virtual 0x96eeb4, a vaga 0 do registro, a regra 0x1c, a escala 12, a Suécia na Europa, as regras da liga (0x1c), da copa (0x1d) e da confederação (1)',
          'builds the Swedish league body on the base: vtable 0x96eeb4, record slot 0, rule set 0x1c, scale 12, Sweden in Europe, the league (0x1c), cup (0x1d) and confederation (1) rule sets'),
         ('rb_sweden_league_process_match_cards', 'RbSwedenLeague::process_match_cards', '0x7e36a0', 'ruling_body/rb_sweden_league.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras da liga sueca: o vermelho põe na lista do órgão uma suspensão por pontos a partir de amanhã e dá 1 jogo; cada partida com amarelo soma 1; o terceiro dá mais 1 jogo e, do quinto em diante, cada ímpar; a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the Swedish league rules: a red card queues on the body\'s list a points ban from tomorrow and bans for 1 match; each match with a booking adds 1; the third bans for 1 more match and, from the fifth on, each odd one; from today, with its news')])
CAMPOS = []
INTERESSA = []
