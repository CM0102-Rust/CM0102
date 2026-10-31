# Mutacoes, todas na fila dos regens jovens (0x7a65d0, todo dia: as entradas vencidas viram regens pela 0x7abac0,
# centenas por carreira ate 2002-06-01). Cada uma muda o dia em que o regen nasce (um por dia, todos de uma vez, um dia
# antes): o regen sorteia nome, atributos e contrato, entao a contagem de sorteios e o banco (pessoas, contratos) mudam.
MUTACOES = [
    ('cpw10_um_regen_por_dia',
     'core::ptr::write_unaligned((this + 0x14) as *mut i32, n - 1); // cpw10: a entrada sai',
     'core::ptr::write_unaligned((this + 0x14) as *mut i32, n - 1); break; // mutacao: um regen por dia',
     'cpw_10 regen da fila'),
    ('cpw10_sem_data',
     'anterior(e + 4, cpw10_d(0xae2c90) as u32, cpw10_d(0xae2c94) as u32) != 0 // cpw10: a entrada vencida',
     'true || anterior(e + 4, cpw10_d(0xae2c90) as u32, cpw10_d(0xae2c94) as u32) != 0 // mutacao: todas vencidas',
     'cpw_10 regen da fila'),
    ('cpw10_vence_no_dia',
     'anterior(e + 4, cpw10_d(0xae2c90) as u32, cpw10_d(0xae2c94) as u32) != 0 // cpw10: a entrada vencida',
     '{ let _ = anterior; let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize); depois(e + 4, cpw10_d(0xae2c90) as u32, cpw10_d(0xae2c94) as u32) == 0 } // mutacao: vence no proprio dia',
     'cpw_10 regen da fila'),
]
