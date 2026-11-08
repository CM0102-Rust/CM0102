# Prova do agente (05/11): na carreira de 8 ligas até 2002-06-01 (--carreira), o número de camisa só vive no contrato, que
# fica fora do hash diário do banco: as mutações que só mudam o número escapam da carreira e são acusadas na confirmação
# no modo comparando (--confirmar da cadeia; DIVERGENCIA no comparador da própria rotina em 40 a 370 s).
# Mutações (cql_3), no que a carreira de 8 ligas grava no banco (o hash diário pega):
# - no começo do jogo nenhuma liga é numerada (os números ficam os do banco, sem a numeração do elenco) (contador
#   "cql_3 numbers the league");
# - o número do banco lido como 0 no começo (a numeração do elenco dá a todos o da ordem) (contador
#   "cql_3 bench number");
# - nas etapas depois da primeira (a 3ª, pela 0x85a9b0, em 2002), a marca de quem é inscrito trocada (1 <-> 2) (contador
#   "cql_3 registers later"; a etapa 1 quase não roda na carreira: só pela notícia do clube humano).
MUTACOES = [
    ('cql3_nenhuma_liga_numerada', 'cada(c); // cql3: numera a liga', '// mutacao', 'cql_3 numbers the league'),
    ('cql3_numero_do_banco_zero', 'gravar(this, p, c, k as u32); // cql3: o número do banco', 'gravar(this, p, c, 0); // mutacao', 'cql_3 bench number'),
    ('cql3_inscricao_trocada', '*m6d(p) = v; // cql3: inscreve depois', '*m6d(p) = v ^ 3; // mutacao', 'cql_3 registers later'),
]
