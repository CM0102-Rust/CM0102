# Mutacoes (sx16_2, telas: a carreira automatica nao poe notas; provadas pelo comparador numa sessao do Lucas).
# - "One Week" no lugar de "One Month": compare_notes_get_reminder_label acusa ao abrir a tela de nota;
# - a nota com lembrete sem os parenteses do motivo: compare_notes_format_note_text acusa no perfil da pessoa.
MUTACOES = [
    ('sx16_2_um_mes_vira_semana',
     '        2 => 0xa69aa4, // "One Month"',
     '        2 => 0xa69a98, // mutation: "One Week"'),
    ('sx16_2_motivo_sem_parenteses',
     '            sprintf(out, 0xa69a5c, label.as_ptr(), text); // "(%s) %s"',
     '            sprintf(out, 0x9a3d8c, label.as_ptr(), text); // mutation: "%s (%s)"'),
]
