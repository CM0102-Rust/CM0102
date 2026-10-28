# Mutacoes das noticias (o comparando roda a original: so o modo Rust e a carreira acusam; noticias quase nao entram
# na impressao: PODEM ESCAPAR). A da divisao entrega a todos; a da nacao so pelo clube; a de todos com o alcance 2.
MUTACOES = [
    ('cpj_noticia_divisao', 'c != 0 && cpj3_d(c + 0x57) == divisao // a divisão do clube',
     'c != 0 // mutacao: qualquer divisao'),
    ('cpj_noticia_selecao', '(c != 0 && cpj3_d(c + 0x53) == nacao) || (s != 0 && cpj3_d(s + 0x53) == nacao) // o clube ou a seleção',
     '(c != 0 && cpj3_d(c + 0x53) == nacao) && s == s // mutacao: so o clube'),
    ('cpj_noticia_alcance', 'cpj3_entregar(this, noticia, 4, |_| true) as u32',
     'cpj3_entregar(this, noticia, 2, |_| true) as u32 // mutacao: alcance 2'),
]
