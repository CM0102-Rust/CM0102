# A gravacao do man_conf.dat (MANAGER_MANAGER::save_man_conf 0x68e920, manager_manager.cpp; leitor sx14b, rodada 5,
# 23-24/11/2026). A rotina inteira (0x68e920..0x68f126, ret 4, com 18 caminhos de erro; o resto ate a proxima fronteira
# e enchimento). Chamada pela gravacao do jogo (0x82f383).
# Convencoes conferidas na desmontagem: thiscall(this, save) ret 4, sem retorno; 0x9326e0 zopen(save, "man_conf.dat",
# 1, 0x16, 4); 0x932e10 zwrite(buf, tamanho, n, z); 0x932b00 zclose(&z); 0x68d1b0 LLIST::get_next thiscall(lista);
# a caixa de erro (0x944cff, 0x90d130, sprintf "v%s %s.%s %d" com nome, extensao e linha, 0x5e8290, [0xb67a34] = 0)
# = show_error_box (transfer_manager.rs); linhas 0x21a (sem save) e 0x222 (sem o item) sem zclose; 0x228, 0x22e,
# 0x234 e as tres de cada lista (contagem, lista mais curta que a contagem, item nao gravado) com zclose.
# Prologo `mov eax, [esp+4]; sub esp, 0x218` (10 bytes, fronteira de instrucao, sem relativo; igual no 1080).
SAVE = True
ROTINAS = [('save_man_conf', 0x68e920, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x18, 0x02, 0x00, 0x00])]
DOCS = ('a gravação do man_conf.dat',
        'the saving of man_conf.dat',
        [('save_man_conf', 'MANAGER_MANAGER::save_man_conf', '0x68e920', 'manager_manager.cpp', 'thiscall, ret 4',
          'grava o objeto da diretoria no man_conf.dat do save: a contagem de clubes, os registros da diretoria (0x49 bytes por clube) e as cinco listas (vagas, saídas, +0xc, ultimatos, apelações), cada uma com a contagem. Sem save ou sem o item, a caixa "Error" do manager_manager.cpp; qualquer falha depois (contagem, nenhum clube, registros, ou em cada lista a contagem, um item faltando ou não gravado) mostra a caixa da linha e fecha o item; no fim fecha o item',
          'writes the board object into the save man_conf.dat: the club count, the board records (0x49 bytes per club) and the five lists (vacancies, departures, +0xc, ultimatums, appeals), each with its count. Without a save or the item, the manager_manager.cpp "Error" box; any later failure (count, no clubs, records, or in each list the count, a missing item or one not written) shows the box of the row and closes the item; at the end it closes the item')])
