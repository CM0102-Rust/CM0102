# Mutacoes (cx03_14), carreira de 8 ligas. As noticias so a comparacao acusa (o texto nao volta para a simulacao; modo
# CM_TEXTO_NOTICIAS). Nenhuma abre caixa.
# - a Merconorte com o texto da Libertadores ("through to second round"): o texto muda;
# - a Libertadores com o mata-mata na fase 6: a fase 6 dos grupos passa ao texto do confronto;
# - a Mercosul com o texto das semifinais: o texto muda.
MUTACOES = [
    ('cx03_14_merconorte_texto_da_libertadores',
     'news, text, 3, 0x9a7dac, "cx03_14 merconorte through to semi finals")',
     'news, text, 3, 0x9a7e24, "cx03_14 merconorte through to semi finals") // mutation'),
    ('cx03_14_libertadores_mata_mata_na_fase_6',
     'news, text, 7, 0x9a7e24, "cx03_14 libertadores through to second round")',
     'news, text, 6, 0x9a7e24, "cx03_14 libertadores through to second round") // mutation'),
    ('cx03_14_mercosul_texto_das_semifinais',
     'news, text, 4, 0x9a7e9c, "cx03_14 mercosur through to quarter finals")',
     'news, text, 4, 0x9a7dac, "cx03_14 mercosur through to quarter finals") // mutation'),
]
