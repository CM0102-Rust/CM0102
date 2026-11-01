# As suspensões internacionais cumpridas no jogo da seleção, o recurso da suspensão e o texto da vaga da suspensão
# (discipline.cpp; a família de mídia, notícias e técnicos não tem mais rotina de simulação deste tamanho: as que
# restam têm de 2 a 11 KB, ver o relatório; discipline.cpp não é de nenhum outro agente nesta rodada).
# - 0x557120 (Discipline::serve_international_bans, 160 chamadas ate 2002-06-01 na cobertura de 17/10, pelo jogo da
#   seleção 0x7e8090): as pessoas da nação no índice das nações vão para uma LLIST local e cada uma cumpre um jogo da
#   suspensão na regra 0 (0x556e90, já em Rust). As mutações estão nela.
# - 0x557000 (Discipline::apply_appeal_result, pelo resultado do recurso 0x6abf99, das notícias do técnico humano):
#   passageira (a carreira automática não tem humano).
# - 0x55d3c0 (Discipline::get_slot_text, pela tela da pessoa 0x87618a): passageira.
# Convenções conferidas nos pontos de chamada da original (lição da 258a):
#   0x557120: thiscall (mov ecx, [0xae28f0] em 0x7e8090), 1 na pilha (ret 4). Dentro: 0x68d020, 0x68d040, 0x68d1b0
#   thiscall (lea ecx, a lista), 0; 0x946464 bsearch cdecl (add esp, 0x14), 5 (chave, base, conta com sinal, 8,
#   0x55e5f0); 0x68d0d0 thiscall (lea ecx, a lista), 2 (o item, 4), ret 8; 0x556e90 thiscall (mov ecx, ebx = this), 3
#   (id, 0, 1), ret 0xc.
#   0x557000: thiscall (mov ecx, [0xae28f0] em 0x6abf99), 8 na pilha (ret 0x20; o 8º não é lido). Dentro: 0x7e82e0
#   thiscall (mov ecx, o órgão [[this] + 4 × regra]), 2 (id, 0); 0x7e91d0 thiscall (o mesmo órgão), 1 (id); 0x556d00
#   thiscall (mov ecx, esi = this), 2 (id, o registrador com o índice na palavra baixa; o alto é sobra e a 0x556d00 só
#   lê a palavra).
#   0x55d3c0: thiscall (mov ecx, [0xae28f0] em 0x87618a), 4 (pessoa, vaga, saída, 0xc8), ret 0x10, lê eax. Dentro:
#   0x7e88d0 thiscall (mov ecx, o órgão da regra), 3 (pessoa, saída, tamanho), ret 0xc conferido.
# Comparadores: 0x557120 só a original com o plano completo (as pessoas e, de cada uma, o contador novo ou a
# suspensão solta, pelo cpm4_jogo da 0x556e90: a contagem grava o histórico no arquivo); 0x557000 só a original com o
# plano (os jogos e os dias novos ou a suspensão solta; a 0x7e91d0 solta nós da lista de pendentes); 0x55d3c0 pela
# devolução sem regra na vaga, só a original com regra (o texto do órgão pode contar a suspensão por dias, 0x556e90).
# Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('cumprir_suspensoes_internacionais', 0x557120, [0x6a, 0xff, 0x68, 0xc8, 0x7c, 0x95, 0x00]),
           ('recurso_da_suspensao', 0x557000, [0x53, 0x55, 0x8b, 0x5c, 0x24, 0x0c]),
           ('texto_da_vaga_da_suspensao', 0x55d3c0, [0x8b, 0x54, 0x24, 0x04, 0x56])]
DOCS = ('as suspensões internacionais cumpridas no jogo da seleção, o recurso da suspensão e o texto da vaga da suspensão',
        'the international bans served in a national team match, the ban appeal and the ban slot text',
        [('cumprir_suspensoes_internacionais', 'Discipline::serve_international_bans', '0x557120', 'discipline.cpp', 'thiscall',
          'no jogo da seleção: as pessoas da nação dela no índice das nações (busca binária pela nação) cumprem um jogo da suspensão na regra 0, uma a uma a partir de uma lista com os ids (a contagem pode soltar a suspensão e reordenar o índice)',
          'in a national team match: the people of its nation in the nation index (binary search by nation) serve one match of their ban under rule 0, one by one from a list of the ids (the count can release the ban and reorder the index)'),
         ('recurso_da_suspensao', 'Discipline::apply_appeal_result', '0x557000', 'discipline.cpp', 'thiscall',
          'o resultado do recurso contra a suspensão: o órgão tira as suspensões pendentes da pessoa; a suspensão por jogos perde jogos (ou fica no contador) e a por dias perde dias, e é solta quando acaba ou quando o recurso a anula',
          'the result of a ban appeal: the ruling body drops the person\'s pending bans; a match ban loses matches (or stops at the counter) and a timed ban loses days, and it is released when over or when the appeal lifts it'),
         ('texto_da_vaga_da_suspensao', 'Discipline::get_slot_text', '0x55d3c0', 'discipline.cpp', 'thiscall',
          'o texto da suspensão da vaga da pessoa, pelo órgão da regra dela; sem regra na vaga, 0',
          'the text of the ban in the person\'s slot, from the ruling body of its rule; with no rule in the slot, 0')])
CAMPOS = [('disciplina +0xc índice das nações (8 bytes: +0 id da pessoa, +4 id da nação), +0x12 conta; ponteiro/valor; lê (cumprir_suspensoes_internacionais)',
           'discipline +0xc nation index (8 bytes: +0 person id, +4 nation id), +0x12 count; pointer/value; reads (cumprir_suspensoes_internacionais)'),
          ('disciplina +4 suspensões (20 bytes: +0 jogos, +1 contador, +2 dias); valor; lê e grava (recurso_da_suspensao)',
           'discipline +4 bans (20 bytes: +0 matches, +1 counter, +2 days); value; reads and writes (recurso_da_suspensao)'),
          ('disciplina +0x14 registro de 29 bytes por pessoa (5 bytes por vaga: a regra); valor; lê (texto_da_vaga_da_suspensao)',
           'discipline +0x14 29-byte record per person (5 bytes per slot: the rule); value; reads (texto_da_vaga_da_suspensao)'),
          ('órgão: entrada de 14 bytes da pessoa (+6 índice da suspensão); valor; lê (0x7e82e0)', 'ruling body: 14-byte person entry (+6 ban index); value; reads (0x7e82e0)'),
          ('clube (seleção) +0x53 nação; ponteiro; lê', 'club (national team) +0x53 nation; pointer; reads')]
INTERESSA = ['CMRTE', 'SGE']
