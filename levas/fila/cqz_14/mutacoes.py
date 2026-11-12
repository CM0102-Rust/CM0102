# Mutacoes (cqz_14): o texto do campeao com o modelo do vice (o texto nao volta para a simulacao: quem pega e a comparacao
# lado a lado, compare_text_writer); o texto das quartas so para o destino 7 (some o texto da classificacao).
MUTACOES = [
    ('cqz14_campeao_vira_vice',
     '1 => 0x98732c, // cqz14: "win"',
     '1 => 0x9872c8, // mutation: the runners-up text',
     'cqz_14 copa america knockout fate'),
    ('cqz14_sem_quartas',
     'if fate as u8 != 6 {',
     'if fate as u8 != 7 { // mutation',
     'cqz_14 copa america quarter finals'),
]
