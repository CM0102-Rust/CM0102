# cqc_12 fora da 266ª (01/11/2026)

Com a `dia_das_selecoes` (0x76b100) em Rust, as duas carreiras Rust até 2002-06-01 saem diferentes (a partir de 2002-148). A causa não é a porta: é um defeito da original.

- A escolha do elenco (0x76c120, original) só preenche os códigos de posição para elencos de 22, 24 ou 26 jogadores.
- Em 29/05/2002 vêm os elencos de 23 da Copa, e o código da 23ª vaga fica com o que havia na pilha. Esse valor vai para a escala (0x6e2000).
- Com a 0x76b100 em Rust, a convocação (0x76bab0) roda mais fundo na pilha particular, e a sobra ali muda a cada execução: metades baixas de ponteiros da heap, como 0x48c8, 0x4078 e 0x4408.
- Prova: encher a pilha com um valor fixo antes da convocação deixa as carreiras iguais (só diagnóstico). Uma zeragem dessas é proibida desde 31/10.

Saídas, a decidir com o Lucas:
- **(b)** Correção do original na 0x76c120: dar códigos aos elencos de 23 e 25, portando a rotina.
- **(c)** Fazer as alocações da própria DLL não deixarem ponteiros da heap na pilha particular (`simulacao.rs`/`alocador.rs`).

Com uma das duas, o item volta à fila sem mudança.

## Resolução: saída (b), aprovada pelo Lucas (01/11/2026)

A 0x76c120 entrou no item como 4ª rotina (`escolher_elenco_da_selecao`), fiel à original e com a correção: nos elencos de 23 e 25 as vagas a partir da 22ª recebem, em ordem, 0x202, 0x88 e 0x808, as vagas que a tabela de 24 tem a mais que a de 22. As 22 primeiras continuam as da original (ver `cqc12_codigos`).

- O comparador roda só a original, com a fita aberta: a escala (0x6e2000) entra como unidade com a lista e os códigos. A cópia Rust monta a lista e os códigos no mesmo estado e confere. Nos elencos de 23 e 25, as vagas da correção ficam fora (ramo).
- Prova sobre 89ce07b + 266ª + o item (4 rotinas): duas carreiras Rust até 2002-06-01 iguais (`f8972df1c83e582c`, também em mais 3 execuções avulsas). A correção do elenco de 23 roda em 2001-170..172, 2002-026 e 2002-148 (29/05/2002).
- Jogo comparando até 2002-01-01: 0 divergências da cqc_12. Ficam 4 divergências de outras rotinas: `veto_dos_fisios_e_da_diretoria` ×3 (2001-198..201, um sorteio a mais ou a menos com a mesma devolução) e `amistoso_enviar_pedido` ×1 (2001-198). A base sem o item tem 0 até 2002-01-01; com o item o modo comparando segue outra trajetória e chega a casos que essas duas comparações não cobriam.