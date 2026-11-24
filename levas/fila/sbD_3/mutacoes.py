# Mutacoes (sbD_3). O texto nao volta para a simulacao: acusadas pela comparacao do escritor (compare_text_writer:
# o texto, os vinculos e o gerador) com o modo CM_TEXTO_NOTICIAS na carreira automatica (CM_TEXTOS=1 na cadeia):
# - a gratidao: o jornal local e o nacional trocados;
# - a virada: o vinculo do adversario aponta o clube;
# - a chance: a redacao invertida.
MUTACOES = [
    ('sbD_3_gratidao_jornal', 'fame == 1, 0x9c666c, 0x9c6640)', 'fame != 1, 0x9c666c, 0x9c6640) /* mutation */', 'sbD_3 thanks manager text body'),
    ('sbD_3_virada_adversario', '    exe::text_add_link(text, opponent.name_addr() as *const u8, 0x7d5, opponent);', '    exe::text_add_link(text, opponent.name_addr() as *const u8, 0x7d5, club); // mutation', 'sbD_3 ecstatic at turnaround text body'),
    ('sbD_3_chance_redacao', '    if wording != 0 {\n        story_lang!(out.as_mut_ptr(), 0x9c6a1c,', '    if wording == 0 { // mutation\n        story_lang!(out.as_mut_ptr(), 0x9c6a1c,', 'sbD_3 pleased with chance text body'),
]
