# Endereços da comunidade (patches de texto, finanças e telas)

Pontos do `cm0102.exe` 3.9.68 que a comunidade já identificou com patches de bytes. Não fazem parte do motor de partida, mas indicam onde ficam regras e textos que talvez queiramos mudar depois da migração.

**Fonte:**
- melanzalko, com contribuições de davdav, no tópico "Miscellaneous CM0102 Patches & Fixes" do fórum champman0102.co.uk (viewtopic.php?t=10877, páginas 1 a 3, agosto a outubro de 2026).
- Obrigado aos dois.

**Convenção dos posts:**
- Os números dos posts são **deslocamentos no arquivo**. Aqui estão convertidos em VA (VA = arquivo + 0x400000 em `.text`, `.rdata` e `.data`).
- Cada byte "antigo" foi conferido contra o exe original em 16/10/2026.
- A rotina é a que contém o endereço em `docs/mapa_funcoes.csv` (o módulo, quando o mapa o tem).

## Regras de jogo

### Instalações de treino
- **VA 0x461532, 0x461552, 0x461572, 0x461592** — rotina 0x460590 (`club_screens.cpp`).
- Limiares da descrição das instalações de treino: 0x190 / 0x258 / 0x320 / 0x3e8, ou seja bom / adequado / precisa de algum / precisa de muito.
- O patch baixa para 285 / 435 / 585 / 950.
- Só muda a descrição, não o nível.

### Situação financeira na diretoria
- **VA 0x5986ee (rico, multiplicador 0x1770) e VA 0x598716 (seguro, multiplicador 0x2904)** — rotina 0x5986b0.
- São os multiplicadores pela reputação do clube.
- O patch troca por 0x9398 e 0x9538, para que nem todo clube fique "rico".

### Dividendos (na prática um imposto sobre a receita)
Ficam na rotina 0x59cfa0, que ainda não tem módulo no mapa.

| VA | O que é | Patch |
|---|---|---|
| 0x59db0c | `je` da decisão aleatória do presidente | `jmp` tira o sorteio |
| 0x59dbf7 | `add ecx, 0x7d0` | — |
| 0x59dca3 | `add ecx, 0x5dc` | — |
| 0x59dc66 / 0x59dcc9 | `cmp word [edi+0x80], 0xfa0` (reputação 4000) | — |
| 0x59dcbb | `shl eax, 3`, a escala do limiar de entrada | vira `imul eax, eax, n`, com n de 1 a 8 |
| 0x59dcc1 / 0x59dcc7 | as duas guardas de caixa | com NOP, todo clube paga |

- O pagamento soma em +0x94 e +0x134 do registro financeiro e subtrai do caixa (64 bits em +0 e +4).
- Depois chama 0x59fe10.
- A faixa de impostos: **VA 0x59714d e VA 0x597a92**, constantes 0x1e46 e 0x1c52 na rotina 0x5968d0. davdav sugere 3500 e 2500.

### Transferências e contratos
- **VA 0x8edb67** — rotina 0x8e9550 (`transfer_screens.cpp`): o teto de 25 (0x19) da porcentagem da próxima venda. davdav: 0x32.
- **VA 0x867abe** — rotina 0x867aa0 (`staff_contracts.cpp`): o bônus por jogo sem sofrer gol fica restrito a goleiros. O patch faz a rotina devolver 1 sempre.

### Excursões
- **VA 0x471ef1** — rotina 0x4719a0 (`club_screens.cpp`).
- É o limiar de reputação para excursões a qualquer continente: o byte 4 multiplica 1000.

## Telas e interface

| Área | VA | Rotina | O que é |
|---|---|---|---|
| Próxima partida | 0x45e8c3, 0x45e8ce, 0x45e920 | 0x45ccf0 (`club_screens.cpp`) | estado do gramado com 13 dias de antecedência |
| Tela do clube | 0x456415 | 0x455942 (`club_screens.cpp`) | botão IMPRIMIR |
| Tela da competição | 0x490219, 0x490220 | 0x48f8e0 (`comp_screens.cpp`) | botão IMPRIMIR |
| Tela do clube | 0x456f85 | 0x456900 (`club_screens.cpp`) | ponteiro do texto "Stats" na tela da partida |
| Jogos do clube | 0x483f55 (casa), 0x483f7f (fora), 0x483fa0 (neutro) | 0x483a90 (`club_screens.cpp`) | cores das letras H/A/N: `mov reg, [cor]` vira `mov reg, imediato` |
| **Tela de tática** | 0x899f00, 0x899efb, 0x899ef7 | 0x899e50 (`tactics_screens.cpp`) | R, G, B da camisa de goleiro 1 |
| **Tela de tática** | 0x89e4ad | 0x899e50 | o salto para o código que pinta a camisa de goleiro 2 |
| Menu do técnico | 0x75e8e5, 0x75ebd4 | 0x75db00 (`Menubar.cpp`) | opções "controlar todas as seleções" e "controlar reservas" |
| Pedidos à diretoria | 0x4254fa | 0x425390 | `push` do texto "Meet with the Board" |

