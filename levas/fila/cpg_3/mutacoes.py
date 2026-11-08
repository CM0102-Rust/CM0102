# Mutacoes da suspensao, do salario padrao e da match_condition de jogo. A da suspensao copia so dois dos tres bits de +0x13
# (as marcas mudam o cumprimento da suspensao): deve acusar se o terceiro bit aparecer numa copia da carreira; senao o
# comparando acusa. A do salario sobe o minimo para 56 (contratos baratos): deve acusar (o contrato esta na impressao
# do banco). A da match_condition devolve um ponto a menos em porcento (escolha da escalacao): deve acusar.
MUTACOES = [
    ('cpg_suspensao_bits', '*m = (*m & !7) | (*((origem + 0x13) as *const u8) & 7); // as três marcas',
     '*m = (*m & !7) | (*((origem + 0x13) as *const u8) & 3); // mutacao: so duas marcas'),
    ('cpg_salario_minimo', '*((this + 0xc) as *mut i32) = if v < 0x37 { 0x37 } else { v }; // o mínimo de 55',
     '*((this + 0xc) as *mut i32) = if v < 0x38 { 0x38 } else { v }; // mutacao: minimo 56'),
    ('cpg_condicao_porcento', 'return match_pl_ftol(c as f64 * *(0x96bac8 as *const f32) as f64) as u16; // em porcento',
     'return (match_pl_ftol(c as f64 * *(0x96bac8 as *const f32) as f64) - 1) as u16; // mutacao: um ponto a menos'),
]
