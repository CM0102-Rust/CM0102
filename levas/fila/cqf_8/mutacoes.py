# Mutacoes (cqf_8), todas na 0x7c3e40 (o historico da carreira: 1087 chamadas na carreira automatica ate 2002-06-01,
# quase todas pelas permissoes de trabalho da criacao do jogo (0x8c6e60: o primeiro grupo de cada temporada da o clube,
# a nacao dele e o ano mais antigo na nacao; menos de cinco anos = permissao pela carreira, senao a segunda
# nacionalidade) e pelos gols na carreira e pelo clube (0x7c3d20, 0x7c3bb0)).
# - O grupo de cada temporada do arquivo sai sem clube (−1): a 0x8c6e60 nao acha mais clubes antigos da nacao.
# - O ano do grupo de cada temporada do arquivo dois anos antes: o ano mais antigo na nacao cai e a regra dos cinco
#   anos muda.
# - A temporada atual sai sem clube (−1).
MUTACOES = [
    ('cqf8_grupo_sem_clube', 'let clube_no = cqf8_d(no + 2) as u32; // cqf8: o clube do grupo do nó',
     'let clube_no = 0xffff_ffffu32; // mutacao: o clube do grupo do nó', 'cqf_8 history season'),
    ('cqf8_grupo_dois_anos_antes', 'let ano_no = cqf8_w(no); // cqf8: o ano do grupo do nó',
     'let ano_no = cqf8_w(no).wrapping_sub(2); // mutacao: o ano do grupo do nó', 'cqf_8 history season'),
    ('cqf8_atual_sem_clube', 'let id = if clube == 0 { 0xffff_ffff } else { cqf8_d(clube) as u32 }; // cqf8: o clube da temporada atual',
     'let id = if clube == 0 { 0xffff_ffff } else { 0xffff_ffffu32 }; // mutacao: o clube da temporada atual', 'cqf_8 current season'),
]
