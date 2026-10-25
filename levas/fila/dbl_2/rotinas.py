# Os vínculos de opinião do contrato (registro de 79 bytes da tabela [0xae2d0c], dois vínculos: id da outra pessoa e
# valor em +0x27/+0x37 e +0x3b/+0x4b). A 0x549c90 roda na criação a cada contrato gerado (create_game →
# CONTRACT_MANAGER::initialise_all_staff_contracts 0x4c7560 → STAFF_CONTRACT::generate_initial_contract 0x85e960):
# a tabela de contratos vai para o .qst da prova do save. A 0x549cf0 (eventos do contrato, 0x867700) e a 0x549d60
# (queixas que passam, 0x867240/0x8674e0) rodam na carreira; nos dias até a data de início só se algum evento de
# contrato disparar (FinanceTable::daily_update → 0x867700). Sem SEH, sem sorteio, sem variável local, sem chamadas.
SAVE = True
ROTINAS = [('limpar_vinculos_de_opiniao', 0x549c90, [0x8b, 0x54, 0x24, 0x04, 0x85, 0xd2]),
           ('juntar_vinculo_de_opiniao', 0x549cf0, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('tirar_vinculo_de_opiniao', 0x549d60, [0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9])]
DOCS = ('os vínculos de opinião do contrato: limpar, juntar e tirar',
        'the contract opinion links: clear, add and remove',
        [('limpar_vinculos_de_opiniao', 'staff_clear_opinion_links', '0x549c90', 'Database.cpp', 'cdecl',
          '−1 nos dois vínculos (id e valor) do contrato da pessoa; pessoa nula ou id ≥ [0xae2364]: nada',
          '−1 in both links (id and value) of the person contract; null person or id ≥ [0xae2364]: nothing'),
         ('juntar_vinculo_de_opiniao', 'staff_add_opinion_link', '0x549cf0', 'Database.cpp', 'cdecl',
          'grava o id da outra pessoa e o valor no primeiro vínculo vazio (+0x27 = −1) ou, senão, no segundo, por cima; as duas pessoas têm de ser válidas',
          'writes the other person id and the value into the first empty link (+0x27 = −1) or else into the second, overwriting; both persons must be valid'),
         ('tirar_vinculo_de_opiniao', 'staff_remove_opinion_link', '0x549d60', 'Database.cpp', 'cdecl',
          'esvazia o primeiro vínculo com o valor dado ou, se não, o segundo',
          'empties the first link with the given value or, if not, the second')])
CAMPOS = [('[0xae2d0c] (contratos, 79 bytes por pessoa) +0x27/+0x3b: id da pessoa do vínculo; +0x37/+0x4b: valor do vínculo; inteiros, −1 = vazio; lê e grava (as três)',
           '[0xae2d0c] (contracts, 79 bytes per person) +0x27/+0x3b: link person id; +0x37/+0x4b: link value; integers, −1 = empty; reads and writes (all three)'),
          ('TStaff +0: id; inteiro; lê (as três). [0xae2364]: quantidade de pessoas; lê', 'TStaff +0: id; integer; reads (all three). [0xae2364]: staff count; reads')]
INTERESSA = ['SGE', 'CMRTE']
