# Conferência da lesão no jogo (saves do CM3SaveEditor)

25/10/2026. Pedido do Lucas (via orquestrador): carregar no laboratório os 5 saves de conferência da lesão
do CM3SaveEditor (SGE) e comparar com as previsões de `D:\Sistemas\CM3SaveEditor\docs\LESAO.md` §3, para o
editor liberar o "Curar" no save. Jogador: Darren Anderton (Tottenham, staff 52037), lesionado no save de
partida (tipo 26, registro 338).

Convenção: **[observado]** = lido na memória do jogo; **[inferência]** = conclusão a partir do observado e
da desmontagem.

## Método

- Bancada da prova do save (`engenharia-reversa/prova_save.py`, carga): jogo de testes (`jogo_testes\cm0102-1080.exe`),
  **modo original** da simulação (`CM_ORIGINAL=1`), `-load <cópia>` + `CM_FERIAS=1`, janela oculta 960x540,
  sem cliques, dentro da trava do laboratório; `game.cfg`, `hall_of_fame.bin`, `*.lng` e `*.tmp` guardados e
  devolvidos (a bancada confirmou o `game.cfg` restaurado nas 6 cargas). DLL do jogo de testes não trocada.
- Um vigia (molde do `renomear_no_save.py --carregar`) lê a memória a cada 0,5 s sem clicar e registra cada
  mudança de: data do jogo (`0xae2c90`), o bloco de 31 bytes do staff 52037 (`[[0xb63d28]] + 31 × 52037`) e
  o registro de 48 bytes apontado por bloco+0x0E, além do registro 338 (o da lesão no save de partida), e a
  caixa de erro (`0xae24d0`). O "estado calculado" é a regra do 0x62c400 aplicada aos campos lidos
  (dias desde +0x20 contra +0x2A/+0x28; registro 0xFF = 0) **[inferência]**.
- Script descartável (fora do repositório): `conferir_lesao.py` no scratchpad da sessão.
- A identidade da pessoa 52037 confere pelos valores: no controle, o bloco e o registro lidos na memória são
  exatamente os do `LESAO.md` §3 (bloco de 212/2001, +8 = 6613, +0xC = −273, +0xE = 338, +0x12/+0x13 = 26/1,
  +0x1D = 26, +0x1E = 50; registro 338: tipo 26, início 192/2001, +0x28 = 99, +0x2A = 59) **[observado]**.
  (A leitura do nome pela tabela de nomes saiu vazia; não insisti.)

### Cópias usadas (em `%TEMP%\cm0102_lesao\`, MD5 igual ao original antes e depois de cada carga)

