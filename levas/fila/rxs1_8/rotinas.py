# As três ordens (qsort) das telas de transferências (transfer_manager.cpp; leitor rxs1, rodada 6, 25/11/2026): as
# transferências passadas pelo valor e pela data, e as futuras de um clube. As três inteiras (0x8e54f0..0x8e5534,
# 0x8e5540..0x8e5584, 0x8e55f0..0x8e5678), iguais no cm0102-1080.exe; contagem 0 na carreira (só as telas ordenam).
# Quem as passa ao qsort: 0x8f5957 (tela das transferências, 0x8e54f0), 0x455fa0 (club_screens, já em Rust, pelo
# exe::crt_qsort com 0x8e5540) e 0x8ccb80 get_club_future_transfer_list (0x8e55f0).
# Convenções conferidas na desmontagem: as três cdecl(a, b) -> int, sem quadro; 0x456770 Date::operator< e 0x456740
# Date::operator> thiscall(data, dia, ano) ret 8; 0x540270 database_compare_staff_by_full_name (já em Rust).
# Comparadores: lado a lado do resultado (compare_order!); funções puras.
TELAS = True
SAVE = False
ROTINAS = [('transfer_manager_compare_past_transfers_by_fee', 0x8e54f0, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('transfer_manager_compare_past_transfers_by_date', 0x8e5540, [0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('transfer_manager_compare_future_transfer_rows', 0x8e55f0, [0x56, 0x8b, 0x74, 0x24, 0x0c])]
DOCS = ('as ordens das transferências passadas (pelo valor e pela data) e das futuras de um clube',
        "the orders of the past transfers (by fee and by date) and of a club's future transfers",
        [('transfer_manager_compare_past_transfers_by_fee', 'transfer_manager_compare_past_transfers_by_fee', '0x8e54f0', 'transfer_manager.cpp', 'cdecl',
          'as linhas de 37 bytes pelo valor (+0x18), o maior primeiro; no empate, a do tipo 2 (+0x1c) vai depois',
          '37-byte rows by the fee (+0x18), the highest first; on a tie a type 2 row (+0x1c) goes after'),
         ('transfer_manager_compare_past_transfers_by_date', 'transfer_manager_compare_past_transfers_by_date', '0x8e5540', 'transfer_manager.cpp', 'cdecl',
          'ponteiros para os registros pela data (+0x1d), a mais antiga primeiro',
          'pointers to the records by the date (+0x1d), the earliest first'),
         ('transfer_manager_compare_future_transfer_rows', 'transfer_manager_compare_future_transfer_rows', '0x8e55f0', 'transfer_manager.cpp', 'cdecl',
          'as linhas de 27 bytes pelo tipo (+0x19), depois pela data (+0) e pelo nome completo da pessoa (+8)',
          '27-byte rows by the kind (+0x19), then the date (+0) and the person\'s full name (+8)')])
