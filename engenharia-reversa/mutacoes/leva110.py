# Mutacoes da leva110 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva110.py --jogo 2001-08-20
ARQUIVO = 'date.rs'
MUTACOES = [
    ('y_frente', "                sum -= days_in_year(year.wrapping_sub(1));", "                sum -= days_in_year(year);"),
    ('y_tras', "                sum += days_in_year(year);\n            }", "                sum += 365;\n            }"),
    ('y_est', "        (*st).ano = year;", "        (*st).ano = a.ano;"),
    ('y_dia', "    sum + a.dia as i32 - other.dia as i32", "    sum + a.dia as i32 - other.dia as i32 + (other.ano > a.ano) as i32"),
]
