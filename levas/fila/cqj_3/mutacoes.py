# Mutações (cqj_3), no calendário das sedes (0x5fa850, todo dia: 516 chamadas até 2002-06-01) e na sede de uma edição
# (0x5fa730, 522 chamadas):
# - a candidatura vencedora sem o sorteio (fica a 0): um sorteio a menos em cada decisão (contador "cqj_3 sede
#   sorteada");
# - no 1º de janeiro as edições passadas andam um ano a mais que o intervalo: as decisões dos anos seguintes caem em
#   outras datas (contador "cqj_3 edicao seguinte");
# - a sede padrão sem a segunda sede (contador "cqj_3 sede padrao").
MUTACOES = [
    ('cqj3_sem_sorteio_da_sede',
     '*k = sorteio(3) as u8; // cqj3: a candidatura vencedora',
     '*k = 0; // mutacao: sem o sorteio',
     'cqj_3 sede sorteada'),
    ('cqj3_intervalo_mais_um',
     'core::ptr::write_unaligned((e + 4) as *mut u16, ae.wrapping_add(*((e + 0x21) as *const i8) as i16 as u16)); // cqj3: o intervalo',
     'core::ptr::write_unaligned((e + 4) as *mut u16, ae.wrapping_add(*((e + 0x21) as *const i8) as i16 as u16).wrapping_add(1)); // mutacao: um ano a mais',
     'cqj_3 edicao seguinte'),
    ('cqj3_padrao_sem_segunda',
     'return host_pair(e, -1, a, b) as u32; // cqj3: a sede padrão',
     '{ let r = host_pair(e, -1, a, b) as u32; *b = u32::MAX; return r; } // mutacao: sem a segunda',
     'cqj_3 sede padrao'),
]
