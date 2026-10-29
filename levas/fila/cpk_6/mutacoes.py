# Mutacoes. O texto lido do dado seguinte (o comparando acusa no primeiro texto; a carreira nao escreve textos sem
# tela: PODE ESCAPAR). A noticia do apuro financeiro sem o sorteio (a carreira acusa pela contagem de sorteios no
# primeiro clube em apuro; o comparando roda a original). A cobertura dos olheiros com a regiao trocada (o comparando
# acusa pelo valor; a carreira, se a busca dos olheiros rodar sem tela).
MUTACOES = [
    ('cpk_texto_dado', 'let origem = (noticia + 5 + 4 * k as usize) as *const u8; // o dado',
     'let origem = (noticia + 9 + 4 * k as usize) as *const u8; // mutacao: o dado seguinte'),
    ('cpk_apuro_sorteio', 'let v = sorteio(2); // o sorteio da notícia',
     'let v = 0; let _ = sorteio; // mutacao: sem o sorteio'),
    ('cpk_olheiro_regiao', 'if *((cpk6_d(c + 0x53) + 0x76) as *const u8) != *((nacao + 0x76) as *const u8) { // fora da região',
     'if *((cpk6_d(c + 0x53) + 0x76) as *const u8) == *((nacao + 0x76) as *const u8) { // mutacao: dentro'),
]
