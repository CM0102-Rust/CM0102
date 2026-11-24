# Mutacoes (sx09c_2) pela prova do save (mutacoes.py --prova-save N), sem o 4o campo (a base da prova roda com as
# rotinas desligadas):
# - a gravacao poe a quantidade de clubes mais um no fine.dat: o item sai com 4 bytes diferentes em qualquer save;
# - a carga devolve 0 no fim: o construtor joga fora as multas, os retornos do tecnico e as contagens lidas e comeca
#   vazio; so a carga do FeriasTesteSave (CARGA) acusa, e so se o save tiver multas ou contagens (o tecnico da IA multa
#   quem falta ao treino): pode escapar se o save de ferias nao tiver nenhuma.
MUTACOES = [
    ('sx09c_clubes_mais_um',
     'if exe::zwrite(G_CLUB_COUNT, 4, 1, z) != 1 {',
     'let wrong = read_u32(G_CLUB_COUNT) + 1; if exe::zwrite(&wrong as *const u32 as usize, 4, 1, z) != 1 { // mutation'),
    ('sx09c_carga_das_multas_falha',
     '    exe::zclose(&mut z);\n    1\n}\n\n/// Comparer for `fine_load`',
     '    exe::zclose(&mut z);\n    0 // mutation\n}\n\n/// Comparer for `fine_load`'),
]
