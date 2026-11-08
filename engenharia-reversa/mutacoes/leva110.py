# Mutacoes da leva110 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva110.py --jogo 2001-08-20
ARQUIVO = 'date.rs'
MUTACOES = [
    ('y_frente', "                sum -= days_in_year(year.wrapping_sub(1));", "                sum -= days_in_year(year);"),
    ('y_tras', "                sum += days_in_year(year);\n            }", "                sum += 365;\n            }"),
    ('y_est', "        (*st).year = year;", "        (*st).year = a.year;"),
    ('y_dia', "    sum + a.day as i32 - other.day as i32", "    sum + a.day as i32 - other.day as i32 + (other.year > a.year) as i32"),
]
