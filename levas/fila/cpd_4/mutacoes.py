# Mutacoes das reservas das selecoes. A da posicao pela nota poe o que entra antes do de nota maior (so com o trecho
# cheio, ramo "reserves full enters by rating"; 6 chamadas): muda quem a 0x775f90 convoca primeiro; PODE ESCAPAR se
# nenhuma convocacao usar o trecho trocado. A da ordem das reservas inverte o qsort (crescente): as reservas ficam
# de tras para a frente, os substitutos mudam, deve acusar. A da ordem pelo clube inverte os clubes (qsort das
# rotinas das selecoes, milhares de chamadas): deve acusar se a ordem mudar alguma escolha.
MUTACOES = [
    ('cpd_reservas_posicao', 'pos = j + 1; // depois das de nota maior', 'pos = j; // mutacao: antes da de nota maior'),
    ('cpd_ordem_reservas', '(slot_person_rating(b) as i32) - slot_person_rating(a) as i32 // decrescente', '(slot_person_rating(a) as i32) - slot_person_rating(b) as i32 // mutacao: crescente'),
    ('cpd_ordem_clube', 'return (*(cx as *const i32)).wrapping_sub(*(cy as *const i32)); // o id do clube', 'return (*(cy as *const i32)).wrapping_sub(*(cx as *const i32)); // mutacao: ao contrario'),
]
