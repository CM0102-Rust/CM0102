# Os dados proprios da Terceira Divisao da Belgica e da Primeira Divisao da Croacia no jogo salvo (bel_third.cpp,
# cro_a1.cpp; leitor cx09, rodada 5, 24/11/2026). Item do SAVE (CARGA): o arquivo temporario de cada liga chama as
# virtuais de ler e gravar os dados proprios com o FILE aberto; os .tmp vao para o .qst.
# - 0x424220 BelThird::load_extra_data thiscall(arquivo) ret 4 -> 1: fread 0x945bfe (cdecl, 4 pushes, add esp 0x40 no
#   fim) de (+0xfe, 1, 2), (+0x100, 1, 2), (+0x102, 1, 2) e (+0xee, 4, 4), sem conferir. Prologo push esi; push edi;
#   mov edi, [esp+0xc] (6 bytes).
# - 0x424280 BelThird::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 (cdecl) na mesma ordem; devolve o eax
#   do ultimo fwrite (nao poe 1). Mesmo prologo.
# - 0x5152f0 CroA1::load_extra_data thiscall(arquivo) ret 4 -> 1: fread(+0xee, 1, 1). Prologo mov eax, [esp+4];
#   add ecx, 0xee (10 bytes).
# Iguais no cm0102-1080.exe. A gravacao da CroA1 nao esta na lista (outra rotina).
# Comparadores: as leituras lado a lado com o arquivo de volta a posicao (compare_cup_extra_data_load do cx02_13);
# a gravacao so a original (a prova e a do save).
# A prova do save precisa de um save com a Belgica e a Croacia carregadas (o FeriasTesteSave so tem a Inglaterra).
SAVE = True
CARGA = True
ROTINAS = [('bel_third_load_extra_data', 0x424220, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('bel_third_save_extra_data', 0x424280, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('cro_a1_load_extra_data', 0x5152f0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xc1, 0xee, 0x00, 0x00, 0x00])]
DOCS = ('os dados próprios da Terceira Divisão da Bélgica e da Primeira Divisão da Croácia no jogo salvo',
        "the Belgian Third Division's and the Croatian First Division's own data in the saved game",
        [('bel_third_load_extra_data', 'BelThird::load_extra_data', '0x424220', 'bel_third.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Terceira Divisão belga os três pares de marcas dos períodos e os quatro clubes guardados da Quarta Divisão para a promoção',
          'reads from the Belgian Third Division temporary file the three period flag pairs and the four Fourth Division clubs kept for promotion'),
         ('bel_third_save_extra_data', 'BelThird::save_extra_data', '0x424280', 'bel_third.cpp', 'thiscall, ret 4',
          'grava no arquivo temporário da Terceira Divisão belga os três pares de marcas dos períodos e os quatro clubes guardados, na ordem da leitura',
          'writes into the Belgian Third Division temporary file the three period flag pairs and the four stored clubs, in the order the load reads them'),
         ('cro_a1_load_extra_data', 'CroA1::load_extra_data', '0x5152f0', 'cro_a1.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Primeira Divisão croata o número de pares do play-off',
          'reads from the Croatian First Division temporary file the play-off pair count')])
CAMPOS = []
INTERESSA = []
