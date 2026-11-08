# A ordem dos árbitros (officials_manager.cpp), pedido de 02/11 (agente C, família das pessoas):
# - 0x79d4e0 (officials_manager_compare_nation_then_quality_desc, 84 bytes): a ordem do qsort da 0x79bf60 que decide os
#   árbitros que ficam por nação (os primeiros pela qualidade +0x28 × 50 + +0x22). Milhares de comparações uma vez por
#   carreira (o jogo novo da carreira automática: "cqh_7 descending order" 16384 na carreira de 02/11). Duas mutações.
# - 0x79d480 (a mesma ordem, crescente, 84 bytes): a do qsort da 0x79c5c0 depois dos árbitros novos (os índices +0 saem
#   dessa ordem; "cqh_7 ascending order" 262144). Uma mutação.
# Item de duas rotinas pequenas: a 0x79d410 foi para a cqg_9 e a 0x79c530 (a cópia do registro, escolhida no começo) já
# estava no motor.rs (cpy_9, 261ª).
# Iguais no cm0102-1080.exe (as duas inteiras, conferido byte a byte).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
#   0x79d4e0, 0x79d480: cdecl (ret), 2: ponteiros passados ao qsort (push 0x79d4e0 em 0x79bf7c, push 0x79d480 em
#   0x79c78c).
# Leitura de memória: no empate, as ordens leem o índice +0 dos registros; nos árbitros novos (0x79c5c0, antes de a
#   0x79bf60 numerar) ele é o que o realloc deixou, e a cópia lê o mesmo.
# Comparadores: as duas pela devolução (só leem).
SAVE = False
ROTINAS = [('ordem_dos_arbitros_decrescente', 0x79d4e0, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('ordem_dos_arbitros_crescente', 0x79d480, [0x8b, 0x54, 0x24, 0x04, 0x56])]
DOCS = ('a ordem dos árbitros por nação e qualidade, nos dois sentidos',
        'the order of the officials by nation and quality, both ways',
        [('ordem_dos_arbitros_decrescente', 'officials_manager_compare_nation_then_quality_desc', '0x79d4e0', 'officials_manager.cpp', 'cdecl',
          'ordena os árbitros pela nação e, nela, pela qualidade (+0x28 × 50 + +0x22) da maior para a menor; empate pelo índice, decrescente',
          'orders the officials by nation and, within it, by quality (+0x28 × 50 + +0x22) from highest to lowest; tie by index, descending'),
         ('ordem_dos_arbitros_crescente', 'officials_manager_compare_nation_then_quality_asc', '0x79d480', 'officials_manager.cpp', 'cdecl',
          'ordena os árbitros pela nação e pela qualidade da menor para a maior; empate pelo índice, crescente',
          'orders the officials by nation and by quality from lowest to highest; tie by index, ascending')])
CAMPOS = [('árbitro +0 índice, +0x16 nação (+0 id), +0x22 nota, +0x28 atributo [inferência: os dois da qualidade na ordem]; valor/ponteiro; lê',
           'official +0 index, +0x16 nation (+0 id), +0x22 rating, +0x28 attribute [inference: the two of the quality in the order]; value/pointer; reads')]
INTERESSA = ['CMRTE']
