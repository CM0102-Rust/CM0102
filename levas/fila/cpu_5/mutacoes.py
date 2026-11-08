# Mutacoes (cpu_5), todas no adiamento pelo clima (0x923630, em todo jogo pelo dia dos jogos e pela preparacao da
# partida): sem o sorteio das financas nos jogos da noite (um sorteio a menos por jogo da noite), o sorteio das financas
# na fatia errada (os jogos fora da noite sorteiam e os da noite nao) e sem o sorteio da chuva forte (um a menos por jogo
# com chuva 5 de 1 a 13 graus em estadio pequeno). A contagem de sorteios muda e tudo o que sorteia depois muda. A da
# chuva pode escapar se nenhum jogo ate 2002-06-01 tiver chuva 5 nessa band (o contador mostra).
MUTACOES = [
    ('cpu5_sem_rolagem_das_financas', '            if cpu5_sortear(n) == 0 { // cpu5: a rolagem das finanças',
     '            if n < 0 { // mutacao: sem o sorteio',
     'cpu_5 rolagem das financas'),
    ('cpu5_fatia_errada', '    if cpu5_b(jogo + 0x3f) == 2 { // cpu5: a fatia da noite',
     '    if cpu5_b(jogo + 0x3f) != 2 { // mutacao: a fatia errada',
     'cpu_5 rolagem das financas'),
    ('cpu5_chuva_sem_sorteio', '            if cpu5_sortear(n) != 0 { // cpu5: o sorteio da chuva forte',
     '            if n != 0 { // mutacao: sem o sorteio',
     'cpu_5 chuva forte'),
]
