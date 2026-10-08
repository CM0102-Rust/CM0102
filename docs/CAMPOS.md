*[English version](CAMPOS.en.md).*

# Campos dos registros do jogo usados pelas rotinas

Nomes dos campos da base de dados que as rotinas reescritas leem, cruzados com o índice da comunidade **"[DOCUMENTATION] Offset Index: Decoding OllyDbg"** (fórum champman0102.co.uk, tópico 2802, mantido por Xeno, com achados de John Locke, Tapani, Saturn e outros; atualizado em 16/02/2023): <https://champman0102.co.uk/viewtopic.php?t=2802>. Os nomes entre parênteses em inglês são os desse índice; a maioria bate com o cabeçalho original da SI (`database.h`, em `referencias/`, fora do git), que dá a mesma ordem de campos.

Deslocamentos em hexadecimal a partir do início de cada registro; o tamanho em bytes vem entre colchetes quando não é 1.

## Jogador (registro de jogador, `CM3_PLAYERS`)

No objeto do jogador na partida (bloco de 20 por lado no motor), o ponteiro em **+0x6d** aponta para este registro. Nas rotinas, a variável se chama `pessoa`.

| Desloc. | Campo | Rotinas |
|---|---|---|
| +0x05 [2] | habilidade atual (currentAbility) | `forca_do_time` |
| +0x1b | aceleração (acceleration) | `forca_do_time` (somada com a velocidade) |
| +0x1e | antecipação (anticipation) | `forca_do_time` |
| +0x21 | regularidade (consistency) | `sorte_do_dia` em `forca_do_time` (sorteio(regularidade) contra sorteio(20)), `escolher_por_papel_c`, `escolher_por_papel_e` |
| +0x26 | drible (dribbling) | `forca_do_time`, `pontuar_casa` |
| +0x2b | cabeceio (heading) | `forca_do_time` |
| +0x2e | impulsão (jumping) | `forca_do_time` |
| +0x2f | liderança (leadership) | `forca_do_time` |
| +0x30 | pé esquerdo (leftFoot) | `apontar`, `classificar_lance` |
| +0x31 | chute de longe (longShots) | `forca_do_time` |
| +0x32 | marcação (marking) | `forca_do_time` |
| +0x33 | movimentação sem bola (offTheBall) | `forca_do_time` |
| +0x35 | mano a mano (oneOnOnes) | `forca_do_time` |
| +0x36 | velocidade (pace) | `forca_do_time` |
| +0x37 | passe (passing) | `forca_do_time` |
| +0x3b | pé direito (rightFoot) | `apontar` (a direção ganha 2 × (pé direito − pé esquerdo)), `classificar_lance` |
| +0x3c | resistência (stamina) | `forca_do_time` (o desgaste usa 30 − resistência) |
| +0x3e | desarme (tackling) | `forca_do_time` |
| +0x3f | trabalho em equipe (teamWork) | `forca_do_time` |
| +0x40 | técnica (technique) | `forca_do_time` |
| +0x43 | criatividade (creativity; "Vision" no cabeçalho da SI) | `forca_do_time` |
| +0x44 | disposição (workrate) | `forca_do_time`, `escolher_por_papel_b` |

## Pessoa (registro de pessoa, `CM3_STAFF`)

No objeto do jogador na partida, o ponteiro em **+0x69** (a "ficha").

| Desloc. | Campo | Rotinas |
|---|---|---|
| +0x5d | temperamento (temperament) | `aplicar_fator` e `atualizar_jogador`: a condição (+0x1a2) não cai abaixo de 2 × (25 − temperamento) |
| +0x65 | preferências (ponteiro): +0x1c, +0x20, +0x24 pessoas de que gosta; +0x28, +0x2c, +0x30 de que não gosta | `gosta_de`, `nao_gosta_de` |

## Árbitro (registro de árbitro, `CM3_OFFICIALS`)

No objeto de arbitragem da partida (`match_official.cpp`), o ponteiro em **+0x04**.

| Desloc. | Campo | Rotinas |
|---|---|---|
| +0x24 | deixar o jogo correr (allowingFlow) | `cartao_pela_gravidade` |
| +0x25 | disciplina (discipline) | `cartao_pela_gravidade` |
| +0x28 | arbitragem (refereeing) | `decidir_falta`: o árbitro deixa passar a falta quando sorteio(arbitragem + 15) = 0 (quanto maior o atributo, mais raro o erro) |
| +0x29 | bandeira (runningTheLine) | `procurar_impedimento`: o impedimento passa sem marcação quando sorteio(bandeira + 25) = 0 |

## Clube (registro de clube, `CM3_CLUBS`)

| Desloc. | Campo | Rotinas |
|---|---|---|
| +0x53 | nação (nation, ponteiro) | `forca_do_time` (estilo da liga nacional, ver Nações) |
| +0x80 [2] | reputação (reputation) | `forca_do_time` (mandante × visitante e o limite de 2500) |

## Variáveis globais

