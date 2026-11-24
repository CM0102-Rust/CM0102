# A oferta de transferencia: o lance do computador (TRANSFER_OFFER::build_bid, 6.832 bytes, transfer_offer.cpp; leitor
# sx21b, rodada 5, 24/11/2026). Roda nas ofertas da carreira ENG (56.887 chamadas no mundo ate 2002-08-01: as listas
# de observados 0x84a70e, o gestor 0x8bf1d1 e o construtor 0x8e56ea). Rotina grande sozinha, em pedacos com nome
# (bid_player_aware, bid_copy_human_offer, bid_rebuild_exchange_list, bid_pre_contract, bid_asking_price,
# bid_valuation, bid_spread_fee, bid_fee_under_ceiling, bid_clauses_within_reach, bid_clauses_out_of_reach,
# bid_settle_fee, bid_clauses_for_next_round, bid_appearance_by_ratio, bid_afford, bid_finish).
# Convencoes conferidas: 0x8e5d60 thiscall(jogador, clube, vaga da lista, teto, pre_contrato (byte), &oferta_de_contrato)
# ret 0x18, quadro SEH (a lista local de 0x3c e o STAFF_CONTRACT novo; sem destrutor no caminho normal). O teto
# (arg 4) e reescrito na escala das parcelas e relido no fim. Sem jogador ou clube: a caixa da linha 0x154.
# Chamadas: 0x600a50 is_human_club cdecl(clube, 0, 1, 0, 0); 0x8cdd80 find_club_offer_for_player(jog, clube, 1);
# 0x417c10 Date::operator=(&tmp, origem); 0x84b6f0 is_compensation_deal([TM+0x856]; vaga, contrato, clube, &x);
# 0x8d0a70 player_club_uses_bosman (ecx = TM, ret 4); 0x4cda70 offer_contract(jog, clube, sc, -1, 1.0, 2);
# 0x4d42d0 get_human_asking_price(vendedor, clube, jog); 0x5444a0 staff_is_transfer_listed cdecl(jog, 0);
# 0x4d1600 calc_asking_price(vendedor, clube, jog, 0, 1, 1|0); 0x867b50 has_active_release_clause (ecx = contrato);
# 0x840d80 value_player_for_club(clube, jog, vaga, &x, -1, 1); 0x4d0c60 calc_player_value(jog, vendedor, 0, -1, 0)
# (chamada ate 4 vezes, como a original); 0x5986b0 get_wealth_band (tabelas de 5 saltos em 0x8e77dc e 0x8e77f0);
# 0x5409b0 club_is_national_team cdecl; 0x8cc930 find_highest_offer_for_player(jog, clube); 0x8e7a70
# get_total_value(0, 0, 0, 1); 0x598800 get_transfer_budget(jog, 0, 0, 1, 0) (i64); 0x8e8330
# restructure_fee_into_clauses(jog, vendedor, clube); 0x53b620 is_player_job cdecl(byte); 0x868210
# check_fifa_protected_period(contrato; clube, 0, 0); 0x8d0780 calc_compensation_fee(jog, clube, 0|1); 0x8ced40
# can_sign_player(clube, &data, jog, vendedor, 1, &palavra); x87 na ordem da original (_ftol truncando; a razao
# preco/avaliacao arredondada a f32 como a original guarda).
# Defeitos do original: o pre-contrato recusado libera o STAFF_CONTRACT e deixa o ponteiro em *out; no caso do
# candidato a selecao (bid_clauses_out_of_reach) a comparacao da venda futura le um double velho da pilha (os dois
# dwords da diferenca e do preco pedido), no lugar da diferenca.
# Comparador: lado a lado nos 0x41 bytes da oferta, *out, G_CONTRACT_LOOKUP_RESULT e o sorteio; quando a original
# alocou (lista de troca mantida ou oferta de contrato em *out), so a original.
SAVE = False
ROTINAS = [
    ('transfer_offer_build_bid', 0x8e5d60, [0x6a, 0xff, 0x68, 0x82, 0x49, 0x96, 0x00]),
]
DOCS = ('a oferta de transferência: o lance do computador',
        "a transfer offer: the computer's bid",
        [('transfer_offer_build_bid', 'transfer_offer_build_bid', '0x8e5d60', 'transfer_offer.cpp', 'thiscall, ret 0x18',
          'regra: a oferta começa vazia (o jogador famoso fica sabendo com uma chance em 20); clube humano repete o último lance pelo jogador ou oferece o valor dele até o orçamento; o computador: acordo de compensação ou jogador livre no fim do contrato vira pré-contrato; senão a taxa é o preço pedido (ou a cláusula de rescisão), cortado pela avaliação de quem compra (vendedor em crise), parcelado quando o comprador é rico e o vendedor pobre (6 a 24 parcelas), levado ao teto com cláusulas de jogos, de seleção e venda futura, confrontado com o maior lance rival (1,2 vez) e com o orçamento; por fim o momento da transferência e se o clube pode contratá-lo',
          "rule: the offer starts empty (a famous player hears of it one time in 20); a human club repeats its last bid for the player or offers his value up to the budget; the computer: a compensation deal or a player free at the end of his contract becomes a pre-contract; otherwise the fee is the asking price (or the release clause), cut by the buyer's valuation (seller in crisis), spread when the buyer is rich and the seller poor (6 to 24 instalments), brought to the ceiling with games, international and sell-on clauses, set against the highest rival bid (1.2 times) and the budget; last, when the move happens and whether the club can sign him")])
CAMPOS = []
INTERESSA = []
