# Mutacoes (rx11b_3), item de telas: acusadas pelos comparadores quando o Lucas pede o relatorio do fisioterapeuta e
# abre a noticia (a avaliacao lado a lado, a noticia guardada na caixa contra o plano, o texto lado a lado).
MUTACOES = [
    ('rx11b_3_fisico_acima_de_13',
     'if fitness > 0xf {',
     'if fitness > 0xd { // mutation',
     'rx11b_3 physio report filed'),
    ('rx11b_3_estagio_da_idade_na_vaga_errada',
     'set_news_slot(news, 0xa, stage as u32); // rx11b_3: the age stage',
     'set_news_slot(news, 0xa, familiarity as u32); // mutation',
     'rx11b_3 physio report filed'),
    ('rx11b_3_atitude_trocada',
     'fmt(ap, 0xa6ac04, 0x64, pl5); // "professional approach to his health"',
     'fmt(ap, 0xa6ab9c, 0x64, pl5); // mutation',
     'rx11b_3 physio report body'),
]
