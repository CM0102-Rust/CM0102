# Os contratos da transferência (transfer_manager.cpp): a permissão de trabalho na mudança de clube
# (TRANSFER_MANAGER::update_work_permit_on_move 0x8c7e20, pela transferência livre 0x8c1af0 e pela troca de clube
# 0x8c9720), concluir a oferta de contrato aceita (TRANSFER_MANAGER::complete_accepted_contract_offer 0x8c38b0, pela
# escolha das ofertas de contrato 0x4ce4e0) e oferecer contrato ao jogador de uma oferta de transferência
# (TRANSFER_MANAGER::offer_contract_to_offer_player 0x8c1e40, pela 0x8c63a0 e pelas notícias 0x784100). Quadros SEH de
# uma data, de uma TRANSFER_OFFER (construtor 0x8e5680 e destrutor 0x8e5710, chamados na ordem da original) e de um
# STAFF_CONTRACT local (0x85e410, sem destrutor). Iguais no cm0102-1080.exe. Comparadores só da original (crescem
# vetores, alocam, pagam e postam notícias); as cópias são provadas pelos ramos e pelas mutações na carreira.
SAVE = False
ROTINAS = [('permissao_de_trabalho_na_mudanca', 0x8c7e20, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('concluir_oferta_de_contrato_aceita', 0x8c38b0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('oferecer_contrato_ao_jogador_da_oferta', 0x8c1e40, [0x6a, 0xff, 0x68, 0xc8, 0x3b, 0x96, 0x00])]
DOCS = ('os contratos da transferência: a permissão de trabalho na mudança de clube, concluir a oferta de contrato aceita e oferecer contrato ao jogador da oferta',
        'the transfer contracts: the work permit on a club move, completing an accepted contract offer and offering a contract to the offer\'s player',
        [('permissao_de_trabalho_na_mudanca', 'TRANSFER_MANAGER::update_work_permit_on_move', '0x8c7e20', 'transfer_manager.cpp', 'thiscall, ret 8',
          'precisando de permissão no clube novo: atualiza a que tem (nação, ano) ou dá uma nova (0x8c7070); sem precisar: tira a permissão que tiver e reordena o vetor',
          'when the new club needs a permit: updates the existing one (nation, year) or grants a new one (0x8c7070); otherwise: removes any permit and re-sorts the array'),
         ('concluir_oferta_de_contrato_aceita', 'TRANSFER_MANAGER::complete_accepted_contract_offer', '0x8c38b0', 'transfer_manager.cpp', 'thiscall, ret 4',
          'sem a permissão de trabalho, a notícia da oferta que caiu ou da permissão recusada e 0; senão assina (0x4cf720), o clube paga as luvas e 1',
          'without the work permit, the news of the collapsed offer or the refused permit and 0; otherwise signs (0x4cf720), the club pays the signing-on fee and 1'),
         ('oferecer_contrato_ao_jogador_da_oferta', 'TRANSFER_MANAGER::offer_contract_to_offer_player', '0x8c1e40', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'monta o contrato do clube (0x4cda70, fator 1) e acrescenta a oferta de contrato; sem contrato, retira a oferta (jogador principal) ou tira o jogador da troca somando o valor dele',
          'builds the club\'s contract (0x4cda70, factor 1) and adds the contract offer; without one, withdraws the offer (main player) or drops the player from the exchange adding his value')])
CAMPOS = [('TRANSFER_MANAGER +0x93a / +0x93e: as permissões de trabalho (14 bytes: +0 pessoa, +4 e +6, +8 ano, +0xa nação) e a conta; ponteiro e valor; lê e grava (permissao_de_trabalho_na_mudanca)',
           'TRANSFER_MANAGER +0x93a / +0x93e: the work permits (14 bytes: +0 person, +4 and +6, +8 year, +0xa nation) and the count; pointer and value; reads and writes (permissao_de_trabalho_na_mudanca)'),
          ('TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa, bit 0x20 (tem permissão); valor; lê e grava (permissao_de_trabalho_na_mudanca)',
           'TRANSFER_MANAGER +0x86e: person\'s 8-byte record, bit 0x20 (has a permit); value; reads and writes (permissao_de_trabalho_na_mudanca)'),
          ('nação (registros de 0x122 bytes em [0xae23a8]) +0x76: a marca 0xe; valor; lê (permissao_de_trabalho_na_mudanca)',
           'nation (0x122-byte records at [0xae23a8]) +0x76: the 0xe marker; value; reads (permissao_de_trabalho_na_mudanca)'),
          ('oferta de contrato +0 / +4: a pessoa e o clube; +8: as luvas; +0x61: o cargo; id e valor; lê (concluir_oferta_de_contrato_aceita)',
           'contract offer +0 / +4: the person and the club; +8: the signing-on fee; +0x61: the job; id and value; reads (concluir_oferta_de_contrato_aceita)'),
          ('oferta de transferência +0xc: o jogador principal; +0x11: o tipo (0xb); +0x1b: o valor; +0x20 / +0x24: a vaga e o valor da oferta de contrato; id e valor; lê e grava +0x1b (oferecer_contrato_ao_jogador_da_oferta)',
           'transfer offer +0xc: the main player; +0x11: the type (0xb); +0x1b: the fee; +0x20 / +0x24: the contract offer slot and value; id and value; reads and writes +0x1b (oferecer_contrato_ao_jogador_da_oferta)'),
          ('staff +0x52: o valor do jogador; valor; lê (oferecer_contrato_ao_jogador_da_oferta)', 'staff +0x52: the player value; value; reads (oferecer_contrato_ao_jogador_da_oferta)'),
          ('clube +0x53: a nação; ponteiro; lê', 'club +0x53: the nation; pointer; reads'),
          ('[0xae1080] o último contrato achado; ponteiro; grava (concluir_oferta_de_contrato_aceita)', '[0xae1080] the last contract found; pointer; writes (concluir_oferta_de_contrato_aceita)')]
INTERESSA = ['CMRTE', 'SGE']
