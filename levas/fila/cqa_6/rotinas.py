# A lista de observação do técnico humano (player_search.cpp, scout_manager.cpp): PLAYER_SEARCH::contains_player
# (0x7af600: pela lista do olheiro do técnico humano 0x7f7490, que a avaliação dos jogadores e os clubes interessados
# consultam a cada dia; a mutação cps8_sempre_na_lista da 255ª foi acusada, a resposta chega à carreira),
# SCOUT_MANAGER::sync_scout_lists (0x7f7270: quando a comissão do clube humano muda, pelas saídas do cargo e as
# transferências e contratações de pessoal) e player_search_find_or_claim_player_slot (0x7bd700: a vaga do jogador no
# registro do clube observado, pela partida dele 0x7bd310 e pelas telas 0x7f8610). Iguais no cm0102-1080.exe; a
# 0x7af600 tem quadro SEH só pelas duas datas locais (sem destrutor chamado); as outras, sem SEH.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x7af600: 0x7afcf6 push esi; mov ecx, ebp; 0x7f75d6 e 0x80457e mov ecx, eax; push pessoa ->
#   thiscall(busca, pessoa), ret 4 (a cópia Rust da 0x7f7490 também).
# - dentro dela: 0x54a110 lea ecx, [data]; call -> thiscall(data); 0x7e4920 push chave; push pessoa (o add esp, 0x1c
#   depois da busca tira os dois) -> cdecl(pessoa, chave); 0x946464 push 0x7bce50; push 0x2d; push conta; push vetor;
#   push chave -> cdecl(chave, vetor, conta, 0x2d, comparação); a caixa com o fonte 0xa701b0 (player_search.cpp).
# - quem chama a 0x7f7270: 0x4cf2cf, 0x4cf80b e 0x4e56d2 mov ecx, [0xb74448]; push técnico -> thiscall(gerência,
#   técnico), ret 4 (as cópias Rust 0x690890 e 0x7aaeb0 também).
# - dentro dela: 0x7f6cd0 mov ecx, [esp+0x14] (this); push técnico; push dword com o índice no byte baixo ->
#   thiscall(this, índice, técnico), ret 8 (lê só o byte); 0x7ae950 mov ecx, busca -> thiscall(busca); 0x7ae8a0 push 0
#   (ou o olheiro, empurrado antes da 0x7f6cd0 que devolve a busca); mov ecx, busca -> thiscall(busca, olheiro), ret 4;
#   0x53cd60 push vetor; push clube; add esp, 8 -> cdecl(clube, vetor); 0x8043c0 mov ecx, edição -> thiscall(edição),
#   ret 0; a caixa com o fonte 0xa7b094 (scout_manager.cpp).
# - quem chama a 0x7bd700: 0x7f8610 push 0; push pessoa; mov ecx, registro -> thiscall(registro, pessoa, reservar),
#   ret 8; dentro dela só a caixa (0xa701b0).
# Comparadores: a 0x7af600 pela devolução; a 0x7f7270 lado a lado nas 10 buscas do técnico (0x2b0 bytes cada) e na
# devolução (com a edição aberta, só a original); a 0x7bd700 pela devolução ou, reservando, lado a lado nas 50 vagas.
SAVE = False
ROTINAS = [('jogador_na_busca', 0x7af600, [0x6a, 0xff, 0x68, 0x86, 0xf3, 0x95, 0x00]),
           ('sincronizar_listas_dos_olheiros', 0x7f7270, [0x81, 0xec, 0x30, 0x02, 0x00, 0x00]),
           ('vaga_do_jogador_observado', 0x7bd700, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a lista de observação do técnico humano: o jogador na busca, as listas dos olheiros acertadas com os olheiros do clube e a vaga do jogador no registro do clube observado',
        "the human manager's shortlist: the player in the search, the scout lists matched with the club's scouts and the player's slot in the scouted club record",
        [('jogador_na_busca', 'PLAYER_SEARCH::contains_player', '0x7af600', 'player_search.cpp', 'thiscall, ret 4',
          'procura o jogador nos resultados da busca (busca binária pelo id) e devolve 1 se achado e não marcado como removido',
          'looks the player up in the search results (binary search by id) and returns 1 when found and not marked as removed'),
         ('sincronizar_listas_dos_olheiros', 'SCOUT_MANAGER::sync_scout_lists', '0x7f7270', 'scout_manager.cpp', 'thiscall, ret 4',
          'tira das listas de olheiro do técnico os olheiros que não são mais do clube e dá a cada olheiro do clube sem lista a primeira lista livre; com a edição aberta, ela refaz os olheiros',
          "removes from the manager's scout lists the scouts no longer at the club and gives each club scout without a list the first free list; with the edit session open, it refreshes the scouts"),
         ('vaga_do_jogador_observado', 'player_search_find_or_claim_player_slot', '0x7bd700', 'player_search.cpp', 'thiscall, ret 8',
          'a vaga do jogador entre as 50 do registro do clube observado; reservando, a primeira livre recebe o id e zera os contadores',
          "the player's slot among the 50 of the scouted club record; when claiming, the first free one gets the id and its counters are zeroed")])
CAMPOS = [('busca +0x1ee / +0x1f6: os resultados (0x2d bytes: +0xc o id, +0x20 bit 0 removido) e a conta; ponteiro e valor; lê (jogador_na_busca)',
           'search +0x1ee / +0x1f6: the results (0x2d bytes: +0xc the id, +0x20 bit 0 removed) and the count; pointer and value; reads (jogador_na_busca)'),
          ('busca +0xa: o tipo da lista (4 = olheiro); valor; lê (sincronizar_listas_dos_olheiros)',
           'search +0xa: the list type (4 = scout); value; reads (sincronizar_listas_dos_olheiros)'),
          ('busca +0x174..+0x195: o olheiro (ids, índice +0x180, data) e as missões; id e valor; grava pela 0x7ae8a0 (sincronizar_listas_dos_olheiros)',
           'search +0x174..+0x195: the scout (ids, index +0x180, date) and the assignments; id and value; writes through 0x7ae8a0 (sincronizar_listas_dos_olheiros)'),
          ('pessoa +0x39 / +0x3d: o clube e o tipo (9 = olheiro); ponteiro e valor; lê (sincronizar_listas_dos_olheiros)',
           'person +0x39 / +0x3d: the club and the type (9 = scout); pointer and value; reads (sincronizar_listas_dos_olheiros)'),
          ('[gerência de olheiros + vaga × 4 + 0xc]: a edição aberta do técnico; ponteiro; lê (sincronizar_listas_dos_olheiros)',
           "[scout manager + slot × 4 + 0xc]: the manager's open edit session; pointer; reads (sincronizar_listas_dos_olheiros)"),
          ('registro do clube observado +4: 50 vagas de 0x1a2 bytes (+0 id, −1 livre; +4 palavra, +0x1a0, +0x1a1 contadores); id e valor; lê e grava (vaga_do_jogador_observado)',
           'scouted club record +4: 50 slots of 0x1a2 bytes (+0 id, −1 free; +4 word, +0x1a0, +0x1a1 counters); id and value; reads and writes (vaga_do_jogador_observado)')]
INTERESSA = ['CMScout']
