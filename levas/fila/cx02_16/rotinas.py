# A lista de inscritos das copas com grupos no jogo salvo, gravacao, e a da Copa da Holanda (gre_cup.cpp, hol_cup.cpp;
# leitor cx02, rodada 5, 24/11/2026). Item do SAVE (CARGA). Usa load_group_cup_entrants e compare_group_cup_entrants_load
# do cx02_14 (integrar depois dele). So com um save que tenha essas copas (Grecia, Irlanda, Irlanda do Norte, Italia,
# Coreia, Gales ou Holanda); o FeriasTesteSave nao as chama.
# - 0x5e9fb0 GreCup::save_extra_data thiscall(arquivo) ret 4: a virtual 0x24 de oito vtables (GreCup, IreLgeCup,
#   ItaCCup, ItaCup, KorLeagueCup, NirLgeCup, WelLgeCup 0x971034, WelPrmCup 0x9710e8); fwrite 0x945ce6 (cdecl) da
#   contagem (+0xf2), os clubes de +0xee viram ids (nulo -1), fwrite dos ids e de volta (-1 vira 0); devolve em eax a
#   contagem lida por ultimo. Prologo push ebx; mov ebx, [esp+8] (5 bytes).
# - 0x5f4e90 HolCup::load_extra_data: como GreCup::load_extra_data, mas a contagem 0 deixa +0xee = 0 e devolve 1 sem
#   malloc nem fread (hol_cup.cpp, linhas 0x44b, 0x455, 0x45d). Prologo sub esp, 0x200.
# - 0x5f5070 HolCup::save_extra_data: como GreCup::save_extra_data, mas sem lista (+0xee nulo) nao grava os ids.
# Iguais no cm0102-1080.exe.
# Comparadores: a gravacao so a original, conferindo que a lista volta como era (clubes, nao ids); a leitura como no
# cx02_14.
SAVE = True
CARGA = True
ROTINAS = [('gre_cup_save_extra_data', 0x5e9fb0, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('hol_cup_load_extra_data', 0x5f4e90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('hol_cup_save_extra_data', 0x5f5070, [0x53, 0x8b, 0x5c, 0x24, 0x08])]
DOCS = ('a lista de inscritos das copas com grupos no jogo salvo (gravação) e a da Copa da Holanda',
        "the group cups' entrant list in the saved game (writing) and the Dutch Cup's",
        [('gre_cup_save_extra_data', 'GreCup::save_extra_data', '0x5e9fb0', 'gre_cup.cpp', 'thiscall, ret 4',
          'grava no arquivo temporário da copa (a Copa da Grécia e as outras copas com grupos que usam esta virtual: as Copas da Liga da Irlanda, da Irlanda do Norte e da Coreia, as Copas da Itália e da Série C e as duas copas de Gales) o número de inscritos e os ids dos clubes (vaga vazia −1), na ordem da leitura; a lista volta a ter os clubes',
          "writes into the cup's temporary file (the Greek Cup and the other group cups that use this virtual: the Irish, Northern Irish and Korean League Cups, the Coppa Italia, the Serie C cup and the two Welsh cups) the entrant count and the clubs' ids (an empty slot −1), in the order the load reads them; the list gets its clubs back"),
         ('hol_cup_load_extra_data', 'HolCup::load_extra_data', '0x5f4e90', 'hol_cup.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Copa da Holanda o número de inscritos e os ids, que voltam a ser clubes; com nenhum inscrito, a lista fica vazia; leitura curta ou falta de memória: a caixa de erro e 0',
          'reads from the Dutch Cup temporary file the entrant count and the ids, which become clubs again; with no entrants, the list stays empty; a short read or no memory: the error box and 0'),
         ('hol_cup_save_extra_data', 'HolCup::save_extra_data', '0x5f5070', 'hol_cup.cpp', 'thiscall, ret 4',
          'grava no arquivo temporário da Copa da Holanda o número de inscritos e, havendo lista, os ids dos clubes',
          "writes into the Dutch Cup temporary file the entrant count and, when there is a list, the clubs' ids")])
CAMPOS = []
INTERESSA = []
