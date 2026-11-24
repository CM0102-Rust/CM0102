# Mutacoes (sbD_1). O texto nao volta para a simulacao: acusadas pela comparacao do escritor (compare_text_writer:
# o texto, os vinculos e o gerador) com o modo CM_TEXTO_NOTICIAS na carreira automatica (CM_TEXTOS=1 na cadeia):
# - o presidente: a paridade invertida na redacao 0;
# - o vestiario: as redacoes 2 e 3 trocadas;
# - o papel: os modelos trocados.
MUTACOES = [
    ('sbD_1_nao_vende_paridade', '            0 if quoted && (chairman.id as i32) % 2 != 0 =>', '            0 if quoted && (chairman.id as i32) % 2 == 0 => // mutation\n', 'sbD_1 not for sale text body'),
    ('sbD_1_vestiario_redacoes', '                    2 => 0x9c592c,\n                    3 => 0x9c58a8,', '                    2 => 0x9c58a8, // mutation\n                    3 => 0x9c592c,', 'sbD_1 dressing room unrest text body'),
    ('sbD_1_papel_redacoes', '|listed| if listed as u8 != 0 { 0x9c5b80 } else { 0x9c5b00 }', '|listed| if listed as u8 != 0 { 0x9c5b00 } else { 0x9c5b80 } /* mutation */', 'sbD_1 unhappy with role text'),
]