| Endereço | O que é | Onde aparece |
|---|---|---|
| 0xAE234C..0xAE2394 | quantidades de cada tabela (continentes, nações, cidades, estádios, clubes, pessoas, jogadores, competições...) | — |
| 0xAE2378 | quantidade de competições de clubes (as 127 últimas são amistosos) | `e_amistoso` (0x5c6710), chamada por `forca_do_time` |
| 0xAE235C / 0xAE2350 | total de clubes (nacionais + 2 × nações) / nações | `e_selecao`, `e_segunda_selecao` |
| 0xAE2364 | quantidade de pessoas (os 16 últimos ids são os técnicos humanos) | `contrato_de`, `tecnico_9a01`, `tecnico_9a05` |
| 0xAE2D0C | tabela de índices de contrato por pessoa (79 bytes por pessoa) | `contrato_de` |
| 0xAE23A4..0xAE23EC | ponteiros para o início de cada tabela (clubes em 0xAE23B4, pessoas em 0xAE23BC, jogadores em 0xAE23C4...) | — |
| 0xAE2C90 / 0xAE2C92 [2] | dia do ano e ano atuais | a simulação para na data pedida lendo daqui (`simulacao.rs`) |
| 0xDE1F58 | ponteiro para o ranking de países da UEFA | vizinho do estado do sorteio (0xDE1F5C, 8 bytes), que a comparação confere |
| 0xDE4400 | *handle* do `HeapCreate` do jogo | substituído pelo alocador fixo da simulação (`alocador.rs`) |
| 0xDE4404 | 1 se o jogo reconheceu a versão do Windows | — |

## Partida (objeto da partida, `MATCH_DATA`)

No motor, o ponteiro em **+0x4792**. Campos pela estrutura que davdav publicou no tópico 10794 (os desconhecidos ficaram de fora), conferidos com o que `forca_do_time` lê:

| Desloc. | Campo | Rotinas |
|---|---|---|
| +0x00 | id da competição | — |
| +0x0C / +0x10 | ids do mandante / visitante | — |
| +0x14 | competição (ponteiro) | `forca_do_time` (nação da competição em +0x5d; 0x5c6710) |
| +0x1C / +0x20 | mandante / visitante (ponteiros de clube) | `forca_do_time` (nação e reputação; o lado 0 é o mandante) |

O mesmo objeto guarda as datas (ano/dia em palavras), as fases e o placar; segundo o tópico, também o estádio, e o motor o consulta bastante.

## Nações (constantes em 0x9CF200..)

Cada nação da base tem um ponteiro global fixo nesta área (0x9CF200 Afeganistão ... 0x9CF4A8 Suécia; continentes em 0x9CFA08..). Fonte: `Helpers/9cf_constants` do NickDLL, mantido por davdav (dm-carmo, <https://github.com/dm-carmo/NickDLL>; o nome vem de Nick+Co, de quem foi a ideia inicial, outra pessoa; sem licença declarada: só os fatos, nada de código copiado).

Em `forca_do_time`, numa competição nacional (a nação da competição, +0x5d, é a nação do mandante, +0x53), a palavra 0x1de da força é multiplicada pelo estilo da liga nacional:

| Nações | Fator |
|---|---|
| Alemanha (0x9CF314), Escócia (0x9CF45C), Bélgica (0x9CF244) | 1,15 |
| Suécia (0x9CF4A8) | 1,25 |
| Noruega (0x9CF408), Inglaterra (0x9CF2E4) | 1,05 |
| Holanda (0x9CF338), Dinamarca (0x9CF2C0), Portugal (0x9CF434), França (0x9CF300) | 1,10 |

## Competições (para a etapa de competições)

Ainda não usados por rotinas reescritas; registrados para a etapa 2 (competições, tabelas, calendário). Fonte: tópico "Modern Champions League format for CM 01/02" (fórum champman0102.co.uk, tópico 10794, tafo e davdav, julho de 2026), conferir cada um na nossa desmontagem antes de usar.

| Onde | O que é |
|---|---|
| fase +0xB1 | tabela de classificação: uma linha de 0x3B bytes por time (+0x00 clube, +0x04 posição, +0x11 jogos, +0x15/+0x17 gols pró/contra, +0x19 pontos, +0x37 destino, +0x39/+0x3A melhor e pior posição possíveis) |
| fase +0xA7 [2] | jogos necessários por time (calculado uma vez e gravado no jogo salvo) |
| fase +0xC5..+0xC8 | critérios de desempate |
| fase +0xC9 | data de fim da fase (a fase termina por data, comparada com 0xAE2C90/0xAE2C92) |
| fase +0xDD | fase concluída |
| 0x6827D0 | ordena a tabela da fase com os desempates dela |
| 0x51C410 / 0x522610 | montagem de rodada de copa / sorteio com cabeças de chave |
| 0x6844B0 | gera o plano de jogos de uma fase em rodízio: por time e rodada, um código com sinal (adversário 1..N; positivo em casa, negativo fora) |
| 0x683AD3 / 0x683E20 | número de rodadas derivado da quantidade de times (N − 1) |
| 0x9086E0 | partidas montadas à mão (MLS); 0x68B100 cria a fase, 0x85C260 e 0x5AA680 registram cada partida |
| 0xADADFC | competição por id (id × 4) |
