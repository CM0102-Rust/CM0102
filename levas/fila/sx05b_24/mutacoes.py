# Mutacoes (sx05b_24, telas: a cadeia nao as roda; para provar o comparador numa sessao do Lucas).
# - os valores prontos pelo cambio em vez do cambio redondo;
# - o nome da libra trocado pelo do dolar.
MUTACOES = [
    ('sx05b_24_cambio_em_vez_do_redondo',
     '    let v = cash_from_display((pounds as f64 * read_f64(CURRENCY_ROUND_RATE)) as i64);',
     '    let v = cash_from_display((pounds as f64 * read_f64(CURRENCY_RATE)) as i64); // mutation'),
    ('sx05b_24_libra_dolar',
     'const CURRENCY_NAMES: [usize; 26] = [0x98b254,',
     'const CURRENCY_NAMES: [usize; 26] = [0x98b3fc, /* mutation */'),
]
