# Mutações (tel31_4). A carreira automática não abre tela: a cadeia não roda nenhuma; estas provam o comparador numa
# sessão do Lucas (JOGAR_COMPARANDO.bat): qualquer texto quebrado em mais de uma linha pega a primeira, qualquer
# texto centrado (títulos de caixa de mensagem) pega a segunda.
# - o corte no espaço tira o espaço do fim da linha;
# - o centro calculado com um terço da sobra.
MUTACOES = [
    ('tel31_4_cut_drops_space', 'write_u8(line + j as usize + 1, 0);', 'write_u8(line + j as usize, 0);'),
    ('tel31_4_centre_third', 'left[k] = x1 + (x2 - x1 + 1 - w) / 2;', 'left[k] = x1 + (x2 - x1 + 1 - w) / 3;'),
]
