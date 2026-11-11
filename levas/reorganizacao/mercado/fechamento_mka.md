*Versão em inglês: [fechamento_mka.en.md](fechamento_mka.en.md). O português é o original.*

# Fechamento do mapa do mercado, seção A §3: o preço pedido (0x4d1600)

`contract_manager_calc_asking_price` (0x4d1600..0x4d4257, contract_manager.cpp) desmontada inteira, instrução por
instrução (`desmonta.py jogo/cm0102.exe 0x4d1600 0x4d4260`, 3135 linhas), com os operandos de pilha do x87 seguidos à
mão e os bytes das formas de registrador conferidos (`DE F9` = `fdivp st(1)`: st1 ← st1/st0; `DE F1` = `fdivrp
st(1)`: st1 ← st0/st1; `D8 E9` = `fsubr st(1)`: st0 ← st1 − st0; `D8 E1` = `fsub st(1)`). As divisões por constante
mágica foram conferidas por força bruta (−40000..40000): `0x51eb851f sar 4` = /50, `0x10624dd3 sar 5` = /500, `sar 6`
= /1000, `0x66666667 sar 3` = /20, `0xc3a13de7 + add sar 9` = /670, `0xef9db22d sar 6` = /−1000. O item é
`levas/fila/mka_2` (compilado; a prova nas bancadas fica com a sessão principal).

**Tudo abaixo passa a [provado]** (desmontagem). Endereços entre colchetes. Pilha: depois do prólogo, vendedor
`[esp+0x90]` (ebp), comprador `+0x94`, jogador `+0x98`, motivos `+0x9c`, ajuste `+0xa0` (byte), bandeira `+0xa4`.

## Assinatura

- **Antes:** `thiscall(contract manager, seller, buyer, player, *reasons, adjust, flag)`, `ret 0x18`; 20 chamadores.
- **Agora:** igual, e: `reasons` é `ushort*` (marca com `or byte [edi+1], k` e limpa com `and word [edi], 0x800`);
  `adjust` é lido como byte [0x4d19c6]; `flag` como dword [0x4d3891]. Os 20 chamadores conferidos por `refs.py`
  (0x4d4429, 0x4d739c, 0x4d8705, 0x696ec5, 0x7847d4, 0x7b6df1, 0x8402fe, 0x840bfb, 0x8433a8, 0x843574, 0x843b17,
  0x84b0e0, 0x8c0adc, 0x8c1542, 0x8c8291, 0x8c8c6e, 0x8cd391, 0x8e6533, 0x8e6554, 0x8f4ba4). A rotina não sorteia nada
  por conta própria; o que escreve: só `*reasons` (e o `G_CONTRACT_LOOKUP_RESULT` dentro do `calc_player_value`).

## Passo 1: saídas cedo

- **Antes:** sem vendedor, sem contrato, contrato que não é do tipo 2 ou de saída Bosman: 0 [provado]; vendedor
  humano de férias sem o bit 4: o valor guardado [provado].
- **Agora:** confirmado [0x4d160d..0x4d16e7]: `get_contract_for_club(this, jogador, vendedor)` [0x4d1663]; `+0x35 & 0x3f
  == 2` [0x4d1676]; `+0x49 == 0` [0x4d1683]; clube humano com (1, 1) [0x4d1697], `HUMAN_MANAGER::is_on_holiday`
  [0x4d16b3], `get_holiday_option(0, técnico)` [0x4d16ce] ≠ 0 e `test [contrato+0x4e], 4` [0x4d16d7] → `staff::VALUE`
  (+0x52). Mais uma: **valor (passo 6) 0 → 0** [0x4d1a69].

## Passo 2: o fator da liga

