# Mutações (cqj_2), na tabela dos clubes da Conmebol (0x4c6280 roda 2 vezes até 2002-06-01, na virada do ano da
# Conmebol, em 2001 e em 2002; a tabela decide quem completa as cotas da Libertadores de 2002, play antes de junho,
# com o Brasil na carreira de 8 ligas):
# - sem o México: os clubes mexicanos ficam fora da tabela e a cota do México fica vazia (contador "cqj_2 clube
#   mexicano");
# - sem tirar a vaga continental de antes: os classificados da edição anterior contam como "já com vaga" e a cota vai a
#   outros clubes (contador "cqj_2 removes the place");
# - sem o nível da divisão na ordem: no Brasil (a única liga sul-americana jogável na carreira) clubes de divisões de
#   baixo passam à frente (contador "cqj_2 division level").
MUTACOES = [
    ('cqj2_sem_mexico',
     'let mexico = cqj2_d(nacao) == cqj2_d(0x9cf3d0); // cqj2: o México entra',
     'let mexico = cqj2_d(nacao) == cqj2_d(0x9cf3d0) && false; // mutacao: sem o México',
     'cqj_2 mexican club'),
    ('cqj2_sem_tirar_a_vaga',
     '*((c + 0x1db) as *mut u32) = u32::MAX; // cqj2: sem vaga continental',
     '// mutacao: a vaga de antes fica',
     'cqj_2 removes the place'),
    ('cqj2_sem_nivel_da_divisao',
     'core::ptr::write_unaligned((r + 9) as *mut u16, core::ptr::read_unaligned((divisao + 0x69) as *const u16)); // cqj2: o nível da divisão',
     'core::ptr::write_unaligned((r + 9) as *mut u16, 0); // mutacao: sem o nível da divisão',
     'cqj_2 division level'),
]
