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

## 5. Simulação reproduzível (`simular.py`)

`engenharia-reversa/simular.py --ate AAAA-MM-DD [--semente N] [--repetir N] [--modo rust|original|comparar]` monta a carreira automática, abre o jogo fora da tela com `-seed N` e a DLL encerra o processo na primeira chamada do sorteio depois da data pedida, gravando a impressão digital (quantidade de sorteios + estado completo do gerador). Mesma semente ⇒ mesma impressão: é o gabarito para provar rotinas reescritas na carreira inteira.

Com semente fixa o jogo ainda variava entre execuções. Fontes acidentais, todas neutralizadas **só no modo simulação** (`CM_PARAR_EM`); o jogo normal não muda:

| Fonte | Sintoma | Neutralização (`simulacao.rs`, `alocador.rs`) |
|---|---|---|
| `time()` | `srand(time)` em manager_manager.cpp (0x696d4b) | instante fixo |
| `clock()` (31 chamadas, 13 rotinas) | esperas do dia de jogo (match_day.cpp 0x6b55c0) dependiam de quantas vezes menus e telas consultavam o relógio | um relógio por rotina, +1 ms por consulta dela |
| Heap do Windows (LFH, endereço aleatório) | desempates por ordem de ponteiro: escalação diferente em 04/04/2001 com o gerador idêntico | `Heap*` do exe (IAT) num alocador determinístico em região fixa (0x18000000, 1 GB) |
| Memória fora da heap | ponteiros para o banco de dados (`GlobalAlloc` em Database.cpp 0x529cdf) e para memória da DLL (superfícies) | `GlobalAlloc`/`GlobalFree` no mesmo alocador; objetos e pixels da camada de vídeo em região própria (0x10000000); `VirtualAlloc(NULL)` do exe em faixa fixa |
| Lixo na pilha | o jogo lê variáveis locais não inicializadas, herdando o que a rotina chamada antes deixou; original, Rust e sondas deixam rascunhos diferentes (carreira mudava com o modo e até com o rastro ligado) | toda rotina desviada roda numa pilha particular de endereço fixo (`desvios::isolar`); no modo original as mesmas rotinas são desviadas para a própria original (`PASSAGEM`) |
| Variáveis de ambiente | a CRT copia o ambiente para a heap na partida; tamanhos diferentes deslocavam tudo | `simular.py` passa um ambiente mínimo fixo; as `CM_*` vão sempre, com `0`/`1` do mesmo tamanho |
| Lixo do Windows na pilha | o código das funções do Windows (arquivos, fontes, mensagens) grava na pilha do jogo valores que mudam a cada execução (endereços da heap do processo, alças) e o jogo os lê como variáveis não inicializadas: com o motor ligado, metade das carreiras mudava em 26/12/2001 (treino), com o gerador idêntico até ali; sondas que deixavam o dia mais lento escondiam o problema | todas as entradas da IAT (139) passam pela pilha particular (`isolar_importacoes`), menos as de exceção e encerramento; o `PeekMessageA` do jogo esvazia a fila e responde sempre "nenhuma" |
| Arquivos `*.tmp` | o jogo reaproveita os arquivos de trabalho deixados pela execução anterior (encerrada à força) | `simular.py` apaga antes de cada execução (e devolve os que já estavam na pasta) |

Também fixos: memória total/livre informada ao jogo (0x7a16d0, 0x7a1710). Janela: nasce e fica fora da tela (`CreateWindowExA`/`MoveWindow`/`ShowWindow` na IAT), sem piscar a tela de quem usa o PC.

Diagnóstico de divergência: `CM_RASTRO=AAAA-MM-DD` grava em `jogo\rastro.log` cada sorteio do dia (pedido, endereço de quem chamou, estado do gerador), cada `qsort` (comparador, hash da entrada e da saída) e cada consulta ao relógio (o endereço de quem chamou vem do trecho de entrada da pilha particular); o diário do `ddraw.log` tem uma linha por dia (sorteios, gerador, hash da tabela de pessoas, relógios). Método: achar o primeiro dia diferente no diário, rastrear o dia, achar a primeira linha diferente e subir pelo código.

### Auditoria das importações (o que entra de fora no jogo)