| Save | MD5 |
|---|---|
| `Testes Claude 2 - gravado pelo editor original.sav` (controle, partida sem edição; de `CM3SaveEditor\jogo\saves\`) | `5f2ff69e646223747fe511c40dbee5eb` |
| `Lesao A_cmexplorer.sav` | `b435e0ccf7e74fedb35ab452eb23dd14` |
| `Lesao B_cmrte.sav` | `8ba221ede5985b26b2334f5b5f17ae8f` |
| `Lesao C_os_dois.sav` | `bb7be58037be7c572f84940b3aee6856` |
| `Lesao D_so_o_grau.sav` | `bd8e4d15e954cfe436061470bb316962` |
| `Lesao E_proposta.sav` | `b34eee8e1e34bc7bd0402e0eaf9a5098` |

### Como as cargas terminaram

As 6 cargas (controle inclusive) carregaram sem caixa, andaram de 213/2001 (1º/08) a 217/2001 e pararam
**todas na mesma data** (segunda, 6 de agosto de 2001, noite) na caixa
`v3.9.68 player_stats..cpp 3828 (PLAYER_STATS::create_stats() : Unable to allocat…` **[observado]**. Como o
save sem edição para igual, a caixa é limite da bancada com este save grande (158.727 pessoas, 240 MB) no
modo simulação, não efeito das edições **[inferência]**. Os 4 dias de jogo bastam para a carga, o 1º dia e
alguns dias seguintes.

## Resultado

| Save | Observado na memória | Previsão do SGE (`LESAO.md` §3) | Bate? |
|---|---|---|---|
| Controle (sem edição) | Na carga, bloco de 212/2001, +0xE = 338. Na 1ª atualização (dia 213) e em todo dia seguinte: continua com +0xE = 338, registro 338 tipo 26, estado calculado 2 (fora); +0x2C/+0x2E caem 1 por dia (78/38 → 73/33); condição 66% → 65 → 64 → 63 → 62% | Estado 2, "Out for ~38 dias", 66% | **Sim** |
| A (zera +0x28/+0x2A) | Na carga, nada muda (bloco de ontem, +0xE = 338). Na 1ª atualização, ainda no dia 213: **o jogo cura**: +0xE = −1, registro 338 = 0xFF (livre), **histórico criado** (+0x14 = +0x18 = 1522416, eram −1), +0x12/+0x13 = 26/1 mantidos; condição 66% → 72% e sobe (94% no dia 214, 100% no 215). No dia 214 o registro 338 já foi **reaproveitado** por outra lesão (tipo 31, início 214) | Curado pelo próprio jogo na 1ª consulta (bloco de ontem), com histórico; registro livre; recorrência fica; condição "perto de 66%" | **Sim** (a condição sobe a partir de 72% em vez de ficar em ~66%; a notícia de recuperação não foi vista: sem tela) |
| B (bloco como o CMRTE) | Na carga e depois: +0xE = −1, condição 100% (+8 = 10000), +0xC = −185 (depois −198, −211…), +0x12/+0x13 = −1/0, +0x1D = −1. **Registro 338 continua tipo 26 (ocupado) sem dono**, decrementado todo dia (+0x2C 78 → 73), nunca reaproveitado nos 4 dias | Sem lesão, ~100%, sem histórico; registro 338 órfão; recorrência apagada | **Sim** |
| C (A + B) | Igual a B; o registro 338 órfão com +0x28/+0x2A = 0, tipo 26, decrementado todo dia | Igual a B | **Sim** |
| D (só +0x12 = 0xFF) | Igual ao controle em tudo, menos +0x12 = −1: +0xE = 338, estado calculado 2, condição 66% → 62% | **Continua lesionado** (~38 dias) | **Sim** |
| E (proposta: registro+0 = 0xFF, +0xE = −1, condição 10000) | Na carga: +0xE = −1, condição 100%, registro 338 = 0xFF; +0x12/+0x13 = 26/1, histórico −1 (sem nó). **Logo na carga (dia 213) o registro 338 foi reaproveitado** por outra lesão (tipo 31, início 213); o Anderton segue com +0xE = −1 e 100% nos 4 dias | Sem lesão, limpo, invariante "ocupado ⇔ apontado" preservado | **Sim** |

Observações:

1. O bloco do Anderton é atualizado (bloco+0 = data do jogo) logo no dia da carga e depois todo dia, nos 6
   saves **[observado]**: o "preguiçoso" do 0x62c680 roda cedo para ele (alguma rotina diária o consulta).
2. No E e no A o registro liberado volta à fila da alocação e é reaproveitado em 0 ou 1 dia (a alocação pega
   o primeiro 0xFF) **[observado]**. A dona do tipo 31 não foi identificada; não é o Anderton (+0xE dele
   é −1) **[observado]**.
3. No B/C o registro órfão não teve efeito visível nos 4 dias **[observado]** (sem caixa); é um estado que o
   jogo nunca produz e um registro perdido para sempre **[inferência]**.

## Recomendação para o "Curar" do editor

Usar a forma **E** (a proposta do `LESAO.md` §4), que reproduz a cura do jogo `0x62e700` sem o histórico
e sem a notícia:

- registro de 48 apontado por bloco+0x0E (só se 0 ≤ índice < contagem): **+0x00 = 0xFF**;
- bloco de 31 bytes: **+0x0E..+0x11 = FF FF FF FF**;
- opcional: condição **+0x08 = 10000** (e +0x0A = 10000 se não for);
- não mexer em +0x0C, +0x12/+0x13 (recorrência), +0x14/+0x18 (histórico), +0x1C..+0x1E nem nas datas/durações
  do registro.

No jogo ela deu o mesmo estado que a cura do próprio jogo deixa (comparada com o A, onde o jogo curou):
+0xE = −1, registro livre e reaproveitado, recorrência mantida, nada órfão **[observado]**. A única
diferença para a cura do jogo é não gravar o nó do histórico (o A ganhou o nó 1522416); a lesão curada pelo
editor não aparece no histórico do jogador **[inferência]**. Se o editor quiser o histórico e a notícia, a
alternativa é a A (zerar +0x28/+0x2A e deixar o jogo curar na 1ª atualização), que também funcionou, mas
depende do bloco ter data diferente da do jogo e leva a condição pela fórmula (72% → 100% em 2 dias), não na
hora. **B/C não** (registro órfão, recorrência e histórico apagados); **D não cura**.

A conferir pelo Lucas, com tela (fora do alcance desta leitura): o perfil ("None") e a lista de lesionados
do Tottenham no E; a notícia de recuperação no A.
