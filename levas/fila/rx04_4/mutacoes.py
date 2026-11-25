# Mutacoes (rx04_4). Rodam na abertura (a configuracao pergunta se a lingua do game.cfg existe; a carga monta a tabela).
# - a lingua disponivel procurada como id + 1 (o comparador da disponivel acusa; a configuracao pode cair no ingles);
# - a entrada nova sem zerar a traducao (o comparador do construtor acusa: a copia fica com 0xcd ali; com a comparacao
#   a tabela e a da original).
# O destrutor fica sem mutacao: so libera (nem a carreira nem o comparador veem uma liberacao a menos, e uma a mais
# derruba o jogo).
MUTACOES = [
    ('rx044_disponivel_id_mais_um',
     '    (0..n.max(0) as usize).any(|k| read_i32(G_LANGLIB_AVAILABLE + 4 * k) == id) as u32',
     '    (0..n.max(0) as usize).any(|k| read_i32(G_LANGLIB_AVAILABLE + 4 * k) == id + 1) as u32 // mutation',
     'rx04_4 language available'),
    ('rx044_entrada_sem_zerar_a_traducao',
     '    for slot in [LANGLIB_ENTRY_KEY, LANGLIB_ENTRY_ARG_FORMAT, LANGLIB_ENTRY_TRANSLATION] {\n        write_usize(this + slot, 0);',
     '    for slot in [LANGLIB_ENTRY_KEY, LANGLIB_ENTRY_ARG_FORMAT] { // mutation\n        write_usize(this + slot, 0);',
     'rx04_4 entry built'),
]
