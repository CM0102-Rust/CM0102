# Mutacoes. A diretoria sem conferir o tecnico do lado e a disciplina sem o tribunal da liga do clube: so o modo Rust e
# a carreira acusam (o comparando roda a original); a diretoria e as suspensoes entram na carreira (provaveis de
# acusar). A shortlist sem recuar ate a primeira entrada (o comparando acusa no primeiro clube com mais de uma). A maior
# oferta com a ultima no empate (o comparando acusa no primeiro empate: PODE ESCAPAR).
MUTACOES = [
    ('cpj_diretoria_tecnico', 'if cpj6_d(clube + 0xcf) != 0 { // o lado tem técnico',
     'if cpj6_d(clube + 0xcf) == 0 { // mutacao: os sem tecnico'),
    ('cpj_disciplina_liga', 'if let Some(t) = comp_tribunal(this, cpj6_d(liga) as i32) { // o tribunal da liga do clube',
     'if let Some(t) = comp_tribunal(this, cpj6_d(liga) as i32 + 1) { // mutacao: a liga seguinte'),
    ('cpj_shortlist_recuo', 'e -= 8; // a anterior também é do clube',
     'break; // mutacao: sem recuar'),
    ('cpj_oferta_empate', 'if v > maior { // a primeira no empate',
     'if v >= maior { // mutacao: a ultima no empate'),
]
