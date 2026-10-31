# Mutacoes (cpt_7). Duas no clima inicial (0x922810, na criacao do jogo pela carreira automatica, 7 sorteios por
# cidade, depois da semente): um sorteio so na medida (dois sorteios a menos por cidade) e o vento sem o sorteio (um a
# menos por cidade); a contagem de sorteios muda desde a criacao e tudo o que sorteia depois muda. Uma na nota do estadio
# neutro (0x85da00, pela busca do estadio neutro dos jogos da selecao inglesa): sem o sorteio(100) final, um sorteio a
# menos por candidato (pode escapar se nenhuma busca neutra passar pela nota antes de 2002-06-01).
MUTACOES = [
    ('cpt7_um_sorteio_na_medida', '            let b = cpt7_sortear(100); // cpt7: o segundo sorteio da medida',
     '            let b = a; // mutacao: um sorteio so',
     'cpt_7 clima inicial'),
    ('cpt7_vento_sem_sorteio', '        let v = cpt7_sortear(3); // cpt7: o vento inicial',
     '        let v = 1; // mutacao: sem o sorteio',
     'cpt_7 clima inicial'),
    ('cpt7_nota_sem_sorteio', '    let r = cpt7_sortear(100); // cpt7: o sorteio da nota',
     '    let r = 50; // mutacao: sem o sorteio',
     'cpt_7 nota do estadio'),
]
