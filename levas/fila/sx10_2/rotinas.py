# A nevoa (fog of war) no save: a gravacao e a destruicao (FOG_OF_WAR::save 0x5afad0, FOG_OF_WAR_MAP::save 0x5b0170,
# FOG_OF_WAR::~FOG_OF_WAR 0x5af6a0, fog_of_war.cpp; leitor sx10, rodada 5, 23/11/2026). Item do save (gravacao).
# Depende do sx10_1 (FOG_OF_WAR_CPP, FOG_DAT, FOG_MAP_SLOTS, fog_free_maps): vem depois dele na ordem.
# Inicio e fim conferidos na desmontagem: 0x5afad0..0x5afcda (ret 4), 0x5b0170..0x5b025c (ret 4), 0x5af6a0..0x5af6d7
# (ret); sem tabelas de saltos.
# Chamadores: o save so por 0x82f331 (a gravacao do jogo, com G_FOG_OF_WAR); o FOG_OF_WAR_MAP::save so pelo save
# (0x5afc0a, resultado ignorado); o destrutor so por free_game_data 0x82ffd4 (com G_FOG_OF_WAR nao nulo; depois o
# operator delete do objeto).
# Convencoes conferidas: thiscall ret 4, ret 4 e ret 0; zwrite 0x932e10 cdecl(buf, tamanho, n, z); zopen com 1, 0x16,
# 4; free 0x9452ca e operator delete 0x944c94 cdecl. O save devolve o eax da ultima chamada (lixo; o chamador ignora):
# a copia e void.
# Comparadores: so a original (grava o item do save; libera). A prova: a gravacao do .qst com e sem as rotinas (a
# carreira de teste tem a nevoa desligada: so o byte da mascara); o ramo dos mapas so roda com a nevoa ligada.
SAVE = True
ROTINAS = [('fog_of_war_save', 0x5afad0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('fog_of_war_map_save', 0x5b0170, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('fog_of_war_destructor', 0x5af6a0, [0x53, 0x56, 0x57, 0x8b, 0xf9])]
DOCS = ('a névoa no save: a gravação e a destruição',
        'the fog of war in the save: writing it and freeing it',
        [('fog_of_war_save', 'FOG_OF_WAR::save', '0x5afad0', 'fog_of_war.cpp', 'thiscall, ret 4',
          'grava no item fog.dat o byte da máscara de atributos e, ligada, para cada uma das 16 vagas uma palavra de presença (1 com mapa, 0 sem) e o mapa (FOG_OF_WAR_MAP::save); cada falha mostra a caixa da linha e sai sem fechar o item',
          'writes to the fog.dat item the attribute masking byte and, when on, for each of the 16 slots a presence word (1 with a map, 0 without) and the map (FOG_OF_WAR_MAP::save); each failure shows the box of its line and leaves without closing the item'),
         ('fog_of_war_map_save', 'FOG_OF_WAR_MAP::save', '0x5b0170', 'fog_of_war.cpp', 'thiscall, ret 4',
          'grava o id do técnico dono do mapa e a grade de conhecimento, 9 bytes por pessoa; gravação curta: caixa',
          "writes the id of the map's manager and the knowledge grid, 9 bytes per person; a short write: box"),
         ('fog_of_war_destructor', 'FOG_OF_WAR::~FOG_OF_WAR', '0x5af6a0', 'fog_of_war.cpp', 'thiscall',
          'libera a grade e o mapa de cada uma das 16 vagas que tem mapa (as vagas ficam apontando para os mapas liberados)',
          'frees the grid and the map of each of the 16 slots that holds one (the slots keep pointing at the freed maps)')])
