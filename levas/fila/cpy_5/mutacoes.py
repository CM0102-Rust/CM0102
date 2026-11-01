# Mutacoes (cpy_5). A 0x584fa0 roda no construtor da Liga dos Campeoes em toda carreira nova (0x583888): a lista dos
# 71 inscritos vira a tabela da qualificacao (0x584990, linhas 70..16 da lista, as bandas 2, 3 e 4) com a rodada em
# que cada clube entra (8 -> 2, 0xd -> 1, 0 -> 0), que os sorteios das tres rodadas da qualificacao usam (julho e
# agosto de 2001). As tres trocam a rodada de entrada de uma banda inteira: outros clubes em cada rodada, outros
# jogos, outros classificados para a fase de grupos (premios de 200 mil por vitoria, coeficiente, elenco cansado),
# que a carreira le. Os contadores ficam dentro de cada banda, colados na entrada gravada, depois das condicoes (a
# competicao do clube, a banda e a vaga).
MUTACOES = [
    ('cpy5_banda2_na_rodada_0xd',
     'cpy5_entrada(t + 0x60 + o2, c, 8); // cpy5: a banda 2 entra na rodada 8',
     'cpy5_entrada(t + 0x60 + o2, c, 0xd); // mutacao: a banda 2 entra com a 3',
     'cpy_5 entrada da banda 2'),
    ('cpy5_banda3_na_rodada_8',
     'cpy5_entrada(t + 0xcc + o3, c, 0xd); // cpy5: a banda 3 entra na rodada 0xd',
     'cpy5_entrada(t + 0xcc + o3, c, 8); // mutacao: a banda 3 entra com a 2',
     'cpy_5 entrada da banda 3'),
    ('cpy5_banda4_na_rodada_0xd',
     'cpy5_entrada(t + 0x13e + o4, c, 0); // cpy5: a banda 4 entra na rodada 0',
     'cpy5_entrada(t + 0x13e + o4, c, 0xd); // mutacao: a banda 4 entra com a 3',
     'cpy_5 entrada da banda 4'),
]
