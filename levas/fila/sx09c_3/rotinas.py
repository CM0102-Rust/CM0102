# O dispute.dat do save: a carga e a gravacao das disputas (DISPUTE::load 0x569910, DISPUTE::save 0x569ae0,
# dispute.cpp; leitor sx09c, rodada 5, 23/11/2026). As duas rotinas inteiras: 0x569910..0x569ad3 (ret 4) e
# 0x569ae0..0x569e24 (ret 4; o resto ate 0x569e30 e enchimento).
# Chamadas: a carga so pelo construtor (0x56980f, com o save); a gravacao pela gravacao do jogo (0x82ef65; o retorno
# nao e usado).
# Convencoes conferidas na desmontagem:
# - 0x569910 thiscall(disputa, save) ret 4 -> al; quadro SEH com uma data local (0x54a110 no registro lido, sem efeito
#   no resultado); a versao dos dados [0xdd7110] em valor absoluto (cdq; xor; sub) comparada com 0x3e046 antes de
#   olhar o save; 0x9326e0 zopen cdecl(save, "dispute.dat", 0, 0x16, 4); 0x932c30 zread cdecl; 0x944e46 malloc;
#   0x944cf1 operator new(0xe) e 0x68d020 LLIST::LLIST; 0x68d0d0 LLIST::add_tail thiscall(lista, dado, 0x11) ret 8;
#   0x932b00 zclose cdecl(&z) em toda saida depois de abrir;
# - 0x569ae0 thiscall(disputa, save) ret 4 -> al; zopen com 1; 0x932e10 zwrite cdecl; 0x68d1b0 LLIST::get_next; a
#   caixa de erro como no show_error_box (transfer_manager.rs), e depois o zclose quando o item esta aberto.
# O dispute.dat: dword = a quantidade de pessoas [0xae2364] na gravacao, essa quantidade de bytes (a marca de disputa
# por pessoa); dword n, n x 17 bytes (as disputas: +0 pessoa, +4 clube, +8 data da audiencia, +0x10 fica no clube).
# Prologos sem salto: 0x569910 push -1; push 0x9580b3 (7); 0x569ae0 mov eax,[esp+4]; sub esp,0x208 (10).
SAVE = True
CARGA = True  # a carga so roda ao carregar: a prova do save carrega o save de ferias com e sem a leva
ROTINAS = [('dispute_load', 0x569910, [0x6a, 0xff, 0x68, 0xb3, 0x80, 0x95, 0x00]),
           ('dispute_save', 0x569ae0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('o dispute.dat do save: a carga e a gravacao das disputas',
        'dispute.dat in the save: loading and saving the disputes',
        [('dispute_load', 'DISPUTE::load', '0x569910', 'dispute.cpp', 'thiscall, ret 4',
          'le o dispute.dat do save: com a versao dos dados (em valor absoluto) abaixo de 0x3e046, sem save ou sem o item, 0; o tamanho da tabela das marcas (0 fecha e devolve 0), a tabela (malloc; falhando, fecha e 0) e os bytes dela, a quantidade de disputas e uma lista nova com elas (17 bytes cada); fecha o item e devolve 1. Leitura curta fecha o item e devolve 0',
          'reads dispute.dat from the save: with the game data version (absolute) below 0x3e046, without a save or the item, 0; the flag table length (0 closes and returns 0), the table (malloc; failing, closes and 0) and its bytes, the dispute count and a new list of them (17 bytes each); closes the item and returns 1. A short read closes the item and returns 0'),
         ('dispute_save', 'DISPUTE::save', '0x569ae0', 'dispute.cpp', 'thiscall, ret 4',
          'grava o dispute.dat: a quantidade de pessoas, a tabela das marcas de disputa (um byte por pessoa), a quantidade e as disputas (17 bytes); fecha o item e devolve 1. Sem save ou sem o item, a caixa "Error" do dispute.cpp (linhas 0xe9, 0xf1) e 0; gravacao curta ou no que falta, a caixa (0xf8, 0xff, 0x108, 0x111, 0x117), o item fechado e 0',
          'writes dispute.dat: the staff count, the dispute flag table (one byte per person), the count and the disputes (17 bytes); closes the item and returns 1. Without a save or the item, the dispute.cpp "Error" box (lines 0xe9, 0xf1) and 0; a short write or a missing node, the box (0xf8, 0xff, 0x108, 0x111, 0x117), the item closed and 0')])
