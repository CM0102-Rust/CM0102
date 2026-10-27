# Mutacoes do elenco das selecoes. A de por grava a marca 0xa no lugar de 0xb (106 chamadas; a pessoa entra na
# impressao do banco): deve acusar. A de tirar nao esvazia a vaga (142; a vaga e do time, na impressao do banco): deve
# acusar. A do sub-21 dispensa tambem os de 21 anos: so roda passada a data do registro (ramo "sub-21 dispensar acima
# da idade") e com alguem de 21 no elenco: PODE ESCAPAR se a dispensa nao acontecer ate 2002-06-01.
MUTACOES = [
    ('cpd_por_marca', '    *((p + 0x28) as *mut u8) = 0xb;\n    let marcar_elenco', '    *((p + 0x28) as *mut u8) = 0xa; // mutacao: outra marca\n    let marcar_elenco'),
    ('cpd_tirar_vaga', 'core::ptr::write_unaligned((time + 0xd7 + 4 * c) as *mut usize, 0);', 'core::ptr::write_unaligned((time + 0xd7 + 4 * c) as *mut usize, p); // mutacao: a vaga fica'),
    ('cpd_sub21_idade', '*((p + 0x18) as *const i8) > 0x15 { // acima de 21 anos', '*((p + 0x18) as *const i8) >= 0x15 { // mutacao: tambem os de 21'),
]
