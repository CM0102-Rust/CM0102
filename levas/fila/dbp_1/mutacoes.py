# Mutacoes das preferencias no diario. A da injury_word_0c troca o 18 do peso pelo 14 (x = 14p - 14): muda o resultado de
# toda chamada pelo caminho da liga (clube abaixo da divisao), que decide se a pessoa quer sair para o clube; a
# carreira ate 2002-06-01 deve acusar. As das contagens trocam a lista (rejeitados <-> favoritos): so mudam o
# resultado quando a pessoa tem favoritos ou rejeitados no clube consultado; a carreira PODE NAO acusar (o comparando
# acusa na primeira contagem diferente).
MUTACOES = [
    ('dbp_reputacao', '    let mut x = v * read_f64_at(0x969558) - read_f64_at(0x969550); // o peso entre 0 e 10', '    let mut x = v * read_f64_at(0x969558) - read_f64_at(0x969558); // mutacao: 14 no lugar do 18'),
    ('dbp_contar_rejeitados', '    count_preferred_people_at_club(pessoa, clube, 0x28) // os rejeitados', '    count_preferred_people_at_club(pessoa, clube, 0x1c) // mutacao: os favoritos no lugar dos rejeitados'),
    ('dbp_contar_favoritos', '    count_preferred_people_at_club(pessoa, clube, 0x1c) // os favoritos', '    count_preferred_people_at_club(pessoa, clube, 0x28) // mutacao: os rejeitados no lugar dos favoritos'),
]
