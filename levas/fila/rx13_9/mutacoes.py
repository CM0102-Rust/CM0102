# Mutacoes (rx13_9): o texto nao volta a simulacao; acusa o comparador do texto (compare_text_writer) com o modo de
# textos das noticias (CM_TEXTO_NOTICIAS, --textos) e um tecnico humano como destinatario (8 ligas: os estaduais de 2002
# no primeiro semestre, a MLS de 2001 de setembro a outubro):
# - o campeao do Carioca com o vinculo da competicao;
# - o vencedor da final da MLS com a manchete da semifinal.
MUTACOES = [
    ('rx13_9_campeao_com_vinculo',
     'STATE_CHAMPIONSHIP_TEXT, true, false); // rx13_9: only the club\'s link',
     'STATE_CHAMPIONSHIP_TEXT, true, true); // mutation',
     'rx13_9 state league champion headline'),
    ('rx13_9_mls_final_como_semifinal',
     '(3, 1) => Some(MLS_WIN_TEXT),',
     '(3, 1) => Some(MLS_FINAL_TEXT), // mutation',
     'rx13_9 mls final headline'),
]
