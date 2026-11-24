# Mutacoes (sbA_7): o calendario do ano que perde jogos ao ser gravado volta errado na carga do ano (set_year, a virada
# de 2001 para 2002 na carreira ENG); o comparador do save_fixtures (o arquivo das duas lado a lado) as acusa na
# comparacao. O construtor sem as listas de um dia derruba o jogo (sem mutacao).
MUTACOES = [
    ('sbA_7_grava_so_uma_faixa',
     '            for j in 0..3 {\n                let list = read_usize(read_usize(slices + j * 4) + k as usize * 4);',
     '            for j in 0..1 { // mutation\n                let list = read_usize(read_usize(slices + j * 4) + k as usize * 4);'),
    ('sbA_7_contagem_errada',
     '                    exe::crt_fwrite(fixture, 0x4f, 1, f);\n                    count += 1;',
     '                    exe::crt_fwrite(fixture, 0x4f, 1, f);\n                    count += 2; // mutation'),
]
