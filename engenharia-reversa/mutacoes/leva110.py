# Mutacoes da leva110 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva110.py --jogo 2001-08-20
ARQUIVO = 'data.rs'
MUTACOES = [
    ('y_frente', "                soma -= dias_do_ano(ano.wrapping_sub(1));", "                soma -= dias_do_ano(ano);"),
    ('y_tras', "                soma += dias_do_ano(ano);\n            }", "                soma += 365;\n            }"),
    ('y_est', "        (*est).ano = ano;", "        (*est).ano = a.ano;"),
    ('y_dia', "    soma + a.dia as i32 - outra.dia as i32", "    soma + a.dia as i32 - outra.dia as i32 + (outra.ano > a.ano) as i32"),
]
