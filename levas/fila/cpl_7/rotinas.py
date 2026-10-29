# Mídia: os interesses de transferência ([0xb6b7c8] Media +0xc, 25 bytes por interesse): acrescentar um interesse
# (Media::add_transfer_interest 0x754740, pela busca de jogadores 0x7b6e86, pela shortlist 0x84079d/0x840c4b e pelas
# transferências 0x8f4c07; aloca e realoca; bsearch 0x946464 e qsort 0x945333 com 0x757ea0; data pela 0x417c10, _ftol
# 0x945640) e as duas comparações do media.cpp: a das chaves de ids (compare_id_pair_keys 0x757ea0, dos interesses,
# da shortlist 0x842080 e das buscas já em Rust) e a das histórias (compare_story_log_records 0x757ee0, do registro de
# histórias). Simulação: as mutações pela carreira (os boatos da 0x754a30 sorteiam por interesse) e a confirmação no jogo
# real comparando. Sem SEH; caixa só sem memória. As comparações são chamadas muitas vezes pelo qsort e pelo bsearch:
# sem contador nem mutação nelas. Comparadores: o interesse pelo plano (a original aloca), as comparações pelo valor.
SAVE = False
ROTINAS = [('acrescentar_interesse_de_transferencia', 0x754740, [0x81, 0xec, 0x24, 0x02, 0x00, 0x00]),
           ('ordem_das_chaves_de_ids', 0x757ea0, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('ordem_das_historias', 0x757ee0, [0x8b, 0x54, 0x24, 0x04, 0x56])]
DOCS = ('os interesses de transferência da mídia: acrescentar um interesse e as comparações das chaves de ids e das histórias',
        'the media transfer interests: adding an interest and the comparisons of id keys and stories',
        [('acrescentar_interesse_de_transferencia', 'Media::add_transfer_interest', '0x754740', 'media.cpp', 'thiscall, ret 0x14',
          'se a chave {jogador, clube} não está, acrescenta o interesse (data de hoje, valor até 50 milhões, avaliação × 100) e reordena; cresce de 20 em 20',
          'if the {player, club} key is missing, appends the interest (today, fee up to 50 million, valuation × 100) and sorts again; grows by 20'),
         ('ordem_das_chaves_de_ids', 'compare_id_pair_keys', '0x757ea0', 'media.cpp', 'cdecl',
          'nulo antes; pelo id +0 e, igual, pelo +4 (−1 casa com qualquer um)',
          'null first; by the +0 id and, when equal, by +4 (−1 matches anything)'),
         ('ordem_das_historias', 'compare_story_log_records', '0x757ee0', 'media.cpp', 'cdecl',
          'nulo antes; pelos sujeitos +8 e +0xc, o tipo +0x13 e o subtipo +0x11',
          'null first; by subjects +8 and +0xc, type +0x13 and subtype +0x11')])
CAMPOS = [('Media +0xc interesses de transferência (25 bytes: +0 id do jogador, +4 id do clube, +8 data, +0x10 boato saiu, +0x11 valor até 50.000.000, +0x15 avaliação × 100), +0x10 capacidade, +0x14 conta; grava (acrescentar_interesse_de_transferencia)',
           'Media +0xc transfer interests (25 bytes: +0 player id, +4 club id, +8 date, +0x10 rumour out, +0x11 fee up to 50,000,000, +0x15 valuation × 100), +0x10 capacity, +0x14 count; writes (acrescentar_interesse_de_transferencia)'),
          ('chave de ids (+0 id, +4 id ou −1 coringa); lê (ordem_das_chaves_de_ids)',
           'id key (+0 id, +4 id or −1 wildcard); reads (ordem_das_chaves_de_ids)'),
          ('Media +0 registro de histórias (+8, +0xc sujeitos, +0x11 subtipo, +0x13 tipo); lê (ordem_das_historias)',
           'Media +0 story log (+8, +0xc subjects, +0x11 subtype, +0x13 type); reads (ordem_das_historias)')]
INTERESSA = ['SGE', 'CMScout', 'CMRTE']
