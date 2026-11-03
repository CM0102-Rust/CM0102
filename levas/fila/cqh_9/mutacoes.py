# Mutacoes (cqh_9): o player_setup.cfg aplicado na criacao do jogo da carreira automatica (as lesoes, suspensoes,
# aposentadorias, transferencias e emprestimos das linhas mudam o elenco e o mercado desde o comeco).
# - A linha INJURY vira o tipo 2 (MANAGER_CONF): as lesoes nao sao aplicadas.
# - O jogador do elenco achado sem conferir o segundo nome (o primeiro do elenco com o mesmo primeiro nome e nome
#   comum; a busca por clube e a das linhas, 128 ou mais na carreira).
# - O texto entre aspas em minusculas: os nomes nao batem mais.
MUTACOES = [
    ('cqh9_lesao_vira_tecnico', '(0xa6b3a0, 1),      // INJURY',
     '(0xa6b3a0, 2),      // mutacao INJURY', 'cqh_9 tipo da linha'),
    ('cqh9_elenco_sem_segundo_nome', 'if q != 0 && cqh9_bate(q, 4, primeiro) && cqh9_bate(q, 8, segundo) && cqh9_bate(q, 0xc, comum) {',
     'if q != 0 && cqh9_bate(q, 4, primeiro) && cqh9_bate(q, 0xc, comum) {', 'cqh_9 jogador do clube pelo nome'),
    ('cqh9_texto_minusculo', '*((destino + n) as *mut u8) = c; // cqh9: o caractere do texto',
     '*((destino + n) as *mut u8) = c.to_ascii_lowercase(); // mutacao: o caractere do texto', 'cqh_9 texto entre aspas'),
]
