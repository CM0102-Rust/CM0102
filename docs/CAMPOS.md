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
| 0xAE2C90 / 0xAE2C92 [2] | dia do ano e ano atuais | a simulação para na data pedida lendo daqui (`simulation.rs`) |
| 0xDE1F58 | ponteiro para o ranking de países da UEFA | vizinho do estado do sorteio (0xDE1F5C, 8 bytes), que a comparação confere |
| 0xDE4400 | *handle* do `HeapCreate` do jogo | substituído pelo alocador fixo da simulação (`allocator.rs`) |
| 0xDE4404 | 1 se o jogo reconheceu a versão do Windows | — |

## Partida (objeto da partida, `MATCH_DATA`)

No motor, o ponteiro em **+0x4792**. Campos pela estrutura que davdav publicou no tópico 10794 (os desconhecidos ficaram de fora), conferidos com o que `forca_do_time` lê:

| Desloc. | Campo | Rotinas |
|---|---|---|
| +0x00 | id da competição | — |
| +0x0C / +0x10 | ids do mandante / visitante | — |
| +0x14 | competição (ponteiro) | `forca_do_time` (nação da competição em +0x5d; 0x5c6710) |
| +0x1C / +0x20 | mandante / visitante (ponteiros de clube) | `forca_do_time` (nação e reputação; o lado 0 é o mandante) |
| +0x04 | estádio (id; ponteiro em +0x18) | — |
| +0x43 / +0x44 | gols do mandante / visitante no fim (0x6BDA86 copia de 0xF5BD/0xF5F3 do motor) | — |
| +0x30 / +0x32 | nome da rodada / prefixo (códigos da tabela de nomes de comp_util.cpp, 0x4B4850) | `placar_agregado` (algarismo das unidades de +0x30), `nova_reputacao` e `reputacao_do_jogador` (pesos maiores em quartas, semi e final e nos grupos A a H) |

O mesmo objeto guarda as datas (ano/dia em palavras), as fases e o placar; segundo o tópico, também o estádio, e o motor o consulta bastante.

**Códigos de rodada** (tópico 1540, página 3, explicação de Saturn sobre os blocos de rodada das copas; conferidos na tabela de nomes 0x4B4850). Cada rodada da copa tem um registro de 0x68 bytes (+0x07 nome, +0x09 número, +0x0B tipo de sorteio, +0x0D/+0x0F/+0x11 desempate do jogo 1/2/3, +0x18 times, +0x1A confrontos, +0x1C times novos, +0x20 jogos extras, +0x21 jogos (ida e volta), +0x22 dias entre eles, +0x5C/+0x60/+0x64 prêmios; as rodadas ficam em sequência a cada 0x68, como mostra a Copa da França em 0x5BA4EA..0x5BA6C9, página 16). O nome é base + variante: base 0x0A primeira fase, 0x14 segunda... 0x78 quartas, 0x82 semifinal, 0x8C terceiro lugar, 0x96 final, 0xA0 repescagem, 0xDC eliminatória; variante 0 jogo único, 1 jogo extra, 2 segundo jogo extra, 3 ida, 4 volta, 5 desempate. Prefixos: 0x41F..0x426 = grupos A a H (0x418..0x41E: grupos de repescagem, temporadas de outono e primavera, "Group Stage", seções norte e sul).

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
| fase +0xB1 | tabela de classificação: uma linha de 0x3B bytes por time (+0x00 clube, +0x04 posição, +0x11 jogos, +0x12/+0x13/+0x14 vitórias/empates/derrotas, +0x15/+0x17 gols pró/contra, +0x19 pontos; as mesmas colunas só em casa a partir de +0x1F e só fora a partir de +0x2D, somadas em 0x686458 depois de cada jogo; +0x37 destino, +0x39/+0x3A melhor e pior posição possíveis) |
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
| fase +0x52 | marcas da fase; o bit 2 liga a numeração fixa do elenco (testado em squad_manager.cpp, 0x859DF4); cada competição grava o seu valor ao ser montada (ex.: 2 no Brasileiro, 0x42A621). Fonte: tópico 1540, página 2 |

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

## Regras (para etapas futuras)

Fonte: tópico 1540, página 5 (Xeno, giovanisantana29 e outros). Conferidos no 3.9.68:

