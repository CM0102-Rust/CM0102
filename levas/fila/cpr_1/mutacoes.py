# Mutacoes (com o ramo de cada uma). A tabela com so o limit principal guarda o limit de nao comunitarios em +2 (o
# limit da partida vira de estrangeiros: os comunitarios passam a contar e a escalacao automatica refaz o banco nas
# ligas com limit de nao comunitarios). O banco do dia conta tambem os estrangeiros da UE no limit de nao
# comunitarios (mais escalacoes passam do limit e o rodizio 0x6e8910 troca jogadores). Quem tem a permissao de
# trabalho continua precisando dela (a escalacao automatica tira os nao comunitarios com permissao dos clubes
# ingleses). Os ramos dizem se a carreira passou por eles.
MUTACOES = [
    ('cpr1_limite_em_2', '*t.add(5) = v; // o limit de não comunitários',
     '*t.add(2) = v; // mutacao: o limit vai para +2',
     'cpr_1 limit de nao comunitarios'),
    ('cpr1_conta_comunitarios', '1 => estrangeira(p, cpr1_d(clube + 0x53), 1) != 0 && comunitaria(p, cpr1_d(clube + 0x53)) == 0,',
     '1 => estrangeira(p, cpr1_d(clube + 0x53), 1) != 0, // mutacao: conta os da UE',
     'cpr_1 conta nao comunitarios'),
    ('cpr1_permissao_ignorada', 'return 0; // tem a permissão',
     'return 1; // mutacao: a permissao nao vale',
     'cpr_1 permissao achada'),
]
