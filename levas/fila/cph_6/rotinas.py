# Permissões de trabalho (transfer_manager.cpp), 6º item: a oferta que precisa de permissão
# (transfer_manager_offer_needs_work_permit 0x8c7470, 647 chamadas na carreira até 2002-06-01; pela 0x8c7560 original,
# 25 milhões de chamadas, que tem caixa no caminho de pessoa nula e fica fora; a lista de troca pela 0x68d1b0) e a
# atualização das permissões (TRANSFER_MANAGER::update_work_permits 0x8c7330, 48; qsort 0x945333 com 0x8e55c0).
# Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem caixa,
# sem sorteio, sem local lida sem valor. Comparadores: a oferta lado a lado pela ficha [0xae1080] e pelo cursor da
# lista de troca (o valor); a atualização lado a lado pelo vetor, pela contagem e, de cada permissão, pela marca da
# pessoa (byte de 8 em [this+0x86e]) e pela segunda nacionalidade (+0x1e). A ficha [0xae1080] é gravada pela
# 0x8c7470 com o contrato de cada jogador de troca.
SAVE = False
ROTINAS = [('oferta_precisa_de_permissao', 0x8c7470, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('atualizar_permissoes_de_trabalho', 0x8c7330, [0x51, 0x55, 0x56, 0x8b, 0xf1])]
DOCS = ('as permissões de trabalho: a oferta que precisa de permissão e a atualização anual',
        'work permits: the offer that needs a permit and the yearly update',
        [('oferta_precisa_de_permissao', 'transfer_manager_offer_needs_work_permit', '0x8c7470', 'transfer_manager.cpp', 'thiscall, ret 4',
          '1 se o jogador da oferta precisa de permissão no clube da oferta (0x8c7560) ou um dos de troca no clube do contrato dele; senão 0',
          '1 when the offer player needs a permit at the offer club (0x8c7560) or one of the exchange players at his contract club; else 0'),
         ('atualizar_permissoes_de_trabalho', 'TRANSFER_MANAGER::update_work_permits', '0x8c7330', 'transfer_manager.cpp', 'thiscall',
          'zera a marca de quem está sem o bit 0x20; com mais de 4 anos dá a segunda nacionalidade e tira a permissão; tira a de quem não é jogador; reordena e desconta as tiradas do fim',
          'clears the flag of whoever lacks bit 0x20; after more than 4 years grants the second nationality and drops the permit; drops the permit of non-players; sorts and trims the dropped ones from the end')])
CAMPOS = [('oferta de transferência: +4 lista de jogadores de troca, +8 clube, +0xc jogador; lê e grava o cursor (+0xa) da lista e [0xae1080] (oferta_precisa_de_permissao); TRANSFER_MANAGER +0x86e marcas (8 bytes por pessoa, bit 0x20), +0x93a/+0x93e permissões (14 bytes: +0 pessoa, +8 ano, +0xa nação); pessoa +0x1e segunda nacionalidade; escreve (atualizar_permissoes_de_trabalho)',
           'transfer offer: +4 exchange player list, +8 club, +0xc player; reads and writes the list cursor (+0xa) and [0xae1080] (oferta_precisa_de_permissao); TRANSFER_MANAGER +0x86e flags (8 bytes per person, bit 0x20), +0x93a/+0x93e permits (14 bytes: +0 person, +8 year, +0xa nation); person +0x1e second nationality; writes (atualizar_permissoes_de_trabalho)')]
INTERESSA = ['SGE', 'CMRTE']
