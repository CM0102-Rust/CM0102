# Mutacoes pela prova do save (o .qst gravado pelo inicio rapido, PROVA.cfg). O id do conteiner
# (0x17 -> 0x16, cabecalho) e a contagem das ligas de frente (gravada no general.dat) mudam o
# arquivo. O ramo sem qst (ferias do tecnico) so roda na carreira automatica: sem mutacao aqui.
MUTACOES = [
    ('sv3_criar_id', "        save_game_data(arquivo.as_ptr(), 0x17);", "        save_game_data(arquivo.as_ptr(), 0x16);"),
    ('sv3_criar_frente', "                        *m = marca;\n                        *(0xae2d00 as *mut i32) += 1;\n                        *(0xae2d04 as *mut i32) += 1;",
     "                        *m = marca;\n                        *(0xae2d00 as *mut i32) += 1;\n                        *(0xae2d04 as *mut i32) += 2;"),
]
