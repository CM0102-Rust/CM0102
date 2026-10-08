# Como continuar o trabalho

Guia para quem pega o projeto (pessoa ou Claude Code). Montagem do jogo: `README.md`, seção "Montar do zero". Método e visão: `PROJETO.md`. Tudo o que se sabe do binário: `DESCOMPILACAO.md`. Cada rotina reescrita e as provas dela: `ROTINAS.md`. Estado atual e próximos passos: `wiki/index.md`.

## Pré-requisitos

- O jogo **CM 01/02 3.9.68** instalado em `jogo/` (não vem no repositório). O `patch_portatil.py` confere o MD5 do `cm0102.exe` e recusa outra versão.
- O `cm0102-1080.exe` gerado pelo `resolucao.py`: é ele que as ferramentas de teste abrem.
- Python 3 com `pefile`, `capstone`, `psutil` e `Pillow`. Rust com o alvo `i686-pc-windows-msvc` (já configurado em `rust/.cargo/config.toml`).

## O ciclo de uma leva

Uma leva tem de 1 a 10 rotinas do motor de partidas (`match_eng.cpp`, `match_pl.cpp`, `match_day.cpp`), reescritas em `rust/src/motor.rs`.

1. **Escolher.** `python candidatos.py` lista as rotinas que o jogo real executa (`docs/cobertura_motor.txt`) e que ainda não estão em Rust, ordenadas pelo que falta reescrever antes delas ("falta 0" = só chama rotinas já provadas). `python candidatos.py 0x6c51e0` mostra o tamanho real e as chamadas pendentes. O topo da lista tem pedaços desalinhados do mapa (endereços quebrados, poucos bytes): conferir com `desmonta.py`.
2. **Ler.** Desmontar a rotina inteira (`desmonta.py` ou capstone) e traduzir instrução por instrução: bytes com sinal, estouros de 8 e 16 bits, ponto flutuante x87 em precisão dupla com os arredondamentos para `float` nos mesmos `fstp`, e valores que ficam na pilha do x87 sem arredondar. As constantes são lidas do próprio `.rdata` (`f32_em`, `f64_em`).
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

- Estado do autoteste que vale para o caso inteiro (tabelas A/B, globais, blocos de jogadores): guardar e restaurar na própria seção. Valores inválidos podem abrir a caixa de erro de uma rotina original; com o jogo oculto, o teste para sem gastar processador (o `mutacoes.py` espera o limite). Não desviar getters de 4 bytes (0x714d30 travou o jogo).
- O que depende do banco carregado (tática do clube em 0xde1d08, pessoas reais) não roda no autoteste sintético, que acontece antes do banco. Para essas rotinas: `mutacoes.py --jogo AAAA-MM-DD` (mutações no jogo real em comparação). Mutar um caminho que o jogo real quase não usa (cobrador designado dos clubes do computador) não prova nada: mirar o caminho comum.
- Gerador do autoteste: um sorteio (`s.proximo()`) por valor. Rotações de um mesmo número (`rotate_left`) saem correlacionadas e podem nunca produzir alguns casos (mutações passam por isso, não por equivalência). E manter a grade consistente: mais de 11 jogadores numa casa corrompe as listas, e um comparador que lê ponteiros delas (0x6f46b0) cai dentro da DLL.
- O cargo não recompila um `.rs` restaurado com data antiga: `mutacoes.py` atualiza a data; à mão, conferir se apareceu "Compiling" e se a DLL foi copiada para `jogo/`.
- Em `asm!`, ponteiros que precisam sobreviver a uma chamada vão em registrador preservado (`edi`, `esi`), não em `eax`.
- O gerador do autoteste pode sobrescrever campos de que outras rotinas dependem (por exemplo, o ponteiro do motor em 0x19e do jogador). Uma queda no autoteste se localiza pelo mapa do linker: `cargo rustc --release --lib -- -C link-arg=/MAP:target/ddraw.map`.
- Rotinas que gravam eventos de narração (0x6d6f70) ou chamam 0x6cd330 usam gravação e reprodução: a original grava as chamadas, e a Rust confere contra a gravação sem repetir os efeitos (`porta_6d6f70`, `GRAVANDO_6CD330`).
- Um teste pesado por vez (um jogo aberto de cada vez). O `simular.py` usa `laboratorio.py`: trava `..\..\.laboratorio.lock` e vigia de memória. Na máquina do Lucas isso existe porque ela é compartilhada com outros projetos; fora dela, a trava só serializa os testes.
- Os testes abrem o jogo sempre em janela (`-windowed`, `CM_JANELA=960x540`) e copiam e restauram o `game.cfg`.
