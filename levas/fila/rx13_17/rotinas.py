# A troca da Segunda Divisão B portuguesa com a Terceira e as suas três zonas (por_prm.cpp; leitor rx13, rodada 6,
# 25/11/2026). Só existem com a base NORMAL (a Segunda B é divisão inferior). Simulação com Portugal (as ligas e a data
# dos itens mwe/rx10_7: ENG SWE FIN NOR POR até 2002-08-01): a virada portuguesa (PorPrm::start_new_season,
# 0x7cfa26/0x7cfa2d) as chama no verão de 2002 quando a Segunda B existe (senão a 0x7cfd30).
# - 0x7cff70 por_prm_exchange_second_b_with_lower thiscall ret 0: malloc(4 x [0xae235c]) (nulo: caixa 0x2ee); os clubes
#   com divisão e nação, a nação [0x9cf434] e a divisão [0x9cf620] ou [0x9cf764]; nenhum: free e a caixa 0x301. A
#   Segunda B [[0xadadfc] + 4 x [0x9cf7c0]] e as fases 0 e 1: linhas +0x37 = 3, League::relegate_club_to_division
#   0x6831a0 thiscall(this, clube, [0xae23d0] + [0x9cf620] x 0x6b, 1) ret 0xc, contados num byte. Rebaixados (byte com
#   sinal) > candidatos (palavra): caixa 0x320 e fim (DEFEITO: a lista não é liberada). Senão
#   comp_util_sort_and_shuffle_top_clubs 0x4afbd0 cdecl(lista, n, rebaixados) e os primeiros "rebaixados" sobem
#   (League::move_club_to_division 0x6830b0 thiscall(this, clube, registro da Segunda B, 1) ret 0xc); free. Prólogo
#   sub esp, 0x214 (6).
# - 0x7d0210 por_prm_assign_second_b_zones thiscall ret 0: os clubes (contador palavra) cuja divisão +0x57 é o registro
#   de [0x9cf61c], [0x9cf614], [0x9cf618] ou [0x9cf7c0], até 60; qsort 0x945333(lista, n, 4, 0x53fd40 latitude); o
#   dword +0x60: k < 20 Sul [0x9cf61c], < 40 Centro [0x9cf618], o resto Norte [0x9cf614]. Prólogo sub esp, 0xf4 (6).
# Iguais no cm0102-1080.exe. Comparadores: a troca só a original (sorteia e move clubes), conferindo depois os
# rebaixados; as zonas lado a lado no dword +0x60 dos clubes. Usa o CLUB_LATITUDE_COMPARE do rx13_16: integrar depois
# dele (INTEGRAR_ANTES.txt).
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
BASE = 'NORMAL'
ROTINAS = [('por_prm_exchange_second_b_with_lower', 0x7cff70, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00]),
           ('por_prm_assign_second_b_zones', 0x7d0210, [0x81, 0xec, 0xf4, 0x00, 0x00, 0x00])]
DOCS = ('a troca da Segunda Divisão B portuguesa com a Terceira e as suas três zonas',
        "the Portuguese Second Division B's exchange with the Third Division and its three zones",
        [('por_prm_exchange_second_b_with_lower', 'por_prm_exchange_second_b_with_lower', '0x7cff70', 'por_prm.cpp', 'thiscall, ret 0',
          'regra: na virada portuguesa com a Segunda B jogada, os rebaixados dela e das suas duas fases descem para a Terceira Divisão, e o mesmo número de clubes portugueses da Terceira ou da divisão inferior, sorteados entre os mais fortes, sobem para a Segunda B',
          'rule: at the Portuguese turnover with a played Second Division B, the clubs relegated from it and its two stages go down to the Third Division, and as many Portuguese clubs of the Third Division or the lower division, drawn from the strongest, go up into the Second Division B'),
         ('por_prm_assign_second_b_zones', 'por_prm_assign_second_b_zones', '0x7d0210', 'por_prm.cpp', 'thiscall, ret 0',
          'regra: os clubes da Segunda B (até 60) são divididos nas três zonas do sul para o norte pela cidade do estádio: os 20 mais ao sul na zona Sul, os 20 seguintes na Centro e o resto na Norte',
          'rule: the Second Division B clubs (up to 60) are dealt into its three zones from south to north by their stadium city: the 20 southernmost to the South zone, the next 20 to the Central zone and the rest to the North zone')])
CAMPOS = [('clube +0x60 (club::RESERVE_DIVISION): em Portugal, o registro da zona da Segunda B',
           'club +0x60 (club::RESERVE_DIVISION): in Portugal, the Second Division B zone record')]
INTERESSA = []
