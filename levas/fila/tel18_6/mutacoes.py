# Mutacoes (tel18_6): provam os comparadores numa sessao do Lucas (JOGAR_COMPARANDO.bat):
# - a confirmacao do periodo de teste diz "1 week" com 2 semanas (perfil de um jogador sem clube > Offer Trial >
#   Offer, com as 2 semanas iniciais);
# - a janela de encerrar o emprestimo troca "Recall" por "Terminate" (perfil de um jogador seu emprestado a outro
#   clube > Recall From Loan).
MUTACOES = [
    ('tel18_6_teste_uma_semana',
     "        if weeks == 1 {\n            lang(TS_TEXT, 0xaa14ac, n);",
     "        if weeks == 2 { // mutation: \"1 week\" at 2 weeks\n            lang(TS_TEXT, 0xaa14ac, n);",
     'tel18_6 confirm offer dialog'),
    ('tel18_6_chamar_de_volta_rescindir',
     "        lang(TS_TEXT, 0xaa16d4, n);",
     "        lang(TS_TEXT, 0xaa170c, 0, n); // mutation: the terminate subtitle on a recall",
     'tel18_6 end loan dialog'),
]
