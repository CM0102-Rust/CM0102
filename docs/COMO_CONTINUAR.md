# Como continuar o trabalho

Guia para quem pega o projeto (pessoa ou Claude Code). Montagem do jogo: `README.md`, seção "Montar do zero". Método e visão: `PROJETO.md`. Tudo o que se sabe do binário: `DESCOMPILACAO.md`. Cada rotina reescrita e as provas dela: `ROTINAS.md`. Estado atual e próximos passos: `wiki/index.md`.

## Pré-requisitos

- O jogo **CM 01/02 3.9.68** instalado em `jogo/` (não vem no repositório). O `patch_portatil.py` confere o MD5 do `cm0102.exe` e recusa outra versão.
- O `cm0102-1080.exe` gerado pelo `resolucao.py`: é ele que as ferramentas de teste abrem.
- Python 3 com `pefile`, `capstone`, `psutil` e `Pillow`. Rust com o alvo `i686-pc-windows-msvc` (já configurado em `rust/.cargo/config.toml`).

## O ciclo de uma leva

Uma leva tem de 1 a 10 rotinas do motor de partidas (`match_eng.cpp`, `match_pl.cpp`, `match_day.cpp`), reescritas em `rust/src/motor.rs`.

1. **Escolher.** `python candidatos.py` lista as rotinas que o jogo real executa (`docs/cobertura_motor.txt`) e que ainda não estão em Rust, ordenadas pelo que falta reescrever antes delas ("falta 0" = só chama rotinas já provadas). `python candidatos.py 0x6c51e0` mostra o tamanho real e as chamadas pendentes. O topo da lista tem pedaços desalinhados do mapa (endereços quebrados, poucos bytes): conferir com `desmonta.py`.
2. **Ler.** Desmontar a rotina inteira (`desmonta.py` ou capstone) e traduzir instrução por instrução: bytes com sinal, estouros de 8 e 16 bits, ponto flutuante x87 em precisão dupla com os arredondamentos para `float` nos mesmos `fstp`, e valores que ficam na pilha do x87 sem arredondar. As constantes são lidas do próprio `.rdata` (`f32_em`, `f64_em`). Nas rotinas grandes (centenas de instruções), começar pelo pseudocódigo do Ghidra (`python descompilar.py 0x...`, uns 10 s por rotina): dá os laços, os desvios e as variáveis da pilha certos, inclusive as lidas com argumento ainda empilhado. Mas esconde o que entra no `__ftol` e os arredondamentos para `float`, e o texto em 16 bits sai como `CONCAT22`: as contas exatas saem da desmontagem.
3. **Escrever.** Em `motor.rs`:
   - a rotina, com comentário `/// { 0xENDERECO } (arquivo.cpp): o que faz`;
   - o `Prologo` (bytes iniciais copiados para o trampolim: instruções inteiras e sem salto relativo; `prologos.py` confere);
   - o comparador `comparar_*` (roda original e Rust a partir do mesmo estado e compara as regiões de memória que a rotina pode tocar, mais o estado do sorteio);
   - as entradas nos vetores `rust`, `comp` e `nomes` de `instalar` (mesma ordem do `PROLOGOS`; `N` = total);
   - o caso no `autoteste` (entradas sintéticas, inclusive limites e casos extremos, sem cair nos caminhos de erro do original, que abrem caixa de mensagem).
   - Chamadas internas a outras rotinas vão **pelo endereço do jogo** (`transmute(0x...)`), para que na comparação passem pelos desvios já provados.
4. **Provar** (os quatro critérios de `ROTINAS.md`):
   1. Autoteste com mutações: escreva a lista em `mutacoes/levaNN.py` (veja os exemplos) e rode `python mutacoes.py mutacoes\levaNN.py`. A base (200 mil casos, cerca de 3 minutos; `--base-completa` a cada 5 levas, cerca de 35 minutos) tem de dar 0 divergências e cada mutação tem de ser acusada (divergência ou exceção); as mutações rodam só o motor com 200 mil casos (`CM_AUTOTESTE=motor:200000`, cerca de 1 segundo cada). Uma mutação que passar com poucos casos: refazer só ela com `--casos 2000000` antes de concluir que é lacuna. A que passar ou é lacuna do teste (corrigir o gerador) ou é equivalente pelos valores possíveis (documentar por quê).
   2. Jogo real: `python simular.py --ate 2002-06-01 --motor sim --modo comparar --limite 900` (0 divergências no `jogo\ddraw.log`).
   3. Carreira: `python simular.py --ate 2002-06-01 --motor sim --modo rust --repetir 2`. A impressão digital tem de ser igual à do modo original **da mesma compilação** (`--modo original`; o conjunto de desvios muda o lixo de pilha nos dois modos, e o número de referência muda com ele). Nas levas 50 a 52, `c76ad0ff6ce3daa6` com 2.357.335.216 sorteios.
5. **Registrar.** Seção da leva em `ROTINAS.md` e no `ROTINAS.en.md`, tradução dos nomes novos em `glossario.py` (gera `GLOSSARIO.md` e `.en.md`), estado em `wiki/index.md`, uma linha em `wiki/log.md`, commit e push.

## Armadilhas já conhecidas

