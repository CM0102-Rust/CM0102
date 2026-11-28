# Mutacoes (r7_01_1). O texto nao volta a simulacao: acusadas pelo comparador lado a lado do modificador de gramatica
# (o guia conta como acusada); na cadeia, roda-las com a comparacao ligada. O jogo de teste esta em portugues, e o
# language.ldb usa em portugues "do" 2.281 vezes, "o" 1.327, "O" 545, "no" 492, "ao" 140, "lower" 82.
# - o genero feminino lido como masculino ("do" no lugar de "da", "o" no lugar de "a");
# - "lower" sem efeito (o texto fica na caixa original);
# - "ao" com as formas de "no" ("no " no lugar de "ao ").
MUTACOES = [
    ('r701_feminino_como_masculino',
     '        0..=4 => grammar_string(rule.forms[gender as usize]),',
     '        0..=4 => grammar_string(rule.forms[if gender == 1 { 0 } else { gender as usize }]), // mutation',
     'r7_01_1 portuguese article'),
    ('r701_lower_sem_efeito',
     '            *c = exe::at_9465fc(*c as i8 as i32) as u8;',
     '            let _ = exe::at_9465fc(*c as i8 as i32); // mutation',
     'r7_01_1 portuguese lower'),
    ('r701_ao_como_no',
     '    PortugueseRule { marks: &[0x9eed80], forms: [0x9eed7c, 0x9f1360, 0x9eed74, 0x9eed70, 0x9f15d4] },',
     '    PortugueseRule { marks: &[0x9eed80], forms: [0x9eee08, 0x9eee04, 0x9eedfc, 0x9eedf4, 0x9eee0c] }, // mutation',
     'r7_01_1 portuguese article'),
]