- **Antes:** [inferido] "sem uso posterior visível; provavelmente dentro de um `_ftol`"; fórmula 1 − (20 − x)² × 0,2/60.
- **Agora:** L = 1 − (20 − y)² × 0,2 × (1/60) [0x4d17fc..0x4d1812], entre 0,1 e 1,1 [0x4d1818..0x4d1852], guardado
  em `[esp+0x64]`. x = `nation::STATE_OF_DEVELOPMENT` (+0x7e, i8), 3 fora de 1..3 [0x4d170c..0x4d1728]. **Com divisão:**
  x = (x + 1) × 0,5, × 4 (`PROFESSIONAL_STATUS` 2) ou × 7,5 (3) [0x4d172e..0x4d174e]; y = (reputação da divisão² × 25 +
  reputação do clube) / (x × 1000) [0x4d1754..0x4d177a]. **Sem divisão:** x × 2 (`fadd st0,st0`) ou × 7,5, sem o (x+1)/2,
  e y = (`nation::LEAGUE_STANDARD` (+0x85)² × 25 + reputação do clube) / (x × 1000) [0x4d17b3..0x4d17f2]. **Sem nação:
  y = 1.** **Uso:** L multiplica o prêmio [0x4d23c1] e o prêmio de reputação [0x4d2482] (não fica sem uso).

## Passo 3: a idade

- **Antes:** acima de 28, um fator de potência (0x945ff0).
- **Agora:** A = (1 − (idade − 28) × 0,1)^2,0 [0x4d1865..0x4d188a] (`_CIpow` com a base em st1, o expoente 2,0
  [0x968950] em st0), `[esp+0x74]`; multiplica o prêmio [0x4d23c5] e o prêmio de reputação [0x4d2486]. Idade = byte
  `staff+0x18`.

## Passo 4: felicidade e papel

- **Antes:** a falta até 75/50/40/25/0 para os papéis 1, 2 e 5, 3, 4, 6; papel 7 → felicidade −100. Ausente sem
  licença: a disciplina do técnico do vendedor zera os termos.
- **Agora:** papel = `SQUAD_STATUS` >> 4 (tabela de saltos 0x4d4258) [0x4d188e..0x4d1932]: 1 → max(75 − h, 0); 2 e 5 →
  max(50 − h, 0); 3 → max(40 − h, 0); 4 → max(25 − h, 0); **6 → max(−h, 0)** (não 0); 7 → h = −100 e falta 0; 0 e 8+ →
  falta 0. h = `contract::HAPPINESS` (+0x3d, i8; tratado em 16 bits). **Ausente** (`AWOL::is_player_awol(jogador, 1)`
  [0x4d1944]): falta 0, o **ajuste desligado** (bl = 0) e h ≤ −5 × `non_player::DISCIPLINE` (+0x14) do técnico do
  vendedor (h = −100 sem técnico) [0x4d194d..0x4d19c4]. Sem ausência, o ajuste vem do 5º argumento [0x4d19c6].

## Passo 5: a moral

- **Antes:** morale × 0,00746 (negativa) ou 0,00333; × 0,8333 **sem** queixas ou 1,2 **com**.
- **Agora (corrigido):** M = 1 + moral × 1/300 [0x968e60] (positiva) ou × 1/134 [0x969090] (zero ou negativa); **× 1,2
  sem queixas** (`GRIEVANCES` +0x45 = 0) [0x968978] e **× 0,8333 com queixas** [0x969088] [0x4d19cd..0x4d1a1a]. O mapa
  estava invertido.

## Passo 6: o valor (0x4d1a5a)

