# Mutacoes (sx12b_4), pela prova do save com carga (mutacoes.py --prova-save N: o FeriasTesteSave carregado com e sem
# as rotinas, a impressao da carreira depois da carga). Os comparadores so rodam a original.
# - a contagem das noticias de recuperacao nao copiada (a fila lida fica sem uso): as voltas de lesao depois da carga
#   nao viram noticia;
# - a instrucao de equipe 0x20000 posta nas duas primeiras taticas de todo tecnico humano, nao so nas antigas: muda a
#   tatica do time do humano (de ferias, o computador escolhe a escalacao, a tatica pode continuar a dele).
MUTACOES = [
    ('sx12b_noticias_de_recuperacao_perdidas',
     '        write_u32(this + f::RECOVERY_NEWS_COUNT, news); // sx12b_4: the recovery news count',
     '        let _ = news; // mutation: the recovery news count left at 0',
     'sx12b_4 injuries loaded'),
    ('sx12b_instrucao_de_equipe_sempre',
     '            if read_i32(record + k * 0x1ecd + 0x584) < 0x8000 { // sx12b_4: +0x584, the team instruction word',
     '            if read_i32(record + k * 0x1ecd + 0x584) < 0x7fffffff { // mutation: every team instruction word',
     'sx12b_4 human managers loaded'),
]
