# Os contratos (contract_manager.cpp): tirar o contrato principal (CONTRACT_MANAGER::remove_contract 0x4d0b50: 21
# chamadas, as saídas do clube no manager_manager.cpp 0x690890, as transferências, os empréstimos e as
# aposentadorias), reenviar as ofertas guardadas (CONTRACT_MANAGER::resubmit_held_offers 0x4cf4a0, pela 0x8bfd4a do
# transfer_manager.cpp) e o mês do fim da temporada do clube (contract_manager_season_end_month 0x4d45f0: dez dias
# depois do fim da temporada da nação do clube; nos contratos, nos regens e nas ofertas). A 0x4d45f0 tem quadro SEH
# com duas datas locais e nenhum destrutor no caminho normal (as datas não têm destrutor chamado). Sem sorteio, sem
# leitura de lixo (as datas locais são preenchidas antes de lidas: 0x54a110 grava os 8 bytes). Iguais no
# cm0102-1080.exe. Comparadores: a 0x4d0b50 lado a lado (a folha 0x598370 refeita dá o mesmo); a 0x4cf4a0 só a
# original com o plano (reenvia e libera itens da lista); a 0x4d45f0 pelo valor.
SAVE = False
ROTINAS = [('tirar_o_contrato_principal', 0x4d0b50, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('reenviar_ofertas_guardadas', 0x4cf4a0, [0x55, 0x56, 0x57, 0x8b, 0xf9]),
           ('mes_do_fim_da_temporada_do_clube', 0x4d45f0, [0x6a, 0xff, 0x68, 0x40, 0x6e, 0x95, 0x00])]
DOCS = ('os contratos: tirar o contrato principal, reenviar as ofertas guardadas e o mês do fim da temporada do clube',
        'contracts: removing the main contract, resubmitting the held offers and the month of the club\'s season end',
        [('tirar_o_contrato_principal', 'CONTRACT_MANAGER::remove_contract', '0x4d0b50', 'contract_manager.cpp', 'thiscall, ret 4',
          'libera a entrada do contrato principal da pessoa (−1), põe −1 na vaga dela no registro de 79 bytes e refaz a folha de salários do clube do contrato',
          'frees the person\'s main contract entry (−1), puts −1 in her slot of the 79-byte record and recalculates the wage bill of the contract\'s club'),
         ('reenviar_ofertas_guardadas', 'CONTRACT_MANAGER::resubmit_held_offers', '0x4cf4a0', 'contract_manager.cpp', 'thiscall, ret 4',
          'reenvia como ofertas vivas as ofertas guardadas da pessoa (com a transferência do mesmo clube, estado 0xf vira 8; sem transferência, com −1), tira cada uma da lista e devolve quantas',
          'resubmits the person\'s held offers as live offers (with the transfer of the same club, state 0xf becomes 8; without a transfer, with −1), removes each one from the list and returns how many'),
         ('mes_do_fim_da_temporada_do_clube', 'contract_manager_season_end_month', '0x4d45f0', 'contract_manager.cpp', 'stdcall, ret 4',
          'o mês (0..11) da data dez dias depois do fim da temporada da nação do clube',
          'the month (0..11) of the date ten days after the end of the season of the club\'s nation')])
CAMPOS = [('[0xae2d0c] registro de 79 bytes da pessoa +0: o índice do contrato principal (−1 sem); valor; lê e grava −1 (tirar_o_contrato_principal)',
           '[0xae2d0c] person\'s 79-byte record +0: the main contract index (−1 none); value; reads and writes −1 (tirar_o_contrato_principal)'),
          ('CONTRACT_MANAGER +0: o vetor dos contratos (0x50 bytes; +0 −1 livre, +4 id do clube); ponteiro; lê e grava (tirar_o_contrato_principal)',
           'CONTRACT_MANAGER +0: the contract array (0x50 bytes; +0 −1 free, +4 club id); pointer; reads and writes (tirar_o_contrato_principal)'),
          ('CONTRACT_MANAGER +0x38: a lista das ofertas guardadas (+0 id da pessoa, +0x59 transferência, +0x5d clube, +0x61 byte); ponteiro; lê e grava (reenviar_ofertas_guardadas)',
           'CONTRACT_MANAGER +0x38: the held offer list (+0 person id, +0x59 transfer, +0x5d club, +0x61 byte); pointer; reads and writes (reenviar_ofertas_guardadas)'),
          ('transferência (0x8c61e0) +0x24, +0x2c: clube e estado (0xf vira 8); id e valor; lê e grava (reenviar_ofertas_guardadas)',
           'transfer (0x8c61e0) +0x24, +0x2c: club and state (0xf becomes 8); id and value; reads and writes (reenviar_ofertas_guardadas)'),
          ('clube: a data do fim da temporada pela nação (0x669bd0); valor; lê (mes_do_fim_da_temporada_do_clube)',
           'club: the season end date by nation (0x669bd0); value; reads (mes_do_fim_da_temporada_do_clube)')]
INTERESSA = ['CMRTE', 'SGE']
