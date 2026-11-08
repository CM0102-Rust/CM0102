# Mutacoes (cqb_10), todas na 0x68d1f0 (LLIST::remove, ~77 chamadores; roda todo dia: as faltas e ausencias
# 0x419xxx, as ofertas de contrato 0x4ccaf9, as vagas e os ultimatos dos tecnicos, os amistosos, as suspensoes
# guardadas). Item que nao sai ou lista com a cauda ou o cursor num no liberado muda o que o jogo processa nos dias
# seguintes (ou derruba o jogo, o que tambem acusa).
# - A busca comeca no segundo no: o item da cabeca (o primeiro da fila, o caso comum) nunca sai e volta a ser tratado.
# - Tirado o ultimo no, a cauda fica nele (liberado): o proximo acrescimo (0x68d0d0) se perde.
# - Tirado o no do cursor, o cursor fica nele (liberado): o proximo da iteracao (0x68d1b0) le memoria solta.
MUTACOES = [
    ('cqb10_busca_pelo_segundo', '    let mut no = cqb10_d(this + 2); // cqb10: a busca começa na cabeça',
     '    let mut no = { let c = cqb10_d(this + 2); if c != 0 { cqb10_d(c + 6) } else { 0 } }; // mutacao: a busca começa no segundo',
     'cqb_10 removed'),
    ('cqb10_cauda_parada', '        cqb10_por(this + 6, ant); // cqb10: a cauda volta ao anterior',
     '        // mutacao: a cauda volta ao anterior', 'cqb_10 tail'),
    ('cqb10_cursor_parado', '        cqb10_por(this + 0xa, cqb10_d(no + 6)); // cqb10: o cursor anda',
     '        // mutacao: o cursor anda', 'cqb_10 cursor'),
]
