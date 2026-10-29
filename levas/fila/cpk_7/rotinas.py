# Jogadores e formação: a fluência da pessoa num idioma (PlayerStats::get_language_fluency 0x7c6830, pelas notas
# 0x6e2f20/0x6e6340 já em Rust, shortlist, contratos da comissão e transferências; contrato pela 0x4cf5f0 já em Rust,
# bsearch 0x946464 com a comparação 0x7c6d40), recomeçar o histórico da pessoa (Staff_Records::rewind_staff_history
# 0x86fb80, pelos regens 0x7ab340/0x7abac0 e pela carga 0x82a9c0; FILE_LLIST::reset_last_offset 0x58f110, original)
# e a posição de uma casa da formação (Formation::set_slot_position 0x5b2ba0, pelas instruções da partida 0x6ddab0 já
# em Rust e as formações prontas; 0x5b59a0, 0x5b73a0 e 0x5b7e60 já em Rust). Simulação: as mutações pela carreira e a
# confirmação no jogo real comparando. Sem SEH, sem sorteio; caixa só com pessoa nula (fatal) ou casa fora da faixa
# (a original). A fluência é chamada muitas vezes pelas notas: sem contador nem mutação nela. Comparadores: a fluência
# pelo valor; o histórico e a formação lado a lado.
SAVE = False
ROTINAS = [('fluencia_no_idioma', 0x7c6830, [0x83, 0xec, 0x08, 0x53, 0x56]),
           ('recomecar_historico_da_pessoa', 0x86fb80, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('posicao_na_formacao', 0x5b2ba0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x1c, 0x02, 0x00, 0x00])]
DOCS = ('a fluência da pessoa num idioma, recomeçar o histórico da pessoa e a posição de uma casa da formação',
        "a person's fluency in a language, rewinding a person's history and the position of a formation slot",
        [('fluencia_no_idioma', 'PlayerStats::get_language_fluency', '0x7c6830', 'player_stats.cpp', 'thiscall, ret 0x10',
          'de 0 a 10: o nível do contrato, 10 pela nação (ou a segunda) que fala o idioma, ou o idioma aprendido (busca binária)',
          'from 0 to 10: the contract level, 10 for a nation (or the second one) speaking the language, or the learned language (binary search)'),
         ('recomecar_historico_da_pessoa', 'Staff_Records::rewind_staff_history', '0x86fb80', 'staff_records.cpp', 'thiscall, ret 4',
          'o último deslocamento da lista do histórico da pessoa (+4 do par em [+0xc] + 8 × id) = −1 (0x58f110)',
          "the last offset of the person's history list (+4 of the pair at [+0xc] + 8 × id) = −1 (0x58f110)"),
         ('posicao_na_formacao', 'Formation::set_slot_position', '0x5b2ba0', 'formation.cpp', 'thiscall, ret 0xc',
          'grava a posição geral ou extra da casa, as marcas, e recalcula as casas no campo sem e com a bola (0x5b7e60)',
          'stores the general or extra position of the slot, the marks, and recomputes the pitch cells without and with the ball (0x5b7e60)')])
CAMPOS = [('pessoa +0 id, +0x1a nação, +0x1e segunda nação, +0x39 clube; nação +0x77 idioma; clube +0x53 nação; contrato +0x4f (nível & 0xf); PlayerStats +0x3c idiomas aprendidos (6 bytes: id, idioma, nível em +5), +0x44 quantos; lê (fluencia_no_idioma)',
           'person +0 id, +0x1a nation, +0x1e second nation, +0x39 club; nation +0x77 language; club +0x53 nation; contract +0x4f (level & 0xf); PlayerStats +0x3c learned languages (6 bytes: id, language, level at +5), +0x44 count; reads (fluencia_no_idioma)'),
          ('Staff_Records +0xc pares (primeiro, último deslocamento) por id; grava −1 no último (recomecar_historico_da_pessoa)',
           'Staff_Records +0xc pairs (first, last offset) per id; writes −1 to the last (recomecar_historico_da_pessoa)'),
          ('formação +0x12d posição geral (11 palavras), +0x143 marcas (11 bytes), +0x14e posição extra (11 palavras), +0x164 casas no campo (24 por casa: 12 sem e 12 com a bola), +0x187b máscara, +0x187f zerado; grava (posicao_na_formacao)',
           'formation +0x12d general position (11 words), +0x143 marks (11 bytes), +0x14e extra position (11 words), +0x164 pitch cells (24 per slot: 12 without and 12 with the ball), +0x187b mask, +0x187f cleared; writes (posicao_na_formacao)')]
INTERESSA = ['CMRTE', 'CMScout', 'SGE']
