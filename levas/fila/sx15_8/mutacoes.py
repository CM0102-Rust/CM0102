# Mutacoes (sx15_8) pela prova do save (mutacoes.py --prova-save N), com a carga do FeriasTesteSave (CARGA): a impressao
# depois da carga com a mutacao contra a base (a mesma DLL sem as rotinas da leva). Sem o 4o campo (contador): a base da
# prova do save roda com as rotinas desligadas.
# - a carga devolvendo 0 no fim: o construtor roda o init_tables por cima do que leu (selecoes escolhidas de novo, com
#   sorteios; as convocacoes mudam), sem caixa;
# - o pool de reservas com a pessoa seguinte (id + 1): as convocacoes das selecoes saem de outras pessoas depois da carga.
MUTACOES = [
    ('sx158_carga_refeita', '    1 // sx15_8: loaded (the constructor does not rebuild the tables)',
     '    0 // mutation: the constructor rebuilds the tables'),
    ('sx158_pool_deslocado', '        write_usize(slot, if id != -1 { staff_at(id as isize).addr() } else { 0 }); // sx15_8: id to person',
     '        write_usize(slot, if id != -1 { staff_at(id as isize + 1).addr() } else { 0 }); // mutation: the next person'),
]
