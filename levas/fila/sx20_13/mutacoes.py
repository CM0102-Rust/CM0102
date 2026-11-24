# Mutacoes (sx20_13), item de TELAS: a carreira automatica nao abre a busca; servem para provar o comparador numa
# sessao do Lucas jogando comparando (Buscar jogadores):
# - o cargo padrao 4 (recomecar os filtros gerais, ou abrir uma busca nova);
# - o minimo lido no atributo seguinte (a tela de busca mostra os minimos de cada atributo);
# - a opcao de faixas nao e ligada na primeira subida de um minimo de atributo.
MUTACOES = [
    ('sx20_13_cargo_padrao',
     'write_u8(this + SEARCH_JOB, 5); // sx20_13: the default job filter',
     'write_u8(this + SEARCH_JOB, 4); // mutation: job 4'),
    ('sx20_13_minimo_seguinte',
     'read_u8(this + minimums + attribute as usize) // sx20_13: the minimum read',
     'read_u8(this + minimums + attribute as usize + 1) // mutation: the next attribute'),
    ('sx20_13_sem_opcao',
     'search_set_option_flag(this, 0, on); // sx20_13: the range option on',
     'let _ = on; // mutation: the range option stays off'),
]
