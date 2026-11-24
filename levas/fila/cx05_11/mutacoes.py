# Mutacoes (cx05_11), carreira de 8 ligas ate 2004-07-15. Os tres comparadores rodam as duas lado a lado: as mutacoes
# sao acusadas pela comparacao (a situacao das selecoes e lida pela avaliacao dos tecnicos das selecoes; as
# classificadas entram pelo fim das eliminatorias, em novembro de 2003).
# - o 8o colocado conta como eliminado nos grupos (-4 e nao 120);
# - a selecao que fica na faixa do meio sobe (1 e nao 0);
# - a classificada entra com o status 5 e nao 6.
MUTACOES = [
    ('cx05_11_oitavo_eliminado',
     '    } else if p >= 9 { // cx05_11: out in the groups',
     '    } else if p >= 8 { // mutation'),
    ('cx05_11_faixa_do_meio_sobe',
     '        [2, 0, -1][band(previous)] // cx05_11: the change from a middle band',
     '        [2, 1, -1][band(previous)] // mutation'),
    ('cx05_11_status_5',
     '    write_u8(entry + 4, 6); // cx05_11: the qualified status',
     '    write_u8(entry + 4, 5); // mutation'),
]
