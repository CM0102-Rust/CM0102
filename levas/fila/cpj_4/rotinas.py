# Diversos com contas: a oferta que bate o recorde de valor pago (Club_Records::offer_breaks_fee_record 0x4486c0,
# chamada pela 0x8d3090 das transferências; a 0x452710 acha ou cria os recordes de sempre; x87 em precisão dupla,
# _ftol), a condição do jogador (Injury::get_condition 0x62c590, chamada pelas notas do motor 0x6e2f20/0x6e6340, pelo
# fôlego 0x6f5810 e por ~14 telas; 0x62c400 original; x87 e _ftol) e o limite de escalação da partida
# (get_selection_limit 0x8cf7e0, pela 0x45ccf0 e 0x8cf880; a tabela de 42 bytes da 0x8cf6b0 vem toda preenchida com
# 0xff antes: sem leitura de lixo). A 0x8cf7e0 passa o ecx de entrada à 0x8cf6b0 (é thiscall de fato). Simulação: as
# mutações pela carreira e a confirmação no jogo real comparando. Sem SEH, sem caixa, sem sorteio. Comparadores pelo
# valor (a 0x62c590 roda em laço quente do motor: comparador leve) e lado a lado pela saída.
SAVE = False
ROTINAS = [('oferta_bate_o_recorde', 0x4486c0, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('condicao_do_jogador', 0x62c590, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('limite_de_escalacao', 0x8cf7e0, [0x83, 0xec, 0x2c, 0x33, 0xc0])]
DOCS = ('a oferta que bate o recorde de valor pago, a condição do jogador e o limite de escalação da partida',
        'the offer that breaks the fee paid record, the player condition and the match selection limit',
        [('oferta_bate_o_recorde', 'Club_Records::offer_breaks_fee_record', '0x4486c0', 'club_records.cpp', 'thiscall, ret 8',
          'o valor da oferta passa 50 × reputação + 500.000 (e reputação² × 0x967880 com exigir) e o recorde (+0x313) dos recordes de sempre do comprador',
          'the offer fee passes 50 × reputation + 500,000 (and reputation² × 0x967880 when required) and the record (+0x313) of the buyer all-time records'),
         ('condicao_do_jogador', 'Injury::get_condition', '0x62c590', 'injury.cpp', 'thiscall, ret 8',
          'forma × condição / 10.000 (mínimo 100, máximo 9000 se recuperando), ou em porcentagem (máximo 100)',
          'fitness × condition / 10,000 (minimum 100, maximum 9000 while recovering), or as a percentage (maximum 100)'),
         ('limite_de_escalacao', 'get_selection_limit', '0x8cf7e0', 'transfer_manager.cpp', 'thiscall, ret 8',
          'da tabela de restrições da partida (0x8cf6b0), a primeira que existe: tipo em +0, quantidade em +5 (e a idade em +1 no tipo 3)',
          'from the match restriction table (0x8cf6b0), the first one present: type at +0, count at +5 (and the age at +1 for type 3)')])
CAMPOS = [('oferta de transferência +8 clube comprador, +0x1b valor; recordes de sempre +0x313 maior valor pago; clube +0x80 reputação; lesões [this] registros de 31 bytes (+8 condição, +0xa forma); limite de escalação (6 bytes: tipo, idade, quantidade em +5); lê (oferta_bate_o_recorde, condicao_do_jogador); escreve (limite_de_escalacao)',
           'transfer offer +8 buying club, +0x1b fee; all-time records +0x313 highest fee paid; club +0x80 reputation; injury [this] 31-byte records (+8 condition, +0xa fitness); selection limit (6 bytes: type, age, count at +5); reads (oferta_bate_o_recorde, condicao_do_jogador); writes (limite_de_escalacao)')]
INTERESSA = ['SGE', 'CMRTE']
