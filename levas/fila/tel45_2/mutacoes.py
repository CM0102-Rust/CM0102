# A carreira automatica nao abre o editor do programa de treino. Prova do comparador numa sessao do Lucas:
# Treino > editar um programa (o construtor do programa chama o cabecalho; o comparador lado a lado acusa).
MUTACOES = [
    ('tel45_2_cabecalho_0x22', '    write_u32(this + 0xd, 0x21); // tel45_2: the last default', '    write_u32(this + 0xd, 0x22); // mutation: another default'),
]
