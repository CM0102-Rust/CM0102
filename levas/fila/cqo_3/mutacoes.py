# Mutações (cqo_3): a tabela padrão roda uma vez, no começo do jogo novo; os dias das decisões e os anos das entradas
# decidem quando o calendário das sedes (0x5fa850) sorteia a candidatura vencedora (0x90cfc0(3)).
# - o primeiro dia sem o sorteio (um sorteio a menos no começo do jogo) (contador "cqo_3 sorteio do dia");
# - a Eurocopa começando em 2002 em vez de 2000 (as edições a decidir mudam de ano). A primeira tentativa (a Copa do
#   Mundo decidida um mês depois) escapou: nenhuma decisão da Copa do Mundo cai na janela da carreira (julho de 2001 a
#   junho de 2002) nem com o dia andado;
# - a Copa América começando em 2001 em vez de 1999 (as edições de 2006/2009 e as decisões mudam de ano).
MUTACOES = [
    ('cqo3_sorteio_a_menos', 'let r = sortear(0x46); // cqo3: o sorteio do dia da decisão', 'let r = if k == 0 { 0x23 } else { sortear(0x46) }; // mutacao: o primeiro sem o sorteio', 'cqo_3 sorteio do dia'),
    ('cqo3_eurocopa_2002', '(0x110, 8, 0x9cf7a4, 2000, 4, 0x20), // cqo3: a Eurocopa', '(0x110, 8, 0x9cf7a4, 2002, 4, 0x20), // mutacao: desde 2002', 'cqo_3 sorteio do dia'),
    ('cqo3_copa_america_2001', '(0x462, 6, 0x9cf7a8, 1999, 2, 0xc), // cqo3: a Copa América', '(0x462, 6, 0x9cf7a8, 2001, 2, 0xc), // mutacao: desde 2001', 'cqo_3 sorteio do dia'),
]
