# Mutacoes (cx01_6) pela prova do save (mutacoes.py --prova-save N; CARGA). Nenhuma abre caixa; o banco (.dat) nao e
# tocado.
# - a primeira busca pula o primeiro .tmp achado: o .qst gravado fica sem ele (e o comparador acusa o nome diferente);
# - a busca seguinte pula todo .tmp: o .qst fica so com o primeiro .tmp de cada pasta, a carga muda.
# O fechamento so libera o identificador do Windows: sem mutacao que mude o jogo (so a original roda no comparador).
MUTACOES = [
    ('cx01_6_primeira_sem_tmp',
     '    covered!("cx01_6 find first");\n    find_take_matching(state, data.as_mut_ptr() as usize)',
     '    covered!("cx01_6 find first");\n    let r = find_take_matching(state, data.as_mut_ptr() as usize); // mutation\n'
     '    if r == 0 && c_string_bytes(state + file_find_state::FILE_NAME).ends_with(b".tmp") { file_find_next(state) } else { r }'),
    ('cx01_6_seguinte_sem_tmp',
     '    covered!("cx01_6 find next");\n    find_take_matching(state, d)',
     '    covered!("cx01_6 find next");\n    let r = find_take_matching(state, d); // mutation\n'
     '    if r == 0 && c_string_bytes(state + file_find_state::FILE_NAME).ends_with(b".tmp") { file_find_next(state) } else { r }'),
]
