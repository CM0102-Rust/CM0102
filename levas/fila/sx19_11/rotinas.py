# O orgao disciplinar da liga escocesa: o construtor, as regras dos cartoes (pontos disciplinares) e a suspensao por
# pontos (rb_scotland_league.cpp; leitor sx19, rodada 5, 24/11/2026). Usa as pecas do sx18_1 e do sx19_1 (own_rules):
# integrar depois deles. O sx19_12 usa SCOTLAND_LEAGUE_RULING_BODY, SCOTLAND_PAST_BANS e SCOTLAND_SEASON_BANS deste.
# So rodam com a Escocia carregada: o construtor na criacao do jogo, as regras nas partidas da liga escocesa com
# cartao, e a suspensao por pontos pelo vermelho da copa escocesa (vtable +0xc, so a liga escocesa a tem; os outros
# orgaos apontam 0x5839d0).
# Convencoes conferidas na desmontagem:
# - 0x7e2470 thiscall(arquivo) ret 4: 0x7e7760, vtable 0x96ee14, +4 = 0, +5 = 0x18, +6 = 5, +0x17 Escocia (0x9cf45c),
#   +0x1b Europa, +0x1f = 0x18, +0x20 = 0x19, +0x21 = 1, +0x22 = 0;
# - 0x7e24c0 thiscall(linha do registro, partida) ret 8: vermelho (+0x2d == 1): 1 jogo (motivo 2, quantidade 1) e a
#   falta do vermelho (+0x2e sem amarelo, +0x2f com um, -1 com dois) soma em +8 (palavra): 9 -> 5, < 12 -> 12, senao 32;
#   cada amarelo k (k = 0 +0x2e, depois +0x2f, com sinal) soma <= 3 -> 2, <= 8 -> 3, 9 -> 5, 10 -> 12, senao 32. Com
#   +8 >= 16 (sem sinal), por vez: o contador da temporada (+0xb, bits 0..2) sobe ate 7, +8 -= 16, os jogos somam
#   esse contador e os das duas temporadas passadas (+0xa bits 0..2 e 3..5, lidos antes) e a quantidade 16; motivo 0.
#   Inicio: hoje com vermelho (+0x2d != 0), senao 14 dias (0x54c5b0 add_days 0xe); regras 18 ff ff ff; 0x556940 e
#   0x557280;
# - 0x7e2720 thiscall(id, partida) ret 8, devolve 1/0: 0x7e82e0(id, 1); a mesma conta dos 16 pontos; com jogos, a
#   suspensao de hoje, motivo 0, regras 18 19 ff ff, marcas com o bit 2 ligado (or cl, 4), 0x556940 add_ban(id) e
#   0x557280 post_ban_news(pessoa, suspensao, partida); devolve 1.
# Defeito do original: os bits de lixo das marcas (+0x13), como nos outros orgaos (na suspensao por pontos o bit 2 vem
# sempre ligado).
SAVE = False
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
ROTINAS = [('rb_scotland_league_construct', 0x7e2470, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_scotland_league_process_match_cards', 0x7e24c0, [0x6a, 0xff, 0x68, 0xb0, 0x04, 0x96, 0x00]),
           ('rb_scotland_league_check_points_ban', 0x7e2720, [0x6a, 0xff, 0x68, 0xc8, 0x04, 0x96, 0x00])]
DOCS = ('o órgão disciplinar da liga escocesa: construtor, pontos e suspensão por pontos',
        'the Scottish league ruling body: constructor, points and the points ban',
        [('rb_scotland_league_construct', 'RbScotlandLeague::RbScotlandLeague', '0x7e2470', 'ruling_body/rb_scotland_league.cpp', 'thiscall, ret 4',
          'monta o órgão da liga escocesa sobre a base: tabela virtual 0x96ee14, a vaga 0 do registro, a regra 0x18, a escala 5, a Escócia na Europa, as regras da liga (0x18), da copa (0x19) e da confederação (1)',
          'builds the Scottish league body on the base: vtable 0x96ee14, record slot 0, rule set 0x18, scale 5, Scotland in Europe, the league (0x18), cup (0x19) and confederation (1) rule sets'),
         ('rb_scotland_league_process_match_cards', 'RbScotlandLeague::process_match_cards', '0x7e24c0', 'ruling_body/rb_scotland_league.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras escocesas: o vermelho dá 1 jogo e pontos pela falta (5, 12 ou 32); cada amarelo dá pontos pela falta (2, 3, 5, 12 ou 32); cada 16 pontos viram uma suspensão de tantos jogos quantas as suspensões por pontos desta temporada e das duas anteriores; começa hoje com vermelho, senão em 14 dias, com a notícia',
          'applies a player\'s cards of a match under the Scottish rules: a red card bans for 1 match and adds its offence\'s points (5, 12 or 32); each booking adds its offence\'s points (2, 3, 5, 12 or 32); each 16 points become a ban of as many matches as the points bans of this season and the two before; it starts today after a red card, otherwise in 14 days, with its news'),
         ('rb_scotland_league_check_points_ban', 'RbScotlandLeague::check_points_ban', '0x7e2720', 'ruling_body/rb_scotland_league.cpp', 'thiscall, ret 8',
          'transforma cada 16 pontos disciplinares do jogador na liga escocesa numa suspensão (a mesma conta das regras), a partir de hoje, nas regras da liga e da copa, com a notícia, e devolve 1; abaixo de 16 pontos, 0; chamada pelo vermelho da copa escocesa',
          'turns each 16 disciplinary points of the player in the Scottish league into a ban (the same count as the card rules), from today, under the league and cup rule sets, with its news, and returns 1; under 16 points, 0; called by the Scottish cup\'s red card')])
CAMPOS = [('entrada do órgão disciplinar escocês +0xa: as suspensões por pontos de duas temporadas atrás (bits 0..2) e da anterior (bits 3..5)',
           'Scottish ruling body entry +0xa: the points bans of two seasons ago (bits 0..2) and of last season (bits 3..5)'),
          ('entrada do órgão disciplinar escocês +0xb: as suspensões por pontos desta temporada (bits 0..2, até 7)',
           'Scottish ruling body entry +0xb: this season\'s points bans (bits 0..2, up to 7)')]
INTERESSA = []
