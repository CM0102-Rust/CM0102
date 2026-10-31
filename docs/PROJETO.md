# CM 01/02 em Rust: o que é o projeto, onde estamos e aonde vamos

*Documento de visão. O estado do dia a dia está em `wiki/index.md`; o que se sabe do binário, em `DESCOMPILACAO.md`; cada rotina reescrita, em `ROTINAS.md`.*

## 1. O que estamos fazendo

**Reescrevendo o Championship Manager 01/02 (versão 3.9.68) em Rust, por dentro do próprio jogo, sem que ele deixe de funcionar em nenhum momento.**

Não é um remake nem uma "inspiração": é a mesma simulação, com as mesmas regras, os mesmos números e os mesmos sorteios, reconstruída rotina por rotina a partir do executável original. A meta é que, partindo do mesmo jogo salvo e da mesma semente, a versão em Rust produza **exatamente** a mesma carreira que o original, bit a bit.

Só depois que a reconstrução for fiel vem a segunda metade: modernizar o que o jogo original não fazia (resolução, fontes, interface, regras e ligas atuais), sobre uma base que entendemos por completo.

## 2. Princípios

1. **Fidelidade antes de tudo.** Cada rotina reescrita tem de dar o mesmo resultado que a original, incluindo arredondamentos de ponto flutuante, estouros de 8 e 16 bits e até lixo de pilha que o original passa adiante, quando isso afeta o resultado.
2. **O jogo continua jogável o tempo todo.** Nada de "parar tudo para reescrever". A versão em Rust entra como uma DLL que o executável original carrega; cada rotina reescrita substitui a original por um desvio (*hook*). A qualquer momento dá para jogar, e as rotinas que ainda não foram reescritas continuam rodando como sempre.
3. **Prova, não confiança.** Nenhuma rotina é considerada pronta porque "parece certa". Ela passa por testes que comparam com a original e, antes disso, provamos que os próprios testes conseguem pegar erros.
4. **Os originais nunca são alterados.** Correções e adaptações vão para cópias geradas por ferramenta, que confere a assinatura do original antes de mexer.
5. **Nada do jogo no repositório.** Nem executáveis, nem dados, nem código descompilado em bruto. O repositório guarda só as ferramentas, o código novo e a documentação.

## 3. Como funciona por dentro

```
cm0102.exe (original, intocado)
   │  carrega
   ▼
ddraw.dll  ← o nosso núcleo em Rust
   ├── vídeo próprio (substitui o DirectDraw: janela, 1920×1080, escala)
   ├── desvios: no começo de cada rotina reescrita, um salto para a versão em Rust
   ├── simulação reproduzível (relógio, memória, disco e sorteio sob controle)
   └── rotinas reescritas (sorteio, datas, motor de partidas...)
```

Cada rotina reescrita pode rodar em três modos:

- **Rust:** a versão nova substitui a original (o modo normal).
- **Original:** só a original roda (referência).
- **Comparação:** as duas rodam lado a lado, a partir do mesmo estado, e qualquer diferença é registrada.

## 4. Como provamos que uma rotina está certa

Cada leva de rotinas passa por quatro provas:

1. **Autoteste:** bilhões de entradas sintéticas, inclusive casos extremos, em que original e Rust têm de dar o mesmo resultado.
2. **Mutações:** alteramos de propósito a versão em Rust (um limiar, um sinal, uma constante) e confirmamos que o autoteste acusa. Se uma mutação passa, ou ela é comprovadamente equivalente (e isso fica documentado) ou o teste tinha uma lacuna, que corrigimos.
3. **Comparação no jogo real:** uma carreira automática roda com as duas versões lado a lado (10 bilhões de comparações por rodada), sem nenhuma divergência.
4. **Carreira completa:** a mesma carreira, com o motor de partidas ligado, até 01/06/2002, em Rust e no original. A "impressão digital" final (2.356.606.166 sorteios e o estado do gerador) tem de ser idêntica.

Rotinas que chamam outras com efeitos colaterais (por exemplo, a narração da partida) são comparadas por **gravação e reprodução**: a original roda e cada chamada interna fica gravada; a versão em Rust é conferida contra essa gravação, sem repetir os efeitos.

## 5. O que já foi feito

### Etapa 0: levantamento (05/10/2026)
- Ferramentas de engenharia reversa: leitura do executável, textos, referências, desmontagem.
- **Mapa do executável:** 11.314 funções, a maior parte ligada ao arquivo-fonte original (`match_eng.cpp`, `Date.cpp`...).
- Formatos dos arquivos de dados.

