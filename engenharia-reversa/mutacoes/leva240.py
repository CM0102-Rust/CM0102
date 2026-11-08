# Mutações da 240a leva (itens dbx_1). Uso: python mutacoes.py mutacoes\leva240.py --carreira 2002-06-01
MUTACOES = [
    ('dbx_nome', '    let name = nacao + nation::NATIONALITY; // o nome da nação', '    let name = nacao + 0x58; // mutacao: o nome da nacao um byte adiante'),
    ('dbx_retorno', '    copy_c_string(text, name);\n    1\n}', '    copy_c_string(text, name);\n    2 // mutacao: devolve 2\n}'),
    ('dbx_forma', '                    match fontes[f.min(5)] { // as formas 5..255 pela última coluna', '                    match fontes[f.min(4)] { // mutacao: as formas 5..255 pela coluna 4'),
    ('dbx_vazio', '            if !nome_se_vazio || *(text as *const u8) != 0 {', '            if *(text as *const u8) != 0 { // mutacao: o nome no fim tambem nos idiomas 4 e 5'),
    ('dbx_cadeia', '                if let Some((_, fontes)) = cadeia.iter().find(|e| *(e.0 as *const i32) == id) {', '                if let Some((_, fontes)) = cadeia.iter().skip(1).find(|e| *(e.0 as *const i32) == id) { // mutacao: sem a primeira nacao da cadeia'),
]
