# Os dados proprios da Segunda Divisao sueca no jogo salvo (swe_second.cpp; leitor rx09, rodada 6, 25/11/2026). Item do
# SAVE (CARGA): o arquivo temporario da liga (League::save_to_temp_file / load_from_temp_file) chama as virtuais de
# gravar e ler com o FILE aberto. So existe com a base NORMAL: o save da prova tem de ser criado com ela.
# - 0x8936e0 SweSecond::load_extra_data thiscall(arquivo) ret 4 -> 1: fread 0x945bfe(this + 0xee, 4, 0xc, arquivo) (4
#   pushes; add esp, 0x10 -> cdecl), sem conferir.
# - 0x893700 SweSecond::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 do mesmo bloco; devolve o eax do fwrite.
# Prologos mov eax, [esp+4]; add ecx, 0xee (10 bytes); iguais no cm0102-1080.exe.
# Comparadores: a leitura lado a lado com o arquivo de volta a posicao (compare_cup_extra_data_load do cx02_13: integrar
# depois dele); a gravacao so a original (a prova e a do save).
SAVE = True
CARGA = True
BASE = 'NORMAL'
ROTINAS = [('swe_second_load_extra_data', 0x8936e0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xc1, 0xee, 0x00, 0x00, 0x00]),
           ('swe_second_save_extra_data', 0x893700, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xc1, 0xee, 0x00, 0x00, 0x00])]
DOCS = ('os dados próprios da Segunda Divisão sueca no jogo salvo', "the Swedish Second Division's own data in the saved game",
        [('swe_second_load_extra_data', 'SweSecond::load_extra_data', '0x8936e0', 'swe_second.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Segunda sueca os ids dos doze clubes da divisão inferior guardados para subir',
          'reads from the Swedish Second Division temporary file the ids of the twelve lower division clubs kept to go up'),
         ('swe_second_save_extra_data', 'SweSecond::save_extra_data', '0x893700', 'swe_second.cpp', 'thiscall, ret 4',
          'grava no arquivo temporário da Segunda sueca os ids dos doze clubes da divisão inferior guardados para subir',
          'writes into the Swedish Second Division temporary file the ids of the twelve lower division clubs kept to go up')])
CAMPOS = []
INTERESSA = []
