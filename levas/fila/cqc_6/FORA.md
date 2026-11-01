# cqc_6 fora da 266ª (01/11/2026)

Com a `liga_jogos_para_a_tv` (0x685640, `League::move_fixtures_for_tv`) em Rust, as duas carreiras Rust saem diferentes, já em 2001-036 na janela curta (12.135 sorteios a menos nesse dia). Na carreira até 2002-06-01, com a 0x76c120 corrigida, ficam dois estados: 96d1c4fee745c48a e f8972df1c83e582c.

- **A cópia Rust está certa:** as entradas e decisões dela (datas, vagas, jogos, notas) saem idênticas em todas as execuções.
- **A causa é sobra da pilha particular.** A rotina é chamada todo dia e quase sempre sai cedo. O que muda a cada execução é a heap do processo (0xeb0000 a 0x11d0000), para onde vão as alocações da própria DLL (Vec, String, format!): o alocador fixo do modo simulação só cobre as do jogo. A base do `ddraw.dll` não muda entre execuções do mesmo arquivo (estudo da opção (c), 01/11).
- **Prova:** encher a pilha abaixo dela com um valor fixo dá 10 de 10 iguais (só diagnóstico; a zeragem é proibida desde 31/10).
- **A rotina original que lê esse lixo não foi achada:** o ponto muda com a disposição da heap.

Saída aprovada pelo Lucas (01/11): a opção (c). As alocações da DLL vão para uma região fixa no modo simulação; no experimento, com isso, deram 6 de 6 carreiras iguais até 2001-03-01 e 3 de 3 até 2002-06-01. Quando ela entrar, o item volta à fila sem mudança.
