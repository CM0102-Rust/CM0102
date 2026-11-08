# Mutacoes do 2o item das competicoes. A do amistoso do clube inverte a comparacao do clube (513 chamadas, marcacao e
# midia dos amistosos): a carreira ate 2002-06-01 deve acusar. A da lista da data troca a fatia (a seguinte, de 0 a 2):
# o set_up_the_day (Rust) monta o dia de jogos com as listas erradas: deve acusar.
MUTACOES = [
    ('cpc_amistoso_do_clube', 'let do_clube = u(a + 0xc) == clube; // o clube do amistoso', 'let do_clube = u(a + 0xc) != clube; // mutacao: outro clube'),
    ('cpc_lista_da_data', 'lista(*(0xae2a58 as *const usize), ano, dia, fatia, grupo) // o gerenciador de partidas', 'lista(*(0xae2a58 as *const usize), ano, dia, (fatia + 1) % 3, grupo) // mutacao: a fatia seguinte'),
]
