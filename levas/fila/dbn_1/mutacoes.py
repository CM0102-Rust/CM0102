# Mutacoes da criacao do jogo (comissao extra, corte dos clubes, poda do historico). A prova e o .qst da prova do save:
# a criacao roda uma vez antes da gravacao. As da comissao extra (dbn1_nascimento, dbn1_comissao_alvo) so aparecem se o
# pessoal gerado sobreviver aos cortes da 0x524d70 e for gravado (os treinadores das selecoes sem treinador sao os mais
# provaveis); podem escapar.
MUTACOES = [
    ('dbn1_nascimento', '    let ano = sorteio(0x1a) + 0x794; // 1940 + até 25', '    let ano = sorteio(0x1a) + 0x795; // mutacao: 1941 + ate 25'),
    ('dbn1_comissao_alvo', '            let alvo = ((rep(c) + 500) * k as i32 / 10000).clamp(1, k as i32); // a comissão pela reputação', '            let alvo = ((rep(c) + 500) * k as i32 / 10000).clamp(2, k as i32); // mutacao: no minimo 2'),
    ('dbn1_cortar_pessoas', '        for &d in &[0x39usize, 0x24] { // o clube e a seleção da pessoa ficam', '        for &d in &[0x24usize] { // mutacao: o clube da pessoa nao a segura'),
    ('dbn1_podar', '            if nac == 0 || *((nac + 0x11c) as *const u8) != 0 { // a linha fica', '            if nac == 0 || nac != 0 { // mutacao: fica tambem a linha de nacao nao escolhida'),
]
