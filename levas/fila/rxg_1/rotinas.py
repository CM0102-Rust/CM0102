# The transfer manager loaded from the saved game (transfer_manager.cpp; reader rxg, round 6, 25/11/2026).
# Item SAVE + CARGA, after rxs1_11 (INTEGRAR_ANTES: TRANSFER_MANAGER_CPP, TRANSFER_DAT, SCHEDULE_BUCKETS). Same bytes
# in cm0102-1080.exe.
# - 0x8bc020 TRANSFER_MANAGER::load_from_disk thiscall(this, save) ret 4, returns 1/0; called only by the loading
#   constructor (0x8bbd25, right after [this+0x85e] = -1). Prologue push -1; push 0x963a30 (7 bytes, SEH frame for the
#   `new` of the offers, the negotiations, the Shortlist_Manager and the loans list; no relative call). Last ret at
#   0x8bd413, the error tails to 0x8bd4a9.
# - Calls: zopen 0x9326e0 cdecl(save, "transfer.dat", 0, 0x16, 4); zread 0x932c30 cdecl(buf, size, n, z); zclose
#   0x932b00 cdecl(&z); calloc 0x945501 cdecl(n, 4); malloc 0x944e46; operator new 0x944cf1; TRANSFER_OFFER::TRANSFER_OFFER
#   0x8e5680 thiscall; TRANSFER_OFFER::load 0x8e5790 thiscall(offer, z, offers | 0) ret 8 (sx21b_5); Shortlist_Manager
#   0x839a80 thiscall(mem, z) ret 4 (sx20_5); LLIST::LLIST 0x68d020; LLIST::add_tail 0x68d0d0 thiscall(list, data, size)
#   ret 8. The local STAFF_CONTRACT (0x85e410, 0x50 bytes at esp+0x23c) is only the buffer of the loan negotiation
#   records: the copy uses plain bytes (each read overwrites it whole).
# - Boxes: "Load Game Error" + "The save game is corrupted" (award_load_error_box, LOAD_ERROR), "Memory Allocation
#   Error" (MEMORY_ERROR), and the plain "Error" (tel26_error_box, line 0x20a, the next serial; it clears 0xb67a34).
# Defects of the original (DEFEITOS): a failed `new` of an offer/negotiation shows the memory box and stops those
# records but the load goes on; the failed reads of the club season loans (0x2ca), the free transfer count (0x2d2) and
# a free transfer record (0x2e6) close the file but return 1, leaving the work permits and the loan negotiations
# unread; the schedule slot reads and the old 5-byte free transfer reads are unchecked.
# Comparer: only the original (allocates and reads the stream). Proof: the load of the holiday save with and without.
SAVE = True
CARGA = True
ROTINAS = [('transfer_manager_load_from_disk', 0x8bc020, [0x6a, 0xff, 0x68, 0x30, 0x3a, 0x96, 0x00])]
_F = 'transfer_manager.cpp'
DOCS = ('o gerente de transferências carregado do save',
        'the transfer manager loaded from the saved game',
        [('transfer_manager_load_from_disk', 'TRANSFER_MANAGER::load_from_disk', '0x8bc020', _F, 'thiscall, ret 4',
          'lê o transfer.dat do save na ordem em que a gravação o escreve: as ofertas (capacidade, contagem, o vetor por calloc e cada oferta na sua vaga) e as negociações (cada uma na vaga que a sua oferta indica); as 150 listas da agenda (50 dias × 3 períodos, as vagas lidas sem conferir); o dia, a data e o período correntes; um gerente de listas curtas novo, construído do mesmo arquivo; o próximo número de série; as transferências passadas (0x25 bytes) e futuras (12), a capacidade lida e copiada para a contagem; o registro de cada pessoa; as prestações; as porcentagens de venda e o dinheiro por jogos (clube e seleção); os empréstimos da temporada de cada clube; a fila das transferências livres (5 bytes e compensação -1 nos saves de versão abaixo de 0x3e003); as permissões de trabalho e as negociações de empréstimo (uma lista nova quando a contagem não é 0). Devolve 1; uma falha mostra a caixa da sua linha, fecha o arquivo e devolve 0. Defeitos do original: falha no new de uma oferta mostra a caixa de memória e segue; as falhas dos empréstimos da temporada e das transferências livres devolvem 1 sem ler o resto',
          "reads transfer.dat from the saved game in the order the save writes it: the offers (capacity, count, the array by calloc and each offer in its slot) and the negotiations (each in the slot its offer names); the 150 schedule lists (50 days × 3 slices, the slot reads unchecked); the current day, date and slice; a new shortlist manager built from the same file; the next serial; the past (0x25 bytes) and future (12) transfers, the capacity read and copied to the count; each person's record; the instalments; the sell-on percentages and the appearance money (club and international); each club's season loans; the free transfer queue (5 bytes and compensation -1 in saves below data version 0x3e003); the work permits and the loan negotiations (a new list when the count is not 0). Returns 1; a failure shows its line's box, closes the file and returns 0. Defects of the original: a failed new of an offer shows the memory box and goes on; the failures of the season loans and the free transfers return 1 without reading the rest")])
CAMPOS = []
INTERESSA = []
