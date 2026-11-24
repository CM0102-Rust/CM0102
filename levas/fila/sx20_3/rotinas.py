# O nome de uma pessoa contra um texto e duas ordens do gerente de listas curtas (shortlist_manager.cpp; leitor sx20,
# rodada 5, 24/11/2026): staff_name_equals (0x84b8b0), shortlist_compare_slots_open_first (0x83b690) e
# compare_id_pair_keys_by_second_id (0x844740). Rotinas inteiras (0x84b8b0..0x84b935 ret; 0x83b690..0x83b6be ret;
# 0x844740..0x84477c ret; o resto e enchimento).
# Quem chama: staff_name_equals, a iniciacao dos jogadores do banco (0x535a80, Rust: a formacao pelo nome, na criacao do
# jogo), o motor de partida (o tecnico "Ruud Gullit", 0x6c63af) e as telas de clube e de pessoa; a ordem das vagas, o
# qsort de Shortlist_Manager::allocate_transfer_budget (0x83b77e; 17.425 chamadas da rotina na cobertura ENG); a ordem
# dos pares, o bsearch/qsort de load (0x83ab12), 0x8420f0, 0x8426d6, update_staff_status (0x843f92) e
# rebuild_club_shortlist (0x84ace5).
# Convencoes conferidas na desmontagem: as tres cdecl; 0x84b8b0(pessoa, texto) -> 0/1 (sem pessoa 0), com 0x53dba0
# database_format_staff_name cdecl(pessoa, buf, 0x1f4, 1) e o strcmp em linha; 0x83b690(a, b) e 0x844740(a, b) -> int.
# Defeitos do original: a ordem das vagas nunca da 0 (precos iguais dao 1), o que o qsort aceita mas deixa a ordem
# dependente da posicao.
ROTINAS = [('staff_name_equals', 0x84b8b0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0xf4, 0x01, 0x00, 0x00]),
           ('shortlist_compare_slots_open_first', 0x83b690, [0x8b, 0x54, 0x24, 0x04, 0x83, 0xc8, 0xff]),
           ('compare_id_pair_keys_by_second_id', 0x844740, [0x8b, 0x54, 0x24, 0x04, 0x56])]
DOCS = ('o nome de uma pessoa contra um texto e duas ordens do gerente de listas curtas',
        "a person's name against a text and two orders of the shortlist manager",
        [('staff_name_equals', 'staff_name_equals', '0x84b8b0', 'shortlist_manager.cpp', 'cdecl',
          'diz se o nome da pessoa (formatado no estilo 1 num buffer de 500 bytes) e exatamente o texto; sem pessoa, 0',
          "says whether the person's name (formatted in style 1 into a 500-byte buffer) is exactly the text; without a person, 0"),
         ('shortlist_compare_slots_open_first', 'shortlist_compare_slots_open_first', '0x83b690', 'shortlist_manager.cpp', 'cdecl',
          'a ordem das vagas de um clube no qsort: vaga com alvo ou com oferta (acima de -1) vai depois de uma sem; duas vagas abertas pelo preco, maior primeiro; precos iguais dao 1',
          "the qsort order of a club's slots: a slot with a target or an offer (above -1) goes after one without; two open slots by price, highest first; equal prices answer 1"),
         ('compare_id_pair_keys_by_second_id', 'compare_id_pair_keys_by_second_id', '0x844740', 'shortlist_manager.cpp', 'cdecl',
          'a ordem dos pares de ids no bsearch/qsort: par nulo antes de tudo; pelo segundo id; com o mesmo segundo id, primeiro id -1 (em qualquer lado) casa (0), senao pelo primeiro id',
          'the order of the id pairs in bsearch/qsort: a null pair before everything; by the second id; with the same second id, a first id of -1 (either side) matches (0), otherwise by the first id')])
