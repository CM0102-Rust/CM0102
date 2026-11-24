# O orgao disciplinar da copa sueca (rb_sweden_cup.cpp: o construtor e as regras dos cartoes; leitor sx19, rodada 5,
# 24/11/2026). Usa as pecas do sx18_1, do sx19_1 e do sx19_6 (sweden_card_verdict): integrar depois deles.
# So roda com a Suecia carregada: o construtor na criacao do jogo, as regras nas partidas da Svenska Cupen com cartao.
# Convencoes conferidas na desmontagem:
# - 0x7e32d0 thiscall(arquivo) ret 4: 0x7e7760, vtable 0x96ee94, +4 = 1, +5 = 0x1d, +6 = 0xc, +0x17 Suecia, +0x1b
#   Europa, +0x1f = 0x1c, +0x20 = 0x1d, +0x21 = 1, +0x22 = 0;
# - 0x7e3320 thiscall(linha do registro, partida) ret 8: como a liga (sx19_6), regras ff 1d ff ff, mas o vermelho so da
#   1 jogo na hora com exatamente 2 amarelos na partida (cmp byte [linha+0x2c], 2); o no pendente do vermelho vai a
#   lista sempre.
# Defeito do original: os bits de lixo do no pendente e das marcas, como na liga.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('rb_sweden_cup_construct', 0x7e32d0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_sweden_cup_process_match_cards', 0x7e3320, [0x6a, 0xff, 0x68, 0xb1, 0x05, 0x96, 0x00])]
DOCS = ('o órgão disciplinar da copa sueca',
        'the Swedish cup ruling body',
        [('rb_sweden_cup_construct', 'RbSwedenCup::RbSwedenCup', '0x7e32d0', 'ruling_body/rb_sweden_cup.cpp', 'thiscall, ret 4',
          'monta o órgão da copa sueca sobre a base: tabela virtual 0x96ee94, a vaga 1 do registro, a regra 0x1d, a escala 12, a Suécia na Europa, as regras da liga (0x1c), da copa (0x1d) e da confederação (1)',
          'builds the Swedish cup body on the base: vtable 0x96ee94, record slot 1, rule set 0x1d, scale 12, Sweden in Europe, the league (0x1c), cup (0x1d) and confederation (1) rule sets'),
         ('rb_sweden_cup_process_match_cards', 'RbSwedenCup::process_match_cards', '0x7e3320', 'ruling_body/rb_sweden_cup.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras da copa sueca: o vermelho põe na lista do órgão uma suspensão por pontos a partir de amanhã e só dá 1 jogo na hora quando veio do segundo amarelo; os amarelos como na liga (o terceiro e, do quinto em diante, cada ímpar); a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the Swedish cup rules: a red card queues on the body\'s list a points ban from tomorrow and only bans for 1 match on the spot when it came from a second yellow; the bookings as in the league (the third and, from the fifth on, each odd one); from today, with its news')])
CAMPOS = []
INTERESSA = []
