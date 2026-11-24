# Item de TELAS (cx07_11): sem mutacoes na cadeia; a prova e o Lucas jogando comparando. Para provar os comparadores
# lado a lado numa sessao dele:
# - o recorte sem prender a direita a largura - 1 (acusada pelo comparador de clip_rect_to_surface ao arrastar uma
#   janela de tela ate a borda);
# - a sobreposicao sem o corte em zero da altura (acusada quando dois retangulos nao se tocam na vertical).
MUTACOES = [
    ('cx07_11_recorte_sem_a_direita',
     '    let right = read_i32(SCREEN_WIDTH) - 1;',
     '    let right = read_i32(SCREEN_WIDTH); // mutation'),
    ('cx07_11_sobreposicao_sem_corte',
     '    let h = a3.min(b3).wrapping_sub(a1.max(b1)).max(0);',
     '    let h = a3.min(b3).wrapping_sub(a1.max(b1)); // mutation'),
]
