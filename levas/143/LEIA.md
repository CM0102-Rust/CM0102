# 143ª leva — o movimento do jogador (0x6f09f0)

**Rotina:** `mover_jogador()` = 0x6f09f0 (match_pl.cpp, 9 KB), o passo do jogador para uma casa da grade com uma ação (conduzir, passar, aproximar...). Sem a bola, a lesão na dividida e o giro; com a bola, o ataque, a narração da aproximação, o desafiante (falta com o evento 0x82 ou disputa limpa), a posse; depois a grade, os marcados, o desgaste e a bola.

**Como foi comparada:** pela fita. As chamadas com efeito fora do motor (narração 0x6d6f70, 0x714d40, domínio, bola parada, bola fora, o evento 0x68d0d0 e as da lesão e da falta) são unidades gravadas da original e reproduzidas na cópia. Como a 0x6f09f0 só roda como unidade de outras fitas, o comparador grava a unidade e, dentro dela, faz a própria comparação (fita aninhada, como na 142ª). O evento 0x82 é montado na pilha: a 0x68d0d0 confere o conteúdo, não o endereço.

**O que deu errado no caminho:**
- nome repetido (`mover` já existia): virou `mover_jogador`;
- máscara do 5º argumento da narração: só o byte baixo é confiável (0xff);
- a área da condição "perde na área" era a própria, não a atacada;
- a primeira cadeia estourou o tempo na carreira Rust: o despacho do modo Rust mandava 0x714d40 e 0x68d0d0 para a chamada padrão da dividida. Ganhou despacho próprio (`despachar_movimento`) — armadilha nova em `docs/COMO_CONTINUAR.md`.

**Resultado** (`cadeia.txt`): comparando até 2001-08-20, 0 divergências; mutações (22): 20 acusadas e 2 lacunas (`m_vaga`, o jogador sem vaga na grade; `m_adiantado`, o lado sem o mais adiantado registrado — os dois ramos não rodam na carreira automática; em `docs/PENDENCIAS.md`); jogo real comparando até 2002-01-01, 0 divergências; carreiras Rust ×2 e original até 2002-06-01 com a impressão de referência 459815f5e0dca92b.

**Refazer:** sobre o commit da 142ª, `python integrar.py` (usa `codigo.rs`), `cargo build --release`, depois `bash engenharia-reversa/cadeia.sh 143 2001-10-01`.