| VA | Regra |
|---|---|
| 0x7DD212 (e 0x7DF766, 0x7DB019) | suspensão começa 14 dias depois (`push 0xE`): Inglaterra, Irlanda e gestos obscenos |
| 0x8C31D2 | visto de trabalho no Reino Unido: 18 + sorteio dias (`add al, 0x12`, 18 a 25) |
| 0x8C7BE4.. | regra do visto de trabalho no Reino Unido (tópico 1540, página 10; conferida): concede com reputação mundial do jogador (+0x0D) acima de 8500, salário semanal do contrato (+0x0C) acima de 34 999, menos de 24 anos (pessoa +0x18) com potencial (+0x07) acima de 190, mais de 20 jogos pela seleção (pessoa +0x22) com a nação (pessoa +0x1A) de reputação (+0x8E) acima de 3000 e menos de 32 anos, ou mais de 70% dos jogos (0x8C7D9E); para quem não é jogador, reputação mundial acima de 160 (0x8C7A94) |
| 0x579C63 | limite de 3 estrangeiros de fora da UE na Inglaterra (`mov byte [edx+5], 3`) |
| 0x96D808 | constante 71,0 (float) comparada em match_pl.cpp (0x706D1B, 0x706DEE, rotina ainda não reescrita); segundo a comunidade, a porcentagem de chutes a gol que não entram (o patch de mais gols troca por 65); tópico 1540, páginas 21 e 22 |
| 0x708A31 | o gol entra no placar: soma 1 ao byte de gols do lado (motor + 0xF5BD + 54 × lado, o mesmo que `placar_agregado` lê); página 24. A comunidade fala também de um "freio" depois de 5 gols, ainda não localizado |
| 0x6BD98F | duração do primeiro tempo no relógio do motor (0x8ED4): 0x1EF = 495; o jogo acaba em 0x3DE = 990 (os dois limites que `momento_marcado` e `placar_agregado` testam); tópico 1540, página 25 |
| 0x6BA74C | na partida do jogador humano, o salto de volta ao laço lance a lance com tela e narração; trocando por salto incondicional a partida corre como as do computador (página 30). 0x6B9ECA: ajuste das porcentagens da tela nos 5 minutos finais. 0x71A3C0: atraso da narração por velocidade (tabela em +0xBA9A6) |
| 0x6B5B41.. | o dia de jogos procura a próxima partida na lista de [0xB67A50] + 0x19 (registros de 0x69 bytes; o objeto da partida começa em +4) |
| 0x4CF5F0 | registro (pessoa, clube) de 80 bytes, pelo índice em [0xAE2D0C] + 79 × id da pessoa, em [0xAE1050]; usado pelo motor (ex.: influência do capitão) |
| fase +0xA7 (copas) | lista dos times que entram, vagas de 6 bytes (+0 clube, +4, +5); a montagem da Copa da Irlanda do Norte (0x78AC90) põe primeiro 20 times da 1ª e 2ª divisões e depois 12 das inferiores (página 26) |
| 0x54CEDB | ano bissexto (divisível por 4, exceto séculos não divisíveis por 400) |
| 0x642A70.. | jogos reais da primeira temporada nas séries italianas (A 0x642A70, B 0x647230, C1 e C2 até 0x65C8E0) |
| 0x43F7F0 | moedas: por moeda, a cotação em libras (double em 0xAD9CB8), o multiplicador da lista de valores de transferência (0xAD9DA0) e o passo dos botões +/− (0xAD9CC0) |
| 0x5999A0 / 0x5A0590 | dá dinheiro a um clube (thiscall nas finanças do clube, argumento o valor); 0x5A0590 acha as finanças do clube (ex.: 1 800 000 por entrar na fase de grupos da Liga dos Campeões, 0x585803) |
| [0xDE1F58] | coeficientes da UEFA: por nação, ponteiro, coeficiente (float) e as vagas na Liga dos Campeões, Copa da UEFA e Intertoto (página 28) |
| 0x54AC4F | dia da semana a partir do ano (0xAE2C92), do dia do ano (0xAE2C90) e da marca de ano bissexto (0xAE2C94) |
| 0x8CFA1E | limite de jogadores de fora da UE numa partida (o salto que o aplica) |
| 0x8C34E0 | executa uma transferência: no registro recebido, +8 id do clube comprador, +0xC id do jogador; tabelas de clubes em 0xAE23B4 e de pessoas em 0xAE23BC (página 23) |
| 0x43F2C8 | estrangeiros no Brasileiro: 2 por jogo (`mov byte [edx+2], 2`; tópico 1540, página 18) |
| 0x533080 | sem nomes reais, os jogadores trocam de clube em pares sorteados (16 × a quantidade, fator em 0x969508); alguns ficam onde estavam (página 19) |
| fase +0x3E | quantidade de times da liga (ex.: 18 na liga galesa, 0x925B42; 14 na grega, 0x5EAD72, que reserva 59 bytes por time, a linha da tabela; páginas 19 e 20) |
| 0x53D900 | as nações que entram nas copas da Oceania (Nova Zelândia, Papua-Nova Guiné, Fiji, Ilhas Salomão, Vanuatu; Austrália e Nova Zelândia são incluídas à parte); página 20 |
| 0x868307 | contrato fica desprotegido depois de 2 anos (+1 abaixo de 28 anos) (`add ebx, 2`) |
| 0x76D4B0.. | convocação das seleções: até 23 em torneio oficial (seleção principal e sub-21), 26/22 no outro caso, 26 e mínimo 22 em amistosos e eliminatórias (tópico 1540, página 16) |
| 0x76CC1F.. | convocação: amistosos, seleção B e sub-21 seguem regras próprias; quem joga fora do continente da sua nação (+0x71) só é chamado se a reputação da liga do clube (+0x69) passa de uma fração (0x968E40) da reputação da competição (tópico 1540, página 29) |
| fase +0x3C | quantidade de rodadas da liga (lida em 0x683E0E) |
| 0x85EB91 | teto do salário num contrato (o tópico dá o deslocamento no arquivo, 0x45EB91): o tópico 1540, página 13, fala em reputação atual × 750; no código (`STAFF_CONTRACT::generate_initial_contract` 0x85E960, na criação do jogo), o salário da base (pessoa +0x4E) só é mantido se for maior que 0 e não passar de reputação atual × 15 (+0x0B do jogador, +0x0A do não jogador; 0x85EB84..0x85EB91). Fora disso, o jogo gera um salário novo (0x864550) [observado]. Os 750 do tópico (Nick+Co, 01/02/2022) são a mesma conta na escala que o editor mostra, de 0 a 200, que é a do exe dividida por 50. O exemplo do tópico, Ryan Giggs com reputação 180 e teto de 135.000, dá 9.000 × 15 = 135.000 no exe. O remendo do tópico para tirar o teto põe NOP nos 6 bytes do `jg` de 0x85EB91 [observado: o código e o tópico batem]. A mesma página mostra a troca da Irlanda do Norte pela Áustria: dezenas de pontos do código com o ponteiro da nação (0x9CF3E4) e o id de cada liga gravados à mão, exemplo de por que as competições hoje se mudam por patch |
| 0x60E100.. | os ponteiros 0x9CF*** das nações são preenchidos pelo nome ao carregar a base (ex.: "Holland" → 0x9CF338); tópico 1540, página 14 |
| 0x92EB46.. | Copa do Mundo de 2002 (`cmp word [edi+0x40], 0x7D2`) com os 32 participantes e grupos gravados no código, nação por nação |
| 0x7ABCDF | jogador gerado (regen): 1 chance em 4 de trocar de nacionalidade, sorteando entre as nações (quantidade em 0xAE2350, vetor em 0xAE23A8, registros de 290 bytes) e só aceitando a sorteada se ela tiver menos de 100 pessoas (+0x88 da nação); tópico 1540, página 12 |
| 0x5265F8 / 0x536A1D | jogador novo vira "promessa" com 1 chance em 100 (potencial −2, 0xFFFE em +0x07); o potencial −2 (e −1) é resolvido em 0x536A1D na criação do jogo |
| 0x53D7F0 / 0x8CEBB5 | o clube só contrata bascos? / a nação usada nessa restrição (0x9CF424, País Basco); tópico 1540, página 8 |
| 0x83A1D5 | listas de observação dos clubes do computador no início do jogo (pulando o salto, nenhuma até a virada de temporada) |
| 0x5E069A | Copa América pela paridade de (ano − 1996); o ano da próxima edição fica em +0x40 da competição |

