# Mutacoes (sx02b_3). Os comparadores rodam lado a lado: cada mutacao muda o valor devolvido e o comparador acusa; na
# carreira Rust, a ordem trocada estraga o bsearch (permissoes e percentuais nao achados) e a ordem das janelas.
MUTACOES = [
    ('sx02b_3_revenda_ao_contrario',
     'let d = read_i32(a + 9).wrapping_sub(read_i32(b + 9)); // sx02b_3: +9, the person id',
     'let d = read_i32(b + 9).wrapping_sub(read_i32(a + 9)); // mutation: people in reverse'),
    ('sx02b_3_janelas_ao_contrario',
     'return ma as i32 - mb as i32; // sx02b_3: by month',
     'return mb as i32 - ma as i32; // mutation: months in reverse'),
    ('sx02b_3_permissao_vazia_primeiro',
     '(true, false) => 1, // sx02b_3: the empty key last',
     '(true, false) => -1, // mutation: the empty key first'),
]
