# Mutacoes da nation_get_nationality_adjective. No jogo dos testes (ingles, idioma 0) so rodam o comeco e o nome da
# nacao: dbx_nome e dbx_retorno sao acusadas onde a rotina for chamada (o texto e o valor pelo comparador); as tres das
# cadeias (dbx_forma, dbx_vazio, dbx_cadeia) so mudam alguma coisa nos idiomas 9, 4, 5, 10 e 6: escapam da carreira
# em ingles (ver o LEIA da leva).
MUTACOES = [
    ('dbx_nome', '    let nome = nacao + 0x57; // o nome da nação',
     '    let nome = nacao + 0x58; // mutacao: o nome da nacao um byte adiante'),
    ('dbx_retorno', '    dbu1_copiar(texto, nome);\n    1\n}',
     '    dbu1_copiar(texto, nome);\n    2 // mutacao: devolve 2\n}'),
    ('dbx_forma', '                    match fontes[f.min(5)] { // as formas 5..255 pela última coluna',
     '                    match fontes[f.min(4)] { // mutacao: as formas 5..255 pela coluna 4'),
    ('dbx_vazio', '            if !nome_se_vazio || *(texto as *const u8) != 0 {',
     '            if *(texto as *const u8) != 0 { // mutacao: o nome no fim tambem nos idiomas 4 e 5'),
    ('dbx_cadeia', '                if let Some((_, fontes)) = cadeia.iter().find(|e| *(e.0 as *const i32) == id) {',
     '                if let Some((_, fontes)) = cadeia.iter().skip(1).find(|e| *(e.0 as *const i32) == id) { // mutacao: sem a primeira nacao da cadeia'),
]
