# Mutacoes (com o ramo de cada uma). Todas nas permissoes do comeco do jogo (as outras duas rotinas so rodam na tela).
# Com 3 anos no lugar de 5, os estrangeiros com 3 ou 4 anos na nacao ganham a segunda nacionalidade em vez da permissao
# (deixam de ser estrangeiros: limites da escalacao, permissoes e transferencias mudam). Sem gravar a segunda
# nacionalidade, quem tem 5 anos ou mais segue estrangeiro. Com o ano mais novo no lugar do mais antigo, mais
# jogadores ficam com menos de 5 anos e recebem a permissao em vez da nacionalidade. Os ramos dizem se a carreira
# passou por eles.
MUTACOES = [
    ('cps2_tres_anos', 'if (cps2_w(0xae2c92) as i32) - (ano as i32) < 5 { // cps2: menos de cinco anos na nação',
     'if (cps2_w(0xae2c92) as i32) - (ano as i32) < 3 { // mutacao: tres anos',
     'cps_2 permissao pela carreira'),
    ('cps2_sem_nacionalidade', 'core::ptr::write_unaligned((p + 0x1e) as *mut usize, cps2_d(cps2_d(p + 0x39) + 0x53)); // cps2: a segunda nacionalidade',
     '// mutacao: sem a segunda nacionalidade',
     'cps_2 segunda nacionalidade'),
    ('cps2_ano_mais_novo', 'if a < ano { // cps2: o ano mais antigo',
     'if a > ano { // mutacao: o mais novo',
     'cps_2 permissao pela carreira'),
]
