# Os dados proprios da Divisao Principal irlandesa e a leitura dos da Primeira Divisao holandesa no jogo salvo
# (ire_prm.cpp, hol_first.cpp; leitor cx10, rodada 5, 24/11/2026). Item do SAVE (CARGA), como o cx02_13: o arquivo
# temporario de cada liga chama as virtuais de gravar e ler os dados proprios com o FILE aberto em CM3_TEMP. So rodam
# com a Irlanda e a Holanda carregadas no jogo salvo (o FeriasTesteSave so tem a Inglaterra: a prova pede um save com
# elas).
# - 0x63ac80 IrePrm::load_extra_data thiscall(arquivo) ret 4 -> 0/1: fread 0x945bfe cdecl(this+0xee, 5, 10, arquivo) e 1
#   quando volta 10. Prologo mov eax, [esp+4]; add ecx, 0xee (10 bytes).
# - 0x63acb0 IrePrm::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 cdecl(this+0xee, 5, 10, arquivo); devolve o
#   eax do fwrite. Mesmo prologo.
# - 0x5f7550 HolFirst::load_extra_data thiscall(arquivo) ret 4 -> 1: fread de 1 byte em +0xee, +0xef, +0xf0 e +0xf1 (as
#   quatro marcas de vencedor de periodo), sem conferir. Prologo push esi; push edi; mov edi, [esp+0xc] (6 bytes).
# Iguais no cm0102-1080.exe (inteiras).
# Comparadores: as leituras lado a lado pelo compare_cup_extra_data_load do cx02_13 (ftell/fseek: o mesmo trecho lido
# duas vezes; os campos, o retorno e a posicao final); a gravacao so a original (a prova e a do save).
# Depende do cx02_13 (compare_cup_extra_data_load): integrar depois dele.
SAVE = True
CARGA = True
ROTINAS = [('ire_prm_load_extra_data', 0x63ac80, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xc1, 0xee, 0x00, 0x00, 0x00]),
           ('ire_prm_save_extra_data', 0x63acb0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xc1, 0xee, 0x00, 0x00, 0x00]),
           ('hol_first_load_extra_data', 0x5f7550, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c])]
DOCS = ('os dados próprios da Divisão Principal irlandesa e da Primeira Divisão holandesa no jogo salvo',
        'the Irish Premier Division\'s and the Dutch First Division\'s own data in the saved game',
        [('ire_prm_load_extra_data', 'IrePrm::load_extra_data', '0x63ac80', 'ire_prm.cpp', 'thiscall, ret 4',
          'lê do jogo salvo as 10 vagas fixas do sorteio (5 bytes cada) e devolve 1 quando chegam as dez',
          'reads the 10 fixed draw slots (5 bytes each) from the saved game and returns 1 when all ten arrive'),
         ('ire_prm_save_extra_data', 'IrePrm::save_extra_data', '0x63acb0', 'ire_prm.cpp', 'thiscall, ret 4',
          'grava no jogo salvo as 10 vagas fixas do sorteio',
          'writes the 10 fixed draw slots to the saved game'),
         ('hol_first_load_extra_data', 'HolFirst::load_extra_data', '0x5f7550', 'hol_first.cpp', 'thiscall, ret 4',
          'lê do jogo salvo as quatro marcas de vencedor de período, um byte cada, sem conferir, e devolve 1',
          'reads the four period winner flags from the saved game, one byte each, unchecked, and returns 1')])
CAMPOS = []
INTERESSA = []
