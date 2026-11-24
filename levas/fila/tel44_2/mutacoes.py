# Mutações (tel44_2), acusadas pelos comparadores (compare_order: o valor lado a lado a cada chamada do qsort) e, quando
# a ordem muda quem fica nos primeiros lugares, pela carreira ENG até 2002-06-01 (os vencedores do prêmio):
# - por nome: os dois nomes trocados (a ordem inversa);
# - por nota: a menor primeiro (o vencedor por nota muda);
# - por votos: o menos votado primeiro (os vencedores mudam).
MUTACOES = [
    ('tel44_2_name_order',
     "exe::utils_string_compare(name_a.as_ptr(), name_b.as_ptr(), 1, 0) // tel44_2: a's name against b's",
     "exe::utils_string_compare(name_b.as_ptr(), name_a.as_ptr(), 1, 0) // mutation: the reverse order"),
    ('tel44_2_score_order',
     "-1 // tel44_2: the higher score first",
     "1 // mutation: the lower score first"),
    ('tel44_2_votes_order',
     "read_i16(b as usize + asl::ENTRY_VOTES) as i32 - read_i16(a as usize + asl::ENTRY_VOTES) as i32 // tel44_2: the most votes first",
     "read_i16(a as usize + asl::ENTRY_VOTES) as i32 - read_i16(b as usize + asl::ENTRY_VOTES) as i32 // mutation: the fewest votes first"),
]
