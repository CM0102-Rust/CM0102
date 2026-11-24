# Item de TELAS (sbC_2): sem mutacoes na cadeia; a prova e o Lucas jogando comparando. Para provar o comparador do
# painel numa sessao dele (a linha e a caixa de texto so rodam a original na comparacao):
# - o chanfro sem a linha de baixo (acusada pelo comparador do painel em qualquer caixa de mensagem ou na barra de
#   progresso de baixo).
MUTACOES = [
    ('sbC_2_chanfro_sem_base',
     '        exe::graphics_draw_line(right, bottom, left + 1, bottom, 2, dark);',
     '        // mutation: no bottom line'),
]
