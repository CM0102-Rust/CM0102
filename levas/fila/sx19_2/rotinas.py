# Os orgaos disciplinares espanhois: o da liga (construtor e regras dos cartoes) e o construtor do da copa
# (rb_spain_league.cpp, rb_spain_cup.cpp; leitor sx19, rodada 5, 24/11/2026). Usa as pecas do sx18_1 e do sx19_1
# (stepped_card_verdict, own_rules): integrar depois deles.
# So rodam com a liga espanhola carregada: os construtores na criacao do jogo (Discipline::create_rule_sets, Espanha
# selecionada), as regras da liga depois de cada partida da Liga com cartao (RulingBody::process_match 0x7e7ff0, vtable
# +8, por linha do registro da partida).
# Convencoes conferidas na desmontagem:
# - 0x7e2f80 / 0x7e2c30 thiscall(arquivo) ret 4: 0x7e7760 (base), depois os campos: liga vtable 0x96ee74, +4 = 0,
#   +5 = 0x1a; copa 0x96ee54, +4 = 1, +5 = 0x1b; as duas +6 = 0xa, +0x17 Espanha (0x9cf488), +0x1b Europa, +0x1f = 0x1a,
#   +0x20 = 0x1b, +0x21 = 1, +0x22 = 0;
# - 0x7e2fd0 thiscall(linha do registro, partida) ret 8: vermelho (+0x2d == 1): 0x7e8e10 make_points_ban thiscall(no,
#   linha, 1) ret 0xc; com suspensao, o no de 0x70 bytes comeca amanha (copia de hoje, 0x54c5b0 add_days thiscall(saida,
#   1) ret 8, 0x417c10 operator=), regras 1a ff ff ff, a partida (0x4f bytes) em +0x20, e vai ao fim da lista do orgao
#   (0x68d0d0 LLIST::add_tail thiscall(no, 0x70) ret 8); depois 1 jogo (motivo 2, quantidade 1). Amarelo: +1 na entrada,
#   a cada quinto mais 1 jogo (motivo 1, quantidade 5). Suspensao a partir de hoje, regras 1a ff ff ff; 0x556940
#   add_ban e 0x557280 post_ban_news.
# Defeito do original: no no pendente, a quantidade e as marcas (+0x11..+0x13) saem com lixo da pilha (a copia grava 0);
# no registro da suspensao, os bits 0, 1 e 3..7 das marcas (+0x13) tambem (o comparador olha so o bit 2).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('rb_spain_league_construct', 0x7e2f80, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_spain_league_process_match_cards', 0x7e2fd0, [0x6a, 0xff, 0x68, 0x71, 0x05, 0x96, 0x00]),
           ('rb_spain_cup_construct', 0x7e2c30, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('o órgão disciplinar da liga espanhola e o construtor do da copa',
        'the Spanish league ruling body and the Spanish cup body\'s constructor',
        [('rb_spain_league_construct', 'RbSpainLeague::RbSpainLeague', '0x7e2f80', 'ruling_body/rb_spain_league.cpp', 'thiscall, ret 4',
          'monta o órgão da liga espanhola sobre a base: tabela virtual 0x96ee74, a vaga 0 do registro, a regra 0x1a, a escala 10, a Espanha na Europa, as regras da liga (0x1a), da copa (0x1b) e da confederação (1)',
          'builds the Spanish league body on the base: vtable 0x96ee74, record slot 0, rule set 0x1a, scale 10, Spain in Europe, the league (0x1a), cup (0x1b) and confederation (1) rule sets'),
         ('rb_spain_league_process_match_cards', 'RbSpainLeague::process_match_cards', '0x7e2fd0', 'ruling_body/rb_spain_league.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras da liga espanhola: o vermelho põe na lista do órgão uma suspensão por pontos a partir de amanhã (quando a falta dá uma) e dá 1 jogo; cada partida com amarelo soma 1 e a cada quinto dá mais 1 jogo; a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the Spanish league rules: a red card queues on the body\'s list a points ban from tomorrow (when the offence brings one) and bans for 1 match; each match with a booking adds 1 and every fifth bans for 1 more match; from today, with its news'),
         ('rb_spain_cup_construct', 'RbSpainCup::RbSpainCup', '0x7e2c30', 'ruling_body/rb_spain_cup.cpp', 'thiscall, ret 4',
          'monta o órgão da copa espanhola sobre a base: tabela virtual 0x96ee54, a vaga 1 do registro, a regra 0x1b, a escala 10, a Espanha na Europa, as regras da liga (0x1a), da copa (0x1b) e da confederação (1)',
          'builds the Spanish cup body on the base: vtable 0x96ee54, record slot 1, rule set 0x1b, scale 10, Spain in Europe, the league (0x1a), cup (0x1b) and confederation (1) rule sets')])
CAMPOS = []
INTERESSA = []
