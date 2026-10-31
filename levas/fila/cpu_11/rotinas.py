# As casas da formação (formation.cpp): Formation::set_slot_coordinate (0x5b3560, 495 chamadas na cobertura de 17/10:
# 11 por formação carregada pela 0x5b5d10, as das táticas da IA), Formation::check_duplicate_position (0x5b3460) e
# Formation::set_marking_type (0x5b57d0), essas duas só pelas telas de tática (passageiras). A 0x5b3560 grava a
# coordenada da casa e escolhe a posição geral (+0x12d) e a extra (+0x14e) pelas posições mais perto das 12 coordenadas
# da casa (0x5b7e60, já em Rust); só chama rotinas já portadas (0x5b7e60, 0x5b73a0, 0x5b59a0) e a caixa de erro. Na
# original a contagem de cada lista não tem valor inicial (a primeira posição sempre a zera) e, com duas posições na
# outra fase, os índices fora das tabelas leem outras variáveis do quadro (reproduzidas, menos um ponteiro de pilha).
# Sem SEH. Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira (comparador lado a lado) e a confirmação no
# jogo real comparando. Comparadores: lado a lado sobre a formação (+0x12d..+0x584, +0x5b4, +0x187b); a posição repetida
# sobre saídas próprias.
SAVE = False
ROTINAS = [('coordenada_da_casa', 0x5b3560, [0x81, 0xec, 0x80, 0x02, 0x00, 0x00]),
           ('posicao_repetida_na_formacao', 0x5b3460, [0x8a, 0x54, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tipo_de_marcacao', 0x5b57d0, [0x8a, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as casas da formação: gravar a coordenada da casa e escolher as posições, a posição repetida e o tipo de marcação',
        "the formation slots: storing the slot coordinate and choosing the positions, the duplicate position and the marking type",
        [('coordenada_da_casa', 'Formation::set_slot_coordinate', '0x5b3560', 'formation.cpp', 'thiscall, ret 0x10',
          'grava a coordenada da casa na fase e na faixa da bola e escolhe a posição geral e a extra pelas posições mais perto das 12 coordenadas da casa',
          "stores the slot coordinate for the phase and ball zone and picks the general and extra positions closest to the slot's 12 coordinates"),
         ('posicao_repetida_na_formacao', 'Formation::check_duplicate_position', '0x5b3460', 'formation.cpp', 'thiscall, ret 8',
          'acha outra casa com a mesma posição geral e marca a saída quando a marca dela é menor',
          'finds another slot with the same general position and sets the output when its flag is lower'),
         ('tipo_de_marcacao', 'Formation::set_marking_type', '0x5b57d0', 'formation.cpp', 'thiscall, ret 8',
          'troca um tipo de marcação da casa (bits 0..2 ou 3..4 do byte +0x5b4) e zera +0x187f',
          "sets one marking type of the slot (bits 0..2 or 3..4 of byte +0x5b4) and clears +0x187f")])
CAMPOS = [('Formation +0x12d: a posição geral das 11 casas (palavra, máscara de setor e lado); +0x143 a marca da casa (byte); +0x14e a posição extra (palavra); valor; lê e grava (coordenada_da_casa, posicao_repetida_na_formacao)',
           'Formation +0x12d: the general position of the 11 slots (word, sector and side mask); +0x143 the slot flag (byte); +0x14e the extra position (word); value; reads and writes (coordenada_da_casa, posicao_repetida_na_formacao)'),
          ('Formation +0x164: as coordenadas das casas (11 × 2 fases × 3 × 4 faixas da bola, dword: x 0..26 e y 0..35); valor; lê e grava (coordenada_da_casa)',
           'Formation +0x164: the slot coordinates (11 × 2 phases × 3 × 4 ball zones, dword: x 0..26 and y 0..35); value; reads and writes (coordenada_da_casa)'),
          ('Formation +0x5b4: o tipo de marcação das 11 casas (bits 0..2 e 3..4); +0x187b a máscara da formação; +0x187f zerado a cada mudança; valor; grava (tipo_de_marcacao, coordenada_da_casa)',
           'Formation +0x5b4: the marking type of the 11 slots (bits 0..2 and 3..4); +0x187b the formation mask; +0x187f cleared on each change; value; writes (tipo_de_marcacao, coordenada_da_casa)'),
          ('[0x9cc368] a tabela das posições (palavras até 0xffff); valor; lê (coordenada_da_casa)',
           '[0x9cc368] the position table (words up to 0xffff); value; reads (coordenada_da_casa)')]
INTERESSA = ['CMRTE']