### Etapa 1: jogo jogável e moderno por fora (05/10/2026)
- Executáveis "portáteis" (sem exigir o CD), gerados por ferramenta.
- Camada de vídeo própria no lugar do DirectDraw: janela redimensionável, 1920×1080 nativo, posição lembrada.

### Etapa 2: banco de provas reproduzível (05 e 06/10/2026)
- O jogo foi tornado **determinístico** dentro da simulação: relógio, memória (alocador fixo), disco, unidades e todas as chamadas ao Windows sob controle.
- Carreira automática "de férias", sem tela e sem cliques.
- **Prova longa:** dez temporadas com Rust e original idênticos.
- As primeiras rotinas em Rust: o gerador de sorteios e as datas do jogo.

### Etapa 3: motor de partidas (06 a 17/10/2026) e simulação, em andamento
- **256 levas, 1194 rotinas** reescritas, de baixo para cima: dos acessores e cálculos às decisões táticas, posse de bola, arbitragem (faltas, cartões, impedimento), estatísticas, movimento dos jogadores, domínio da bola, narração e, como provas de viabilidade, o **valor do chute a gol** e a **força do time** (rotinas de 5 KB).
- Infraestrutura nova: verificação dos desvios (`prologos.py`), porta de gravação para a narração e reprodução de chamadas internas.
- O motor de partidas (fora as telas) chegou a 100% em 17/10/2026; desde então as levas seguem pela simulação (`candidatos.py --simulacao`) e, a partir da 180ª, pelas telas da partida (`match_screens.cpp`).

## 6. Aonde vamos

1. **Fechar o motor com as telas da partida.** As telas montam uma lista de exibição (o `Display` do gerente de telas), comparada como dado; as provas são o jogo do Lucas em modo Rust e comparando.
2. **O resto da simulação:** o dia de jogos, competições, tabelas, transferências, finanças, contratos, lesões, notícias. Mesmo método: rotina por rotina, provada contra a original. **Ordem (Lucas, 22/10/2026):** fechar `match_screens.cpp` e `match_day.cpp` (faltam 6 funções); depois mapear e portar, em blocos, o **save** (`zipdir.cpp`, o contêiner com RLE, e `Setup.cpp`, gravar/carregar; referência: o CM3SaveEditor, já em Rust), o **banco de dados** (`Database.cpp`, `index.cpp`: a leitura dos `Data/*.dat`) e por fim as **competições** (~180 classes em `comp/`: 87 ligas, 60 copas, europeias, intercontinentais e Copa do Mundo, com a estrutura escrita no código). Cada bloco abre frentes nas ferramentas irmãs (editor, CMRTE, Save Game Editor); as competições em Rust são o caminho para o editor mexer na estrutura (item 8).
3. **A interface (decidido em 19/10/2026: abordagem híbrida).** Traduzir a base da interface e a lógica de cada tela, reescrever a apresentação sobre a base conferida pixel a pixel e só depois modernizar.
4. **Independência do executável.** Com a simulação inteira em Rust, o núcleo vira uma biblioteca própria (64 bits), que roda sem o `cm0102.exe`.
5. **Modernização.** Resolução e fontes, fotos ajustadas à tela e, sobre a base fiel, mudanças de regras e realismo.
6. **Novas formas de ver o jogo.** O motor já calcula a partida num tabuleiro (posições de cada jogador e da bola a cada lance); com ele em Rust, dá para desenhar a partida (2D, "jogo de botão") numa interface moderna (Unity, Godot ou outra), lendo o motor.
7. **Modo desafio (ideia do Lucas, 08/10/2026).** Vários jogadores, cada um em casa, jogam a mesma carreira (ex.: "Flamengo na semente 1") e comparam os resultados; quem vai melhor, em tese, joga melhor. A simulação já é reproduzível (mesma semente e mesmas decisões dão o mesmo resultado), mas a mesma semente só garante o mesmo mundo inicial: o jogo usa uma fila única de sorteios, e qualquer decisão diferente muda quantos sorteios são gastos, deslocando toda a sorte dali em diante (inclusive nos jogos dos outros clubes). Para comparar habilidade com justiça, com o motor todo em Rust:
   - **semente por evento**, derivada de semente + data + mandante + visitante (e o mesmo para lesões, mercado, juvenis): os jogos alheios saem iguais para todos e o do jogador começa com a mesma sorte, ficando a diferença por conta das decisões;
   - **contra recarregar o save**: o jogador envia a semente e o registro das decisões, e um verificador refaz a carreira e confirma o resultado (possível justamente porque a simulação é determinística);
   - **mesmas condições**: mesma base (`Data/`), mesma versão e mesmas opções (ligas carregadas, detalhe da partida);
   - só com o motor 100% em Rust: no exe original o lixo de pilha influencia os resultados (ver ROTINAS, nota sobre a impressão digital da carreira).
   É um modo à parte: quebra a fidelidade ao original de propósito e não substitui o jogo idêntico.
