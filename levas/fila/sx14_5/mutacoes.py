# Mutacoes (sx14_5), de TELAS: a carreira automatica nao abre estas noticias; servem para provar o comparador numa
# sessao do Lucas (compare_text_writer acusa texto ou vinculo diferente).
# - o corpo da oferta troca o modelo de jogador pelo dos demais;
# - a manchete da reacao troca "bloqueio" por "choque".
MUTACOES = [
    ('sx14_5_oferta_modelo_trocado',
     'let template = if job == 0xb || job == 0xf || job == 0xd { 0xa05900 } else { 0xa059e8 };',
     'let template = if job == 0xb || job == 0xf || job == 0xd { 0xa059e8 } else { 0xa05900 }; // mutation',
     'sx14_5 job offer body'),
    ('sx14_5_bloqueio_vira_choque',
     '            0xa06020 // "Board block <name> departure"',
     '            0xa05fe0 // mutation',
     'sx14_5 release reaction headline'),
]
