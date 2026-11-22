# Mutacoes (tel18_5): provam os comparadores numa sessao do Lucas (JOGAR_COMPARANDO.bat):
# - a tela da proposta de emprestimo pinta a contribuicao em dinheiro "(...)" de cinza claro, nao laranja (perfil de um
#   jogador de outro clube > Transfer > Make Loan Offer);
# - o "-" da contribuicao salarial tira 5, nao 10 (a mesma tela > "-" de Wage Contribution).
MUTACOES = [
    ('tel18_5_contribuicao_sem_laranja',
     "ts_at(area, 3, row, 1, 0, 0xc, TS_COLOUR_ORANGE, TS_TEXT);",
     "ts_at(area, 3, row, 1, 0, 0xc, G_GUI_COLOUR_PALE_GREY, TS_TEXT); // mutation: pale instead of orange",
     'tel18_5 loan bid screen'),
    ('tel18_5_contribuicao_passo_5',
     "share.wrapping_sub(10)",
     "share.wrapping_sub(5) /* mutation: steps of 5 */",
     'tel18_5 loan bid share step'),
]
