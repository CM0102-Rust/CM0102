# A virada da temporada do orgao da liga escocesa e o orgao da copa escocesa (construtor e regras dos cartoes)
# (rb_scotland_league.cpp, rb_scotland_cup.cpp; leitor sx19, rodada 5, 24/11/2026). Usa as pecas do sx18_1, do sx19_1
# (own_rules) e do sx19_11 (SCOTLAND_LEAGUE_RULING_BODY, SCOTLAND_PAST_BANS, SCOTLAND_SEASON_BANS): integrar depois
# deles. ruling_body_of_rule e do discipline.rs.
# So rodam com a Escocia carregada: o construtor na criacao do jogo, as regras nas partidas da copa escocesa com
# cartao; a virada quando a liga escocesa vira a temporada (junho/julho de 2002: ATE 2002-08-01).
# Convencoes conferidas na desmontagem:
# - 0x7e2980 thiscall() ret 0: para cada entrada (+0xb, 14 bytes, contagem +0x13 com sinal): +0xa = (+0xa >> 3 & 7) |
#   ((+0xb & 7) << 3) | (+0xa & 0xc0); +0xb &= 0xf8; +8 (palavra) = 0. Nao libera entradas nem zera os amarelos;
# - 0x7e21a0 thiscall(arquivo) ret 4: 0x7e7760, vtable 0x96edf4, +4 = 1, +5 = 0x19, +6 = 5, +0x17 Escocia, +0x1b
#   Europa, +0x1f = 0x18, +0x20 = 0x19, +0x21 = 1, +0x22 = 0;
# - 0x7e21f0 thiscall(linha do registro, partida) ret 8: vermelho (+0x2d != 0): 1 jogo (motivo 2, quantidade 1); a
#   entrada soma todos os amarelos; com mais de 1: jogos += contagem / 2, motivo 1, quantidade = a contagem (movzx),
#   a entrada fica com o resto; vermelho == 1 com menos de 2 amarelos: na liga escocesa ([[0xae28f0] + 0x60]) 0x7e82e0
#   (id, 1), +8 += 12 e a vtable +0xc (check_points_ban, thiscall(id, partida) ret 8); se ela suspendeu, a copa nao
#   suspende. Regras ff 19 ff ff, a partir de hoje; 0x556940 e 0x557280.
# Defeito do original: sem vermelho, a quantidade comeca com o endereco da partida (lixo), mas so e usada quando os
# amarelos a regravam; os bits de lixo das marcas, como nos outros orgaos.
SAVE = False
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
ROTINAS = [('rb_scotland_league_start_new_season', 0x7e2980, [0x51, 0x56, 0x57, 0x8b, 0xf9]),
           ('rb_scotland_cup_construct', 0x7e21a0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('rb_scotland_cup_process_match_cards', 0x7e21f0, [0x6a, 0xff, 0x68, 0x88, 0x04, 0x96, 0x00])]
DOCS = ('a virada da temporada do órgão escocês e o órgão da copa escocesa',
        'the Scottish league body\'s new season and the Scottish cup ruling body',
        [('rb_scotland_league_start_new_season', 'RbScotlandLeague::start_new_season', '0x7e2980', 'ruling_body/rb_scotland_league.cpp', 'thiscall',
          'na virada da temporada, em cada entrada do órgão escocês, as suspensões por pontos recuam uma temporada (a anterior vira a de duas atrás, a desta vira a anterior), a desta e os pontos vão a zero; diferente dos outros órgãos, não libera entradas nem zera os cartões',
          'at the new season, in each entry of the Scottish body the points bans move back a season (last season\'s become two seasons ago, this season\'s become last season\'s), this season\'s and the points go to zero; unlike the other bodies, no entry is freed and the cards stay'),
         ('rb_scotland_cup_construct', 'RbScotlandCup::RbScotlandCup', '0x7e21a0', 'ruling_body/rb_scotland_cup.cpp', 'thiscall, ret 4',
          'monta o órgão da copa escocesa sobre a base: tabela virtual 0x96edf4, a vaga 1 do registro, a regra 0x19, a escala 5, a Escócia na Europa, as regras da liga (0x18), da copa (0x19) e da confederação (1)',
          'builds the Scottish cup body on the base: vtable 0x96edf4, record slot 1, rule set 0x19, scale 5, Scotland in Europe, the league (0x18), cup (0x19) and confederation (1) rule sets'),
         ('rb_scotland_cup_process_match_cards', 'RbScotlandCup::process_match_cards', '0x7e21f0', 'ruling_body/rb_scotland_cup.cpp', 'thiscall, ret 8',
          'aplica os cartões de um jogador numa partida pelas regras da copa escocesa: o vermelho dá 1 jogo; os amarelos somam e cada par dá 1 jogo (fica o ímpar); o vermelho direto também dá 12 pontos na liga escocesa e, se ela suspender por pontos, a copa não suspende; a partir de hoje, com a notícia',
          'applies a player\'s cards of a match under the Scottish cup rules: a red card bans for 1 match; bookings add up and each pair bans for 1 match (the odd one stays); a straight red also adds 12 points in the Scottish league and, when the league hands out a points ban, the cup hands out none; from today, with its news')])
CAMPOS = []
INTERESSA = []
