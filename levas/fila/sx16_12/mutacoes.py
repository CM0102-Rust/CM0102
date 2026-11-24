# Mutacoes (sx16_12). O texto nao volta para a simulacao: acusadas pela comparacao com o modo dos textos das noticias
# (CM_TEXTO_NOTICIAS=1, compare_text_writer).
# - o corpo sem o nome curto do clube (o nome comprido no lugar);
# - a manchete sempre "promoted to reserve squad".
MUTACOES = [
    ('sx16_12_nome_comprido',
     'format(GUI_SCRATCH_TEXT, text, g, g, name.as_ptr(), club.addr() + club::SHORT_NAME); // sx16_12: the promotion sentence',
     'format(GUI_SCRATCH_TEXT, text, g, g, name.as_ptr(), club.addr() + club::NAME); // mutation: the long name',
     'sx16_12 youth promotion body'),
    ('sx16_12_manchete_fixa',
     'format(GUI_SCRATCH_TEXT, if academy != 0 { 0xa6c708 } else { 0xa6c6cc }, name.as_ptr()); // promoted from youth academy / to reserve squad',
     'format(GUI_SCRATCH_TEXT, 0xa6c6cc, name.as_ptr()); // mutation: always to the reserve squad',
     'sx16_12 youth promotion headline'),
]