A tela de tática interessa para o elemento gráfico novo que o Lucas cogitou: a 0x899e50 desenha as camisas, com a cor em RGB.

## Textos (na `.data`)

| VA | Texto |
|---|---|
| 0x98f9b0 | "Dividends" (finanças) |
| 0x9c9720 | "Shareholder dividend payout" |
| 0x9c96b8 | "The board have announced…" |
| 0x98fa94 | "Season Tickets" |
| 0x98ee70 | "Stats" |
| 0x98ff14 | "Non-Playing Staff" |
| 0xaa0dec | "Set Staff Transfer Status" |
| 0x98f8d0, 0x990d54, 0x98f834, 0x990dc4, 0x9908c3, 0x9908d3, 0x99091b | "Tour" e frases da excursão |
| 0x98a1a8 | instrução dos pedidos à diretoria |
| 0x9c740c | instrução dos pedidos à federação |
| 0xa0fd54, 0xa0fd7b, 0xa0fea6 | opções e notícias de investimento |

## Áreas usadas como "caverna" (depende do exe)
- **VA 0x58ff74 (arquivo 0x18ff74):** NOPs dentro da rotina 0x58f500 (`file_screens.cpp`). O patch "Stats" grava um texto ali.
- **Arquivo 0x5e8200 e 0x6dd254:** não batem com o exe original. O primeiro não é zero na `.data`; o segundo fica depois do fim do arquivo. Só valem num exe já expandido pelo CM0102Patcher. Não usar sem conferir.

## Eliminatórias da Copa (o travamento das bases que começam cedo)

Fonte: Danek1985, no tópico "I've been studying the match engine, happy to share" do mesmo fórum (viewtopic.php?t=10790, 12/07/2026).

- **O ano em que cada confederação sorteia as eliminatórias** é uma palavra (`cmp`/`mov` com o ano) dentro das rotinas de cada liga das eliminatórias:
  - Ásia: VA 0x911c4d, 0x911c88, 0x911ca4 e 0x911cb8, na rotina 0x911bf0 (`wc_asia_league.cpp`);
  - Europa: VA 0x9182dc, na rotina 0x918200 (`wc_europe_league.cpp`);
  - América do Sul: VA 0x92036e, na rotina 0x9202b0 (`wc_south_american_league.cpp`).
- **No exe 3.9.68 original** valem 2000 (0x7d0) nos três primeiros pontos da Ásia e 2004 (0x7d4) nos outros. O post corrige um exe com o início mudado para 1994, em que ficam em 1994 (0x7ca); a correção põe 1996 (0x7cc).
- **A causa, segundo ele:** os sorteios ficam escalonados (Ásia em 1995, Europa em 1996). No intervalo, o código das eliminatórias asiáticas consulta o grupo europeu ainda vazio e lê um ponteiro nulo: leitura de 0x3e em VA 0x91df0e, na rotina 0x91def0. Só vale para jogo novo.
- **Para nós:** é um caso real de referência cruzada entre competições, a lembrar quando portarmos as eliminatórias ou mudarmos o ano inicial.

## Textos em português (language.ldb e estatísticas)

Fonte: o mesmo tópico do Danek1985, página 2 (t=10790&start=25, 18/07/2026), com observações de tafo.

- **Pé preferido em português:** o `Data\language.ldb` (204 MB) guarda os rótulos traduzidos. Em 0x070B0198 do arquivo está "Esquerdo"; trocar por "Só Esque" corrige o "Só esquerdo" exibido errado. Segundo tafo, cada rótulo pode aparecer duas vezes no arquivo, a alguns KB de distância.
- **Coluna "Key" vazia nas estatísticas da partida em português:** VA 0xa161cc (arquivo 0x6161cc), na lista de abreviações "Won", "Tck", "Key", "Cmp" da `.data`. O patch troca "Key" por "Imp".
- **Para nós:** quando a interface for nossa, os rótulos e abreviações traduzidos vêm do `language.ldb`, e as abreviações em inglês servem de chave.