## Outros pontos (tópico 1540, páginas 6 e 7)

Conferidos no 3.9.68 quando indicado:

| Onde | O que é |
|---|---|
| competição/fase +0x49, +0x4A | reservas relacionados e substituições permitidas (4 bits cada); um método da competição (por volta de 0x48F2FA, com versão própria por competição na vtable +0x7C) copia os dois para a palavra +0x4D do objeto recebido (provavelmente a partida). A Copa da França grava os seus em 0x5BA19B; a Série A italiana, 7 e 3 (0x6416D6); o tópico 1540, página 11, lista esses valores de cada competição. Conferido |
| regras de transferência (ex.: italy_rules.cpp, 0x6613F0) | janelas em registros de 6 bytes: tipo (0x0E), ?, dia da semana (0xFF = a própria data), dia, mês (0 = janeiro), ?; tópico 1540, página 11 |
| 0xA158B0 | texto "Only %d home players" de match_eng.cpp (linhas 612 e 652): a partida acusa time com jogadores de menos |
| 0x68A160, 0x68A1C0 | calendário das ligas. 0x68A160(fase, rodada, dia, mês, ano da temporada 0/1, dia da semana 0 = segunda, turno 0 manhã/1 tarde/2 noite, ?, local 0 = casa do mandante): a rodada cai no dia da semana mais próximo da data (calendário de 2001-02). 0x68A1C0(fase, rodada, nº do jogo na TV, dia da semana, turno, ...) marca os jogos transmitidos; a última chamada da rodada dá a quantidade deles. Ex.: Série B italiana em 0x646574, Premier League em 0x57379D; tópico 1540, página 17. Conferido |
| 0x585498..0x5857C6 | sorteio dos grupos da Liga dos Campeões: 32 "pares" de 9 bytes (clube, clube do mesmo país ou 0, metade sorteada 0/1/2); pote a pote, sorteia o par (`sorteio(8)`), a metade (A–D ou E–H, pelas vagas que restam) e o primeiro grupo livre; o par do mesmo país vai para a outra metade; no fim troca clubes entre grupos da mesma metade até não haver dois do mesmo país (+0x53) num grupo. Resultado: 32 ponteiros, 4 por grupo (página 27) |
| local da partida (último argumento de 0x68A160) | 0 casa do primeiro sorteado, 1 e 0x10 local desconhecido, 4 estádio nacional (ou o maior do país), 5 maior estádio, 8 neutro, 0x2A meio do caminho entre os dois (30 a 50 mil), 0x40..0x4000 o 1º ao 9º maior estádio, 0x8000 Stade Louis II (observações da comunidade; ex.: final da Copa da Alemanha em 0x5D92B9) |
| 0x5836C4, 0x632E30 | ano inicial (2001, `cmp ax, 0x7D1`) para os prêmios internacionais e europeus do ano anterior. Conferido |
| 0x5E4800 | monta uma cor da interface a partir de três componentes (azul, verde, vermelho empilhados antes da chamada); a interface tem dezenas dessas chamadas. Conferido |
| nação +0x11C | liga selecionada (`NationLeagueSelected` do `database.h`); 0xAE2D04 parece ser a quantidade de ligas selecionadas (não conferido) |

