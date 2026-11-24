# O gerente de olheiros no save: o construtor, a carga e a gravacao (scout_manager.cpp; leitor sx19b, rodada 5,
# 24/11/2026): SCOUT_MANAGER::SCOUT_MANAGER (0x7f6760..0x7f6a8f, ret 4), SCOUT_MANAGER::load (0x7f7bb0..0x7f8018,
# ret 4) e SCOUT_MANAGER::save (0x7f8020..0x7f82e3, ret 4).
# DEPENDE DO sx09c_5 (require_game_cd): integrar junto ou depois. Usa o new_player_search (0x7ae390, sx17_1) e as
# PLAYER_SEARCH::load/save (sx17_1) pelo endereco: nao depende deles.
# Chamadas: o construtor pela criacao do jogo (0x828a9a, sem save) e pela carga (0x82bb1a, com o save); a carga pelo
# construtor (0x7f68ac, dados a partir da versao 0x3e025); a gravacao pelo save do jogo (0x82ea57).
# Convencoes conferidas na desmontagem:
# - 0x7f6760 thiscall(gerente, arquivo do save) ret 4 -> this; quadro SEH so para o operator new do vetor; a busca do
#   CD (GetLogicalDriveStringsA, GetDriveTypeA, sprintf "%s%c..." cm0102.exe, 0x4c0aa0 / 0x4c0c10, "Insert CD" pelo
#   0x66f4e0 e 0x5e8290 com 1); no cm0102-1080.exe o jne 0x7f67d1 virou nop nop e o test 0x7f6815 virou xor (a copia le
#   esses bytes do exe que roda: require_game_cd); 0x944cf1 operator new(0x1ae04), 0x9463af
#   vector_constructor_iterator(buscas, 0x2b0, 0xa0, 0x7ae390, 0x7ae4f0); na falta de memoria a caixa (linha 0x65),
#   0x5cc7d0, 0x5a8f60 e exit(-1); 0x7ae570 set_manager, 0x7ae640 set_slot_index, 0x7ae6e0 set_list_type (thiscall,
#   ret 4, ja Rust); 0x7fc150 scout_manager_write_list_name cdecl(k, busca+0xb, 100) (ja Rust). O indice das listas vai
#   num dword da pilha com so o byte baixo escrito; as tres rotinas leem so esse byte.
# - 0x7f7bb0 / 0x7f8020 thiscall(gerente, arquivo do save) ret 4 -> 1/0; 0x9326e0 zopen cdecl(arquivo,
#   "scout_manager.dat", 0 ler / 1 gravar, 0x16, 4); 0x932b00 zclose cdecl(&item); 0x932c30 zread / 0x932e10 zwrite
#   cdecl(buf, tamanho, quantos, item); 0x7b17e0 / 0x7b2360 PLAYER_SEARCH::load/save thiscall(busca, item) ret 4;
#   0x944e46 malloc (17 bytes por pedido).
# Defeitos do original: na carga, a leitura curta da contagem de pedidos (0x38f), a falta de memoria (0x39f) e a
# leitura curta dos pedidos (0x3a7) voltam 0 sem fechar o item do arquivo (as outras saidas fecham); na gravacao, a
# contagem e os pedidos sao gravados sem conferir.
# Comparadores: so a original (CD, alocacoes, arquivo); o construtor confere depois, no jogo novo, as 160 buscas e as
# listas escolhidas = 1; a carga confere capacidade = contagem.
SAVE = True
CARGA = True  # a carga so roda ao carregar: a prova do save carrega o save de ferias com e sem a leva
ROTINAS = [('new_scout_manager', 0x7f6760, [0x6a, 0xff, 0x68, 0x0e, 0x0f, 0x96, 0x00]),
           ('scout_manager_load', 0x7f7bb0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('scout_manager_save', 0x7f8020, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('o gerente de olheiros no save: o construtor, a carga e a gravação',
        'the scout manager in the saved game: the constructor, loading and saving',
        [('new_scout_manager', 'SCOUT_MANAGER::SCOUT_MANAGER', '0x7f6760', 'scout_manager.cpp', 'thiscall, ret 4',
          'monta o gerente de olheiros: zera os jogadores de comparação 1 e 2 dos 16 técnicos humanos, exige o CD do jogo (respeitando o patch sem CD do exe 1080), zera as 16 sessões de edição; com save de versão 0x3e025 ou mais cuja carga responde, nada mais; senão 160 buscas (10 por técnico humano), sem pedidos de observação, e para cada técnico humano as 10 listas com dono, vaga, tipo (lista curta, busca de jogadores, busca de comissão e, da 4ª em diante, olheiros) e o nome traduzido, e a lista escolhida 1',
          'builds the scout manager: zeroes compare players 1 and 2 of the 16 human managers, requires the game CD (honouring the 1080 exe no-CD patch), zeroes the 16 edit sessions; with a save of version 0x3e025 or later whose load answers, nothing more; otherwise 160 searches (10 per human manager), no scout requests, and for each human manager the 10 lists with owner, slot, type (short list, player search, staff search and, from the 4th on, scouts) and the translated name, and the selected list 1'),
         ('scout_manager_load', 'SCOUT_MANAGER::load', '0x7f7bb0', 'scout_manager.cpp', 'thiscall, ret 4',
          'lê o scout_manager.dat do save: o total de buscas (nenhuma deixa o vetor nulo) e cada busca, o total de pedidos de observação (também a capacidade) e os pedidos de 17 bytes, e os 16 bytes das listas escolhidas; fecha o item e devolve 1. Erros: a caixa "Error" com a linha e 0 (três deles sem fechar o item)',
          'reads scout_manager.dat from the save: the search count (none leaves the array null) and each search, the scout request count (also the capacity) and the 17-byte requests, and the 16 selected list bytes; closes the item and returns 1. Errors: the "Error" box with the line and 0 (three of them without closing the item)'),
         ('scout_manager_save', 'SCOUT_MANAGER::save', '0x7f8020', 'scout_manager.cpp', 'thiscall, ret 4',
          'grava o scout_manager.dat no save: o total de buscas e cada busca, o total de pedidos e os pedidos (sem conferir) e os 16 bytes das listas escolhidas; fecha o item e devolve 1. Erros: a caixa "Error" com a linha e 0',
          'writes scout_manager.dat to the save: the search count and each search, the request count and the requests (unchecked) and the 16 selected list bytes; closes the item and returns 1. Errors: the "Error" box with the line and 0')])
CAMPOS = [('gerente de olheiros +8: o total de buscas (160); +0x60: a capacidade do vetor de pedidos de observação; +0x68 e +0xa8: os jogadores de comparação 1 e 2 dos 16 técnicos humanos (16 dwords cada)',
           'scout manager +8: the search count (160); +0x60: the scout request array capacity; +0x68 and +0xa8: compare players 1 and 2 of the 16 human managers (16 dwords each)')]
INTERESSA = []
