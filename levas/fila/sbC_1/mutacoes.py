# Item de TELAS (sbC_1): sem mutacoes na cadeia; a prova e o Lucas jogando comparando. Para provar os comparadores
# lado a lado numa sessao dele:
# - o tracejado com 3 pixels acesos em vez de 4 (acusada pelo comparador da linha nas molduras tracejadas);
# - a caixa escurecida pulando a ultima coluna (acusada ao abrir uma janela por cima de uma tela).
MUTACOES = [
    ('sbC_1_tracejado_curto',
     '            if solid || (k - first) % 6 < 4 {',
     '            if solid || (k - first) % 6 < 3 { // mutation'),
    ('sbC_1_escurece_sem_ultima_coluna',
     '        for x in rect[0]..=rect[2] {\n            let pixel = pixel_at(surface, pitch, x - rect[0], y - rect[1]);',
     '        for x in rect[0]..rect[2] { // mutation\n            let pixel = pixel_at(surface, pitch, x - rect[0], y - rect[1]);'),
]
