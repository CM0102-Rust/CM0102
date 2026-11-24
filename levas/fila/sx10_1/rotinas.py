# A nevoa (fog of war) no save: a construcao e a carga (FOG_OF_WAR::FOG_OF_WAR 0x5af630, FOG_OF_WAR::load 0x5af6e0,
# FOG_OF_WAR_MAP::load 0x5b0010, fog_of_war.cpp; leitor sx10, rodada 5, 23/11/2026). Item do save (CARGA).
# Inicio e fim conferidos na desmontagem: 0x5af630..0x5af69b (ret 4), 0x5af6e0..0x5afac4 (moldura SEH, ret 4),
# 0x5b0010..0x5b015f (ret 4); sem tabelas de saltos.
# Chamadores: o construtor e chamado so por 0x829cd8 (nova carreira, sem save) e 0x82ca3c (a carga de um save, com o
# arquivo), sempre num bloco de 0x40 bytes de operator new guardado em G_FOG_OF_WAR 0xae2a60; o load so pelo
# construtor (0x5af653); o FOG_OF_WAR_MAP::load so pelo load (0x5af8ef).
# Convencoes conferidas: os tres thiscall ret 4; zopen 0x9326e0 cdecl(save, nome, 0, 0x16, 4); zread 0x932c30
# cdecl(buf, tamanho, n, z); zclose 0x932b00 cdecl(&z); operator new 0x944cf1 cdecl(8); o construtor do mapa 0x5aff00
# thiscall(mapa, tecnico) ret 4; FOG_OF_WAR::add_manager_map 0x5afd70 thiscall(nevoa, tecnico) ret 4; as caixas pelo
# _splitpath + utils_get_version_string + sprintf "v%s %s.%s %d" + gui_show_message_box (show_error_at).
# Comparadores: so a original (le o item do save e aloca). A prova: a do save com a carga do FeriasTesteSave (o
# construtor com o arquivo e o load com a nevoa desligada no save); o ramo da nevoa ligada (mapas lidos) so roda com
# um save feito com "attribute masking" ligado. O sx10_2 depende deste item (FOG_OF_WAR_CPP, FOG_DAT, fog_free_maps).
SAVE = True
CARGA = True
ROTINAS = [('fog_of_war_new', 0x5af630, [0x55, 0x8b, 0xe9, 0x57, 0xb9, 0x10, 0x00, 0x00, 0x00]),
           ('fog_of_war_load', 0x5af6e0, [0x6a, 0xff, 0x68, 0x6c, 0x98, 0x95, 0x00]),
           ('fog_of_war_map_load', 0x5b0010, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a névoa no save: a construção e a carga',
        'the fog of war in the save: building it and reading it',
        [('fog_of_war_new', 'FOG_OF_WAR::FOG_OF_WAR', '0x5af630', 'fog_of_war.cpp', 'thiscall, ret 4',
          'zera as 16 vagas de mapa; com a máscara de atributos ligada e um save, lê a névoa do save (FOG_OF_WAR::load) e, se a leitura falha, dá um mapa novo a cada técnico humano entre as 16 últimas pessoas da tabela (classificação 1); a carreira nova não passa save e começa sem mapas',
          'clears the 16 map slots; with attribute masking on and a save, reads the fog of war from the save (FOG_OF_WAR::load) and, when that fails, gives a fresh map to each human manager among the last 16 people of the table (classification 1); a new career passes no save and starts with no maps'),
         ('fog_of_war_load', 'FOG_OF_WAR::load', '0x5af6e0', 'fog_of_war.cpp', 'thiscall, ret 4',
          'save de versão anterior a 0x3e036: um mapa novo para cada técnico humano, na vaga dele, e 1; senão lê do item fog.dat o byte da máscara de atributos e, ligada, para cada uma das 16 vagas uma palavra de presença e o mapa (FOG_OF_WAR_MAP::load); cada falha mostra a caixa da linha e devolve 0 (as de alocação e de leitura do mapa liberam todos os mapas)',
          'a save older than version 0x3e036: a fresh map for each human manager, in his slot, and 1; otherwise reads from the fog.dat item the attribute masking byte and, when on, for each of the 16 slots a presence word and the map (FOG_OF_WAR_MAP::load); each failure shows the box of its line and returns 0 (the allocation and map read ones free every map)'),
         ('fog_of_war_map_load', 'FOG_OF_WAR_MAP::load', '0x5b0010', 'fog_of_war.cpp', 'thiscall, ret 4',
          'lê o id do técnico dono do mapa (fora de 0..total de pessoas: caixa e 0; o total em si é aceito) e a grade de conhecimento, 9 bytes por pessoa (o nível e a data em que subiu); leitura curta: caixa e 0',
          "reads the id of the map's manager (outside 0..the people count: box and 0; the count itself is accepted) and the knowledge grid, 9 bytes per person (the level and the date it last rose); a short read: box and 0")])
