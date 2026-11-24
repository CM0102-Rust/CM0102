# As regras dos cartoes do orgao russo e o orgao da liga turca (construtor e regras dos cartoes) (rb_russia.cpp,
# rb_turkey_league.cpp; leitor sx19, rodada 5, 24/11/2026). Usa as pecas do sx18_1, do sx19_1 (stepped_card_verdict,
# own_rules) e do sx19_8 (RUSSIA_RULING_BODY): integrar depois deles. O sx19_10 usa TURKEY_LEAGUE_RULING_BODY deste.
# So rodam com as ligas carregadas (Russia e Turquia selecionadas): as regras nas partidas das ligas com cartao (a
# liga russa joga de marco a novembro de 2002: ATE 2002-08-01).
# Convencoes conferidas na desmontagem:
# - 0x7e1e50 thiscall(linha do registro, partida) ret 8: a entrada soma todos os amarelos da partida (+0x2c) e, com 4
#   ou mais, desconta 4 (sem suspensao); so com vermelho (+0x2d == 1): 0x7e8e10 make_points_ban(no, linha, 1), o no
#   a partir do primeiro dia de semana 3 depois de hoje, regras 27 ff ff ff, a partida em +0x20, a lista do orgao
#   (0x68d0d0); e 1 jogo (motivo 2, quantidade 1) a partir de hoje, regras 27 ff ff ff;
# - 0x7e3d60 thiscall(arquivo) ret 4: 0x7e7760, vtable 0x96eef4, +4 = 0, +5 = 0x25, +6 = 7, +0x17 Turquia (0x9cf4dc),
#   +0x1b Europa, +0x1f = 0x25, +0x20 = 0x26, +0x21 = 1, +0x22 = 0;
# - 0x7e3db0 thiscall(linha do registro, partida) ret 8: o vermelho poe o no pendente (amanha, regras 25 ff ff ff) e da
#   1 jogo; cada partida com amarelo soma 1 e a cada quarto mais 1 jogo (motivo 1, quantidade 4); a partir de hoje.
# Defeitos do original: na Russia o ciclo de quatro amarelos nunca suspende (a contagem so volta 4; o
# is_one_booking_from_ban russo avisa no terceiro, mas o quarto nao suspende); os bits de lixo do no pendente e das
# marcas, como nos outros orgaos.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('rb_russia_process_match_cards', 0x7e1e50, [0x6a, 0xff, 0x68, 0x6c, 0x04, 0x96, 0x00]),
           ('rb_turkey_league_construct', 0x7e3d60, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_turkey_league_process_match_cards', 0x7e3db0, [0x6a, 0xff, 0x68, 0x71, 0x06, 0x96, 0x00])]
DOCS = ('as regras dos cartões do órgão russo e o órgão da liga turca',
        'the Russian card rules and the Turkish league ruling body',
        [('rb_russia_process_match_cards', 'RbRussia::process_match_cards', '0x7e1e50', 'ruling_body/rb_russia.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras russas: os amarelos somam e a contagem volta 4 ao chegar a 4, sem suspensão; o vermelho põe na lista do órgão uma suspensão por pontos a partir da próxima quarta (dia de semana 3) e dá 1 jogo a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the Russian rules: bookings add up and the count drops by 4 on reaching 4, with no ban; a red card queues on the body\'s list a points ban from the next weekday 3 and bans for 1 match from today, with its news'),
         ('rb_turkey_league_construct', 'RbTurkeyLeague::RbTurkeyLeague', '0x7e3d60', 'ruling_body/rb_turkey_league.cpp', 'thiscall, ret 4',
          'monta o órgão da liga turca sobre a base: tabela virtual 0x96eef4, a vaga 0 do registro, a regra 0x25, a escala 7, a Turquia na Europa, as regras da liga (0x25), da copa (0x26) e da confederação (1)',
          'builds the Turkish league body on the base: vtable 0x96eef4, record slot 0, rule set 0x25, scale 7, Turkey in Europe, the league (0x25), cup (0x26) and confederation (1) rule sets'),
         ('rb_turkey_league_process_match_cards', 'RbTurkeyLeague::process_match_cards', '0x7e3db0', 'ruling_body/rb_turkey_league.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras da liga turca: o vermelho põe na lista do órgão uma suspensão por pontos a partir de amanhã (quando a falta dá uma) e dá 1 jogo; cada partida com amarelo soma 1 e a cada quarto dá mais 1 jogo; a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the Turkish league rules: a red card queues on the body\'s list a points ban from tomorrow (when the offence brings one) and bans for 1 match; each match with a booking adds 1 and every fourth bans for 1 more match; from today, with its news')])
CAMPOS = []
INTERESSA = []
