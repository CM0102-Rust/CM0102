# Mutacoes (sbD_8), pela carreira com a Argentina ate 2002-08-01 e pelos comparadores (o plano dos registros contra a
# original; o texto lado a lado):
# - a copa de acesso: a linha do promovido com o status 1 em vez de 0;
# - as repescagens: o clube da Primeira rebaixado com o status 2 em vez de 3;
# - o texto das repescagens: "escapa do rebaixamento" e "rebaixado" trocados.
MUTACOES = [
    ('sbD_8_status_da_zona', '                    p.rows[0] = (row, 0);\n                    p.n = 1;\n                }\n            }\n            p.result = -1;',
     '                    p.rows[0] = (row, 1); // mutation\n                    p.n = 1;\n                }\n            }\n            p.result = -1;',
     'sbD_8 argentine second division promotion zone record'),
    ('sbD_8_rebaixado_da_primeira', 'let s = if status == 2 { 3 } else { 0xfe };', 'let s = if status == 2 { 2 } else { 0xfe }; // mutation',
     'sbD_8 argentine second division play-off record'),
    ('sbD_8_textos_trocados', '        (true, true) => 0x987594,   // "{club} avoid relegation"',
     '        (true, true) => 0x987564, // mutation', 'sbD_8 argentine second division play-off text'),
]
