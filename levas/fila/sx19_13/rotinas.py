# O orgao disciplinar do Pais de Gales (rb_wales.cpp: o construtor e as regras dos cartoes; leitor sx19, rodada 5,
# 24/11/2026). Usa as pecas do sx18_1 e do sx19_1 (stepped_card_verdict, own_rules): integrar depois deles.
# So roda com Gales carregado: o construtor na criacao do jogo, as regras nas partidas da League of Wales com cartao.
# Convencoes conferidas na desmontagem:
# - 0x7e43c0 thiscall(arquivo) ret 4: 0x7e7760, vtable 0x96ef34, +4 = 0, +5 = 0x28, +6 = 8, +0x17 Gales (0x9cf510),
#   +0x1b Europa, +0x1f = 0x28, +0x20 = 0xff, +0x21 = 1, +0x22 = 0;
# - 0x7e4410 thiscall(linha do registro, partida) ret 8: vermelho (+0x2d == 1): 0x7e8e10 make_points_ban(no, linha, 1),
#   o no a partir de amanha (0x54c5b0 add_days de hoje, 1), regras 28 ff ff ff, a partida em +0x20, a lista do orgao
#   (0x68d0d0); 1 jogo (motivo 2, quantidade 1); amarelo: +1, a cada quinto mais 1 jogo (motivo 1, quantidade 5); a
#   partir de hoje, regras 28 ff ff ff; 0x556940 e 0x557280.
# Defeito do original: os bits de lixo do no pendente e das marcas, como nos outros orgaos.
SAVE = False
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
ROTINAS = [('rb_wales_construct', 0x7e43c0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_wales_process_match_cards', 0x7e4410, [0x6a, 0xff, 0x68, 0xc6, 0x06, 0x96, 0x00])]
DOCS = ('o órgão disciplinar do País de Gales',
        'the Welsh ruling body',
        [('rb_wales_construct', 'RbWales::RbWales', '0x7e43c0', 'ruling_body/rb_wales.cpp', 'thiscall, ret 4',
          'monta o órgão galês sobre a base: tabela virtual 0x96ef34, a vaga 0 do registro, a regra 0x28, a escala 8, Gales na Europa, a regra da liga 0x28, sem copa, a da confederação 1',
          'builds the Welsh body on the base: vtable 0x96ef34, record slot 0, rule set 0x28, scale 8, Wales in Europe, league rule 0x28, no cup rule, confederation rule 1'),
         ('rb_wales_process_match_cards', 'RbWales::process_match_cards', '0x7e4410', 'ruling_body/rb_wales.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras galesas: o vermelho põe na lista do órgão uma suspensão por pontos a partir de amanhã (quando a falta dá uma) e dá 1 jogo; cada partida com amarelo soma 1 e a cada quinto dá mais 1 jogo; a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the Welsh rules: a red card queues on the body\'s list a points ban from tomorrow (when the offence brings one) and bans for 1 match; each match with a booking adds 1 and every fifth bans for 1 more match; from today, with its news')])
CAMPOS = []
INTERESSA = []
