# Gravar as listas curtas e liberar o gerente de listas curtas e os registros de pessoas (shortlist_manager.cpp,
# staff_records.cpp; leitor sx20, rodada 5, 24/11/2026): Shortlist_Manager::save (0x83b370),
# Shortlist_Manager::~Shortlist_Manager (0x839d90) e Staff_Records::~Staff_Records (0x8687c0). Rotinas inteiras
# (0x83b370..0x83b662 ret 4; 0x839d90..0x839e58 ret; 0x8687c0..0x868823 ret).
# DEPENDE DO sx20_5 (integrar junto ou depois): usa SHORTLIST_MANAGER_SOURCE, SHORTLIST_CLUB_ENTRY_SIZE,
# SHORTLIST_SLOT_SIZE e SHORTLIST_SLOTS (o sx20_5 e de simulacao: entra antes ou na mesma leva).
# Quem chama: save, o TRANSFER_MANAGER::save (0x8bd9ee) com o item ja aberto (o retorno nao e lido); os destrutores, o
# destrutor do TRANSFER_MANAGER (0x8bbee0) e o free_game_data (0x83024a, em Rust pelo endereco), que o .qst da prova do
# save roda depois de gravar. Na carreira automatica nenhuma das tres roda.
# Convencoes conferidas na desmontagem: thiscall; 0x83b370(this, item) ret 4 com 0x932e10 zwrite cdecl(dados, tamanho,
# quantos, item), a caixa de erro (shortlist_manager.cpp 0x3b6, 0x3be, 0x3c7, 0x3cf, 0x3db, 0x3e5, 0x3f3) e para; os
# destrutores thiscall(this) ret: 0x9452ca free; 0x58e330 FILE_LLIST::~FILE_LLIST thiscall e 0x944c94 delete.
# Defeitos do original: ~Staff_Records nao zera os ponteiros que libera.
SAVE = True
ROTINAS = [('shortlist_manager_save', 0x83b370, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('shortlist_manager_destructor', 0x839d90, [0x51, 0x53, 0x8b, 0xd9, 0x56, 0x57]),
           ('staff_records_destructor', 0x8687c0, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x3e])]
DOCS = ('gravar as listas curtas e liberar o gerente de listas curtas e os registros de pessoas',
        'writing the shortlists and freeing the shortlist manager and the staff records',
        [('shortlist_manager_save', 'Shortlist_Manager::save', '0x83b370', 'shortlist_manager.cpp', 'thiscall, ret 4',
          'grava as listas curtas no item aberto do save: o numero de pares por pessoa e os pares (8 bytes), as 16 contagens das listas de posicao e a capacidade, as 32 listas de posicao (entradas de 12 bytes, cada uma do tamanho da sua contagem), as entradas de 9 bytes dos clubes e as 40 vagas de 0x25 bytes de cada clube; erro: a caixa e para; os pares por clube nao sao gravados (a carga copia e ordena os por pessoa)',
          'writes the shortlists into the open save item: the by-person pair count and the pairs (8 bytes), the 16 position list counts and the capacity, the 32 position lists (12-byte entries, each as long as its count), the 9-byte club entries and the 40 slots of 0x25 bytes of each club; on error the box and it stops; the by-club pairs are not written (the load copies and sorts the by-person ones)'),
         ('shortlist_manager_destructor', 'Shortlist_Manager::~Shortlist_Manager', '0x839d90', 'shortlist_manager.cpp', 'thiscall',
          'libera o gerente: com tabela de clubes, as vagas de cada clube (zerando) e a tabela (zerando); as 32 listas de posicao e os dois vetores de pares, cada um zerado',
          'frees the manager: with a club table, each club slots (cleared) and the table (cleared); the 32 position lists and both pair arrays, each cleared'),
         ('staff_records_destructor', 'Staff_Records::~Staff_Records', '0x8687c0', 'staff_records.cpp', 'thiscall',
          'libera os registros de pessoas: as listas temporarias de historico de pessoas e de clubes (destrutor e delete), os registros de tecnico e as duas tabelas de cabecas de lista; os ponteiros ficam com o valor',
          'frees the staff records: the person and club history temporary lists (destructor and delete), the manager records and the two list head tables; the pointers keep their values')])