- **Antes:** o valor = `calc_player_value(jogador, vendedor, 0, habilidade vista pelo técnico (0x6e86d0), 0)`; 0 → 0.
- **Agora:** a "habilidade vista" é o **potencial** visto: `perceived_ability(técnico do vendedor (+0xcf), jogador,
  vendedor, 1, rep)` [0x4d1a43], com o tipo 1 (potencial; 2 seria a atual) e rep = `player_sheet::CURRENT_REPUTATION`
  (+0xb) / 2 + `POTENTIAL_ABILITY` (+7) × 25 em 16 bits (`imul dx, dx, 0x19`) [0x4d1a21..0x4d1a35]; cdecl, `add esp,
  0x14`. Depois `calc_player_value(this, jogador, vendedor, 0, visto, 0)` [0x4d1a5a] (o ecx é lido em `[esp+0x40]` antes
  do `add esp`, = o `this`); o visto só tem os 16 bits baixos válidos (`mov ax, ..` no retorno) e o `calc_player_value`
  só lê 16 bits. V em `[esp+0x14]` e `[esp+0x1c]`. **Ponto de extensão confirmado:** o preço é V + prêmio(V); escalar V
  escala tudo, e o "visto" é o único lugar em que o técnico do vendedor (o julgamento dele) entra.

## Passo 7: "contrato acabando" → **contratação recente**

- **Antes:** [inferido] com `adjust`, vendedor da IA, não listado: fator por papel (7,5; 6,0; 5,5; 3,5) menos os dias até
  o fim (121 a 365) / (365 / (f − 1)); acima de 2 marca 0x100.
- **Agora (corrigido):** os dias são **desde que o jogador chegou ao clube** (`staff::DATE_JOINED_CLUB` +0x3e/+0x40 do
  jogador, não o contrato): d = 365 × (ano atual − ano de chegada) + dia atual − dia de chegada [0x4d1aff..0x4d1b25],
  entre 121 e 365 (negativo → 121) [0x4d1b2b..0x4d1b43]. f por papel (tabela 0x4d4274, f32): 2 → 6,0 [0x969084]; 3 → 5,5
  [0x96907c]; 4 e 5 → 3,5 [0x969080]; os outros → 7,5 [0x969078]; papéis 7 e 8 fora [0x4d1abc..0x4d1ac8]. C = f − d /
  (365 / (f − 1)) [0x4d1b4b..0x4d1b63]: f até 4 meses depois da chegada, 1 depois de um ano. Acima de 2 marca 0x100
  [0x4d1b6b..0x4d1b7c]. Condições: ajuste ligado, vendedor da IA (`is_human_club(.., 0, 1)` = 0) e não listado nem
  pedindo saída (0x544520). C entra em Q, no teto D²C, no prêmio (× min(C², 25)) e no teto 3V × D × C.

## Passo 8: felicidade alta

- **Antes:** (falta/2 + felicidade) × 0,01167 + 1,4165, e acima de 49 num clube da IA × (… × 0,006 + 0,7); motivos 0x200
  (≥ 61) e 0x400 (21..60).
- **Agora:** H = (falta/2 + h) × 0,01167 + 1,4165 [0x4d1b80..0x4d1bae]; com h ≥ 50 e vendedor da IA: H = ((falta/2 +
  h)² / 50 × 0,006 + 0,7) × H [0x4d1bb4..0x4d1bf7]. Motivos: h > 60 → 0x200, senão h > 20 → 0x400 [0x4d1bfb..0x4d1c11]
  (só com `reasons`).

## Passo 9: as finanças do vendedor

- **Antes:** o saldo contra a tabela 0x9a7f48 por reputação / 500 (50 mil a 25 milhões); rico: saldo / tabela; pobre,
  amortecido; somado a felicidade × 0,002 e o nibble de observação² × 0,05; ≤ 1 → × 0,2125 + 0,7875, senão × 0,25 +
  0,75; no mínimo felicidade × 0,0025 + 0,5, no máximo 2,5.
