# Tirar de vetores ordenados, 4º item: os interesses de transferência da pessoa na mídia (Media::remove_transfer_interest
# 0x7549b0, 2.344 chamadas na carreira até 2002-06-01; media.cpp, chamada pela 0x4ce4e0), o pedido de observação de
# um clube (SCOUT_MANAGER::remove_scout_request 0x7f8ee0, 750; scout_manager.cpp) e o apelido da pessoa em todos os
# técnicos humanos (HUMAN_MANAGER::remove_nickname_from_all 0x5fff30, 248; human_manager.cpp). Pelo qsort 0x945333 e
# pelo bsearch 0x946464 da CRT com as comparações do jogo (0x757ea0, 0x7fc270, 0x601a10), a data pela 0x417c10 e a
# padrão pela 0x54a110. Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real comparando.
# Sem SEH, sem caixa, sem sorteio. Locais lidas sem valor: as chaves do bsearch têm bytes sem valor (a 0x7f8ee0 o
# +0x10 de 17; a 0x5fff30 os 25 depois do +4 de 30), mas as comparações leem só os 8 (0x7fc270) e os 4 (0x601a10)
# primeiros: a cópia zera. Quirk do original na 0x5fff30: reordena só o vetor do último técnico visto (e, sem técnicos,
# pelo próprio `id`). Comparadores lado a lado pelos vetores e contagens (a 0x5fff30 só a original sem técnicos).
SAVE = False
ROTINAS = [('tirar_interesses_de_transferencia', 0x7549b0, [0x51, 0x8b, 0xc1, 0x33, 0xd2]),
           ('tirar_pedido_de_observacao', 0x7f8ee0, [0x83, 0xec, 0x1c, 0x53, 0x8b, 0xd9]),
           ('tirar_apelido_de_todos', 0x5fff30, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x24])]
DOCS = ('tirar de vetores ordenados: os interesses de transferência da mídia, o pedido de observação e o apelido em todos os técnicos humanos',
        'removing from sorted arrays: the media transfer interests, the scout request and the nickname from every human manager',
        [('tirar_interesses_de_transferencia', 'Media::remove_transfer_interest', '0x7549b0', 'media.cpp', 'thiscall, ret 4',
          'tira os interesses da pessoa (25 bytes) pondo o último no lugar; reordena (0x757ea0)',
          'removes the person interests (25 bytes) moving the last one into the gap; sorts again (0x757ea0)'),
         ('tirar_pedido_de_observacao', 'SCOUT_MANAGER::remove_scout_request', '0x7f8ee0', 'scout_manager.cpp', 'thiscall, ret 8',
          'busca o pedido (clube, pessoa; 17 bytes), puxa os seguintes um lugar e reordena (0x7fc270); clube nulo = o do técnico humano atual',
          'finds the request (club, person; 17 bytes), pulls the next ones up and sorts again (0x7fc270); null club = the current human manager club'),
         ('tirar_apelido_de_todos', 'HUMAN_MANAGER::remove_nickname_from_all', '0x5fff30', 'human_manager.cpp', 'thiscall, ret 4',
          'em cada técnico humano, busca o apelido da pessoa (30 bytes), puxa os seguintes, apaga o último e desce a contagem; reordena o vetor do último técnico',
          'in each human manager, finds the person nickname (30 bytes), pulls the next ones up, blanks the last one and lowers the count; sorts the last manager array')])
CAMPOS = [('Media +0xc/+0x14 interesses de transferência (25 bytes, +0 pessoa); SCOUT_MANAGER +0x5c/+0x64 pedidos (17 bytes: clube, pessoa, data +8, +0x10); técnico humano (0xa6ef bytes) +0xa6e3/+0xa6eb apelidos (30 bytes: +0 id, +4); escreve (tirar_interesses_de_transferencia, tirar_pedido_de_observacao, tirar_apelido_de_todos)',
           'Media +0xc/+0x14 transfer interests (25 bytes, +0 person); SCOUT_MANAGER +0x5c/+0x64 requests (17 bytes: club, person, date +8, +0x10); human manager (0xa6ef bytes) +0xa6e3/+0xa6eb nicknames (30 bytes: +0 id, +4); writes (tirar_interesses_de_transferencia, tirar_pedido_de_observacao, tirar_apelido_de_todos)')]
INTERESSA = ['SGE', 'CMRTE']
