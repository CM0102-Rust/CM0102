# Pendências de teste (ramos que os testes automáticos não alcançam)

Ramos marcados na cópia Rust (`crate::comparacao::ramo("...")`, desde a 127ª leva) que **nunca rodaram** nas carreiras automáticas (até 2002-06-01 com a semente 12345 e, nas do pós-jogo, até 2003-06-01 com a 777). Cada um precisa de uma partida/carreira que passe por ele. Depois de uma sessão do Lucas, `python engenharia-reversa/pendencias.py` lê o `jogo/ddraw.log` e diz quais destes rodaram.

A sessão do Lucas no modo comparação confere todas as rotinas listadas: desde 14/10/2026 as do pós-jogo (0x6b74a0, 0x6b6cc0, 0x6b79a0) também têm comparação lado a lado (pela fita).

| Ramo | Rotina | Coberto na sessão comparando? | Observação |
|---|---|---|---|
| `dividida vantagem` | 0x70ec80 (127ª/126ª) | sim | |
| `dividida media de acertos` | 0x70ec80 | sim | |
| `dividida defesa` | 0x70ec80 | sim | |
| `disputa fora do campo` | 0x6c9cd0 (127ª) | sim | |
| `jogada goleiro` | 0x70dfd0 (129ª) | — | inalcançável: o goleiro sai no início da rotina |
| `eventos 0x82` | 0x6b74a0 (130ª) | sim | |
| `eventos 0x13 sem marca` | 0x6b74a0 | sim | |
| `eventos 0x13 noticia` | 0x6b74a0 | sim | |
| `jogadores marca 0x20` | 0x6b6cc0 (131ª) | sim | roda só na carreira longa (semente 777, 2+) |
| `clube 5896d0` | 0x6b79a0 (132ª) | sim | ações forçadas pegas pelas impressões (`leva132b.py`) |
| `clube 589bc0` | 0x6b79a0 | sim | idem |
| `clube 547d90` | 0x6b79a0 | sim | idem |
| `sem humano clube ligado` | 0x6b66a0 (133ª) | sim | e as condições sempre falsas nas carreiras automáticas: clube humano na partida, competições [0x9cf7bc]/[0x9cf6e4]/[0x9cf958], o 16º humano técnico de seleção |
| `interesse partida marcada` | 0x6b5040 (134ª) | sim | **coberto** na sessão do Lucas de 14/10 (8+, sem divergência); continuam pendentes os outros humanos (as 16 vagas, o vínculo +0x24) |
| `batedor do clube` | 0x897990 (138ª) | — | a carreira automática só passa pela célula vazia (16 milhões de vezes): os clubes da IA não têm batedor escolhido. Precisa de um clube humano com batedores escolhidos na tela de táticas (pênalti, falta, escanteio) e uma partida com bola parada |
| `batedor de fora do clube` | 0x897990 | — | batedor escolhido que não é do clube (seleção ou vínculo 0x775790): técnico de seleção com batedores escolhidos |
| `batedor saiu do clube` | 0x897990 | — | batedor escolhido que saiu do clube (vendido, emprestado) antes da partida |
| `batedor sem ficha` | 0x897990 | — | batedor escolhido sem a ficha +0x61 (aposentado?) |
| `batedor alem das pessoas` | 0x897990 | — | célula com id acima das pessoas (dado corrompido): provavelmente inalcançável |
| `instrucoes guardadas` | 0x896fe0 (138ª) | — | a carreira só apaga as instruções (32 mil vezes). Guardar e devolver dependem do humano da vez ([0xb77c7a]) com o flag ligado: provavelmente mais de um humano, ou o humano assumindo/deixando um clube |
| `instrucoes devolvidas` | 0x896fe0 | — | idem |
| `instrucoes liberadas` | 0x896fe0 | — | idem (sem o flag) |
| `numero da competicao` | 0x6b6570 (138ª) | — | nunca roda na carreira automática; chamada pela 0x6b55c0 (dia de jogos) e pela interface: dia de jogo com técnico humano |
| `simulacao rapida abandono` | 0x6beeb0 (140ª) | — | mais de 4 expulsos de um lado na simulação rápida (3 x 0 para o outro): nunca acontece na carreira automática até 2002-06-01 (expulsões 2048 vezes, segundo amarelo 1024). Talvez inalcançável na prática; precisaria de um lado com muitos cartões |
| `simulacao rapida contador reiniciado` | 0x6beeb0 | — | no fim da simulação rápida, o contador de ataques (+0xf5e2/+0xf618) abaixo de 10 reinicia (20 e sorteio(6) + 12): os ataques passam de 10 em toda partida da carreira |
| `lesao para 3 e sai` | 0x6d0980 (141ª) | — | lesão leve (gravidade até 50) com o fôlego tirado acima do limite (0x96d1a8) num jogador já marcado em +0x1c (lesão grave anterior na mesma partida): nunca acontece na carreira automática até 2002-06-01 |
| `chute a gol marca` | 0x70a910 (142ª) | — | as narrações de marca da carreira (primeiro gol, 50º, 100º...; 0x200a..0x2012, destaque 0x21b6) nunca saem na carreira automática até 2002-06-01: os gols das partidas detalhadas são poucos |
| `chute a gol empate tardio` | 0x70a910 | — | gol que empata depois dos 70 minutos numa partida detalhada (0x1fd6..0x1fd9): não acontece na carreira automática |
| `movimento sem vaga na grade` | 0x6f09f0 (143ª) | — | o jogador que anda sem vaga na casa (+0x28 negativo) entra na casa nova; na carreira automática todos já entram em campo com vaga |
| `movimento sem mais adiantado` | 0x6f09f0 | — | o lado 1 sem o mais adiantado registrado (+0xf59a vazio) usa a linha 9; na carreira automática o registro nunca está vazio quando alguém anda |
| `conducao goleiro sem marca` | 0x7109f0 (144ª) | — | quem guarda a posição 1 do adversário sem a marca de goleiro (tabela &1 = 0) dá +5000 à nota do chute; na carreira automática o guarda da posição 1 é sempre goleiro |
| `laco ficha sem motor` | 0x6b98c0 (145ª) | — | ficha em campo (+0x19 >= 0) sem motor (+0x19e = 0) é tirada (-5) no início do laço; na carreira automática não acontece |
| `laco cobrador da bola parada` | 0x6b98c0 | — | bola parada sem dono (ou dono expulso) escolhe o cobrador (0x6ceb90) e o vira para o gol; o reinício já deixa um dono na carreira automática |
| `chute bloqueado` | 0x702050 (147ª) | — | a interceptação no caminho do gol devolve 2 (alguém desvia): 0x1fd3/0x1fd4, o chute muda em sorteio(1000) - sorteio(2000) e, fora do alvo, sai para escanteio; na carreira automática até 2002-06-01 não ocorreu |
| `prejogo ceu com chuva` | 0x6b7f20 (148ª) | — | com a marca de chuva (+0x4d & 0x2000) e o céu fora de 0/2/4, o limite +0xf638 do laço vem de sorteio(25) + 15 ou sorteio(30) + 50; na carreira automática até 2002-06-01 o limite nunca pesou (a mutação que o fixa em 1 não muda nada) |
| `prejogo codigo da insatisfacao` | 0x6b7f20 | — | o evento de pré-jogo 0x13 (jogador insatisfeito com o contrato) entra na lista com o código; na carreira automática quem o lê não muda o resultado (a mutação para 0x1e não muda nada) |
| `escalacao ficha do tecnico` | 0x6db500 (149ª) | — | a tática copia três campos da ficha do técnico (+0xa, +8, +0xc) para +0, +2, +4; na carreira automática o segundo (+2) não pesa (a mutação que o troca não muda nada) |
| `dia passo a passo` | 0x6b5ce0 (151ª) | — | um passo do motor por chamada (sem `ate_o_fim`) não roda na carreira automática: o contador de ramo "andar um passo" fica em zero; lá o laço do dia joga cada partida até o fim Exercitado no jogo do Lucas (JOGAR.bat, 17/10/2026): o contador apareceu e o jogo seguiu sem erro. |
| `dia contador de terminadas` | 0x6b5ce0 (151ª) | — | o +0x25 do dia (partidas terminadas) só alimenta a porcentagem da barra; contar 2 em vez de 1 não muda a carreira |
| `dia rodada do humano` | 0x6b5f00 (151ª) | — | a rodada (+0x2d de 0x4a4850) e a troca dos lados no 0x6b79a0 só existem com técnico humano; o contador "fechar rodada do humano" fica em zero na carreira automática |
| `dia historia` | 0x6b5f00 (151ª) | — | o método 1 de [0xb74434] (a história) roda (contador "fechar historia" acima de 2048), mas a história não entra nas impressões da carreira (sorteios, partidas, banco): tirar a chamada não muda nada nelas |
| `dia linhas do humano` | 0x6b4360 (152ª) | — | no fechamento do dia, as táticas guardadas dos humanos (0x5fbe20, 0x5fbd00, 0x5b5960) só são refeitas com técnico humano; o contador "linhas do humano refeitas" fica em zero na carreira automática (a mutação que tira a chamada escapa; o contador de terminadas, +0x25, é a lacuna já anotada na 151ª) Exercitado no jogo do Lucas (JOGAR.bat, 17/10/2026): o contador apareceu e o jogo seguiu sem erro. |
| `dia primeira volta` | 0x6b3c10 (153ª) | — | a 1ª volta do dia (+0x33 = 1) só começa partidas cuja competição (+0x63 da partida) tem +0x10, que a montagem cria zerado; na carreira automática a 1ª volta não escolhe nada e tudo cai na 2ª, então começar na 2ª ou apontar outra competição não muda a carreira |
| `dia partidas ja marcadas` | 0x6b3c10 (153ª) | — | a montagem pula as partidas da lista já marcadas -2; na carreira automática nenhuma chega marcada assim (a mutação que as deixa entrar escapa, mesmo a forte) |
| `sons dos eventos` | 0x6d8c70 (153ª) | — | a espera dos sons dos eventos só roda com a tela da partida (0x6b55c0, jogo em rede): o contador "sons dos eventos" fica em zero na carreira automática Exercitado no jogo do Lucas (JOGAR.bat, 17/10/2026): o contador apareceu e o jogo seguiu sem erro. |
| `eventos gravados` | 0x6d98b0, 0x6d9c40 (154ª) | — | a gravação roda na carreira (contador acima de 2048), mas os arquivos "events1/2.tmp" não entram nas impressões, e a leitura de volta (rever os lances) só roda com a tela: o teste é no jogo, revendo os lances de uma partida assistida |
| `passo em rede` | 0x6b55c0 (154ª) | — | a partida do humano em rede (pelo relógio e pelo placar dos humanos) não roda na carreira automática; o contador "passo em rede" fica em zero Exercitado no jogo do Lucas (JOGAR.bat, 17/10/2026): o contador apareceu e o jogo seguiu sem erro. |

Levas anteriores à 127ª não tinham ramos marcados: as lacunas delas estão descritas no texto de cada leva em `ROTINAS.md` (procure "lacuna", "não roda").
