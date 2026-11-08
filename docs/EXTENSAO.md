*[English version](EXTENSAO.en.md).*

# Ponto de extensão: uma DLL externa que lê o motor

Criado em 17/10/2026 para o jogo de botão do Lucas (um motor 2D em Rust que desenha as partidas), num trabalho à parte, na branch `jogo-de-botao`.

## O contrato (versão 1)

- A `ddraw.dll` (o núcleo, `rust/src/extension.rs`) carrega, se existir, a `botao.dll` da pasta do jogo, no primeiro instante de partida.
- A DLL exporta, em C:
  - `extern "C" fn cm_instante(motor: *const u8)`: chamada no início de cada instante do laço do motor (0x6b98c0, `laco_do_motor` em `motor.rs`);
  - `extern "C" fn cm_versao() -> u32` (opcional): a versão do contrato que ela espera; diferente de `extensao::VERSAO`, a DLL não é usada.
- Sem a DLL, nada muda: a carreira de referência é a mesma (provado com a carreira Rust até 2002-06-01).

## Regras

- **Só leitura.** A extensão não escreve no motor, não chama rotinas do jogo e não sorteia: a carreira tem de ser idêntica com ou sem ela.
- O que há no motor (posições dos jogadores, bola, relógio, placar, narração) está em `motor.rs` e em `ROTINAS.md`; a extensão lê os campos pelos deslocamentos documentados.
- Mudança no contrato (outra chamada, outro argumento) sobe `VERSAO` na `main` e no lado da extensão juntos.

## Como trabalhar sem atrapalhar a `main`

- Branch `jogo-de-botao`, numa pasta própria (worktree), nunca `git checkout` na pasta principal `D:\Sistemas\CM0102`, onde as levas rodam.
- A extensão fica num crate próprio (fora de `rust/src`), compilado como `botao.dll`.
- Uma pasta de jogo própria para testar (cópia de `jogo/`), porque a `jogo/ddraw.dll` é trocada a cada leva.
- Testes pesados respeitam a trava do laboratório (`engenharia-reversa/laboratorio.py`); a máquina é compartilhada.
