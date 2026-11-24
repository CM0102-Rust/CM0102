# O construtor padrao da formacao (Formation::Formation 0x5b27c0, formation.cpp; leitor sx10, rodada 5, 23/11/2026).
# Inicio no prologo 0x5b27c0 (push ebx/ebp/esi; mov ebp, ecx: 5 bytes), fim no ret 0x5b292b (sem tabela de saltos).
# Chamadores: o construtor de vetor do estado da partida (match_day 0x6b7eb5/0x6b7ed7: 2 formacoes por lado, 0x18e3
# bytes cada), o das taticas (tactics.cpp 0x893f38, 0x895126, 0x898752, 0x898a74), award_screens 0x415fc4,
# human_manager 0x5fb723 e as telas de taticas (0x8a749e...). Roda em toda partida da carreira.
# Convencoes conferidas na desmontagem: thiscall, ret 0, devolve this; Formation::set_slot_position 0x5b2ba0
# thiscall(vaga, posicao, 0) ret 0xc (ja Rust); Formation::build_position_lookup 0x5b7160 thiscall (ja Rust).
# Comparador: lado a lado sobre o registro inteiro (formation_fields::SIZE).
ROTINAS = [('formation_new', 0x5b27c0, [0x53, 0x55, 0x56, 0x8b, 0xe9])]
DOCS = ('a formação padrão',
        'the default formation',
        [('formation_new', 'Formation::Formation', '0x5b27c0', 'formation.cpp', 'thiscall',
          'zera o registro inteiro, dá o nome "Uninitialised", põe nas 11 vagas um 4-4-2 (GK; D R, D L, D CL, D CR; M CL, M R, M CR; F CL, F CR; M L) pela set_slot_position, que também dá as coordenadas de cada vaga; instruções do time 0x22a91, as de cada jogador 0x15552221 com 0xa, marcação 0x12, ninguém designado (-1); refaz a tabela de posições; alterada = 1, sem senha, não gravada; roda nas duas formações de cada lado de toda partida, nas táticas e na formação do técnico humano',
          "zeroes the whole record, names it \"Uninitialised\", puts a 4-4-2 in the 11 slots (GK; D R, D L, D CL, D CR; M CL, M R, M CR; F CL, F CR; M L) through set_slot_position, which also sets each slot's coordinates; team instructions 0x22a91, each player's 0x15552221 with 0xa, marking 0x12, nobody designated (-1); rebuilds the position lookup; changed = 1, no password, not saved; runs for the two formations of each side of every match, for the tactics and for the human manager's formation")])
