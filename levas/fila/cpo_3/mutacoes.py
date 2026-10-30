# Mutacoes. Os pontos do mes nao zerados na virada (so o modo Rust e a carreira: o comparador roda a original; o
# tecnico do mes passa a somar os meses e o premio muda). O tecnico do mes sem a regra das duas semanas e com o empate
# (e o zero) trocando o escolhido: comparando, a devolucao difere; na carreira, o premio do mes vai para outro tecnico.
# Se o premio do mes so mudar o que a carreira nao le, as tres PODEM ESCAPAR da carreira (comparando, as duas ultimas
# acusam).
MUTACOES = [
    ('cpo_pontos_do_mes', 'cpo3_por(e + 0xce, 0); // os pontos do mês',
     '// mutacao: os pontos do mes acumulam',
     'cpo_3 virada do mes'),
    ('cpo_tecnico_recente', 'if recente {',
     'if false { // mutacao: sem a regra das duas semanas',
     'cpo_3 tecnico recente'),
    ('cpo_melhor_do_mes', 'if pontos > melhor_pontos { // o melhor do mês',
     'if pontos >= melhor_pontos { // mutacao: o empate troca',
     'cpo_3 melhor do mes'),
]
