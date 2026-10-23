# Mutacoes do empacotar (a gravacao do .qst empacota os *.lng no save_database e os *.tmp no fim do
# save_game_data). Todas mudam o conteudo descomprimido ou a lista de itens, que a prova do save
# compara (mudanca so na codificacao do RLE passa como IGUAL quando ha campo variavel ignorado).
# O desempacotar so roda na carga (--sav); a limpeza da pasta temporaria nao muda o .qst (os .tmp
# sao regravados com os mesmos nomes): sem mutacao propria. Nenhuma faz o empacotar devolver 0
# (o save_game_data cairia no erro e a prova so acabaria no tempo).
MUTACOES = [
    ('sv_emp_bloco', "            gravar(z + 0x41705, 1, n, z);", "            gravar(z + 0x41705, 1, n - 1, z);"),
    ('sv_emp_resto', "        let mut n = ler(z + 0x41705, 1, 0x30d40, f);\n        while n > 0 {", "        let mut n = ler(z + 0x41705, 1, 0x30d40, f);\n        while n > 0x1000 {"),
    ('sv_emp_so_o_primeiro', "        if proximo(achado_p) != 0 {", "        if proximo(achado_p) != 0 || true {"),
]