- **Agora:** a tabela lida do exe (21 doubles): 50 mil, 100 mil, 200, 300, 400, 500, 750 mil, 1, 1,25, 1,5, 2, 4, 6, 8,
  10, 12,5, 15, 17,5, 20, 22,5, 25 milhões. B = `finance::BALANCE` (i64, 0 sem finanças) [0x4d1c15..0x4d1c3a]. B ≥ T:
  B / T [0x4d1cbc]; B < T: ((B − T)/T + x) / (x + 1), x = B × 1,25e-6 [0x969050] entre 0 e 10 [0x4d1c64..0x4d1cba].
  G = (isso + n² × 0,05) + h × 0,002 [0x4d1cbe..0x4d1d03], n = o nibble baixo de `staff_state::STATUS_FLAGS` (+0xb do
  registro de 79 bytes) [inferido: o nível de conhecimento/observação]. G > 1 → × 0,25 + 0,75; senão × 0,2125 + 0,7875
  [0x4d1d05..0x4d1d2c]. Piso h × 0,0025 + 0,5 (quando o piso vence, **não** há teto); senão teto 2,5 [0x4d1d30..0x4d1d6c].
  G também limita Q (≤ 6G).

## Passo 10: o comprador

- **Antes:** Celtic × Rangers: 2,3 + …; dérbi ou rivais, na mesma cidade ou não: 1,3 a 2; mesma divisão (fora de
  0x9cf764): 1 + … × 0,005 (motivos 8 e 0x10).
