# Mutacoes (com o ramo de cada uma). Os dois ajustes sao chamados centenas de vezes na carreira ate 2002-06-01 (367 e
# 339) e os chamadores passam alvos grandes (3000 para a goal_crowd_reaction, -2000 ou a conta da 0x741020 para a diretoria), quase
# sempre fora de +-500: a noticia sai e com ela o sorteio(4). Sem esse sorteio, a sequencia do gerador muda (so o modo
# Rust e a carreira acusam: os comparadores rodam so a original). A terceira muda a confianca da diretoria (+6 da
# linha do clube, lida pelo gerente dos tecnicos e pela proxima chamada); o comparador da diretoria tambem acusa,
# porque o plano usa a mesma variacao.
MUTACOES = [
    ('cpo_torcida_sem_sorteio',
     'let s = sorteio(4);\n    gravar(n, 3, s as u32);\n    gravar(n, 4, cpo5_d(clube) as u32);\n    let grau = if v < -750 { 5 }',
     'let s = 0; // mutacao: sem o sorteio\n    gravar(n, 3, s as u32);\n    gravar(n, 4, cpo5_d(clube) as u32);\n    let grau = if v < -750 { 5 }',
     'cpo_5 goal_crowd_reaction'),
    ('cpo_diretoria_sem_sorteio',
     'let s = sorteio(4);\n    gravar(n, 3, s as u32);\n    gravar(n, 4, cpo5_d(clube) as u32);\n    let grau = if v < -750 && (',
     'let s = 0; // mutacao: sem o sorteio\n    gravar(n, 3, s as u32);\n    gravar(n, 4, cpo5_d(clube) as u32);\n    let grau = if v < -750 && (',
     'cpo_5 diretoria noticia'),
    ('cpo_diretoria_variacao', 'let s = ((conf - 6000) / 20).wrapping_add(alvo as i32) as i16;',
     'let s = ((conf - 6000) / 10).wrapping_add(alvo as i32) as i16; // mutacao: /10',
     'cpo_5 diretoria'),
]
