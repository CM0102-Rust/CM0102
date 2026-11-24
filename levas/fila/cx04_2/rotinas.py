# Os dados proprios da Intertoto e da Copa da UEFA no jogo salvo (intertoto_cup.cpp, uefa_cup.cpp; leitor cx04, rodada
# 5, 24/11/2026). Item do SAVE (CARGA): o arquivo temporario de cada copa (Cup::save_cup_to_disk 0x5210f0 /
# Cup::load_from_disk 0x51fc00) chama as virtuais 0x24 (gravar) e 0x20 (ler) com o FILE aberto; as duas copas entram na
# carreira ENG (os clubes ingleses jogam), entao estao no save de ferias.
# - 0x633eb0 IntertotoCup::load_extra_data thiscall(arquivo) ret 4 -> 0/1: fread 0x945bfe (4 pushes; add esp, 0x10 ->
#   cdecl) de 4 bytes em +0xb2; curto: a caixa do intertoto_cup.cpp (linha 0x2d5) e 0. Prologo mov eax, [esp+4]; sub
#   esp, 0x200 (10 bytes).
# - 0x633f50 IntertotoCup::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 (cdecl) de 4 bytes de +0xb2; devolve o
#   eax do fwrite. Prologo mov eax, [esp+4]; add ecx, 0xb2 (10 bytes).
# - 0x903300 UefaCup::load_extra_data thiscall(arquivo) ret 4 -> 1: com a versao dos dados [0xdd7110] (abs) > 0x3e003,
#   fread de 1 byte em +0xb2 (sem conferir); senao +0xb2 = 0. Prologo mov eax, [0xdd7110] (5 bytes).
# Iguais no cm0102-1080.exe.
# Comparadores: as leituras lado a lado com o arquivo de volta a posicao (compare_cup_extra_data_load do cx02_13:
# integrar depois dele); a gravacao so a original (a prova e a do save).
SAVE = True
CARGA = True
ROTINAS = [('intertoto_cup_load_extra_data', 0x633eb0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('intertoto_cup_save_extra_data', 0x633f50, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xc1, 0xb2, 0x00, 0x00, 0x00]),
           ('uefa_cup_load_extra_data', 0x903300, [0xa1, 0x10, 0x71, 0xdd, 0x00])]
DOCS = ('os dados próprios da Intertoto e da Copa da UEFA no jogo salvo', "the Intertoto and UEFA cups' own data in the saved game",
        [('intertoto_cup_load_extra_data', 'IntertotoCup::load_extra_data', '0x633eb0', 'intertoto_cup.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Intertoto se os vencedores já foram passados à Copa da UEFA (4 bytes); leitura curta: a caixa de erro e 0',
          'reads from the Intertoto temporary file whether the winners were already passed into the UEFA Cup (4 bytes); a short read: the error box and 0'),
         ('intertoto_cup_save_extra_data', 'IntertotoCup::save_extra_data', '0x633f50', 'intertoto_cup.cpp', 'thiscall, ret 4',
          'grava no arquivo temporário da Intertoto se os vencedores já foram passados à Copa da UEFA (4 bytes)',
          'writes into the Intertoto temporary file whether the winners were already passed into the UEFA Cup (4 bytes)'),
         ('uefa_cup_load_extra_data', 'UefaCup::load_extra_data', '0x903300', 'uefa_cup.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Copa da UEFA o número de clubes da rodada preliminar, nos saves de versão nova; nos antigos ele fica 0',
          'reads from the UEFA Cup temporary file the count of clubs in the preliminary round, in newer saves; in older ones it becomes 0')])
CAMPOS = []
INTERESSA = []
