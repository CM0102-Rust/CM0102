# Renomear uma chave fixa impede o save de carregar?

Teste de 24/10/2026, com o executável original (modo original da simulação). Ferramenta: `engenharia-reversa/renomear_no_save.py`.

## Resposta curta

- **[observado]** Sim, mas só quando muda o nome **em inglês** de uma chave, que fica no item `eng.lng` do save. A carga para na caixa "Unable to find the F_C__BARCELONA index" (ou "...BRAZIL index"), `v3.9.68 index.cpp 5809`, e espera um clique.
- **[observado]** Renomear o mesmo clube ou a mesma nação no `club.dat`/`nation.dat` do save **não** impede a carga neste save (nomes traduzidos). Mudar o nome curto também não.
- **[inferência, pela desmontagem]** Num save com os nomes em inglês (idioma 0), a chave é procurada direto no nome do `club.dat`/`nation.dat`. Aí renomear lá uma chave da lista impede a carga.

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
python engenharia-reversa/renomear_no_save.py --carregar COPIA.sav [--modo original|rust] [--ate AAAA-MM-DD]
```

- A ferramenta troca só o campo, gravando o nome novo e um zero. No tipo 3, a troca é no lugar. No tipo 4, recodifica o item em RLE válido (b ≤ 0x80 literal, b > 0x80 vira `0x81 b`) e desloca as posições seguintes do índice.
- No fim, confere que todos os itens descomprimidos da saída são iguais aos da entrada, menos o item trocado, que tem de sair igual ao esperado.
- O tipo 4 foi conferido só por essa conferência, numa cópia do `TesteTelasComprimido.sav`. O jogo não carregou nenhum save tipo 4 regravado.
- O `--carregar` usa a carga da `prova_save.py` (bancada: trava do laboratório, `game.cfg` e `*.tmp` restaurados) e o vigia da caixa.

## Onde fica o idioma

Levantamento só pela desmontagem e pela leitura (sem abrir o jogo) do `FeriasTesteSave.sav` (MD5 `33fda465193b5e8e6d7ace93ecd44559`, igual antes e depois). O Lucas manteve o bloqueio do nome-chave em todos os idiomas, e por isso o caso do idioma 0 não foi carregado no jogo.

- **Na memória:** o dword `[0x9b5984]` (`.data`). A carga zera ele para −1 em 0x52851c, antes de ler o save. A troca de idioma do jogo (0x5339c0) grava nele o idioma novo em 0x535806, e o leitor é 0x535920. **[observado na desmontagem]**
- **No arquivo:** são os **4 últimos bytes do item `index.dat`** (descomprimido). A gravação (0x531400..0x5314b7) escreve, nesta ordem, depois dos 0x16 registros de 0x43 bytes: um byte de marca (=1), 0x1cc bytes, a contagem u32 `[0xae235c]` (−1 se a lista `[0xae23fc]` não existe), a lista (contagem × 4 bytes: linha do save → linha do `eng.lng`) e o idioma u32. A leitura (0x52881a..0x528974) segue a mesma ordem. **[observado na desmontagem]**
- **No `FeriasTesteSave.sav`:** o `index.dat` tem 30.067 bytes, a cauda começa em +0x5ca, a contagem é 7.029 e o idioma, em **+0x756f** (os 4 últimos bytes), vale **9** (POR). O `eng.lng` está no save. **[observado no arquivo]**
- **Marca 0:** se o byte de marca for 0 (save sem a cauda), a carga põe `[0x9b5984] = 0` (0x528a30) e não lê o resto. **[observado na desmontagem]**
- **Idioma 0 no save:** **[inferência]** vale 0 quando a carreira foi criada em inglês (`LANGUAGE=ENG_UK` na configuração automática, ou o jogo em inglês) e ninguém trocou o idioma depois. Aí a chave é casada direto no nome +4 do `club.dat`/`nation.dat` (ver "Por que") e o `eng.lng` não é aberto. Para saber o idioma de um save, basta ler o último u32 do `index.dat`. A presença do item `eng.lng` é só um indício, porque o jogo só abre o arquivo quando o idioma ≠ 0.
