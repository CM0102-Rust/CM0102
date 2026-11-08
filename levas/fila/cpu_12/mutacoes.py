# Mutacoes. A ordem sorteada vira a grade de jogos de cada liga no comeco da temporada (36 chamadas ate 2002-06-01;
# na Inglaterra sem estadio dividido nem Celtic e Rangers, todos os clubes passam pelo sorteio e pelas vagas livres):
# a semente do rand + 1 (outra ordem, outra grade), a troca que perde o segundo registro (a tabela fica na ordem de
# antes) e a ultima vaga livre em vez da primeira (a ordem sai invertida). O comparador e so a original: as mutacoes
# saem pela carreira no modo Rust. Os ramos ficam depois de todas as condicoes, colados no efeito.
MUTACOES = [
    ('cpu12_semente', 'semear((cpu12_w(this + 0x40) as i32).wrapping_add(*(0xdd7120 as *const i32)) as u32); // cpu12: a semente',
     'semear((cpu12_w(this + 0x40) as i32).wrapping_add(*(0xdd7120 as *const i32)).wrapping_add(1) as u32); // mutacao: semente + 1',
     'cpu_12 ordem sorteada'),
    ('cpu12_sem_troca', 'core::ptr::copy(tmp.as_ptr(), (tab() + b * CUP_LEAGUE_TABLE_ROW_SIZE) as *mut u8, CUP_LEAGUE_TABLE_ROW_SIZE); // cpu12: a troca',
     'core::ptr::copy(tmp.as_ptr(), (tab() + a * CUP_LEAGUE_TABLE_ROW_SIZE) as *mut u8, CUP_LEAGUE_TABLE_ROW_SIZE); // mutacao: sem troca',
     'cpu_12 ordem sorteada'),
    ('cpu12_ultima_vaga', 'if let Some(j) = (0..n().max(0) as usize).find(|&j| *posto.add(j) == 0) { // cpu12: a primeira vaga livre',
     'if let Some(j) = (0..n().max(0) as usize).rev().find(|&j| *posto.add(j) == 0) { // mutacao: a ultima',
     'cpu_12 clube na vaga livre'),
]
