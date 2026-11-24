# Mutacoes (sx16_6). As medidas nao mudam a carreira (so decidem avisos de falta de memoria e disco), entao sao
# acusadas pelo comparador na comparacao (a criacao do jogo da carreira chama a memoria livre e o disco varias vezes):
# - a memoria livre sem o arquivo de paginacao;
# - o disco com 4 GB livres ou mais vale 2048 (e nao 4096; as maquinas de teste tem mais de 4 GB livres).
MUTACOES = [
    ('sx16_6_memoria_sem_paginacao',
     '    m[3].wrapping_add(m[5]) >> 20',
     '    m[3] >> 20 // mutation: without the page file',
     'sx16_6 disk space'),
    ('sx16_6_disco_2048',
     '    if available >> 32 != 0 { 0x1000 } else { (available as u32) >> 20 }',
     '    if available >> 32 != 0 { 0x800 } else { (available as u32) >> 20 } // mutation',
     'sx16_6 disk space'),
]
