# Mutacoes. A forma inicial com 74 por dia em vez de 75 (todos os jogadores comecam com outra forma: as escalacoes e
# as partidas mudam; comparando, o lado a lado acusa). As lesoes iniciais com 1 em 99 (o sorteio muda de band e de
# sequencia: outra carreira; so o modo Rust e a carreira, o comparador roda a original). As listas dos jovens com os
# clubes das nacoes com +0x11c (o regen dessas nacoes acha clube na lista; comparando, o plano usa a mesma regra e a
# original nao: a lista difere). As tres rodam na criacao da carreira automatica.
MUTACOES = [
    ('cpp9_forma_inicial', 'core::ptr::write_unaligned((r + 8) as *mut u16, (10000 - v * 75) as u16); // a forma inicial',
     'core::ptr::write_unaligned((r + 8) as *mut u16, (10000 - v * 74) as u16); // mutacao: 74',
     'cpp_9 forma inicial'),
    ('cpp9_lesao_inicial', 'if cpp9_sorteio(100) == 0 { // 1 em 100 começa lesionado',
     'if cpp9_sorteio(99) == 0 { // mutacao: 1 em 99',
     'cpp_9 lesao inicial'),
    ('cpp9_nacao_chave', 'if !(cpp9_w(clube + 0x80) <= 2 && cpp9_d(clube + 0x57) != 0) { // nação com +0x11c',
     'if !(cpp9_w(clube + 0x80) <= 0x7fff && cpp9_d(clube + 0x57) != 0) { // mutacao: todos os clubes com divisao',
     'cpp_9 nacao chave'),
]