## Defeitos conhecidos do original (tópico 1540, página 9)

Reproduzidos de propósito enquanto a meta é a fidelidade; candidatos a correção na fase de modernização. Conferidos no 3.9.68:

| Onde | Defeito |
|---|---|
| 0x411D82 (liga australiana) | a competição não é zerada no fim da temporada (falta gravar −1 em +0x30 da fase): o mesmo campeão todo ano. E a divisão abertura/encerramento sai 36/5 jogos em vez de 24/10 (+0x3C da fase) |
| 0x797474, 0x40AEDE, 0x40CFCE | Liga dos Campeões da Oceania, da Ásia e da África (ou Recopa asiática) com disputa de terceiro lugar (3 rodadas finais), às vezes marcada depois da final, o que pode travar a competição |

## Motor das competições (tópico 1540, página 31; tópicos 10790 e 10877)

Mapa publicado por quem fez o patch das competições europeias modernas. Os pontos marcados foram conferidos no 3.9.68; o resto fica como guia para a etapa das competições.

| Onde | O que é |
|---|---|
| 0x6826D0 | linha N da tabela da fase: `[this+0xB1] + 0x3B·N` (thiscall, ret 4). Conferido |
| 0x6827D0 | ordena fisicamente a tabela da fase com os critérios de desempate dela (4 códigos em +0xC5..+0xC8); argumento 0 = a tabela da própria fase; a posição +0x04 da linha é gravada em 0x682AEE. Comparação em 0x682B30. Chamada no fim dos grupos em 0x684D14. Conferido. A ordem do vetor não é a ordem da tabela |
| fase +0x3C, +0x3E, +0xA7, +0xC9, +0xDD | turnos (quantas vezes os times se enfrentam), times, jogos necessários em cache = (times − 1) × turnos (0x689B27, gravado no save), data de fim (8 bytes), fase concluída (gravada em 0x689C06). Conferido |
| 0x689AD0, 0x684946 | as duas formas de terminar um grupo: por jogos (0x689AD0) e por data (tique diário que chama a vtable +0x94 = 0x684CD0, que ordena, marca o destino +0x37 e grava a classificação europeia em 0xAE2D14) |
| 0x6844B0 | gerador de turno e returno; 0x683AD3/0x683E20 multiplicam times por turnos |
| 0x5851D0 | vtable +0x28 da copa europeia: avança a fase atual (+0x30) quando as anteriores terminam; a fase 8 monta a 2ª fase de grupos (0x585AB0), a fase 12 o mata-mata (0x586330). Posições nulas no vetor de fases contam como concluídas |
| vtable 0x96A1D0 | copa europeia: +0x38 nome da rodada (0x586FA0), +0x3C fábrica de rodadas (0x583C50; índice FF = plano das eliminatórias, 0–7 grupos da 1ª fase, 8–B da 2ª, C em diante mata-mata), +0x44 datas |
| 0x583EDE, 0x58409D | rodadas por grupo (6) da 1ª e da 2ª fase de grupos. Conferido |
| 0x4B01D0 | código do evento → nome: 0x43D..0x444 (grupos) viram 0x449; 0x445..0x448 viram 0x44A ("Phase 2 Group Stage"). Conferido |
| 0x68A160 | registra a data de uma rodada; a 2ª fase de grupos chama seis vezes entre 0x584146 e 0x584257 |
| copa +0x14, +0x36, +0x38, +0xA7, +0x3E | livro de inscritos (6 bytes: clube, classe de entrada, marca), usados, capacidade, participantes (copiados em 0x584A4F, conferido) e quantos. Três pontos acrescentam inscritos no meio da temporada sem conferir a capacidade: 0x585A5A (conferido), 0x5862A4, 0x633E03 |
| 0x585290, 0x586F60 | monta a fase de grupos; ordena os candidatos por classe e reputação (ordem dos potes) |
| copa +0xAD | rodadas do mata-mata: confrontos de 0x35B bytes (+0x00/+0x02 times, +0x06/+0x08/+0x12 para onde vai o vencedor, +0x0A..+0x10 de onde vêm) |
| 0x51C410 | rodada diária de cada copa: espera as origens e então sorteia. Bit 4 de +0x0B = sorteio com potes (0x51D9F0); bit 0x200 de +0x0D = confrontos prontos; senão 0x51E220 (único chamador em 0x51C5F1); depois 0x51E870 e, com dois jogos (+0x21 = 2), o returno em 0x522610. Conferido |
| 0x586396 | sorteio do mata-mata da Liga dos Campeões com o gerador 0x90CFC0. Conferido |
| 0xDE1F58 | distribuição anual das vagas europeias: trava por ano em 0x9036B6 (conferido); linhas por país (0x903B50: coeficiente, cotas); candidatos (0x904040, zera a marca +0x1DB dos clubes); `euro.cfg` (0x9059E0); distribuidores 0x904210/0x904580/0x904E30; seletor 0x9050D0. A ordenação por coeficiente não tem desempate (0x905CB0, conferido) |
| [0xB74C26 + 300·i], com i em [0xB77C7A] | treinador humano; o time é dele quando `[time+0xCF]` aponta para esse objeto |
| 0x79D098 | gera os nomes dos árbitros chamando duas vezes a função do primeiro nome (0x539C60) em vez da do sobrenome (0x53A1B0): é o defeito dos sobrenomes trocados. Conferido |
| 0x911C4D, 0x911C88, 0x911CA4, 0x911CB8, 0x9182DC, 0x92036E | anos de início das eliminatórias da Copa (Ásia, Europa, América do Sul; 2000/2004 no original). Em bancos com início antecipado a Ásia sorteia antes da Europa e 0x91DF0E lê um grupo vazio (tópico 10790). Conferido que são as constantes de ano |
| 0xA161CC | texto "Key" da coluna das estatísticas da partida (a tradução portuguesa deixa vazio). Conferido |
| 0x59DBD0..0x59DD25 | dividendos do clube (tópico 10877, pág. 2): limiar por reputação (+0x80, cortes em 4000 em 0x59DC66/0x59DCC9), escala do limiar em 0x59DCBB (`shl eax, 3`), proteções de caixa em 0x59DCC1/0x59DCC7, decisão aleatória do presidente em 0x59DB0C. Conferidos 0x59DCBB e 0x59DB0C |
| 0x8EDB66 | teto do percentual da próxima venda (`cmp al, 0x19` = 25%). Conferido |

