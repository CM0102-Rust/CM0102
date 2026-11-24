# O orgao disciplinar polones (construtor e regras dos cartoes) e o construtor do russo (rb_poland.cpp,
# rb_russia.cpp; leitor sx19, rodada 5, 24/11/2026). Usa as pecas do sx18_1 (pending_points_ban,
# first_weekday_after_today, ...) e do sx19_1 (own_rules): integrar depois deles.
# So rodam com as ligas carregadas: os construtores na criacao do jogo (Polonia e Russia selecionadas), as regras
# polonesas nas partidas da liga polonesa com cartao.
# Convencoes conferidas na desmontagem:
# - 0x7e1610 / 0x7e1e00 thiscall(arquivo) ret 4: 0x7e7760 (base), depois: Polonia vtable 0x96ed94, +4 = 0, +5 = 0x1f,
#   +6 = 0xa, +0x17 Polonia (0x9cf430), +0x1f = 0x1f; Russia 0x96edd4, +4 = 0, +5 = 0x27, +6 = 8, +0x17 Russia
#   (0x9cf448), +0x1f = 0x27; os dois +0x1b Europa, +0x20 = 0xff, +0x21 = 1, +0x22 = 0;
# - 0x7e1660 thiscall(linha do registro, partida) ret 8: para cada amarelo da partida (al de 0 a +0x2c, movsx), +1 na
#   entrada; 4 e 7 somam 1 jogo, de 10 em diante cada par soma 2; com jogos, motivo 1 e quantidade = a contagem (bl).
#   Depois, vermelho (+0x2d == 1): 0x7e8e10 make_points_ban(no, linha, 1); com suspensao, o no comeca no primeiro dia
#   de semana 3 depois de hoje (amanha, 0x54c5b0 add_days 1 ate 0x54a3f0 get_weekday == 3), regras 1f ff ff ff, a
#   partida em +0x20, e vai a lista do orgao (0x68d0d0); motivo 2, mais 1 jogo, quantidade 1. A partir de hoje, regras
#   1f ff ff ff; 0x556940 add_ban e 0x557280 post_ban_news.
# Defeito do original: os bits de lixo do no pendente e das marcas, como nos outros orgaos.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('rb_poland_construct', 0x7e1610, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_poland_process_match_cards', 0x7e1660, [0x6a, 0xff, 0x68, 0xec, 0x03, 0x96, 0x00]),
           ('rb_russia_construct', 0x7e1e00, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('o órgão disciplinar polonês e o construtor do russo',
        'the Polish ruling body and the Russian body\'s constructor',
        [('rb_poland_construct', 'RbPoland::RbPoland', '0x7e1610', 'ruling_body/rb_poland.cpp', 'thiscall, ret 4',
          'monta o órgão polonês sobre a base: tabela virtual 0x96ed94, a vaga 0 do registro, a regra 0x1f, a escala 10, a Polônia na Europa, a regra da liga 0x1f, sem copa, a da confederação 1',
          'builds the Polish body on the base: vtable 0x96ed94, record slot 0, rule set 0x1f, scale 10, Poland in Europe, league rule 0x1f, no cup rule, confederation rule 1'),
         ('rb_poland_process_match_cards', 'RbPoland::process_match_cards', '0x7e1660', 'ruling_body/rb_poland.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras polonesas: cada amarelo soma 1; o 4º e o 7º dão 1 jogo e, do 10º em diante, cada par dá 2; o vermelho põe na lista do órgão uma suspensão por pontos a partir da próxima quarta (dia de semana 3) e dá mais 1 jogo; a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the Polish rules: each booking adds 1; the 4th and the 7th ban for 1 match and, from the 10th on, each even one for 2; a red card queues on the body\'s list a points ban from the next weekday 3 and bans for 1 more match; from today, with its news'),
         ('rb_russia_construct', 'RbRussia::RbRussia', '0x7e1e00', 'ruling_body/rb_russia.cpp', 'thiscall, ret 4',
          'monta o órgão russo sobre a base: tabela virtual 0x96edd4, a vaga 0 do registro, a regra 0x27, a escala 8, a Rússia na Europa, a regra da liga 0x27, sem copa, a da confederação 1',
          'builds the Russian body on the base: vtable 0x96edd4, record slot 0, rule set 0x27, scale 8, Russia in Europe, league rule 0x27, no cup rule, confederation rule 1')])
CAMPOS = []
INTERESSA = []