- **Agora:** D = 1 sem comprador [0x4d2002]. h' = h, ou ftol(h × 1,5) quando h > 20 sem queixas [0x4d1d83..0x4d1da7].
  - Celtic × Rangers (os dois clubes entre `G_KEY_CLUB_GLASGOW_CELTIC`/`RANGERS`): D = max(h' × 0,04 + 2,3, 2,3)
    [0x4d1fb5..0x4d1fe7];
  - confronto de liga (`comp_util_get_league_clash_type(comprador, vendedor)` ≠ 0 [0x4d1dd4]): mesma cidade
    (`stadium::CITY` +0x38 dos dois estádios): max(h'/30 + 2, 2); senão max(h' × 0,02 + 1,6, 1,3) [0x4d1de4..0x4d1e62];
  - rival do vendedor (`club::RIVAL1..3` = o comprador): mesma cidade max(h' × 0,02 + 1,6, 1,3); senão max(h' × 0,01 +
    1,3, 1,3) [0x4d1e88..0x4d1f9a];
  - esses três marcam **0x800** com D ≥ 1,3 [0x4d1e6e, 0x4d1fa2, 0x4d1fef];
  - mesma divisão (ponteiro igual, e o id da divisão do comprador ≠ `KEY_CLUB_COMP_A_LOWER_DIVISION`): max(h' × 0,005 +
    1, 1), marca **0x1000** com D > 1,3 [0x4d1ea8..0x4d1f1b].
  (Os motivos são 0x800 e 0x1000, não 8 e 0x10: o `or` é no byte alto.)

## Passo 11: a importância no elenco

- **Antes:** habilidade acima de reputação/50 × 17/20, idade < 25, papel < 7: 1 + … × 0,04 (motivo 0x20).
- **Agora:** k = reputação/50 + 1 (i16), limiar = k × 17 / 20 [0x4d2006..0x4d203d]. Com `reasons`, papel 5 marca **0x2000** e não
  calcula [0x4d2043..0x4d2052] (sem `reasons`, segue). Visto > limiar, idade < 25, papel < 7 [0x4d205b..0x4d2085]: com `reasons`, papel 6 marca 0x2000 e não
  calcula; senão S = 1 + ftol((visto − limiar) × k × 0,005) × 0,04, no mínimo 1 [0x4d20a2..0x4d20f5]; **papel 4 também
  multiplica H por S** [0x4d20f9..0x4d2107]; S > 1,6 marca 0x2000 [0x4d210f..0x4d211c].

## Passo 12: o preço

- **Antes:** o preço = valor + valor × (produto dos fatores, limitado 4,25..10 e 6 × o fator das finanças), entre −3 e 50
  milhões; mínimo 1000. Permissão de trabalho e listado: os motivos ficam só com 0x800. Clube rico com reputação > 4250 e
  idade > 18: ajustado pela faixa de riqueza.
- **Agora:**
  - Q = G × H × M × S × D × C (nessa ordem) [0x4d215c..0x4d2177], no máximo clamp(D² × C, **4,25 é o piso** [0x969008],
    10) [0x4d2124..0x4d218a] e no máximo 6G [0x4d219e..0x4d21b5]. Com D = C = 1 o teto é 4,25.
  - R = ftol((reputação atual + `HOME_REPUTATION`) × 0,3 + `WORLD_REPUTATION` × 0,4) [0x4d21bd..0x4d21fb]; e = (rep do
    clube + R − 4000) × 225 entre 150 mil e 2 milhões [0x4d2200..0x4d223e]; E = (((1 + (R − (1,2 rep − 2000)) × R ×
    5e-8) × u + 100) − u) × 0,01, u = h × 0,5 + 50 [0x4d2246..0x4d228c]; acima de 1, E⁵ (`_CIpow`) [0x4d22a3..0x4d22b2];
    E entre 1 e 3 [0x4d22b6..0x4d22f2].
  - prêmio bruto p = ftol(V × Q − V) [0x4d2307..0x4d231b]. **Negativo:** max(p, −0,4 V) [0x968fe8], e no mínimo −3
    milhões [0x4d232e..0x4d236e]. **Positivo:** min(p × C, ftol(e × Q)) × min(C², 25) × L × A [0x4d2373..0x4d23d2];
    metade se listado ou pedindo saída [0x4d23d8..0x4d2401]; até 50 milhões [0x4d2403].
  - preço = V + prêmio; com E > 1, + ftol(L × A × t), t = ftol(preço × E − preço), acima de 5 milhões 0,625 t + 1,875
    milhão, acima de 10 milhões 0,125 t + 8,75 milhões [0x4d2410..0x4d2495]; no mínimo 1000 [0x4d2499].
  - **Permissão de trabalho** (`player_needs_work_permit(jogador, vendedor, 0)` [0x4d24bb]): no máximo V (os motivos não
    mudam) [0x4d24c4..0x4d24d0]. **Listado ou pedindo saída:** os motivos ficam só com 0x800 e o preço ≤ `staff::VALUE` ×
    D [0x4d24d5..0x4d257a]. **Clube com time B ligado** (`club_get_linked_b_team(clube do jogador, &b, 1)` ≠ 0 e b = 0,
    ou seja o time A) e papel fora de 1, 5, 2, 3: o mesmo [0x4d250f..0x4d257a]. (O `b` não é inicializado no original;
    a rotina chamada o escreve sempre que devolve um clube [0x5412a7, 0x54187c → 0x541e42].)
  - **Piso do clube rico (corrigido):** vendedor da IA, motivo 0x800 já marcado, reputação > 4250, idade > 18, papel ≠
    7, faixa de riqueza ≠ −1 e −2: se o preço < (rep do vendedor + 25000) × 100, o preço vira (rep **do clube do
    jogador** (`staff::CLUB_JOB`) + 25000) × 100 [0x4d2585..0x4d261b]. Não é "ajuste pela faixa": a faixa só filtra.
  - **Novo:** no máximo max(3V, V + 5 milhões) × D × C [0x4d261f..0x4d267a].

## Passo 13: as queixas

- **Antes:** queixas (& 0x6b ou 0x100) com menos de um ano de contrato: o preço desce para o valor.
- **Agora:** preço > V e `GRIEVANCES` & 0x16b [0x4d267e..0x4d2692]: dias = dias até o fim do contrato
  (`CONTRACT_EXPIRES` +0x2d/+0x2f) [0x4d2698..0x4d26bf]; V' = V (≥ 365 dias) ou ftol(dias/365 × V); w = dias × 0,0027 −
  0,485 entre 0 e 1; p' = ftol((1 − w) V' + w × preço) [0x4d26e7..0x4d274d]; razão = (V' + p') / max(2 × B × D, 1) entre
  1 e 3; y = 1,375 − 0,375 razão; preço = ftol((1 − y) V' + y p') [0x4d2754..0x4d27e3]. Vale também com mais de um ano
  de contrato (aí só o saldo puxa).

