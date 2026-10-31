# Mutacoes. Na linha de uma nacao (0x669340) e na linha do clube (0x669360), que dao as datas da temporada da linha
# (inicio +0x15, fim) a quem pergunta. A linha seguinte da nacao muda as datas das aposentadorias 0x7a7310 (fim da
# temporada da nacao do jogador), da moral do jogo 0x4cb250 e do amistoso livre 0x5c6b70 (pre-temporada). A linha
# seguinte da nacao escolhida do clube muda as aposentadorias, o adiamento pelo tempo 0x923630 (o inverno de outra
# nacao) e os dias ate a temporada 0x59b4c0 (financas). Sem conferir a escolha, o clube de uma nacao-chave nao escolhida
# (os clubes estrangeiros das copas europeias, dos jogadores sondados) usa as datas da propria nacao e nao as do
# continente: muda as aposentadorias dos jogadores desses clubes (pode escapar se as datas da nacao forem iguais as do
# continente). Chamadas a cada jogador: sem contador.
MUTACOES = [
    ('cpu1_nacao_linha_seguinte', 'return k as u8; // cpu1: a linha da nacao dada',
     'return (k + 1) as u8; // mutacao: a linha seguinte'),
    ('cpu1_clube_linha_seguinte', 'return k as u8; // cpu1: a linha da nacao escolhida',
     'return (k + 1) as u8; // mutacao: a linha seguinte'),
    ('cpu1_sem_conferir_a_escolha', 'let nacao = cpu1_d(clube + 0x53); // cpu1: a nacao do continente',
     'let nacao = cpu1_d(clube + 0x53); if let Some(k) = (0..0x23usize).find(|&k| nacao != 0 && cpu1_d(0xb63d60 + 72 * k) == nacao) { return k as u8; } // mutacao: sem conferir a escolha'),
]
