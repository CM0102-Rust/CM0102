# Mutacoes (sx15_9) pela prova do save (mutacoes.py --prova-save N): o .qst gravado no modo rust com a mutacao contra a
# base. Sem o 4o campo (contador): a base da prova do save roda com as rotinas desligadas.
# - as marcas olimpicas com uma nacao a menos (conta e conferencia juntas, sem caixa): o national_teams.dat sai 1 byte
#   menor;
# - a conta das convocacoes gravada com +1: os 2 bytes da conta mudam no .qst.
# O destrutor fica sem mutacao: o que ele faz (liberar) nao aparece no .qst nem na impressao.
MUTACOES = [
    ('sx159_marcas_curtas', '    let marks = read_u32(G_NATION_COUNT); // sx15_9: the Olympic marks, one byte per nation',
     '    let marks = read_u32(G_NATION_COUNT) - 1; // mutation: one nation fewer'),
    ('sx159_conta_mais_um', '        count = read_u16(list + llist::COUNT) as u32;',
     '        count = read_u16(list + llist::COUNT) as u32 + 1; // mutation: one call-up more'),
]
