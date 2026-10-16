# 146ª leva — a execução do passe (0x6fad40)

**Rotina:** `executar_passe()` = 0x6fad40 (match_pl.cpp, 22 KB, a maior portada até aqui). Executa todo passe, cruzamento, lançamento, escanteio, lateral e falta:
- nota do passe pelo tipo e pela distância;
- desafiante, com falta (cartão, pênalti, evento 0x82) ou desarme;
- sem desafiante:
  - fôlego e passe impreciso (o alvo sorteado pelo fator de +0x4756);
  - custo em +0x35;
  - narração da aproximação e do tipo de passe (mais de 150 códigos);
  - a bola parada seguinte;
  - o traçado (0x6ccb60), que termina em chute (0x70c040/0x70a910) ou na bola em trânsito.

**Primeira leva lida a partir do pacote** (`engenharia-reversa/pacote.py`). Medição combinada com o Lucas:
- **Tamanho real.** O pacote, pelas fronteiras reais, mediu 22 KB. O `mapa_funcoes.csv` cortava a rotina em 0x6fe364 (e o `ROTINAS.md` dizia 13,8 KB). Sem isso, a leitura teria parado no meio.
- **Notas antigas.** Trouxe a do ponteiro de saída (`COMO_CONTINUAR`) e a porta de gravação que existia para a rotina.
- **Quem chama.** Mostrou que o traçado 0x6ccb60 só é chamado pela 0x6fad40, o que permitiu fazê-lo unidade da fita sem efeito em outro lugar.
- **Constantes.** Os valores já resolvidos (1,2/1,4/1,6/1,8 do fator do passe, 0,95…0,875 dos pênaltis, 2,0/3,0 das distâncias) pouparam idas ao `.rdata`.
- **O que não ajudou nesta leva:** a seção de campos (quem mais lê e escreve cada deslocamento). A rotina foi lida pelo assembly, e os campos ficaram claros no próprio código.

**Estimativa de ganho:** umas 15 consultas a menos na leitura e um erro grande evitado, o tamanho. O tempo da leva continua dominado pela leitura do assembly (~6.900 instruções) e pela cadeia de testes (~3 h).

**Como foi comparada:** pela fita, com comparação dentro da unidade, porque ela roda como unidade do passe 0x6fa250 e do lançamento 0x6fa490.
- Desafiante, falta e desarme ficaram em versões próprias. Diferem da 143ª na ordem das escritas, nos sorteios do pênalti e no evento 0x82, que aqui não depende do cartão.
- As narrações pelo tipo estão em tabelas: quatro grupos de sete códigos, escolhidos pela saída e por +0x1a7, e a direção do passe escolhe dentro do grupo.

**Resultado** (`cadeia.txt`):
- Comparando até 2001-08-20: 0 divergências, com 12 dos 17 ramos exercitados.
- Mutações (25): 24 acusadas pela carreira e 1 pela confirmação.
- Jogo real comparando até 2002-01-01: 0 divergências.
- Carreiras Rust ×2 e original até 2002-06-01: impressão 459815f5e0dca92b.

**Refazer:** sobre o commit da 145ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 146 2001-10-01`
