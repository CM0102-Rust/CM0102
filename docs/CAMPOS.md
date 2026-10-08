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

### A área 0x9CF*** inteira (tópico 2801)

O tópico "[DOCUMENTATION] The [9CF***] Thread" (fórum champman0102.co.uk, tópico 2801, John Locke e colaboradores, atualizado em 15/01/2022) lista toda a área de ponteiros fixos que o jogo preenche ao carregar a base. Faixas (conferir cada endereço na nossa desmontagem antes de usar):

| Faixa | O que é |
|---|---|
| 0x9CF178..0x9CF1FC | cores (34) |
| 0x9CF200..0x9CF544 | nações (Brasil 0x9CF260, Argentina 0x9CF218, Inglaterra 0x9CF2E4...) |
| 0x9CF548..0x9CFA04 | competições (ex.: Brasileiro série A 0x9CF9EC, Libertadores 0x9CF63C, amistoso 0x9CF668, Copa do Mundo 0x9CF79C) |
| 0x9CFA08..0x9CFA1C | continentes |
| 0x9CFA20..0x9D00A0 | prêmios |
| 0x9D00A4..0x9D0524 e 0xB63C9C..0xB63D00 | clubes citados no código (ex.: Flamengo 0x9D0358); fora da área: Falkirk 0xA8EB48 e quatro clubes irlandeses em 0x9E9FC0..0x9E9FF4 |

O mesmo tópico lista os ids decimais das competições (0 = Belgian First Division ... 432 = Belgian Fourth Division D); às vezes o executável usa o id em vez do ponteiro.

Uso já visto no motor, ainda não reescrito: 0x6c6a30 (match_eng.cpp) compara a nação ligada à ficha do dono da bola (+0x69, +0x1a) e escolhe um fator float: 1,0 para Inglaterra, Escócia, País de Gales, Irlanda do Norte, Irlanda e Noruega; 0,6 para Suécia, Dinamarca e Alemanha; 0,2 para as demais.

## Tática: casas, formação e preferências (tópico 2799)

Fonte: tópico "About AI Manager preferred formations & AI tactic selection" (fórum champman0102.co.uk, tópico 2799, Xeno, MadScientist e John Locke, 2021). Conferido com o que já reescrevemos.

**Tabelas A e B do motor** (`tabela_a` 0x6C38C0 e `tabela_b` 0x6C38F0, por posição tática 0..10 em motor + 0x8EBC/0x8EC4 + 4 × lado): a A diz a casa do jogador na tática, a B para onde aponta a seta dele (igual à A sem seta). Bits: setor 1 goleiro, 2 líbero, 4 defesa, 8 volante, 0x10 meio, 0x20 meia ofensivo, 0x40 ataque; lado 0x80 esquerda, 0x100 centro-esquerda, 0x200 centro, 0x400 centro-direita, 0x800 direita. Bate com o que já usamos (`marcador_direto` espelha setor e lado; `escolher_cobrador` testa 0x40; `marcado_designado` compara A com B).

**Máscara da formação** (formação do lado + 0x187B, 32 bits): um bit por casa do campo, 31 casas: 0 goleiro; 1..5 linha do líbero; 6..10 defesa; 11..15 volantes; 16..20 meio; 21..25 meias; 26..30 ataque (em cada linha, da direita para a esquerda, olhando do próprio gol). Ex.: 0x281B06C1 = 4-4-2. Em `marcado_designado`, o bit 0x10000000 (casa 28: centroavante pelo meio) da formação adversária é um dos testes. 0x5B59A0 (formation.cpp) monta a máscara pelas 11 casas (0x5B5A60, `bit_da_casa`) e troca algumas máscaras pela da formação preferida mais próxima (4-5-1, 3-4-3, 5-3-2, 3-5-2).

**Formações preferidas dos técnicos** (0x896184..0x8961BC, nove constantes na ordem da lista do editor): 5-3-2 com líbero 0x280E8A89, 5-3-2 0x280E8B81, 4-4-2 0x281B06C1 (também o padrão), 4-3-3 0x381506C1, 4-2-4 0x2A2A06C1, 3-5-2 0x281B2381, 4-1-2-1-2 0x288A26C1, 4-5-1 0x101F06C1, 3-4-3 0x381B0381. Os arquivos `.pct` das táticas do computador são carregados em 0x893DC0 (28 e mais 17, seis repetidos).

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

## Finanças e estrutura do clube (para etapas futuras)

Fonte: tópico "Miscellaneous CM0102 Patches & Fixes by melanzalko" (fórum champman0102.co.uk, tópico 10877, melanzalko e davdav, agosto de 2026). O tópico mistura deslocamento no arquivo e VA; abaixo, tudo em VA, conferido na nossa desmontagem do 3.9.68.

| VA | O que é (original) |
|---|---|
| 0x461531, 0x461551, 0x461571, 0x461591 | limites do texto das instalações de treino: 400, 600, 800, 1000 ("Good facilities" até 400, depois "Adequate", "Need some", "Need major investment") |
| 0x5986B0 | situação financeira para a diretoria: "Rich" se o saldo passa de (25 × rep + 50 000) × 100 (lea com 6000 e 2000 em 0x5986EB/0x5986F2); "Secure" com 10 500 no lugar de 6000 (0x598713) |
| 0x59714D, 0x597A8E | faixas do "Dividends" (na prática um imposto sobre a receita): 7750 (0x1E46) e 7250 (0x1C52), comparadas com a palavra +0xB |
| 0x8EDB67 | teto da porcentagem da próxima venda nas negociações: 25 (byte 0x19) |
| 0x98F9B0 | texto "  Dividends" da tela de finanças |

**Prêmios e cotas de TV das competições** (tópico "Offsets", fórum champman0102.co.uk, tópico 1540, 2020): os valores em libras estão como constantes imediatas no código de cada competição; o tópico lista os deslocamentos no arquivo (VA = deslocamento + 0x400000) de ligas, copas e torneios europeus. Conferidos: Brasileiro série A, TV 1 000 000 em 0x42A461; série B, TV 500 000 em 0x42CA51; Premier League, prêmio 250 000 em 0x574B1D e TV 12 000 000 em 0x574B40; Liga dos Campeões, campeão 4 000 000 em 0x584539.

Ainda não usados: o motor não toca em finanças. Ficam para a etapa da simulação fora da partida.
