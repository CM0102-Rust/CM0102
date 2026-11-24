# A oferta de transferencia nas telas de oferta: montar uma oferta como lance, comparar os termos de duas ofertas e
# tirar um jogador de troca (transfer_offer.cpp; leitor sx21, rodada 5, 24/11/2026). Nao rodam nas carreiras de teste
# (0 chamadas no mundo ate 2002-08-01 para as duas ultimas; a primeira so pelas telas, itens tel17_1 e tel31_3 a citam):
# tipo TELAS.
# Caminho no jogo: com o proprio clube, abrir a tela de proposta de transferencia por um jogador de outro clube, incluir
# jogadores de troca, tirar um deles (remove_exchange_player) e enviar; reabrir a proposta e reenviar com os mesmos
# termos (has_same_terms); o lance pela tela monta a oferta pelo construtor 0x8e56a0.
# Convencoes conferidas: 0x8e56a0 thiscall(a1..a5) ret 0x14 (repassa os cinco e um 0 ao 0x8e5d60); 0x8e7f70
# thiscall(outra) ret 4 (0x43f460 thiscall(4) ret 4, Cash::get_display_amount); 0x8e81a0 thiscall(jogador)
# ret 4 (sem lista, a caixa da linha 0x677 de transfer_offer.cpp).
# Comparadores: o construtor e a retirada so a original (montam e liberam); os termos pelo valor.
SAVE = False
TELAS = True
ROTINAS = [
    ('construct_transfer_offer_bid', 0x8e56a0, [0x6a, 0xff, 0x68, 0xeb, 0x48, 0x96, 0x00]),
    ('transfer_offer_has_same_terms', 0x8e7f70, [0x6a, 0xff, 0x68, 0xc0, 0x49, 0x96, 0x00]),
    ('transfer_offer_remove_exchange_player', 0x8e81a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
]
DOCS = ('a oferta de transferência nas telas: montar um lance, comparar os termos de duas ofertas e tirar um jogador de troca',
        "a transfer offer on the offer screens: building a bid, comparing two offers' terms and taking an exchange player out",
        [('construct_transfer_offer_bid', 'construct_transfer_offer_bid', '0x8e56a0', 'transfer_offer.cpp', 'thiscall, ret 0x14',
          'regra: a data da oferta construída e o lance montado com os cinco argumentos',
          "rule: the offer's date built and the bid made with the five arguments"),
         ('transfer_offer_has_same_terms', 'transfer_offer_has_same_terms', '0x8e7f70', 'transfer_offer.cpp', 'thiscall, ret 4',
          'regra: iguais quando a taxa, as cláusulas (valor e jogos, se positivas), a venda futura, as parcelas, o momento da conclusão e os jogadores de troca batem (valores comparados como exibidos)',
          'rule: the same when the fee, the clauses (amount and games, when positive), the sell-on share, the instalments, the completion timing and the exchange players match (amounts compared as displayed)'),
         ('transfer_offer_remove_exchange_player', 'transfer_offer_remove_exchange_player', '0x8e81a0', 'transfer_offer.cpp', 'thiscall, ret 4',
          'regra: tira o jogador da lista de troca (liberando a lista vazia), soma a avaliação dele à taxa e baixa em um a conta de ofertas em que ele é jogador de troca',
          "rule: takes the player out of the exchange list (freeing an empty list), adds his valuation to the fee and lowers by one his count of offers as an exchange player")])
CAMPOS = []
INTERESSA = []
