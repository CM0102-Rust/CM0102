# O gerente de transferências gravado no save e destruído (transfer_manager.cpp; leitor rxs1, rodada 6, 25/11/2026).
# Item do save (SAVE), no molde do rx07_4 (a gravação e o destrutor juntos). Iguais no cm0102-1080.exe.
# - 0x8bd4b0 TRANSFER_MANAGER::save_to_disk thiscall(this, save) ret 4, chamada só pela gravação do jogo (save_game,
#   SaveStep 0x1c, Rust, pelo endereço); prólogo sub esp, 0x208 (6). O retorno não é lido: a cópia é void.
# - 0x8bbe20 TRANSFER_MANAGER::~TRANSFER_MANAGER thiscall(this) ret 0, chamado só pelo free_game_data (Rust, pelo
#   endereço; depois o operator delete do objeto): antes de uma carga com um jogo aberto e na volta ao menu. Prólogo
#   push -1; push 0x9639e3 (7).
# Convenções: zopen 0x9326e0 cdecl(save, "transfer.dat", 1, 0x16, 4); zwrite 0x932e10 cdecl(buf, tamanho, n, z); zclose
# 0x932b00 cdecl(&z); TRANSFER_OFFER::save_to_disk 0x8e5b30 thiscall(oferta, z) ret 4 e SHORTLIST_MANAGER::save_to_disk
# 0x83b370 thiscall(gerente, z) ret 4, sem conferir o retorno; LLIST::get_next 0x68d1b0 (o cursor +0xa recebe a cabeça
# +2 antes); as contagens das listas em 2 bytes de um dword local (lixo em cima, só 2 bytes vão). O destrutor:
# TRANSFER_OFFER::~ 0x8e5710, SHORTLIST_MANAGER::~ 0x839d90, TRANSFER_RULE::~ 0x8e8540, LLIST::~ 0x68d040, operator
# delete 0x944c94, free 0x9452ca, o vector destructor iterator 0x9462bb(this+0x18, 0xe, 0x96, 0x68d040). A caixa: o
# award_load_error_box com transfer_manager.cpp (0xa8d514), "Save Game Error" 0x9af90c e 0x9af828.
# A gravação mexe na memória antes de gravar: a capacidade dos vetores de ofertas e de negociações passa ao último
# lugar usado + 1, e a contagem de ofertas é recontada (a de negociações não).
# Defeito do original: as escritas dos itens das 150 listas da agenda, das ofertas e das listas curtas não são
# conferidas (uma falha passa calada); a cópia faz o mesmo.
# Comparadores: só a original (grava o item do save; libera). A prova: a gravação do .qst com e sem as rotinas.
SAVE = True
ROTINAS = [('transfer_manager_save_to_disk', 0x8bd4b0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('transfer_manager_destructor', 0x8bbe20, [0x6a, 0xff, 0x68, 0xe3, 0x39, 0x96, 0x00])]
_F = 'transfer_manager.cpp'
DOCS = ('o gerente de transferências gravado no save e destruído',
        'the transfer manager written to the saved game and destroyed',
        [('transfer_manager_save_to_disk', 'TRANSFER_MANAGER::save_to_disk', '0x8bd4b0', _F, 'thiscall, ret 4',
          'grava o transfer.dat: a capacidade do vetor de ofertas (antes cortada ao último lugar usado + 1), a contagem das ofertas vivas (recontada) e cada oferta; o mesmo para as negociações (a contagem como está); as 150 listas da agenda (50 dias × 3 períodos); o dia, a data e o período correntes; as listas curtas; o próximo número de série; as transferências passadas e futuras; o registro de cada pessoa; as prestações; as porcentagens de venda e o dinheiro por jogos (clube e seleção); os empréstimos da temporada de cada clube; a fila das transferências livres; as permissões de trabalho e as negociações de empréstimo. Cada falha mostra a caixa "Save Game Error" com a sua linha e fecha o arquivo',
          "writes transfer.dat: the offer array's capacity (first cut to the last used place + 1), the count of live offers (recounted) and each offer; the same for the negotiations (the count as kept); the 150 schedule lists (50 days × 3 slices); the current day, date and slice; the shortlists; the next serial; the past and future transfers; each person's record; the instalments; the sell-on percentages and the appearance money (club and international); each club's season loans; the free transfer queue; the work permits and the loan negotiations. Each failure shows the \"Save Game Error\" box with its line and closes the file"),
         ('transfer_manager_destructor', 'TRANSFER_MANAGER::~TRANSFER_MANAGER', '0x8bbe20', _F, 'thiscall, ret 0',
          'destrói o gerente de transferências com o jogo: as ofertas e as negociações e os seus vetores, as transferências passadas, o gerente das listas curtas, os registros das pessoas, os empréstimos da temporada, os vetores de dinheiro e as transferências futuras, as 28 regras, as janelas e as permissões, a lista das negociações de empréstimo e as listas embutidas',
          'destroys the transfer manager with the game: the offers and negotiations and their arrays, the past transfers, the shortlist manager, the person records, the season loans, the money arrays and the future transfers, the 28 rules, the windows and the permits, the loan negotiations list and the inline lists')])
CAMPOS = []
INTERESSA = []
