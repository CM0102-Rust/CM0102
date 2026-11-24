# O fine.dat do save: a carga e a gravacao do gerente das multas (Fine::load 0x5a5180, Fine::save 0x5a53d0, Fine.cpp;
# leitor sx09c, rodada 5, 23/11/2026). As duas rotinas inteiras: 0x5a5180..0x5a53bb (ret 4) e 0x5a53d0..0x5a5812
# (ret 4; o resto ate 0x5a5820 e enchimento).
# Chamadas: a carga so pelo construtor (0x5a4ff3, com o save); a gravacao pela gravacao do jogo (0x82eec1, com o
# ponteiro do conteiner na pilha do chamador; o retorno nao e usado).
# Convencoes conferidas na desmontagem:
# - 0x5a5180 thiscall(fine, save) ret 4 -> al; quadro SEH com duas datas locais (0x54a110 Date::Date nos registros
#   lidos; sem efeito no resultado); 0x9326e0 zopen cdecl(save, "fine.dat", 0, 0x16, 4); 0x932c30 zread cdecl(buf,
#   tamanho, quantos, z); 0x944cf1 operator new cdecl(0xe) e 0x68d020 LLIST::LLIST (nulo se falha); 0x68d0d0
#   LLIST::add_tail thiscall(lista, dado, tamanho) ret 8; 0x944e46 malloc cdecl (sem conferir); 0x932b00 zclose
#   cdecl(&z);
# - 0x5a53d0 thiscall(fine, save) ret 4 -> al; zopen com 1; 0x932e10 zwrite cdecl(buf, tamanho, quantos, z); 0x68d1b0
#   LLIST::get_next; a caixa de erro: 0x944cff _splitpath, 0x90d130 cdecl(nome, ext, linha) cujos argumentos ficam na
#   pilha para o sprintf (0x944c9f, "v%s %s.%s %d" em 0xae24d0), 0x5e8290 ("Error", texto, 0) e [0xb67a34] = 0
#   (show_error_box, transfer_manager.rs); nos erros depois de abrir, a caixa e depois o zclose.
# O fine.dat: dword n, n x 14 bytes (multas); dword = a quantidade de clubes [0xae235c] na gravacao, essa quantidade
# de bytes (multas por clube); dword m, m x 17 bytes (retornos do tecnico). A carga le o tamanho da tabela do arquivo.
# Prologos sem salto: 0x5a5180 push -1; push 0x959306 (7); 0x5a53d0 mov eax,[esp+4]; sub esp,0x208 (10).
SAVE = True
CARGA = True  # a carga so roda ao carregar: a prova do save carrega o save de ferias com e sem a leva
ROTINAS = [('fine_load', 0x5a5180, [0x6a, 0xff, 0x68, 0x06, 0x93, 0x95, 0x00]),
           ('fine_save', 0x5a53d0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('o fine.dat do save: a carga e a gravacao das multas',
        'fine.dat in the save: loading and saving the fines',
        [('fine_load', 'Fine::load', '0x5a5180', 'Fine.cpp', 'thiscall, ret 4',
          'le o fine.dat do save: sem save ou sem o item, 0; a quantidade de multas e uma lista nova com elas (14 bytes cada), o tamanho da tabela das multas por clube e os bytes dela (malloc, sem conferir), a quantidade de retornos do tecnico e uma lista nova com eles (17 bytes cada); fecha o item e devolve 1. Leitura curta fecha o item e devolve 0 (o construtor refaz tudo vazio)',
          'reads fine.dat from the save: without a save or the item, 0; the fine count and a new list of them (14 bytes each), the length of the per-club fine count table and its bytes (malloc, unchecked), the coach feedback count and a new list of them (17 bytes each); closes the item and returns 1. A short read closes the item and returns 0 (the constructor starts again empty)'),
         ('fine_save', 'Fine::save', '0x5a53d0', 'Fine.cpp', 'thiscall, ret 4',
          'grava o fine.dat: a quantidade e as multas (14 bytes), a quantidade de clubes e a tabela das multas por clube (um byte cada), a quantidade e os retornos do tecnico (17 bytes); fecha o item e devolve 1. Sem save ou sem o item, a caixa "Error" do Fine.cpp (linhas 0x109, 0x111) e 0; gravacao curta ou no que falta, a caixa (0x119, 0x123, 0x129, 0x132, 0x13a, 0x143, 0x14d, 0x153), o item fechado e 0',
          'writes fine.dat: the count and the fines (14 bytes), the club count and the per-club fine count table (one byte each), the count and the coach feedback records (17 bytes); closes the item and returns 1. Without a save or the item, the Fine.cpp "Error" box (lines 0x109, 0x111) and 0; a short write or a missing node, the box (0x119, 0x123, 0x129, 0x132, 0x13a, 0x143, 0x14d, 0x153), the item closed and 0')])
