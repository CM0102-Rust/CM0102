# Mutacoes dos diversos. A do olheiro devolve a pessoa mesmo quando ela nao confere: o comparando acusa na primeira
# vaga mudada; a carreira PODE NAO ACUSAR (159 chamadas, olheiros quase nao decidem). A da sequencia pede menos de 9
# dias e a do rival pede 500 de reputacao: o comparando acusa; a carreira, se a midia usar (noticias e comentarios
# entram pouco na impressao): PODEM ESCAPAR da carreira. A do recordista grava a data em +8: o comparando acusa na
# primeira construcao.
MUTACOES = [
    ('cph_olheiro_confere', 'return 0; // a vaga mudou de pessoa',
     'return pessoa; // mutacao: sem conferir'),
    ('cph_sequencia_dias', '< 10 { // há menos de 10 dias',
     '< 9 { // mutacao: 9 dias'),
    ('cph_recordista_data', 'padrao(this + 0x10); // a data do recordista',
     'padrao(this + 0x8); // mutacao: em +8'),
    ('cph_rival_reputacao', 'if reputacao(clube) + 0x2ee >= reputacao(r) {',
     'if reputacao(clube) + 0x1f4 >= reputacao(r) { // mutacao: 500'),
]
