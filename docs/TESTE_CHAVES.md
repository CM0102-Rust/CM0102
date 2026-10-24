# Renomear uma chave fixa impede o save de carregar?

Teste de 24/10/2026, com o executável original (modo original da simulação). Ferramenta: `engenharia-reversa/renomear_no_save.py`.

## Resposta curta

- **[observado]** Sim, mas só quando muda o nome **em inglês** de uma chave, que fica no item `eng.lng` do save. A carga para na caixa "Unable to find the F_C__BARCELONA index" (ou "...BRAZIL index"), `v3.9.68 index.cpp 5809`, e espera um clique.
- **[observado]** Renomear o mesmo clube ou a mesma nação no `club.dat`/`nation.dat` do save **não** impede a carga neste save (nomes traduzidos). Mudar o nome curto também não.
- **[observado, ver "Matriz por idioma"]** Num save com os nomes em inglês (idioma 0), a chave é procurada direto no nome do `club.dat`/`nation.dat`. Aí, renomear lá uma chave da lista impede a carga, com o jogo em inglês ou em português. **Renomear o nome completo de um clube ou de uma nação da lista só no `club.dat`/`nation.dat` não é seguro em todos os idiomas.**

## Por que (desmontagem)

- `set_key_ids` 0x601a40 tem uma chamada só, em 0x52d8fa, no fim de `load_database`. É o mesmo caminho da criação a partir do `Data\` e da carga do save (`load_game_data` → `load_database(…, 1)`). **[observado]**: a caixa aparece na carga.
- Com o idioma dos nomes `[0x9b5984] ≠ 0` (vem da cauda do `index.dat` do save; neste save vale 9), o jogo abre `CM3_TEMP\eng.lng` (0x603c5e). A carga extrai esse arquivo do próprio save. As chaves são casadas com os nomes em inglês: clubes pela lista 0xae23fc (linha do save → linha do `eng.lng`) em 0x603e6f/0x603e7e, nações em 0x604081. Sem o `eng.lng`, ou com idioma 0, o jogo usa o nome em +4 do registro de nação (0x604226) e do de clube (0x604391).
- Só o nome longo é comparado (`_stricmp` 0x951c50, sem distinguir maiúsculas). O nome curto nunca é.
- A sessão do CMRTE também viu na desmontagem **[observado]** que o `set_key_ids` passa ao 0x6146b0 só o nome completo do clube (`[0xae23b4]+i+4`), sem o nome curto, e que o editor dela não muda nome de nação. Isso confere com o ramo do idioma 0 descrito acima. No ramo traduzido, o que o 0x6146b0 recebe é a linha do `eng.lng`.
- Cada chave que ficar −1 chama 0x60d780: grava "Unable to find the %s index" em 0xde1f64 e abre a caixa 0x5e8290 com "v3.9.68 index.cpp 5809" (0xae24d0). Essa caixa espera clique.

## Layout do `.lng`

8 contagens (0x20 bytes). Depois vêm, em sequência: clubes (0x4f bytes cada; 13.640 linhas, a base inteira e não só a cortada), continentes (0xba), competições de nação (0x53) e nações (0x6d). O nome fica em +0 (51 bytes) e o curto em +0x34 (26 bytes).

## Cópias e resultado

Base: cópia do `jogo\FeriasTesteSave.sav` (contêiner tipo 3, 142 itens), MD5 `33fda465193b5e8e6d7ace93ecd44559`, igual antes e depois de tudo. As cópias ficaram em `%TEMP%\cm0102_teste_chave\`. Cada carga usou `-load`, `CM_FERIAS` e `--ate 2001-07-31`, no modo original. Um vigia lia a memória do jogo (0xae24d0, 0xde1f64), sem clicar, e encerrava o jogo quando a caixa aparecia.

| Cópia | Mudança | MD5 | Resultado |
|---|---|---|---|
| `base.sav` | nenhuma | 33fda465193b5e8e6d7ace93ecd44559 | carrega (2001-211, impressão d7abb94be53efcc4) |
| `ida_volta.sav` | regravada sem mudança | igual à base, byte a byte | — |
| `a_clubdat_barcelona.sav` | `club.dat`: "F.C. Barcelona" → "F.C. Barcelonx" (clube **da lista**) | e42bc66df2017586ca9cd3f760ac0101 | **carrega**, mesma impressão |
| `a2_englng_barcelona.sav` | `eng.lng` (clubes): o mesmo nome | b85c1b7092a4706cb7e005de379649ee | **caixa**: "Unable to find the F_C__BARCELONA index", index.cpp 5809; para em 5 s |
| `b_controle_villarreal2.sav` | "Villarreal C.F." → "Villarreal C.X." no `eng.lng` **e** no `club.dat` (fora da lista) | fa973efd94bf1e68590ced241bdf041a | carrega, mesma impressão |
| `c_nationdat_brasil.sav` | `nation.dat`: "Brasil" → "Brasix" (nação da lista, nome traduzido) | 3f586e8f7686962dc6ad2f65d8171b0f | **carrega**, mesma impressão |
| `c2_englng_brazil.sav` | `eng.lng` (nações): "Brazil" → "Brazix" | 8c4e3554a98cb64c48c454d45809fcde | **caixa**: "Unable to find the BRAZIL index", index.cpp 5809 |
| `d_curto_barcelona2.sav` | nome curto "Barcelona" → "Barcelonx" no `club.dat` **e** no `eng.lng` | 46057c6d63911c8350bf79c3cd5946d3 | carrega, mesma impressão |

As cópias que carregaram chegaram à mesma data, com os mesmos sorteios e a mesma impressão da base. Só a impressão `banco` mudou, porque ela cobre os nomes trocados.

## E o `.qst`?

**[inferência]** Vale o mesmo. O `.qst` é o mesmo contêiner, com o mesmo `index.dat` (cauda e idioma) e os mesmos `*.lng`, e o `-load` dele passa pelo mesmo `load_database`, que chama o `set_key_ids` 0x601a40 no fim (a única chamada, 0x52d8fa). Um `.qst` já criado não traz as chaves prontas: elas são globais (0x9cf178..), não vão para o save e são refeitas pelos nomes em cada carga. Não testei carregando um `.qst`, porque o `-load` de um `.qst` para na tela do nome do técnico (`docs/COMO_CONTINUAR.md`, "Prova do save").

## A ferramenta

```
python engenharia-reversa/renomear_no_save.py ENTRADA SAIDA --item club.dat --de "F.C. Barcelona" --para "F.C. Barcelonx" [--curto]
python engenharia-reversa/renomear_no_save.py ENTRADA SAIDA --item eng.lng --tabela nacao --de Brazil --para Brazix
python engenharia-reversa/renomear_no_save.py ENTRADA SAIDA          (ida e volta)
python engenharia-reversa/renomear_no_save.py --carregar COPIA.sav|COPIA.qst [--idioma 0|9] [--modo original|rust] [--ate AAAA-MM-DD]
python engenharia-reversa/renomear_no_save.py --gerar-qst SAIDA.qst [--language ENG_UK|POR]
```

- A ferramenta troca só o campo, gravando o nome novo e um zero. No tipo 3, a troca é no lugar. No tipo 4, recodifica o item em RLE válido (b ≤ 0x80 literal, b > 0x80 vira `0x81 b`) e desloca as posições seguintes do índice.
- No fim, confere que todos os itens descomprimidos da saída são iguais aos da entrada, menos o item trocado, que tem de sair igual ao esperado.
- O tipo 4 foi conferido só por essa conferência, numa cópia do `TesteTelasComprimido.sav`. O jogo não carregou nenhum save tipo 4 regravado.
- O `--carregar` usa a carga da `prova_save.py` (bancada: trava do laboratório, `game.cfg` e `*.tmp` restaurados) e o vigia da caixa.
- `--idioma N` grava o idioma do jogo no `game.cfg` dos testes só durante a carga (a bancada devolve a cópia).
- A carga extrai os `*.lng` do save na pasta do jogo dos testes, e eles entram na impressão das carreiras. Por isso a ferramenta guarda os `*.lng` dessa pasta antes e os devolve depois.
- A cada 2 s, o vigia lê da memória `[0x9b5984]`, o idioma do jogo `[[0xae3138]+8]`, as chaves de Barcelona (0x9d0138) e de Brazil (0x9cf260) e o nome, na memória, do clube da chave de Barcelona.
- Com um `.qst`, a carga para na tela do nome do técnico. O vigia dá a carga como feita quando a chave sai de −1 e passam 15 s sem caixa (o `set_key_ids` abre a caixa na hora). Aí encerra o jogo.
- `--gerar-qst` grava um `.qst` pelo início rápido (o da `prova_save.gravar`) com o `LANGUAGE=` pedido e informa o idioma da cauda do `index.dat`.

## Onde fica o idioma

Levantamento só pela desmontagem e pela leitura (sem abrir o jogo) do `FeriasTesteSave.sav` (MD5 `33fda465193b5e8e6d7ace93ecd44559`, igual antes e depois). O Lucas manteve o bloqueio do nome-chave em todos os idiomas, e por isso o caso do idioma 0 não foi carregado no jogo.

- **Na memória:** o dword `[0x9b5984]` (`.data`). A carga zera ele para −1 em 0x52851c, antes de ler o save. A troca de idioma do jogo (0x5339c0) grava nele o idioma novo em 0x535806, e o leitor é 0x535920. **[observado na desmontagem]**
- **No arquivo:** são os **4 últimos bytes do item `index.dat`** (descomprimido). A gravação (0x531400..0x5314b7) escreve, nesta ordem, depois dos 0x16 registros de 0x43 bytes: um byte de marca (=1), 0x1cc bytes, a contagem u32 `[0xae235c]` (−1 se a lista `[0xae23fc]` não existe), a lista (contagem × 4 bytes: linha do save → linha do `eng.lng`) e o idioma u32. A leitura (0x52881a..0x528974) segue a mesma ordem. **[observado na desmontagem]**
- **No `FeriasTesteSave.sav`:** o `index.dat` tem 30.067 bytes, a cauda começa em +0x5ca, a contagem é 7.029 e o idioma, em **+0x756f** (os 4 últimos bytes), vale **9** (POR). O `eng.lng` está no save. **[observado no arquivo]**
- **Marca 0:** se o byte de marca for 0 (save sem a cauda), a carga põe `[0x9b5984] = 0` (0x528a30) e não lê o resto. **[observado na desmontagem]**
- **Idioma 0 no save:** **[observado no `.qst` do início rápido com `LANGUAGE=ENG_UK`; ver "Matriz por idioma"]** vale 0 quando a carreira foi criada em inglês (`LANGUAGE=ENG_UK` na configuração automática, ou o jogo em inglês) e ninguém trocou o idioma depois. Aí a chave é casada direto no nome +4 do `club.dat`/`nation.dat` (ver "Por que") e o `eng.lng` não é aberto. Para saber o idioma de um save, basta ler o último u32 do `index.dat`. A presença do item `eng.lng` é só um indício, porque o jogo só abre o arquivo quando o idioma ≠ 0.

## Matriz por idioma

Teste de 24/10/2026, pedido do Lucas. Pergunta: renomear o **nome completo** de um clube da lista de chaves (`levas/227/codigo.rs`) **só no `club.dat`**, como faria o editor (que nunca toca o `eng.lng`), deixa o save carregar em qualquer configuração de idioma? Condições: modo original da simulação, jogo dos testes, janela oculta 960×540, sem cliques. As cargas de `.sav` vão até 2001-07-31.

### Como o idioma é escolhido na carga (desmontagem e observado)

1. **[desmontagem]** A chave `-load` (0x81bd70) aplica as preferências (0x5d8480 em 0x81be9e) **antes** de carregar. Nesse momento ainda não há base carregada (`[0xae23a8]` = 0), e a troca de idioma 0x5339a0 não faz nada.
2. **[desmontagem]** O `load_database` (0x528540) extrai os `*.lng` do save para `CM3_TEMP` (0x933320). Se o save não tem nenhum, ou se falta o `.lng` do idioma do jogo (0x535930), o `load_database` põe o jogo em inglês (0x5d8020(0)). Essa troca muda só a configuração e a interface, não os nomes.
3. **[desmontagem]** O idioma dos nomes `[0x9b5984]` é lido **do save**: é o último u32 do `index.dat` (0x52896b). O `set_key_ids` (0x601a40, chamado em 0x52d8fa no fim do `load_database`) casa as chaves com esse idioma. Se for ≠ 0, casa pelo `eng.lng` extraído. Se for 0, casa pelo nome em +4 do `club.dat`/`nation.dat`. **A configuração do jogo não entra nessa decisão.**
4. **[desmontagem]** Depois da carga, o `-load` aplica as preferências de novo (0x5d8480 em 0x81c002). Se o idioma do jogo for diferente de `[0x9b5984]`, ele chama a troca de idioma (0x5d85a8..0x5d85bd → 0x5339a0). A troca passa primeiro pelo inglês (aplica o `eng.lng`) e depois aplica o `.lng` do idioma novo, regravando os nomes da base.
5. **[observado]** No fim da carga, `[0x9b5984]` vale sempre o idioma do jogo: o save POR carregado em inglês termina com 0, e o save ENG carregado em português termina com 9. Quando a caixa aparece, porém, `[0x9b5984]` ainda tem o valor do save (0 no save ENG com o jogo em POR). Ou seja, a caixa vem antes da troca.
6. **[observado]** A troca **apaga a edição**. No save POR carregado em inglês, o clube da chave de Barcelona aparece na memória como "F.C. Barcelona", e não como "F.C. Barcelonx". No save POR carregado em português, a edição fica ("F.C. Barcelonx").

### Cópias

As cópias ficaram em `%TEMP%\cm0102_teste_chave\`. O `FeriasTesteSave.sav` teve MD5 `33fda465193b5e8e6d7ace93ecd44559` antes e depois de tudo. O `game.cfg` dos testes foi restaurado em cada carga, e não sobrou `quick_start.cfg`, `auto_setup.cfg` nem `PROVA.cfg`.

| Cópia | Origem | Mudança | MD5 |
|---|---|---|---|
| `base.sav` | cópia do `FeriasTesteSave.sav` (idioma 9) | nenhuma | 33fda465193b5e8e6d7ace93ecd44559 |
| `a_clubdat_barcelona.sav` | `base.sav` | `club.dat`: "F.C. Barcelona" → "F.C. Barcelonx" (**da lista**) | e42bc66df2017586ca9cd3f760ac0101 |
| `e_controle_villarreal_clubdat.sav` | `base.sav` | `club.dat`: "Villarreal C.F." → "Villarreal C.X." (controle, fora da lista) | 0d38b78986479826ed3eba66b3d7bf5a |
| `c_nationdat_brasil.sav` | `base.sav` | `nation.dat`: "Brasil" → "Brasix" (**da lista**) | 3f586e8f7686962dc6ad2f65d8171b0f |
| `eng_base.qst` | gravado pelo jogo: início rápido com `LANGUAGE=ENG_UK` (ENG, base mínima); idioma no `index.dat` = **0**; tem os 10 `*.lng` | nenhuma | 4c1c985e16fcbda8d4262a5be420772d |
| `eng_clubdat_barcelona.qst` | `eng_base.qst` | `club.dat`: "F.C. Barcelona" → "F.C. Barcelonx" | c01c36f0ac702cdabc6b997ba891b137 |
| `eng_controle_villarreal_clubdat.qst` | `eng_base.qst` | `club.dat`: "Villarreal C.F." → "Villarreal C.X." | 0edf092c83dd5b34beb09e73107b813a |
| `eng_nationdat_brasil.qst` | `eng_base.qst` | `nation.dat`: "Brazil" → "Brazix" | 4d0a618b34ecb07f9803cf4c4a89cef0 |
| `por_base.qst` | o mesmo início rápido com `LANGUAGE=POR`; idioma = **9** (controle do método `.qst`) | nenhuma | a08de2630963aac61f3dd02813c682e2 |
| `por_clubdat_barcelona.qst` | `por_base.qst` | `club.dat`: "F.C. Barcelona" → "F.C. Barcelonx" | fac6bf6746062283f073dd5cf4ca01a2 |
| `por_controle_villarreal_clubdat.qst` | `por_base.qst` | `club.dat`: "Villarreal C.F." → "Villarreal C.X." | e07460980ed67da8a3706cf8baed52b0 |
| `por_nationdat_brasil.qst` | `por_base.qst` | `nation.dat`: "Brasil" → "Brasix" | 4f81161cd417025a28cae863f89d8fa9 |

**Por que `.qst` no idioma 0.** O jogo não grava save com o técnico de férias, e o único `.sav` com técnico é o do Lucas, em POR. Um `.sav` genuíno em inglês pediria cliques (menu Salvar). O `.qst` do início rápido é gravado pelo próprio jogo. Ele tem o mesmo contêiner, a mesma cauda do `index.dat` e os mesmos `*.lng`. O `-load` dele passa pelo mesmo `load_database`/`set_key_ids` e só depois para na tela do nome do técnico, e isso basta para a pergunta. Os `por_*.qst` dão o mesmo resultado que o `.sav` POR, o que confirma o método.

### Resultado [observado]

O idioma do jogo vem de `--idioma` (+8 do `game.cfg` dos testes; 0 = inglês, 9 = português). "Carrega" quer dizer: o `.sav` chegou a 2001-07-31 (2001-211); o `.qst` teve as chaves feitas e passou 15 s sem caixa. "Caixa" é a "v3.9.68 index.cpp 5809", que espera clique.

| Save (idioma dos nomes) | Mudança | Jogo em português (9) | Jogo em inglês (0) |
|---|---|---|---|
| `.sav` POR (9) | nenhuma | carrega (teste anterior: d7abb94be53efcc4) | carrega, impressão d6c1ca019c200c59, `[0x9b5984]`=0 no fim |
| `.sav` POR (9) | Barcelona no `club.dat` | **carrega**, d7abb94be53efcc4; na memória, "F.C. Barcelonx" | **carrega**, d6c1ca019c200c59; na memória, "F.C. Barcelona" (a troca apagou a edição) |
| `.sav` POR (9) | controle Villarreal no `club.dat` | carrega, d7abb94be53efcc4 | carrega, d6c1ca019c200c59 |
| `.sav` POR (9) | Brasil no `nation.dat` | carrega (teste anterior) | **carrega**, d6c1ca019c200c59 |
| `.qst` ENG (0) | nenhuma | carrega; `[0x9b5984]`=9 no fim | carrega; `[0x9b5984]`=0 |
| `.qst` ENG (0) | Barcelona no `club.dat` | **CAIXA**: "Unable to find the F_C__BARCELONA index"; `[0x9b5984]`=0, chave −1 | **CAIXA**: "Unable to find the F_C__BARCELONA index" |
| `.qst` ENG (0) | controle Villarreal no `club.dat` | carrega | carrega |
| `.qst` ENG (0) | Brazil no `nation.dat` | **CAIXA**: "Unable to find the BRAZIL index" | **CAIXA**: "Unable to find the BRAZIL index" |
| `.qst` POR (9) | nenhuma / Barcelona / controle / Brasil | carrega (os 4); na memória, "F.C. Barcelonx" | carrega (os 4); na memória, "F.C. Barcelona" |

As cargas do `.sav` com o jogo em português repetem as do teste anterior, agora com o retrato da memória. As do `.sav` com o jogo em inglês dão todas a mesma impressão da base em inglês. Essa impressão é diferente da do jogo em português porque os nomes são outros. As cargas a partir das 15h34 usaram a `ddraw.dll` d9d0817d0d42a834c67a0dc1fc4b377d (a da 228 publicada), e as anteriores podem ter usado a de antes, trocada às 15h31. Isso não muda o resultado: tudo rodou no modo original, e o `.sav` deu a mesma impressão em todas as cargas do mesmo idioma.

### Conclusão

- **[observado]** **Não é seguro em qualquer idioma.** Num save com os nomes em inglês (último u32 do `index.dat` = 0), renomear só no `club.dat`/`nation.dat` o nome completo de um clube ou de uma nação da lista **impede a carga**. Isso vale com o jogo em inglês **e** em português: a carga para na caixa, que espera clique. Pela regra do Lucas, a edição do nome fica **bloqueada**.
- **[observado]** Quem decide é o idioma **gravado no save**, e não a configuração do jogo: a caixa vem antes da troca para o idioma do jogo.
- **[observado]** Nos saves em outro idioma (≠ 0, com `eng.lng`), a carga funciona nas duas configurações. Mas a edição some sempre que o jogo troca os nomes, seja porque o jogo está noutro idioma, seja por uma troca nas opções: o `eng.lng` e o `.lng` de destino regravam o nome.
- **[inferência]** O idioma de um save muda quando ele é gravado de novo com o jogo noutro idioma. No fim da carga, `[0x9b5984]` vale o idioma do jogo, e a gravação (0x5314b3) escreve esse valor. Um save POR carregado e gravado com o jogo em inglês passa então a ter idioma 0, e a partir daí a mesma edição impede a carga. Por isso, liberar a edição só quando o idioma do save for ≠ 0 também não seria seguro ao longo da vida do save.
