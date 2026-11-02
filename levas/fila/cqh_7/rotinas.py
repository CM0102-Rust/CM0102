# A ordem dos árbitros e a cópia do registro (officials_manager.cpp), pedido de 02/11 (agente C, família das pessoas):
# - 0x79d4e0 (officials_manager_compare_nation_then_quality_desc, 84 bytes): a ordem do qsort da 0x79bf60 que decide os
#   árbitros que ficam por nação (os primeiros pela qualidade +0x28 × 50 + +0x22). Milhares de comparações uma vez por
#   carreira (o jogo novo da carreira automática). Duas mutações nela.
# - 0x79d480 (a mesma ordem, crescente, 84 bytes): a do qsort da 0x79c5c0 depois dos árbitros novos (os índices +0
#   saem dessa ordem). Uma mutação nela.
# - 0x79d410 (officials_manager_compare_comp_level, 104 bytes): a ordem das vagas de competição de cada nação (qsort da
#   0x79bf60): passageira.
# - 0x79c530 (Official::operator=, 140 bytes): a cópia do registro dos árbitros que ficam (pela 0x79bf60): passageira.
# Iguais no cm0102-1080.exe (as quatro inteiras, conferido byte a byte).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
#   0x79d4e0, 0x79d480, 0x79d410: cdecl (ret), 2: ponteiros passados ao qsort (push 0x79d4e0 em 0x79bf7c, push
#   0x79d480 em 0x79c78c, push 0x79d410 em 0x79c3f7).
#   0x79c530: thiscall (mov ecx, ebp = o destino; push origem em 0x79c04d), 1 (ret 4), devolve o destino. Dentro:
#   0x417c10 thiscall (lea ecx, destino + 0xc; push temporária; push origem + 0xc), 2.
# Leitura de memória: no empate, as ordens leem o índice +0 dos registros; nos árbitros novos (0x79c5c0, antes de a
#   0x79bf60 numerar) ele é o que o realloc deixou, e a cópia lê o mesmo.
# Comparadores: as três ordens pela devolução (só leem); 0x79c530 lado a lado no destino e na devolução.
SAVE = False
ROTINAS = [('ordem_dos_arbitros_decrescente', 0x79d4e0, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('ordem_dos_arbitros_crescente', 0x79d480, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('ordem_das_vagas_de_competicao', 0x79d410, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('copiar_arbitro', 0x79c530, [0x83, 0xec, 0x08, 0x56, 0x57])]
DOCS = ('a ordem dos árbitros (por nação e qualidade, nos dois sentidos), a ordem das vagas pelo nível da competição e a cópia do registro do árbitro',
        'the order of the officials (by nation and quality, both ways), the order of the slots by competition level and the official record copy',
        [('ordem_dos_arbitros_decrescente', 'officials_manager_compare_nation_then_quality_desc', '0x79d4e0', 'officials_manager.cpp', 'cdecl',
          'ordena os árbitros pela nação e, nela, pela qualidade (+0x28 × 50 + +0x22) da maior para a menor; empate pelo índice, decrescente',
          'orders the officials by nation and, within it, by quality (+0x28 × 50 + +0x22) from highest to lowest; tie by index, descending'),
         ('ordem_dos_arbitros_crescente', 'officials_manager_compare_nation_then_quality_asc', '0x79d480', 'officials_manager.cpp', 'cdecl',
          'ordena os árbitros pela nação e pela qualidade da menor para a maior; empate pelo índice, crescente',
          'orders the officials by nation and by quality from lowest to highest; tie by index, ascending'),
         ('ordem_das_vagas_de_competicao', 'officials_manager_compare_comp_level', '0x79d410', 'officials_manager.cpp', 'cdecl',
          'ordena as vagas de árbitros de uma nação pelo nível da competição (as inválidas por último ou primeiro), empate pelo id',
          'orders the official slots of a nation by competition level (the invalid ones last or first), tie by id'),
         ('copiar_arbitro', 'Official::operator=', '0x79c530', 'officials_manager.cpp', 'thiscall',
          'copia o registro de um árbitro (com a data de nascimento) e devolve o destino',
          'copies an official record (with the birth date) and returns the destination')])
CAMPOS = [('árbitro +0 índice, +0x16 nação (+0 id), +0x22 nota, +0x28 atributo [inferência: os dois da qualidade na ordem]; valor/ponteiro; lê (as ordens); o registro inteiro (0x2b); grava (copiar_arbitro)',
           'official +0 index, +0x16 nation (+0 id), +0x22 rating, +0x28 attribute [inference: the two of the quality in the order]; value/pointer; reads (the orders); the whole record (0x2b); writes (copiar_arbitro)'),
          ('vaga de árbitros (16 bytes) +0xc a competição; competição ([0xae23d0], 0x6b bytes) +0x69 o nível; [0xae2378] a última competição; valor; lê',
           'official slot (16 bytes) +0xc the competition; competition ([0xae23d0], 0x6b bytes) +0x69 the level; [0xae2378] the last competition; value; reads')]
INTERESSA = ['CMRTE']
