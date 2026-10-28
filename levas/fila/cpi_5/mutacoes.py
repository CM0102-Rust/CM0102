# Mutacoes das listas e vagas. As ofertas: tirar as de outra pessoa (o comparando roda a original: so o modo Rust e a
# carreira acusam; ofertas guardadas sao raras: PODE ESCAPAR). As suspensoes: sem conferir a nacao do +0x71 (o
# comparando acusa pelo plano; emprestimos com suspensao sao raros: PODE ESCAPAR da carreira). A vaga: o 15 procura 6
# (o comparando acusa pela pessoa escolhida; a carreira, se um 15 vagar).
MUTACOES = [
    ('cpi_ofertas_pessoa', 'if cpi5_d(e) as u32 == pessoa { // a oferta da pessoa',
     'if cpi5_d(e) as u32 != pessoa { // mutacao: as dos outros'),
    ('cpi_suspensao_nacao', 'm != 0 && cpi5_d(m) as i32 == x // a do +0x71 da nação',
     'm != 0 // mutacao: sem conferir'),
    ('cpi_vaga_15', '15 => Some((cargo & !0xff) | 8), // o 15 procura 8',
     '15 => Some((cargo & !0xff) | 6), // mutacao: procura 6'),
]