## Passo 14: as grandes nações → as escadas A e B e a volta ao valor

- **Antes:** [inferido] grandes nações com reputação ≥ 4001: multiplicadores por papel (1,25; 1,1; 1,4; 1,15; 1,5;
  1,75; ×2 até +2M; ×5 até +5M) e 1,025..1,1 a mais na mesma divisão; as bandeiras do contrato e da pessoa.
- **Agora:** a entrada da lista do **vendedor** para o jogador (`find_club_shortlist_entry(vendedor, jogador, 0, 1)`
  [0x4d27ff]). "Grande nação" [0x4d2806..0x4d28e1]: reputação > 4000, nação em {Inglaterra, Itália, Espanha, França,
  Alemanha, Escócia, EUA, Japão, Holanda, Portugal}, (`LEAGUE_STANDARD` > 12 ou reputação ≥ 7000), finanças com faixa
  de riqueza ≠ −1/−2, o jogador não listado (0x5444a0) nem listado a pedido (0x544600).
  - **Escada A** (entrada que mira o jogador, `TARGET_STAFF_ID`, ou `staff::SQUAD_SELECTED_FOR` = 2 com competição
    continental) [0x4d28e7..0x4d2905]: o passo do jovem (abaixo de 25, papel fora de 6/7, visto × 50 > reputação):
    min(visto / (125 − moral/4) × preço, preço − |idade − 20| × 1.333.333 + 5 milhões) [0x4d290b..0x4d29b0]. Clube
    pequeno (rep ≤ 6750 e reputação da divisão ≤ 15 ou sem divisão): com queixas e moral ≤ 0, só com queixas 0x101 e
    mais de 425 dias de contrato (`Date::days_since`): papel 1 +375 mil/×1,25 (mesma divisão +50 mil/×1,05), papel 2
    +200 mil/×1,2 (+37,5 mil/×1,025); sem isso: papel 1 +750 mil/×1,75 (+100 mil/×1,1), 2 +400 mil/×1,4 (+75 mil/×1,075),
    3 +150 mil/×1,15 (+25 mil/×1,025) [0x4d29b4..0x4d2cf0]. Clube grande: com queixas e moral ≤ −50: (0x101 e > 425 dias)
    papel 1 min(+2,5M, ×2) (+1M/×2), 2 +1,5M/×1,5 (+750 mil/×1,75), 3 +1M/×1,25 (+400 mil/×1,25); senão, com comprador,
    > 225 dias, papel ≠ 7, profissionalismo e lealdade > 5, reputação atual > 6250, felicidade > −lealdade, 23 a 30 anos:
    +1M/×1,25 e, na mesma divisão com confronto de liga (`clash_type(vendedor, comprador)`), +400 mil/×1,25; sem queixas
    graves: papel 1 min(+5M, ×5) (+1M/×2), 2 +2,5M/×2,5 (+750 mil/×1,75), 3 min(+1,5M, ×2) (+400 mil/×1,4)
    [0x4d2cf5..0x4d3135]. Cada passo é min(preço + K, preço × f).
  - **Escada B** (entrada vazia, alvo −1) [0x4d313a..0x4d3824]: o passo do jovem abaixo de 22; clube pequeno sem (queixas
    e moral ≤ 0): papel 1 +250 mil/×1,25 (+50 mil/×1,05); papel 2, ou outro com felicidade > 40 − lealdade, +100 mil/×1,1
    (+25 mil/×1,025). Clube grande: com queixas e moral ≤ 0: (0x101, > 425 dias) papel 1 +500 mil/×1,25 (+250 mil/×1,1),
    2 +1M/×1,2 (+200 mil/×1,05), 3 +750 mil/×1,15 (+150 mil/×1,02); senão (> 225 dias, papel ≠ 7, lealdade +
    profissionalismo > 10, reputação atual > 5250, felicidade > 10 − lealdade, 23 a 30 anos) +750 mil/×1,15 (+150
    mil/×1,02); sem: papel 1 min(+2M, ×2) (+100 mil/×1,1), 2 +1,5M/×1,5 (+75 mil/×1,075), 3 +1M/×1,5 (+20 mil/×1,02).
  - Grande nação com entrada que mira outro: nada.
  - **Novo, importante:** **sem entrada** de lista e fora das condições de grande nação (ou grande nação sem entrada que
    não caia na escada A), preço > V: h < 60 → 0,67 V + 0,33 preço; senão t = 1,27 − h × 0,01 e (1 − t) preço + t V
    [0x4d3826..0x4d387e]. É o caso comum fora das grandes ligas: o preço volta dois terços do caminho ao valor.
  - As bandeiras do contrato (0x8676f0), a lealdade, o profissionalismo e a moral **não estão nas escadas**: estão no
    piso do jovem (passo 15 novo).

