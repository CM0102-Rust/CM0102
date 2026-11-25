# Mutacoes pela prova do save (o .qst gravado pela original e pela copia, item a item; e a carga do save de ferias):
# - rx08_7_bolo_em_ponteiros: o bolo vai para o save como ponteiros, sem virar indices: o arquivo temporario da secao
#   europeia no .qst muda (acusada pela comparacao do .qst).
# - rx08_7_formato_errado: com uma ou duas isentas o formato carregado vira o de tres (6, 7, 8): acusada se a impressao
#   da carga cobrir os campos da secao europeia; senao so a carreira carregada ate 2004 a veria (fora da cadeia).
MUTACOES = [
    ('rx08_7_bolo_em_ponteiros',
     '    club_pointers_to_indices(pool, n as i32);\n    exe::crt_fwrite(pool, 4, n, file);\n    club_indices_to_pointers(pool, n as i32);\n    n\n',
     '    exe::crt_fwrite(pool, 4, n, file); // rx08_7 mutation: pointers, not indices\n    n\n',
     'rx08_7 european data saved'),
    ('rx08_7_formato_errado',
     '        1 | 2 => [7, 8, 9],\n        _ => [6, 7, 8],\n    };\n    for (k, v) in format.into_iter().enumerate() {\n        write_u8(this + WC_EUROPE_LAST_GROUP + k, v);\n    }\n    let mut count = 0i32;',
     '        1 | 2 => [6, 7, 8],\n        _ => [6, 7, 8],\n    };\n    for (k, v) in format.into_iter().enumerate() {\n        write_u8(this + WC_EUROPE_LAST_GROUP + k, v);\n    }\n    let mut count = 0i32;',
     'rx08_7 european data loaded'),
]
