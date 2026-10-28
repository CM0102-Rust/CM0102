# As listas de pendências: soltar as suspensões guardadas de uma pessoa (Discipline::drop_held_bans 0x5567d0, chamada
# pela 0x8c9720 das transferências; ver docs/MAPA_DISCIPLINA.md), tirar as ofertas de contrato pendentes dela
# (CONTRACT_MANAGER::remove_pending_offers_for_staff 0x4ccbf0; aposentadoria 0x7a59e0, carga do jogo; 0x8c5430 já em
# Rust) e marcar a decisão depois da transferência acertada (CONTRACT_MANAGER::schedule_decision_for_transfer 0x4cc930,
# pelas 0x8c0660/0x8c63a0; a 0x4cd230 agenda, original). Simulação: as mutações pela carreira e a confirmação no jogo
# real comparando. Sem SEH, sem caixa, sem sorteio, sem locais. Comparadores: a original roda e o resultado é
# conferido com o plano da cópia (os itens que ficaram e o contador; a oferta marcada e o valor).
SAVE = False
ROTINAS = [('soltar_suspensoes_guardadas', 0x5567d0, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('tirar_ofertas_pendentes', 0x4ccbf0, [0x56, 0x57, 0x8b, 0xf9, 0x8b, 0x47, 0x34]),
           ('marcar_decisao_da_transferencia', 0x4cc930, [0x56, 0x57, 0x8b, 0xf9, 0x8b, 0x47, 0x34])]
DOCS = ('as listas de pendências: soltar as suspensões guardadas, tirar as ofertas pendentes da pessoa e marcar a decisão depois da transferência acertada',
        'the pending lists: dropping the held bans, removing the pending offers of a person and marking the decision after an agreed transfer',
        [('soltar_suspensoes_guardadas', 'Discipline::drop_held_bans', '0x5567d0', 'discipline.cpp', 'thiscall, ret 4',
          'tira da lista das guardadas (+0x1c) os itens da pessoa enquanto o contador +0x1c do registro dela for positivo',
          'removes from the held list (+0x1c) the items of the person while the +0x1c counter of her record is positive'),
         ('tirar_ofertas_pendentes', 'CONTRACT_MANAGER::remove_pending_offers_for_staff', '0x4ccbf0', 'contract_manager.cpp', 'thiscall, ret 4',
          'com ofertas pendentes (+8 do registro de 79 bytes), desmarca o contrato acertado (0x8c5430) e tira as ofertas da pessoa (+0x34) até o byte zerar',
          'with pending offers (+8 of the 79-byte record), unmarks the agreed contract (0x8c5430) and removes the person offers (+0x34) until the byte is zero'),
         ('marcar_decisao_da_transferencia', 'CONTRACT_MANAGER::schedule_decision_for_transfer', '0x4cc930', 'contract_manager.cpp', 'thiscall, ret 4',
          'marca (+0x62) a primeira oferta da pessoa e do clube ainda sem decisão e agenda a decisão (0x4cd230); devolve habilidade / 500 > reputação do clube',
          'marks (+0x62) the first offer of the person and club still undecided and schedules the decision (0x4cd230); returns ability / 500 > club reputation')])
CAMPOS = [('disciplina +0x1c lista das guardadas, registros de 29 bytes (+0x1c contador); gerenciador de contratos +0x34 lista das ofertas (+0 pessoa, +4 clube, +0x62 decidida); registro de 79 bytes [0xae2d0c] +8 pendentes; oferta de transferência +8 clube, +0xc pessoa, +0x20 vaga; jogador +0xb habilidade; clube +0x80 reputação; escreve (soltar_suspensoes_guardadas, tirar_ofertas_pendentes, marcar_decisao_da_transferencia)',
           'discipline +0x1c held list, 29-byte records (+0x1c counter); contract manager +0x34 offers list (+0 person, +4 club, +0x62 decided); 79-byte record [0xae2d0c] +8 pending; transfer offer +8 club, +0xc person, +0x20 slot; player +0xb ability; club +0x80 reputation; writes (soltar_suspensoes_guardadas, tirar_ofertas_pendentes, marcar_decisao_da_transferencia)')]
INTERESSA = ['SGE', 'CMRTE']
