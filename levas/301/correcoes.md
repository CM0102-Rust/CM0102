# A 301ª leva: os três defeitos achados pela cadeia (16/11/2026)

A janela desta leva (ENG POL TUR GRE HRV RUS até 2002-08-01, `levas/301/LIGAS` e `levas/301/ATE`) passa a virada de
temporada, e foi ela que achou os dois primeiros; o terceiro apareceu no estreio da comparação paralela.

## 1. O bit das regras ligado e apagado em seguida (defeito nosso, no código) — corrigido

**Sintoma:** a corrida base da cadeia (sem mutação) morreu em 2002-170 (19/06/2002) com caixa de erro do jogo,
`Error | v3.9.68 gre_prm..cpp 483`. As mutações foram canceladas ("base sem resultado").

**O caminho:** a virada grega chama as regras (assertiva da linha 476) e depois a tabela `0x5ead60` (linha 483). A
tabela percorre os clubes e conta os que têm divisão igual ao id grego de primeira (`[0x9cf86c]`); se o total não é 14
(`[esi+0x3e]`, escrito por ela mesma), devolve 0 e a assertiva estoura.

**Descartados antes de achar:** o alocador da simulação (`grep "region exhausted"` no `ddraw.log` = 0: a arena de 1 GB
nunca esgotou) e o molde do construtor (`construct_division_league` confere passo a passo com `0x5ea410`). As listas
de campos de `gre_prm_rules`/`gre_second_rules` também conferem com `0x5eae90`/`0x5ebde0`, campo por campo.

**A causa:** `apply_league_rules_with_flags` (`rust/src/motor.rs`) ligava o bit e logo a lista o apagava:

```rust
write_u8(this.addr() + league::LEAGUE_FLAGS, (this.league_flags() as u8) | flags); // liga o 0x40
eng_prm_apply_league_rules(this, written)   // ...e a lista escreve (LEAGUE_FLAGS, 1, 0) em cima
```

O original faz `or byte ptr [esi+0xd9], 0x40` e **nunca** zera o campo; a entrada `(league::LEAGUE_FLAGS, 1, 0)` está na
lista só para o comparador saber que endereço conferir. Pior: o original liga o bit **antes** da chamada virtual que
monta o calendário (`comp_vtable::CREATE_SCHEDULE`, +0x3c), e o bit 0x40 de `+0xd9` é a ordem das rodadas — o
calendário grego saía montado com o bit errado.

**A correção:** o aplicador filtra o campo de bandeiras da lista de escrita. Os 5 chamadores são todos desta leva
(croatas e gregos), então nada publicado foi afetado.

**A prova:** a mesma carreira no **motor original** chegou a 2002-212 sem nenhuma caixa de erro (`grep -c "msgbox 2002"`
= 0), isto é, a virada grega funciona no jogo original com essas ligas e a falha era nossa. Com a correção, a corrida
base chegou a 2002-212 e as mutações rodaram: **14 de 14, 13 acusadas e 1 escapada** (explicada em `escapadas.py`).

## 2. O comparador dos inscritos croatas não restaura o gerador (defeito do comparador)

**Sintoma:** no jogo real, 2 divergências, as duas a mesma linha, nas duas únicas vezes em que a rotina roda (a criação
da carreira e a virada):

```
engine: DIVERGENCE cro_cup_collect_entrants(0x21967710) returned 1 x 1 memory true list false (48 entries)
```

Retorno igual, memória igual, a **lista** diferente.

**A causa:** `compare_entrants_with_seeds` (`rust/src/motor.rs`) passa como regiões de `side_by_side`
`[(this + cup::TEAM_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]`. O `side_by_side` restaura essas regiões antes de rodar
a cópia (`before.write_back()`), e o cabeçalho do molde promete "o gerador restaurado entre as duas" — mas
`G_MOUSE_LEFT_BUTTON_DOWN` é `0xb67a34`, outro global (os caminhos de caixa de erro escrevem nele, por exemplo
`0x5eade2`), **não** o estado do gerador. `utils_random` (0x90cfc0) trabalha com `[0xde1f60]` (o ponteiro na tabela
fixa de sorteios, de 0xaa2454 a `[0xde2798]`) e `[0xde1f5c]` (o deslocamento). O projeto já tem a região certa:

