# A oferta de transferencia: achar um jogador de troca, o valor total e o destrutor (transfer_offer.cpp; leitor sx21,
# rodada 5, 24/11/2026). Rodam nas ofertas da carreira ENG (o valor total na avaliacao das ofertas; o destrutor quando
# uma oferta acaba).
# Convencoes conferidas: 0x8e82f0 thiscall(jogador) ret 4; 0x8e7a70 thiscall(so_taxa, sem_troca, avaliado, com_venda)
# ret 0x10; 0x8e5710 thiscall ret 0 (quadro SEH sem destrutor no caminho normal); 0x4d0c60 thiscall(pessoa, 0, 0, -1,
# 0) ret 0x14; 0x68d040 thiscall ret 0; 0x68d1f0 thiscall(item) ret 4; 0x944c94 cdecl.
# Defeito do original (no cabecalho): no valor total avaliado, cada jogador de troca soma a avaliacao do alvo.
# Comparadores: a entrada; o valor (lado a lado no estado do sorteio); o destrutor so a original (libera memoria).
SAVE = False
ROTINAS = [
    ('transfer_offer_find_exchange_player', 0x8e82f0, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x46, 0x04]),
    ('transfer_offer_get_total_value', 0x8e7a70, [0x53, 0x56, 0x57, 0x8b, 0xf9]),
    ('destroy_transfer_offer', 0x8e5710, [0x6a, 0xff, 0x68, 0x0b, 0x49, 0x96, 0x00]),
]
DOCS = ('a oferta de transferência: achar um jogador de troca, o valor total e o destrutor',
        "a transfer offer: finding an exchange player, the total worth and the destructor",
        [('transfer_offer_find_exchange_player', 'transfer_offer_find_exchange_player', '0x8e82f0', 'transfer_offer.cpp', 'thiscall, ret 4',
          'regra: a entrada da lista de troca com o id do jogador, ou 0',
          "rule: the exchange list entry with the player's id, or 0"),
         ('transfer_offer_get_total_value', 'transfer_offer_get_total_value', '0x8e7a70', 'transfer_offer.cpp', 'thiscall, ret 0x10',
          'regra: a taxa; sem "só a taxa", as cláusulas positivas e, com a parte da venda futura, essa porcentagem do valor do alvo; sem "sem troca", cada jogador de troca pelo valor dele (ou, avaliado, pela avaliação do alvo: defeito)',
          "rule: the fee; without 'fee only', the positive clauses and, with the sell-on share, that percentage of the target's value; without 'no exchange', each exchange player by his value (or, valued, by the target's valuation: defect)"),
         ('destroy_transfer_offer', 'destroy_transfer_offer', '0x8e5710', 'transfer_offer.cpp', 'thiscall, ret 0',
          'regra: sai da lista de ofertas rivais do jogador (liberando-a se era a última) e libera a lista de troca',
          "rule: leaves the player's rival offer list (freeing it when it was the last) and frees the exchange list")])
CAMPOS = []
INTERESSA = []
