# Mutacoes (rx13_8): o texto nao volta a simulacao; acusa o comparador do texto (compare_text_writer) com o modo de
# textos das noticias (CM_TEXTO_NOTICIAS, --textos) e um tecnico humano como destinatario, quando os campeonatos
# brasileiros de 2001 decidem as fases (8 ligas):
# - o vice da final recebe a manchete do campeao;
# - a manchete da segunda fase da terceira divisao sem o vinculo da competicao.
MUTACOES = [
    ('rx13_8_vice_como_campeao',
     '2 => Some(BRA_LOSE_FINAL_TEXT), // rx13_8: the final lost',
     '2 => Some(BRA_WIN_TEXT), // mutation: the winner text',
     'rx13_8 brazilian final headline'),
    ('rx13_8_terceira_sem_vinculo',
     'return fate_headline(text, club_id, read_usize(this + 4), competition::SHORT_NAME, t, long, true);',
     'return fate_headline(text, club_id, read_usize(this + 4), competition::SHORT_NAME, t, long, false); // mutation',
     'rx13_8 brazilian third second round headline'),
]
