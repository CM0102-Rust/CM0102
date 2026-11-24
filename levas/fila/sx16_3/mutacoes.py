# Mutacoes (sx16_3, telas; provadas pelo comparador numa sessao do Lucas).
# - jovem promissor ate 25 anos: compare_notes_is_reason_valid_for_staff acusa na tela de nota de um jogador de 24;
# - o corpo do lembrete sem o vinculo do nome: compare_text_writer acusa ao abrir a noticia do lembrete.
MUTACOES = [
    ('sx16_3_jovem_ate_25',
     '        1 => player && read_i8(person + staff::AGE) < 0x18,',
     '        1 => player && read_i8(person + staff::AGE) < 0x1a, // mutation'),
    ('sx16_3_lembrete_sem_vinculo',
     '    if exe::database_staff_matches_names(person, first, second, common, 0usize) != 0 {',
     '    if exe::database_staff_matches_names(person, first, second, common, 0usize) == 2 { // mutation'),
]
