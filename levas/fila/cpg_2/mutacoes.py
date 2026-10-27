# Mutacoes da fila de regens, da lista de observados e do doador de dados. A da entrada da fila troca a pessoa do regen
# (a fila decide quem vira regen): deve acusar. A da busca diz que nao esta na lista (os clubes voltam a observar quem
# ja observam: muda as transferencias): deve acusar. A do doador aceita nota zero: so muda algo quando a melhor nota da
# nacao e 0 ou nenhuma e positiva (22 chamadas): PODE ESCAPAR (o comparando acusa no primeiro).
MUTACOES = [
    ('cpg_fila_pessoa', 'core::ptr::write_unaligned(this as *mut u32, core::ptr::read_unaligned(origem as *const u32)); // a pessoa do regen',
     'core::ptr::write_unaligned(this as *mut u32, core::ptr::read_unaligned(origem as *const u32).wrapping_add(1)); // mutacao: a pessoa seguinte'),
    ('cpg_observados', '(p as u32 & !0xff) | (p != 0) as u32',
     '(p as u32 & !0xff) | 0 // mutacao: nunca esta'),
    ('cpg_doador_zero', 'if s > melhor && s > 0 { // a melhor da nação',
     'if s > melhor && s >= 0 { // mutacao: aceita nota zero'),
]