## Mensagens de erro e proteções (tópico 3473)

| Onde | O que é |
|---|---|
| "v3.9.68 arquivo.cpp N" | N é a linha do fonte, em decimal; no exe aparece como `push N` em hexadecimal logo antes da montagem com o formato "v%s %s.%s %d" (0x9870E8) no texto 0xAE24D0, mostrado pela caixa 0x5E8290; depois grava 0 em 0xB67A34. Ex.: index.cpp 5809 = `push 0x16B1` em 0x60D7C3. O detalhe ("Unable to find the %s index") fica em 0xDE1F64. Conferido |
| 0x8E7BEC | lê os dados de jogador (+0x61) de uma pessoa com clube sem testar nulo; banco com pessoa sem esses dados derruba o jogo aqui (proteção da comunidade). Conferido |

## Cores dos uniformes (tópico 2376)

| Onde | O que é |
|---|---|
| 0x540490 | duas cores colidem? (cdecl, 2 cores; 1 = colidem). Usa a cor primária de cada uma (0x540520; grupos do `database.h`: 6 azul, 7 roxo, 8 vermelho, 9 laranja): mesma primária, ou os pares 8–9, 8–7 e 6–7 (este último também quando a cor é a de id [0x9CF1DC]). Conferido o início e a leitura de 0x9CF1DC |

