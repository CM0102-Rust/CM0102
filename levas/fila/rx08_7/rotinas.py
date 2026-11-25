# Os dados proprios das eliminatorias europeias da Copa do Mundo no jogo salvo (wc_europe_league.cpp; leitor rx08,
# rodada 6, 25/11/2026). Item do SAVE (CARGA): o arquivo temporario de cada liga (League::save_to_temp_file 0x6891c0 /
# League::load_from_temp_file 0x687b10) chama as virtuais +0x24 (gravar) e +0x20 (ler) com o FILE aberto; a secao
# europeia existe em toda carreira (dormente em 2001-2002, com o bolo da campanha de 2004 ja montado pelo construtor),
# entao esta no save de ferias; a lista dos resultados guardados (+0x10) esta vazia antes de 2004.
# - 0x91a9f0 WcEuropeLeague::load_extra_data thiscall(arquivo) ret 4 -> 0/1: fread 0x945bfe (cdecl) de +0xf6 (1 byte;
#   caixa 0x693) e o formato (+0xff, +0x100, +0x101) pela contagem de isentas; a contagem de resultados (caixa 0x6b4);
#   cada resultado (registro de partida, 0x4f bytes; caixa 0x6bc) com os ponteiros refeitos pelos ids (mandante +0xc ->
#   +0x1c e visitante +0x10 -> +0x20 pela tabela de clubes 0x245, -1: caixas 0x6c4 e 0x6cb; a competicao +0 -> +0x14 pela
#   tabela 0x6b, -1: caixa 0x6d3; o estadio +4 -> +0x18 pela tabela 0x4e ou 0) e LLIST::add_tail 0x68d0d0 (thiscall,
#   ret 8) na lista +0x10; depois +0xfb (caixa 0x6ed) e +0xf2 (caixa 0x6f5); contagem 0: devolve 1; senao malloc(4 x n)
#   em +0xee (caixa 0x700), fread dos indices (caixa 0x708), indices -> clubes (-1 -> 0). Todas as caixas sao simples
#   (sem sair do jogo) e devolvem 0. Prologo sub esp, 0x254 (6 bytes).
# - 0x91ae80 WcEuropeLeague::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 (cdecl) de +0xf6 (sem conferir),
#   fgetpos 0x946c1e (caixa 0x733), a contagem 0 (caixa 0x73d), cada resultado da lista (o cursor +0xa volta a cabeca
#   +2; LLIST::get_next 0x68d1b0) sem conferir, fgetpos do fim (0x74c), fsetpos 0x946c06 de volta (0x753), a contagem de
#   verdade (0x758), fsetpos do fim (0x75f), +0xfb (0x766), +0xf2 e o bolo como indices (o id do clube, +0; nulo -> -1)
#   sem conferir, e de volta a ponteiros; devolve a contagem do bolo (o original devolve o eax da caixa nos erros, a
#   copia 0). Prologo sub esp, 0x214 (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: a leitura so a original (acrescenta a lista e aloca o bolo), com o
# plano (o arquivo andou 1 + 4 + n x 0x4f + 4 + 4 + 4 x bolo, a lista cresceu n e o formato segue as isentas); a
# gravacao so a original (a prova e a do save).
# Depende do cx06_2 (FN_CRT_FGETPOS, FN_CRT_FSETPOS), do cx16_6 (club_indices_to_pointers, club_pointers_to_indices)
# (WC_EUROPE_EXEMPT e WC_EUROPE_LAST_GROUP ja estao no HEAD): integrar depois do cx06_2 e do cx16_6.
SAVE = True
CARGA = True
ROTINAS = [('wc_europe_league_load_extra_data', 0x91a9f0, [0x81, 0xec, 0x54, 0x02, 0x00, 0x00]),
           ('wc_europe_league_save_extra_data', 0x91ae80, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00])]
DOCS = ('os dados próprios das eliminatórias europeias da Copa do Mundo no jogo salvo', "the European World Cup qualifying's own data in the saved game",
        [('wc_europe_league_load_extra_data', 'WcEuropeLeague::load_extra_data', '0x91a9f0', 'wc_europe_league.cpp', 'thiscall, ret 4',
          'lê do jogo salvo o número de isentas (que dá o formato dos grupos), os resultados de grupo guardados (cada partida com os ponteiros refeitos pelos ids), a marca de concluída e o bolo das seleções (os índices viram clubes); leitura curta ou id −1 do mandante, do visitante ou da competição: a caixa de erro e 0',
          'reads from the saved game the exempt count (which sets the group format), the stored group results (each fixture with its pointers rebuilt from the ids), the done flag and the nation pool (the indices become clubs); a short read or a home, away or competition id of −1: the error box and 0'),
         ('wc_europe_league_save_extra_data', 'WcEuropeLeague::save_extra_data', '0x91ae80', 'wc_europe_league.cpp', 'thiscall, ret 4',
          'grava no jogo salvo o número de isentas, os resultados de grupo guardados (a contagem escrita depois, por cima de um zero, voltando a posição do arquivo), a marca de concluída e o bolo como índices de clube; falha de escrita ou de posição: a caixa de erro',
          'writes to the saved game the exempt count, the stored group results (the count written afterwards over a zero, going back in the file), the done flag and the pool as club indices; a failed write or position: the error box')])
CAMPOS = []
INTERESSA = []
