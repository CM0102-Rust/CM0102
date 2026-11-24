# Os dados proprios das copas inglesas no jogo salvo (eng_cc_cup.cpp, eng_fa_cup.cpp; leitor cx02, rodada 5,
# 24/11/2026). Item do SAVE (CARGA): o arquivo temporario de cada copa (Cup::save_cup_to_disk 0x5210f0 / Cup::load_from_disk
# 0x51fc00) chama as virtuais 0x24 (gravar) e 0x20 (ler) com o FILE aberto em CM3_TEMP; os .tmp vao para o .qst.
# - 0x56d2e0 EngCcCup::load_extra_data thiscall(arquivo) ret 4 -> 0/1: fread 0x945bfe (4 pushes; add esp, 0x10 -> cdecl)
#   de 1 byte em +0xb2; curto: a caixa do eng_cc_cup.cpp (linha 0x2e4) e 0. Prologo mov eax, [esp+4]; sub esp, 0x200.
# - 0x570830 EngFaCup::load_extra_data thiscall(arquivo) ret 4 -> 1: fread de +0xb3 e +0xb4 (sem conferir); com a versao
#   dos dados [0xdd7110] (abs) > 0x3e001, fread de +0xb2; senao +0xb2 = 3. Prologo push esi; push edi; mov edi, [esp+0xc].
# - 0x5708a0 EngFaCup::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 (cdecl) de +0xb3, +0xb4 e +0xb2; devolve o
#   eax do ultimo fwrite. Mesmo prologo.
# Iguais no cm0102-1080.exe.
# Comparadores: as leituras lado a lado com o arquivo de volta a posicao (ftell/fseek: o mesmo trecho lido duas vezes),
# os campos, o retorno e a posicao final; a gravacao so a original (a prova e a do save).
SAVE = True
CARGA = True
ROTINAS = [('eng_cc_cup_load_extra_data', 0x56d2e0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('eng_fa_cup_load_extra_data', 0x570830, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('eng_fa_cup_save_extra_data', 0x5708a0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c])]
DOCS = ('os dados próprios das copas inglesas no jogo salvo', "the English cups' own data in the saved game",
        [('eng_cc_cup_load_extra_data', 'EngCcCup::load_extra_data', '0x56d2e0', 'eng_cc_cup.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Copa da Liga inglesa o número de clubes nas copas europeias (que entram mais tarde); leitura curta: a caixa de erro e 0',
          'reads from the League Cup temporary file the count of clubs in the European cups (who enter later); a short read: the error box and 0'),
         ('eng_fa_cup_load_extra_data', 'EngFaCup::load_extra_data', '0x570830', 'eng_fa_cup.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da FA Cup o número de isentos e a escolha de entrar na quinta rodada; nos saves de versão nova, também se a pergunta da quinta rodada já foi feita (nos antigos ela fica 3)',
          'reads from the FA Cup temporary file the exemption count and the fifth-round entry choice; in newer saves also whether the fifth-round question was asked (in older ones it becomes 3)'),
         ('eng_fa_cup_save_extra_data', 'EngFaCup::save_extra_data', '0x5708a0', 'eng_fa_cup.cpp', 'thiscall, ret 4',
          'grava no arquivo temporário da FA Cup o número de isentos, a escolha da quinta rodada e se a pergunta foi feita, na ordem da leitura',
          'writes into the FA Cup temporary file the exemption count, the fifth-round choice and whether the question was asked, in the order the load reads them')])
CAMPOS = []
INTERESSA = []
