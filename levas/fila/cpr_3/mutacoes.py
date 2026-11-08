# Mutacoes (com o ramo de cada uma). O jogador com transferencia futura acertada nao e achado (quem pergunta pela
# 0x8cca10, as listas de venda, os alvos e as escolhas do elenco, passa a trata-lo como livre). A oferta acertada para
# o futuro fica no estado 0x12 em vez de 0x13 (a conclusao das futuras do dia e a 0x8cca10 nao a acham: a
# transferencia nao se completa). O jogador da oferta acertada para o futuro fica sem a marca 0x10 (a 0x8cca10 sai
# logo com 0). Os ramos dizem se a carreira passou por eles.
MUTACOES = [
    ('cpr3_futura_ignorada', '1 // a transferência futura existe',
     '0 // mutacao: nao acha a futura',
     'cpr_3 futura achada'),
    ('cpr3_estado_futura', '*((oferta + 0x2c) as *mut u8) = 0x13; // transferência futura',
     '*((oferta + 0x2c) as *mut u8) = 0x12; // mutacao: estado errado',
     'cpr_3 acrescentar futura'),
    ('cpr3_sem_marca', '*staff_flags_byte(this, cpr3_d(oferta + 0xc)) |= 0x10; // a marca da transferência futura',
     'let _ = 0x10; // mutacao: sem a marca',
     'cpr_3 acrescentar futura'),
]
