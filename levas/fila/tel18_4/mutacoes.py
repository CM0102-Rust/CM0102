# Mutacoes (tel18_4): provam os comparadores numa sessao do Lucas (JOGAR_COMPARANDO.bat):
# - a janela de dispensa nao divide a indenizacao por 4 quando o jogador quer sair (perfil de um jogador seu que pediu
#   para sair > Release);
# - a janela do periodo de teste cinza o "+" com 3 semanas (perfil de um jogador sem clube > Offer Trial > +).
MUTACOES = [
    ('tel18_4_indenizacao_inteira',
     "pay /= 4;",
     "pay /= 2; // mutation: half instead of a quarter",
     'tel18_4 release dialog'),
    ('tel18_4_teste_mais_cinza',
     "    if weeks == 4 {\n        tel26_area_text([area, 4, 0, 0x30, dark, 0xc, 3, grey, TS_PLUS as u32]);",
     "    if weeks >= 3 { // mutation: \"+\" greyed at 3 weeks\n        tel26_area_text([area, 4, 0, 0x30, dark, 0xc, 3, grey, TS_PLUS as u32]);",
     'tel18_4 offer trial dialog'),
]
