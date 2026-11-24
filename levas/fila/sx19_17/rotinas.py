# As entradas disciplinares de um orgao: achar ou acrescentar a de um jogador, ligar e desligar a suspensao dele
# (ruling_body.cpp; leitor sx19, rodada 5, 24/11/2026). Rodam muito na carreira ENG (mundo ate 2002-08-01:
# find_or_add 2.669.319 vezes, attach_ban 5.896, detach_ban 5.504): cada cartao, cada suspensao dada ou solta, cada
# pergunta "suspenso?". Sem dependencia dos outros itens (table_penalty e do discipline.rs).
# Convencoes conferidas na desmontagem:
# - 0x7e82e0 thiscall(id, criar) ret 8: 0x55cae0 table_penalty thiscall(id, vaga +4, regra +5) ret 0xc (o indice da
#   entrada no registro do jogador, -1 sem); sem indice e sem criar: 0; com criar: 0x55cb60 find_held_ban thiscall(id,
#   vaga) ret 8, com a regra (+5 do item) igual a do orgao: caixa de erro da linha 0x295 (e segue); cheio (+0x13 ==
#   +0xf): +0xf += 100 e 0x945667 realloc(+0xb, cap * 14) ou 0x944e46 malloc(cap * 14) (cdecl); falhando, caixa da
#   linha 0x2b0 e 0; a entrada nova (14 bytes) zerada, +6 = -1, +0 = id, +0x13 += 1 e 0x55cb20 set_slot thiscall(id,
#   vaga, regra, indice) ret 0x10. Indice fora de [0, +0xf): set_slot(id, vaga, regra, -1) e 0;
# - 0x7e8510 thiscall(id, suspensao (palavra)) ret 8: a entrada (0x7e82e0 com 1); com suspensao antiga (+6 != -1):
#   dias da antiga (+2, com sinal) > 0 somam nos da nova; senao jogos (+0, com sinal) > 0 somam nos da nova; 0x556d00
#   release_ban thiscall(id, antiga) ret 8 (a palavra com lixo em cima: a rotina le so a palavra); +6 = nova;
# - 0x7e8590 thiscall(id) ret 4: a entrada (0x7e82e0 com 0); existindo, +6 = -1.
# Comparadores: a procura roda as duas e compara o endereco; acrescentar e ligar/desligar so a original, conferida
# depois (a entrada nova, a suspensao ligada com o que sobrou da antiga, a entrada sem suspensao).
SAVE = False
ROTINAS = [('find_or_add_discipline_entry', 0x7e82e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('attach_ban', 0x7e8510, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('detach_ban', 0x7e8590, [0x8b, 0x44, 0x24, 0x04, 0x6a, 0x00])]
DOCS = ('as entradas disciplinares de um órgão: achar ou acrescentar, ligar e desligar a suspensão',
        'a ruling body\'s disciplinary entries: finding or adding, attaching and detaching the ban',
        [('find_or_add_discipline_entry', 'RulingBody::find_or_add_discipline_entry', '0x7e82e0', 'ruling_body/ruling_body.cpp', 'thiscall, ret 8',
          'devolve a entrada disciplinar do jogador neste órgão (o índice vem do registro disciplinar dele, pela vaga e pela regra do órgão); sem entrada e com "criar", acrescenta uma de 14 bytes zerada, com o id e sem suspensão, crescendo o vetor de 100 em 100, e grava o índice no registro; índice fora do vetor: limpa a vaga e devolve 0',
          'returns the player\'s disciplinary entry in this body (the index comes from his discipline record, by the body\'s slot and rule set); without one and with "create", adds a zeroed 14-byte entry with his id and no ban, growing the array by 100, and writes the index into the record; an index outside the array: clears the slot and returns 0'),
         ('attach_ban', 'RulingBody::attach_ban', '0x7e8510', 'ruling_body/ruling_body.cpp', 'thiscall, ret 8',
          'liga uma suspensão nova à entrada do jogador; se já havia uma, o que sobrou dela (os dias, ou senão os jogos) passa para a nova e a antiga é solta',
          'links a new ban to the player\'s entry; when there was one, what is left of it (its days, otherwise its matches) goes onto the new one and the old one is released'),
         ('detach_ban', 'RulingBody::detach_ban', '0x7e8590', 'ruling_body/ruling_body.cpp', 'thiscall, ret 4',
          'tira a suspensão da entrada do jogador (-1), quando ele tem entrada; chamada quando a suspensão é solta',
          'unlinks the ban from the player\'s entry (-1), when he has one; called when the ban is released')])
CAMPOS = []
INTERESSA = []
