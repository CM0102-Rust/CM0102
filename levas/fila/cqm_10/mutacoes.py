# Mutacoes (cqm_10), nos inscritos das copas inglesas (a Inglaterra e uma das 8 ligas): na Copa da Liga, quem caiu da
# Premier entra na First com a classe 0x10 - posicao em vez de 0x46 - posicao (o qsort o poe antes dos outros da First:
# muda quem entra em qual rodada e a ordem do sorteio); no Trofeu da Liga, a segunda chave na ordem inversa (muda os
# confrontos sorteados); na Supercopa, o vice da Premier no lugar do campeao (outro clube joga em 13/8). As tres rodam
# uma vez no comeco da carreira (2001-205). Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqm10_caiu_da_premier_classe_baixa',
     '0x46u8.wrapping_sub(pos) // cqm10: quem caiu da Premier',
     '0x10u8.wrapping_sub(pos) // mutacao: quem caiu da Premier',
     'cqm_10 relegated from premier'),
    ('cqm10_trofeu_ordem_inversa',
     'let c = cqm10_d(cqm10_d(this + 0xb2) + 0x70 + k as usize * 4); // cqm10: o k-ésimo da segunda metade',
     'let c = cqm10_d(cqm10_d(this + 0xb2) + 0x70 + (27 - k as usize) * 4); // mutacao: ordem inversa',
     'cqm_10 league trophy second section'),
    ('cqm10_supercopa_com_o_vice',
     'let a = latest_placed_club(0x9cf5bc, 0); // cqm10: o campeão inglês',
     'let a = latest_placed_club(0x9cf5bc, 1); // mutacao: o vice',
     'cqm_10 english super cup clubs'),
]
