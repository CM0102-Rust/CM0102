# Mutacoes das consultas do conteiner. A gravacao do .qst nao passa por nenhuma das tres: o padrao
# roda na carga (zextract "*.lng" no load_database e "*.tmp" no load_game_data; zget_matching_size
# "*.tmp"), entao so a etapa de carga da prova do save (--sav, o save com tecnico de ferias) as pega.
# O id so roda na caixa de arquivos (cliques) e a soma do tamanho so muda a conferencia de espaco
# (passa com qualquer valor pequeno): sem mutacao propria.
MUTACOES = [
    ('sv_padrao_letra', "            if maiuscula(c as u32) != n {", "            if maiuscula(c as u32 + 1) != n {"),
    ('sv_padrao_ponto', "                if *nome.add(j) == b'.' && achar(nome.add(j + 1), b'.' as i32) == 0 {", "                if *nome.add(j) == b'.' && achar(nome.add(j + 1), b'.' as i32) != 0 {"),
    ('sv_padrao_fim', "        if *padrao.add(i) == 0 {\n            return 1;", "        if *padrao.add(i) == 0 {\n            return 0;"),
]
