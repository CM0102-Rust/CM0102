# O gerente de contratos gravado no save e destruído (contract_manager.cpp; leitor rx07, rodada 6, 25/11/2026). Item
# do save (SAVE), no molde do sx12b_6 (névoa: a gravação e o destrutor juntos). Iguais no cm0102-1080.exe.
# - 0x4dc3a0 CONTRACT_MANAGER::save_to_disk thiscall(this, save) ret 4, chamada só pela gravação do jogo (save_game
#   0x82e430, SaveStep 0x1a, Rust, pelo endereço); vai ao ret 4 de 0x4dc906 (o comparador das datas de decisão
#   0x4dc910 vem logo depois, já Rust). Prólogo sub esp, 0x20c (6). O eax da volta é lixo e o chamador não o lê: a
#   cópia é void.
# - 0x4c74f0 CONTRACT_MANAGER::~CONTRACT_MANAGER thiscall(this) ret 0, chamado só pelo free_game_data 0x82ff40 (Rust,
#   pelo endereço; depois o operator delete do objeto): antes de uma carga com um jogo aberto e na volta ao menu.
#   Prólogo push esi; mov esi, ecx; mov eax, [esi] (5).
# Convenções: qsort 0x945333 cdecl(base, n, 0xe, 0x4dc910); zopen 0x9326e0 cdecl(save, "contract.dat", 1, 0x16, 4);
# zwrite 0x932e10 cdecl(buf, tamanho, n, z); zclose 0x932b00 cdecl(&z); LLIST::get_next 0x68d1b0 thiscall (o cursor
# +0xa recebe a cabeça +2 antes); free 0x9452ca; LLIST::~LLIST 0x68d040 thiscall; operator delete 0x944c94. A caixa:
# 0x944cff, 0x90d130 (os argumentos ficam na pilha para o sprintf 0x944c9f "v%s %s.%s %d (%s)"), 0x66f4e0 do título
# "Save Game Error" 0x9af90c e do corpo 0x9af828, 0x5e8290 com 0 (o award_load_error_box).
# Defeito do original: depois de uma escrita que falha, o arquivo fica aberto (sem zclose); a cópia faz o mesmo.
# Comparadores: só a original (grava o item do save; libera). A prova: a gravação do .qst com e sem as rotinas.
SAVE = True
ROTINAS = [('contract_manager_save_to_disk', 0x4dc3a0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('contract_manager_destructor', 0x4c74f0, [0x56, 0x8b, 0xf1, 0x8b, 0x06])]
_F = 'contract_manager.cpp'
DOCS = ('o gerente de contratos gravado no save e destruído',
        'the contract manager written to the saved game and destroyed',
        [('contract_manager_save_to_disk', 'CONTRACT_MANAGER::save_to_disk', '0x4dc3a0', _F, 'thiscall, ret 4',
          'grava o contract.dat: ordena antes as datas de decisão acrescentadas desde a última ordenação (da próxima a tratar até o fim); depois as conversas (contagem e registros de 21 bytes), os contratos (contagem e registros de 80 bytes), as datas de decisão (contagem e registros de 14 bytes), o índice da próxima decisão, as duas listas de propostas (contagem em 2 bytes e propostas de 111 bytes) e o byte de versão 1, e fecha. Cada falha mostra a caixa "Save Game Error" com a sua linha e para, deixando o arquivo aberto',
          'writes contract.dat: first sorts the decision dates added since the last sort (from the next one to process to the end); then the talks (count and 21-byte records), the contracts (count and 80-byte records), the decision dates (count and 14-byte records), the next decision index, the two offer lists (count in 2 bytes and 111-byte offers) and the version byte 1, and closes it. Each failure shows the "Save Game Error" box with its line and stops, leaving the file open'),
         ('contract_manager_destructor', 'CONTRACT_MANAGER::~CONTRACT_MANAGER', '0x4c74f0', _F, 'thiscall, ret 0',
          'destrói o gerente de contratos com o jogo: libera os vetores de contratos, de datas de decisão e de conversas e destrói e apaga as duas listas de propostas',
          'destroys the contract manager with the game: frees the contract, decision date and talks arrays and destroys and deletes the two offer lists')])
CAMPOS = []
INTERESSA = []