## Passo 15 (novo): o piso do jovem e os Jovens do técnico (0x4d3ccb)

- **Antes:** só citado no pedido ("o papel dos Jovens do técnico em 0x4d3ccb").
- **Agora:** com a bandeira (6º argumento), idade < 24, visto > 110, nação e papel ≠ 7 [0x4d3886..0x4d38cd]: si =
  visto; fora dos goleiros (`GOALKEEPER` = 20): fator de idade a = 2,6 − 0,07 idade; físico = ftol(((resistência × 1,2
  + força) × a + equipe × 0,8) + ritmo de trabalho × 1,2 + impulsão × 0,75); drible e finalização =
  `staff_get_player_attribute(11 / 12)` × a, até 20 [0x4d38d3..0x4d39d7]; por velocidade + aceleração: 40 → +15/+10/+8/
  +5/+3 por físico (85/75/65/50/35) e + (drible − 13)², + (finalização − 14)²; > 34 → +8/+5/+3/+1, (drible − 15)², 0,8 ×
  (finalização − 15)²; > 25 → +5/+3/+1, −3 com físico < 35, 0,8 × (drible − 15)², 0,5 × (finalização − 15)²; < 15 → −5/
  −8/−10 [0x4d39db..0x4d3ba2]. Com a entrada mirando o jogador: + rep/1000 [0x4d3ba5..0x4d3bd6]. Papel 1: + rep/500
  (abaixo de 19 anos no mínimo 121); 2 e 3: + rep/670 (19+) ou rep/500; 4 abaixo de 19: + rep/670 [0x4d3be1..0x4d3c59,
  0x4d3d61..0x4d3d83]; acima de 20 anos com habilidade atual < 45: − rep/1000 [0x4d3c5b..0x4d3c83]; visto > 120 e
  `club::TRAINING` > 10: + treino − 10 [0x4d3c85..0x4d3c98].
  **Os Jovens [0x4d3c9c..0x4d3cd0]:** com técnico (`club::MANAGER`) e ficha de não-jogador, num **clube da IA**
  (`is_human_club(vendedor, 1, 1)` = 0): si += `non_player::YOUNGSTERS` (+0x22) − 10 (em 16 bits). É a única entrada do
  técnico nesta rotina além do julgamento do visto. Depois, se o jogador chegou ao clube depois do técnico
  (`Date::operator>` das datas de chegada) e si < potencial: + metade da diferença [0x4d3cd4..0x4d3d15].
  si > 120 [0x4d3d17]: si ≤ clamp(rep × 0,0167 + 100, 150, 220) [0x4d3d21..0x4d3da9]; bônus (si > 150, entrada, rep ≥
  2500, padrão da liga > 10) = (min(4 si − 600, 200))² × 250 × rep / 6000 [0x4d3dab..0x4d3e0b]; Y = ftol((si² − 14400)
  × 250 × 0,6153846 + bônus) [0x4d3e13..0x4d3e3b]; padrão < 13 ou status 3: × r/5250 com r a reputação (× 0,75 no status
  3 abaixo de 5250; padrão < 5 acima de 4000: 4000 + (r − 4000)/5) [0x4d3e40..0x4d3ee4]; queixas: × 0,75 (moral < −50),
  × 0,8 (0x800000), × 0,95 (bandeira 8 do contrato), × 0,9 (1), × 0,9 (0x40) [0x4d3ee8..0x4d3f88]; lealdade < 6 e
  profissionalismo < 10: × 0,9 (moral < 0) e × 0,9 (comprador 750 acima); lealdade > 15 sem moral negativa: × 1,1
  [0x4d3f8c..0x4d4013]. O preço é no mínimo Y, com o motivo 0x2000 [0x4d4015..0x4d402a].