O exe importa 144 funções. Tudo o que ele sabe do mundo entra por elas, pela memória (endereços, lixo na pilha) ou pelos arquivos. Classificação para a simulação:

| Entrada | Onde | Situação |
|---|---|---|
| Relógio: `GetLocalTime`, `GetSystemTime`, `GetSystemTimeAsFileTime` | só em `time()` e `clock()` da CRT | fixo |
| `GlobalMemoryStatus` | 0x7a16d0, 0x7a1710 | fixo |
| Espaço em disco: `GetDiskFreeSpaceA` (e `…ExA` por `GetProcAddress`) | os.cpp 0x7a1820, 136 chamadas (menus, dia, Setup) | fixo em 4096 MB (o máximo que a rotina devolve) |
| Unidades: `GetLogicalDriveStringsA`, `GetDriveTypeA` | as 33 checagens de CD (35 chamadas) | lista fixa `C:\` |
| Ambiente, `GetCommandLineA` | CRT na partida | fixos pelo `simular.py` |
| Memória: `Heap*`, `GlobalAlloc`, `VirtualAlloc` | CRT, Database.cpp | regiões fixas |
| Listagem de pastas: `FindFirstFileA` + datas de arquivo | utils.cpp 0x90bf90 (tela de arquivos, configuração, menus, Setup) | **não fixado**: o conteúdo das pastas do jogo entra na carreira; um save novo pode mudar a impressão. Comparar execuções feitas com a pasta no mesmo estado |
| Caminhos: `GetCurrentDirectoryA`, `GetFullPathNameA`, `GetModuleFileNameA` | CRT, Setup | constantes enquanto o jogo não mudar de pasta |
| Registro: `RegQueryValueExA` | main.cpp 0x68d370 (partida) | constante na máquina |
| Tela e fontes: `GetSystemMetrics`, `GetDeviceCaps`, `GetTextMetricsA` | game_config.cpp, display.cpp | constantes na máquina (mudar a resolução do Windows pode mudar a impressão) |
| Mensagens: `PeekMessageA`, `GetMessageA` | laço principal | sem efeito medido (relógio por rotina) |
| Rede (`WSOCK32`), impressão, `CreateProcessA`, `WinExec`, `MessageBoxA` | jogo em rede, imprimir, abrir programas, erros | fora do caminho da simulação |
| `GetVersionExA`, `GetSystemInfo`, `GetKeyboardLayout`, `GetACP`/`GetOEMCP` | partida, os.cpp | constantes na máquina |

### Ferramentas de diagnóstico

- **Cobertura:** `cobertura.py <módulos>` grava `jogo\cobertura.txt` (só inícios comprovados de rotina: fronteira de instrução na varredura linear do `.text` e destino de `call` real ou ponteiro nos dados; o mapa tem pedaços desalinhados e um contador no meio de uma instrução derruba o jogo). `simular.py --cobertura` conta as chamadas (`inc [contador]; jmp original`, não muda a carreira) e resume por arquivo; a lista completa sai no `ddraw.log`. Motor ligado, 2 meses: 210 de 229 funções do motor exercitadas.
- **Exceções:** na simulação, toda exceção vai para o `ddraw.log` (código, endereço, registradores, pilha), porque o Windows nem sempre registra a queda de um jogo oculto.
- **Impressão e ambiente:** a impressão digital depende do conjunto exato de variáveis de ambiente (a CRT as copia para a heap). Uma variável nova no `simular.py` muda as impressões; compare sempre execuções da mesma versão do `simular.py` e da DLL.
- **Motor ligado:** `--motor sim` também é determinístico e Rust = original; dois meses de temporada em ~40 s (600 milhões de sorteios).
- **Mapa da memória:** com `CM_RASTRO` ligado, o diário ganha o hash de toda a memória do jogo e `jogo\memoria.log` o hash de cada bloco (4 KB da `.data`, 64 KB da heap) por dia; no dia do rastro saem `dados.bin` e `heap.bin` inteiros. Comparando execuções boas separa-se o ruído (alças de fonte, estado da interface, base de tempo do `clock` original) do primeiro dado que diverge. Atenção: essas sondas deixam cada dia mais lento e já esconderam uma divergência dependente de tempo; a prova final é sempre sem sondas.