## Carga do banco: quais jogadores entram (tópico 2554)

| Onde | O que é |
|---|---|
| 0x5255FF, 0x525637 | corte de reputação do clube (+0x80 ≥ 7500, `0x1D4C`) para carregar o elenco |
| 0x52562E, 0x525887 | corte pelo nível da liga da nação (+0x85 ≥ 10 e ≥ 6) |
| 0x5269EF | salto que deixa de carregar clubes fora das ligas escolhidas |
| — | segundo a comunidade, sempre entram: PA negativo, jogador com convocação pela seleção, transferência futura; jogadores sem clube seguem esses cortes |

Os cinco pontos foram conferidos. Vale para a etapa da carga do banco: a versão reescrita precisa escolher exatamente os mesmos jogadores.

## Promoção e rebaixamento entre ligas (tópico 3189)

Na montagem de cada liga (ex.: Grécia, 1ª divisão em 0x5EAEF1..0x5EAF23 e 2ª em 0x5EBE3B..0x5EBE47): +0xC1 vagas de rebaixamento direto, +0xBE de acesso direto, +0x20 a liga para onde se cai e +0x1C a liga para onde se sobe (ids em 0x9CF870, 0x9CF86C). Mudar só um lado (zerar as vagas sem ajustar a liga vizinha) dá o erro gre_second.cpp 483 (`push 0x1E3` em 0x5EBF5F) ou trava: as ligas ligadas precisam ficar coerentes. Conferido.

## Jogo salvo e o limite de 2 GB (tópico 4153)

- O `.sav` é uma coleção de arquivos (Nick+Co). Durante o jogo eles ficam como `.tmp` na pasta do jogo; históricos antigos podem ser tirados sem efeito.
- **Conferido no binário:** os erros `comp_stats.cpp 6000` e `6007` saem da carga das estatísticas de competição (0x4aba76 e 0x4abaf8). O 6000 é falha do `malloc` de n × 118 bytes; o 6007 é `fread` (0x945bfe) que leu menos de n registros de 0x76 bytes. Os dois mostram a caixa de erro e saem com `exit(-1)`.
- Segundo o Nick+Co, os endereços de arquivo do exe são variáveis de 32 bits com sinal, e por isso o arquivo não passa de 2 GB. A flag Large Address Aware (4 GB) foi testada pela comunidade e não resolve a carga. Nossos exe não têm a flag (`Characteristics` 0x10f).
- **Truque da comunidade (l22677):** com o jogo carregado, tirar da pasta alguns `.tmp`, salvar sem compressão (o arquivo cai de >2 GB para ~0,5 GB) e devolvê-los antes de continuar. Não serve depois que o arquivo já passou de 2 GB.
- **Para a versão reescrita:** ao portar a gravação e a carga, usar deslocamentos de 64 bits. A poda opcional de históricos antigos entra como modernização, só depois da interface idêntica. Os registros se referem por índice, então a poda precisa renumerar.
