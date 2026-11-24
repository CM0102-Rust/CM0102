# Mutacoes (sx12_2). A ordem e a contagem de partes so montam menus (nao voltam para a simulacao): acusadas pelos
# comparadores da rotina, que rodam a cada montagem da barra de menus na carreira de ferias.
# - o qsort sem a ultima linha (a ordem dos menus muda);
# - as nacoes sempre pelo nome curto (a ordem de varias nacoes muda);
# - o menu partido a partir de uma nacao (a carreira ENG tem uma nacao escolhida).
MUTACOES = [
    ('sx12_ordem_sem_a_ultima',
     'exe::crt_qsort(0xb63d38usize, 0x23, 1, 0x66a100usize); // sx12_2: key_nation_compare_entries',
     'exe::crt_qsort(0xb63d38usize, 0x22, 1, 0x66a100usize); // mutation: the last row left out of the sort',
     'sx12_2 key nation order'),
    ('sx12_nome_curto_sempre',
     "exe::utils_string_compare((na + 4) as *const u8, (nb + 4) as *const u8, 1, 0) // sx12_2: the nations' names",
     'exe::utils_string_compare((na + nation::SHORT_NAME) as *const u8, (nb + nation::SHORT_NAME) as *const u8, 1, 0) // mutation: always the short names',
     'sx12_2 nation rows'),
    ('sx12_partes_do_menu',
     '(read_i8(this + 9) > 16) as u8 + 1 // sx12_2: more than 16 nations',
     '(read_i8(this + 9) > 0) as u8 + 1 // mutation: split from one nation on',
     'sx12_2 menu parts'),
]
