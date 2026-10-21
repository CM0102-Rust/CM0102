# Soltar o limite de 30 linhas das áreas (segunda etapa)

## O problema

O relatório da partida monta a lista (0x55fba0 → `Area::init` 0x402ac0) com `n = altura do painel / altura da fonte 2` linhas e 2 colunas, sem pesos de linha. Em 800x600, `n` ≤ 30. Em 1080, `n` passa de 30 e:

1. `Area::init` enche de 1 os pesos das linhas (+0xbcb, vetor de 30) com `n` bytes. A partir de 31 passa do vetor: cai em +0xbe9 (aberta) e, com 35 ou mais, em +0xbed (tem barra de rolagem), que vira "verdadeiro" por engano. Com mais de 38 sai da área (0xbf1 bytes) e entra na área seguinte (ou na tabela de itens, se for a última vaga).
2. `Area::layout` 0x403350 confere +0xbac > 30 e mostra a caixa **Area.cpp 507** (a 497 é a das colunas).

## A estrutura não cresce

As áreas ficam dentro do Display, 249 vagas de 0xbf1 bytes a partir de Display+4, e logo depois vem a tabela de itens (Display+0xba95e). Não dá para aumentar o objeto. Os vetores ficam onde estão (as primeiras 30 entradas continuam iguais às da original) e as cópias Rust ganham uma **tabela ao lado**, por endereço de área: pesos [u8; 255] e início/fim de colunas e linhas [i32; 255]. O u8 das contagens limita a 255.

## Quem usa os vetores (busca pelos deslocamentos em todo o .text)

- pesos +0xbad/+0xbcb: só `Area::init` 0x402ac0 e `Area::layout` 0x403350;
- grades +0x1c/+0x94/+0x10c/+0x184: só `Area::layout` (grava), `Area::draw` 0x402e70 e `Area::scroll_rows` 0x403ee0 (leem);
- contagens +0xbab/+0xbac (só a contagem, sem os vetores): `Area::set_scroll_row` 0x403320, `setup_scrollbar` 0x403600, `Guio::init` 0x5ee150 (soma uma linha às áreas que abrem por cima, sem limite), Display 0x560490, 0x560c30, 0x561210 e 0x561f10 (+0xbb0 a partir do Display). Com mais de 30 linhas essas continuam certas.

## Ordem

1. Esta leva: `layout`, `init` e `draw` em Rust, idênticos à original (com o limite e a caixa).
2. Leva seguinte: `scroll_rows` 0x403ee0 (o relatório rola).
3. Soltar: com as quatro em Rust, `init` e `layout` aceitam até 255 linhas usando a tabela ao lado (sem passar do vetor) e `draw`/`scroll_rows` leem dela. Com no máximo 30 linhas, tudo segue igual à original e os comparadores continuam valendo; acima de 30 só a cópia roda (a original daria a caixa), e a prova é o relatório aberto no jogo do Lucas em 1080.

## Riscos

- Acessos aos vetores por ponteiro calculado não aparecem na busca pelos deslocamentos. Antes de soltar, conferir com `CM_ESPIAO` ou com uma guarda de página nas áreas de 1080.
- A tabela ao lado precisa ser limpa quando a área é reiniciada (`init`) e quando o Display é zerado (0x55f7c0 `Display::reset`). Uma entrada velha desenharia itens no lugar errado.
- O Display remoto (rede, 0x569540) manda os pesos com `extra_len`. Não é o caso do relatório (ponteiro nulo), mas uma tela que mande mais de 30 pesos pela rede continua quebrando do outro lado.
- `init` difere entre os exes (0x31f/0x257 → 0x77f/0x437): a cópia lê os imediatos do próprio exe.
