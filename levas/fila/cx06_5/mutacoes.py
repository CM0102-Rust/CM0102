# Mutacoes (cx06_5), com as 8 ligas ate 2002-06-01:
# - o segundo de cada grupo e procurado na linha 0 (o primeiro): a tabela dos segundos muda (o comparador lado a lado
#   acusa no dia do sorteio, 10/03/2002 [inferencia], na comparacao de 8 ligas que passe dele);
# - os desempates da fase dos segundos ficam 2, 1, 5 (gols pro antes do saldo): a ordem da tabela dos segundos so pesa
#   no fim dos grupos (out/2003); ate 2002-06-01 a mutacao escapa da carreira e do comparador da tabela (as duas copias
#   usam a fase da original). Fica como prova do molde numa carreira com ATE alem de out/2003.
MUTACOES = [
    ('cx06_5_segundo_errado',
     '        let runner = read_usize(exe::league_get_table_row(group, 1));',
     '        let runner = read_usize(exe::league_get_table_row(group, 0)); // mutation',
     'cx06_5 runner-up row copied'),
    ('cx06_5_desempates_trocados',
     '    let tiebreaks: [u8; 4] = [1, 2, 5, 0];',
     '    let tiebreaks: [u8; 4] = [2, 1, 5, 0]; // mutation',
     'cx06_5 runners-up stage built'),
]
