# O orgao disciplinar da liga norueguesa (construtor e regras dos cartoes) e o construtor do portugues
# (rb_norway_league.cpp, rb_portugal.cpp; leitor sx19, rodada 5, 24/11/2026). Usa as pecas do sx18_1 e do sx19_1
# (own_rules): integrar depois deles.
# So rodam com as ligas carregadas: os construtores na criacao do jogo (Noruega e Portugal selecionados), as regras
# norueguesas nas partidas da Tippeligaen com cartao (temporada de abril a outubro de 2002: ATE 2002-08-01).
# Convencoes conferidas na desmontagem:
# - 0x7e1150 / 0x7e1a60 thiscall(arquivo) ret 4: 0x7e7760 (base), depois os campos: Noruega vtable 0x96ed54, +4 = 0,
#   +5 = 0x15, +6 = 0xa, +0x17 Noruega (0x9cf408), +0x1f = 0x15, +0x20 = 0x16; Portugal 0x96edb4, +4 = 0, +5 = 0x17,
#   +6 = 8, +0x17 Portugal (0x9cf434), +0x1f = 0x17, +0x20 = 0xff; os dois +0x1b Europa, +0x21 = 1, +0x22 = 0;
# - 0x7e11a0 thiscall(linha do registro, partida) ret 8: o vermelho (+0x2d == 1) da 1 jogo (motivo 2, quantidade 1) e
#   pula os amarelos; senao, com amarelo, +1 na entrada: 3 -> quantidade 3; 4 ou menos (fora o 3) -> nada; 5 ->
#   quantidade 2 (sete bl, inc ebx); acima -> 1; sempre 1 jogo, motivo 1, a partir de hoje, regras 15 ff ff ff;
#   0x556940 add_ban e 0x557280 post_ban_news.
# Defeito do original: os bits de lixo das marcas (+0x13), como nos outros orgaos.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('rb_norway_league_construct', 0x7e1150, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_norway_league_process_match_cards', 0x7e11a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('rb_portugal_construct', 0x7e1a60, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('o órgão disciplinar da liga norueguesa e o construtor do português',
        'the Norwegian league ruling body and the Portuguese body\'s constructor',
        [('rb_norway_league_construct', 'RbNorwayLeague::RbNorwayLeague', '0x7e1150', 'ruling_body/rb_norway_league.cpp', 'thiscall, ret 4',
          'monta o órgão da liga norueguesa sobre a base: tabela virtual 0x96ed54, a vaga 0 do registro, a regra 0x15, a escala 10, a Noruega na Europa, as regras da liga (0x15), da copa (0x16) e da confederação (1)',
          'builds the Norwegian league body on the base: vtable 0x96ed54, record slot 0, rule set 0x15, scale 10, Norway in Europe, the league (0x15), cup (0x16) and confederation (1) rule sets'),
         ('rb_norway_league_process_match_cards', 'RbNorwayLeague::process_match_cards', '0x7e11a0', 'ruling_body/rb_norway_league.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras da liga norueguesa: o vermelho dá 1 jogo (e os amarelos da partida não contam); senão cada partida com amarelo soma 1 e o terceiro, o quinto e cada um depois dele dão 1 jogo (o quarto não); a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the Norwegian league rules: a red card bans for 1 match (and the match\'s bookings do not count); otherwise each match with a booking adds 1 and the third, the fifth and each one after it ban for 1 match (the fourth does not); from today, with its news'),
         ('rb_portugal_construct', 'RbPortugal::RbPortugal', '0x7e1a60', 'ruling_body/rb_portugal.cpp', 'thiscall, ret 4',
          'monta o órgão português sobre a base: tabela virtual 0x96edb4, a vaga 0 do registro, a regra 0x17, a escala 8, Portugal na Europa, a regra da liga 0x17, sem copa, a da confederação 1',
          'builds the Portuguese body on the base: vtable 0x96edb4, record slot 0, rule set 0x17, scale 8, Portugal in Europe, league rule 0x17, no cup rule, confederation rule 1')])
CAMPOS = []
INTERESSA = []
