# O orgao disciplinar da copa turca (rb_turkey_cup.cpp: o construtor e as regras dos cartoes; leitor sx19, rodada 5,
# 24/11/2026). Usa as pecas do sx18_1 e do sx19_1 (stepped_card_verdict, own_rules): integrar depois deles.
# So roda com a Turquia carregada: o construtor na criacao do jogo, as regras nas partidas da copa turca com cartao.
# Convencoes conferidas na desmontagem:
# - 0x7e3a10 thiscall(arquivo) ret 4: 0x7e7760, vtable 0x96eed4, +4 = 1, +5 = 0x26, +6 = 0xa, +0x17 Turquia, +0x1b
#   Europa, +0x1f = 0x25, +0x20 = 0x26, +0x21 = 1, +0x22 = 0;
# - 0x7e3a60 thiscall(linha do registro, partida) ret 8: como a liga (sx19_9), regras ff 26 ff ff: o vermelho poe o no
#   pendente (amanha: copia de hoje e 0x54c5b0 add_days 1) e da 1 jogo; a cada quarto amarelo (um por partida) mais 1
#   jogo (motivo 1, quantidade 4); a partir de hoje.
# Defeito do original: os bits de lixo do no pendente e das marcas, como nos outros orgaos.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('rb_turkey_cup_construct', 0x7e3a10, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_turkey_cup_process_match_cards', 0x7e3a60, [0x6a, 0xff, 0x68, 0x31, 0x06, 0x96, 0x00])]
DOCS = ('o órgão disciplinar da copa turca',
        'the Turkish cup ruling body',
        [('rb_turkey_cup_construct', 'RbTurkeyCup::RbTurkeyCup', '0x7e3a10', 'ruling_body/rb_turkey_cup.cpp', 'thiscall, ret 4',
          'monta o órgão da copa turca sobre a base: tabela virtual 0x96eed4, a vaga 1 do registro, a regra 0x26, a escala 10, a Turquia na Europa, as regras da liga (0x25), da copa (0x26) e da confederação (1)',
          'builds the Turkish cup body on the base: vtable 0x96eed4, record slot 1, rule set 0x26, scale 10, Turkey in Europe, the league (0x25), cup (0x26) and confederation (1) rule sets'),
         ('rb_turkey_cup_process_match_cards', 'RbTurkeyCup::process_match_cards', '0x7e3a60', 'ruling_body/rb_turkey_cup.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras da copa turca: o vermelho põe na lista do órgão uma suspensão por pontos a partir de amanhã (quando a falta dá uma) e dá 1 jogo; cada partida com amarelo soma 1 e a cada quarto dá mais 1 jogo; a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the Turkish cup rules: a red card queues on the body\'s list a points ban from tomorrow (when the offence brings one) and bans for 1 match; each match with a booking adds 1 and every fourth bans for 1 more match; from today, with its news')])
CAMPOS = []
INTERESSA = []
