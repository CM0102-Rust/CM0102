# Versão sem tela: o que o próprio jogo já oferece

Estudo de 05/10/2026 (`Setup.cpp`, função 0x827110 e o início do jogo em 0x81b890).

A SI deixou no executável uma linha de produção de jogos sem interface, usada para gerar os inícios rápidos (`qstart\*.qst`) e, ao que tudo indica, para testes internos. Ela funciona no 3.9.68 com a nossa instalação.

## 1. Arquivo de configuração automática

Formato de texto, uma chave por linha; os espaços da linha são removidos antes da leitura e **a ordem importa** (a linha é aplicada quando lida):

| Chave | Valores | Efeito |
|---|---|---|
| `LANGUAGE=` | `ENG_UK`, `ENG_US`, `FRA`, `DEU`, `ITA`, `ESP`, `DAN`, `NED`, `NOR`, `POR`, `SVE`, `JAP`, `KOR`, `POL`, `RUS`, `CHI_SIM`, `CHI_TRA`, `POR_BRA`, `ESP_MEX` | idioma do jogo (tabela em 0x9eb244) |
| `DATABASE=` | `NORMAL`, `MINIMUM`, `MAXIMUM` | tamanho da base |
| `MATCH_STATS=` | `NORMAL`, `SMALL`, `LARGE` | partidas guardadas |
| `FOREGROUND_LEAGUES=` | **códigos de 3 letras** das nações: `ENG`, `BRA`, `ARG`, `GER`... | ligas jogáveis |
| `BACKGROUND_LEAGUES=` | idem | ligas de fundo |
| `ENABLE_MATCH_ENGINE=` | `YES`/`NO` | com `NO` as partidas são resolvidas sem o motor de lances (muito mais rápido) |
| `ENABLE_COMPUTER_BIDS=` | `YES`/`NO` | ofertas da IA |
| `ON_HOLIDAY=` | `YES`/`NO` | treinador "de férias": o jogo avança sozinho |

As ligas são reconhecidas pelo código da nação (`+0x53` no registro de 0x122 bytes do `nation.dat`). O nome por extenso (`+4`) vem no idioma carregado ("Inglaterra" em português) e não serve. Para gerar, é preciso ao menos uma liga em primeiro plano.

## 2. Dois modos de uso

- **Gerar um início rápido:** `quick_start.cfg` na pasta do jogo com `AUTO_SETUP=<arquivo ou padrão>`. Para cada arquivo encontrado sem `.qst` correspondente em `qstart\`, o jogo cria a carreira e grava `qstart\<nome>.qst`, depois fecha. Testado: Inglaterra, base mínima, **~15 s**, sem nenhum clique. O `.qst` sai sem compressão (indicador 3, 67 MB; os originais têm indicador 4, ~20 MB).
- **Rodar sozinho:** `auto_setup.cfg` na pasta do jogo. O jogo cria a carreira com um treinador de testes da SI ("Andy Ryan") e, com `ON_HOLIDAY=YES`, avança o calendário sozinho. Testado com `ENABLE_MATCH_ENGINE=NO`: de 28/07/2001 a 08/09/2001 em ~90 s, com as rodadas sendo disputadas.

Os `.qst` têm o mesmo contêiner dos saves (`.sav`): indicador de compressão, versão, número de blocos e um diretório de entradas de 268 bytes (`continent.dat`, `officials.dat`...).

## 3. Cuidados

- **Nunca deixar `auto_setup.cfg` ou `quick_start.cfg` na pasta do jogo**: a abertura seguinte do Lucas começaria a carreira automática.
- **Copiar o `game.cfg` antes e restaurar depois.** Quando o jogo sai normalmente, grava as preferências com o que o arquivo de configuração mandou (idioma, estatísticas). Aconteceu no primeiro teste e foi corrigido à mão: idioma de volta a 9 (POR), estatísticas NORMAL e o byte seguinte de volta a 18.
- O jogo cria arquivos `*.tmp` de trabalho na pasta; se for fechado à força, eles ficam (são recriados no jogo seguinte).

## 4. Caminho para a versão sem tela

1. **Já disponível:** carreira automática + férias + motor de lances desligado = simulação em lote. Falta controlar quando parar e ler os resultados (pelos saves, cujo formato o `CM0102Patcher` já descreve, ou pela memória do processo).
2. **Com a nossa DLL:** esconder a janela e não desenhar nada (a camada de vídeo é nossa), fixar a semente (`-seed`) para simulações reproduzíveis, e definir pela linha de comando a data de parada e o save de saída.
3. **Com o motor em Rust:** à medida que as rotinas migram, a simulação deixa de depender do exe original.
