# Mutacoes (sx22_2).
# - O corpo da abandonada sai sempre com o texto generico ("will be replayed at a later date"): o comparador do
#   escritor acusa todo corpo com um dos bits 0x10, 4 ou 8.
# O destrutor do tempo so libera memoria no fim do jogo: sem mutacao (o comparador confere os dois ponteiros nulos).
MUTACOES = [
    ('sx22_2_corpo_generico',
     '    weather_news_body(s, out, Some(format)); // sx22_2: the body by the reason',
     '    let _ = format; weather_news_body(s, out, Some(0xad52b0)); // mutation: always the generic body',
     'sx22_2 abandoned body'),
]
