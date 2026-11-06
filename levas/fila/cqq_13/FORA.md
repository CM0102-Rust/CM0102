# cqq_13 sem PRONTO (06/11/2026)

As três rotinas (os registros dos clubes na Copa da Itália, na Copa da Série C e no LDV Vans Trophy: 0x63ff00,
0x63e270, 0x56bee0) rodam na carreira de 8 ligas (78, 142 e 110 chamadas até 2002-06-01; os contadores das mutações
rodaram) e não dão divergência na comparação até 2001-10-01. Mas a mutação não muda o resultado da carreira:

- o peso da classificação nos grupos da Copa da Itália (0x28 → 0x14) escapou: a carreira Rust até 2002-06-01 saiu
  igual à base (impressão f29c85195c2a534f);
- as outras duas (o peso zero na Copa da Série C, o peso mínimo da eliminação no LDV) não chegaram a rodar até o fim
  (a rodada foi parada para não gastar a trava do laboratório); pelo mesmo mecanismo devem escapar.

Os registros de staff (0x86bdd0, 0x86c000, 0x86c1d0, 0x86a800, 0x86b0b0) vão para o histórico; dentro da janela de
17 meses nada que a carreira lê depende do peso [inferência]. As mutações acusadas da cqi_10 e da cpz_4 nessas rotinas
mexiam na marca da competição europeia do clube (+0x1db), que estas copas não gravam.

Saídas: entrar como passageiras num item com rotina de efeito, ou esperar uma mutação no histórico que a carreira leia
(por exemplo, a reputação do técnico na virada da temporada, depois de 2002-06-01).