- `fst dword` seguido de comparação: guarda o float arredondado mas compara o double; reproduzir guardando `v as f32` e comparando `v`. `pow` do x87 (`_CIpow`): base em st1, expoente em st0 (o primeiro carregado é a base). Retorno de 64 bits (`edx:eax`): ver qual registrador o teste usa.
- Globais que vêm da base (ids de clubes, competições) valem 0 no autoteste, que roda antes da carga: dar valores distintos durante o teste, senão mutações nelas passam.
- Dado sintético que vira endereço de escrita na rotina original (índice com sinal, deslocamento): restaurar toda a faixa que ele alcança, ou usar só valores reais. Caixa de erro do jogo durante o autoteste aparece na tela do Lucas (o autoteste roda antes da interface) e trava o teste sem gastar processador. Não interromper o `mutacoes.py` no meio: o `motor.rs` fica mutado.
- A impressão do modo de comparação (`--modo comparar`) não é critério e muda entre levas: a original e a Rust rodam as duas, e uma Rust que chama rotinas com desvio passa pelos comparadores delas. O critério é zero divergências e a mesma impressão nas carreiras Rust e original. Variável de ambiente nova no `simular.py` muda a heap e a impressão de referência: reaproveitar uma CM_* com valor do mesmo tamanho (ex.: `CM_OCULTO=2` para o som).
- Não editar o `motor.rs` enquanto o `mutacoes.py` roda: no fim ele regrava o arquivo como estava no começo. Rotina de técnico humano: a carreira automática quase não passa por ela; as mutações precisam do autoteste sintético.
- Rotina que devolve os casos de erro à original: uma mutação no caminho que decide "é erro" pode sumir, porque a original acerta o resultado. No autoteste, deixar o caminho errado cair num caso válido diferente (ex.: outra pessoa válida na vaga que a mutação leria).
- Comparador de rotina que sorteia: passar `crate::aleatorio::REGIOES_DO_ESTADO` ao `lado_a_lado` e rodar a Rust dentro de `sem_contar`; senão a Rust sorteia a partir do estado que a original deixou. Variáveis de pilha lidas logo depois de um `call` cujo argumento ainda não foi retirado (`add esp, 4` depois): o deslocamento é 4 a menos. Mutação que soma a mesma constante a todos os candidatos não muda o escolhido (equivalente): trocar por uma que mude a ordem.
- Estado do autoteste que vale para o caso inteiro (tabelas A/B, globais, blocos de jogadores): guardar e restaurar na própria seção. Valores inválidos podem abrir a caixa de erro de uma rotina original; com o jogo oculto, o teste para sem gastar processador (o `mutacoes.py` espera o limite). Não desviar getters de 4 bytes (0x714d30 travou o jogo).
- O que depende do banco carregado (tática do clube em 0xde1d08, pessoas reais) não roda no autoteste sintético, que acontece antes do banco. Para essas rotinas: `mutacoes.py --jogo AAAA-MM-DD` (mutações no jogo real em comparação). Mutar um caminho que o jogo real quase não usa (cobrador designado dos clubes do computador) não prova nada: mirar o caminho comum.
- Gerador do autoteste: um sorteio (`s.proximo()`) por valor. Rotações de um mesmo número (`rotate_left`) saem correlacionadas e podem nunca produzir alguns casos (mutações passam por isso, não por equivalência). E manter a grade consistente: mais de 11 jogadores numa casa corrompe as listas, e um comparador que lê ponteiros delas (0x6f46b0) cai dentro da DLL.
- O cargo não recompila um `.rs` restaurado com data antiga: `mutacoes.py` atualiza a data; à mão, conferir se apareceu "Compiling" e se a DLL foi copiada para `jogo/`.
- Em `asm!`, ponteiros que precisam sobreviver a uma chamada vão em registrador preservado (`edi`, `esi`), não em `eax`.
- O gerador do autoteste pode sobrescrever campos de que outras rotinas dependem (por exemplo, o ponteiro do motor em 0x19e do jogador). Uma queda no autoteste se localiza pelo mapa do linker: `cargo rustc --release --lib -- -C link-arg=/MAP:target/ddraw.map`.
- Rotinas que gravam eventos de narração (0x6d6f70) ou chamam 0x6cd330 usam gravação e reprodução: a original grava as chamadas, e a Rust confere contra a gravação sem repetir os efeitos (`porta_6d6f70`, `GRAVANDO_6CD330`).
- Um teste pesado por vez (um jogo aberto de cada vez). O `simular.py` usa `laboratorio.py`: trava `..\..\.laboratorio.lock` e vigia de memória. Na máquina do Lucas isso existe porque ela é compartilhada com outros projetos; fora dela, a trava só serializa os testes.
- Os testes abrem o jogo sempre em janela (`-windowed`, `CM_JANELA=960x540`) e copiam e restauram o `game.cfg`.
- Antes de escolher uma rotina, procurar o endereço em todos os módulos (`grep 0x... rust/src/*.rs`): a 0x417c10 (cópia de data) já era desviada pelo `data.rs`, e o desvio repetido é recusado ("bytes ... sao [e9 ...]"), deixando a original da tabela em 0 (salto para 0 no autoteste).
- O auxiliar `byte()` do `motor.rs` devolve `i8`: para `shr` (deslocamento lógico) ou `movzx`/`xor eax,eax; mov al` (byte sem sinal), escrever `byte(p, o) as u8`. Na 79ª leva, `byte(r, 0x4f) >> 4` com sinal fez o autoteste divergir já na base.
