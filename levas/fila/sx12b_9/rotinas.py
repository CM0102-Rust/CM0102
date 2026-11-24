# Os apelidos que o tecnico humano da as pessoas (human_manager.cpp; leitor sx12b, rodada 5, 23/11/2026).
# DEPENDE do sx12_4 (human_record_or_error, human_record_found): integrar junto ou depois dele.
# Quem chama: add/remove/set pelo menu do elenco e do historico da tela da pessoa (0x88b910); get_nickname pelo formato
# do nome (0x53dba0, 0x53e140, so quando has_nickname disse que ha apelido) e pela tela da pessoa; compare_int_keys pelo
# qsort e bsearch da lista (tambem has_nickname e remove_nickname_from_all, ja em Rust).
# Caminho no jogo para o Lucas: perfil de uma pessoa -> menu -> Apelido (dar, mudar, tirar); o apelido aparece no nome
# dela nas telas e nas noticias.
# Convencoes conferidas na desmontagem (thiscall; tecnico nulo = o humano atual):
# - 0x600090 (pessoa, texto, gerente) ret 0xc; caixas 0x9b3/0x9bf, 0x9c7 (texto > 26); malloc 0x944e46 de 300 e
#   realloc 0x945667 de (capacidade + 10) x 30, cada um fatal sem memoria (0x9d4, 0x9e0: 0x5cc7d0, 0x5a8f60, exit(-1));
#   a capacidade em +0xa6e7; strcpy do texto (nulo nao e conferido); qsort 0x945333 cdecl(lista, n, 0x1e, 0x601a10).
# - 0x6003d0 (pessoa, gerente) ret 8; caixas 0xa02/0xa0e; bsearch 0x946464 cdecl(chave, lista, n, 0x1e, 0x601a10), a
#   chave com o id e o byte +4 zerado; o indice pela divisao sem sinal por 30; puxa as seguintes; a ultima com id -1.
# - 0x6005e0 (pessoa, gerente) ret 8 -> eax; caixas 0xa4b/0xa57.
# - 0x600870 (pessoa, texto, gerente) ret 0xc; caixas 0xaca/0xad6, 0xade.
# - 0x601a10 cdecl(a, b): a nulo -1, b nulo 1, chaves iguais 0, senao a - b.
# Comparadores: add_nickname so a original (aloca/cresce a lista), com a entrada conferida depois; os outros lado a
# lado sobre os campos da lista e a lista (remove, set) ou a resposta (get, compare_int_keys).
# Defeito do original: add_nickname nao procura um apelido existente (a mesma pessoa pode ficar com duas entradas).
TELAS = True
SAVE = False
ROTINAS = [('add_nickname', 0x600090, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('remove_nickname', 0x6003d0, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('get_nickname', 0x6005e0, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('set_nickname', 0x600870, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('compare_int_keys', 0x601a10, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0])]
DOCS = ('os apelidos do técnico humano',
        'the human manager\'s nicknames',
        [('add_nickname', 'HUMAN_MANAGER::add_nickname', '0x600090', 'human_manager.cpp', 'thiscall, ret 0xc',
          'acrescenta o apelido (até 26 letras) que o técnico dá a uma pessoa à lista dele, que cresce de 10 em 10 e é reordenada pelo id da pessoa',
          'adds the nickname (up to 26 characters) the manager gives a person to his list, which grows by ten and is sorted again by the person\'s id'),
         ('remove_nickname', 'HUMAN_MANAGER::remove_nickname', '0x6003d0', 'human_manager.cpp', 'thiscall, ret 8',
          'tira o apelido de uma pessoa da lista do técnico: as seguintes sobem, a última fica vazia e a contagem desce',
          'removes a person\'s nickname from the manager\'s list: the following ones move up, the last is emptied and the count goes down'),
         ('get_nickname', 'HUMAN_MANAGER::get_nickname', '0x6005e0', 'human_manager.cpp', 'thiscall, ret 8',
          'devolve o apelido que o técnico deu à pessoa, ou nada',
          'returns the nickname the manager gave the person, or nothing'),
         ('set_nickname', 'HUMAN_MANAGER::set_nickname', '0x600870', 'human_manager.cpp', 'thiscall, ret 0xc',
          'troca o texto do apelido de uma pessoa na lista do técnico; recusa texto acima de 26 letras',
          'replaces the text of a person\'s nickname in the manager\'s list; refuses text over 26 characters'),
         ('compare_int_keys', 'compare_int_keys', '0x601a10', 'human_manager.cpp', 'cdecl',
          'ordena dois registros pela chave inteira do começo (o id), para qsort e bsearch; nulo primeiro',
          'orders two records by their leading int key (the id), for qsort and bsearch; null first')])
