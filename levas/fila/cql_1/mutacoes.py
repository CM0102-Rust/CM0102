# Prova do agente (05/11): na carreira de 8 ligas até 2002-06-01 (--carreira), o número de camisa só vive no contrato, que
# fica fora do hash diário do banco: as mutações que só mudam o número escapam da carreira e são acusadas na confirmação
# no modo comparando (--confirmar da cadeia; DIVERGENCIA no comparador da própria rotina em 40 a 370 s).
# Mutações (cql_1), nos números de camisa que a carreira de 8 ligas grava nos contratos (o hash diário do banco os pega):
# - o número do contrato lido como 0: a 0x85a470 (dar número a quem chega) acha que o jogador não tem número e lhe dá
#   outro, e a lista dos números ocupados do elenco fica sem os dele (contador "cql_1 contract number");
# - nenhum clube usa números: quem chega por transferência ou empréstimo fica sem número (contador "cql_1 uses numbers");
# - o número gravado com o sinal trocado (contador "cql_1 writes into contract").
MUTACOES = [
    ('cql1_numero_lido_zero', 'return *((k + 0x3a) as *const i8); // cql1: o número do contrato', 'return 0; // mutacao', 'cql_1 contract number'),
    ('cql1_ninguem_usa_numeros', '1 // cql1: usa números', '0 // mutacao', 'cql_1 uses numbers'),
    ('cql1_numero_com_sinal_trocado', '*((k + 0x3a) as *mut i8) = n; // cql1: grava no contrato', '*((k + 0x3a) as *mut i8) = n.wrapping_neg(); // mutacao', 'cql_1 writes into contract'),
]
