# As rodadas da Challenge Cup escocesa e da Copa de Gales, e os inscritos da Copa da Liga escocesa (sco_chal_cup.cpp,
# wel_cup.cpp, sco_lge_cup.cpp; leitor cx03, rodada 5, 24/11/2026). Competicoes da Escocia e de Gales: LIGAS do grupo
# ENG SCO WAL, ate 2002-08-01 (os inscritos rodam 2 vezes no mundo ate essa data; as rodadas na criacao do jogo).
# - 0x7ed2e0 ScoChalCup::set_up_rounds thiscall() ret 0 -> 1: a virtual 0x8c da Challenge Cup e de duas outras copas
#   (vtables 0x96a360, 0x96cd30): +0x28 = 200.000 e a tabela pela virtual 0x3c em +0xa3 (set_up_prize_rounds, cx02_4).
# - 0x924e00 WelCup::set_up_rounds: o mesmo com 50.000. Prologo das duas: push esi; mov esi, ecx; push 0.
# - 0x7f0400 ScoLgeCup::collect_entrants thiscall() ret 0 -> -1/0/1 (chamada pelo dia da copa, sco_lge_cup_update_for_date,
#   e pela virada): +0x3e = 0x28 em 1999 e 0x2a depois; +0xb2 (byte com sinal) = 0; malloc 6 por time (a caixa da linha
#   0x191 e -1); as quatro divisoes (chaves 0x9cf5e8, 0x9cf5e4, 0x9cf5e0 e 0x9cf5dc), cada clube fora da Liga dos
#   Campeoes e da Copa da UEFA (+0x1db contra 0x9cf6e8 e 0x9cf6f0) com a classe +0x5f + 0x50/0x3c/0x28/0x14 (0x28 para
#   quem caiu da Premier: [[+0x5b]] = [0x9cf5dc], sem conferir o ponteiro); so a ultima passada para com a lista cheia
#   (as tres primeiras nao conferem o tamanho); depois os escoceses nas copas europeias (classe 0, +0xb2 + 1); qsort de
#   +0x3e - byes entradas (0x945333, cdecl, comparador 0x522190); com mais de 7 byes, a de menor reputacao da copa
#   europeia (+0x69 do registro de 0x6b bytes de [0xae23d0]) troca o clube com a primeira das byes e as byes descem
#   (a caixa da linha 0x213 sem candidata); lista incompleta: 0; senao byes acima de 12 viram 12 e 1.
# Usa set_up_prize_rounds e compare_set_up_prize_rounds do cx02_4 (integrar depois dele).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: as rodadas lado a lado (compare_set_up_prize_rounds); os inscritos lado a lado
# (eng_cc_cup_compare_entrant_list: +0x3e, +0xb2, o retorno e a lista; a lista da copia e liberada).
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('sco_chal_cup_set_up_rounds', 0x7ed2e0, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('wel_cup_set_up_rounds', 0x924e00, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('sco_lge_cup_collect_entrants', 0x7f0400, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('as rodadas da Challenge Cup escocesa e da Copa de Gales, e os inscritos da Copa da Liga escocesa',
        'the round set-up of the Scottish Challenge Cup and the Welsh Cup, and the Scottish League Cup entrants',
        [('sco_chal_cup_set_up_rounds', 'ScoChalCup::set_up_rounds', '0x7ed2e0', 'sco_chal_cup.cpp', 'thiscall, ret 0',
          'regra: a Challenge Cup escocesa e outras duas copas que dividem a virtual têm um prêmio total de 200.000; monta a tabela de rodadas',
          'rule: the Scottish Challenge Cup and two other cups that share the virtual have a 200,000 prize fund; builds the round table'),
         ('wel_cup_set_up_rounds', 'WelCup::set_up_rounds', '0x924e00', 'wel_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa de Gales tem um prêmio total de 50.000; monta a tabela de rodadas',
          'rule: the Welsh Cup has a 50,000 prize fund; builds the round table'),
         ('sco_lge_cup_collect_entrants', 'ScoLgeCup::collect_entrants', '0x7f0400', 'sco_lge_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Liga escocesa recebe 40 clubes em 1999 e 42 depois: as quatro divisões escocesas a partir da Terceira (a Terceira, a Segunda e a Primeira inteiras, a Premier até encher), depois os escoceses que jogam a Liga dos Campeões ou a Copa da UEFA, que entram isentos; com mais de 7 isentos, o da copa europeia de menor reputação perde a isenção, um de cada vez, até 7; no máximo 12 isentos. A classe do sorteio é a última posição na liga mais 0x50 na Terceira, 0x3c na Segunda, 0x28 na Primeira (só 0x28 para quem caiu da Premier) e 0x14 na Premier; os europeus 0',
          'rule: the Scottish League Cup takes 40 clubs in 1999 and 42 after: the four Scottish divisions from the Third up (the Third, Second and First whole, the Premier until the list is full), then the Scottish clubs in the Champions League or the UEFA Cup, who get a bye; with more than 7 of those, the one whose European cup has the lowest reputation loses the bye, one at a time, down to 7; at most 12 byes. The draw class is the last league position plus 0x50 in the Third, 0x3c in the Second, 0x28 in the First (0x28 alone for a club relegated from the Premier) and 0x14 in the Premier; the European clubs 0')])
CAMPOS = [('copa da liga escocesa +0xb2 byte com sinal: os isentos (clubes nas copas europeias)', 'Scottish League Cup +0xb2 signed byte: the byes (clubs in the European cups)')]
INTERESSA = []
