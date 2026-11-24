# As vagas e as coordenadas da formacao nas telas de taticas (formation.cpp; leitor sx10, rodada 5, 23/11/2026).
# Cada rotina provada pela desmontagem: inicio no prologo (sub esp, imm32, ou mov reg, [esp+k] + sub esp), fim no
# ultimo ret antes do enchimento de nop, sem tabelas de saltos. Chamadores:
# - Formation(registro da partida, lado) 0x5b2930: tactics_screens 0x8a7a3d (a formacao do adversario da partida);
# - swap_slots 0x5b2f30: tactics_screens 0x8a2266, 0x8ab987, 0x8ab9e1;
# - get_slots_in_position_order 0x5b31b0: club_screens 0x459a9b, 0x4685d7;
# - set/get_zone_coordinates 0x5b3de0/0x5b3f90, set/get_phase_coordinates 0x5b4120/0x5b42a0, find_slot_at_coordinate
#   0x5b4410, swap_slot_coordinates 0x5b45b0, mirror_zone_coordinates 0x5b47a0, is_slot_customised 0x5b4a60:
#   tactics_screens (0x8a9d55, 0x8a13ac, 0x8a9d23, 0x8a1421, 0x8a2477, 0x8a9df9, 0x8a1511, 0x8a9fc4);
# - is_slot_coordinate_customised 0x5b4b70: is_slot_customised 0x5b4aa7 e tactics_screens 0x8a9fe8.
# Nenhuma roda na carreira automatica (cobertura do mundo: 0 chamadas).
# Convencoes conferidas na desmontagem (todas thiscall): 0x5b2930 (registro, lado) ret 8 -> this; swap_slots (a, b)
# ret 8; get_slots_in_position_order (saida, tamanho) ret 8; set/get_zone_coordinates (fase, zona, vetor) ret 0xc;
# set/get_phase_coordinates (fase, vetor) ret 8; find_slot_at_coordinate (fase, zona, ponto) ret 0xc -> al;
# swap_slot_coordinates (a, b, fase, zona) ret 0x10; mirror_zone_coordinates (fase, zona) ret 8; is_slot_customised
# (vaga) ret 4; is_slot_coordinate_customised (vaga, fase, zona) ret 0xc. A zona e um dword: faixa na palavra baixa,
# altura na alta. Chamadas: set_slot_position 0x5b2ba0, set_slot_coordinate 0x5b3560, get_slot_coordinate 0x5b3c70,
# update_shape_code 0x5b59a0, build_position_lookup 0x5b7160 (ja Rust), formation_get_default_coordinate 0x5b7e60 e
# formation_split_position_code 0x5b73a0 cdecl, utils_swap_bytes 0x90ba90 cdecl(a, b, n), langlib_sprintf 0x66f4e0.
# Comparadores: lado a lado sobre o registro inteiro (as que mudam a formacao), sobre o vetor escrito (os getters) ou
# o valor devolvido; argumento fora da faixa (a caixa de erro): so a original.
TELAS = True
ROTINAS = [('formation_new_vs_club', 0x5b2930, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('formation_swap_slots', 0x5b2f30, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('formation_slots_in_position_order', 0x5b31b0, [0x81, 0xec, 0x3c, 0x04, 0x00, 0x00]),
           ('formation_set_zone_coordinates', 0x5b3de0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('formation_get_zone_coordinates', 0x5b3f90, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('formation_set_phase_coordinates', 0x5b4120, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('formation_get_phase_coordinates', 0x5b42a0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('formation_find_slot_at_coordinate', 0x5b4410, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('formation_swap_slot_coordinates', 0x5b45b0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('formation_mirror_zone_coordinates', 0x5b47a0, [0x81, 0xec, 0x74, 0x02, 0x00, 0x00]),
           ('formation_is_slot_customised', 0x5b4a60, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('formation_is_slot_coordinate_customised', 0x5b4b70, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00])]
DOCS = ('as vagas e as coordenadas da formação nas telas de táticas',
        'the formation\'s slots and pitch coordinates on the tactics screens',
        [('formation_new_vs_club', 'Formation::Formation', '0x5b2930', 'formation.cpp', 'thiscall',
          'monta a formação de um lado do registro da partida, com o nome "Formation vs <clube>" (o clube do outro lado), as posições gerais e extras de cada vaga, as instruções do time do registro e o resto como no construtor padrão (instruções dos jogadores, marcação 0x12, ninguém designado); sem registro, só a caixa de erro e o registro fica como estava',
          'builds the formation of one side of the match record, named "Formation vs <club>" (the other side\'s club), with each slot\'s general and extra positions, the record\'s team instructions and the rest as in the default constructor (player instructions, marking 0x12, nobody designated); without a record, only the error box and the record is left as it was'),
         ('formation_swap_slots', 'Formation::swap_slots', '0x5b2f30', 'formation.cpp', 'thiscall',
          'troca duas vagas por inteiro: as posições gerais (com a ordem de exibição refeita), as extras, as 24 coordenadas, as instruções do jogador, a marcação e o designado',
          'swaps two slots completely: the general positions (with the display order redone), the extra ones, the 24 coordinates, the player instructions, the marking and the designated person'),
         ('formation_slots_in_position_order', 'Formation::get_slots_in_position_order', '0x5b31b0', 'formation.cpp', 'thiscall',
          'lista as 11 vagas do goleiro para a frente e, em cada linha, da esquerda para a direita (esquerda, direita, centro-esquerda, centro, centro-direita)',
          'lists the 11 slots from the goalkeeper forwards and, within each line, left to right (left, right, centre left, centre, centre right)'),
         ('formation_set_zone_coordinates', 'Formation::set_zone_coordinates', '0x5b3de0', 'formation.cpp', 'thiscall',
          'grava as coordenadas das 11 vagas numa fase e zona da bola (cada vaga reescolhe as posições)',
          'sets the coordinates of the 11 slots in one phase and ball zone (each slot picks its positions again)'),
         ('formation_get_zone_coordinates', 'Formation::get_zone_coordinates', '0x5b3f90', 'formation.cpp', 'thiscall',
          'copia as coordenadas das 11 vagas numa fase e zona da bola',
          'copies the coordinates of the 11 slots in one phase and ball zone'),
         ('formation_set_phase_coordinates', 'Formation::set_phase_coordinates', '0x5b4120', 'formation.cpp', 'thiscall',
          'grava as coordenadas de todas as vagas em todas as zonas de uma fase (faixa, altura, vaga)',
          'sets the coordinates of every slot in every zone of one phase (band, height, slot)'),
         ('formation_get_phase_coordinates', 'Formation::get_phase_coordinates', '0x5b42a0', 'formation.cpp', 'thiscall',
          'copia as coordenadas de todas as vagas em todas as zonas de uma fase',
          'copies the coordinates of every slot in every zone of one phase'),
         ('formation_find_slot_at_coordinate', 'Formation::find_slot_at_coordinate', '0x5b4410', 'formation.cpp', 'thiscall',
          'a primeira vaga que está num ponto do campo numa fase e zona, ou 0xff',
          'the first slot standing on a pitch point in one phase and zone, or 0xff'),
         ('formation_swap_slot_coordinates', 'Formation::swap_slot_coordinates', '0x5b45b0', 'formation.cpp', 'thiscall',
          'troca as coordenadas de duas vagas numa fase e zona',
          'swaps the coordinates of two slots in one phase and zone'),
         ('formation_mirror_zone_coordinates', 'Formation::mirror_zone_coordinates', '0x5b47a0', 'formation.cpp', 'thiscall',
          'copia uma zona para a do lado oposto, espelhada, trocando em cada linha a vaga mais à esquerda com a mais à direita para cada jogador ficar no seu lado',
          'copies a zone to the opposite flank, mirrored, swapping in each line the leftmost slot with the rightmost so that each player keeps his side'),
         ('formation_is_slot_customised', 'Formation::is_slot_customised', '0x5b4a60', 'formation.cpp', 'thiscall',
          '1 se alguma coordenada da vaga (fase, faixa, altura) foge do padrão das suas posições',
          '1 when any coordinate of the slot (phase, band, height) differs from its positions\' default'),
         ('formation_is_slot_coordinate_customised', 'Formation::is_slot_coordinate_customised', '0x5b4b70', 'formation.cpp', 'thiscall',
          '1 se a coordenada da vaga numa fase e zona foge do padrão das suas posições (a posição geral ou a extra, pelo setor, como na set_slot_position)',
          '1 when the slot\'s coordinate in one phase and zone differs from its positions\' default (the general or the extra position, by sector, as in set_slot_position)')])
