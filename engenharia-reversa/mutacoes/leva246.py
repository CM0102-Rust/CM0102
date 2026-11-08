# Mutações da 246a leva (itens cpj_2, cpj_3, cpj_4, cpj_5, cpj_6). Uso: python mutacoes.py mutacoes\leva246.py --carreira 2002-06-01
MUTACOES = [
    ('cpj_suspensoes_desconto', '*n = (*n).wrapping_sub(1); // one fewer\n        }', '// mutacao: sem descontar\n        }'),
    ('cpj_pendentes_fim', 'return; // the last pending one', '// mutacao: segue'),
    ('cpj_decisao_500', 'as i32 / 500;', 'as i32 / 400; // mutacao: 400'),
    ('cpj_noticia_divisao', 'c != 0 && read_usize(c + 0x57) == division // the club’s division', 'c != 0 // mutacao: qualquer divisao'),
    ('cpj_noticia_selecao', '(c != 0 && read_usize(c + 0x53) == nacao) || (s != 0 && read_usize(s + 0x53) == nacao) // the club or the national team', '(c != 0 && read_usize(c + 0x53) == nacao) && s == s // mutacao: so o clube'),
    ('cpj_noticia_alcance', 'deliver_to_human_managers(this, noticia, 4, |_| true) as u32', 'deliver_to_human_managers(this, noticia, 2, |_| true) as u32 // mutacao: alcance 2'),
    ('cpj_recorde_minimo', 'rep.wrapping_mul(25).wrapping_add(250_000).wrapping_mul(2) {', 'rep.wrapping_mul(25).wrapping_add(200_000).wrapping_mul(2) { // mutacao: 400.000'),
    ('cpj_condicao_teto', '9000 // the ceiling for those recovering', 'v // mutacao: sem o teto'),
    ('cpj_limite_terceiro', '(2, t[2]) // o 3º limite', '(2, t[1]) // mutacao: o 2o byte'),
    ('cpj_assistida_79', 'core::ptr::write_bytes(f as *mut u8, 0, 79); // a partida sai', 'core::ptr::write_bytes(f as *mut u8, 0, 78); // mutacao: 78 bytes'),
    ('cpj_alvos_clube', 'if read_usize(e) as u32 == id { // the club’s request', 'if read_usize(e) as u32 != id { // mutacao: os dos outros'),
    ('cpj_olheiro_mesmo', 'return 1; // o mesmo olheiro', '// mutacao: troca mesmo assim'),
    ('cpj_diretoria_tecnico', 'if read_usize(clube + club::MANAGER) != 0 { // the side has a manager', 'if read_usize(clube + 0xcf) == 0 { // mutacao: os sem tecnico'),
    ('cpj_disciplina_liga', 'if let Some(t) = comp_tribunal(this, read_usize(league) as i32) { // the tribunal of the club’s league', 'if let Some(t) = comp_tribunal(this, read_usize(league) as i32 + 1) { // mutacao: a liga seguinte'),
    ('cpj_shortlist_recuo', 'e -= 8; // the previous one also belongs to the club', 'break; // mutacao: sem recuar'),
    ('cpj_oferta_empate', 'if v > maior { // the first one on a tie', 'if v >= maior { // mutacao: a ultima no empate'),
]