```rust
// rust/src/rng.rs:56
pub const STATE_REGIONS: [(usize, usize); 2] = [(0xde1f5c, 8), (0xad6c30, 4)];
```

Sem ela, o original consome os 32 sorteios das sementes da Segunda Divisão e a cópia consome os **32 seguintes**: as
sementes saem diferentes por construção, e a lista difere sempre. Daí as duas divergências serem as duas únicas
chamadas.

**É falso alarme:** as sementes não mudam a carreira. `Cup::draw_round` (0x51c410) indexa a tabela de inscritos só em
+0, o ponteiro do clube; nenhuma rotina portada lê +0x4; e o próprio original não escreve +0x4 para os 16 inscritos da
Primeira Divisão, que ficam com o lixo do `crt_malloc`. É o mesmo fato que fez a mutação `mwd8_semente_menor` escapar
(ver `escapadas.py`): o byte é escrito e nunca lido.

**A correção:** acrescentar `crate::rng::STATE_REGIONS` às regiões do molde, mantendo as duas que já estão (o
`0xb67a34` cobre o caminho da caixa de erro da rotina, linha 0x12b). Com o gerador restaurado as sementes passam a ser
as mesmas nas duas corridas, e a comparação delas volta a discriminar.

## 3. O recurso da comparação paralela (`COMPARAR_BANCADA`) no seu primeiro uso

O `COMPARAR_BANCADA=K` (escrito em 14/11, a pedido do Lucas: "tá impraticável assim") roda a comparação do jogo real
numa bancada ao lado, em paralelo com as mutações. Esta leva foi a primeira a usá-lo, e ele tinha três defeitos — dois
deles só visíveis em execução:

1. **Sincronizava o código mas não compilava.** O `bancada.py sincronizar` copia o `rust/src`, e o `simular.py` não
   copia a DLL para o `jogo_testes` (quem copia é o `mutacoes.py`). A comparação paralela teria rodado com a DLL antiga
   da bancada, dando um resultado que não é o do código da leva. Agora o bloco compila e instala a DLL antes de
   comparar. (A mesma armadilha pegou uma verificação curta minha, que rodou com a DLL de antes da correção e repetiu a
   divergência — 15 minutos perdidos.)
2. **`wait` num pid de outro shell.** O trabalho paralelo é lançado no shell externo, mas o `wait` estava dentro do
   subshell que escreve o `cadeia.txt`: `wait: pid 49797 is not a child of this shell`. A cadeia seguiu sem esperar e o
   `cat` do arquivo paralelo saiu vazio, deixando o bloco do jogo real sem resultado (e o `gen_docs.cadeia` reprovando a
   leva, corretamente). Agora o trabalho escreve num `.parcial` e só renomeia no fim, e o passo do jogo real espera o
   arquivo aparecer.
3. **A limpeza do parcial vinha depois do lançamento** (na primeira tentativa de correção): apagaria o arquivo que o
   trabalho acabou de abrir, e o `mv` falharia. Passou para antes.

O `cadeia.txt` desta leva guarda a linha do erro do `wait` com uma nota, e a saída da comparação da bancada 2 copiada
no lugar onde o `cat` a teria posto — a mesma DLL, a mesma janela. O registro mostra o que correu, não uma versão
limpa.

**Resultado final da leva:** 14 mutações (13 acusadas, 1 escapada provada), jogo real `2ce549afd0fa92fa` com 0
divergências nas três fitas, carreiras Rust ×2 iguais (`be15cf32d7c80775`), detector igual, original registrada
(`1016b83c9995b595`). `gen_docs.cadeia(301)` = ok.
