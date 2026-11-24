# O dia de um orgao disciplinar: as suspensoes pendentes cuja data chegou (ruling_body.cpp; leitor sx19, rodada 5,
# 24/11/2026). Usa FN_POST_BAN_NEWS do sx18_1: integrar depois dele. Roda na carreira ENG todo dia, em cada orgao
# (vtable +4; 24.811 vezes no mundo ate 2002-08-01); as suspensoes pendentes sao as por pontos dos vermelhos (ingles,
# italiano, ... via make_points_ban), que comecam dias depois.
# Convencoes conferidas na desmontagem: 0x7e7d50 thiscall(&data) ret 4: a lista (+7; nula: nada), cursor no inicio
# (+0xa = +2), 0x68d1b0 get_next; para cada no ate o primeiro que comeca depois da data (no +0x16 ano sem sinal, +0x14
# dia com sinal; para tudo, nao so pula): 0x417c10 operator=(no+4, no+0x14); 0x7e82e0(id +0x1c, 0); sem entrada:
# 0x68d1f0 remove(no) e cursor no inicio; com entrada: 0x556e90 update_ban_progress(id, regra +5, 0); sem suspensao
# (+6 == -1): marcas do no &= 0xfe, no+0x6f = 1, 0x556940 add_ban(id, no); com: marcas do no |= 1 e, na suspensao, jogos
# (+0 != 0xff: 0xff vira os do no, senao soma) ou dias (-1 vira os do no, senao soma) e marcas |= 4; 0x557280
# post_ban_news(pessoa, no, no+0x20); 0x68d1f0 remove(no) e cursor no inicio.
# Comparador: so a original (as suspensoes e as noticias nao se repetem), e depois a lista perdeu so os nos do dia.
SAVE = False
ROTINAS = [('ruling_body_daily_update', 0x7e7d50, [0x83, 0xec, 0x10, 0x53, 0x55])]
DOCS = ('o dia de um órgão disciplinar: as suspensões pendentes que chegaram',
        'a ruling body\'s day: the pending bans whose date has come',
        [('ruling_body_daily_update', 'RulingBody::daily_update', '0x7e7d50', 'ruling_body/ruling_body.cpp', 'thiscall, ret 4',
          'aplica as suspensões pendentes do órgão cuja data chegou, em ordem, até a primeira que começa depois: sem entrada do jogador, a pendente só sai; sem suspensão, ela vira uma suspensão nova; com uma, os jogos (ou dias) dela somam na que existe; a notícia sai e a pendente deixa a lista',
          'applies the body\'s pending bans whose date has come, in order, until the first that starts later: without the player\'s entry the pending ban just goes; without a ban, it becomes a new one; with one, its matches (or days) add to the existing ban; the news goes out and the pending ban leaves the list')])
CAMPOS = [('suspensão pendente (nó de 0x70 bytes) +0x14: a data de início; +0x1c o id da pessoa; +0x20 a partida (0x4f bytes); +0x6f marcado 1 quando vira suspensão nova',
           'pending ban (0x70-byte node) +0x14: the start date; +0x1c the person id; +0x20 the fixture (0x4f bytes); +0x6f set to 1 when it becomes a new ban')]
INTERESSA = []
