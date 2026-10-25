# Conferência da remoção de suspensão no jogo

25/10/2026. Pedido do Lucas (via orquestrador): "Suspensão: o CM0102 prova no laboratório, sem a minha tela".
Provar que remover a suspensão como o editor (`CM0102-Editor/CMRTE`) vai fazer funciona no jogo: o modelo de
bytes do recurso aceito (`docs/MAPA_DISCIPLINA.md` §2) aplicado no `discipline.dat` de cópias do
`jogo/FeriasTesteSave.sav`, carregadas no jogo original. Molde: `docs/CONFERENCIA_LESAO.md`.

Convenção: **[observado]** = lido no arquivo ou na memória do jogo; **[inferência]** = conclusão a partir do
observado e da desmontagem.

## Método

- **Base:** cópia de `jogo/FeriasTesteSave.sav` em `%TEMP%\cm0102_suspensao\`. O original ficou intocado
  (MD5 `33fda465193b5e8e6d7ace93ecd44559` antes e depois) **[observado]**. Contêiner tipo 3 (sem compressão),
  142 itens; `discipline.dat` com 1.602.112 bytes, 10 regras **[observado]**.
- **As duas pessoas, no `discipline.dat` da cópia** **[observado]**:
  - **51013**: vaga 3 = (regra 0 = FIFA, entrada 36); entrada +6 = 33; suspensão 33 = 1 jogo, 0 cumprido,
    início 113/2001, regras `[−1, −1, −1, 0]`, motivo 2 (vermelho), bits 4 (sem o bit 1); par `(51013, 3)` no
    índice por nação (a nação 3 é a do staff +0x1a).
  - **29220**: vaga 3 = (regra 0, entrada 0); entrada +6 = 0; suspensão 0 = **por tempo, 304 dias**, início
    0/2001, regras `[−1, −1, −1, 0]`, motivo 3 (conduta), bits 6 (bit 1: vale em todas as competições); par
    `(29220, 61)` no índice (staff +0x1a = 61).
  - Nenhum pendente das duas (o único pendente do save é da 46487, na regra 6).
- **Gerador:** `engenharia-reversa/remover_suspensao.py` (novo). Lê o `discipline.dat` no leiaute **de arquivo**
  (MAPA §1: registros N × 29; K regras, cada uma `i32 n` + entradas + `i16 p` + pendentes, K achado por ser o
  único que fecha o item; `i16 b` + suspensões; `i16 m` + índice por nação; `i16 h` + guardadas), confere a ida
  e volta do item, aplica o recurso aceito e regrava o contêiner com `renomear_no_save.regravar`, que foi
  estendido para item de **tamanho diferente** (o tamanho no índice muda e as posições seguintes andam, nos tipos
  3 e 4). A saída é conferida com `renomear_no_save.conferir` (todos os outros 141 itens iguais byte a byte; o
  `discipline.dat`, igual ao esperado). Regravar sem mudança dá o arquivo idêntico (MD5 do original)
  **[observado]**. Sem listar pessoa, o script lista as suspensões valendo, os pendentes e o índice.
- **Carga:** script descartável `conferir_suspensao.py` (scratchpad da sessão; molde do `conferir_lesao.py`):
  bancada da `prova_save.py`, jogo de testes (`jogo_testes\cm0102-1080.exe`), **modo original**, `-load <cópia>` +
  `CM_FERIAS=1`, janela oculta 960x540, sem cliques, dentro da trava do laboratório (esperou o outro teste
  liberar). Um vigia lê a memória a cada 0,5 s e registra cada mudança da cadeia do MAPA §4 para as duas
  pessoas, de `D+0xa`, `D+0x12`, do índice por nação, dos pendentes da regra 6 e da caixa de erro (`0xae24d0`).
  `game.cfg`, `hall_of_fame.bin`, `*.lng` e `*.tmp` guardados e devolvidos pela bancada.
- **`is_player_banned` 0x55d270:** não foi chamada de fora (a 0x7e85b0 que ela usa chama `update_ban_progress`
  0x556e90, que mexe no estado). A regra foi **reproduzida** com a memória lida **[inferência sobre o
  observado]**: primeiro a FIFA (regra 0) com a marca 1, que só vale se a suspensão tem o bit 1 de +0x13; depois a
  regra da competição com a marca 0. Valendo = hoje ≥ início e (por jogos: cumpridos + 1 ≤ jogos; por tempo: dias
  desde o início < dias). Na tabela, "FIFA" = o resultado do primeiro passo (o que vale em qualquer competição);
  "na regra 0" = o do segundo passo para uma competição da FIFA.

### Cópias

| Cópia | O que muda | `discipline.dat` | MD5 |
|---|---|---|---|
| (a) `a_controle.sav` | nada (cópia do original) | 1.602.112 | `33fda465193b5e8e6d7ace93ecd44559` |
| (b) `b_51013.sav` | 51013 removida | 1.602.104 | `5fdea32000ca00db90f255fcb37694a3` |
| (c) `c_29220.sav` | 29220 removida | 1.602.104 | `7416c30f23e5fd49efbe0521f5b226f8` |
| (d) `d_as_duas.sav` | as duas removidas | 1.602.096 | `0cad2b1fe6c29e541ca1117a08aea050` |

Os MD5 das cópias não mudaram com as cargas **[observado]**.

### Como as cargas terminaram

As 4 cargas abriram sem caixa, começaram em **205/2001** e andaram até a data de parada (215/2001), 23 a 29 s
cada; nenhuma caixa de erro **[observado]**. Impressão da carreira no fim: (a) e (b) `6efcb5697e0ca2a5` (iguais);
(c) `261b980f1a3f191a`; (d) `b670115a2578bbb4` **[observado]**.

## Resultado

Leituras da carga (205/2001) e de cada dia até 214/2001: as 10 leituras de cada cópia deram o mesmo estado das
duas pessoas **[observado]**.

| Cópia | Observado na memória (carga e dias 206..214) | Esperado | Bate? |
|---|---|---|---|
| (a) controle | `D+0xa` = 85, `D+0x12` = 10, índice com `(51013, 3)` e `(29220, 61)`. 51013: vaga 3 → regra 0 entrada 36, +6 = 33, suspensão 33 = `01 00 ffff 7100 d107 … ff ff ff 00 02 0100 04` (1 jogo, início 113/2001) → valendo na regra 0 = **sim**, FIFA = não (sem bit 1). 29220: entrada 0, +6 = 0, suspensão 0 = `ff 00 3001 0000 d107 … 03 0000 06` (304 dias desde 0/2001) → valendo = **sim**, FIFA = **sim** | as duas continuam suspensas | **Sim** |
| (b) 51013 | 51013: vaga 3 → regra 0 entrada 36 (registro mantido), amarelos 1 (mantido), **+6 = −1**, fora do índice; `D+0x12` = 9, índice = o do controle sem `(51013, 3)`, na ordem; `D+0xa` = 85. 29220 igual ao controle (suspensa) | 51013 sem suspensão; 29220 suspensa | **Sim** |
| (c) 29220 | 29220: entrada 0 mantida, **+6 = −1**, fora do índice; is_player_banned = **não** (FIFA e regra 0); `D+0x12` = 9. 51013 igual ao controle | 29220 sem suspensão; 51013 suspensa | **Sim** |
| (d) as duas | as duas com +6 = −1 e fora do índice; `D+0x12` = 8; `D+0xa` = 85 | as duas sem suspensão | **Sim** |

Observações:

1. **O efeito aparece na simulação** **[observado]**: (c) e (d) têm impressão diferente do controle; (b) dá a
   mesma. **[inferência]**: a 29220 tem o bit 1 (suspensa em qualquer competição) e, sem a suspensão, volta a ser
   escalável e muda os jogos; a 51013 só estava suspensa nas competições da FIFA (bit 1 desligado) e não houve
   jogo da FIFA para ela entre 205 e 215, então nada muda.
2. O jogo aceitou o `discipline.dat` menor (8 ou 16 bytes a menos) e o contêiner com as posições deslocadas, sem
   caixa **[observado]**.
3. A remoção é estável: o jogo não recria a suspensão nem o par do índice nos 10 dias; a vaga liberada da
   suspensão não aparece mais pela pessoa **[observado]**. `D+0xa` (a marca d'água) fica em 85, como no jogo
   **[observado]**. Se uma suspensão nova reaproveitou a vaga liberada nesses dias não foi lido **[não conferido]**.
4. O único pendente do save (46487, regra 6) entrou no dia 206 nas 4 cópias, igual **[observado]**.

## O modelo de bytes que o editor deve gravar

Sim: o modelo usado nas cópias é o que o editor deve gravar (o recurso aceito do jogo, `apply_appeal_result`
0x557000 → 0x7e91d0 + `release_ban` 0x556d00). No `discipline.dat`, para a pessoa `s`:

1. Para cada vaga k = 0..3 do registro `s × 29` com regra `rs ≠ −1`: a entrada `idx` (i32 em `5k+1`) da regra
   `rs`; só se `0 ≤ idx < n` e a entrada +0 == `s`.
2. Tirar da lista de pendentes **dessa regra** os nós de 0x70 com `+0x1c == s` (e o `i16 p` dela diminui).
3. Se a entrada +6 (`b`) ≠ −1: na suspensão `b`, para cada vaga j com `+0xc+j ≠ −1`, a entrada de `s` na regra
   `+0xc+j` (pela vaga j do registro) recebe **+6 = −1**; se j = 3 (FIFA), tirar do índice por nação o par
   `(s, nação)` (a nação é o staff +0x1a, ou +0x1e se o par for da segunda nacionalidade) e manter o índice
   ordenado por (nação, pessoa) (`i16 m` −1). A suspensão `b` fica **`+0 = 0xff`, `+2 = 0xffff`**; os outros 16
   bytes ficam; o `i16 b` (contagem) **não muda**.
4. Não mexer: o registro da pessoa (vagas, histórico +0x14/+0x18, +0x1c), os amarelos e contadores da entrada,
   o `ban_history.tmp`.

O que o editor precisa saber:

- **Leiaute em arquivo ≠ memória:** no arquivo, cada regra traz as entradas e os pendentes juntos, na ordem das
  regras existentes (só as não nulas, sem índice: a posição da regra se acha pelas vagas que apontam de volta, ou
  pela ordem crescente); as contagens são `i32 n` (entradas) e `i16` (pendentes, suspensões, índice, guardadas).
  O `D+0x12` da memória é o `i16 m` do arquivo.
- **O item muda de tamanho** (−8 bytes por par do índice, −0x70 por pendente): o editor tem de regravar o índice do
  contêiner (tamanho do item e posições dos seguintes), no tipo 3 e no tipo 4. O jogo aceitou **[observado]**.
- O índice por nação tem de continuar ordenado (o jogo usa `bsearch`; um par que falte quando o jogo procurar
  abre a caixa `discipline.cpp 0x47d`, mas o editor tira o par junto com a suspensão, então ninguém procura).
- **Não tratado aqui (fica para o editor recusar ou tratar à parte):** pessoa com vagas guardadas (empréstimo,
  registro +0x1c > 0): a entrada sai pelo nó guardado (MAPA §2, "Cuidado"); o script recusa.
- Remover só a suspensão de uma competição é outro caso (MAPA §2); aqui foi a remoção total.

A conferir pelo Lucas, com tela (fora do alcance desta leitura): o perfil do jogador sem "Suspended" e a
escalação de um jogo da 29220.
