# Mutacoes (cx04_12). O texto nao volta para a simulacao: acusadas pela comparacao do escritor (compare_text_writer: o
# texto, os vinculos e o gerador) com o modo CM_TEXTO_NOTICIAS na carreira automatica:
# - o olheiro: as redacoes 0 e 1 trocadas no corpo;
# - o sistema de transferencias: o corpo da mudanca no lugar do da corrida (tipo 0);
# - o seguro: o vinculo do clube com o tipo de pessoa (0x7d7).
MUTACOES = [
    ('cx04_12_olheiro_redacoes_trocadas',
     '            0 => 0x9c4b04,\n            1 => 0x9c4b9c,',
     '            0 => 0x9c4b9c, // mutation\n            1 => 0x9c4b04,',
     'cx04_12 scout watching text body'),
    ('cx04_12_sistema_corpo_trocado',
     '        (true, false) => 0x9c705c,',
     '        (true, false) => 0x9c71b4, // mutation',
     'cx04_12 transfer system text'),
    ('cx04_12_seguro_vinculo_pessoa',
     '            exe::text_add_link(text, (club.addr() + club::NAME) as *const u8, 0x7d5, club.addr());',
     '            exe::text_add_link(text, (club.addr() + club::NAME) as *const u8, 0x7d7, club.addr()); // mutation',
     'cx04_12 insurance payout text body'),
]