8. **Depois do jogo 1:1: os editores dentro do projeto (roadmap do Lucas, 21/10/2026).** Tudo isto vem só depois de o jogo estar inteiro em Rust e idêntico ao original.
   - **O CMRTE dentro do jogo.** Um botão liga a edição na própria tela, como o editor dentro do jogo do FM. A base é o projeto irmão `D:\Sistemas\CMRTE` (o CMRTE v0.1.11 descompilado: núcleo em Rust pronto, provado pelo ouro do original e por mutações). Dentro do jogo não é preciso ler e escrever a memória de outro processo: a edição mexe direto nas estruturas que o jogo usa.
   - **O editor oficial refeito.** Descompilar o editor da base que vem com o jogo e refazê-lo nos moldes do editor do Elifoot II Bombado (`D:\Sistemas\Elifoot II\bombado\editor`): um núcleo sem tela (base, entidades, validação, patch) e a janela por cima (Tauri, HTML/CSS/JS sem Node); a base só grava o que passa na validação, e as mudanças podem sair como patch (a diferença para a base de origem).
   - **O editor de saves no mesmo editor.** A lógica do Save Game Editor já está descompilada e em Rust no projeto irmão `D:\Sistemas\CM3SaveEditor` (o `SaveGame_Editor.exe` v4.0 do Graeme Kelly; núcleo `cm3sav`, abrir e gravar byte a byte como o original, validador e edição em lote). Ela entra no editor novo: um programa só para a base e para o save.
   - **Saves que corrompem, resolvidos na raiz.** Entender por que os saves ficam com ids com problemas (clubes e pessoas repetidos, que hoje só o "reparo" do editor de saves acerta) e corrigir no jogo, para não precisar mais do reparo. Pistas já levantadas nos tópicos antigos do fórum (`ENDERECOS_COMUNIDADE.md`, seção dos tópicos antigos): editores usados juntos, mudança de nacionalidade, cargo ou aposentadoria por editor, dois exes na mesma pasta dividindo os `.tmp`, queda de energia na gravação, o VirtualStore do Windows.
9. **Sugestões que atravessam os projetos (para discutir).**
   - **Um núcleo de dados comum.** Hoje os formatos da base e do save estão descritos em três lugares (este projeto, o CM3SaveEditor e o CMRTE). Uma biblioteca Rust só com os formatos, usada pelo jogo, pelo editor e pela edição na tela, evita três versões das mesmas estruturas.
   - **O validador no jogo.** O validador do editor de saves pode rodar no próprio jogo ao gravar e ao carregar: o problema é apontado (e, quando seguro, corrigido) no momento em que nasce, e não anos depois.
   - **Os oráculos dos irmãos na cadeia.** O ouro do CMRTE e o oráculo byte a byte do `cm3sav` podem entrar nos testes daqui, provando junto o jogo e os editores.
   - **O save estendido junto com o editor.** O formato para passar de 4 GB deve nascer junto com o editor, que precisa ler os dois formatos.
   - **Regras em dados.** O que hoje só se muda com patch de bytes no exe (ano inicial, limites de estrangeiros, substituições, janelas de transferência, ligas jogáveis; ver os tópicos antigos do fórum) vira configuração editável no editor, como as regras em dados do Elifoot II Bombado.
   - **Outras plataformas.** Com o jogo inteiro em Rust e a janela própria, o caminho do Elifoot II vale aqui: Linux, depois Mac.

## 7. Riscos e decisões em aberto

- **Ritmo:** de 2 a 10 rotinas por leva e de 30 a 60 minutos de testes por leva. O motor sai em mais algumas dezenas de levas; o jogo inteiro, no mesmo método, é trabalho de meses.
- **Máquina compartilhada:** os testes pesados respeitam uma trava entre sessões e um vigia de memória (houve um vazamento de driver que travou o PC).
- **Interface:** decidida em 19/10/2026 (abordagem híbrida, seção 6.3); as telas da partida já estão em Rust e em teste.
- **Rotinas grandes:** o método escala. O valor do chute (5,2 KB) e a força do time (5,1 KB, cheia de chamadas internas com efeito) bateram com a original na primeira tentativa; os problemas foram só de infraestrutura de teste.
