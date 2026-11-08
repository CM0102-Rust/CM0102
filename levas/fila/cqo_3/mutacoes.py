# Mutações (cqo_3): a tabela padrão roda uma vez, no começo do jogo novo; os dias das decisões e os anos das entradas
# decidem quando o calendário das sedes (0x5fa850) sorteia a candidatura vencedora (0x90cfc0(3)).
# - o primeiro dia sem o sorteio (um sorteio a menos no começo do jogo) (contador "cqo_3 day draw");
# - a Copa Africana de 4 em 4 anos em vez de 2 em 2 (as edições a decidir mudam). Tentativas anteriores: a Copa do
#   Mundo decidida um mês depois escapou (nenhuma decisão dela cai na janela da carreira, julho de 2001 a junho de
#   2002); a Eurocopa desde 2002 derrubava na caixa de erro de euro_champ.cpp (linha 837: edição sem sede); as edições
#   anuais ([0x9cf95c]) de 2 em 2 escaparam (nenhuma decisão delas na janela);
# - a Copa América começando em 2001 em vez de 1999 (as edições de 2006/2009 e as decisões mudam de ano).
MUTACOES = [
    ('cqo3_sorteio_a_menos', 'let r = sortear(0x46); // cqo3: o sorteio do dia da decisão', 'let r = if k == 0 { 0x23 } else { sortear(0x46) }; // mutacao: o primeiro sem o sorteio', 'cqo_3 day draw'),
    ('cqo3_africana_de_4_em_4', '(0x220, 8, 0x9cf7ac, 2000, 2, 0x10), // cqo3: a Copa Africana', '(0x220, 8, 0x9cf7ac, 2000, 4, 0x10), // mutacao: de 4 em 4', 'cqo_3 day draw'),
    ('cqo3_copa_america_2001', '(0x462, 6, 0x9cf7a8, 1999, 2, 0xc), // cqo3: a Copa América', '(0x462, 6, 0x9cf7a8, 2001, 2, 0xc), // mutacao: desde 2001', 'cqo_3 day draw'),
]