## Passo 15 antigo (teto por idade) → passo 16

- **Antes:** [provado no pseudo-C] 29 (26 goleiro): valor + 5M ... 38 a 45: o valor.
- **Agora (corrigido o goleiro):** [0x4d402e..0x4d4153], tabela 0x4d4288: goleiro (`GOALKEEPER` = 20) conta 3 anos
  a menos, então o teto dele começa aos **32** (não aos 26); 29 +5M,
  30 +4M, 31 +3,25M, 32 +2,5M, 33 +1,75M, 34 +1M, 35 +500 mil, 36 +200 mil, 37 +100 mil, 38..45 V; acima de 45, nada.

## Passo 16 antigo (Portugal, Coreia) → passo 17

- **Antes:** Portugal acima do valor: < 6M × (preço × 1,667e−7 × 0,6 + 0,3); senão × 0,9. Coreia: um ajuste.
- **Agora:** Portugal, preço > V: < 6 milhões, x = (preço × 1,667e−7 × 0,6 + 0,3) × preço; senão 0,9 × preço; o preço =
  max(V, x) [0x4d416b..0x4d41eb]. Coreia do Sul, preço > V e > 50000: (preço − 50000) × 0,15 + 50000 [0x4d41ed..0x4d421d].

## Passo 17 antigo (multa) → passo 18

- **Antes:** [provado] a multa ativa limita.
- **Agora:** confirmado [0x4d422a..0x4d423e]: `has_active_release_clause` e preço ≥ `RELEASE_FEE` (+0x21) → a multa.

## Pontos de extensão (revistos)

- O retorno de 0x4d1600 (20 chamadores) e `*reasons` (0x100 recente, 0x200/0x400 felicidade, 0x800 rival/dérbi,
  0x1000 mesma divisão, 0x2000 importante/jovem).
- A chamada do valor [0x4d1a5a]: V entra linear (V + V × (Q − 1) × ...).
- O piso 4,25 do teto [0x969008] e o 6G: controlam o prêmio inteiro (com D = C = 1, Q ≤ 4,25: o prêmio chega a 3,25 V).
- A volta ao valor sem entrada [0x968f80 = 0,67, 0x968f78 = 0,33]: a regra que mais pesa fora das grandes ligas.
- Os Jovens [0x4d3ccb]: `movsx cx, [eax+0x22]` e o `lea esi, [esi+ecx−0xa]` (o −10 é imediato).
- A tabela da riqueza 0x9a7f48 (21 doubles) e o teto por idade (imediatos 0x4d4067..0x4d4133).

## Defeitos do original

- **[provado]** No passo 4 com ausência sem licença, os 16 bits altos de `[esp+0x24]` ficam com lixo (o `movsx ax`
  sobre o ponteiro do técnico); só os 16 baixos são lidos: sem efeito.
- **[provado]** O piso do clube rico compara com a reputação do vendedor e grava com a do **clube do jogador**
  (`staff::CLUB_JOB`), que normalmente é o próprio vendedor; num emprestado ou num jogador de outro clube, as duas
  diferem [0x4d25ea / 0x4d2603].
- **[provado]** O `b` do time B não é inicializado; sem efeito porque o chamado escreve sempre que devolve um clube.
