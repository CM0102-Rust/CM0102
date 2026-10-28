# Mutacoes. A partida assistida zera so 78 bytes (o comparando acusa na primeira limpeza; a carreira automatica nao tem
# partidas assistidas: PODE ESCAPAR). Os alvos sem conferir o clube (o comparando acusa pelo valor e pela lista; a
# carreira, se a busca usar). O olheiro trocado mesmo quando e o mesmo (o comparando acusa quando ele confere; PODE
# ESCAPAR da carreira).
MUTACOES = [
    ('cpj_assistida_79', 'core::ptr::write_bytes(f as *mut u8, 0, 79); // a partida sai',
     'core::ptr::write_bytes(f as *mut u8, 0, 78); // mutacao: 78 bytes'),
    ('cpj_alvos_clube', 'if cpj5_d(e) as u32 == id { // o pedido do clube',
     'if cpj5_d(e) as u32 != id { // mutacao: os dos outros'),
    ('cpj_olheiro_mesmo', 'return 1; // o mesmo olheiro',
     '// mutacao: troca mesmo assim'),
]
