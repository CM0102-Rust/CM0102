# Prova do agente (05/11): na carreira de 8 ligas até 2002-06-01 (--carreira), o número de camisa só vive no contrato, que
# fica fora do hash diário do banco: as mutações que só mudam o número escapam da carreira e são acusadas na confirmação
# no modo comparando (--confirmar da cadeia; DIVERGENCIA no comparador da própria rotina em 40 a 370 s).
# Mutações (cql_2), nos números que a carreira de 8 ligas grava nos contratos (o hash diário do banco os pega):
# - quem chega e acha livre o número da sua posição também ganha depois um de 12 a 50 por cima (contador
#   "cql_2 position number");
# - a escalação de referência sem goleiro na 1ª vaga (outra ordem do elenco: outros números para quem não tem) (contador
#   "cql_2 picks the team");
# - os números guardados regravados um abaixo (−2, −3...) nos clubes sem técnico humano (contador "cql_2 rewrites negative").
MUTACOES = [
    ('cql2_posicao_e_mais_um', 'return; // cql2: o número da posição', '// mutacao', 'cql_2 position number'),
    ('cql2_escala_sem_goleiro', '0x1, 0x804, 0x84, 0x210, 0x204, 0x204, 0x810, 0x210, 0x240, 0x240, 0x90, // cql2: os starters', '0x4, 0x804, 0x84, 0x210, 0x204, 0x204, 0x810, 0x210, 0x240, 0x240, 0x90, // mutacao', 'cql_2 picks the team'),
    ('cql2_regrava_um_abaixo', 'gravar(this, x, c, v as u8 as u32); // cql2: regrava negativo', 'gravar(this, x, c, v.wrapping_sub(1) as u8 as u32); // mutacao', 'cql_2 rewrites negative'),
]
