# Mutacoes (sx05b_21). A moeda so muda o que a tela e os textos mostram (os valores guardados sao em libras): acusadas
# pelo comparador lado a lado na carga das preferencias.
# - a libra com o separador de milhar ponto;
# - sem copiar o sufixo;
# - o id gravado mais um.
MUTACOES = [
    ('sx05b_21_libra_ponto',
     'prefix: 0x98b254, suffix: 0xad9c64, thousands: 0x2c,',
     'prefix: 0x98b254, suffix: 0xad9c64, thousands: 0x2e, /* mutation */',
     'sx05b_21 currency set'),
    ('sx05b_21_sem_sufixo',
     '    tel26_copy_text(CURRENCY_SUFFIX as *mut u8, c.suffix as *const u8);',
     '    // mutation: no suffix',
     'sx05b_21 currency set'),
    ('sx05b_21_id_mais_um',
     '    write_u8(CURRENCY_ID, id as u8);',
     '    write_u8(CURRENCY_ID, id as u8 + 1); // mutation',
     'sx05b_21 currency set'),
]
