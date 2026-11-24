# As regras dos cartoes da copa espanhola e o orgao disciplinar dos Estados Unidos (construtor e regras dos cartoes)
# (rb_spain_cup.cpp, rb_usa.cpp; leitor sx19, rodada 5, 24/11/2026). Usa as pecas do sx18_1, do sx19_1
# (stepped_card_verdict, own_rules) e do sx19_2 (SPAIN_CUP_RULING_BODY): integrar depois deles.
# So rodam com as ligas carregadas: a copa espanhola nas partidas da Copa do Rei com cartao; o orgao americano na
# criacao do jogo (Estados Unidos selecionados) e nas partidas da MLS com cartao.
# Convencoes conferidas na desmontagem:
# - 0x7e2c80 thiscall(linha do registro, partida) ret 8: como a liga (sx19_2), regras ff 1b ff ff, a cada segundo
#   amarelo (quantidade 2); o no pendente comeca amanha (copia de hoje e 0x54c5b0 add_days 1);
# - 0x7e40f0 thiscall(arquivo) ret 4: 0x7e7760, vtable 0x96ef14, +4 = 0, +5 = 0x1e, +6 = 0xa, +0x17 Estados Unidos
#   (0x9cf4f8), +0x1b America do Norte (0x9cfa14), +0x1f = 0x1e, +0x20/+0x21 = 0xff, +0x22 = 0;
# - 0x7e4140 thiscall(linha do registro, partida) ret 8: o vermelho (+0x2d == 1) da 1 jogo (motivo 2, quantidade 1);
#   cada amarelo k (k = 0: falta +0x2e; depois +0x2f, com sinal) soma em +8 da entrada (palavra) 1 ate o codigo 3, 2 ate
#   o 6 e 4 acima; com +8 >= 0x20: mais 1 jogo e +8 -= 0xb; senao, com +8 >= 0x15: mais 1 jogo (sem descontar); a
#   suspensao so por pontos tem motivo 0 e quantidade 0xb ou 0x15. Regras 1e ff ff ff, a partir de hoje; 0x556940 e
#   0x557280.
# Defeito do original: de 21 a 31 pontos os pontos nao descontam, e cada partida seguinte com cartao suspende de novo
# (ate passar de 32 ou a virada da temporada). Sem vermelho, a quantidade comeca com o endereco da linha (lixo), mas so
# e usada quando os pontos a regravam. Os bits de lixo das marcas (+0x13) como nos outros orgaos.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('rb_spain_cup_process_match_cards', 0x7e2c80, [0x6a, 0xff, 0x68, 0x31, 0x05, 0x96, 0x00]),
           ('rb_usa_construct', 0x7e40f0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_usa_process_match_cards', 0x7e4140, [0x6a, 0xff, 0x68, 0x88, 0x06, 0x96, 0x00])]
DOCS = ('as regras dos cartões da copa espanhola e o órgão disciplinar dos Estados Unidos',
        'the Spanish cup card rules and the United States ruling body',
        [('rb_spain_cup_process_match_cards', 'RbSpainCup::process_match_cards', '0x7e2c80', 'ruling_body/rb_spain_cup.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras da copa espanhola: o vermelho põe na lista do órgão uma suspensão por pontos a partir de amanhã (quando a falta dá uma) e dá 1 jogo; cada partida com amarelo soma 1 e a cada segundo dá mais 1 jogo; a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the Spanish cup rules: a red card queues on the body\'s list a points ban from tomorrow (when the offence brings one) and bans for 1 match; each match with a booking adds 1 and every second bans for 1 more match; from today, with its news'),
         ('rb_usa_construct', 'RbUsa::RbUsa', '0x7e40f0', 'ruling_body/rb_usa.cpp', 'thiscall, ret 4',
          'monta o órgão americano sobre a base: tabela virtual 0x96ef14, a vaga 0 do registro, a regra 0x1e, a escala 10, os Estados Unidos na América do Norte, a regra da liga 0x1e, sem copa nem confederação',
          'builds the US body on the base: vtable 0x96ef14, record slot 0, rule set 0x1e, scale 10, the United States in North America, league rule 0x1e, no cup nor confederation rule'),
         ('rb_usa_process_match_cards', 'RbUsa::process_match_cards', '0x7e4140', 'ruling_body/rb_usa.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras americanas: o vermelho dá 1 jogo; cada amarelo soma pontos pela falta (1 até o código 3, 2 até o 6, 4 acima); com 32 pontos ou mais, mais 1 jogo e menos 11 pontos; de 21 a 31, mais 1 jogo sem descontar; a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the US rules: a red card bans for 1 match; each booking adds points by its offence (1 up to code 3, 2 up to 6, 4 above); at 32 points or more, 1 more match and 11 points off; from 21 to 31, 1 more match without taking any off; from today, with its news')])
CAMPOS = [('entrada do órgão disciplinar +8: os pontos disciplinares (regra americana: 1, 2 ou 4 por amarelo; suspensão a partir de 21)',
           'ruling body entry +8: disciplinary points (US rule: 1, 2 or 4 per booking; a ban from 21)')]
INTERESSA = []
