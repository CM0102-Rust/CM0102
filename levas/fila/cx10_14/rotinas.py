# A gravacao dos dados proprios da Primeira Divisao holandesa no jogo salvo (hol_first.cpp; leitor cx10, rodada 5,
# 24/11/2026). Item do SAVE (gravar), como a gravacao do cx02_13: o arquivo temporario da liga chama a virtual de gravar
# os dados proprios com o FILE aberto em CM3_TEMP. So roda com a Holanda carregada no jogo salvo (a prova pede um save
# com ela).
# - 0x5f75b0 HolFirst::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 cdecl de 1 byte de +0xee, +0xef, +0xf0 e
#   +0xf1 (as quatro marcas de vencedor de periodo), sem conferir; devolve o eax do ultimo fwrite. Prologo push esi;
#   push edi; mov edi, [esp+0xc] (6 bytes). Igual no cm0102-1080.exe (inteira).
# Comparador: so a original (grava o jogo salvo; a prova e a do save).
SAVE = True
ROTINAS = [('hol_first_save_extra_data', 0x5f75b0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c])]
DOCS = ('a gravação dos dados próprios da Primeira Divisão holandesa no jogo salvo',
        'the Dutch First Division\'s own data written to the saved game',
        [('hol_first_save_extra_data', 'HolFirst::save_extra_data', '0x5f75b0', 'hol_first.cpp', 'thiscall, ret 4',
          'grava no jogo salvo as quatro marcas de vencedor de período, um byte cada, na ordem em que a carga as lê',
          'writes the four period winner flags to the saved game, one byte each, in the order the load reads them')])
CAMPOS = []
INTERESSA = []
