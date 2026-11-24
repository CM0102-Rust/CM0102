# Item de TELAS (sbC_3): sem mutacoes na cadeia; a prova e o Lucas jogando comparando (com o cm0102.exe). Para provar
# o comparador do circulo numa sessao dele:
# - o circulo tracejado com 3 passos acesos em vez de 4 (acusada nos paineis de elipse tracejados da tela de jogo).
MUTACOES = [
    ('sbC_3_circulo_tracejado_curto',
     '    let visible = |steps: i32| filled || style & 2 != 0 || steps % 6 < 4;',
     '    let visible = |steps: i32| filled || style & 2 != 0 || steps % 6 < 3; // mutation'),
]
