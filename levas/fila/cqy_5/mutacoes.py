# Mutacoes (cqy_5): texto de noticia, acusado pelo comparador lado a lado (compare_text_writer), nao pela carreira:
# - "cai em" so com o resultado 2 (o 4 e o -1 passam a "vence em");
# - o 7 deixa de contar como classificado (nao escreve nada).
MUTACOES = [
    ('cqy5_derrota_so_com_2',
     '2 | 4 | 0xff => 0x9a3dbc, // cqy5: "defeat in"',
     '2 => 0x9a3dbc, // mutation: only 2 is a defeat',
     'cqy_5 cup win or defeat news'),
    ('cqy5_sete_nao_classifica',
     '6 | 7 => true, // cqy5: qualified',
     '6 => true, // mutation: 7 writes nothing',
     'cqy_5 group news with the stage'),
]
