# Mutacoes (tel17_2): provam os comparadores numa sessao do Lucas (JOGAR_COMPARANDO.bat):
# - a confirmacao de negociar mostra o texto de cancelar (tela de proposta respondendo a uma oferta > Negotiate);
# - o motivo 0x200 da recusa fica cinza (tela de proposta respondendo a uma oferta > Reject).
MUTACOES = [
    ('tel17_2_texto_negociar',
     "lang(TS_TEXT, 0xa9eac0, n);",
     "lang(TS_TEXT, 0xa9eb04, n); // mutation: the cancel text",
     'tel17_2 bid confirm dialog'),
    ('tel17_2_motivo_cinza',
     "reason_row(0x200, true);",
     "reason_row(0x200, false); // mutation: reason 0x200 greyed",
     'tel17_2 reject bid dialog'),
]
