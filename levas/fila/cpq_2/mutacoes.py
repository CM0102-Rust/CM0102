# Mutacoes da federacao e da diretoria (cpq_2). O apuro financeiro sem olhar o saldo (qualquer clube com a confianca
# financeira abaixo de 3000: a narracao dos cantos deixa de sortear para esses clubes; o motor de lances corre na
# carreira; sem ramo, a rotina e quente). A avaliacao da selecao sem o piso de 6500 do tecnico novo e a federacao que
# demite em vez de pensar (a 0x69aba0 sorteia): mudam a confianca e as saidas dos tecnicos das selecoes; so rodam no
# fim de competicao de selecao (eliminatorias de 2001): PODEM ESCAPAR se nenhuma acabar antes de 2002-06-01 (ver os
# ramos; a confianca da federacao fica fora do banco).
MUTACOES = [
    ('cpq2_apuro_sem_saldo', '    (core::ptr::read_unaligned(f as *const i64) < 0) as u8',
     '    1 // mutacao: sem olhar o saldo'),
    ('cpq2_selecao_sem_piso', '        cpq2_pw(cpq2_linha(this, selecao) + 6, 0x1964); // técnico novo',
     '        // mutacao: sem o piso do tecnico novo',
     'cpq_2 national team evaluated'),
    ('cpq2_federacao_demite', '    let pensar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x69aba0usize);\n    pensar(this, selecao, 0);',
     '    record_national_departure(this, selecao, 0); // mutacao: demite em vez de pensar',
     'cpq_2 the federation considers'),
]
