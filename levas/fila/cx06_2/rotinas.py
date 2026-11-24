# Os dados proprios das eliminatorias da Eurocopa no jogo salvo (euro_champ_qual.cpp; leitor cx06, rodada 5,
# 24/11/2026). Item do SAVE (CARGA): o arquivo temporario de cada liga (League::save_to_temp_file /
# League::load_from_temp_file 0x687b10) chama as virtuais 0x24 (gravar) e 0x20 (ler) com o FILE aberto. As eliminatorias
# sao criadas por create_other_int_comps em toda carreira (inclusive a ENG), entao estao no save de ferias; antes de
# set/2002 a lista dos resultados de grupo (+0x10) esta vazia (contagem 0).
# - 0x580130 EuroChampQual::load_extra_data thiscall(arquivo) ret 4 -> 0/1: fread 0x945bfe (cdecl) da contagem (4
#   bytes); cada resultado (registro de partida, 0x4f bytes) com os ponteiros refeitos pelos ids (mandante +0x1c e
#   visitante +0x20 pela tabela de clubes 0x245; a competicao +0x14 pela tabela 0x6b; o estadio +0x18 pela tabela 0x4e
#   ou 0 com -1) e LLIST::add_tail 0x68d0d0 (thiscall, ret 8) na lista +0x10; depois +0xf6 (1 byte) e +0xfb (4 bytes).
#   Leitura curta ou id -1 do mandante, do visitante ou da competicao: a caixa (linhas 0x64a, 0x652, 0x65a, 0x661,
#   0x669, 0x683, 0x68a) e 0. Prologo sub esp, 0x254 (6 bytes).
# - 0x580410 EuroChampQual::save_extra_data thiscall(arquivo) ret 4: fgetpos 0x946c1e (cdecl) da posicao da contagem,
#   fwrite 0x945ce6 (cdecl) da contagem 0, cada resultado da lista (o cursor +0xa volta a cabeca +2; LLIST::get_next
#   0x68d1b0) sem conferir, fgetpos do fim, fsetpos 0x946c06 (cdecl) de volta, a contagem de verdade, fsetpos do fim,
#   +0xf6 e +0xfb. Falha: a caixa (linhas 0x6a2, 0x6ac, 0x6bb, 0x6c2, 0x6c7, 0x6ce, 0x6d5, 0x6dc); o original devolve o
#   eax da caixa, a copia 0. Prologo sub esp, 0x214 (6 bytes).
# Iguais no cm0102-1080.exe.
# Comparadores: a leitura so a original (acrescenta a lista), com o plano (o arquivo andou 4 + n x 0x4f + 5 e a lista
# cresceu n); a gravacao so a original (a prova e a do save).
SAVE = True
CARGA = True
ROTINAS = [('euro_champ_qual_load_extra_data', 0x580130, [0x81, 0xec, 0x54, 0x02, 0x00, 0x00]),
           ('euro_champ_qual_save_extra_data', 0x580410, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00])]
DOCS = ('os dados próprios das eliminatórias da Eurocopa no jogo salvo', "the European Championship qualifiers' own data in the saved game",
        [('euro_champ_qual_load_extra_data', 'EuroChampQual::load_extra_data', '0x580130', 'euro_champ_qual.cpp', 'thiscall, ret 4',
          'lê do jogo salvo os resultados de grupo guardados das eliminatórias (cada partida com os ponteiros refeitos pelos ids do mandante, do visitante, da competição e do estádio), o número de sedes europeias e a marca dos classificados; leitura curta ou id −1 do mandante, do visitante ou da competição: a caixa de erro e 0',
          'reads the qualifiers\' stored group results from the saved game (each fixture with its pointers rebuilt from the home, away, competition and stadium ids), the European hosts count and the qualified flag; a short read or a home, away or competition id of −1: the error box and 0'),
         ('euro_champ_qual_save_extra_data', 'EuroChampQual::save_extra_data', '0x580410', 'euro_champ_qual.cpp', 'thiscall, ret 4',
          'grava no jogo salvo os resultados de grupo guardados das eliminatórias (a contagem é escrita depois, por cima de um zero, voltando a posição do arquivo), o número de sedes europeias e a marca dos classificados; falha de escrita ou de posição: a caixa de erro',
          'writes the qualifiers\' stored group results to the saved game (the count is written afterwards over a zero, going back in the file), the European hosts count and the qualified flag; a failed write or position: the error box')])
CAMPOS = []
INTERESSA = []
