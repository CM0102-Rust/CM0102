# Os dados proprios da Primeira Divisao espanhola e da Primeira Liga russa no jogo salvo (spa_first.cpp, rus_prm.cpp;
# leitor cx13, rodada 5, 24/11/2026). Item do SAVE (grava e carrega).
# - 0x8508c0 SpaFirst::load_extra_data thiscall(arquivo) ret 4: nada lido do arquivo; se a nacao da competicao (+4,
#   +0x5d) nao tem o bit 4 de +0x11c (LOWER_DIVISIONS_LOADED), a divisao inferior espanhola nova (operator new 0xee,
#   SpaLower::SpaLower 0x851020 com o ano, thiscall ret 4) em +0xee, senao 0; devolve 1. O quadro SEH so cobre o
#   construtor. Prologo push -1; push imm32 (7 bytes).
# - 0x850950 SpaFirst::save_extra_data thiscall(arquivo) ret 4: com divisao inferior, o slot 0x1c dela
#   (save_to_temp_file) com 1 (o arquivo nao e repassado); sem ela, devolve o eax da entrada (a copia, 0). Prologo mov
#   ecx, [ecx+0xee] (6 bytes).
# - 0x7ec1f0 RusPrm::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 (+0xee, 1, 1, arquivo; cdecl) do byte das
#   duplas do play-off do titulo. Prologo mov eax, [esp+4]; add ecx, 0xee (10 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (gravam ou constroem), a carga com o plano (a
# divisao inferior existe se e so se a nacao nao tem as divisoes inferiores; devolve 1).
# A Espanha e a Russia nao estao no FeriasTesteSave (so a Inglaterra): a prova pede um save com elas.
SAVE = True
CARGA = True
ROTINAS = [('spa_first_load_extra_data', 0x8508c0, [0x6a, 0xff, 0x68, 0xeb, 0x26, 0x96, 0x00]),
           ('spa_first_save_extra_data', 0x850950, [0x8b, 0x89, 0xee, 0x00, 0x00, 0x00]),
           ('rus_prm_save_extra_data', 0x7ec1f0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xc1, 0xee, 0x00, 0x00, 0x00])]
DOCS = ('os dados próprios da Primeira Divisão espanhola e da Primeira Liga russa no jogo salvo',
        'the Spanish First Division\'s and the Russian Premier League\'s own data in the saved game',
        [('spa_first_load_extra_data', 'SpaFirst::load_extra_data', '0x8508c0', 'spa_first.cpp', 'thiscall, ret 4',
          'depois da carga, refaz a divisão inferior espanhola quando a nação não tem as divisões inferiores carregadas (nada é lido do arquivo)',
          'after a load, rebuilds the Spanish lower division when the nation does not have the lower divisions loaded (nothing is read from the file)'),
         ('spa_first_save_extra_data', 'SpaFirst::save_extra_data', '0x850950', 'spa_first.cpp', 'thiscall, ret 4',
          'grava o arquivo temporário da divisão inferior espanhola, quando ela existe',
          'saves the Spanish lower division\'s temporary file, when it exists'),
         ('rus_prm_save_extra_data', 'RusPrm::save_extra_data', '0x7ec1f0', 'rus_prm.cpp', 'thiscall, ret 4',
          'grava o byte das duplas do play-off do título',
          'writes the byte of the title play-off pair count')])
CAMPOS = []
INTERESSA = []
