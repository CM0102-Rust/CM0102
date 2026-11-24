# Os dados proprios da Primeira Divisao da Finlandia no jogo salvo (fin_first.cpp; leitor cx09, rodada 5,
# 24/11/2026). Item do SAVE (CARGA), como o cx09_6.
# - 0x594fb0 FinFirst::load_extra_data thiscall(arquivo) ret 4 -> 1: fread 0x945bfe (cdecl) de (+0xee, 4, 4), sem
#   conferir. Prologo mov eax, [esp+4]; add ecx, 0xee (10 bytes).
# - 0x594fd0 FinFirst::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 (cdecl) de (+0xee, 4, 4); devolve o eax
#   do fwrite. Mesmo prologo.
# Iguais no cm0102-1080.exe.
# Comparadores: a leitura lado a lado (compare_cup_extra_data_load do cx02_13); a gravacao so a original.
# A prova do save precisa de um save com a Finlandia carregada (o FeriasTesteSave so tem a Inglaterra).
SAVE = True
CARGA = True
ROTINAS = [('fin_first_load_extra_data', 0x594fb0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xc1, 0xee, 0x00, 0x00, 0x00]),
           ('fin_first_save_extra_data', 0x594fd0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xc1, 0xee, 0x00, 0x00, 0x00])]
DOCS = ('os dados próprios da Primeira Divisão da Finlândia no jogo salvo',
        "the Finnish First Division's own data in the saved game",
        [('fin_first_load_extra_data', 'FinFirst::load_extra_data', '0x594fb0', 'fin_first.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Primeira Divisão finlandesa os quatro clubes da Segunda Divisão guardados para a promoção',
          'reads from the Finnish First Division temporary file the four Second Division clubs kept for promotion'),
         ('fin_first_save_extra_data', 'FinFirst::save_extra_data', '0x594fd0', 'fin_first.cpp', 'thiscall, ret 4',
          'grava no arquivo temporário da Primeira Divisão finlandesa os quatro clubes guardados',
          'writes into the Finnish First Division temporary file the four stored clubs')])
CAMPOS = []
INTERESSA = []
