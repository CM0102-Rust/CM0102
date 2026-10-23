# Rotinas migradas para Rust

Cada rotina entra por desvio (`rust/src/desvios.rs`) e só vira oficial depois de:
1. autoteste (`CM_AUTOTESTE=1`): entradas sintéticas logo que o jogo abre o vídeo, original × Rust, comparando resultado e memória tocada;
2. mutação de controle: um defeito proposital na versão Rust tem de ser acusado;
3. jogo real em modo comparação (`CM_COMPARAR=1`): carreira automática de férias (`docs/VERSAO_SEM_TELA.md`) ou partida do Lucas;
4. carreira inteira: `simular.py` com `--modo rust` e `--modo original` (mesma DLL, mesma semente) tem de dar a mesma impressão digital.

## Módulos

### Sorteio — `rust/src/aleatorio.rs` (oficial em 06/10/2026)

| Rotina | Original | Convenção |
|---|---|---|
| `aleatorio(n)` | 0x90cfc0 | cdecl |

Autoteste: 5.000.000 casos, 0 divergências. Mutações acusadas: +4 no ramo de n grande; volta da tabela. Jogo real: >100 milhões de chamadas, 0 divergências.

### Datas — `si/code/Date.cpp` → `rust/src/data.rs` (oficial em 06/10/2026)

| Rotina | Original | Convenção |
|---|---|---|
| `Data()` | 0x54a110 | thiscall |
| `Data(dia, ano)` | 0x549ef0 | thiscall |
| `Data(dia_do_mes, mes, ano, dia_da_semana)` | 0x549f70 | thiscall, ret 0x10 |
| `dia_da_semana()` | 0x54a3f0 | thiscall |
| `ajustar_semana(alvo, dest)` | 0x54a2d0 | thiscall |
| `operator=(src)` → cópia | 0x417c10 | thiscall, retorno por valor |
| `operator+(n dias)` | 0x54c5b0 | thiscall, retorno por valor, estática local |
| `operator-(n dias)` | 0x54c8e0 | thiscall, retorno por valor, estática local |
| `mais_semanas(n)` | 0x54c770 | thiscall, retorno por valor, estática local |

Autoteste: 9.446.717 casos (todos os anos de 16 bits nos construtores e no dia da semana), 0 divergências. Mutações acusadas: bissexto `% 200`; ano anterior bissexto somando 365. Jogo real: >10 milhões de chamadas, 0 divergências.

Esquisitices preservadas:
- `dia_da_semana` devolve −1 para 1600, dia 0, bissexto (resto negativo).
- `operator+`/`operator-` com n = −32768: a negação em 16 bits devolve −32768 e os dois se chamam até estourar a pilha (igual na original; o jogo nunca pede isso). Fica fora do autoteste.
- Cada operador inicializa a sua `Data` estática na primeira chamada e registra um destrutor vazio com `atexit`; a versão Rust faz o mesmo (por isso os autotestes só rodam depois de a CRT do jogo estar pronta).

Pendente no `Date.cpp`: as rotinas maiores (0x54a130, 0x54a470, 0x54a650, 0x54a950, 0x54ae10, 0x54b2f0, 0x54b550, 0x54ba20, 0x54bef0, 0x54c3b0), que formatam datas em texto.

## Escolha dos próximos alvos

`engenharia-reversa/alvos.py` (funções-folha pequenas e muito chamadas) e `docs/MAPA_MODULOS.md`.

### Motor de partidas, primeira leva — `match_eng.cpp`/`match_pl.cpp` → `rust/src/motor.rs` (oficial em 06/10/2026)

Escolhidas pela cobertura (`cobertura.py` + `simular.py --cobertura`, carreira com o motor ligado até 01/10/2001): as folhas mais chamadas. Só leem tabelas do motor.

| Rotina | Original | Convenção | Chamadas em 2 meses |
|---|---|---|---|
| `tabela_a(jogador)`: palavra da tabela do lado, pela posição 0..=10 | 0x6c38c0 | thiscall, ret 4, devolve `ax` | 870 milhões |
| `tabela_b(jogador)`: idem, outra tabela | 0x6c38f0 | thiscall, ret 4, `ax` | 18 milhões |
| `diferenca_lado(lado, v)` | 0x6c4180 | thiscall, ret 8, `al` | 9 milhões |
| `faixa(lado, b, c)` | 0x6cb960 | stdcall, ret 0xc, `al` | 8 milhões |
| `por_diferenca(a, b, c, d)`: tabela global 0xb67c30 | 0x6cb6c0 | thiscall, ret 0x10, `ax` | 6 milhões |
| `limite(lado, v)` | 0x6cb930 | stdcall, ret 8, `al` | 4 milhões |
| `condicao_jogador()` | 0x70dfa0 (match_pl.cpp) | thiscall, `al` | 21 milhões |

Retorno parcial: as originais só definem `al`/`ax`; os bits altos de `eax` são lixo (em 0x6c4180, `sub eax, ecx` mistura o `this`). A versão Rust reproduz a parte que o tipo define; a impressão digital da carreira pegaria um chamador que usasse o resto.

Autoteste: 14.000.000 casos (objeto do motor e jogador falsos, posições e lados fora das faixas), 0 divergências. Mutação acusada: `v > 8` → `v >= 8` em `limite` (divergência em `limite(0, 8)`). Jogo real, motor ligado, comparação: ~950 milhões de chamadas, 0 divergências. Carreira até 01/10/2001: Rust = original (597.180.435 sorteios, mesma impressão).

### Motor de partidas, segunda e terceira levas — `rust/src/motor.rs` (oficiais em 06/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `tabela_do_jogador()` | 0x717410 | thiscall | `tabela_a` do motor do jogador (0x19e), chamada pelo endereço do jogo |
| `diferenca_jogador(j)` | 0x6c4130 | thiscall, ret 4, `al` | 0 se `j` for o guardado em 0xf582 |
| `byte_do_lado(j)` | 0x6cfda0 | thiscall, ret 4, `al` | |
| `limite_jogador()` | 0x70df40 | thiscall, `al` | |
| `faixa_jogador(lado)` | 0x6f5b20 | thiscall, ret 4, `eax` inteiro | |
| `byte_da_posicao(i)` | 0x5b57c0 | thiscall, ret 4, `al` | `[this + 0x5b4 + i]` |
| `campo_pela_posicao()` | 0x6f4600 | thiscall, `al` | bloco do lado no motor: 0x91e2 + 6371 × lado |
| `pode_escolher(j, b, c)` | 0x6bce20 | thiscall, ret 0xc, `eax` inteiro | |
| `apontar(a, b, c, d)` | 0x6f4920 | thiscall, ret 0x10 | direção em graus (0x19a, 0..=359); **sorteia** (360, 120, 30, 30) e grava |

0x714d30 (`mov al, [ecx+0x27]; ret`, 4 bytes) não cabe um desvio; as versões Rust leem o campo direto.

Rotinas que sorteiam: na comparação, original e Rust partem do mesmo estado do gerador (`aleatorio::REGIOES_DO_ESTADO` no `lado_a_lado`), os sorteios da réplica não entram na contagem (`aleatorio::sem_contar`) e o jogo segue com o estado da original.

Autotestes: 37 e 51 milhões de casos, 0 divergências. Mutações acusadas (11): retorno sem motor; campo trocado; bytes de lado trocados; limiares `<`/`<=` e faixas; índice da tabela; bit da marca de posição; `0x2b >= 2`; ângulo fixo 180 → 181; `<` → `<=` no limiar do sorteio; ordem dos dois sorteios de ±30. Jogo real em comparação: >1 bilhão de chamadas, 0 divergências. Carreira com o motor ligado até 01/06/2002: Rust = original (2.340.778.192 sorteios), duas execuções Rust iguais.

### Motor de partidas, quarta leva: ponto flutuante — `rust/src/motor.rs` (oficial em 06/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `desgastar(x, codigo)` | 0x6f56c0 | thiscall, ret 8 | float do jogador em 0x4d, nunca abaixo de zero. Código −1: `x` × (12, 10, 8 ou 6 pela faixa do valor: acima de 9500, 8750, 7750) × 1/11. Outros: (200 − 8,5 × float de 0xd5) × fator do código (19 e 33: 0,3818; 21, 36 e 46: 0,5454; demais: 0,1636) |
| `virado_para(a, b)` | 0x6fa140 | thiscall, ret 8, `al` | direção (0x19a) contra o ângulo da tabela 0xb67c30, tolerância `_ftol`(0x10c × 2,25 + 45) |

Ponto flutuante: o x87 do jogo roda em precisão dupla (padrão do Windows), então cada operação arredonda como `f64` em SSE2. O que se reproduz é onde o original arredonda para `float`: `fstp`+`fld` compara o valor já arredondado, `fst` seguido de comparação compara o de 53 bits. As constantes são lidas do `.rdata` do jogo. `_ftol` (0x945640) = truncamento para `i64`.

Autoteste: 55.479.770 casos, 0 divergências. Mutações acusadas: faixa `>` → `>=`; fator do código 46; **produto calculado em `float` em vez de `double` (diferença de um bit)**; tolerância `<` → `<=`; volta `360 − d` → `359 − d`. Duas mutações não acusadas eram equivalentes (comparar com zero antes ou depois de arredondar só difere abaixo de 10⁻⁴⁵; `> 180` e `>= 180` dão o mesmo em 180) e foram trocadas. Jogo real: >1 bilhão de comparações, 0 divergências. Carreira com o motor ligado até 01/06/2002: Rust = original (2.340.778.192 sorteios).

### Motor de partidas, quinta leva — `rust/src/motor.rs` (oficial em 06/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `procurar_no_lado(lado, grupo, a3..a6, modo)` | 0x6cd330 | thiscall, ret 0x1c, `al` | percorre os jogadores do lado (quantidade em grupo[0x58 + lado], ponteiros em grupo + 44 × lado); modo 2 só conta; senão pergunta a 0x6f5c30 (pelo endereço do jogo, argumentos na ordem a4, a5, a3, a6) e ao primeiro sim zera 0x1a5/0x1a6 e devolve 1 |
| `alcanca(j, b, c)` | 0x6cb9b0 | stdcall, ret 0xc, `al` | faixas de coordenadas; no fim, tabela de distâncias (0xa15e44, 12 × 9, ≈ √(l² + c²), simétrica) contra 1,5 |
| `classificar_lance(codigo, a, b, c, d, forca, saida)` | 0x713e90 | thiscall, ret 0x1c, `al` | ignora 11 códigos (devolve 2); ângulo contra a tolerância `_ftol`(90 − (18 × forca − float de 0xe5)); fora dela grava (20 − atributo) × 12 em `saida` |

**Gravar e reproduzir** (rotinas que chamam outra com efeitos colaterais): 0x6cd330 chama 0x6f5c30 (3,9 KB, sorteia e altera jogadores). Na comparação, uma porta em 0x6f5c30 grava cada chamada da original (argumentos e resposta); a cópia Rust recebe a gravação no lugar da rotina e a sequência de chamadas também é comparada. A cópia tem de ver o estado que a original viu: os bytes que o laço lê (quantidade, 0x19 e 0x2b de cada jogador) entram nas regiões restauradas pelo `lado_a_lado`; a memória comparada é só a que a rotina escreve. No autoteste a porta responde de forma sintética (jogadores falsos derrubariam a rotina real).

Autoteste: 63.974.778 casos, 0 divergências. Mutações acusadas (7): jogador ativo `<= 1` → `< 1`; ordem dos argumentos repassados (a gravação acusa); escrita de 0x1a5; `x == 4` → `x == 5`; atributo 0x3b → 0x30; sinal na tolerância; um código da lista. Equivalentes (2): limiar `>` → `>=` 1,5 e índice linha/coluna da tabela (simétrica, e nas posições alcançáveis a coluna é ≥ 2, logo o valor é ≥ 2: a checagem final do original é sempre verdadeira). Jogo real: >1 bilhão de comparações, 0 divergências. Carreira com o motor ligado até 01/06/2002: Rust ×2 = original (2.356.606.166 sorteios).

### Motor de partidas, sexta leva: distâncias — `rust/src/motor.rs` (oficial em 06/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `distancia_ao_ponto(a, b)` | 0x6f09b0 | thiscall, ret 8, `float` em st0 | tabela de distâncias: linha \|y − b\|, coluna \|x − a\| |
| `distancia_ao_gol()` | 0x6fa1e0 | thiscall, `float` em st0 | até (4, 11) do lado 0, (4, 0) do outro |
| `mais_perto_do_gol(j)` | 0x6cb780 | thiscall, ret 4, `al` | 0: bloqueado (0x70dfa0, casa marcada na grade motor + 0x21b6 + lado oposto + 90 × (y + 12x)) ou adversário ativo a distância ≤ limite; 2: colega ativo a distância < limite; senão 1. Limite: distância dele ao gol + 0,5 (+ 1,0 fora da área), arredondado para `float` a cada passo. Blocos de 20 jogadores por lado em motor + 0x4796 + 8920 × lado, de 0x1be em 0x1be |

0x6f4650 (na área) chama 0x6f9f90, que sorteia e chama 0x6cbc00 (1,1 KB): fica para a leva da 0x6f9f90. Na comparação de 0x6cb780 as chamadas a ela passam por gravação e reprodução (porta em 0x6f4650; a gravação agora guarda qual rotina foi chamada).

Autoteste: 97.418.758 casos, 0 divergências. Mutações acusadas (6): eixos trocados na distância ao ponto; linha do gol 11 → 10; margem 0,5 → 1,0; casa sem o deslocamento do lado; bloco de adversários trocado; retorno 2 → 1. Equivalentes (2): `>` ↔ `>=` contra o limite (nas posições alcançáveis nenhum par da tabela difere exatamente em 0,5 ou 1,5). Jogo real: >1 bilhão de comparações, 0 divergências. Carreira com o motor ligado até 01/06/2002: Rust ×2 = original (2.356.606.166 sorteios).

### Motor de partidas, sétima leva: linha de passe — `rust/src/motor.rs` (oficial em 06/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `jogadores_na_linha(x1, y1, x2, y2)` | 0x6cbc00 | thiscall, ret 0x10, `al` | conta os jogadores nas casas da linha entre os dois pontos (grade 9 × 12, grupos em motor + 0x215e + 90 × (y + 12x)), percorrida ao estilo de Bresenham: vertical e horizontal do menor ao maior sem a última casa; diagonais com as pontas, a partir da de menor x (rasa) ou menor y (íngreme), erro subtraído antes da casa e somado depois. Cada casa por `procurar_no_lado` em modo 2 (só conta; o float repassado não é usado). O original reaproveita os próprios argumentos na pilha como variáveis |
| `consegue_jogar(a, b, calcular, forca, ajuste, saida)` | 0x6f9f90 | thiscall, ret 0x18, `al` | 1 se já virado para o alvo; tolerância 75° ou ftol(0x10b × 4 ou 5 + 10 × forca + 60 ou 40) − ftol((distância − ajuste) × 20 − 50), limitada a 30..135°; com `saida`, grava os jogadores na linha (0x6cbc00) e **sorteia**: 1 se 0x10b ≥ sorteio(c²) |
| `na_area()` | 0x6f4650 | thiscall, `al` | linha do gol e colunas 3..5; senão 0x6f9f90 virado para o meio do gol, sem cálculo nem saída (não sorteia) |

Com isso 0x6f4650 deixou de precisar da porta de gravação (era pura no caminho usado).

Autoteste: 130.887.068 casos, 0 divergências. Mutações acusadas (8): última casa incluída na vertical; `>=` → `>` no erro; passo invertido; limite 135 → 134; tolerância 75 → 74; `>=` → `>` no sorteio; sinal do ajuste; colunas 3..5 → 3..4. Equivalente (1): sinal do passo com `>= 0` → `> 0` (só difere com diferença vertical zero, que não chega à diagonal). Jogo real: >1 bilhão de comparações, 0 divergências. Carreira com o motor ligado até 01/06/2002: Rust ×2 = original (2.356.606.166 sorteios).

### Motor de partidas, oitava leva: posicionamento tático — `rust/src/motor.rs` (oficial em 06/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `marcas_da_posicao(posicao)` | 0x5b5270 | thiscall, ret 4, `u64` em edx:eax | instruções táticas da posição: `[this + 0x188b + 8 × posição]` |
| `papel_no_ataque()` | 0x716270 | thiscall, `al` | marcas do jogador (bloco 0x91e2 + 6371 × lado): 1 (0x20000); 0 se `tabela_a` = 1; 2 (0x40000000); 0 |
| `papel_na_defesa()` | 0x716310 | thiscall, `al` | 0 se `tabela_a` = 1; 3 (0x80000000); 4 (0x80000); 0 |
| `posicionar(j, saida_x, saida_y, _)` | 0x6c2be0 | thiscall, ret 0x10 | onde o jogador deve ficar. "Atacando": posse do time dele, o jogador guardado em 0xf57a é do lado dele, ou bola na entrada da área adversária com 0x8ea9 ≠ −1. Casa da tabela de formação do lado (`[motor + 0xf56e + 4 × lado]` + 12 × (9 × (atacando + 2 × posição) + bola_x) + bola_y; x no meio byte baixo, y no alto), bola limitada a 9 × 12. Com 0x8eb1 > 0: posição atual se o papel manda ficar, tudo limitado à grade. Senão (salvo `tabela_do_jogador` = 1): marcas do lado em 0x9766 (0x1000/0x10000/0x8000: um passo, com limites) e 0x4000 (acompanhar a linha da bola: ±2 parado, ou faixas encadeadas com 9/2 e ±2/±4) |

Autoteste: 172.911.600 casos, 0 divergências (tabelas de formação com margem, porque o índice fica negativo com posição < 0). Mutações acusadas (10): deslocamento das marcas; bit de papel no ataque; bit de papel na defesa; 0x8ea9 ≠ −1 → ≠ 0; limite de x da bola; direção do passo; +2 → +1 na linha; −2 → −1; "parado" do lado errado; saída x copiando y. Jogo real: >1 bilhão de comparações, 0 divergências. Carreira com o motor ligado até 01/06/2002: Rust ×2 = original (2.356.606.166 sorteios).

### Motor de partidas, nona leva: linhas defensivas — `rust/src/motor.rs` (oficial em 06/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `linhas_defensivas()` | 0x6c3aa0 | thiscall | impedimento: lado 1, maior y (0x103) entre os ativos partindo de 6, mais 1, em 0x8eaf; lado 0, menor y partindo de 5, menos 1, em 0x8eb0 (os campos lidos por `diferenca_lado` e `diferenca_jogador`). Ativo: posição >= 0 e `tabela_do_jogador` != 1. O original tem os dois laços de 20 jogadores desenrolados (1,7 KB) |

Autoteste: 581.108.689 comparações (cada chamada consulta 40 jogadores, também comparados), 0 divergências. Mutações acusadas (4): início 6 → 7; +1 removido; início 5 → 4; filtro de ativo. Jogo real: >1 bilhão de comparações, 0 divergências. Carreira com o motor ligado até 01/06/2002: Rust ×2 = original (2.356.606.166 sorteios).

### Motor de partidas, décima leva: atualização do jogador a cada lance — `rust/src/motor.rs` (oficial em 06/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `atualizar_jogador()` | 0x6f4aa0 | thiscall | 2,8 KB, 908 instruções. Contador 0x2b (decresce até 0); float 0x1a2 (× 0,85, piso 2 × (25 − atributo 0x5d da pessoa), teto 100); palavra 0x35 (disposição, ~4.000 a ~8.750): faixas pelos atributos e, pelo papel tático (bits 1, 2, 4, 8, 0x10, 0x20, 0x40 de `tabela_a`; "forte" = bits 0x880 da palavra inteira), sorteios de 1/75 a 1/1000, razões em ponto flutuante entre atributos e o estado do lado (0xf5bd/0xf5f3). Transcrita bloco a bloco, com os endereços do original nos comentários; os finais compartilhados viraram funções locais |

Autoteste: 588.775.364 casos, 0 divergências. Mutações acusadas (12): piso, teto, limiar 7350, `[0x1f] > 30`, faixa (b + 14) × 500, divisão por 11, razão `a4 > 4`, fator "forte", base (29 − v) × 250, `v == 1`, `−5v`, `+5c`. Quatro delas só foram acusadas depois de melhorar o gerador de casos (campos zerados com chance 1/2, faixa inteira de valores, palavra 0x35 exatamente nos limiares): eram lacunas de cobertura, não mutações equivalentes. Jogo real: >1 bilhão de comparações, 0 divergências. Carreira com o motor ligado até 01/06/2002: Rust ×2 = original (2.356.606.166 sorteios).

### Motor de partidas, 11ª e 12ª levas (de baixo para cima) — `rust/src/motor.rs` (oficiais em 06/10/2026)

A partir daqui a ordem é de baixo para cima: primeiro as rotinas que só chamam rotinas já portadas (as integradoras grandes, como 0x7109f0 com 13 KB e 46 chamadas, ficam para quando o que está embaixo estiver pronto, para a comparação ser segura).

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `sortear_momento()` | 0x6f44a0 | thiscall | atributos do momento (0x104 a 0x10d), vários "melhor de dois sorteios"; 15 sorteios na ordem do original |
| `manter_posicao_um(lado)` | 0x6ce110 | thiscall, ret 4, ponteiro | jogador de posição 1 de cada lado (0xf59e, 0xf5a2): zera se perdido, procura quem tem `tabela` 1, senão promove o primeiro ativo trocando com a reserva (0x1a, posição −2, 0x20 = −1) |
| `caminho_livre(j, ignorar_casa)` | 0x6bd5b0 | thiscall, ret 8, `al` | 0 se a casa dele está marcada ou se algum adversário ativo à frente está na mesma coluna |
| `espaco_a_frente(j)` | 0x6cbad0 | thiscall, ret 4, `al` | casa à frente livre (diagonais primeiro) na grade do lado oposto |
| `fora_do_meio(v)` | 0x6cb910 | stdcall, ret 4, `al` | 1 fora de 2..=6 |
| `alem_da_linha(y, lado, sem_guardado)` | 0x6cb700 | thiscall, ret 0xc, `al` | quantos ativos do lado estão além da linha y |
| `distancia_entre(a, b, c, d)` | 0x6cc4e0 | stdcall, ret 0x10, `float` | tabela de distâncias, 1000 fora |
| leitores e gravadores do estado da partida | 0x700320..0x700410 | thiscall | bola (0x8ea7/0x8ea8), 0x8ea9, 0x8eaa, 0x8eb2, 0x8ed8, 0x8f7a |
| `registrar_posse(lado)` | 0x7003b0 | thiscall, ret 4 | 0x8eab = lado; exceto −1, +1 no contador do lado (0x8d46 + lado) e −10 no do outro, se positivo |
| `na_lista(alvo)` | 0x714c60 | thiscall, ret 4, `al` | ponteiro na lista 0x1ae (quantidade em 0x101) |
| `guardar_extremos()` | 0x6cd660 | thiscall | jogadores de menor e maior y de cada lado (0xf58e, 0xf596, 0xf592, 0xf59a); sem candidato o valor anterior fica |

11ª leva (7 rotinas): autoteste 667.013.079 casos, 10 mutações acusadas, >1 bilhão de comparações, carreira Rust = original. 12ª leva (14 rotinas): autoteste 695.082.635 casos, 11 mutações acusadas, carreira Rust = original.

Correções do autoteste nestas levas: o jogador falso (0x1a4 bytes) e o registro de pessoa falso (0x46 bytes) eram menores do que as rotinas leem e escrevem (0x1ce e 0x5d); a escrita fora do vetor corrompia a heap do processo e derrubava o autoteste de vez em quando (queda dentro do `ntdll`, registrada pelo tratador de exceções). Agora 0x200 e 0x80.

### Motor de partidas, 13ª leva — `rust/src/motor.rs` (oficial em 06/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `contar_ativos(lado)` | 0x6cbbb0 | thiscall, ret 4, `al` | ativos do lado, guardado em motor[4 + lado] |
| `codigo_especial(codigo)` | 0x6cb650 | stdcall, ret 4, `al` | 1 para os códigos 37, 42..=50, 96 e 97 (tabelas do switch lidas do exe) |
| `atras_do_extremo(j)` | 0x6cd5f0 | thiscall, ret 4, `al` | lado 0: y <= y do menor (0xf58e, sem ele 12); senão y >= y do maior (0xf592, sem ele −1) |
| `sem_partida()` | 0x6d4280 | thiscall, `al` | `*[0x4792] == −2` |
| `pressao_na_area(lado)` | 0x6bd6b0 | thiscall, ret 4 | soma dos adversários nas linhas perto do gol (pesos 6..1, 16 bits) > sorteio(15) + 30 → motor[6 + lado] = 1 |
| `mais_proximo_do_ponto(j, a, b)` | 0x6c3010 | thiscall, ret 0xc, `al` | o mais próximo entre os colegas disponíveis (empate conta contra quem tem `tabela` 1) |
| `pressao_no_ponto(lado, a, b, todos)` | 0x6c3a00 | thiscall, ret 0x10, `al` | +3 na própria casa, +1 a distância <= 1,5 |
| `menor_distancia(lado, a, b, todos)` | 0x6c3950 | thiscall, ret 0x10, `float` | partindo de 5000; 0 se alguém está na casa; filtro pelo bit 0 de `tabela` |

Autoteste: 1.073.998.479 casos, 0 divergências. Mutações acusadas (10), duas delas só depois de deixar a grade de contagem quase vazia (com contagens altas a soma da pressão passava sempre do limiar).

**Correção importante do autoteste:** o motor falso tinha 0xf590 bytes, mas desde a décima leva o teste grava campos até 0xf5f5 (estado do lado, posição 1). As escritas além do fim corrompiam a heap e derrubavam o autoteste de vez em quando, conforme o jeito de lançar o jogo (o ambiente muda a disposição da heap). Agora 0x10000. Além disso, ponteiros fictícios (0xdead…) nos extremos eram lidos por uma rotina nova; trocados por jogadores falsos válidos. Regra para as próximas levas: ao usar um campo novo nos objetos falsos, conferir o tamanho do buffer.

Jogo real: >1 bilhão de comparações, 0 divergências. Carreira com o motor ligado até 01/06/2002: Rust ×2 = original (2.356.606.166 sorteios).

### Motor de partidas, 14ª leva — `rust/src/motor.rs` (oficial em 06/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `repartir_a(lado, delta)` | 0x6c06b0 | thiscall, ret 8 | soma à disposição (palavra 0x35) de cada ativo `ftol(delta × fator)` pelo primeiro bit do papel; papel 1 fica de fora |
| `repartir_b(lado, delta)` | 0x6c07a0 | thiscall, ret 8 | idem, outra tabela de fatores; papel 4 recebe o delta inteiro |
| `repartir_c(lado, delta)` | 0x6c0a70 | thiscall, ret 8 | idem, terceira tabela de fatores |
| `escolher_por_papel(lado)` | 0x6c2750 | thiscall, ret 4, `al` | maior sorteio(25) × fator do papel (0,1/0,2/0,5/0,75/1,0 ou 1,05 com bits 0x700); −1 sem candidato |

Curiosidade do original: `escolher_por_papel` guarda o fator no lugar do argumento na pilha. Jogador sem bit de papel herda o fator anterior, e o primeiro herda os bits da palavra inteira que o chamador empilhou (por isso o argumento é tratado como u32).

Autoteste: 1.524.875.300 casos, 0 divergências; 9 mutações acusadas. Jogo real: >1 bilhão de comparações, 0 divergências. Carreira com o motor ligado até 01/06/2002: Rust ×2 = original (2.356.606.166 sorteios, impressão 7866d02de7578a90).

### Motor de partidas, 15ª a 17ª levas — `rust/src/motor.rs` (oficiais em 06/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `direcao()` | 0x6f4a30 | thiscall, `ax` | palavra 0x19a do jogador |
| `zerar_guardado(lado)` | 0x6ce0f0 | thiscall, ret 4 | zera 0xf59e (lado 0) ou 0xf5a2 |
| `x_fora_do_meio()` | 0x70df20 | thiscall, `al` | x (0x102) fora de 2..=6 |
| `e_o_guardado()` | 0x7103c0 | thiscall, `al` | jogador == motor[0xf582] |
| `lado_do_lance()` | 0x6bba50 | thiscall, `al` | 0xff sem lance; senão jogador do lance é do lado 0 |
| `jogador_no_bloco(lado, k)` | 0x6c2fe0 | thiscall, ret 8, ponteiro | 0x4796 + 0x1be × (20 × lado + k) |
| `tabela_a_1a(j)` | 0x6c3920 | thiscall, ret 4, `ax` | `tabela_a` pela posição em 0x1a |
| `momento_marcado()` | 0x6d4250 | thiscall, `eax` | 0x8ed4 fora de 0x1ef/0x3de e bit 0x80 do byte 0x38 da partida |
| `estado_do_lado()` / `estado_do_adversario()` | 0x6f4a40 / 0x6f4a70 | thiscall, `al` | 0xf5bd / 0xf5f3 conforme o lado |
| `gravar_19c(v)` | 0x70a890 | thiscall, ret 4 | limitado a −200..=200 |
| `codigo_de_bola_parada(c)` | 0x70a8c0 | ret 4, `al` | 12, 13, 21, 37, 46, 59 |
| `perto_do_gol()` | 0x70df70 | thiscall, `al` | lado 1: y < 4; lado 0: y > 7 |
| `limpar_lance()` | 0x6bba10 | thiscall | 0x475a..=0x4769 |
| `somar_disposicao(lado, delta)` | 0x6cf710 | thiscall, ret 8 | delta inteiro em 0x35 de todos os ativos |
| `repartir_d/e/f(lado, delta)` | 0x6c0890 / 0x6c0980 / 0x6c0b70 | thiscall, ret 8 | mais três tabelas de fatores; em `f`, papel 1 recebe o dobro (`fadd st0, st0`) |
| `registrar_lance(...)` | 0x6bb9a0 | thiscall, ret 0x24 | 9 argumentos, só se 0x4761 estiver vazio |
| `remover_da_lista(p)` | 0x714ca0 | thiscall, ret 4, `al` | troca pelo último; devolve a nova quantidade |
| `entrar_em_campo(j)` | 0x6cdd00 | thiscall, ret 4, `al` | põe em (8, 6) e na lista do lado (0x453a, 11 por lado) |
| `em_perigo()` | 0x70d630 | thiscall, `al` | tabela 12 × 9 de floats (0xa15e44) ≤ 1,0, senão `atras_do_extremo` |
| `conferir_lance()` | 0x6bba80 | thiscall, `al` | 1/2 conforme lado e área do lance; sorteio(200) decide limpar |
| `caminho_livre_ate(lado, x, y)` | 0x6c41b0 | thiscall, ret 0xc, `al` | adversário na casa ou na coluna entre o ponto e o gol |
| `alcanca_o_ponto(x, y)` | 0x713d90 | thiscall, ret 8, `al` | `por_diferenca` contra 0x10c + 10, e `distancia_entre` contra a tabela 12 × 9 |

Achado da 16ª: em `entrar_em_campo`, com lado fora de 0/1 a vaga gravada cai sobre a própria contagem (0x4592), e o original relê a contagem depois de gravar; a primeira versão em Rust guardava o valor lido antes e divergia no autoteste. Corrigido relendo como o original.

Autotestes: 1,56 / 2,23 / 2,44 bilhões de casos, 0 divergências. Mutações: 12/12, 11/13 e 12/13 acusadas; as que passaram são equivalentes (lista vazia devolve 0 pelos dois caminhos; a tabela não tem NaN; com y igual, a coluna só coincide na própria casa, já tratada antes). Uma lacuna (igualdade exata no limiar de `alcanca_o_ponto`) foi fechada no gerador. Jogo real: >1 bilhão de comparações, 0 divergências. Carreira com o motor ligado até 01/06/2002: Rust ×2 = original (impressão 7866d02de7578a90).

### Motor de partidas, 18ª a 20ª levas — `rust/src/motor.rs` (oficiais em 07/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `escolher_destaque()` | 0x6d0f80 | thiscall | entre os (byte 0x4d da partida & 0xf) + 11 de cada lado: pontuação 16 bits (disposição + 125 × (0x10 + 2 × 0x0c) + 50 × 0x03 + 35 × 0x06) e ftol(disposição × c); guarda o id da ficha em 0x477a |
| `escolher_por_papel_b(lado)` | 0x6c2880 | thiscall, ret 4, `al` | fatores 1,2/1,2/0,65/0,5/0,35/0,25 ou 0,15, ajustados por três atributos da pessoa; o fator vive no slot do argumento e passa adiante |
| `casa_da_formacao(j, &x, &y)` | 0x6c2a40 | thiscall, ret 0xc | nibbles de um byte da formação (216 bytes por posição); campo conforme 0x8eae e o lado |
| `calcular_percentuais(i)` | 0x6ce9c0 | thiscall, ret 4 | índices i, 3 e 4; ftol(a / (a + b) × 100) e o complemento; 50/50 sem total positivo |
| `escolher_por_papel_c(lado, modo)` | 0x6c2050 | thiscall, ret 8, `al` | sorteio sobre atributos float do jogador (quatro fórmulas, duas sem o fator), pessoa (palavra 5, sorteio contra 0x21), desconto de sorteio(5) acima de um limite |
| `aplicar_fator(f)` | 0x716450 | thiscall, ret 4 | condição 0x1a2 × f, com mínimo 2 × (25 − 0x5d da ficha) e teto 100 |
| `valor_pela_condicao(alvo)` | 0x7165b0 | thiscall, ret 4, `eax` | duas fórmulas conforme `alvo` == 0x1a6 |
| `aplicar_ao_time(j, f)` / `aplicar_ao_lado(lado, f)` | 0x6bb8d0 / 0x6bb950 | thiscall, ret 8 | fator ao jogador e derivados aos colegas com ficha |
| `escolher_pela_condicao(lado, modo)` | 0x6c2310 | thiscall, ret 8, `al` | sorteio(valor + 5 ou + 25) × (0,2 com papel 1, senão 1) |

Mutações: 14/14, 12/12 e 10/12 acusadas (as duas restantes são equivalentes: o teto é 100,0, e o arredondamento para f32 só mudaria um valor que é regravado igual). O script de mutações agora para na primeira divergência. Jogo real: 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 21ª a 23ª levas — `rust/src/motor.rs` (oficiais em 07/10/2026)

Rotinas sem arquivo no mapa (`docs/mapa_funcoes.csv` com "-"), mas chamadas pelo motor; a lista de cobertura só via as de `match_*.cpp`. Achadas pelas chamadas das rotinas cobertas.

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `ler_1bc()` | 0x717400 | thiscall, `ax` | palavra 0x1bc |
| `somar_2b(v)` / `gravar_2b(v)` | 0x7160d0 / 0x7160e0 | thiscall, ret 4 | byte 0x2b |
| `tabela_a_1a_do_jogador()` / `tabela_b_do_jogador()` / `diferenca_do_jogador()` | 0x717430 / 0x717450 / 0x715f30 | thiscall | pelo motor do jogador; sem motor 0 (ou 0xff) |
| `marcas_do_jogador_rot()` | 0x7163c0 | thiscall, `edx:eax` | 0x5b5270 da posição |
| `marca_800000()` | 0x7163f0 | thiscall, `al` | bit 0x1000 do bloco: byte 6 + lado; senão bit 0x800000 das marcas |
| `perto_do_proprio_gol()` | 0x70def0 | thiscall, `al` | lado 0: y < 3; lado 1: y > 8 |
| `papel_880()` | 0x717c70 | thiscall, `al` | 0x880 em `tabela_a` e `tabela_b` |
| `soltar_bola()` | 0x6f0970 | thiscall | zera o dono e o estado do lance |
| `jogador_pela_ficha(f)` | 0x6e1cd0 | thiscall, ret 4 | busca nos 20 de [this + 0x23] |
| `byte_do_codigo(c)` | 0x6d5cd0 | ret 4, `eax` | tabela de 18 bytes por código em 0xb68788 (área não inicializada: o autoteste grava valores temporários) |
| `nota_ajustada(base)` | 0x715f50 | thiscall, ret 4, `eax` | três parcelas (base ou valor por lado), sorteios contra atributos da pessoa, / 3 |
| `ajustar_17(delta)` | 0x7172d0 | thiscall, ret 4, `al` | byte 0x17 ± delta com sorteios perto de 10, piso pela disposição, faixa 1..=20 |
| `pegar_bola()` | 0x6f0630 | thiscall | o jogador pega a bola: solta a do dono anterior, credita o passe (contador e disposição por zona), registra posse e bola na casa dele |

**Limite do desvio:** 0x714d00 ficou em Original. O prólogo curto (4 bytes) é seguido de um `jne` relativo, e o trampolim copiaria o salto fora do lugar (no autoteste, saltou para o lixo). `prologo_ok.py` (rascunho da sessão) confere todos os prólogos antes de cada leva: instruções inteiras e sem salto ou chamada relativa.

Mutações: 12/12 (depois de fechar duas lacunas: a tabela zerada e o byte 0x1a7 sempre zero), 13/13 e 11/12 (a restante, soltar a bola do próprio jogador antes de ele pegá-la, é equivalente). Jogo real: 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 24ª e 25ª levas — arbitragem e cartões (oficiais em 07/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `iniciar_rigor(motor, pessoa)` | 0x6e9470 | thiscall, ret 8 | objeto do árbitro (motor, pessoa, valor 5000 sem pessoa ou 7750..5250 por sorteios contra a palavra 0x1e) |
| `lance_valendo()` | 0x6bab90 | thiscall, `al` | o lance registrado continua? a bola passando a linha do lance o encerra (limpa) |
| `decidir_falta(j, codigo, &tipo, &valor)` | 0x6e9540 | thiscall, ret 0x10, `al` | falta marcada? papel 1 na área, a pessoa do árbitro, sorteios por código; grava tipo 4 e sorteio(0x14) + 1 |
| `punir_falta(j, &tipo, x, sev, flag, &peso)` | 0x6e9630 | thiscall, ret 0x18 | vermelho direto, amarelo (0x24/0x25/0x26) ou −25 na disposição; com flag, sorteio(3000) |
| `punir_lance(j, outro, &tipo, x, sev, flag, &peso, expulsar)` | 0x6ea4e0 | thiscall, ret 0x20 | variante: inativo (peso 10), tipo 7, vermelho tipo 5 ou 4, contadores 0x14 e 0x15 |

O desconto do amarelo é ftol(c × −225) com c = 1, ou 45 / (0x8ed0 / 11) acima de 45: contínuo no limiar (45/45 = 1), por isso a mutação do limiar é equivalente. Outras equivalentes: com lados 0 e 1, "dono do outro lado" escrito de duas formas.

Mutações: 13/14 e 16/17 (as restantes, equivalentes). Jogo real: 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

Ferramenta nova: `engenharia-reversa/prologos.py` confere os prólogos de `motor.rs` (instruções inteiras, sem salto ou chamada relativa).

### Motor de partidas, 26ª a 28ª levas — impedimento e cartão pela gravidade (oficiais em 07/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `motor_decidir_falta` / `motor_punir_lance` / `motor_punir_falta` | 0x6cfca0 / 0x6cfcd0 / 0x6cfd10 | thiscall | invólucros: a rotina do árbitro do motor (this + 0xf5b2) |
| `procurar_impedimento(lado, linha, y, passador)` | 0x6ea760 | thiscall, ret 0x10, `eax` | match_official.cpp: atacante em impedimento por distância à última linha (0x8eaf/0x8eb0) e sorteios contra o byte 0x107 (um deles com 0x107³ + 25); ajusta o valor do árbitro |
| `motor_procurar_impedimento` | 0x6cfd70 | thiscall, ret 0x10 | invólucro |
| `cartao_pela_gravidade(j, &tipo, gravidade, flag)` | 0x6eab60 | thiscall, ret 0x10 | sorteio(0x14) ajustado pelo árbitro e pela gravidade (1..6); segundo amarelo, vermelho ou amarelo; o código do registro fica no slot do argumento `j` (sem gravidade conhecida, o byte baixo do ponteiro) |
| `motor_cartao_pela_gravidade` | 0x6cfd40 | thiscall, ret 0x10 | invólucro |

Os invólucros comparam-se pelas mesmas regiões da rotina interna (a interna, desviada, compara-se dentro deles).

Mutações: 3/3, 13/13 e 13/13. Jogo real: 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 29ª a 31ª levas (oficiais em 07/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `escolher_por_papel_e(lado, modo)` | 0x6c23f0 | thiscall, ret 8, `al` | quinta escolha ponderada (855 bytes): fatores por papel com variantes 0x880, ajustes por cinco atributos da pessoa (um deles com constante double), três fórmulas de sorteio; 1 em 5 descarta a escolha |
| `estatistica_de_posse(lado, janela)` | 0x6cf750 | thiscall, ret 8 | posse por zona da bola e período; com `janela`, totais dos últimos 55 instantes (janela circular em 0x8e9b) |
| `resumo_da_partida(momento)` | 0x6ce460 | thiscall, ret 4 | ritmo sorteado (0xf628/0xf62c/0xf630), nota de cada jogador ((disposição + 500) / 1000 em 1..=10, em 0x1b) e somas de atributos por lado; sempre recalcula os percentuais |

Equivalentes por construção: multiplicar por 0,75 em f32 ou em double (0,75 é exato); nota 10 limitada a 10 por `>` ou `>=`. Duas mutações da 30ª só passavam porque o teste não tinha lado −2 nem bytes 0xff na janela: entradas ampliadas, ambas acusadas.

Mutações: 10/11, 10/10 e 10/11. Jogo real: 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 32ª e 33ª levas (oficiais em 07/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `mais_perto_no_lado(lado, a, b, todos)` | 0x6c30e0 | thiscall, ret 0x10, `eax` | laço de 20 desenrolado no original: o mais perto de (a, b) pela tabela, distância zero devolve na hora |
| `defesa_do_goleiro(a, b, a3, codigo, &saida, &rebote)` | 0x700420 | thiscall, ret 0x18, `ax` | valor da defesa: base pela palavra 0x168/0x16a + 220, sorteio(0x3b) contra sorteio(285) escolhe a fórmula (0,7/0,3), ajustes por código (3/5, 26, 27, 0x33), por área e por distância à bola; rebote quando o sorteio final perde |

Equivalentes nas entradas possíveis: a tabela de distâncias não tem o valor 2,1 do limiar (`>` e `>=` iguais) nem valores negativos (a primeira distância zero vence de qualquer jeito).

Mutações: 14/16 (as duas restantes, equivalentes). Jogo real: 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 34ª leva — grade de ocupação (oficial em 07/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `mover_para(x, y)` | 0x6f46b0 | thiscall, ret 8 | move o jogador na grade (90 bytes por casa a partir de 0x215e; por lado, lista de 11 ponteiros e a quantidade em 0x58 + lado): o último da lista ocupa a vaga dele (0x28), ele entra no fim da lista da casa nova; atualiza a bola se ele for o dono |

Estados impossíveis do original: lista vazia ao tirar (o original mostra uma caixa de erro e volta; em Rust, só volta) e mais de 11 ao pôr (o original mostra o erro e encerra o jogo; em Rust, encerra igual). O autoteste monta grades consistentes para nunca cair neles.

Mutações: 6/6 e a sétima (tamanho da linha da grade) derrubou o jogo pela saída do original. Jogo real: 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 35ª e 36ª levas — escolha da casa e domínio do lance (oficiais em 07/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `escolher_casa(j, &x, &y, flags)` | 0x6bce80 | thiscall, ret 0x10, `al` | o "cérebro" do movimento: pontua as 9 casas em volta do jogador (adversários na casa ou o mais perto dela, recuo, linha de impedimento com a flag 4, faixa central, alvo anterior com a flag 2) partindo de 7,5; empate fica com a última |
| `dominio_do_lance(&codigo, &a, &b)` | 0x70d690 | thiscall, ret 0xc, `ax` | valor do domínio por código (duas tabelas de desvio): palavra do jogador menos sorteios pela condição física, pela direção da bola sem dono, pelo valor do dono e 20 × (0x8eaa − 1)³; fora da área pode perder na hora (−20); o goleiro na área troca o código |

Equivalentes: com a flag 2, a casa do próprio jogador já sai por `virado_para` (que devolve 0 para (0, 0)); diferença de 180 graus dá 180 por `>` ou `>=`. Lacunas fechadas: as tabelas de papel não tinham os bits 0x10/0x20 com 0x80/0x800 e o outro dono tinha 0x19c sempre zerado. O autoteste completo já passa de 600 s (4,7 bilhões de casos); o limite do script de mutações subiu para 1500 s.

Mutações: 21/22 e 14/15 relevantes (as restantes, equivalentes). Jogo real: 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 37ª leva — traçado do lance, com chamadas internas gravadas (oficial em 07/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `tracar_lance(x0, y0, x1, y1, codigo, a6, a7)` | 0x6ccb60 | thiscall, ret 0x1c, `al` | percorre as casas da linha (eixo maior com erro acumulado, quatro sentidos) e em cada uma pergunta a `procurar_no_lado` (0x6cd330) por um jogador de um lado sorteado e do outro, com a distância desde a origem; tipo do lance pelo código (switch) |

**Gravar e reproduzir chamadas internas.** 0x6cd330 tem efeitos nos jogadores (pela 0x6f5c30). Na comparação, a original de `tracar_lance` roda com o desvio de 0x6cd330 gravando cada chamada (argumentos, resposta e o estado do sorteio depois); a cópia Rust recebe a chamada como closure (`tracar_com`) e, no lugar de rodar 0x6cd330, consome a gravação: confere os argumentos (tipo, a6 e a7 só pelo byte ou palavra baixa, porque o resto é lixo da pilha no original), devolve a resposta e põe o sorteio no estado gravado. Os efeitos da original ficam na memória (fora das regiões comparadas), e a sequência de chamadas também é comparada. Fora da comparação, a Rust chama 0x6cd330 de verdade.

Equivalentes pela geometria do traçado: numa linha de um ponto, o y já é deslocado antes da primeira casa (a condição "linha não degenerada" nunca decide); o erro acumulado sempre chega exatamente no destino (terminar uma casa antes dá no mesmo, porque a última é o alvo, pulado).

Mutações: 7/9 (as duas restantes, equivalentes). Jogo real: 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 38ª e 39ª levas — jogos do dia e comentários (oficiais em 07/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `tem_jogo(clube, so_marcados)` | 0x6b59a0 | thiscall, ret 8, `eax` | match_day.cpp: o clube está na lista de jogos do dia (mandante ou visitante)? |
| `venceu_o_jogo(jogo, clube)` | 0x6b7cf0 | cdecl, `al` | match_day.cpp: decide por pênaltis, prorrogação ou tempo normal (placar −1 não conta) |
| `comentario_a(...)` / `comentario_b(...)` | 0x7002f0 / 0x70a860 | thiscall, ret 0x14 / 0x10 | invólucros de 0x6d6f70 (enfileirar um evento de comentário) com campos −1 |

Casos de erro do original (ponteiros nulos, clube fora do jogo) mostram uma caixa de mensagem e devolvem 0; em Rust, só devolvem 0 (nunca ocorrem em jogo; o autoteste não os provoca).

**Porta de 0x6d6f70 (comentários).** Na comparação, um desvio em 0x6d6f70 grava as chamadas feitas pela original de uma rotina portada (com o estado do sorteio depois); a cópia Rust recebe a chamada como closure, confere a sequência e não roda o comentário de novo (sem eventos em dobro). 0x6d6f70 pode chamar a si mesma (um comentário puxa outro): só a chamada de primeiro nível é gravada (contador de profundidade) — a primeira versão gravava as internas e acusava divergência falsa. Sem autoteste sintético (o objeto de comentários é do jogo); a discriminação foi provada no jogo real com uma mutação em cada invólucro (342 mil e 3,6 mil divergências).

Mutações: 6/6 (38ª, autoteste). Jogo real: 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 40ª leva — bola fora do campo (oficial em 07/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `bola_fora(lado)` | 0x6cd490 | thiscall, ret 4, `al` | solta a bola do dono e, com a bola fora do campo, marca a reposição (0x8eb2: 2 lateral, 3/4 pela linha de fundo e pelo lado que tocou; 0x8eae: quem repõe) e narra (0x1f76/0x1f75/0x1f77) |

A conferência das chamadas a 0x6d6f70 agora usa máscaras: a rotina lê o 1º argumento como palavra e o 3º ao 6º como bytes, e o original empilha lixo nos bytes altos (por exemplo, `sete al` sobre um eax antigo). Sem autoteste sintético (narração); discriminação provada no jogo real com uma mutação no lado que repõe (638 divergências). Jogo real: 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 41ª leva — valor do chute, prova com rotina grande (oficial em 07/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `valor_do_chute(&tipo, &som)` | 0x700bd0 | thiscall, ret 8, `ax` | `MATCH_PLAYER::get_shot_score` (5,2 KB, cerca de 1.600 instruções): escolhe o tipo do chute quando não vem dado (cobertura contra o goleiro adiantado, conforme a situação do lance em 0x8ea9 e os atributos 0x146..0x154), calcula o valor (palavra por tipo em 0x11a + 2 × tipo, condição física, goleiro, ângulo, distância ao gol com potências 8 e 7, vento no chute de longe), grava a parte de qualidade em 0x39 e o comentário em `som` |

**Prova de viabilidade (pedida pelo Lucas).** Leitura e tradução numa sessão. Os problemas encontrados foram todos de infraestrutura, nenhum de lógica:
1. O gerador do autoteste sobrescrevia o ponteiro do motor (0x19e) ao preencher as palavras por tipo; laço limitado a 0x18e. A queda foi localizada pelo mapa do linker (`-C link-arg=/MAP`).
2. A potência chama a da CRT do jogo (0x945ff0, `_CIpow`) por `asm!`, para resultado idêntico bit a bit; o ponteiro do buffer tem de ficar num registrador preservado pela chamada (`edi`), e não em `eax`.

A lógica bateu na primeira tentativa: 4,77 bilhões de casos e 0 divergências. Mutações: 26/30 (as 4 restantes são equivalentes pelos valores possíveis: a tabela de distâncias não tem 1,5 nem 2,2 exatos, e os ângulos vêm de frações com denominador de 1 a 4, que nunca caem nas faixas mudadas). Jogo real: 1 bilhão de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 42ª leva — força do time, integradora grande (oficial em 07/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `forca_do_time(lado)` | 0x6c0c60 | thiscall, ret 4 | match_eng.cpp (5,1 KB, cerca de 1.250 instruções): zera e soma as sete palavras de força do lado (0x1da … 0x1f2, de 4 em 4, + 2 × lado) pelos 20 jogadores do bloco. Por jogador ativo: desgaste (0x6f56c0), sete componentes (combinações dos floats do jogador; degraus de multiplicadores pelos atributos da pessoa; sorteio do dia, atributo 0x21 contra 20; valor pela condição de 0x7165b0) e, pelo papel tático (bits 1..0x40), ajuste da palavra 0x35 (desconto e ±5 contra um limiar) e as parcelas truncadas. Depois, ajustes pelo país do jogo (estilo da liga nacional: nações em 0x9cf2c0..0x9cf4a8, ver `CAMPOS.md`), por 0x5c6710, pela reputação dos clubes (+0x80) e pelo lado |

Segundo degrau da prova de viabilidade: integradora grande, com chamadas internas com efeito (desgaste, sorteios, condição), comparadas pela própria execução lado a lado (as chamadas caem nos desvios já provados). Detalhes do x87 reproduzidos: um componente fica na pilha do x87 sem passar por float (só os outros são guardados em 32 bits); um dos degraus multiplica direto em st0, sem arredondar a cada passo; a correção por competição lê a palavra do lado 0 e grava na do lado da chamada. Jogador sem ficha (0x69): o original mostra uma caixa de erro e zera 0xb67a34; em Rust, só zera (nunca ocorre em jogo; o autoteste não provoca).

A lógica bateu na primeira tentativa: 5,1 bilhões de casos e 0 divergências. O autoteste escreve ids distintos nas competições do jogo durante o teste (no início do jogo podem estar vazias) e os restaura. Mutações: 22/22.
Jogo real: 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 43ª leva — pontapé inicial (oficial em 07/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `pontape_inicial()` | 0x6c51e0 | thiscall | match_eng.cpp: quem sai jogando (0x8eae; fora de 0..=1, o lado 0), bola no centro (4, 5 ou 6); todos os ativos vão para a casa da formação (tabela 0xf56e + 0x9e do lado 0 ou 0xf572 + 0xa6 do lado 1, 216 bytes por posição; nibble baixo x, alto y, limitado ao próprio campo) por 0x6f46b0 e olham para a frente (0x6f4920). Do lado que sai, o mais perto da bola (posição > 10 vira −1) pega a bola (0x6f0630) e olha para a casa calculada com a última casa da formação usada (mesmo ponto: o byte baixo de 0x8ed8, com sinal). Sem ninguém, marca o byte 0 do motor |

No original, x e y começam com lixo da pilha, mas só são lidos depois de algum jogador ativo (sem ativo, a rotina sai antes). A comparação confere o objeto do motor inteiro (jogadores, grade e bola), mais o sorteio.

Autoteste: 5,5 bilhões de casos, 0 divergências, com a grade do campo consistente (cada jogador na lista da casa dele) e formações com casas distintas. Mutações: 11/13. As duas que passam são equivalentes: o limite de y (no máximo 5 no lado 0, no mínimo 6 no lado 1) torna sempre verdadeiros os testes `y < 9` e `y > 3` da direção. Uma lacuna encontrada e corrigida: o gerador nunca deixava sem ativos o lado que sai, e a mutação do "sem ninguém" passava. Jogo real: 1 bilhão de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 44ª leva — bola rolando (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `rolar_bola()` | 0x6bbe40 | thiscall | match_eng.cpp: bola solta (sem dono em 0xf582, sem 0xf57a) com impulso (0x8eaa). Lance parado: zera o impulso e 0x8ea9. Impulso 1 só anda se sorteio(1) ≥ sorteio(2); impulso ≤ 0 não anda. Ao andar, gasta um de impulso (e um de 0x8ea9) e dá um passo pela direção 0x8ed8: (220, 320) x − 1, (40, 140) x + 1, (130, 230) y − 1, abaixo de 50 ou acima de 310 y + 1. Fora do campo: bola fora (0x6cd490) pelo lado de 0x8eab |

**Porta de narração no autoteste.** No autoteste, a porta de 0x6d6f70 só grava as chamadas (o narrador é falso) em vez de rodar o comentário: as rotinas que narram passam a ter autoteste sintético (antes, só jogo real). **Comparações aninhadas:** quando uma rotina comparada chama outra que também narra (a bola rolando chama a bola fora), a comparação de dentro só passa a chamada adiante, e os comentários entram na gravação de fora (a primeira versão comparava a original de dentro com um valor vazio).

Autoteste: 5,5 bilhões de casos, 0 divergências (direções nas bordas das faixas, saídas pelos quatro lados, com narração). Mutações: 9/12; as 3 que passam são equivalentes: `v <= 0` × `v < 0` (impulso 0 já saiu antes), o limite inferior 130 e o 50 (as faixas anteriores, de 40 a 140, já pegam esses valores). Jogo real: 1 bilhão de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 45ª leva — teste de condição (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `teste_de_condicao(codigo, alvo)` | 0x7164b0 | thiscall, ret 8, `al` | match_pl.cpp: v = condição² (0x1a2, potência pela CRT do jogo); limite 8 se `alvo` for o guardado em 0x1a6, senão 1. Código 1: sorteio(110 − condição) < limite; 2 a 5: sorteio(K / v) < limite, K = 400 mil, 2, 4 e 8 milhões; outros: 0 |

**`_ftol` fora da faixa.** Com condição abaixo de 1, v = 0 e K / v é infinito: o `fistp` do x87 grava o inteiro indefinido (parte baixa 0), enquanto o `as` do Rust satura (−1). O `ftol` em Rust passou a devolver 0 fora da faixa (só muda valores que o original já tratava assim). Mutações: 9/10 (a conversão para byte com sinal da condição é equivalente: a condição vai até 100).

### Motor de partidas, 46ª leva — preferências da pessoa e marca por posição (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `gosta_de(alvo, pessoa)` | 0x547810 | cdecl, `eax` | `alvo` está entre as pessoas de que `pessoa` gosta (preferências em +0x65; vagas +0x1c, +0x20, +0x24) |
| `nao_gosta_de(alvo, pessoa)` | 0x547840 | cdecl, `eax` | idem, das que ela não gosta (+0x28, +0x2c, +0x30) |
| `marcar_posicao(posicao, ficha)` | 0x5b5910 | thiscall, ret 8 | grava na tabela por posição (+0x5bf + 4 × posição) o valor da ficha (−1 sem ficha), zera +0x187f, devolve 1 |

Mutações: 8/8, depois de fechar duas lacunas do gerador (vagas vazias nas preferências, e +0x187f já zerado no motor falso).

### Motor de partidas, 47ª leva — posições finas para desenhar a partida (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `posicoes_finas(saida)` | 0x6d1770 | thiscall, ret 4 | match_eng.cpp: posição fina (grade 27 × 36) de cada jogador para desenhar a partida. Cada casa vira 3 × 3 subcasas; quem tem a bola fica no centro; os outros sorteiam uma subcasa (nunca o centro da casa de quem tem a bola), aceitando uma já ocupada só depois de 50 tentativas por ocupante; o lado que começa em cada casa é sorteado. Saída: bola (x, y) e, por lado e posição tática, 6 bytes (x e y finos, byte 0 do jogador, papel, marca 1) |

É a base de uma futura visão 2D da partida. O "aliasing de pilha" que tinha adiado esta rotina não existia: era o deslocamento de um `push` não contado na leitura. **Gerador do teste corrigido:** as formações da 43ª leva empilhavam mais de 11 jogadores numa casa (y fora do campo rebaixado para a mesma linha), corrompendo as listas da grade; o pontapé passava no teste (original e Rust corrompem igual), mas a nova rotina lê as listas e o original caía. Agora as formações têm casas distintas por posição, e um diagnóstico confere a grade antes e depois do pontapé. Mutações: 8/9 (marcar o centro como ocupado por quem tem a bola é equivalente: o centro da casa dele já é vetado aos outros).

### Motor de partidas, 48ª leva — folhas de outros módulos (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `unidades(n)` | 0x4b0850 | cdecl, `al` | comp_util.cpp: algarismo das unidades de n (abaixo de 1000; senão 0) |
| `e_amistoso(comp)` | 0x5c6710 | thiscall, ret 4 | friendly.cpp: a competição é a guardada em +4 ou está entre as 127 últimas competições de clubes (0xae2378) |
| `contrato_de(pessoa)` | 0x4cf5a0 | thiscall, ret 4 | contract_manager.cpp: contrato (80 bytes) pelo índice na tabela 0xae2d0c (79 bytes por pessoa; quantidade em 0xae2364) |
| `e_selecao(clube)` | 0x53b900 | cdecl | Database.cpp: o id está depois dos clubes nacionais (0xae235c − 2 × 0xae2350) |

Primeiras rotinas fora de `match_*.cpp`: liberam integradoras do motor que dependiam delas. O autoteste troca as globais do jogo durante o teste e as restaura. Mutações: 9/10 (`n <= 1000` é equivalente: para 1000 os dois dão 0). Uma rodada de mutação caiu dentro da DLL; repetida com o mapa do linker, não se reproduziu (6 bilhões de casos, 0 divergências).

### Motor de partidas, 49ª leva — técnico humano e máscaras de formação (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `tecnico_9a01(pessoa)` | 0x5fd210 | thiscall, ret 4 | human_manager.cpp: campo +0x9a01 do registro do técnico humano da pessoa (as 16 últimas pessoas, registros de 42.735 bytes; sem pessoa, o técnico atual da tabela 0xb74c26) |
| `tecnico_9a05(pessoa)` | 0x5fd480 | thiscall, ret 4 | idem, o byte +0x9a05 é diferente de zero? |
| `bit_da_casa(codigo)` | 0x5b5a60 | stdcall, ret 4 | formation.cpp: bit da máscara de formação pelo código da casa tática (31 códigos, tirados do próprio jogo; 0x802 e 0x804 dão o mesmo bit no original) |
| `separar_mascara(m, &setor, &lado)` | 0x5b73a0 | cdecl | formation.cpp: o primeiro bit de setor (1..0x40) e o de lado (0x80, 0x100, 0x200 também com o bit 1, 0x400, 0x800) |

Mutações: 11/11. Jogo real (levas 45 a 49 juntas): 1 bilhão de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

### Motor de partidas, 50ª leva — segunda seleção e placar agregado (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `e_segunda_selecao(clube)` | 0x5409b0 | cdecl | Database.cpp: o id está a partir de total de clubes − nações (0xae235c − 0xae2350) |
| `placar_agregado(lado, &casa, &fora, &classificado, &ida, &fase3, bonus)` | 0x6ce310 | thiscall, ret 0x1c, `al` | match_eng.cpp: gols desta partida (0xf5bd/0xf5f3, trocados com lado 1) somados aos da ida guardados no objeto da partida (+0x4b/+0x4c; 0xff = sem ida); `fase3` pelo algarismo das unidades de +0x30; com o agregado empatado (mais o bônus), sem o bit 4 de +0x38 e, com o bit 0x4000, só depois do tempo 0x3de, decide pelos gols fora. Devolve 1 classificado, senão casa − fora |

Mutações: 11/11.

### Motor de partidas, 51ª leva — marcador direto (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `marcador_direto(lado, k)` | 0x6d4330 | thiscall, ret 8, `al` | match_eng.cpp: o adversário com o papel espelhado (setor: 2/4 ↔ 0x40, 8 ↔ 0x20, 0x10 ↔ 0x10; lado: 0x80 ↔ 0x800, 0x100 ↔ 0x400, 0x200 ↔ 0x200); sem ninguém, o mais perto pela tabela de distâncias com a formação espelhada (0x5b73a0). −1 para goleiro ou jogador fora |

**Gerador correlacionado.** As tabelas de papéis do teste vinham de rotações de um mesmo número (`rotate_left` de 3 em 3 bits): o resto por 9 saía tão correlacionado que alguns setores quase nunca apareciam, e duas mutações passavam. Agora cada entrada tem o seu sorteio. Mutações: 10/10.

### Motor de partidas, 52ª leva — tirar de campo (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `tirar_de_campo(jogador, modo)` | 0x6cdd50 | thiscall, ret 8 | match_eng.cpp: tira o jogador da lista da casa (o último ocupa a vaga) e desfaz as marcações (0x1aa e a lista 0x1ae). Com modo −3, ou sendo o goleiro guardado (0xf59e/0xf5a2), e não sendo atacante: escolhe quem assume a posição, pela maior pontuação de papel (atacante 70 mil, meia ofensivo 50 mil, 0x10 15 mil, outros −5 mil; +5 mil pelos lados); o primeiro candidato entra sem filtro, os seguintes não podem ser goleiro nem papel 2 ou 4. Time de técnico humano só escolhe sozinho com a marca 0x904b, sem ser seleção e sem 0x5fd480 |

**Queda intermitente explicada.** As quedas raras dentro da DLL (em `memcpy`, copiando 1 byte de um endereço inválido) vinham do comparador de 0x6f46b0: ele guarda o byte +0x28 do jogador "movido", lido da lista da casa; com mais de 11 jogadores numa casa (gerador da 43ª leva), esse jogador era lixo. O gerador agora põe cada posição numa casa distinta (no máximo 4 por casa com o limite de y). Mutações da 43ª refeitas com o gerador novo: as mesmas. Mutações da 52ª: 13/14 (15 mil × 20 mil para o papel 0x10 é equivalente: só a ordem das pontuações importa). Jogo real (levas 50 a 52): 1 bilhão de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original.

**A impressão de referência mudou (nas levas 50 a 52).** A carreira em Rust deu c76ad0ff6ce3daa6 (2.357.335.216 sorteios), diferente do 7866d02de7578a90 de antes; a bisseção (`SEM_ROTINAS` em `motor.rs`) apontou `e_segunda_selecao`, que devolve sempre o mesmo valor que a original (conferido em toda a temporada). A causa é a pilha: na simulação todo desvio roda na pilha particular, inclusive no modo "original" (a original passa por ela isolada), e a original de 0x5409b0 deixa o `esi` do chamador 0x204 bytes abaixo da pilha do jogo, lixo que alguma rotina adiante lê. O modo "original" da mesma compilação também dá c76ad0ff6ce3daa6: **a carreira se compara sempre com o modo original da mesma compilação**, nunca com um número memorizado. Com clube nulo, a versão Rust chama a própria original (o caminho de erro tem efeitos).

### Motor de partidas, 53ª leva — marcação individual (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `marcacao_individual()` | 0x6bbf60 | thiscall | match_eng.cpp: para cada lado e jogador ativo cuja posição tem as marcas 0x10 e 4 na tabela do lado (0x5b57c0), acha o marcador direto (0x6d4330) e grava a ficha dele (+0x69) na tabela por posição (0x5b5910). Marcador −1 lê o "jogador" anterior ao bloco, como o original |

Mutações: 5/5.

### Motor de partidas, 54ª leva — nova reputação (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `nova_reputacao(motor, lado, valor, alt, bonus)` | 0x6d3de0 | cdecl | match_eng.cpp, chamada no fim da partida para cada lado (exceto seleção): grava em +0x1d6 + 2 × lado a reputação nova do clube (1..=10000), média móvel com peso `n` entre a reputação atual (+0x80 do clube) e um alvo. Nível (até 20): a reputação da competição (+0x69), ou a média dela com a reputação do adversário / 500 quando a partida tem +0x40; 5 na partida −2. Peso: 202 − 10 × nível; seleção, e competição tipo 8 com certos códigos (+0x32 em 0x41f..0x426, com +1; +0x30 em 0x78..0x7c, 0x82..0x87, 0x96..0x9b, com o nível sem teto), reputação/1000 + 16 (mínimo 21) − nível. Alvo: (nível × 0,5 + 5) × 50 × (valor − razão), com a diferença negativa elevada ao quadrado (com sinal); a razão vem das estatísticas globais (0xb74350: +0x20/+0x28 ou +0x10/+0x18, 6,8 se o divisor for zero). Com `bonus`, o alvo vale no mínimo 250 × nível × valor / razão. O resultado não passa de max(300 × nível + 4000, reputação) nem se afasta da reputação mais que 5 × nível × (2 − reputação/10000) |

Mutações: 13/13 (`n_21` acusada por queda: a versão mutada divide por zero). Nível 21..=28 com os códigos 0x78.. daria peso zero também no original (divisão por zero); o gerador não produz esse caso.

**Testes mais rápidos.** `CM_AUTOTESTE=motor:N` roda só o autoteste do motor, com N casos. O `mutacoes.py` roda a base completa (cerca de 30 minutos) e depois cada mutação com `motor:200000` (`--casos` muda): na 54ª leva, cada mutação foi acusada em cerca de 1 segundo depois de abrir o jogo, contra alguns minutos antes. Jogo real (levas 53 e 54): mais de 1 bilhão comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original (c76ad0ff6ce3daa6, 2.357.335.216 sorteios).

### Motor de partidas, 55ª leva — reputação do jogador (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `reputacao_do_jogador(reg, meu, outro, partida, valor, alt, bonus)` | 0x6d3610 | cdecl | match_eng.cpp, no fim da partida para cada jogador (registro em motor + 0x9032, jogador em +0x1b): a mesma conta da reputação do clube (54ª) para as três reputações da ficha (+0x69 do jogador): atual (+8), em casa (+0xa) e mundial (+0xc); grava (casa, atual, mundial) no registro. Pesos: casa = (reputação/1000 + 16, mínimo 22) − nível; atual = o mesmo se o clube é seleção ou o clube do jogador (+0x1a) é o do clube (+0x53) ou o da competição (+0x5d), senão a média disso com 202 − 10 × nível (mínimo 4); mundial como a do clube (mínimo 21). Alvo: (nível × 0,5 + 5) × 50 × 1,3 × (valor − razão), sem o quadrado do clube. Faixa por partida (× 2 − reputação/10000): casa −250/+300, atual −250/+375, mundial −500/+100 × nível. Teto: 400 × nível (÷ 2 sem o caso especial, para atual e mundial) entre 5000 (mundial 3000) e 10000, nunca abaixo da reputação atual |

Mutações: 13/13.

### Motor de partidas, 56ª leva — escolher o cobrador (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `escolher_cobrador(lado, tipo, excluidos)` | 0x6ceb90 | thiscall, ret 0xc | match_eng.cpp: o cobrador do lado para a bola parada `tipo` (1..=8), pela maior pontuação entre os 20 do bloco (o primeiro empate fica). Fora: posição < 0 ou marcado em `excluidos` (tipo 5); posição > 10 ou sem pessoa (+0x6d): a posição vira −1. O goleiro guardado não cobra 1, 2, 3, 8, nem 6/7 longe da área. 1 e 2: atributo (0xdd, 0xe9) − 2 × distância à bola; 3 (escanteio): o designado na tática do clube (0x897990 sobre 0xde1d08; 10000/9000, com sorteio do lado contrário) ou 0x89 − 0xad/2, × 0,75 (1,25 se negativo) com a marca 0x40; 4: 100 com a tabela = 1; 5 (pênalti): a lista do técnico humano (10000 − ordem, 0x601070) ou 0xc9 pela idade, sorteio de +0x5a e +0xb da pessoa; 6/7 (falta): perto do gol, o designado ou bola parada (+0x29 da pessoa, 0x109); longe, 0xa1 − distâncias; 8: 100 − distância. Sem ninguém: 0 com `excluidos`, senão erro fatal |

**Mutações no jogo real.** A tática do clube (0xde1d08) só existe com o banco carregado; no autoteste sintético o escanteio e a falta perto do gol não rodam (o gerador troca para o tipo 8). `mutacoes.py --jogo AAAA-MM-DD` roda cada mutação no jogo real em comparação até a data (cerca de 95 s até 15/08/2001). Os clubes do computador quase nunca têm cobrador designado: mutações só no designado passam (equivalentes na prática); as da pontuação do escanteio e da falta são acusadas. Mutações: 18/18 no autoteste (uma lacuna fechada: a lista de pênaltis do técnico humano agora tem fichas dos jogadores), 4/4 no jogo real. Jogo real (levas 55 e 56): 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original (c76ad0ff6ce3daa6).

**Base mais curta.** A partir da 57ª leva, a base das mutações roda com 200 mil casos (cerca de 3 minutos); a base completa (2 milhões, cerca de 35 minutos, `--base-completa`) fica para cada 5 levas.

### Motor de partidas, 57ª leva — lateral, escanteio e pênalti (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `lateral()` | 0x6c4290 | thiscall | match_eng.cpp: lateral do lado 0x8eae. Conta (0xf5c8 + 54 × lado), põe a bola (0x8ea7, 0x8ea8) dentro do campo, escolhe o cobrador (tipo 2), arruma os 40 jogadores pela tabela da formação (0xf56e/0xf572 pelo lado do jogador; índice: do lado que cobra?, posição, coluna e linha da bola; byte = coluna + 16 × linha) e põe o cobrador na bola, virado para dentro (90 ou 270). Sem cobrador o original mostra uma caixa de erro (aqui sem a caixa) e marca o motor (+0 = 1) |
| `escanteio()` | 0x6c4470 | thiscall | match_eng.cpp: conta (0xf5c7 + 54 × lado), marca o lance (0x8eb1 = 3, 0x8eae = −1) e arruma os 40: pelo papel no ataque (0x716270, do lado que cobra) ou na defesa (0x716310), alguns vão para casas sorteadas na área (coluna 4 ± 1; linha 5/6, ou 0..1/10..11 sorteada), os outros pela tabela da formação (+0x3b ou +0x30). Fora do campo: erro fatal. Cobrador (tipo 3) na bandeira (coluna 0 ou 8 pelo sorteio contra 0x8ea7; linha 0 ou 11) |
| `penalti()` | 0x6c50d0 | thiscall | match_eng.cpp: cobrador (tipo 5) com a bola na marca (coluna 4; linha 1 ou 10, também em 0x8ea7/0x8ea8); os goleiros guardados vão para o gol, quem está nas linhas 0..1 vai para a 2 e quem está nas 10..11 para a 9 |

Sem autoteste sintético: as três arrumam os 40 jogadores na grade (um gerador sintético teria de manter a grade consistente) e o escanteio consulta a tática do clube. Prova pelo jogo real: mutações 8/8 com `--jogo 2001-08-15`; jogo real 10 bilhões de comparações, 0 divergências; carreira até 01/06/2002: Rust ×2 = original (c76ad0ff6ce3daa6).

### Motor de partidas, 58ª leva — casas da formação e craques (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `gravar_casa(posicao, mascara)` | 0x5b5290 | thiscall, ret 0xc | formation.cpp: liga o bit `mascara` (64 bits, um só) na máscara de casas da posição (+0x188b + 8 × posição), apagando antes o grupo dele (13 grupos de bits que se excluem; dois no dword alto), e zera +0x187f; devolve 1. Posição fora de 0..=10 ou bit desconhecido: o original mostra uma caixa de erro (aqui sem a caixa), zera 0xb67a34 e devolve 0 |
| `marcar_craques(lado)` | 0x6d10a0 | thiscall, ret 4 | match_eng.cpp: liga o bit 0x800000 na formação do lado para os jogadores ativos com nota acima de 8500: 10 × (+0x43 + +0x33 + +0x27 da pessoa) + forma (+0x4d), pelo papel (0x717410): goleiro fora; 2 +500; 4 −1000 (−500 com 0x800 ou 0x80); 8 −300 (sem 0x800 e 0x80); 0x10 +1000; 0x20 +2000 |

Mutações: 12/12 (o caso de erro de `gravar_casa` abre a caixa do jogo e fica fora do autoteste). Primeira leva com a base de 200 mil casos (194 s). Jogo real: 10 bilhões de comparações, 0 divergências. Carreira até 01/06/2002: Rust ×2 = original (c76ad0ff6ce3daa6).

### Motor de partidas, 59ª leva — marcação designada (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `designado_da_posicao(posicao)` | 0x5b58e0 | thiscall, ret 4 | formation.cpp: a pessoa designada para a posição (índice em +0x5bf + 4 × posição, no vetor de pessoas de 0xae23bc, 110 bytes cada); índice negativo: 0 |
| `marcado_designado()` | 0x7160f0 | thiscall | match_pl.cpp: o adversário que o jogador marca individualmente, ou 0. Precisa da marca 4 na casa da posição; procura entre os adversários ativos a ficha designada para a posição. Com a marca 0x10: se a formação adversária tem centroavante pelo meio (bit 0x10000000 de +0x187b) ou o byte +0xaf do outro lado passa de 1, o jogador com 0x106 não marca; e, se o marcado tem as tabelas A e B diferentes (seta para outra casa) e a B sem a marca de ataque 0x40, o jogador com 0x106 também não marca |

Mutações: 9/9. **Duas armadilhas.** (1) Um desvio na 0x714d30 (o lado do jogador, 4 bytes: lê +0x27 e retorna) travou o jogo no início; um getter assim não vale desvio e ficou de fora. (2) O teste mexia nas tabelas A e B, que valem para o caso inteiro, e uma seção seguinte do autoteste parava sem gastar processador (provavelmente a caixa de erro de uma rotina original, invisível com o jogo oculto); o teste agora guarda e restaura as tabelas.

### Motor de partidas, 60ª leva — clube da seleção (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `selecao_principal(nacao)` | 0x53b3d0 | cdecl | Database.cpp: o clube da seleção principal da nação. As seleções ficam no fim do vetor de clubes (0xae23b4, 581 bytes cada): índice = id da nação − 2 × nações (0xae2350) + clubes (0xae235c). Sem nação, sem clubes ou sem 0xae23a8: fica com a original (caixa de erro) |
| `selecao_b(nacao)` | 0x53b4f0 | cdecl | Database.cpp: o clube da seleção B; índice = id − nações + clubes |

Mutações: 5/5, com a base completa (2 milhões de casos, 0 divergências). Muito chamadas (mais de 600 pontos do exe); no jogo real em comparação, 0 divergências.

### Motor de partidas, 61ª leva — quem marcar (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `quem_marcar(jogador)` | 0x6bc030 | thiscall, ret 4 | match_eng.cpp, 3 KB (cerca de 920 instruções): o adversário que o jogador vai marcar, ou 0. Não marca com a marca 1 na casa dele; o marcado designado (0x7160f0) ativo vem antes. Pela casa (x, y) onde ele deve estar (0x6c2be0), avalia quem está com a bola (0xf582, se perto: 0x968950) e os adversários das casas y−1..y+1 × x−1..x+1 (listas em motor + 0x215e, 90 bytes por casa). Nota: sorteios, cerco de adversários, distância ao gol, atributos 0x0a/0x0b/0x10/0x35/0x101, papel na formação (0x8ebc/0x8ec4), reputação (casa, atual ou mundial). Fica a maior nota em 16 bits; o guardado em 0x1aa pode ser escolhido na hora |

Mutações no jogo real (`--jogo`): 11/11. A `q_reput` original somava 1 a todas as notas e não muda o escolhido (mutação equivalente); a substituta (`leva61b.py`) foi pega. **Duas armadilhas.** (1) A comparação não restaurava o estado do sorteio entre a original e a Rust (faltava `REGIOES_DO_ESTADO` no `lado_a_lado`): 50 mil divergências falsas. (2) No trecho de quem está com a bola, o original lê `[esp+0x1c]` com o argumento do sorteio ainda na pilha: é a distância ao gol, não a distância ao jogador. Achado registrando as notas de cada candidato nas divergências.

### Motor de partidas, 62ª leva — técnico humano (0x9ace, 0x9a06) (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `tecnico_9ace(pessoa)` | 0x5fe040 | thiscall, ret 4 | human_manager.cpp: dword +0x9ace do registro do técnico humano da pessoa (ou do técnico atual, 0xb74c26 + 768 × [0xb77c7a]). Sem pessoa ou fora da faixa: a original (caixa de erro) |
| `tecnico_9a06(pessoa, saida)` | 0x5fd6b0 | thiscall, ret 8 | human_manager.cpp: byte +0x9a06 ≠ 0? Com clube (+0x24) de nação com +0x88 < 400: falso e `saida` = 1; `saida` começa em 0 |

Primeira leva lida a partir do pseudocódigo do Ghidra (`descompilar.py`). Mutações: 6/6. A `t_atual` (pegar a vaga 0 em vez da do técnico atual) passou na primeira rodada: quando a pessoa errada não resolve, a rotina devolve o caso à original, que acerta, e o erro some. O autoteste agora põe na vaga 0 outra pessoa válida, e a mutação é pega.

### Motor de partidas, 63ª leva — clube controlado por humano (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `clube_humano(clube, sem_tatica, qualquer, pessoa, forcar)` | 0x600a50 | cdecl | human_manager.cpp: o clube é dirigido por um técnico humano que decide por ele? O técnico (`pessoa`, ou o do clube em +0xcf) tem de ser o do clube e um dos 16 humanos. Sem `sem_tatica`, ficam de fora os que têm 0x9ace (0x5fe040) ou +0x9a01. Com `forcar` ou `qualquer`: sim. Senão, seleção: 0x5fd6b0; clube: 0x5fd480. Se não, e o clube é do segundo conjunto de seleções (0x5409b0), repete com a seleção principal da nação (0x53b3d0), sem `qualquer`. Clube nulo: a original (caixa de erro) |

Mutações: 8/8 no autoteste (clube, seleção e segunda seleção que volta à principal; técnico do clube, outro ou nenhum; com e sem tática). No jogo real só 2/8 apareciam: a carreira automática usa pouco essa rotina. **Armadilha:** editar o `motor.rs` enquanto o `mutacoes.py` roda perde a edição, porque no fim ele regrava o arquivo inteiro como estava no começo.

### Motor de partidas, 64ª leva — lado sob controle (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `lado_sob_controle(lado)` | 0x6d42a0 | thiscall, ret 4 | match_eng.cpp: não nas partidas de tipo +0x3a 1 ou 9. Clube do lado dirigido por humano (0x600a50, `qualquer` = byte +0x40 da partida): pela marca 0x4000 do bloco do lado (0x9766 + 6371 × lado). Senão: o modo do lado em +0x1ab é 3 ou 4 |

Mutações: 5/5 no autoteste (montado sobre o da 63ª leva). Junto: nos testes (jogo oculto), a DLL recusa o `DirectSoundCreate` (DSOUND, importado pelo número 1) com DSERR_NODRIVER; o jogo segue sem som e a carreira dá a mesma impressão.

### Motor de partidas, 65ª leva — tabela tática do clube (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `linha_tatica(clube, humano)` | 0x8982b0 | stdcall, ret 8 (o `this` não é usado) | tactics.cpp: a linha do clube na tabela tática. Clube dirigido por humano (0x600a50 com 1, 1) e `humano` ligado: segunda seleção, −1; senão a linha do técnico depois dos clubes (0xae235c): pessoas (0xae2364) − id do técnico (+0xcf) − 1, ou + 15 numa seleção. Fora da faixa ou sem técnico: a original (caixa de erro). Nos outros casos, o id do clube |
| `byte_tatico(clube, pos, humano)` | 0x898040 | thiscall, ret 0xc | tactics.cpp: byte `pos` (0..=10) da linha tática na tabela em +0x601 (0x91 bytes por linha, a partir de +0x86); linha −1: 0 |

Mutações: 6/6 no autoteste. Junto: `simular.py --som` mantém o som do jogo (`CM_OCULTO=2`; uma variável de ambiente a mais mudaria a heap e a carreira).

### Motor de partidas, 66ª leva — ordem dos cobradores de pênalti (oficial em 08/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `ordem_dos_penaltis()` | 0x6be520 | thiscall | match_eng.cpp: a ordem dos 11 cobradores de cada lado em +0x1af + 11 × lado (−1 antes), marcando os escolhidos nas 20 casas de cada lado em +0x8eda (zeradas antes). Técnico do lado (+0x904d + 208 × lado) humano, com +0x5f, sem 0x9a01 e (com +0x904b) de seleção ou com 0x9a05: a ordem vem da tabela tática (0x898040 sobre [0xde1d08]). Senão, `escolher_cobrador` tipo 5; sem ninguém, zera as marcas do lado e repete a vez |

Mutações: no jogo real `p_marca`, `p_tipo5` e `p_11`; no autoteste (caminho da tabela) `p_marca` e `p_11`. `p_limpa` é equivalente (o laço regrava as 22 vagas). **Limitação:** `p_5f` (o teste da marca +0x5f do técnico) não é pega: a carreira automática não passa pelo caminho humano, e o autoteste não leva um lado ao `escolher_cobrador` com blocos sintéticos. **Armadilhas:** (1) a rotina marca `+0x8eda + 20 × lado + byte da tabela` (com sinal); com bytes aleatórios na tabela sintética a marca caía até 128 bytes em volta (ponteiros da formação em 0x8ebc), e um teste seguinte abria a caixa de erro formation.cpp 3133/3174, visível na tela porque o autoteste roda antes da interface do jogo; o teste agora guarda e restaura essa faixa. (2) Interromper o `mutacoes.py` no meio de uma mutação deixa o `motor.rs` mutado: conferir as listas de mutação antes de seguir.

### Motor de partidas, 67ª leva — ordem dos jogadores no lance (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `ordem_dos_jogadores()` | 0x6bd760 | thiscall | match_eng.cpp: a lista dos jogadores ativos (+0x19 ≥ 0) em +0x8f7e, contagem em +0x8eb8. Para cada vaga 0..20 sorteia (0x90cfc0(2)) qual lado entra primeiro. Ordena com o `qsort` da CRT (0x945333) e a comparação do próprio jogo (0x6bd840, que lê o motor em 0xb67a64) |

Mutações: 4/4 no autoteste (a que não grava 0xb67a64 derruba a comparação original: exceção, também detectada).

### Motor de partidas, 68ª leva — vagas da tabela tática (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `vaga_marcada(clube, humano, menos_um)` | 0x897590 | thiscall, ret 0xc | tactics.cpp: a vaga (0..20) da linha tática (0x8982b0) cuja pessoa (índice em +2 + 4 × vaga, no vetor de 0xae23bc) tem o id guardado em +0x52; sem nenhuma, −1 com `menos_um`, senão 0. Linha −1: 0 |
| `marcar_vaga(clube, vaga, humano)` | 0x897760 | thiscall, ret 0xc | tactics.cpp: guarda em +0x52 o índice da pessoa da vaga (0..20); 1 se havia alguém |
| `pessoa_da_linha(clube, qual, humano)` | 0x897dc0 | thiscall, ret 0xc | tactics.cpp: a pessoa do índice em +0x56 + 4 × `qual` (0 ou 1); negativo: 0 |

Erros (clube nulo, vaga fora): a original (caixa de erro). Mutações: 7/7 no autoteste (pessoas sintéticas; índices sempre de 0 a 29, para não ler fora do vetor sintético).

### Motor de partidas, 69ª leva — clube na competição (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `clube_na_competicao(clube)` | 0x48db80 | thiscall, ret 4 | comp.cpp: o clube está na competição? Antes, pela nação: seleção (id a partir de clubes − nações) de outro continente que o da competição (+4 → +0x59): não. Clube da nação da competição (+4 → +0x5d) e da própria divisão (+0x57): sim; de nação de outro continente: não. Depois percorre a competição (−1) e as fases (+0xc, até +0x30): a última de tipo 2 (+0x43; tabela em +0xb1, linhas de 0x3b) ou, sem nenhuma, a fase da vez (participantes em +0xa7, 6 bytes); quantos em +0x3e |
| `clube_especial(clube)` | 0x76d940 | stdcall, ret 4 | o clube é um dos nove guardados em 0x9cf78c, 0x9cf790, 0x9cf79c..0x9cf7ac, 0x9cf8a0, 0x9cf964 (pelo id)? |

Mutações: 8/8 no autoteste. A `e_lista` só foi pega depois de o autoteste pôr valores distintos nos nove endereços: antes de carregar a base todos são 0, e tirar um da lista não muda nada.

### Motor de partidas, 70ª leva — decisão do árbitro na falta (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `decisao_da_falta(j, outro, tipo, quem, cartao, grav, vantagem)` | 0x6e9820 | thiscall, ret 0x1c | match_man.cpp, 3 KB: o árbitro decide a falta `tipo` do jogador `j` sobre `outro`; devolve 1 ou 7 (cartão dado) e escreve o cartão (4, 5, 7, 10, 11), a gravidade e a vantagem. `this`: +0 motor, +4 árbitro, +8 paciência. Fator de rigor pela severidade (+0x25), pelo modo do lado (+0x1ad) e o estilo (+0x27), pela vantagem e pelo placar (0xf5c9 + 54 × lado), limitado a 0,1..10; amistoso (0x5c6710) alivia. 19 tipos de falta, cada um com seu sorteio contra o fator; a gravidade sobe pela dureza (+0x14) e cai pelo minuto (0x8ed0 / 11). Vantagem: 20 (ou 5) × ficha+0x5c^1,5 / (1 ou 3), com o `pow` da CRT |

Mutações no jogo real: 8/10. **Limitação:** `f_vantagem` (divisor 3 quando o próprio `j` foi o último a derrubar `outro`) e `f_teto` (fator acima de 10) não acontecem até 01/01/2002 na carreira automática; não há autoteste sintético para a rotina.

### Motor de partidas, 71ª leva — escolha do companheiro para a jogada (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `escolher_companheiro(bx, by, tipo, moveu, x, y, melhor, pedido)` | 0x6c6a30 | thiscall, ret 0x20 | match_eng.cpp, 7,4 KB (a maior até agora, cerca de 2.100 instruções e 24 `ftol`): escolhe o companheiro e o tipo de jogada (passe, cruzamento, lançamento, tabela; códigos 7, 8, 0xb, 0x1e, 0x1f, 0x3c, 0x3d) para quem está com a bola. Para cada colega: espaço livre (adversário mais próximo dele e do ponto para onde pode se deslocar, 0x6bce80), papel na formação, posição em relação à linha de impedimento (0x8eaf/0x8eb0), cerco, marcas táticas (0x7163c0, retorno de 64 bits), preferência do técnico (0x897dc0), gostos pessoais (0x547840/0x547810), fator por nação; depois o tipo pela distância até o alvo (ou o `pedido`), e fica a maior nota acima de `*melhor` |

Lida pelo pseudocódigo do Ghidra e conferida instrução por instrução. Pontos que o pseudocódigo escondia: o `pow` com base (3 − cerco) e expoente 3; comparações sobre o valor ainda não arredondado depois de `fst dword` (o float é guardado, mas compara-se o double); testes no bit 32 do retorno de 0x7163c0 (`edx`). Mutações no jogo real: 10/12 (as duas que passam são quase equivalentes: comparar o valor arredondado em vez do não arredondado, e +1 numa nota que raramente empata). Zero divergências já na primeira comparação.

### Motor de partidas, 72ª leva — falta pelo motor, técnico humano (0x9b23), clubes preferidos (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `falta_no_motor(...)` | 0x6cfc60 | thiscall, ret 0x1c | match_eng.cpp: repassa a decisão da falta (0x6e9820) com o árbitro do motor (+0xf5b2) |
| `tecnico_9b23(i, pessoa)` | 0x5fea50 | thiscall, ret 8 | human_manager.cpp: byte +0x9b23 + `i` do registro do técnico humano; erros: a original |
| `gosta_do_clube(pessoa, clube)` | 0x547d60 | cdecl | preferências +0x04, +0x08, +0x0c |
| `nao_gosta_do_clube(pessoa, clube)` | 0x547d30 | cdecl | preferências +0x10, +0x14, +0x18 |

Mutações: 4/4 (a da 0x9b23 só depois de o autoteste encher os bytes em volta de +0x9b23 com valores variados; com zeros, ler um byte ao lado dava o mesmo valor).

### Motor de partidas, 73ª leva — clube parceiro (time B) (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `clube_parceiro(clube, eh_a, ativo)` | 0x540a50 | cdecl | Database.cpp: time A ↔ time B; só clubes de nação com id em 0x9cf488/0x9cf314/0x9cf434; 82 pares de globais (A→B aceita parceiro ≠ −1, B→A aceita ≥ 0); parceiro recusado zera +0x244; registro do parceiro no vetor 0xae23b4 (581 bytes) |

Mutações: 8/8 (os 82 pares foram extraídos da desmontagem; o autoteste dá valores distintos às 167 globais, com parceiros −1 e −5 para separar os dois testes de aceite).

### Motor de partidas, 74ª leva — contrato da pessoa com o clube (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `contrato_com_clube(pessoa, clube)` | 0x4cf5f0 | thiscall, ret 8 | registros de 80 bytes em [this]; a pessoa tem até dois índices (tabela 0xae2d0c, 79 bytes por pessoa); vale o registro cujo +4 é o id do clube; sem nenhum, tenta o clube parceiro (0x540a50) quando ele é o time A |
| `nivel_do_contrato(pessoa, clube)` | 0x53ce90 | cdecl | (byte +0x4f do contrato com 0xae1050 & 0xf) − 10; clube de seleção ou pessoa inválida: o próprio ponteiro do clube com o byte baixo zerado |

Mutações: 7/7 (a do limite das seleções só depois de o autoteste dar valores a 0xae235c e 0xae2350, que valem 0 antes do banco de dados).

### Motor de partidas, 75ª leva — névoa de guerra (o que o clube humano sabe dos jogadores) (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `chance_de_conhecer(pessoa, nivel)` | 0x544410 | cdecl | x = (palavra +4 + 2 × nível) × 0,05 + 10; (x + 0,1 x²) × ⅓ + 0,5, limitado a 1..20 e truncado (`_ftol`) |
| `conhecer_pessoa(pessoa, fonte)` | 0x5b09d0 | thiscall, ret 8 | fog_of_war.cpp: registro de 9 bytes por id (nível 0..3 e a data de hoje, 0xae2c90, por 0x417c10); sem observador o nível vai a 3; com ele sorteia pela chance (0x544410) |
| `nevoa_apos_partida(partida, dados)` | 0x5afe00 | stdcall | fog_of_war.cpp: para os lados humanos (0x600a50) com quadro de conhecimento (0xae2a60), as 20 vagas da escalação ficam conhecidas |

Mutações: 13/15. Duas equivalentes: multiplicar por ⅓ (constante 0x968a80) em vez de dividir por 3 quase nunca muda o truncamento; e o limite `id < 0xae2364` na 0x5afe00, que a 0x5b09d0 refaz. A troca dos lados só apareceu com clubes diferentes nos dois lados.

### Motor de partidas, 76ª leva — lista de observação, bandeiras do contrato (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `entrada_da_lista(pessoa, alvo, nao_11, tambem_13)` | 0x849ce0 | thiscall, ret 0x10 | shortlist_manager.cpp: a entrada (40 de 37 bytes, ponteiro em [this+0xdc] + id × 9) cujo +0xf (ou +0x13) é o id do alvo, com tipo (+0) 11 ou diferente de 11 |
| `bandeiras_do_contrato()` | 0x8676f0 | thiscall | staff_contracts.cpp: +0x40 (4 bytes de rotina; o desvio cobre o primeiro nop do alinhamento) |

Mutações: 6/6.

### Motor de partidas, 77ª leva — próximo da lista, palavra +0xc, arredondamento, nações do grupo, marcadores (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `proximo_da_lista()` | 0x68d1b0 | thiscall | cursor em +0xa (desalinhado): devolve o +0 do nó e avança para o +6 |
| `palavra_0c(id)` | 0x62c3d0 | thiscall, ret 4 | injury.cpp: palavra com sinal +0xc do registro de 31 bytes; id a partir de 0xae2364: 0 |
| `arredondar_abaixo_de_1000(v)` | 0x4b0870 | cdecl | comp_util.cpp: `v` menos o resto (com sinal) por 10 da palavra baixa, quando ela é menor que 1000 |
| `nacao_do_grupo(nacao)` | 0x7e6f20 | stdcall | a nação é uma das seis de 0x9cf794, 0x9cf684, 0x9cf798, 0x9cf6b0, 0x9cf5d8, 0x9cf548? |
| `quantos_marcam(alvo)` | 0x6d59b0 | thiscall, ret 4 | quantos dos 20 do outro lado (ativos) marcam o alvo (0x7160f0) |
| `quem_marca(alvo)` | 0x6d5a10 | thiscall, ret 4 | o primeiro deles, ou 0 |

Mutações: 12/13 (uma equivalente: `< 1000` virar `<= 1000`, já que 1000 tem resto 0). Os marcadores entram no autoteste da 59ª leva, com motor e lado válidos nos 20 do outro lado durante as chamadas.

### Motor de partidas, 78ª leva — competição que conta, comparações de ordenação (comp_util.cpp) (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `competicao_conta(id)` | 0x4b0700 | cdecl | comp_util.cpp: as três de 0x9cf6b0/0x9cf5d8/0x9cf548 sim; sem o +0x5d do registro (0xae23d0, 107 bytes) não; senão só o objeto (0xadadfc) em fase 2 (+0x43), com rodadas (+0x3e) e a primeira rodada (+0xb1) com o clube +4 em +0x57 ou +0x60 |
| `ordem_por_5f(a, b)` | 0x4b0780 | cdecl (qsort) | byte +0x5f de b − o de a (nulo vale 0) |
| `ordem_por_80(a, b)` | 0x4b07b0 | cdecl (qsort) | palavra +0x80 de b − a de a |
| `ordem_por_80_desc(a, b)` | 0x4b07d0 | cdecl (qsort) | idem, com os nulos no fim |
| `ordem_por_80_cresc(a, b)` | 0x4b0810 | cdecl (qsort) | crescente, com os nulos no fim |

Mutações: 11/11.

### Motor de partidas, 79ª leva — nota do jogador (contrato, forma e idade) (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `nota_do_jogador()` | 0x6ebd70 | thiscall | match_pl.cpp: n = nível do contrato (0x53ce90); forma = (palavra +0xd da ficha − palavra +0x80 do clube) × 0,002 + 1,5, limitada a −4..4; c = byte +0x44 do contrato (0x4cf5f0) / 75 + 0,5, ajustado pelo tipo (+0x4f >> 4: −10, +2,5, +2, −3, −15, −20, −50); x = byte +0x22 × 75 na seleção, senão os dias de vida (0x54cdb0); y = 1 + (x − 1000) × 0,000125 acima de 1000. Nota = byte +0x2f × y − (n² + 5) × 0,1 + forma + c + 0,5, truncada |

Mutações: 10/10 (a do sinal do byte +0x44 do contrato só depois de o autoteste da 74ª leva encher esse byte; antes ele era sempre 0).

### Motor de partidas, 80ª leva — esquecer a pessoa e quadro do técnico (névoa), casas da formação (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `esquecer_pessoa(pessoa)` | 0x5afd00 | thiscall, ret 4 | fog_of_war.cpp: com a névoa ligada (0x9cc270) e a pessoa válida, zera o nível dela nos 16 quadros ativos (+4) |
| `quadro_do_tecnico(tecnico)` | 0x5afde0 | thiscall, ret 4 | fog_of_war.cpp: [this + (0xae2364 − id) × 4 − 4]; sem técnico, 0 |
| `casa_do_setor_ocupada(mascara, jogadores)` | 0x5b2e80 | thiscall, ret 8 | formation.cpp: a primeira das 11 casas (+0x12d) com o setor da máscara (0x5b73a0) e o bit 0x200; sem jogadores, 1; senão se algum dos 20 (+0x19, ou +0x1a com 0xfe) está nela |
| `vaga_da_casa(codigo)` | 0x5b2f10 | thiscall, ret 4 | formation.cpp: a vaga 0..10 da casa, ou 0xff |
| `limpar_designados()` | 0x5b5960 | thiscall | formation.cpp: marcas (+0x5b4) com 4 e 8 viram 2; fichas designadas (+0x5bf) voltam a −1; devolve 1 |
| `mascara_da_formacao()` | 0x5b59a0 | thiscall | formation.cpp: +0x187b = OU dos bits das 11 casas (0x5b5a60), com 11 máscaras trocadas por 4 formações de referência |

Mutações: 13/13.

### Motor de partidas, 81ª leva — casa da vaga, critério da competição (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `casa_da_vaga(vaga, segunda)` | 0x5b2ae0 | thiscall, ret 8 | formation.cpp: palavra +0x12d (ou +0x14e) da vaga 0..10; fora da faixa, a original (caixa de erro e 1) |
| `criterio_da_competicao(registro, a, b, tipo)` | 0x4bff70 | cdecl | comp_util.cpp: com a palavra baixa de b (ou de a), 0x4ba8f0(palavra, 0); senão o byte +0x37 (tipo 1) ou +0x52 do registro; sem registro, 1. O caminho das palavras depende da configuração em 0xae3138 e fica coberto só pelo jogo real |

Mutações: 5/5 (a da palavra de `a` derruba a 0x4ba8f0 sem configuração, o que conta como acusada).

### Motor de partidas, 82ª leva — tipos de pessoa, classe da seleção, marcas por pessoa, registro livre (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `tipo_nao_11(tipo)` | 0x53b610 | cdecl | Database.cpp: tipo ≠ 11 |
| `tipo_11_a_15(tipo)` | 0x53b620 | cdecl | Database.cpp: tipo 11, 12, 13 ou 15 |
| `classe_da_selecao(clube, flag)` | 0x76d780 | stdcall | national_teams.cpp: com `flag`, 1; sem ela, 2 para a segunda seleção (0x5409b0) e 0 para as outras; devolve só `al` |
| `ler_marca_da_pessoa(pessoa, k)` | 0x76d7b0 | thiscall, ret 8 | national_teams.cpp: byte [this+4] + id × 110 + k × 20 + 0x45; devolve só `al` |
| `zerar_marca_da_pessoa(pessoa, k)` | 0x76d7e0 | thiscall, ret 8 | national_teams.cpp: zera o mesmo byte |
| `registro_livre()` | 0x53a5b0 | cdecl | Database.cpp: o primeiro registro com +0 zero na tabela 0xae23f8 (60 bytes, 0xae23a0 registros), guardado em 0x9b59b4; sem nenhum, um sorteado |

Mutações: 9/9.

### Motor de partidas, 83ª leva — mais campos por pessoa (national_teams.cpp), global 0x9b5984 (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `ligar_44_da_pessoa(pessoa, k)` | 0x76c0f0 | thiscall, ret 8 | grava 1 no byte +0x44 do bloco k da pessoa ([this+4] + id × 110 + k × 20) |
| `byte_44_da_pessoa(pessoa, k)` | 0x76e010 | thiscall, ret 8 | o mesmo byte, com sinal |
| `campo_3a_da_pessoa(pessoa, k)` | 0x76e040 | thiscall, ret 8 | o endereço do campo +0x3a do bloco |
| `classe_pela_selecao(pessoa)` | 0x76e070 | stdcall | sem seleção (+0x24), 0xff; segunda seleção (0x5409b0), 2; senão o bit 3 do byte +0x5e; devolve só `al` |
| `global_9b5984()` | 0x535920 | cdecl | Database.cpp: lê a global 0x9b5984 |

Mutações: 8/8.

### Motor de partidas, 84ª leva — competição da pessoa, bytes do bloco da seleção, seleções especiais, data com marca (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `competicao_da_pessoa(pessoa)` | 0x7751e0 | thiscall, ret 4 | national_teams.cpp: a competição (0xae23d0, 107 bytes) do índice +4 do registro da pessoa ([this+4], 110 bytes); sem pessoa ou índice negativo, 0 |
| `byte_42_da_selecao(clube, flag)` | 0x7754c0 | thiscall, ret 8 | o byte +0x42 do bloco da classe (como 0x76d780) da nação do clube (+0x53); a original chama 0x5409b0 duas vezes |
| `byte_43_da_selecao(clube, flag)` | 0x775520 | thiscall, ret 8 | idem, byte +0x43 |
| `selecao_ou_especial(clube)` | 0x775790 | stdcall | seleção (0x53b900) ou um dos clubes de 0x9d00a4/0x9d00a8 |
| `nao_gosta_da_selecao(pessoa)` | 0x776380 | stdcall | a pessoa não gosta (0x547d30) da seleção principal (0x53b3d0) da nação +0x1a? |
| `copiar_data_e_marca(origem)` | 0x777590 | thiscall, ret 4 | copia a data (0x417c10) e o byte +8; devolve `this` |

Mutações: 8/8 (a do 0x9d00a8 só depois de o autoteste pôr nele um id que não é de seleção; com 4850, o teste de seleção respondia antes).

### Motor de partidas, 85ª leva — atributos do jogador na partida (0x6ec010, ~11 KB) (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `atualizar_atributos()` | 0x6ec010 | thiscall | match_pl.cpp: recalcula a palavra +0x3b e os 34 campos f32 de +0x71 a +0xf5 do jogador: nota do designado (0x6ebd70, vaga 0x897590), idade, dias de vida, contrato (0x4cf5f0, bits +0x45, histórico +0x10/+0x14), moral, tabelas táticas A/B (0x6c38c0/0x6c38f0), formação (0x6d5b20/0x6d5af0), técnicos e preferências (0x547810/0x547840/0x547d30/0x547d60), potências (0x945ff0) e sorteios. Só o registro do jogador muda |

Mutações: 10/12 no jogo real (9 até 20/08/2001; a do bit 0x40000 do contrato só até 2002-01-01). Duas não alcançadas nem até 2002-01-01: a posição 5 da tática (só o bit 0x40) e o limite do byte +0x1b nos tipos de partida 1/8/32; ficam para a carreira real do Lucas.

### Motor de partidas, 86ª leva — desempenho do jogador na partida (0x6eecb0, ~6,4 KB) (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `desempenho_na_partida(marcar)` | 0x6eecb0 | thiscall, ret 4 | match_pl.cpp: chama 0x6ec010; palavra +0x37 pelas tabelas táticas A e B contra os bytes +0x10..+0x1a da ficha (com `marcar`, soma 1 ou 2 no byte +0xaf do lado no motor); bytes +0x10e..+0x117 (mínimo 2); ~60 palavras derivadas de +0x11a a +0x196 (T = x·10 + 0,5, Q = (x² + 1)/2, médias inteiras); 63 delas ajustadas por +0x37 numa tabela de 8 fórmulas; por fim 0x6f44a0 |

Mutações: 11/12 no jogo real até 20/08/2001 (a que não apareceu exige o byte +0xf da ficha exatamente 15).

### Motor de partidas, 87ª leva — força do lado (0x6bbb60) (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `forca_do_lado(lado)` | 0x6bbb60 | thiscall, ret 4 | match_eng.cpp: zera a palavra +0x1ce + 2 × lado e o byte +0xaf do lado; para cada um dos 20 ativos, −500 quando o bit da casa (0x717410, 0x5b5a60) já apareceu (a máscara soma, não junta), ajustes pela nota (0x6ebd70) conforme o contrato (0x4cf5a0 em 0xae1080, 0x4cf5f0), três sorteios e o designado da tática (0x897590); no fim, 0x6eecb0 com marca para cada ativo |

Mutações: 5/8 no jogo real. As três restantes (máscara somada × juntada, o −750 e o −3× do designado) dependem do ramo do contrato com três sorteios e bits 0x17a0b, que a carreira automática não alcança nem até 2002-01-01; ficam para a carreira real do Lucas.

### Motor de partidas, 88ª leva — bola no ataque, evolução do momento, partida de interesse; prólogos com instrução relativa (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `bola_no_ataque()` | 0x714d00 | thiscall | match_pl.cpp: já existia em Rust, sem desvio; agora desviada (prólogo com `jne` relativo refeito no trampolim) |
| `evoluir_momento()` | 0x6f55c0 | thiscall | match_pl.cpp: 0x6f44a0; 1 em 11: +0xff cai 1 acima de (+0x100/2 + ficha +0x25 + 10)/4, +0x4d soma +0x59 (que zera se 1 ou cai à metade), acima de 50 em +1 tira (+1 − 50)^1,25/4 (até 25); +0x4d em 1..10000 (prólogo com `call` relativo) |
| `partida_de_interesse(partida)` | 0x6b61e0 | cdecl | match_day.cpp: a partida interessa a algum dos 16 técnicos humanos? Pela competição (+0x14): seleções, amistoso (0x5c6710), rodada 150 (0x4b0870), nação (+0x5d) ou competição continental (0xae23a4) |

Mutações: 5/8 no jogo real até 20/08/2001; `i_amistoso`, `i_principal` e `i_16` sobreviveram: o único humano da carreira automática (técnico de testes, de férias) não tem clube, então os ramos de clube/amistoso não disparam e o 16º técnico não existe. Ficam para a carreira real.

### Motor de partidas, 89ª leva — grupo decidido, narração da classificação (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `grupo_decidido(clube, partida)` | 0x6854a0 | thiscall, ret 8 | league.cpp: com a rodada +0xa9 ≥ 24 numa competição que conta, o clube está na frente numa cópia ordenada da tabela (0x687710/0x685f60/0x6827d0/0x687970) e ninguém o alcança com os jogos que faltam (+0xa7 − jogos) × pontos por vitória (+0xc2) |
| `narrar_classificacao(lado)` | 0x6be120 | thiscall, ret 4 | match_eng.cpp: comentários 0x2171/0x2170 (classificação decidida: grupo ou mata-mata via 0x6ce310), 0x2120 (à frente no agregado), 0x2121 (média do +0x1b dos titulares abaixo de 7,0 ou 6,67 e força +0x80 do adversário pesando; 0,8 para o visitante), 0x2001 (lado 1, jogo em aberto) |

Mutações: 1/8 no jogo real até 20/08/2001 (`n_2001`). As outras sete estão em ramos que não ocorrem até agosto: grupo decidido só a partir da rodada 24, e o trecho de +0x8ebb (agregado de mata-mata). Vão para uma rodada mais longa de mutações.

### Motor de partidas, 90ª leva — quem tenta a jogada, jogadores no trajeto; gravação da 0x70dfd0 (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `procurar_jogada(lado, grupo, ..., modo, ...)` | 0x6cd3d0 | thiscall, ret 0x1c | match_eng.cpp: na lista do lado na casa (+0x58 jogadores), quem pode agir (+0x2b até 1, +0x19 ≥ 0); modo 2 só conta; senão cada um tenta a jogada (0x70dfd0, argumentos 4, 3, 6, 7): 1 = conseguiu (+0x12 sobe), 2 = outra saída; o jogador fica em +0xf5ae. Comparação com gravação e reprodução da 0x70dfd0 (porta que grava também as regiões pedidas pelo comparador) |
| `jogadores_no_trajeto(x1, y1, x2, y2)` | 0x6cc070 | thiscall, ret 0x10 | match_eng.cpp: o percurso da 0x6cbc00 (agora `percorrer_linha`, compartilhado) contando pela 0x6cd3d0 em modo 2, com as duas pontas nas linhas retas e x até 11 na horizontal |

Mutações: 7/9 no jogo real até 20/08/2001. `j_ret2` sobreviveu (a resposta 2 da jogada não ocorreu até agosto) e `t_11` é equivalente na prática (x de 9 a 11 já está fora da grade de 9 colunas).

### Motor de partidas, 91ª leva — interceptação no caminho do gol; comparadores sem alocação (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `interceptar(x, y, a3, gol, a5, a6)` | 0x6cc540 | thiscall, ret 0x18 | match_eng.cpp: casas de (x, y) até o centro do gol (x 4; y 11 com `gol` = 1, senão 0), ao estilo de Bresenham nos quatro quadrantes; em cada casa sorteia (2) o lado que tenta primeiro e procura nos dois pela 0x6cd3d0 em modo 1 (distância da tabela desde a saída, ou 1000); volta o primeiro código ≠ 0; +0xf5ae zera no início. Comparação com a fita da 0x70dfd0 cobrindo a grade inteira e os bytes de todos os jogadores |

Mutações: 7/7 no jogo real até 20/08/2001 (seis delas refeitas depois de corrigir o `mutacoes.py`).

### Motor de partidas, 92ª leva — avançar uma linha; fita de gravação genérica (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `avancar_linha(direto)` | 0x6f5b80 | thiscall, ret 4 | match_pl.cpp: o jogador anda uma linha para o lado dele (+0x27 = 1 sobe até 11, senão desce até 0); com `direto` vai (0x6f46b0) para a coluna atual na linha da bola (+0x8eb0 + 1 ou +0x8eaf − 1 do motor), senão reposiciona (0x6f09f0) na nova linha com −1, 0, 0. Comparada pela fita genérica: a 0x6f09f0 ganhou porta de gravação e a 0x6f46b0 grava pelo próprio comparador |

Mutações: 4/6 no jogo real até 20/08/2001. `a_11` e `a_0` (limites da linha no caminho pela 0x6f09f0) sobreviveram: o caminho ocorre (`a_direto` e `a_menos1` foram pegas), mas nenhum jogador estava na linha extrema.

### Motor de partidas, 93ª leva — arrumar para a saída de bola (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `arrumar_saida()` | 0x6c4ec0 | thiscall | match_eng.cpp: quem sai jogando (+0xf59e lado 0, centro 4, 0; +0xf5a2 lado 1, centro 4, 11); solta a bola (0x6f0970); cada jogador em campo vai à casa da formação (tabela +0xf56e da direção, byte +0xa2 da posição: x no nibble baixo, y no alto), refaz as linhas defensivas (0x6c3aa0) e, no lado que sai jogando, quem passou da linha volta uma (0x6f5b80); +0x2b zera; no fim quem sai jogando vai ao centro e pega a bola (0x6f0630), +0x19c = 200, +0x19a = 180 fora da linha 0. Sem ele: a original (caixa de erro). Comparada com `lado_a_lado` sobre o motor de 0x1a0, a grade, os 40 jogadores e os ponteiros de 0xf56e |

Mutações: 6/6 no jogo real até 20/08/2001.

### Motor de partidas, 94ª leva — arrumar para uma bola parada (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `arrumar_bola_parada()` | 0x6c4820 | thiscall | match_eng.cpp: contador +0xf5c6 + 54 × lado; solta a bola. Bola na área do próprio lado: formação (como 0x6c4ec0) e quem do outro lado cair na marca sai uma linha. Senão: marca limitada à grade, longe (+0x8eb1 = 7) ou perto (−1), coluna base 4 − sorteio + sorteio, linha base a 3/4 da marca; perto, todos pela tabela de bola parada (12 linhas × 9 colunas por posição e lado que cobra); longe, pelo papel (0x716270 no ataque: 1 fundo, 2 meio; 0x716310 na defesa: 4 fundo, 3 meio). No fim o cobrador (0x6ceb90, tipo +0x8eb2) vai à marca, pega a bola e aponta (0x6f4920). Casa fora da grade: erro fatal, como a original |

Mutações: 8/8 no jogo real até 20/08/2001.

### Motor de partidas, 95ª leva — o jogador sem a bola decide para onde ir (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `decidir_sem_bola(_)` | 0x713fc0 | thiscall, ret 4 (argumento não lido) | match_pl.cpp, ~3,3 KB: ação em +0x198 e a casa. Bola fora de jogo: formação (0x6c2a40, 0x68). Bola parada: quem bate (+0xf57a) ou quem o lado aponta vai à marca (0x100). No jogo: perto da área adversária, ou com a marca 0x200000 da posição, tenta subir para casa livre (0x6bce80; 0x6a, +0x35 sobe 7) por sorteio contra (1 − 0x35·c)·0x101·0x107; senão parte da casa de 0x6c2be0 (x por 0x715f50), pontas vão ao canto pela bola (marcas 0x80/0x800 de `tabela_a`), a linha vai ao fundo por sorteio ou pela linha (0x6cb700) e anda pela diferença do lado (0x6c4180); marcando alguém (+0x1aa, 0x105) segue a casa dele, um passo à frente pelo sorteio(21) contra o 0xad dele; senão avança uma linha (0x6f5b80, 0x6b) pela diferença do jogador; na casa: fica (0x76) ou procura casa livre (0x6a/0x69); fora: procura casa livre ou vai (0x68). Comparada pela fita (0x6f09f0, 0x6f46b0 e a própria 0x6f5b80 como unidade) |

Mutações: 8/10 no jogo real até 20/08/2001. `s_formacao` (o despachante 0x7103e0 só chama com a bola em jogo) e `s_fica` (a ação gravada antes da chamada era apagada pela foto de depois) sobreviveram; a leva 96 traz a foto de antes e refaz as duas.

### Motor de partidas, 96ª leva — o movimento geral sem a bola; comentários e pegar a bola na fita (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `mover_sem_bola()` | 0x714d40 | thiscall | match_pl.cpp, ~4,7 KB (fora da faixa de ataque; dentro dela, com a bola em jogo, é 0x713fc0): bola parada (marca, 0x100), formação (0x68), infiltração além da linha de impedimento (0x7a, pela ficha +0x105/+0x106/+0x10a/+0x10b e o goleiro guardado), apoio e pressão (0x100), marcação individual (+0x1aa, 0x105, ou larga), limiar de pressão (1,5 ou ~2, reduzido sem +0x106/+0x10a e pelo sorteio contra +0x1a2). Goleiro: pega e narra (0x1f46, 0x103), sai à bola (0x103) ou volta (0x71). Os outros: fundo (0x78), avançar (0x6b), ir à bola (0x100; nela pega e narra 0x1f46/0x2113, ou 0x104), ficar na área adversária (0x104), casa (0x68/0x76) |

Mutações: 9/12 no jogo real até 20/08/2001, incluindo `s_fica` e `s_formacao` da leva 95, agora pegas pela foto de antes da chamada. Sobreviveram `m_goleiro_9` (sem goleiro guardado), `m_2113` (comentário de bola parada perto do gol com +0x1cd) e `m_largar` (marcado com posição −3): situações que não ocorreram até agosto.

### Motor de partidas, 97ª leva — a decisão de cada jogador a cada instante (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `decidir_jogador()` | 0x7103e0 | thiscall | match_pl.cpp: sem a bola, "desliga" (0x67) se sorteio(10) + +0x4d·c não passar de sorteio(65); com a bola fora do alcance do lado, sorteio(20) zero e sem +0x10d desliga cansando (+0x35 − 15, +0x2b = 1, +0x4d + 5..9); com o lado na situação 5 (+0x1ab) e longe do gol, desliga sem o −15; com +0x2b, parado. Antes de cada desfecho os marcados (+0x1ae) se mexem (0x714d40) por sorteio(900) contra +0x107·+0x104. O dono da bola conduz (0x7109f0, nova porta da fita) e pode decidir de novo (sorteio(+0x7d) contra sorteio(5)); os outros vão a 0x713fc0 (bola com o lado, na faixa, em jogo) ou 0x714d40 |

Mutações: 8/8 no jogo real até 20/08/2001. Carreiras Rust ×2 e original iguais em `cd2c213e0ddda4d1` (2.357.622.872 sorteios): a referência mudou porque os arquivos de idioma (`jogo\*.lng`) foram reescritos às 13h04 de 10/10, fora deste trabalho.

### Motor de partidas, 98ª leva — o passe e o impedimento (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `passar(tipo, receptor, saida, a4, a5)` | 0x6fa250 | thiscall, ret 0x14 | match_pl.cpp: o passe (+0x2b sobe). Executa pela 0x6fad40 (13,8 KB, nova porta da fita: grava x, y, tipo, receptor e o 4o argumento e devolve a resposta com o byte que ela escreve na saída). Fora do tipo 0x36 procura o impedido (0x6cfd70); sorteio(2·d1·d2 da distância dele ao gol) zero: impedimento, narra 0x1f78, conta (+0xf5ca), +0x13 do impedido, −1 em +1 do passador, bola com o outro lado (+0x8eb2 = 7). Senão registra o lance (0x6bb9a0). Com a saída do passe: receptor vira o batedor (+0xf57a), posses (+0x8d46) e decisão imediata do receptor (0x7103e0) por sorteio(80) contra +0x105·+0x107 |

Mutações: 8/8 no jogo real até 20/08/2001, com a comparação aninhada (antes dela `p_2b` sobrevivia: o passe nunca era comparado).

### Motor de partidas, 99ª leva — o lançamento (oficial em 09/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `lancar(modo, refinar)` | 0x6fa490 | thiscall, ret 8 | match_pl.cpp: chutão, cruzamento ou bola longa. A direção (+0x19a, graus) marca para que lados a bola vai; perto do gol e fora do miolo, cruzamento (0x5e) para a linha 0..1/10..11 e coluna 3..5; no próprio campo 0x5c (modo 4) ou 0x5d; do meio para a frente 0x5b (modo 4) ou 9/0x3e (bola em jogo); os desvios são sorteios de 2 a 5 em torno da marca. Sem `refinar`, longe do próprio gol e com +0x10b ou sorteio(3), o alvo vai a meio caminho da ponta (coluna 1 ou 7) e da área. Executa (0x6fad40), impedimento como 0x6fa250, bola fora da grade: 0x6cd490 (unidade da fita) |

Mutações: 9/9 no jogo real até 20/08/2001. Carreiras Rust ×2 e original em `cd2c213e0ddda4d1`.

### Motor de partidas, 100ª leva — a bola desviada (oficial em 10/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `desviar_bola(x, y, espalhar, lado, dentro, cansa, chutador)` | 0x6cd770 | thiscall, ret 0x1c | match_eng.cpp (13 chamadores): solta a bola e ela espirra de (x, y): pela lateral (coluna 9 ou −1) fora do miolo por sorteio(4), ou em x, y ± sorteio(espalhar + 1) (com `dentro`, até a linha caber). Altura +0x8eaa, efeito +0x8ed8 pela tabela 0xb67c30 do desvio, +0x8ea9 = sorteio(4), posse do `lado` (+0x8d46). Com `cansa`, sorteios em +0x2b dos jogadores (0x7160d0). Na linha de fundo junto ao gol com o `chutador`: 0x70c040 ou 0x70a910 (novas portas da fita); depois, a bola fora (0x6cd490) |

Mutações: 8 de 8 pegas (b_lateral, b_espalhar, b_dentro, b_altura, b_efeito, b_estado, b_cansa, b_fundo); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (cd2c213e0ddda4d1).

### Motor de partidas, 101ª leva — a defesa do goleiro (oficial em 10/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `defender(forca)` | 0x70d430 | thiscall, ret 4 | match_pl.cpp: acima de 1000, ou sem vencer sorteio(ficha +0x21 + +0xa5) contra sorteio(20), não defende (0x1fe2, devolve 1). Senão espalma (+0x106 acima de sorteio(8): 0x1fe3, bola desviada 3 casas) ou rebate (0x216f, 1 casa; cansa +0x35 − 250 e pode perder sorteio(3) em +0x17 com o desempenho refeito, 0x7172d0/0x6eecb0); +0x2b soma sorteio(2 ou 3); a bola fica em jogo com sorteio(10) e o goleiro é chutador da linha de fundo com sorteio(200) zero. A bola desviada (0x6cd770) é unidade da fita quando chamada direto pela comparada |

Mutações: 6 de 8 pegas (g_20, g_espalma, g_2b, g_dentro, g_espalhar, g_cansa); sobrevivem g_1000 (força exatamente 1000) e g_17 (rebate com sorteio(30 × v + 20) zero), caminhos raros; jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (cd2c213e0ddda4d1).

### Motor de partidas, 102ª leva — o fôlego do jogador (oficial em 10/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `recuperar_folego(teto)` | 0x6f5810 | thiscall, ret 4 | match_pl.cpp, sem sorteio: recupera o fôlego (+0x4d) e o teto dele (+0x51). Fator 1, ou 1,4 − 0,0004 × lesão (0x62c3d0) acima de 1000. Sem `teto`, soma 10 × (ritmo +0x34 da ficha + 15) × fator em campo (0x6c38c0 = 1) ou fora do banco (12 × ritmo + 325 com +0x1f = −1), até 10000. Com `teto`, +0x51 = fôlego + (12 × ritmo + 85) × fator, multiplicado por 5,56 × (+0x20 ou +0x21) − 255,76 com esse valor entre 46 e 64; fora de jogo, se a lesão (0x62c590) passar do teto, vira 6500; por fim soma +0x59, até 10000 |

Mutações: 8 de 9 pegas (f_campo, f_banco, f_fora, f_k, f_marca, f_copia, f_6500, f_59); sobrevive f_1000 (lesão exatamente 1000, limite); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (cd2c213e0ddda4d1).

### Motor de partidas, 103ª leva — o fim do período (oficial em 10/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `periodo_encerrado()` | 0x6bd930 | thiscall | match_eng.cpp (`period_completed`): fecha o período de +0x8ed4 quando o relógio (+0x8ed0) passa do fim mais os acréscimos (+0x8eb7). 1º tempo: placar do intervalo no jogo (+0x4792 +0x45/+0x46), narra 0x20f1, recupera o fôlego e vai ao 2º (1). 2º tempo: pela regra do jogo (+0x38) e o agregado (+0x4b/+0x4c), encerra (4: classificação 0x6be120, teto do fôlego, resumo 0x6ce460; pênaltis em +0x8eb3) ou vai à prorrogação (0x20f2, 2). Prorrogação: 0x20f3 e 2º tempo (3), depois encerra. Período desconhecido: registra o erro (0x944c9f). A classificação e o resumo viram unidades da fita quando chamados direto pela comparada |

Mutações: 9 de 9 pegas (p_passou, p_intervalo, p_20f1, p_3de, p_teto0, p_saida, p_placar, p_fim, p_ordem); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (cd2c213e0ddda4d1).

### Motor de partidas, 104ª leva — a pressão sobre o jogador (oficial em 10/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `pressao(j, saida, lista)` | 0x6bcbd0 | thiscall, ret 0xc | match_eng.cpp, sem sorteio: soma em `*saida` a pressão dos adversários ativos na casa de `j` e devolve quantos pesam (2 por marcador). Quem marca `j` (0x714c60) vale (0x6f4600 + 0x10a − +0x19c/20 − +0x107 de `j`) × 1,7; os outros em pé, fora da tabela 1 (0x717410) e com +0x10a ou o bit 0x100 de +0x198, valem (0x10a − …) × 1,7 / (+0x2b + 1). Com `lista`, os dois que mais pesam (acima de 5) entram nela como cópia de 0x1be bytes (0x68d0d0, unidade da fita para a cópia Rust não alocar de novo) |

Mutações: 9 de 9 pegas (x_lado, x_base, x_marca2, x_dobro, x_2b, x_bit, x_5, x_segundo, x_ordem); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (cd2c213e0ddda4d1). A 1ª tentativa leu 0x66666667 + sar 3 como /10 (é /20): a base pegou na hora.

### Motor de partidas, 105ª leva — o desafio dos adversários (oficial em 10/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `desafio_adversario(atacante, lado, tipo, a, b, nota, lance, arriscado, livre)` | 0x6c87e0 | thiscall, ret 0x24 | match_eng.cpp (`get_opposition_challenge`, ~5,3 KB): entre os adversários ativos na casa da bola, quem desafia o lance `tipo`. A distância ao gol (tabela 0xa15e44) dá a dificuldade; cada um tem um peso (marcas táticas 0x7163c0 ou a estratégia do lado, marcação +0x1a6/+0x1aa, sorteio pela leitura +0x106 e +0x19c do atacante, forma +0x24/+0x75, fôlego +0x4d, mínimo 0,05) que divide o k de cada tipo (0xc–0xe, 0xf/0x10/0x67, 0x11/0x12, 0x15/0x25/0x2e/0x38 com a potência 5 de quantos estão na casa, 0x3b, 0x6c, 0x6d) num sorteio contra +0x10d + +0x10a − distância. Quem vence recebe código e lances alternativos; impedimento (0x6cb780) e posição marcam `*arriscado`; valor (0x7165b0, 0x5407e0, 0x6f5b20) escolhe o lance e o domínio (0x70d690) dá a nota. O de maior nota fica em `*nota`/`*lance` e cansa (0x6f56c0). O peso segue em precisão dupla entre as contas, como no registro do x87 |

Mutações: 17 de 18 pegas; sobrevive o_marca (o peso 2,25 de quem marca o atacante, +0x1a6: caminho não exercitado até 2001-08-20, conferido na desmontagem); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (cd2c213e0ddda4d1).

### Motor de partidas, 106ª leva — a evolução do jogador depois do jogo (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `evoluir_jogador(jm, pessoa, clube, rep, comp, nota, media_b, forte)` | 0x6d26b0 | cdecl | match_eng.cpp (~3,9 KB): as três notas da ficha (+0x61: +0x9, +0xb, +0xd) andam pela nota do jogo. O nível vem da competição (−2: 5; +0x40: média da reputação/500 e do +0x69; seleções 0x76d940 no mínimo 15; até 20); cada nota converge para nota + (nível/2 + 5) × desvio × 50 (desvio contra a média do campeonato 0xb74350, ao quadrado com sinal abaixo dela), com divisores pela casa de milhar da nota + 16 menos o nível; `forte` põe 250 × nível × nota/média como piso. Pisos sobem com reputação e nota em jogos grandes (+0x30/+0x32 da competição). A diferença na +0xb dá o moral por sorteio até ±20 (0x7172d0 no jogador do jogo ou 0x544710 na pessoa); depois as notas ficam na faixa do potencial (+0x5) e da idade (250/300/375/500 × (2 − nota/10000)) e a +0xb puxa a reputação do clube (0x4cf5f0, potência 3 pela pow da CRT). Com `jm`, grava +0x47/+0x49/+0x4b dele |

Mutações: 13 de 15 pegas; sobrevivem e_nivel20 (teto 20 do nível, competições de nível 20 não aparecem até 2001-08-20) e e_div9 (divisor da +9 abaixo de 5, raro fora de jogos grandes); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (cd2c213e0ddda4d1).

### Motor de partidas, 107ª leva — o uniforme (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `uniforme(clube, outro, qualquer)` | 0x6d47e0 | cdecl | match_eng.cpp: qual dos três uniformes do clube (pares de cores +0x83/+0x87, +0x8b/+0x8f, +0x93/+0x97) usar contra o outro. Sem clube ou outro, registra o erro e devolve 0. A 0x48ce70 (devolve 0) decidiria trocar as cores (0x48ce90). Sem `qualquer`, o par vale se a segunda cor não choca (0x540490) com a do outro (+0x87, ou a da tabela 0xae23ec pelo índice 0x9cf17c); devolve o número do par, 1 com cor zero, 2 se faltar o 2º/3º uniforme, 1 se nenhum servir |

Mutações: 4 de 5 pegas; sobrevive u_nenhum (os três uniformes chocando, não ocorre até 2001-08-20); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (cd2c213e0ddda4d1).

### Motor de partidas, 108ª leva — as folhas das lesões (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `gravidade_da_lesao(tipo)` | 0x62ec00 | thiscall, ret 4 | injury.cpp: o byte de gravidade do tipo de lesão (tabela de 13 bytes por tipo em 0x9e7ad9) |
| `grau_da_lesao(pessoa)` | 0x62ece0 | thiscall, ret 4 | injury.cpp: o grau da lesão da pessoa (registro de 31 bytes por id na tabela das lesões, +0x13, pelo menos 1), ou 0 sem pessoa, sem ficha (+0x61), id fora da tabela (0xae2364) ou sem lesão (+0x12 negativo) |

Mutações: 2 de 3 pegas; sobrevive l_sem (registro com +0x12 exatamente 0); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (cd2c213e0ddda4d1). As 108ª e 109ª preparadas antes (0x6d3de0, 0x6d3610) foram descartadas: já eram portadas.

### Motor de partidas, 109ª leva — a lesão na dividida (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `lesao_na_dividida(a, tipo_a, b, tipo_b, direto)` | 0x6cfe10 | thiscall, ret 0x14 | match_eng.cpp (~2,6 KB): a dividida entre `a` e `b` pode machucar. Cada um tem chance (sorteio(sorteio(102)), refeita acima de 50) e risco 4 (6 já machucado, gravidade 0x62ec00, chance dobrada em 24/25) ajustado pelo fôlego (10000/max(1000, +0x4d/2 + 5000)), pela ficha (+0x2d, sempre a de `a` decide o ramo) e pelo grau da lesão (0x62ece0). `a` sem fôlego (< 4000) se machuca na hora. Com `direto` só `a` arrisca; senão quem tem a bola soma o risco pelo lance do outro (+0xd9 × fator), o lance 5 divide por +0x79, e o tipo vem da gravidade ou de sorteio. A lesão atual (0x62ed70, que antes a atualiza pela 0x62c680) e a lesão aplicada (0x6d0980, que aloca) são unidades da fita |

Mutações: 6 de 9 pegas; sobrevivem d_ramo (ficha +0x2d exatamente 11), d_grau (lesão de grau acima de 3) e d_5 (+0x79 até 2 no lance 5), valores raros até 2001-08-20; jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (cd2c213e0ddda4d1).

### Datas, 110ª leva — a diferença em dias entre datas (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `dias_ate(outra)` | 0x54cdb0 | thiscall, ret 8 | Date.cpp (`rust/src/data.rs`): this − outra em dias, com `outra` por valor. No mesmo ano é a diferença dos dias; com anos diferentes, a Data estática 0xae2470 recebe *this e anda ano a ano até o de `outra` somando os dias de cada ano (para trás os do ano alcançado; para a frente, como no original, os do ano anterior a cada um), e fica com o ano de `outra`. O `mutacoes.py` passou a aceitar `ARQUIVO = 'data.rs'` na lista |

Mutações: 2 de 4 pegas; sobrevivem y_frente e y_dia (o caminho com a outra data depois desta não roda até 2001-08-20: o jogo só conta para trás, hoje menos o nascimento); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais com a nova referência db8ee4d7a7e1ef8e (os .lng foram regravados pelo jogo às 08:14 de 11/10, como em 10/10; a original deu a mesma impressão).

### Motor de partidas, 111ª leva — a avaliação do jogador pela comissão (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `avaliar_jogador(pessoa, jogador, clube)` | 0x6e1cf0 | cdecl | match_man.cpp: a nota (i16) que a comissão dá à pessoa (ou à do `jogador` +0x69) para o clube; −10000 sem pessoa. Soma a ficha +0x2f vezes o fator dos dias de vida (0x54cdb0 com a data de hoje 0xae2c90; nas seleções +0x22 × 75; acima de 1000 dias, 1 + (dias − 1000) × 0,000125), tira (encaixe² + 5) × 0,1 (0x53ce90), soma a diferença da +0xd para a reputação do clube (× 0,002 + 1,5, entre −4 e 4) e o contrato (0x4cf5f0: +0x44/75 + 0,5 ajustado pelo tipo em +0x4f >> 4). Depois pela idade (×0,5/0,75/0,9/1,05), ×0,9 com dias abaixo de (50 − +0x59) × 10 e ×(0,01 × +0x5b + 0,9) com +0x5b abaixo de 10 |

Mutações: 7 de 7 pegas; jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (db8ee4d7a7e1ef8e). Primeira rotina do match_man.cpp.

### Motor de partidas, 112ª leva — o mais bem avaliado do lado (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `melhor_avaliado()` | 0x6e1a40 | thiscall | match_man.cpp: o índice (0..20) do jogador ativo (+0x19 ≥ 0) mais bem avaliado pela comissão (0x6e1cf0 com o clube +0x2f) entre os 20 do bloco que começa em +0x23; o primeiro ativo entra mesmo com nota negativa, depois só nota estritamente maior; −1 sem nenhum ativo |

Mutações: 2 de 2 pegas; jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (db8ee4d7a7e1ef8e).

### Motor de partidas, 113ª leva — o mais bem avaliado da lista (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `melhor_da_lista(lista, n)` | 0x6e1940 | thiscall, ret 8 | match_man.cpp: entre as `n` pessoas da lista (com ficha +0x61), a mais bem avaliada pela comissão (0x6e1cf0 com o clube +0x2f); 0 sem nenhuma. Com o objeto +0x1f, se o valor dele (0x544410 com o byte +0x17) ficar abaixo do id da pessoa mod 20, a nota vira (1,1 − 0,01 × (endereço do objeto mod 20)) × nota, como no original; empate fica com a primeira |

Mutações: 3 de 3 pegas; jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (db8ee4d7a7e1ef8e).

### Motor de partidas, 114ª leva — o jogador que sai de campo (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `jogador_saiu(j, ja_fora)` | 0x6cda20 | thiscall, ret 8 | match_eng.cpp: `j` sai de campo (lesão ou expulsão). Sem `ja_fora`, soma +0x23, grava o minuto (+0x8ed0/11, no máximo o fim do período) em +0x21 e tira da grade (0x6cdd50). Fica com +0x19 = −3. Com técnico humano no lado (0x5fd210/0x5fd480) só segue com +0x904b e fora de seleção. Goleiro com +0x9049: a comissão troca o goleiro (0x6e1090 com 1); senão refaz o time (0x6e1aa0, 0x6e1090), escolhe novo capitão se era ele (0x897590, 0x6e1a40, 0x897760) e recompõe o jogador de posição 1 (0x6ce110); guarda o minuto da troca (+0x8ecc) e refaz força do lado (0x6bbb60) e marcação (0x6bbf60). A comissão (0x6e1aa0, 0x6e1090, ainda no original) e o capitão gravado são unidades da fita |

Mutações: 6 de 7 pegas; sobrevive s_zera (zerar o jogador de posição 1 do outro lado: a 0x6ce110 refaz os dois em seguida); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (db8ee4d7a7e1ef8e). A 1ª base divergiu: a substituição (unidade) troca o capitão na linha da tática (+0x52), fora das regiões; a linha dos dois clubes entrou nas regiões. O mutacoes.py tenta compilar de novo quando o cargo falha sem mensagem.

### Motor de partidas, 115ª leva — a comissão refaz o time; a fita com vetores (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `refazer_time()` | 0x6e1aa0 | thiscall | match_man.cpp: a comissão refaz o time. Com 11 em campo e a escalação reprovada (0x6e0230 com 0x8c1, 2) narra 0x2007. Junta os de linha em campo (posição +0x19, tabelas 0x717410/0x717450, marcas 0x5b5270 e byte 0x5b57c0 da posição, ficha +0x69), escala (0x6e2000 com 0x8800, ou 0x408800 com técnico humano a ser consultado: 0x600a50, 0x5fd210, 0x5fea50) e devolve a cada ficha escalada a posição do lugar que ganhou, refazendo o desempenho (0x6eecb0). A 0x6e0230 e a 0x6e2000 (escalar, recursiva, aloca) são unidades com vetores: a fita grava os bytes dos vetores de entrada e de saída passados por ponteiro, confere os de entrada da cópia Rust e escreve os de saída (`chamar_gravando_vetores`, `pedir_vetores`). A porta provisória da 0x6e1aa0 da 114ª leva saiu |

Mutações: 5 de 5 pegas; jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (db8ee4d7a7e1ef8e).

### Motor de partidas, 116ª leva — a substituição (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `substituir(goleiro)` | 0x6e1090 | thiscall, ret 4 | match_man.cpp (~2,2 KB): a comissão faz até +0x17 trocas. Os elegíveis (em campo, ou −2 sem +0x1e) são ordenados pelo qsort da CRT (0x945333 com os comparadores do jogo 0x6e0df0/0x6e0fe0); os primeiros dão as vagas (tabelas, posição, marcas e byte da posição; a primeira é 0 com `goleiro`) e os reservas livres do 12º em diante entram como candidatos. O modo da escala vem do clube, da força contra o adversário (com sorteios), da competição (+0x3a; +0x1a = 3 e 0x48f150), de +0x16 e do técnico humano. Quem ganha vaga e estava no banco entra (0x6cdd00, minuto em +0x1f); quem fica sem vaga sai (0x6cdd50, +0x19 = −4, minuto em +0x20). Narra as trocas (0x1f59/0x1f5c/0x1f5a), o cansaço (0x211d) e a lesão (0x211e) por sorteio e 0x20d7/0x1f5b; depois o mais bem avaliado vira capitão (0x897760) e o time é refeito (0x6e1aa0). A escala (0x6e2000) é unidade com vetores; a porta provisória da 0x6e1090 da 114ª leva saiu |

Mutações: 4 de 8 pegas (rodada conjunta com a 117ª); sobrevivem v_qsort (só difere com exatamente 11 elegíveis e ordens diferentes nos dois comparadores), v_goleiro (troca de goleiro com +0x9049: não ocorreu até 20/08/2001), v_forca (igualdade exata no sorteio de 3000) e v_cansaco (sorteio com +0x31 negativo); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (7cb2218eb00107c9, nova referência: o conjunto de desvios muda o lixo de pilha nos dois modos).

### Motor de partidas, 117ª leva — a escala (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `escalar(...)` | 0x6e2000 | cdecl, 15 argumentos | match_man.cpp (~3,9 KB, recursiva): distribui `n` pessoas por `m` vagas (+ `x` extras) e grava a pessoa de cada vaga. Cada par (pessoa, vaga) vale a nota da 0x6e2f20 (código da vaga, marcas, byte e curto da vaga, modo, clube, comissão; com o bit 0x400000 e técnico humano, 0x5fed50). Com menos pessoas que vagas, cada pessoa livre procura a vaga onde rende mais que o ocupante (até 10000 voltas) e no fim saem as vagas pares com ficha +0xf = 20 e as ímpares com +0xf < 10; com mais pessoas, cada vaga pega a melhor e os repetidos são resolvidos pela menor perda (até 10000 voltas). As extras recebem as pessoas que sobraram pela própria 0x6e2000 (modo | 0x200); por fim troca pares de vagas enquanto a soma melhora (até 100 voltas). A memória de trabalho é a da CRT do jogo (0x945501/0x9452ca), na mesma ordem. Dentro de uma comparação com fita é unidade com vetores (a porta provisória da 115ª leva saiu) |

Mutações: 5 de 8 pegas (rodada conjunta com a 116ª); sobrevivem e_impar (fronteira +0xf = 9), e_extras (o curto das vagas extras quase nunca pesa na nota da 0x6e2f20) e e_tecnico (só com técnico humano a consultar); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (7cb2218eb00107c9).

### Motor de partidas, 118ª leva — a postura do time (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `postura()` | 0x6df460 | thiscall | match_man.cpp (~3 KB): a postura que a comissão pede (1 ataque total … 5 retranca). Parte da força dos lados (com o motor rodando, +0x8ea5, a soma das notas 0x6e86d0 dos que estão em campo × 50 / 11; senão a reputação +0x80), da posse (+0x1fc/+0x206/+0x210), do tempo (+0x8ed0), dos expulsos (+0x1ad, 0x6cbbb0) e do placar agregado (0x6ce310); ajusta pela liga (posições na tabela por 0x4a4850 e datas por 0x54cdb0), pelo técnico (+0xe) e pelo fim do jogo (+0x8ed4 = 0x3de). Guarda o fator em motor+0x476a[lado] (10 e −1 nos extremos); com o motor rodando e o fator a menos de 0,5 do guardado, mantém a postura atual (+0x1ab) |

Mutações: 9 de 9 pegas (rodada conjunta 118ª–120ª); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (7cb2218eb00107c9). A 1ª base divergiu no fator do lado 1: no bloco da liga o expoente é 1,5 (0x968940) e não 2,0 (0x968950, a constante vizinha); achado com um rastro temporário dos valores intermediários.

### Motor de partidas, 119ª leva — a formação (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `escolher_formacao(fichas, n, modo, tipo)` | 0x6e0230 | thiscall, ret 0x10 | match_man.cpp (~2,8 KB): a comissão escolhe a formação do clube e carrega a tática. Com o motor em +0 sorteia; com `tipo` ≠ 2 só recarrega; com técnico humano a consultar (0x5fd210/0x5fea50) usa a tática dele (0x5fbe20/0x5fbd00). Senão dá nota (0x6e2f20) a cada setor (7) e faixa (5) pelas duas melhores das `n` pessoas (o goleiro só na faixa do meio; com menos de 11, o último setor vale metade e as pontas dos setores 1 a 3 ficam com a menor) e julga cada formação do banco de táticas (0xde1d08): tempo (+2, até 3000) e postura (0x6df460) pesam a nota da formação (até 3), o técnico ajusta (a favorita +0x43 vale ×1,1 ou até ×3,75; as outras ×0,1), cada vaga soma a nota do seu setor e faixa (0x5b2ae0) com as marcas (0x5b5270/0x5b57c0), e o equilíbrio dos setores entra pela postura. Grava a melhor (0x8967d0), copia a tática (0x896660) para o motor (+0x91e2 do lado) e para +0x2b e acerta as instruções (0x6ddab0). As notas ficam numa lista da CRT do jogo (0x945501/0x9452ca). Dentro de uma comparação com fita é unidade com vetores (a porta provisória da 115ª leva saiu) |

Mutações: 7 de 9 pegas (rodada conjunta 118ª–120ª); sobrevivem f_metade (só com menos de 11 pessoas) e f_escala (o piso do divisor comum a todas as formações não muda a vencedora: equivalente); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (7cb2218eb00107c9).

### Motor de partidas, 120ª leva — as instruções individuais (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `instrucoes()` | 0x6ddab0 | thiscall | match_man.cpp (~6,4 KB): a comissão acerta as instruções da tática copiada em +0x2b (0x5b50d0 gerais, 0x5b2ba0 por posição, 0x5b5290 marcas de 64 bits, 0x5b5910 marcação individual no bloco tático do motor) e no banco de táticas (0x897ee0). Com técnico humano a consultar não mexe. Em casa (ou com sorteio contra a diferença de reputação nas competições +0x3a = 1 ou 9) pede 0x20; o técnico (+0x13, +0x1f, +0x1c) escolhe ritmo e marcação. Um fator (reputações / 17000, técnico +0xe e o resto da soma dos ids por 13 − 0x544410/2) é comparado com os atributos (0x5431e0) de cada um dos 20 do lado em campo: avanço pelas pontas, marcação individual do adversário de 0x6d4330 (com 0x7c1e30 para os meias), passes, chutes de longe, dribles, desarmes, impedimento, cobranças (descontado o acumulado dos que têm +0x2e alto), cruzamentos por tabela e o cobrador de bola parada (0x897ee0 para quem está em +6/+0xa/+0xe da comissão). Peculiaridade preservada: o lado do marcado sai de +0x19 do jogador (posição 0 vira o outro lado) |

Mutações: 9 de 9 pegas (rodada conjunta 118ª–120ª); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (7cb2218eb00107c9).

### Motor de partidas, 121ª leva — as peças pequenas da comissão (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `colocar_na_vaga(pos, pessoa, extra)` | 0x6e0d00 | thiscall, ret 0xc | match_man.cpp: põe a pessoa na vaga 0..19 do bloco da comissão (+0x23) pela 0x6eaea0; sem pessoa devolve 0 |
| `ordem_por_tabela(a, b)` | 0x6e0df0 | cdecl | comparador do qsort da substituição: sem ficha vai para o fim, +0x19 = −2 e goleiro com +0xf < 20 vêm primeiro; senão a condição (+0x35 + +0x4d, truncada) pesada pela tabela de `a` (2 e 4: ×1,35; 8: ×1,2 em `a` e ×1,25 em `b`; 0x10: ×1,1) |
| `ordem_por_condicao(a, b)` | 0x6e0fe0 | cdecl | o outro comparador: as mesmas pontas e só a condição |
| `titulares()` | 0x6dbb80 | thiscall | match_man.cpp: para cada vaga de 0 a 10 + (partida +0x4d & 0xf), a pessoa da tática do clube (0x8968f0) entra (0x6e0d00) se puder jogar (0x62c400 ≠ 2, 0x55d270 e 0x7757d0 sem impedimento); devolve quantos |
| `guardar_tatica()` | 0x6dbb10 | thiscall, virtual | match_man.cpp: guarda a tática da comissão (+0x3f) no banco (0x8984d0); com técnico humano só quando 0x600a50 nega ou 0x5fd480 confirma |

Mutações: 2 de 8 pegas (rodada conjunta com a 122ª); sobrevivem c_goleiro (goleiro com +0xf < 20, raro), c_peso e c_oito (o comparador por tabela só entra com menos de 11 elegíveis) e c_total, c_estado e c_extra (os titulares só rodam para técnico humano que não está de férias: a carreira automática nunca passa ali; cobrir com uma carreira real comparando; em 12/10/2026 a carreira real do Lucas, com a escalação mexida e duas partidas assistidas, passou com 0 divergências). A 1ª base acusou colocar_na_vaga: a 0x6eaea0 (inicia o jogador, sorteia e mexe fora do motor) rodava duas vezes, na original e na cópia; virou unidade da fita (porta nova), e colocar_na_vaga e titulares passaram a ser comparadas pela fita. jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (7cb2218eb00107c9).

### Motor de partidas, 122ª leva — as trocas marcadas (oficial em 11/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `aplicar_trocas()` | 0x6d0b60 | thiscall | match_eng.cpp (~1 KB): aplica as trocas marcadas (+0x1d) dos dois lados. Com +0x1c o jogador sai (em campo: narra 0x1f56, 0x6cdd50 com −1 ou −2 conforme +0x9049, +0x1a = posição e +0x19 = −2; com −3: narra 0x1f57 e 0x6cdd50 com 0) e o minuto vai para +0x22; sem +0x1c narra 0x1f58 por sorteio(4) ou com +0x55; com +0x55 narra 0x20f6 com o lado de cada um (0x714d30). Havendo marcados e fora dos jogos com técnico humano que não delega: com +0x9049 a comissão substitui (0x6e1090); senão refaz o time (0x6e1aa0), dá a braçadeira (0x897760, o mais bem avaliado de 0x6e1a40) se o capitão (0x897590) saiu, zera +0xf59e/+0xf5a2 e recompõe (0x6ce110). Guarda o minuto (+0x8ecc), refaz a força do lado (0x6bbb60) e a marcação (0x6bbf60); no fim zera +0x8eb5. Comparada pela fita (narração, comissão e capitão como unidades); o 6º argumento da narração 0x20f6 vai no byte 1 do 5º (a 0x6d6f70 lê os dois como bytes) |

Mutações: 6 de 7 pegas (rodada conjunta com a 121ª); sobrevive a_capitao (só muda com o capitão na vaga 0, a do goleiro); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (7cb2218eb00107c9). A 0x6db500 (comissão no início da partida) e uma porta na 0x6dbc50 chegaram a passar nas mutações e no jogo real, mas mudaram as carreiras (ver Como continuar) e saíram da fila.

### Motor de partidas, 123ª leva — as punições pendentes (oficial em 12/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `punicoes(primeiro)` | 0x6d1210 | thiscall, ret 4 | match_eng.cpp (~1,4 KB): na 1ª chamada (+0x8eb4 = 0) prepara `primeiro` (0x716630) e marca +0x8eb4 = 1 se houver pendentes. Depois aplica cada um dos +0x1a3 lances pendentes (11 bytes a partir de +0xb1: jogador, outro, severidade, código, x): a punição do árbitro (0x6ea4e0) dá o cartão; tipo 2 narra 0x1ff9, 3 e 4 (expulsão) narram 0x1ff7/0x1ff8 e marcam +0x1cb do lado (goleiro: tabela 1); sem cartão, severidade acima de sorteio(7) + 2 com o jogador em campo narra 0x210a; narrando, antes vêm 0x20f5 (sorteio(severidade) > sorteio(20)) e 0x2109. Fora dos códigos 0x25, 0x2a–0x32, 0x60 e 0x61: por sorteio(9) narra 0x20df (0x20e0 por sorteio(3); tipos 3 a 6 marcam +0x16 e 0x20d9 por sorteio(7)), passa o peso da punição aos do lado (0x716450: o próprio, os em campo (p + 1) / 2, os de fora p / 6 + 0,83) e 0x716e10 (1 na expulsão); expulsão tira o jogador (0x6cda20). No fim, por lado marcado e fora dos jogos com técnico humano que não delega: com +0x9049 a comissão troca o goleiro (0x6e1090 com 1), senão refaz o time (0x6e1aa0), dá a braçadeira ao mais bem avaliado (0x6e1a40/0x897760), zera +0xf59e/+0xf5a2 e recompõe (0x6ce110); guarda o minuto e refaz força e marcação. Termina com +0x1a3 = 0 e +0x8eb4 = 2. Comparada pela fita (narração, comissão e capitão; a 0x6cda20 entra pela `jogador_saiu_com`) |

Mutações: 8 de 8 pegas (rodada conjunta com a 124ª); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (7cb2218eb00107c9). A 0x716630 e a 0x716e10 narram por dentro: ganharam portas que só gravam na fita durante esta comparação.

### Motor de partidas, 124ª leva — a disputa de pênaltis (oficial em 12/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `penaltis(modo)` | 0x6be710 | thiscall, ret 4 | match_eng.cpp (~1,9 KB): um passo da disputa pelo estado +0x8eb3. 1 e 4 preparam (zeram o lance, guardam e calam o narrador em +0x8ea4, cinco cobranças por lado em +0x8ea1/+0x8ea2, começa o lado 1 ou o sorteado, 0x6be520) e vão para 2 e 5. 2 e 5: sem cobranças sobrando vem a morte súbita (+0x8ea3); o batedor é o da vez na lista do lado (+0x1af + 11 × lado, +0x8e9f conta), o goleiro o do outro lado (0x6ce110); marca quem bateu (+0x8eda), a força sai de sorteio(+0x35 da ficha) e das cobranças que faltam e cresce (×2,25 com a narração 0x2005 quando o lado precisa marcar, ×0,75 com 0x2006 quando pode decidir). Com `modo` 3 o chute (0x700bd0, 0x702050) sai direto; senão posiciona batedor, goleiro e os outros (0x6f46b0, 0x7160e0), a direção sai da força e o goleiro reage (0x714d40, até 4 vezes 0x7109f0) antes do chute. Passa a vez e, decidido o placar (+0xf5bc × +0xf5f2 com as cobranças que faltam), vai para 3 (modo 4) ou 6. 3 e 6 narram o resultado (0x2004), gravam o placar dos pênaltis na partida (+0x49/+0x4a) e devolvem o narrador. Comparada pela fita: narração, cobrança (0x702050, porta nova que só grava nesta comparação), posicionamento, domínio (0x6f0630) e reação do goleiro são unidades. As carreiras automáticas só usam o chute direto (`modo` 3): o ramo com posicionamento e reação do goleiro (as mutações n_direcao e n_reacao sobrevivem) e a 11ª cobrança de um lado (n_vez) esperam uma partida assistida |

Mutações: 5 de 8 pegas (rodada conjunta com a 123ª); sobrevivem n_direcao e n_reacao (ramo da partida assistida, que as carreiras automáticas não usam) e n_vez (só na 11ª cobrança de um lado); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (7cb2218eb00107c9).

### Motor de partidas, 125ª leva — a falta marcada (oficial em 12/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `aplicar_falta()` | 0x6bac40 | thiscall, devolve al | match_eng.cpp (~3,2 KB): aplica o lance pendente em +0x475a (tipo; +0x4761 quem fez, +0x4765 quem sofreu, +0x475b/+0x475c onde, +0x475f o código). Sem tipo devolve 0; com bola parada em andamento e o lance atrás da posição marcada (+0x8ea8) do lado que tinha a vantagem, descarta e devolve 0 (senão narra 0x20f0). Tipo 8: narra 0x20ee, conta no placar do lado (+0xf5ca) e no jogador (+0x13), bola parada 7. Sem quem sofreu: punição da falta (0x6e9630) e 0x20fb; com ele: punição do lance (0x6ea4e0) e a narração pelo código (0x20fc a 0x2107). O tipo devolvido escolhe o cartão (0x1ff5 a 0x1ff9, ou 0x210a pela severidade); perto da área adversária vira pênalti (+0x8eb2 = 5, 0x1f7a, −500 em +0x35), senão falta (6, 0x1f79). Com cartão, 1 em 4 narra 0x2109 (e 0x20f5), narra o cartão e a expulsão tira o jogador (0x6cda20). Fora dos códigos de bola parada: narrações 0x20df/0x20e0/0x20d9 por sorteio, o time de quem fez se desgasta pelo peso da punição (0x716450) e quem fez cansa (0x716e10). Limpa o lance e devolve 1. Comparada pela fita: narração e cansaço (0x716e10) são unidades |

Mutações: 9 de 12 pegas; sobrevivem f_vazio (quem chama só chama com lance pendente), f_marcada (lance exatamente na posição marcada durante bola parada) e f_parada (código de bola parada 0x32, raro); p_achou, do comparador novo de procurar_no_lado, é pega; jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (7cb2218eb00107c9). A 1ª base acusou procurar_no_lado no dia 271: o comparador antigo reproduzia as respostas da 0x6f5c30 sem os efeitos dela; agora compara pela fita (ver Como continuar).

### Motor de partidas, 126ª leva — a dividida (oficial em 12/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `dividida(forte)` | 0x70ec80 | thiscall, ret 4 | match_pl.cpp (~5,9 KB, chamada pela 0x7109f0): a dividida contra o dono da bola. Sorteia pelo fôlego (+0x4d) e só segue com `j` na posse (+0xf582); com `forte` o lado adversário ganha 2,25 (0x6bb950) e o de `j` +10 de posse. O ataque é sorteio(+0x19c − fôlego + +0x13a − sorteio(+0x8ea9 × 50)); o desafiante, a nota e o lance saem da 0x6c87e0 (lance 0x10), e a lesão na dividida (0x6cfe10) vem logo. Sem desafiante ou com o ataque acima da nota, `j` passa: fora da área pode crescer (+0x1a2 × 1,05, 0x211d), conta a posse e narra 0x2133/0x216d/0x21de/0x1f40. Senão o árbitro decide (0x6cfc60): falta com punição (0x6cfcd0, narração pelo lance, cartão 0x1ff6 a 0x1ff9 ou 0x210a, bola parada 7, pênalti 5 ou falta 6, cobrador 0x6ceb90 e narrações 0x21df a 0x21e2, expulsão 0x6cda20, desgaste do time 0x6bb8d0, cansaço) ou disputa limpa (o desafiante marca o lance, perde 59/70 de fôlego, narra o lance ou defende 0x70d430, domina 0x6f0630 com +0x19c = nota − ataque − 200; abaixo do limite do lance a bola espirra 0x6cd770 e ele cai, ou `j` sofre 0x716c90 com pênalti marcado; senão vira para o lance 0x6f4920 e pode narrar 0x21cd/0x21ce pela média de acertos; ganha fôlego e 1 em 7 narra 0x1f41). Comparada pela fita: narração, cansaço, 0x716c90 (porta nova pela mesma chave da 0x716e10), domínio, bola que espirra e as unidades da lesão, da defesa e da saída de campo |

Mutações: 12 de 12 pegas; jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (7cb2218eb00107c9). A 1ª base acusou a decisão do árbitro no dia 262: as comparações deixavam de fora o fim do motor (ver Como continuar); agora todas usam o tamanho alocado, 0xf63b.

### Motor de partidas, 127ª leva — a disputa de bola (oficial em 12/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `disputa(x, y)` | 0x6c9cd0 | thiscall, ret 8, devolve o vencedor | match_eng.cpp (~6,5 KB, chamada pela 0x6b98c0): a bola solta na casa (x, y) disputada pelos desafiantes dos dois lados (0x6c87e0, lance 0x15; a nota corrigida pelo ângulo da direção contra +0x8ed8). Fora do campo é bola fora (0x6cd490); sem desafiante, guarda a casa e devolve 0. O de nota maior ganha; a lesão na dividida (0x6cfe10) vem logo. Com os dois, o árbitro olha a falta do vencedor (0x6e9820, +4 de fôlego antes, 0x2168 na área) e depois a do perdedor; sem falta, a 0x6e9540 pode marcar uma. A falta vira lance pendente (+0x475a) por sorteio(distância ao gol × 10 + 2) = 0 sem risco, ou é punida na hora (0x6ea4e0 ou 0x6e9630): narração pelo lance (0x1fe8 a 0x1ff4), cartão, bola parada 7, pênalti 5 ou falta 6, expulsão (0x6cda20), narrações 0x20df/0x20e0/0x20d9, desgaste do time de quem fez (0x716450) e cansaço; os lances especiais saem antes do desgaste. Sem falta, a disputa limpa: no lance 1 a defesa (0x70d430); com os dois, a leitura (+0xad: 0x2164, 0x214f ou 0x1f5d) e as tentativas (+7, +8, +9 com perigo 0x70d630); o vencedor marca o lance, vira para a jogada (0x6f4920), domina (0x6f0630) com +0x19c = nota − nota do outro − 225 (±200), o perdedor soma o 2º byte do desafio a +0x2b (0x7160d0) e o vencedor segue (0x7103e0). As duas metades (falta do vencedor e do perdedor) não são simétricas no lado narrado nem em quem recebe +0x16. Comparada pela fita: narração, cansaço, bola fora, domínio, o lance do vencedor e as unidades da lesão, da defesa e da saída de campo |

Mutações: 9 de 11 rodadas pegas; t_fora pulada (o ramo da bola fora do campo não rodou na base: lacuna medida); sobrevivem t_angulo (só com a diferença de ângulo exatamente 180) e t_defesa (a defesa com os dois desafiantes, força ±1); jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (7cb2218eb00107c9).

### Motor de partidas, 128ª leva — a bola que chega ao jogador (oficial em 12/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `responder(forca, tipo, altura, conta)` | 0x6f5c30 | thiscall, ret 0x10, devolve 0/1 | match_pl.cpp (~3,9 KB, chamada pela 0x6cd330 `procurar_no_lado`): o jogador disputa a bola que chega. Quem cobra (+0xf576) não disputa; do lado sem a bola, com +0x10d × 2 < sorteio(6) e a linha fora do limite (0x6cb930), também não. O ataque é sorteio(2000 − +0xa5 × 40) para o goleiro na área, senão pelo tipo (500, 600, 700, 1000, 1800; o 5 vale sorteio(500000) + 200 e vira 4); com altura < 1,5 os tipos 4/5 não disputam; com altura, ataque × (+0x8eaa + 1) / (altura + 1). Com a bola no próprio lado (+0x8eab), 0x1f5e e a bola espirra. Senão o lance e o atributo (domínio 1 com +0x11c, 0x15 +0x144, 0x24 +0x162, 0x29 +0x16c, cabeçada 0x25 +0x164) dão a nota = sorteio(atributo) − força / 2 − ataque. O árbitro olha a falta (0x6cfca0): 1 em 10 a registra (0x6bb9a0), senão pune (0x6cfd10, 0x1fe8, cartão, pênalti com ajuste do +0x17 e desempenho 0x6eecb0, cobrador, 0x21df a 0x21e2, desgaste 0x6bb8d0, cansaço). Sem punição, com a nota no limite (1500 − altura² × 100 + +0x8eaa × 100, mínimo 160) o jogador domina (0x1fe2/0x1f5f, 0x6f0630, 0x21cd/0x21ce); senão a bola espirra (0x20d4/0x213e/0x1f71, 0x6cd770). Na comparação o desvio é a própria porta da unidade (que só grava dentro do `procurar_no_lado`); no modo Rust entra a cópia. Comparada pela fita: narração, cansaço, bola que espirra, domínio e as unidades da saída de campo |

Mutações: 12 de 12 pegas; jogo real até 2002-01-01 com 0 divergências; carreiras Rust ×2 e original iguais (7cb2218eb00107c9). A 1ª base não exercitou a cópia Rust (a contagem de ramos acusou zero): a rotina só roda como unidade dentro do procurar_no_lado; agora a porta a compara dentro da unidade (comparação aninhada).

### Motor de partidas, 129ª leva — a jogada (oficial em 13/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `jogada(forca, altura, estilo, forte)` | 0x70dfd0 | thiscall, ret 0x10, devolve 0/1 | match_pl.cpp (~7 KB, chamada pela 0x6cd3d0 `procurar_jogada`): o jogador tenta a jogada contra a força do adversário. O goleiro do lado (0x6ce110), quem cobra (+0xf582) e, do lado sem a bola, quem tem +0x10d zerado não jogam. O ataque é sorteio pelo +0x8ea9 (300, 400, 500, 1000, 1800) dividido por altura + 1. Com a bola no próprio lado (+0x8eab), 0x1f5e e a bola espirra. O lance e o atributo seguem a 0x6f5c30, mas pelo +0x8ea9; a cabeçada 0x25 vale o dobro do +0x164 e, pelo alto, o atributo vale 85%. Nota = sorteio(atributo) − força / 3 − ataque. Falta forte: força > sorteio(150) + 200. O árbitro olha a falta (0x6cfca0): 1 em 5, sem falta forte, a registra e segue; senão pune como na 0x6f5c30 (o pênalti só tira 500 do +0x35). Nota de 400 vence (0x1ffe/0x21d6/0x1f66/0x1f65, 0x6f0630); senão o goleiro narra 0x1fe7, da meia-lua com nota > 200 0x1ffd e a bola espirra sem chutador, ou 0x21d8/0x1f85/0x1f84 e a bola espirra. O 7º argumento da narração (o estilo) vai no alto do código. Na comparação o desvio é a própria porta da unidade, que compara dentro dela; no modo Rust entra a cópia. Comparada pela fita, com o 7º argumento da narração |

Mutações: `mutacoes/leva129.py` (14, até 2001-10-01): 12 pegas; escaparam `j_falta_forte` (fraca: só um empate de sorteio a muda) e `j_estilo` (a narração 0x1f65 não rodou na janela). Reforços: `leva129b.py` (`j_estilo2` na 0x1f66, pega: a fita guardou 0x51f66 e a cópia mandou 0x61f66; `j_falta_forte2` com limite 100 escapou: só 3 faltas na janela) e `leva129c.py` (`j_falta_forte3` sempre ligada, pega). O ramo `jogada goleiro` (0x1fe7) é inalcançável: o goleiro sai no início. Jogo real comparando até 2002-01-01: 0 divergências; carreiras Rust ×2 e original: 7cb2218eb00107c9

### Motor de partidas, 130ª leva — os eventos do pós-jogo (oficial em 13/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `eventos_do_pos_jogo(partida)` | 0x6b74a0 | cdecl | match_day.cpp (chamada em 0x6b60c2, depois da 0x6b6cc0): percorre a lista de eventos em [0xb67dc8] (cursor reposto; 0x68d1b0) e trata os dos dois clubes da partida que não estão fora (0x53b900). Evento: +0 pessoa, +4 clube, +8 marca, +0x13 tipo, +0x15 outra pessoa. 0x82: 0x759610. 0x13: sem marca tira os bits 0x40 e 0x2000000 do contrato (0x4cf5f0); com marca, notícia 0x72c1f0 e 0x2000000, ou o 0x40 por sorteio(+0x5b). 0x0a: 0x4000000 e, por sorteio, a notícia se +0xb da ficha > sorteio(2000) + 4500. 0x0b: 0x7daf50. 0x1e: 0x40000 por sorteio, 0x733a60 e −50 (com 0x547af0) ou −75/−75 nas palavras +0xa/+0xc da linha do clube em [[0xb67a44]] (entre 1 e 10000). Os outros: 0x40000 por sorteio(+0x5a) < sorteio(15/8/6 pela idade) e a notícia. **Sem comparação lado a lado** (chama 14 rotinas do banco com efeitos): no modo comparação fica a original; confere-se pela carreira, com o banco igual ao do original em todos os dias |

Mutações: `mutacoes/leva130.py` (11, pela carreira até 2002-06-01): 7 pegas (`e_0b` derruba o jogo). Escaparam `e_0a_sorteio` e `e_0a_limite` (o tipo 0x0a só rodou 2 vezes, sem virada), `e_1e_tecnico` e `e_1e_sem`. Com a impressão diária do banco (`leva130b.py`) a `e_1e_tecnico` passou a ser pega (a tabela de [[0xb67a44]] muda e é apagada depois); a `e_1e_sem` não: o jogo repõe a linha em 5000 no mesmo dia (medido: 5000 → 4925 → 5000). Reforços (`leva130c.py`): `e_0a_limite2` (sem os 4500) pega; `e_1e_sem2` (soma em vez de subtração) escapa pelo mesmo motivo: lacuna conhecida, só a comparação lado a lado a veria. Ramos medidos até 2002-06-01: outros 16+, 0x1e 4+ (técnico 2+, sem técnico 1), 0x0a 2, 0x0b 2, 0x13 marca 2; nunca rodaram 0x82, 0x13 sem marca e 0x13 notícia (nem até 2003-06-01 com a semente 777). Jogo real comparando até 2002-01-01: 0 divergências; carreiras Rust ×2 e original: 7cb2218eb00107c9, partidas 0ac236ae64589294, banco 923a4e0edd9f2e09

### Motor de partidas, 131ª leva — os jogadores no pós-jogo (oficial em 13/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `jogadores_do_pos_jogo(partida, t)` | 0x6b6cc0 | cdecl | match_day.cpp (chamada em 0x6b60bc, antes da 0x6b74a0): para os dois lados do registro `t` (clube em t+0/t+4; lado em t + lado × 0x625), os 20 jogadores (linhas de 0x49 a partir de +0x1c5; pessoa em −0x35) com ficha (+0x61) e da base. Na competição da partida, com o clube dela e sem +0x4c: por contrato 0x10/0x20/0x30, 0x849ce0/0x600a50, a nota +3 acima de 5000 e de 1,1 × +0x18e (ou +7 acima de 0,9 ×, ou +5 da ficha acima de 0,024 ×), +0 até 6, o placar do lado não acima do outro, sorteio(10) + 5 < nota 1 (0x4a31a0), sorteio(10) acima de +0x45 da ficha e a nota 0x11 a menos de 0,2 da média da competição ([0xb74350]+0x10/+0x18, ou 6,8): a notícia 0x72c1f0 tipo 0x11. Com 0x76d940, +0 de 9 em diante, o placar não abaixo, sorteio(5) + 2 < nota 1 e a nota 0x11 acima de sorteio(9) × 0,1 + média + 0,5: 0x744240 (0x34), uma por partida. Copia −4, +3, +5, +7 da linha para +0x45, +0xb, +9, +0xd da ficha. Mesma nação nos dois clubes e −8 na linha: com 0x7c1e30 não zero e as condições de clube anterior, ficha e pessoa, o 0x20 no contrato (sorteio(3) do próprio clube [0x9cf300] ou sorteio(10)). O técnico (+0x184) recebe +0x188/+0x18a/+0x18c na ficha (+0x69) e o auxiliar (+0xd3) e os 5 de +0x19f um quarto da diferença (entre 1 e 10000); +0x80 do clube = +0x18e. **Sem comparação lado a lado**: no modo comparação fica a original; confere-se pela carreira (banco igual dia a dia) |

Mutações: `mutacoes/leva131.py` (12, pela carreira até 2002-06-01): 11 pegas — as três da ficha (+0x45, +0xb, +9), o técnico, os auxiliares, o limite 10000 (`j_limite` só pela impressão do banco), o +0x80 do clube e as notícias 0x11/0x34. Escapou `j_n34` (limite 0,1 menor); o reforço `leva131b.py` (`j_n34_forte`, 1,0 menor) pega. Ramos até 2002-06-01: técnico 8192+, notícia 0x11 8+, 0x34 4+; a marca 0x20 só roda na carreira longa (semente 777 até 2003-06-01: 2+), onde as três impressões também batem (banco 9a32333b47dbbdcd). Jogo real comparando até 2002-01-01: 0 divergências; carreiras Rust ×2 e original: 7cb2218eb00107c9, partidas 0ac236ae64589294, banco 923a4e0edd9f2e09

### Motor de partidas, 132ª leva — o clube no pós-jogo (oficial em 13/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `clube_do_pos_jogo(clube, outro, partida, n, t)` | 0x6b79a0 | cdecl | match_day.cpp (chamada duas vezes em 0x6b61a9/0x6b61bd, uma por clube): sem clube, outro ou partida, caixa de erro. Sem técnico (+0xcf), nada; sem 0x600a50, com +0x80 abaixo de 4750 e sorteio(100) ≠ 0, nada. Com 0x6b7cf0, partida +0x40/+0x41 e o clube não fora (0x53b900): entre os 50 de +0xd7 com contrato (0x4cf5a0) sem +0x45, +0xb da ficha acima de 4250, +0x59 > 5, +0x57 > 15, +0x58 > 10, `n` > 2 e sorteio(n) ≠ 0: 0x5896d0(pessoa, clube, outro, n) e fim. Depois os 20 da linha do clube no registro `t` (+0x1ce, pessoa em −0x3e, só com +0 na linha): ficha, da base, idade até 20, contrato sem +0x45, a partida da competição do clube, +0x59 > 15, +0x57 acima de sorteio(5) + 10, +0xb da ficha abaixo de 4250 e 0x7c3a40 zero: 0x589bc0 e, sem 0x547d60, 0x547d90(pessoa, clube, 1). **Sem comparação lado a lado**: no modo comparação fica a original; confere-se pela carreira |

Mutações: `mutacoes/leva132.py` (11, pela carreira até 2002-06-01): 6 pegas (sorteio(100), +0x80, técnico, idade, +0x59, lado); `c_acao` pulada (o ramo da 0x547d90 não roda). Escaparam `c_57`, `c_ficha`, `c_n` e `c_elenco57`: filtros logo antes das ações finais, que nunca disparam nas carreiras automáticas (nem até 2003-06-01 com a semente 777; o corpo roda — `clube segue` 4096+, `clube elenco` 32768+, `clube jovem` 512+). Reforços (`leva132b.py`) que forçam as ações: `c_589bc0_forcada` e `c_5896d0_forcada` pegas — as impressões veem esses caminhos. Lacuna medida: 0x5896d0, 0x589bc0 e 0x547d90 nunca rodam. Jogo real comparando até 2002-01-01: 0 divergências; carreiras Rust ×2 e original: 7cb2218eb00107c9, partidas 0ac236ae64589294, banco 923a4e0edd9f2e09; semente 777 até 2003-06-01 também igual

### Motor de partidas, 133ª leva — a partida sem o humano (oficial em 13/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `partida_sem_humano(partida)` | 0x6b66a0 | cdecl, devolve 0/1 | match_day.cpp (chamada em 0x6b4ae3 e 0x6b5c9a): 1 se a partida não envolve o humano, pela 0x7f9720. Antes, uma vez ([0xa15704]), marca as seleções dos humanos em 0xb67a58 (vagas de 0x48 em 0xb63d60 com +0x11c bit 2 e seleção +0x71). Sem [0xdd7118]: 1. Devolve 0 se um dos clubes (+0x1c, +0x20) é de humano (0x600a50) ou o clube ligado a ele (0x540a50); se a competição (+0) é [0x9cf7bc] ou [0x9cf6e4]; se é a [0x9cf958] com um dos clubes [0x9cf2e4]/[0x9cf510] humano; se um dos 16 humanos no fim da tabela de pessoas (+0x5f = 1) treina clube da mesma nação (+0x57). Na [0x9cf684] sem clube humano, direto à 0x7f9720. Fora de seleção (0x53b900) com 0x5c2990, ou na [0x9cf6f8]: 0 se o +0x80 dos dois somar mais de 12000. Senão a vaga da 0x48dd70 (tabela em 0xb63d74): 0 ou 1 dá 0; 3 a 7 com a seleção da competição marcada, 0; vaga de humano com +0x11c bit 2, 0. Comparação lado a lado (resposta e as marcas) |

Mutações: `mutacoes/leva133.py` (12, pela carreira até 2002-06-01, com confirmação no modo comparação até 2001-10-01): 7 pegas (`s_flag`, `s_684`, `s_5f`, `s_80`, `s_vaga`, `s_marcada`, `s_final`); `s_ligado` pulada (o ramo do clube ligado não roda). Escaparam nos dois modos `s_humano`, `s_comp1`, `s_958` e `s_16`: condições sempre falsas nas carreiras automáticas (nenhum clube humano nas partidas, competições fixas que não aparecem, o 16º humano nunca técnico de seleção) — só uma carreira do Lucas as alcança. Ramos até 2002-06-01: competição direta 1024+, técnico da nação 512+, seleção marcada 256+, pela reputação 128+, 0x7f9720 2+. Jogo real comparando até 2002-01-01: 0 divergências; carreiras Rust ×2 e original: 7cb2218eb00107c9, partidas 0ac236ae64589294, banco 923a4e0edd9f2e09

### Motor de partidas, 134ª leva — o interesse do humano no dia de jogos (oficial em 13/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `interesse_do_dia(d)` | 0x6b5040 | thiscall, devolve 1 | match_day.cpp (chamada pela human_manager.cpp e pela match_day.cpp): marca no dia de jogos `d` o que interessa ao humano. Primeiro a lista dos humanos (0x5fe8d0 em [0xb63c98]); zera +0x29, +0x2a, +0x2b. Para cada grupo (+1, de 0x18 em 0x18, quantos em +9) e cada competição dele (+4 a +8, em +0xd de 0x54 em 0x54): a competição interessa (+0xc) se um dos 16 humanos no fim da tabela de pessoas tem o clube (+0x39) ou o outro vínculo (+0x24) nela (0x48db80). Cada partida (+4 a +8, em +0x19 de 0x69 em 0x69): com clube humano (0x600a50 com +0x20/+0x44 ou +0x24/+0x45) ou pela 0x5fe420, é marcada (+0x53) e conta o grupo (+0x29) e a competição (+0x2a) na primeira vez; senão, pela 0x6b61e0, só a competição interessa. Qualquer marca liga +0x2b. Comparação lado a lado (todas as marcas do dia) |

Mutações: `mutacoes/leva134.py` (11, pela carreira até 2002-06-01, com confirmação no modo comparação até 2001-10-01): 4 pegas — `i_zera29` (só pela comparação), `i_2b` e `i_6b61e0` (o jogo para numa tela esperando o humano e estoura o limite), `i_comp_zera`; `i_marcada` pulada (nenhuma partida tem clube humano nas carreiras automáticas). Escaparam nos dois modos `i_16`, `i_k`, `i_outro`, `i_byte44`, `i_5fe420` (só a vaga do técnico de férias está ocupada entre os 16 humanos; nenhum clube humano em campo) e `i_marca_comp` (a competição que ela deixaria de marcar é marcada depois pela 0x6b61e0). Ramos até 2002-06-01: pela 0x6b61e0 1024+, humano na competição 128+. Jogo real comparando até 2002-01-01: 0 divergências; carreiras Rust ×2 e original: 7cb2218eb00107c9, partidas 0ac236ae64589294, banco 923a4e0edd9f2e09

### Motor de partidas, 135ª leva — o reinício do lance (oficial em 13/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `reiniciar_lance()` | 0x6c54a0 | thiscall | match_eng.cpp (3,2 KB, chamada pela 0x6b98c0): zera o estado do lance (+0x1c5, +0x4782.., +0x8ea9/+0x8eaa, +0xf576.., o lance pendente +0x475a..) e, sem +0x1a8: os 15 minutos desde a última mudança de cada lado (+0x1d2/+0x1d4); depois dos 60 minutos, 1 em 6, o primeiro jogador sem fôlego (+0x4d < 6750) narrado (0x21cf); a postura de cada lado (0x6df460) guardada em +0x1ab com a anterior; com postura 3/4 de quem tem a bola, sorteio(10) de acréscimo (+0x8ed0, metade em +0x8eb7) e o cansaço do outro lado na postura 1 (0x716450); a substituição da IA (0x6e1090 em +0x9032 + 0xd0 × lado; sem humano, ou humano automático) com minuto > sorteio(50) + 10 e 55 de jogo desde a última, 0x6bbb60 e 0x6bbf60; o acréscimo narrado (0x21c1) perto do fim de cada tempo; a tática da IA pela postura (marcas 0x5b50d0 no objeto +0x91e2 + 0x18e3 × lado, pelo +0xe/+0x13 do técnico, a formação +0x905d e +0x476a; narrações 0x2135 a 0x213d, 0x6d10a0, a marca 0x400000 das casas); com +0x8ebb os cantos da torcida (placar 0x6ce310, clubes fixos [0x9d0230].., nação +0x76, "Ruud Gullit" 0x84b8b0); e a bola parada por +0x8eb2 (lateral, escanteio, saída, pênalti, falta, pontapé inicial). O pseudocódigo do Ghidra erra as posturas 2 e 3 ("anterior 3/4/5" e "anterior 1/2" viram "== true"): portado da desmontagem. Comparada pela fita (as bolas paradas são unidades com `GRAVAR_REINICIO`, porque reposicionam jogadores pela 0x6f09f0) |

Mutações: `mutacoes/leva135.py` (14, pela carreira até 2002-06-01, confirmação no modo comparação até 2001-10-01): 12 pegas — `r_acresc_narr` só pela comparação (o 7º argumento da narração 0x21c1); `r_canto` pulada (os cantos da torcida só rodam em partida narrada, +0x8ebb); escapou `r_postura` (minuto ≤ 2 no lugar de ≤ 1: no minuto 2 a postura calculada é a guardada). Ramos até 2002-06-01: substituição da IA 32768+, acréscimo 16384+, sem fôlego 8192+, posturas 1/2 4096+, 3 2048+, 4/5 1024+, acréscimo narrado 4096+. Jogo real comparando até 2002-01-01: 0 divergências. Carreiras na pasta de testes `jogo_testes` (nova referência, ver COMO_CONTINUAR): Rust ×2 e original 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 136ª leva — o chute (oficial em 15/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `chute()` | 0x6f6c00 | thiscall, ret 0xc | match_pl.cpp (12,7 KB, chamada só pela 0x6b98c0 com a força +0x8ed6 e o alvo +0x1c8/+0x1c9): a chance pela distância ao gol (tabela 0xa15e44 no lado 0, 0x6f09b0 no lado 1), o fôlego, o ângulo da direção (+0x19a contra +0x8ed8) pesado pelo lance +0x8ea9, a força menos 20 × (+0x8eaa − 1)³ (e mais pelo byte 1 do motor); com ângulo até 90 vira para o gol ou para o alvo (0x6f4920). O lance e o atributo: cabeçada 0x25 (+0x164); com +0x8ea9 4+ o 0xc de longe (força e caminho livre 0x6c41b0, +0x8ea9 vira 2), 0x2e (+0x176) ou 0x15 (+0x144); com 2/3 o 0x3b (+0x190) perto do gol, senão 0xc (+0x132); os outros 0xd (+0x134). Nota = sorteio(atributo) − sorteio(fôlego) − sorteio(ângulo) + sorteio(força). Três faltas possíveis: a do chutador antes (0x6cfca0; registrada por sorteio(chance × 5 + 2), senão punida como na 0x6f5c30), a do desafiante (0x6c87e0, lesão 0x6cfe10; punida como na dividida e com ajuste do +0x17 no pênalti) e a do chutador sobre ele (narração pela tabela do lance, pênalti marcado para o desafiante). O desafiante com a nota maior fica com a bola (falta narrada na área, defesa do goleiro 0x70d430 ou a narração do lance; a bola espirra abaixo do limite; contas, fôlego, +0x17; 1 em 7 0x1f41). Senão o chute segue: +0x4782/+0x4786 perdem a nota × 2/11; de cabeça 0x2164/0x21cb/0x214f/0x1f5d e 0x7103e0; senão 0x216e ou as narrações do chute de longe (0x21cc, 0x21db, 0x21dc, 0x1f47), +0x19c ganha a diferença (±200) e por sorteio 0x7103e0 (com +0x1a7 e +100 de posse); fora da área de ataque o jogador pode crescer (0x211d). A falta punida da dividida virou função comum (`punir_desafiante_com`); a do chutador (`punir_falta_de_quem_disputa_com`) repete a da 0x6f5c30, que fica com a sua cópia em linha (ver COMO_CONTINUAR). Comparada pela fita |

Mutações: `mutacoes/leva136.py` (17, pela carreira até 2002-06-01, confirmação no modo comparação até 2001-10-01): todas pegas. A `c_3b` original (chance < 1,5 no lugar de < 1,41) era equivalente — a tabela 0xa15e44 não tem valor entre 1,41 e 1,5 — e foi refeita (< 1,4, tira o 1,41 da diagonal). Comparando até 2001-08-20 o chute passou por todos os ramos marcados (65.536+ chutes sem desafiante, 8.192+ com o desafiante ganhando, faltas registradas e punidas dos dois lados, pênalti, expulsão, defesa do goleiro, 0x716c90 uma vez). Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 137ª leva — a nota do árbitro (oficial em 15/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `nota_do_arbitro()` | 0x6d4a20 | thiscall | match_eng.cpp (336 bytes, chamada pela 0x6b98c0): o árbitro da partida (+0xf5b6, id ≠ −1), fora das competições amistosas (0x5c6710 em [0xae2c74]). Cartões (+0xf5cb e +0xf601 dos dois lados) e faltas (+0xf5cc e +0xf602); a nota parte de +0xf5ba / 1000, perde 2 com mais de 2 cartões, 1 com 2, ganha 1 sem cartão; perde 1 com mais de 5 faltas, ganha 1 com menos de 3; abaixo de 4 vira 4 (salvo 1 em 500), acima de 8 vira 8 (salvo 1 em 25), e fica entre 1 e 10. Grava pela 0x79ce10 em [0xb74340] (registro de 8 bytes: id, cartões, faltas, nota; o 8º byte é lixo de pilha e não é lido). Comparada lado a lado (sorteio e ficha do árbitro) |
| `registrar_nota_do_arbitro()` | 0x79ce10 | thiscall, ret 4 | officials_manager.cpp (208 bytes): soma a nota na ficha do árbitro (17 bytes em [this] + id × 17, id em 0..[0xae2390]): +0xf os cartões, as palavras +8 e +0xa as faltas e a nota, +0x10 a última nota, +0xc os jogos e +4 a média (+0xa / +0xc, float). Comparada lado a lado (a ficha) |

Mutações: `mutacoes/leva137.py` (9, pela carreira até 2002-06-01, confirmação no modo comparação até 2001-10-01): todas pegas — 3 pela carreira (`a_cartoes`, `a_sem_cartao`, `a_faltas`: mudam quantas notas caem abaixo de 4 e com isso os sorteios) e 6 só pela comparação (`a_minuto`, `a_quatro`, `a_oito`, `a_cartoes_ficha`, `a_media`, `a_ultima`: a ficha do árbitro não pesa na carreira de 5 meses). A 0x5c6710 (competição amistosa), que a nota consulta, já era Rust desde a 48ª leva (`e_amistoso`). Comparando até 2001-08-20: 0 divergências; os ramos `nota do arbitro minima`/`maxima` não rodaram (nota abaixo de 1 ou acima de 10 é rara). Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 138ª leva — o batedor guardado, as instruções guardadas e o número da competição (oficial em 15/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `batedor_guardado()` | 0x897990 | thiscall, ret 0x10 | tactics.cpp (544 bytes, chamada pela escolha do cobrador 0x6ceb90): quem bate a bola parada `tipo` (0..4) no `lado` (0..1): a célula (dword) na linha tática do clube (0x8982b0) em +0x601 + linha × 0x91 + 0x5e + (lado + tipo × 2) × 4. Acima de [0xae2364] + 0x52d vira −1; negativa, 0; a pessoa (0x6e bytes em [0xae23bc]) sem +0x61 apaga a célula; do clube (+0x39), numa seleção (0x53b900) ou com o vínculo 0x775790 é devolvida; senão apaga. Clube nulo, tipo ou lado fora: a original (caixa de erro). Comparada lado a lado (a célula) |
| `guardar_instrucoes()` | 0x896fe0 | thiscall, ret 0x10 | tactics.cpp (624 bytes, chamada pelo passo do motor 0x6b98c0 e outras): guarda ou devolve as 20 instruções (dwords da linha tática + 2) do clube num dos 16 espaços de 0x5c bytes (+0x30; ocupados em +0x5f0), o do humano da vez ([0xb74c26 + [0xb77c7a] × 0x300]; [0xae2364] − id − 1); guardando, as instruções viram −1. Comparada lado a lado (os espaços e a linha) |
| `numero_da_competicao()` | 0x6b6570 | cdecl | match_day.cpp (304 bytes): o número pelo código (0: 5/4, 1: 11/10, 0x1ef: 1/0, 0x3de: 3/2, 0x483: 7/6, 0x528: 9/8; o primeiro com o segundo argumento 1); outro código: a original (caixa de erro). Comparada pelo resultado |

Mutações: `mutacoes/leva138.py` (12, pela carreira até 2002-06-01): a carreira automática só passa pelos caminhos triviais — o batedor guardado sempre com a célula vazia (16 milhões de vezes: a IA não escolhe batedores), as instruções só apagadas (32 mil vezes), o número da competição nunca. Pega `i_menos_um` (a 20ª instrução não apagada muda a impressão das partidas); as outras 11 ficam puladas como lacunas (ramo sem passagem) e os ramos entraram em `docs/PENDENCIAS.md` para a sessão do Lucas comparando (clube humano com batedores escolhidos, mais de um humano, dia de jogo com humano). Primeira leva com três rotinas de uma vez (proposta do Lucas, 15/10). Comparando até 2001-08-20: 0 divergências. Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 139ª leva — o fechamento da partida e o original determinizado (oficial em 15/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `favorito_de()` | 0x547790 | cdecl | `b` tem `a` entre os três favoritos (+0x9b, +0x9f, +0xa3)? Comparada pelo resultado |
| `rejeitado_por()` | 0x5477d0 | cdecl | `b` tem `a` entre os três rejeitados (+0xa7, +0xab, +0xaf)? Comparada pelo resultado |
| `fechamento()` | 0x6d1cc0 | cdecl | match_eng.cpp (2,5 KB, chamada pelo passo do motor 0x6b98c0): com a lista de eventos do dia (+0x478e) e a partida, percorre os eventos (0x68d1b0): 0x82 vai para a lista global [0xb67dc8] (0x68d0d0); 0x1e pelo placar, pela força dos clubes e pelo técnico do outro lado (0x544410) marca o evento (+0xe, +0xf) ou o compara com a força do clube; os outros, do jogador da partida (6 pelas marcas +0x24/+0x23; os demais por sorteio). Depois, por lado, a ficha de cada jogador (0x6d26b0), o pedido para sair (evento 0xa) e a insatisfação (evento 0xb, rival, favorito/rejeitado 0x547790/0x5477d0, preferências 0x547d30/0x547d60), e a média das notas pelos minutos (±0,25 casa/fora) para as reputações (0x6d3610, 0x6d3de0). Primeira rotina do **original determinizado**: a média de um lado sem minutos e 4 bytes do evento montado na pilha eram lixo; a zeragem na entrada os deixa em zero em todos os modos. Comparada pela fita (a ficha, as reputações e a cópia do evento são unidades; o conteúdo do evento é conferido) |

Mutações: `mutacoes/leva139.py` (12, pela carreira até 2002-06-01, confirmação no modo comparação até 2001-10-01): todas pegas — 8 pela carreira (favorito, rejeitado, média da casa, os 90 minutos, minutos do reserva, evento do jogador, evento 6, proposta) e 4 só pela comparação (`f_insatisfacao`, `f_rival`, `f_gosta`: o valor pedido ao sorteio da insatisfação, conferido pela fita; `f_lixo_evento`: um dos 4 bytes do evento que eram lixo de pilha, conferido byte a byte na entrada da 0x68d0d0). A primeira `f_tipo6` (+0x24 = 3 no lugar de 2) era equivalente nos dados (nos eventos 6 da carreira o +0x24 nunca vale 2 ou 3) e foi refeita invertendo o +0x23. Primeira leva do original determinizado: a 0x6d1cc0 lia a média de um lado sem minutos e 4 bytes do evento montado na pilha sem valor; com a zeragem na entrada (`ZERADAS`) a carreira original não mudou (459815f5e0dca92b) e a cópia usa zero. Comparando até 2001-08-20: 0 divergências. Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 140ª leva — a linha tática e a simulação rápida (oficial em 15/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `ler_linha_tatica()` | 0x8983f0 | thiscall, ret 0xc | tactics.cpp: copia a linha tática do clube (0x91 bytes em +0x601, linha da 0x8982b0) para o destino. Clube nulo: a original (caixa de erro); linha -1: 0. Comparada lado a lado |
| `gravar_linha_tatica()` | 0x8984d0 | thiscall, ret 0xc | tactics.cpp: o inverso, grava os 0x91 bytes na linha (linha -1 ou além do total +0x605: 0). Chamada pela 0x6dbb10 (`guardar_tatica`). Comparada lado a lado |
| `simulacao_rapida()` | 0x6beeb0 | thiscall | match_eng.cpp (6 KB, chamada pelo passo do motor 0x6b98c0): a partida sem lances, em períodos de duas rodadas. Por período, a força dos times (0x6c0c60) com quatro pares limitados entre metade e o dobro e o lado que começa; por rodada, três ataques com duelo de um jogador ativo, o segundo e o terceiro duelo (0x6c2880), o desgaste (0x6c2750), os cartões do outro lado (0x6c2310; o segundo amarelo expulsa, 0x6cda20; mais de 4 expulsos: abandono 3 x 0), o fator do placar, a disputa (0x6c06b0/0x6c07a0/0x6c0a70), o chute (0x6c23f0, 0x6c0890/0x6c0980) e o gol (0x6c2050, goleiros 0x717410, narra 0x1fcc/0x1fcf, ou 0x1fce no pênalti perdido). Nos minutos 45/90/105/120 os placares (+0x45/+0x43/+0x47), prorrogação, gol de ouro e pênaltis; o agregado; o gol do mando; as lesões (0x6d0980); as notas (amistoso: pela reputação do clube). Lia bytes altos de dwords da pilha sem valor (original determinizado). Comparada pela fita (narração, lesões e as chamadas da saída do jogador são unidades) |

Mutações: `mutacoes/leva140.py` (23, pela carreira até 2002-06-01, confirmação no modo comparação até 2001-10-01): 21 pegas pela carreira e 2 lacunas (o abandono por expulsões e o reinício do contador de ataques nunca ocorrem na carreira; em `PENDENCIAS.md`). A primeira `t_gravar` (gravar um byte a menos) era equivalente nos dados e foi refeita copiando da origem deslocada; a primeira `s_contador` escapou da carreira e da comparação e revelou a lacuna (ganhou o ramo e a irmã `s_contador_igual`, pega). Na primeira comparação, 421 divergências de sorteios na 0x6beeb0 com a memória igual: os comparadores da 0x6c0c60 e das escolhas (0x6c2750...) fazem o lado a lado mesmo dentro da gravação da fita, e os sorteios da cópia deles entravam no rastro da original (a força do time aparecia duas vezes). Corrigido em `comparacao::lado_a_lado`: o lado Rust roda com o rastro pausado. Comparando até 2001-08-20: 0 divergências. Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 141ª leva — a lesão e o destaque do jogador (oficial em 15/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `lesionar()` | 0x6d0980 | thiscall, ret 0x10 | match_eng.cpp: a lesão do jogador na partida. Com mais de 3 lesões no jogo (+0xf639) sorteios a evitam; mais de 6, nunca. Registra (0x62be00), marca o causador (+0x55, -10 em +0x31), tira fôlego pela gravidade (acima de 50: o quadrado; senão sorteio), grava +0x59 e reduz +0xd5; sem fôlego sai, senão para (0x7160e0 com 5 ou 3) pela força da pessoa. Comparada pela fita quando roda fora de outra fita (a lista de lesões e a cópia 0x68d0d0); dentro, é unidade |
| `registrar_lesao()` | 0x62be00 | thiscall, ret 0xc | injury.cpp: a lesão da ficha na lista de lesões (+0x28 de [0xb63d28]): mesma lesão soma a gravidade (até 100), outra fica a mais grave; ficha nova vai para a lista (0x68d0d0, o registro montado na pilha conferido pela fita). Sem lista, a original (cria a lista) |
| `destaque_do_jogador()` | 0x717470 | thiscall, ret 4 | o jogador pode ganhar o destaque `codigo` (0x1f40..0x21e4)? Não se o lado já o teve (tabela em +0x21e do motor) ou a ficha é de fora; 0x219e..0x21b8 com regras de atuação, gols, cartões, tempo em campo e a busca no histórico (0x757da0, últimos 7 ou 30 dias). Comparada lado a lado |

Mutações: `mutacoes/leva141.py` (15, pela carreira até 2002-06-01, confirmação no modo comparação até 2001-10-01): 14 pegas pela carreira e 1 lacuna (`l_para3`: a lesão leve que para e sai nunca ocorre; em `PENDENCIAS.md`). O destaque só é concedido 16 vezes na carreira: as primeiras mutações dele (limites, faixa, padrão) e três das refeitas (a data da busca do 0x219e, o código do 0x21b1, a condição comum) não mudam nenhum resultado nos dados e saíram da lista; `d_sim` (força o 0x21b1) e `d_21b5` (sem a condição +0x30) provam que a comparação enxerga o resultado. A lesão (0x6d0980) só roda dentro de outras comparações pela fita, como unidade: a cópia dela é provada pela carreira e pelas mutações. Na primeira comparação o modo comparação mudou a carreira (5.264 sorteios a mais): faltava o `sem_contar` no lado Rust do comparador do destaque. Comparando até 2001-08-20: 0 divergências. Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 142ª leva — o chute a gol e o desfecho do chute (oficial em 15/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `chute_a_gol()` | 0x70a910 | thiscall, ret 4 | match_pl.cpp (5,9 KB): o chute de `p` contra o goleiro do outro lado (0x6ce110, defesa 0x700420). A qualidade pela força e por sorteios dá o tipo: fora (0x1fc6..0x1fcb, tiro de meta), no alvo; no alvo, a parte do chute contra a defesa decide gol (contagens do jogador, hat-trick, marcas da carreira 0x200a..0x2012, narrações do placar e do tempo, disputa de pênaltis 0x6bba50/0x6bac40), escanteio, linha de fundo ou defesa (destaque do goleiro 0x717470). Comparada pela fita (narração, bola parada, 0x716e10, 0x6f0630 e as da falta como unidades) |
| `desfecho_do_chute()` | 0x70c040 | thiscall, ret 4 | match_pl.cpp (5,1 KB): a mesma estrutura contra o goleiro do próprio lado (o gol vai para o outro lado, as narrações e o fator de lado espelhados). Original determinizado. Comparada pela fita |

Mutações: `mutacoes/leva142.py` (20, pela carreira até 2002-06-01, confirmação no modo comparação até 2001-10-01): 17 pegas pela carreira e 3 lacunas (as marcas da carreira e o empate tardio do chute a gol; em `PENDENCIAS.md`). A primeira `c_fator` mudava só o último bit do fator 0,1 (sem efeito depois do arredondamento) e foi refeita com 0,2. Leva de duas rotinas grandes (11 KB juntas, as últimas do chute). A 0x70c040 lê o rebote e a saída da defesa sem valor quando o próprio goleiro chuta: entrou no original determinizado (`ZERADAS`). As narrações de marca passam contagens em palavra como último argumento: a fita confere o byte baixo e a cópia Rust as passa inteiras. As duas (e a lesão da 141ª) só rodavam como unidades de outras fitas, sem comparação: agora, dentro da unidade, rodam a própria comparação (fita aninhada); na lesão isso pediu os nós e o ponteiro da lista nas regiões. Comparando até 2001-08-20: 0 divergências. Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 143ª leva — o movimento do jogador (oficial em 16/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `mover_jogador()` | 0x6f09f0 | thiscall, ret 0x14 | match_pl.cpp (9 KB): o jogador anda uma casa (duas, se for rápido) para (x, y) com a ação dada. Sem a bola, a lesão na dividida e, sem estar virado para o alvo, só vira. Com a bola: o ataque pelo fôlego e pela ação, a narração da aproximação, o desafiante (falta com o evento 0x82, disputa limpa), a posse e as narrações da ação; depois a grade, o mais adiantado, os marcados que saem, o desgaste pelas marcas da posição e a bola (acompanha ou domina a livre). Comparada pela fita (narração, 0x714d40, domínio, bola parada, bola fora, o evento e as da lesão e da falta como unidades) |

Mutações: `mutacoes/leva143.py` (22, pela carreira até 2002-06-01, confirmação no modo comparação até 2001-10-01): 20 pegas pela carreira e 2 lacunas (`m_vaga` e `m_adiantado`: o jogador sem vaga na grade e o lado sem o mais adiantado registrado; em `PENDENCIAS.md`). Leva de uma rotina grande (9 KB). Só rodava como unidade de outras fitas: o comparador grava a unidade e, dentro dela, roda a própria comparação (como na 142ª). O evento 0x82 vai para a lista montado na pilha: a 0x68d0d0 confere o conteúdo, não o endereço. Na primeira cadeia a carreira Rust estourou o tempo: o despacho do modo Rust mandava a 0x714d40 e a 0x68d0d0 para a chamada padrão da dividida; o despacho próprio do movimento resolveu (armadilha nova). Comparando até 2001-08-20: 0 divergências. Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 144ª leva — o jogador com a bola (oficial em 16/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `conduzir()` | 0x7109f0 | thiscall, ret 4 | match_pl.cpp (13 KB): a decisão de quem está com a bola. Bola parada: escanteios, lateral e faltas passam (0x6c6a30 escolhe o companheiro, senão uma casa sorteada); pênalti e falta perto do gol chutam (0x700bd0 + 0x702050). Em jogo, notas do chute, do passe e da condução pela distância ao gol, a pressão (0x6bcbd0, com lista), o placar, o goleiro, as marcas da posição e o relógio; lança (0x6fa490), divide (0x70ec80), chuta, passa (0x6fa250) ou conduz (0x6f09f0) para a casa de 0x6bce80. Modo 4: o goleiro nos pênaltis (devolve 1 quando fica). Original determinizado. Comparada pela fita |

Mutações: `mutacoes/leva144.py` (24, pela carreira até 2002-06-01, confirmação no modo comparação até 2001-10-01): 23 pegas pela carreira e 1 lacuna (`c_goleiro`: o guarda da posição 1 sem a marca de goleiro; em `PENDENCIAS.md`). Leva de uma rotina grande (13 KB, a decisão do jogador com a bola). Lê variáveis sem valor no modo 4 (o goleiro na disputa de pênaltis: a nota do passe em 0x712ec4 e o companheiro em 0x713107 nunca são escritos) e passa bytes com a parte alta sem valor ao passe e ao chute: entrou no original determinizado (`ZERADAS`). A lista da pressão (0x68d020) fica na pilha: a cópia na lista (0x68d0d0) entra na fita sem o endereço da lista e o chute (0x702050, ainda original) entra como unidade com a cabeça da lista ignorada. O passe, o lançamento e a dividida rodam por dentro da cópia (as unidades deles entram na fita da condução). Comparando até 2001-08-20: 0 divergências. Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 145ª leva — o laço do motor (oficial em 16/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `laco_do_motor()` | 0x6b98c0 | thiscall, ret 4 | match_eng.cpp (4,8 KB): `n` instantes da partida. Partida decidida: simulação rápida, destaque, fechamento, instruções dos humanos de volta, nota do árbitro (2). A cada instante: punições, pênaltis, pontapé inicial, substituição sorteada, lance marcado, reinício e cobrador; no relógio, abandono (8), período (4) e fim da partida (2 ou 4); depois posse, tempo de cada jogador, linhas, mais perto da bola, marcações e a decisão de cada jogador ativo (0x7103e0), com o chute pendente e a disputa. Validada pela carreira |

Mutações: `mutacoes/leva145.py` (18, só pela carreira até 2002-06-01): 16 pegas e 2 lacunas (`l_limpeza`, a ficha em campo sem motor; `l_direcao`, o cobrador da bola parada sem dono; em `PENDENCIAS.md`). Três da primeira rodada eram equivalentes ou fracas (−5 por −6 numa marca só testada como negativa; o relógio no pontapé inicial nunca vale 2; 19 contra 20 tentativas de sorteio) e foram refeitas (`leva145b.py`). Leva de uma rotina: o laço do motor, que só ordena chamadas. Sem comparador lado a lado (a fita fotografaria o motor a cada decisão de jogador): validada pela carreira, como as rotinas de muitos efeitos. As rotinas que ela chama seguem comparadas. Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba. Semente 777 até 2003-06-01, Rust e original: 4.738.450.581 sorteios, impressão 17dcb32ab70df0bd, partidas b006b77a7a0cc3ec, banco 480fccdff099e75f

### Motor de partidas, 146ª leva — a execução do passe (oficial em 16/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `executar_passe()` | 0x6fad40 | thiscall, ret 0x1c | match_pl.cpp (22 KB): o passe de `p` (tipo 6..=0x5e) de (x, y) para (tx, ty), com receptor e a saída (1 quando não chega a ele). Nota pelo tipo e pela distância; desafiante com falta ou desarme; sem ele, fôlego, passe impreciso pelo fator de +0x4756, custo em +0x35, narração da aproximação e do tipo de passe, a bola parada seguinte (+0x8ea9/+0x8eaa) e o traçado (0x6ccb60): chute (0x70c040/0x70a910) ou bola em trânsito (+0x1c5..+0x1c9, +0x477e). Comparada pela fita, com comparação dentro da unidade |

Mutações: `mutacoes/leva146.py` (25, pela carreira até 2002-06-01, confirmação no modo comparação até 2001-10-01): 24 pegas pela carreira e 1 pela confirmação (`p_cai`: o sorteio muda, o resultado não, e a fita confere os sorteios). Leva de uma rotina, a maior até aqui (22 KB: o mapa de funções cortava em 0x6fe364 uma rotina que vai até 0x6fffee). Primeira leva lida a partir do pacote (`pacote.py`), que mostrou o tamanho real, que o traçado do passe 0x6ccb60 só tem esse chamador (virou unidade da fita da 0x6fad40) e a nota antiga do ponteiro de saída. Desafiante, falta e desarme em versões próprias (a ordem das escritas, os sorteios do pênalti, o evento 0x82 fora do teste do cartão e as narrações diferem da 143ª); as narrações pelo tipo de passe em tabelas (quatro grupos de sete pela saída e por +0x1a7, a direção do passe escolhe no grupo). Comparando até 2001-08-20: 0 divergências, 12 dos 17 ramos. Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 147ª leva — o chute (oficial em 16/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `chutar()` | 0x702050 | thiscall, ret 0x18 | match_pl.cpp (35 KB): o chute de `p` (tipo, valor, narração, modo 1 jogo / 2 sem desafiante / 3 pênaltis / 4 cobrança / 5-6 repetir, a lista dos companheiros da pressão). Desafiante (0x6c87e0) com falta ou desarme; narração; impedimento; interceptação (0x6cc540); a classe do desfecho; gol (anulável), defesa, trave ou fora, com as narrações. Comparada pela fita, com comparação dentro da unidade |
| `atras_no_agregado()` | 0x6d5950 | thiscall, ret 4 | match_eng.cpp: o lado perde no agregado (0x6ce310 sem bônus) ou, no jogo de ida sem terceira fase nem classificado, empata |
| `companheiro_livre()` | 0x6d5a70 | thiscall, ret 4 | match_eng.cpp: o primeiro companheiro ativo no ataque fundo (0x70df40), no meio (0x70df20) e sem pressão (0x6bcbd0) |

Mutações: `mutacoes/leva147.py` (26, pela carreira até 2002-06-01, confirmação no modo comparação até 2001-10-01): 25 pegas pela carreira e 1 lacuna (`k_bloqueio`: o chute bloqueado no caminho do gol não ocorreu; ver `PENDENCIAS.md`). A maior rotina portada (35 KB, 10.356 instruções, recursiva: a falta na cobrança e o lance repetido chutam de novo). O desafiante, a falta e o desarme em versões próprias; o resto em partes: a classe do desfecho pela nota contra o limite, a razão chute/(defesa + chute) decide gol (acima de 71), rebatida (61), escanteio (51) ou defesa segura; o gol passa por até cinco chances de ser anulado pelo lance marcado (0x6bba50/0x6bac40), as marcas do artilheiro, a virada e o placar (0x6d4b70 fica na original); o chute para fora tem as narrações pela classe, pelo momento (+0x4782/+0x4786) e pelos companheiros da lista da pressão. A interceptação (0x6cc540) roda por dentro da cópia (o comparador dela não grava unidade) e as jogadas são as unidades. A porta de gravação da 0x702050 sai: o comparador grava a unidade sempre que há fita. Comparando até 2001-08-20: 0 divergências, 23 dos 31 ramos. Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 148ª leva — o motor nasce, prepara a partida e morre (oficial em 17/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `construir_motor()` | 0x6b7e50 | thiscall | match_eng.cpp: o construtor do motor (os vetores de jogadores, táticas e linhas táticas pelo construtor de vetores 0x9463af; o rigor 0x6e9460; a lista vazia) |
| `preparar_partida()` | 0x6b7f20 | thiscall, ret 0x10 | match_eng.cpp (6 KB): zera o estado, o clima (0x923010/0x923970/0x923630), os uniformes, o árbitro, o gramado (0x85cba0, +0x1ad/+0x1ae), os elencos (0x6db500), o narrador (dado ou novo), a força dos lados, a marcação, a tabela de ângulos e a lista de eventos de pré-jogo (insatisfação 0x13, interesse 0x1e, forma 5/4, fim de contrato 6); sorteia quem começa |
| `destruir_motor()` | 0x6b97a0 | thiscall | match_eng.cpp: o destrutor (o narrador próprio, a lista, o rigor e os vetores) |

Mutações: `mutacoes/leva148.py` (18, pela carreira até 2002-06-01) e `mutacoes/leva148b.py` (as 4 que escaparam, refeitas mais fortes): 16 pegas pela carreira e 2 lacunas (`i_ceu`: o limite do céu com chuva não pesa na carreira automática; `i_insatisfeito`: o código do evento 0x13 não muda o resultado; ver `PENDENCIAS.md`). Sem comparação própria: as três alocam (o narrador e a lista de eventos) ou liberam, e a fita teria de reproduzir as alocações; validadas pela carreira (Rust × original), como o laço do motor. As rotinas que elas chamam seguem comparadas. A tabela de ângulos (0xb67c30), calculada na primeira partida com fpatan, sai do atan do Rust e bate. Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 149ª leva — a escalação de cada lado e o dia de jogos (oficial em 17/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `escalar_lado()` | 0x6db500 | thiscall, ret 0x24 | match_man.cpp: monta a tática de um lado (técnico, jogadores, clube, adversário, partida, mando) e escala: no clube do humano as linhas guardadas (0x5fbe20/0x5fbd00) e a automática 0x6dbc50 modo 1 ou os titulares (0x6dbb80); fora isso a automática modo 2 e a conta dos da casa; os reservas pelo banco e a linha tática do clube. Devolve quantos escalou |
| `encerrar_dia()` | 0x6b4190 | thiscall | match_day.cpp: libera a lista global, os vetores do dia e, de cada partida, o motor (0x6b97a0), o resumo e o objeto +0x5f |
| `escolher_partida_do_dia()` | 0x6b42a0 | thiscall | match_day.cpp: limpa os campos de cada partida do dia e escolhe a do humano (ou a primeira marcada) para a tela (0x717cc0, 0x7fc7e0) |

Mutações: `mutacoes/leva149.py` (8, pela carreira até 2002-06-01): 7 pegas e 1 lacuna (`e_ficha`: o segundo campo da ficha do técnico não pesa na carreira automática; ver `PENDENCIAS.md`). Sem comparação própria: a escalação de cada lado chama a automática (0x6dbc50, ainda na original, que aloca a lista e lê a pilha) e o dia de jogos libera motores e listas; validadas pela carreira (Rust × original). As rotinas que elas chamam seguem comparadas. Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 150ª leva — a escalação automática (oficial em 17/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `escalar_automatico()` | 0x6dbc50 | thiscall, ret 8 | match_man.cpp (~7,7 KB): a comissão escala o time contra o adversário. Fora do humano, mede o jogo de hoje contra o próximo da lista (0x5acbe0: reputação da competição, rival, mesma cidade, dias até lá) para poupar (+0x1a 0/1/2 e as máscaras de vagas); filtra os 50 das linhas (estado, suspensão, já jogou, impedido, emprestado), escala (0x6e2000), escolhe a formação (0x6e0230), as três referências (+6/+0xa/+0xe), o rodízio (0x8cf880/0x6e8910), as vagas (0x6e0d00), o capitão (0x897760), as instruções (0x6ddab0) e as ordens gerais pela postura (0x5b50d0). Os locais vivem num quadro com os deslocamentos da pilha original (zerado, como a original determinizada) |

Mutações: `mutacoes/leva150.py` (17, pela carreira até 2002-06-01): 12 acusadas e 5 escaparam por serem fracas; refeitas mais fortes (`mutacoes/leva150b.py`: o peso dos dias, o filtro “já jogou”, a máscara, o rodízio e a postura 3), as 5 acusadas. Sem comparação própria: a escalação automática aloca a lista e os vetores do rodízio e chama 24 rotinas ainda originais; validada pela carreira (Rust × original). As rotinas que ela chama seguem comparadas. Jogo real comparando até 2002-01-01: 0 divergências (6500 escalações pela sonda, nenhuma com lixo). Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 151ª leva — o andamento do dia de jogos (oficial em 17/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `iniciar_proxima_partida()` | 0x6b5a70 | thiscall | match_day.cpp: começa a próxima partida do dia depois da atual (+0x2f): pula as decididas, as de clube humano e as já começadas; cria o motor (0x6b7e50) e o prepara (0x6b7f20); na 1ª volta sem nenhuma passa à 2ª. Devolve 1 se começou uma |
| `andar_partida_do_dia()` | 0x6b5ce0 | thiscall, ret 4 | match_day.cpp: um passo do motor da partida atual (1 de cada 3 ou 6 chamadas, pelos modos de [0xae3138]) ou até o fim; ao terminar, cria o resumo, registra (0x7293f0), libera o motor e tira da fila. Devolve 1 se terminou |
| `fechar_partida_do_dia()` | 0x6b5f00 | thiscall, ret 4 | match_day.cpp: copia a partida do dia para a partida, registra o resultado na competição (método 1, ou 0x924050 e 0x5ab510/0x5aa680 no jogo de fora) e roda o pós-jogo: jogadores, eventos, contratos, suspensões, estados, a história e o clube de cada lado (0x6b79a0) |

Mutações: `mutacoes/leva151.py` (14 e 2 refeitas, pela carreira até 2002-06-01): 6 acusadas e 8 escaparam; com os contadores de ramo na base: 2 eram fracas e, refeitas (`mutacoes/leva151b.py`: o intervalo como fim, a cópia da partida), as 2 acusadas; 1 é equivalente (o teste de partida decidida, -2, é redundante: a escolhida precisa de -1); 5 viram 4 lacunas em `PENDENCIAS.md` (o passo a passo e a rodada do humano não rodam na carreira automática; o contador de terminadas só serve à barra; a história não entra nas impressões). Sem comparação própria: o dia de jogos cria e libera motores e resumos e chama dezenas de rotinas ainda originais; validadas pela carreira (Rust × original). Jogo real comparando até 2002-01-01: 0 divergências. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba

### Motor de partidas, 152ª leva — o laço do dia de jogos (oficial em 17/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `jogar_o_dia()` | 0x6b4360 | thiscall | match_day.cpp (~3,3 KB): joga o dia. Em rede, liga os humanos às partidas deles (placar de 0x11d bytes, 0x765480, 0x5fcff0/0x5fd100) e as joga até o fim; depois cada partida sem motor: com humano em rede anda a atual (0x6b5ce0) e recomeça; senão cria o motor, joga até o fim, cria o resumo, registra e escreve o placar com a porcentagem na barra (0x66f4e0, 0x802f80). Por fim fecha cada partida (0x6b5f00), refaz as táticas guardadas dos humanos e libera os resumos |

Mutações: `mutacoes/leva152.py` (8, pela carreira até 2002-06-01): 5 acusadas e 3 escaparam: a da fila era fraca e, refeita sem a chamada (`mutacoes/leva152b.py`), foi acusada; a das linhas do humano não roda na carreira automática (lacuna nova em `PENDENCIAS.md`); a do contador de terminadas é a lacuna já anotada na 151ª. Sem comparação própria: o laço do dia cria e libera motores e resumos e mexe na tela; validado pela carreira (Rust × original). Jogo real comparando até 2002-01-01: 0 divergências (impressão nova 963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: as três iguais, numa referência nova (com o laço do dia desviado, as originais que ele chama rodam na pilha particular e leem outro lixo, igual nos dois modos): 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 153ª leva — a montagem do dia de jogos (oficial em 17/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `montar_o_dia()` | 0x6b3c10 | thiscall, ret 8 | match_day.cpp: monta o dia (data, modo 0..2): zera os vetores, junta as partidas pendentes de cada lista (0x5ae6a0), registra as já decididas (0x923630/0x923cc0), agrupa por lista e por competição (qsort 0x6b6510), aponta cada partida à sua competição (+0x63), cria a lista global [0xb67dc8] e calcula o interesse do dia (0x6b5040). Devolve o dia |
| `lado_do_jogador()` | 0x714d30 | thiscall | match_pl.cpp: o lado do jogador (+0x27) |
| `sons_terminados()` | 0x6d8c70 | thiscall, ret 4 | match_events.cpp: os sons dos 16 eventos marcados já tocaram e passaram da janela (relógio 0x90c510; para os canais que acabaram, 0x802a50)? |

Mutações: `mutacoes/leva153.py` (8 e 1 refeita, pela carreira até 2002-06-01): 4 acusadas e 4 escaparam; pelos contadores de ramo, viram 3 lacunas em `PENDENCIAS.md`: a 1ª volta do dia não escolhe nada na carreira automática (2 mutações), nenhuma partida chega marcada -2 à montagem (refeita forte em `mutacoes/leva153b.py`, escapou também) e os sons dos eventos só rodam com a tela. Sem comparação própria: a montagem aloca os vetores do dia e chama rotinas ainda originais; o lado é só leitura e os sons são tela; validadas pela carreira (Rust × original), com contadores de ramo. Jogo real comparando até 2002-01-01: 0 divergências (impressão 963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e. Testes rodados com outro projeto da máquina (CMRTE) aberto ao mesmo tempo, depois do ajuste da trava do `mutacoes.py` (só os exe desta pasta): tempos iguais aos das levas anteriores

### Motor de partidas, 154ª leva — os eventos gravados e o passo em rede (oficial em 17/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `gravar_eventos()` | 0x6d98b0 | thiscall, ret 8 | match_events.cpp: grava os eventos (+0x720, 0xe bytes cada) num dos dois arquivos "events1/2.tmp" (humano ou computador) de CM3_TEMP; cabeçalho versão 1, número, quantos; cheios os dois (2500), recria o mais antigo. Em `desc`: posição, quantos, número, humano |
| `ler_eventos()` | 0x6d9c40 | thiscall, ret 4 | match_events.cpp: lê de volta os eventos descritos em `desc` do arquivo com o número certo |
| `passo_em_rede()` | 0x6b55c0 | thiscall, ret 4 | match_day.cpp: um passo da partida do humano em rede, pelo relógio e pelo placar dos humanos (todos veem cada estado; sons, eventos, passo do motor, fim, intervalo, gols, minuto); registra (0x729ff0) e escolhe a partida do humano do dia |

Mutações: `mutacoes/leva154.py` (6, pela carreira até 2002-06-01): as 6 escaparam, e os contadores de ramo explicam: a gravação dos eventos roda ("eventos arquivo com espaco" acima de 2048, "eventos arquivo novo" 2), mas vai para arquivos temporários que não entram nas impressões; a leitura de volta e o passo em rede não rodam na carreira automática. Viram 2 lacunas em `PENDENCIAS.md`; o teste delas é no jogo (rever os lances de uma partida assistida). Sem comparação própria: mexem em arquivos e só rodam com a tela ou em rede; validadas pela carreira (Rust × original). Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 155ª leva — o leitor dos eventos da partida (oficial em 17/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `ler_eventos_cfg()` | 0x6d9e50 | cdecl | match_events.cpp (~5,2 KB): lê o `events_<idioma>.cfg` (ou o `events_eng.cfg`) para a tabela dos 677 códigos de evento em [0xb68788] (7 bytes de opções, os sons de 0x110 bytes com peso, tipo, nome e os dois textos `G`/`R`); confere marcas `<X`, eventos sem texto e, com `-verifysounds`, os arquivos de som; no fim converte os pesos e dá o som padrão a quem não tem |

Mutações: `mutacoes/leva155.py` (6 e 2 refeitas, pela carreira até 2002-06-01): 2 acusadas (o maior peso dos sons e, refeita, a opção 2) e 6 escaparam: os pesos, tipos e nomes dos sons e as opções 0, 1 e 4 só servem ao áudio, à tela e ao passo em rede (lacuna em `PENDENCIAS.md`; no jogo do Lucas a narração e os sons funcionaram). Sem comparação própria: lê arquivo, aloca os textos e grava a tabela global uma vez na abertura; validada pela carreira (Rust × original). Junto, a leitura dos lances da 154ª (0x6d9c40) ganhou comparação de verdade (lê só arquivo): no jogo do Lucas, 0 divergências. Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 156ª leva — o texto dos eventos e o começo da partida em rede (oficial em 17/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `texto_do_evento()` | 0x6d8d10 | stdcall, ret 0x10 | match_events.cpp: o texto `R` do som do evento com as marcas trocadas (`<P1>` jogador, `<T1>`/`<T2>` clube, `<C>` competição, `<F>` fase, `<D>`/`<N>` número, `<rf>`/`<Rf>` árbitro), prefixo e sufixo `{...}` (0x6701f0) e maiúsculas; escreve em [0xb67ea0] |
| `confirmar_partida_em_rede()` | 0x6b5260 | thiscall, ret 4 | match_day.cpp: cada humano confirma a partida em rede; com todos, cria o motor e o prepara para ver, ou joga até o fim e registra |

Mutações: `mutacoes/leva156.py` (3, pela carreira até 2002-06-01): as 3 escaparam; os contadores de ramo ("texto do evento", "partida em rede confirmada") ficam em zero: as duas só rodam com a tela da partida (lacuna em `PENDENCIAS.md`). O texto do evento ganhou comparação de verdade (só escreve nos buffers de texto): no `JOGAR_COMPARANDO`, cada linha de narração é montada pela original e pela cópia e conferida. A confirmação em rede não tem comparação (cria e libera motor e resumo). Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 157ª leva — a narração no painel (oficial em 17/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `narrar_no_painel()` | 0x6d7b00 | thiscall, ret 0x20 | match_events.cpp (~4,4 KB): repõe o som da torcida, monta o texto `G` do evento (como a 0x6d8d10) numa linha com as cores do clube (`<O>` as do outro, `<Y>`/`<R>` as padrão), quebra ou corta com "..." pela largura (0x5e5ad0, 0x90cb20), desenha no painel (0x801040…0x800580), toca o som do evento (0x8025e0) e marca a hora e a duração (+0x2d6c, +0x2dac) |

Mutações: `mutacoes/leva157.py` (2, pela carreira até 2002-06-01): as 2 escaparam; o contador "narracao no painel" fica em zero: a rotina só roda com a tela da partida (lacuna em `PENDENCIAS.md`; o teste é assistir partidas no `JOGAR.bat`). Sem comparação própria: desenha no painel e toca som. Com ela, o motor de partidas sem as telas fica 100% em Rust. Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Save, 158ª leva — o save acima de 2 GB (o contêiner do jogo) (oficial em 17/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `abrir_no_conteiner()` | 0x9326e0 | cdecl | zipdir.cpp (`zopen`): abre o item `nome` do contêiner (cabeçalho de 12 bytes em +0x19, índice de 1000 itens de 0x10c bytes em +0x25: posição, tamanho, nome); grava acrescentando no fim (máx. 999) ou lê indo à posição. **Diferença de propósito:** posição sem sinal e busca/posição em 64 bits (a original usa `fseek`/`ftell` de 32 bits com sinal: acima de 2 GB o índice ganha 0xffffffff e a leitura falha); acima de 4 GB recusa o item sem estragar o arquivo |
| `buscar_no_conteiner()` | 0x933080 | cdecl | zipdir.cpp (`zseek`): anda no item aberto para leitura (origem 0/1/2); fora do item, -1; tipo 4 (RLE) volta ao começo e relê até o ponto, tipo 3 busca direto. Posições sem sinal, busca em 64 bits |
| `posicao_no_conteiner()` | 0x933170 | cdecl | zipdir.cpp (`ztell`): tipo 4 devolve posição do item + z+8; tipo 3, o `ftell` (aqui em 64 bits, descontando o buffer como a 0x946aae) menos a posição do item |

Mutações: `mutacoes/leva158.py` (4, pela carreira até 2002-06-01): as 4 escaparam; a carreira automática não salva nem carrega (lacuna em `PENDENCIAS.md`). A prova é o `save_grande.py`: um início rápido com um enchimento de 2,2 GB gera um `.qst` de 2,38 GB; na original os 45 itens além de 2 GB ficam com posição 0xffffffff, com a 158ª o índice sai certo. No jogo do Lucas: save de 2,55 GB (187 itens além de 2 GB) gravado e carregado com a nossa DLL; com as originais (`CM_ORIGINAL=1`) o carregamento falha (`Friendly.cpp`, primeiro item além de 2 GB). Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 159ª leva — a formação: posições do jogador e casas no campo (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `mascara_de_posicoes()` | 0x5b7d20 | cdecl | formation.cpp: a máscara das posições fortes pelas notas (+0xf..+0x15, do nível 15 ao 10) e os lados com nota 15 (+0x17..+0x19); devolve só `ax` |
| `casa_no_campo()` | 0x5b7e60 | cdecl | formation.cpp: coluna 0..26 e linha 0..35 da posição (setor e lado pela 0x5b73a0) pelo ataque e pela faixa (tabelas de 4 e de 5, encostos de 1 ou 5 nas faixas, limites) |
| `vagas_sem_conflito()` | 0x5b8960 | thiscall | formation.cpp: 0 se alguma das 11 vagas com lado aberto (0x880) é de atacante, meia, volante ou meia-atacante (casas +0x12d/+0x14e e 0x5b2ae0); devolve só `al` |

Mutações: `mutacoes/leva159.py` (7, pela carreira até 2002-06-01, confirmando as que escapam no modo comparação até 2001-08-20): 5 acusadas; `m_nivel` (descer o nível de 2 em 2) e `v_a20` (0x10 no lugar de 0x20) escaparam nas duas provas: no banco do jogo a posição natural tem nota 20 (o laço para no nível 15) e nenhuma vaga com lado aberto é de meia (lacuna em `PENDENCIAS.md`). Primeira leva fora do motor de partidas: as candidatas vêm de `candidatos.py --simulacao` (cobertura da carreira automática em 26 módulos da simulação, `docs/cobertura_simulacao.txt`). Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 160ª leva — estatísticas do jogador e zagueiros na formação (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `estatistica_do_jogador()` | 0x7c2150 | stdcall | player_stats.cpp: a estatística `qual` 1..22 do registro (bytes, porcentagens ×100.0f, médias por jogo, as duas notas completadas com menos de 20 jogos ×0.05f), em x87 como a original |
| `media_das_notas()` | 0x7c1950 | thiscall | player_stats.cpp: a média das cinco notas de 4 bits do registro (zero conta 27), em 16 bits dividida por 4 |
| `zagueiros_sem_conflito()` | 0x5b8dc0 | thiscall | formation.cpp: 0 se alguma vaga com lado aberto é de zagueiro ou volante |

Mutações: `mutacoes/leva160.py` (7, pela carreira até 2002-06-01): as 7 acusadas. A estatística em ponto flutuante (divisões e produtos em f64) bate com o x87 da original em toda a carreira. Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 161ª leva — lista de observação, código da posição e critério da comissão (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `serve_para_a_vaga()` | 0x844300 | stdcall | shortlist_manager.cpp: o jogador serve para a vaga 0..15 da lista de observação (nota 15 no setor e no lado; a 15 pede contrato, id e a 0x600a50 em zero); devolve só `al` |
| `codigo_da_posicao()` | 0x83bd00 | stdcall | shortlist_manager.cpp: o código da posição pela máscara de setor e lado (28 combinações; o resto 0xe); devolve só `al` |
| `criterio_da_comissao()` | 0x866f60 | thiscall | staff_contracts.cpp: o critério da comissão técnica: uma nota em 8 bits (id mod 7, notas, idade, reputação, número de funções) decide pelas funções se devolve 1 |

Mutações: `mutacoes/leva161.py` (9, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 6 acusadas. `s_id` escapou: mudava só o limite exato do id (−16 para −17); refeita como `s_id_b` em `mutacoes/leva161b.py` e acusada (`mutacoes_refeitas.txt`). `c_idade` escapou: nem refeita com +15 (`c_idade_b`) é acusada: nenhuma pessoa da comissão tem +0x44 entre 51 e 75 nos dados. `c_rep` escapou: mudava só a reputação exata 0x1c53; refeita como `c_rep_b` e acusada. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 162ª leva — as vagas da tabela tática (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `gravar_vaga_tatica()` | 0x896a90 | thiscall | tactics.cpp: grava a pessoa na vaga 0..19 da linha do clube (0x8982b0) e apaga as instruções pendentes desse clube |
| `pessoa_da_vaga_tatica()` | 0x8968f0 | thiscall | tactics.cpp: a pessoa da vaga; sem ficha ou de outra nação numa seleção, a vaga vira −1 |
| `gravar_formacao_tatica()` | 0x8967d0 | thiscall | tactics.cpp: grava a formação na linha do clube |

Mutações: `mutacoes/leva162.py` (5, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 2 acusadas. `g_pendente` escapou: as instruções pendentes (+0x5f0) só existem com técnico humano. `p_ficha` escapou: na carreira automática as vagas da tabela tática estão vazias (−1): nem a condição invertida (`p_ficha_b`, `mutacoes/leva162b.py`) é acusada; a parte com pessoa só roda na tática do técnico humano. `p_nacao` escapou: idem (`p_nacao_b` invertida também escapa). (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 163ª leva — a formação (marcas e casas) e o reinício dos recordes do clube (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `casa_guardada()` | 0x5b3c70 | thiscall | formation.cpp: a casa guardada da vaga pelo ataque e pela faixa |
| `ligar_marca_da_formacao()` | 0x5b50d0 | thiscall | formation.cpp: liga a marca em +0x584 apagando as do mesmo grupo e zera +0x187f |
| `reiniciar_recordes()` | 0x452fe0 | cdecl | club_records.cpp: −1.0f e −1 nos campos dos recordes do clube |

Mutações: `mutacoes/leva163.py` (5, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 5 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 164ª leva — o calendário das ligas (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `girar_rodada()` | 0x68a030 | cdecl | league.cpp: a próxima posição no rodízio do calendário (liga especial, saltos de 3 por sorteio, voltas em n) |
| `registrar_jogo_da_rodada()` | 0x68a1c0 | cdecl | league.cpp: grava o jogo da rodada (três bytes e um dword) |
| `marcar_ano_no_calendario()` | 0x5af180 | cdecl | fix_man.cpp: marca o ano do jogo nas entradas dos dois clubes (ano × 2 + bit, regra de 2001) |

Mutações: `mutacoes/leva164.py` (7, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 5 acusadas. `g_meio` escapou: a carreira automática só joga ligas inglesas, sem a marca 0x80 do rodízio; nem refeita mais forte (`g_meio_b`, `mutacoes/leva164b.py`) é acusada. `g_volta` escapou: a volta no fim do rodízio não acontece na carreira; nem refeita mais forte (`g_volta_b`) é acusada. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 165ª leva — as vagas extras da tática, as idades da seleção e o dado do técnico humano (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `vaga_extra_tatica()` | 0x897ee0 | thiscall | tactics.cpp: as duas vagas extras (+0x56, +0x5a) da linha do clube |
| `idades_da_selecao()` | 0x76d430 | stdcall | national_teams.cpp: as idades limite (23/23 nas competições de base, 24/22, 26/22) |
| `gravar_dado_do_humano()` | 0x5fdd30 | thiscall | human_manager.cpp: grava +0x9a65 do registro do técnico humano (0xa6ef bytes) |

Mutações: `mutacoes/leva165.py` (5, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 4 acusadas. `v_desce` escapou: ao apagar a primeira vaga extra, a segunda já está vazia (−1) nos clubes da IA: descer −1 ou gravar −1 dá o mesmo (os contadores "vaga extra apagada" 65536 e "vaga extra gravada" 4096 mostram que os dois caminhos rodam). (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 166ª leva — o observador do humano, o humano no clube e a categoria do contrato (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `registro_do_observador()` | 0x7f6cd0 | thiscall | scout_manager.cpp: o registro de observação 0..9 do técnico humano (0x2b0 bytes) |
| `humano_manda_no_clube()` | 0x600c90 | cdecl | human_manager.cpp: o técnico humano da vez manda no clube (ou na seleção principal da nação)? |
| `categoria_do_contrato()` | 0x4ca470 | stdcall | contract_manager.cpp: a categoria 1..8 do jogador no contrato (situação, satisfação, idade, ambição, reputação) |

Mutações: `mutacoes/leva166.py` (7, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 3 acusadas. `h_forcar` escapou: o contador "humano manda no clube" fica em zero: sem técnico humano, a rotina nunca devolve 1 na carreira. `c_k0` escapou: limite exato; nem refeita mais forte (`c_k0_b`, `mutacoes/leva166b.py`) é acusada: a faixa da situação 0 com satisfação de 40 a 75 não aparece. `c_k5` escapou: limite exato; refeita como `c_k5_b` e acusada. `c_tipo5` escapou: contrato do tipo 5 não aparece na carreira; nem refeita mais forte (`c_tipo5_b`) é acusada. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 167ª leva — o fechamento do motor (1): o gol da bola, a partida abandonada e as vagas do lado (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `bola_na_area()` | 0x6d58f0 | thiscall | match_eng.cpp: 1 com a bola na área de cima, −1 na de baixo, senão 0; devolve só `al` |
| `partida_abandonada()` | 0x6d5930 | thiscall | match_eng.cpp: o minuto do abandono × 11 abaixo do relógio (16 bits); devolve só `al` |
| `vagas_do_lado_sem_conflito()` | 0x6d5af0 | thiscall | match_eng.cpp: a 0x5b8960 na formação do lado |

Mutações: `mutacoes/leva167.py` (4, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 3 acusadas. `a_onze` escapou: nenhuma partida é abandonada na carreira automática (o minuto do abandono fica em −1). (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 168ª leva — o fechamento do motor (2): os zagueiros do lado, o árbitro e o jogador da partida (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `zagueiros_do_lado_sem_conflito()` | 0x6d5b20 | thiscall | match_eng.cpp: a 0x5b8dc0 na formação do lado |
| `construir_arbitro()` | 0x6e9460 | thiscall | match_official.cpp: o construtor do árbitro (zera +0 e +4) |
| `destruir_jogador_da_partida()` | 0x6ebd60 | thiscall | match_pl.cpp: o destrutor do jogador da partida (zera +0x19e) |

Mutações: `mutacoes/leva168.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 3 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 169ª leva — o fechamento do motor (3): a fila da narração (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `evento_na_frente()` | 0x6d5d00 | thiscall | match_events.cpp: põe um evento na frente da fila da narração (recua a posição no anel de 12, tira o repetido) e enfileira pela 0x6d6f70 |
| `construir_fila_da_narracao()` | 0x6d5bf0 | thiscall | match_events.cpp: o construtor da fila da narração: zera a fila, lê os eventos e, se pedido, toca o loop da torcida |
| `liberar_tabelas_de_eventos()` | 0x6d5b50 | cdecl | match_events.cpp: libera as tabelas de eventos carregadas e os textos de cada registro |

Mutações: `mutacoes/leva169.py` (4, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 2 acusadas. `c_tempo` escapou: mutação equivalente: o +0x2db0 inicial (125) só é comparado com 20 e 101 e as telas da partida o regravam. `l_fim` escapou: mutação equivalente: a 0x6d9e50, que chama a liberação, regrava 0xa1599c (o idioma dos eventos) logo depois. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 170ª leva — o fechamento do motor (4): as ordens das partidas e dos jogadores e a variante do evento (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `ordem_das_partidas_do_dia()` | 0x6b6510 | cdecl | match_eng.cpp: a ordem das partidas do dia (qsort): competição, campos do registro e o nome do clube |
| `ordem_de_acao()` | 0x6bd840 | cdecl | match_eng.cpp: a ordem dos jogadores na vez do motor (qsort): os marcados primeiro, depois pela força |
| `variante_do_evento()` | 0x6d5d80 | stdcall | match_events.cpp: troca o evento da narração por uma variante, por sorteio e pelo tipo do lance |

Mutações: `mutacoes/leva170.py` (5, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 4 acusadas. `o_campos` escapou: na liga inglesa as partidas do dia empatadas até +0x36 nunca divergem em sinal entre +0x38 e +0x34 (falta de dados; vai para a carreira ampla). (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 171ª leva — o fechamento do motor (5): a narração combinada, o comentário de contexto e o jogador da partida (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `combinar_com_o_anterior()` | 0x6d60b0 | thiscall | match_events.cpp: combina o evento novo da narração com o último da fila (devolve o evento combinado e copia jogador, lado e valor do anterior) |
| `comentario_de_contexto()` | 0x6d4b70 | thiscall | match_eng.cpp: decide se um comentário de contexto (corrida da liga, notícias do clube) pode tocar agora |
| `construir_jogador_da_partida()` | 0x6eada0 | thiscall | match_pl.cpp: o construtor do jogador da partida (zera os campos, com os valores iniciais de alguns) |

Mutações: `mutacoes/leva171.py` (7, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 7 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 172ª leva — o fechamento do motor (6): a leitura da narração, o nome do jogador e a habilidade vista (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `proximo_evento()` | 0x6d78b0 | thiscall | match_events.cpp: o próximo evento da narração a mostrar, parte por parte do texto (separadas por |), avançando a leitura da fila |
| `nome_do_jogador_na_narracao()` | 0x6db2d0 | cdecl | match_events.cpp: o nome do jogador na narração: a forma curta, ou a longa quando outro jogador dos dois clubes tem o mesmo nome curto |
| `habilidade_vista()` | 0x6e86d0 | cdecl | match_man.cpp: a habilidade atual ou potencial do jogador como um observador a vê (reputação, ruído e conhecimento do observador) |

Mutações: `mutacoes/leva172.py` (6, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 3 acusadas. `p_parte` escapou: a 0x6d78b0 só roda com a narração na tela, que a carreira automática não mostra (vai para a partida assistida do Lucas). `p_anel` escapou: a 0x6d78b0 só roda com a narração na tela, que a carreira automática não mostra. `n_longa` escapou: a 0x6db2d0 só roda com a narração na tela, que a carreira automática não mostra. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 173ª leva — o fechamento do motor (7): a regra do elenco e as notas do jogador (oficial em 18/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `cumprir_regra_do_elenco()` | 0x6e8910 | cdecl | match_man.cpp: cumpre a regra do elenco (estrangeiros, extracomunitários ou idade): troca os restritos pelos candidatos de menor perda na nota |
| `nota_fora_da_partida()` | 0x6e6340 | cdecl | match_man.cpp: a nota do jogador para uma posição fora da partida (transferências e observação): posição, forma, habilidade vista, contrato, idade, conhecimento do observador |
| `nota_para_a_vaga()` | 0x6e2f20 | cdecl | match_man.cpp: a nota do jogador para uma vaga da escalação: como a nota fora da partida, mais a segunda posição, o fôlego na partida, a tática, os veteranos e as férias |

Mutações: `mutacoes/leva173.py` (12, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 8 acusadas. `r_banco` escapou: a fase do banco da regra de elenco (0x6e8910) não teve restritos no banco na liga inglesa (vai para a carreira ampla). `r_idade` escapou: a regra por idade (tipo 3) nunca teve um jogador exatamente no limite. `n_veterano` escapou: o ajuste do contrato (flag 0x2000000) nunca pegou um veterano (mais de 34 anos) sem jogos. `v_ferias` escapou: o bônus das férias da 0x6e2f20 só roda com a lista de férias de um técnico humano. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 174ª leva — a pessoa na lista do técnico, a ficha de contrato e o desgaste diário (oficial em 19/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `pessoa_na_lista()` | 0x69fc60 | thiscall | manager_manager.cpp: a pessoa está na lista (iterador 0x68d1b0, devolvido no fim)? |
| `ficha_de_contrato()` | 0x4cf6d0 | thiscall | contract_manager.cpp: a ficha de contrato (0x50 bytes) da pessoa |
| `desgaste_diario()` | 0x62c2f0 | thiscall | injury.cpp: o desgaste do dia na palavra +0xc (idade, sorteios de 25 e 13, teto de goleiro) |

Mutações: `mutacoes/leva174.py` (7, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 7 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 175ª leva — a formação da tabela tática, o peso das competições do jogador e o dado da notícia (oficial em 19/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `registro_da_formacao()` | 0x896570 | thiscall | tactics.cpp: o registro da formação (0x18e3 bytes) nas duas tabelas |
| `peso_da_competicao()` | 0x7c6930 | stdcall | player_stats.cpp: o peso da competição pelas duas competições do clube (500/1000/300/100) |
| `gravar_dado_da_noticia()` | 0x785790 | thiscall | news.cpp: grava o dado 0..49 da notícia |

Mutações: `mutacoes/leva175.py` (6, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 5 acusadas. `f_limite` escapou: o limite só muda para as formações além das padrão pedidas com o humano (as formações próprias de um técnico humano), que a carreira automática não usa. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 176ª leva — a recuperação da lesão, a pessoa na lista de transferências e a punição da disciplina (oficial em 19/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `recuperar_lesao()` | 0x62bd10 | thiscall | injury.cpp: o dia de recuperação (abaixo de −1000 em x87 com −0.0035 e 2.5; senão o ritmo ou sorteio) |
| `pessoa_na_lista_de_transferencias()` | 0x8c1870 | thiscall | transfer_manager.cpp: a pessoa está na lista +0x92c? |
| `punicao_da_tabela()` | 0x55cae0 | thiscall | discipline.cpp: a punição da linha e da coluna da tabela, se o tipo bater |

Mutações: `mutacoes/leva176.py` (6, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 5 acusadas. `t_item` escapou: a lista +0x92c do gerente de transferências nunca tem a pessoa procurada na carreira automática (a rotina devolve sempre 0); vai para a carreira ampla. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 177ª leva — o bit do contrato, a marca da competição e o registro do clube na liga (oficial em 19/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `bits_do_registro()` | 0x7a7eb0 | stdcall | player_regen.cpp: os bits 4..5 do byte +0xb do registro de 79 bytes da pessoa |
| `competicao_marcada()` | 0x684790 | thiscall | league.cpp: a competição tem +0xbe e a marca 2 de +0xd9? |
| `registro_da_liga()` | 0x6826d0 | thiscall | league.cpp: o registro (59 bytes) da tabela +0xb1 |

Mutações: `mutacoes/leva177.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 2 acusadas. `m_marca` escapou: nas ligas inglesas as competições com +0xbe ligado nunca têm os bits 1 e 2 de +0xd9 diferentes (falta de dados; vai para a carreira ampla). (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 178ª leva — a média do jogo, o valor do contrato e o nível para a seleção (oficial em 19/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `media_do_jogo()` | 0x5d5a60 | thiscall | game.cpp: a média f64 +0x20/+0x28 ou +0x10/+0x18 (divisor zero: 6.8) |
| `valor_do_contrato()` | 0x4d0c20 | thiscall | contract_manager.cpp: o dword +0xc da ficha de contrato da pessoa |
| `nivel_para_a_selecao()` | 0x76d810 | stdcall | national_teams.cpp: o nível 1..3 do jogador para a seleção pela reputação e pelas notas de posição |

Mutações: `mutacoes/leva178.py` (5, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 4 acusadas. `m_padrao` escapou: o divisor da média (+0x18/+0x28) nunca é zero quando a carreira consulta a média: o 6,8 padrão não aparece. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 179ª leva — as ofertas de contrato e a oferta de transferência fechada (oficial em 19/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `oferta_de_contrato()` | 0x4cc9e0 | thiscall | contract_manager.cpp: a oferta de contrato da pessoa para o clube, na lista do gerente de contratos |
| `oferta_acertada()` | 0x4cca30 | thiscall | contract_manager.cpp: a oferta de contrato acertada (estado 2) da pessoa |
| `oferta_fechada()` | 0x8d0d90 | stdcall | transfer_offer.cpp: a oferta de transferência está encerrada (pelo estado e, no estado 0xf, pela fase) |

Mutações: `mutacoes/leva179.py` (4, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 2 acusadas. `f_estado` escapou: na carreira as ofertas de transferência consultadas nunca estão no estado 4. `f_fase` escapou: nenhuma oferta no estado 0xf está na fase 2 quando é consultada. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 180ª leva — as telas da partida (1): o rótulo do controle, fechar a tela e a espera (oficial em 19/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `rotulo_do_controle()` | 0x725a70 | cdecl | match_screens.cpp: o rótulo do botão de controle da partida (esperando, começar, recomeçar, pausar ou retomar, continuar) e se ele fica desligado |
| `fechar_tela_da_partida()` | 0x71b120 | cdecl | match_screens.cpp: fecha a tela da partida: destrói e libera os parâmetros 0x2d e 0x1d do gerente de telas |
| `abrir_tela_de_espera()` | 0x727f00 | cdecl | match_screens.cpp: abre a tela de espera do técnico humano no gerente de telas |

Mutações: `mutacoes/leva180.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 181ª leva — as notícias do jogador na mídia e a importância da partida de copa (oficial em 19/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `noticia_com_resposta_2()` | 0x735780 | thiscall | media.cpp: o jogador tem uma notícia com a resposta 2 no registro da mídia |
| `tem_noticia()` | 0x735840 | thiscall | media.cpp: o jogador tem alguma notícia no registro da mídia |
| `importancia_da_partida()` | 0x5221f0 | thiscall | Cup.cpp: a importância da partida da copa pela fase e a diferença do placar da ida |

Mutações: `mutacoes/leva181.py` (4, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 2 acusadas. `m_alguma` escapou: as notícias do registro da mídia consultadas na carreira nunca têm a resposta 0. `c_fase` escapou: mutação equivalente: com a fase 1 trocar por 1 não muda nada (não é lacuna do teste). (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 182ª leva — as telas da partida (2): a tela dos últimos placares (oficial em 19/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `abrir_ultimos_placares()` | 0x71b1b0 | cdecl | match_screens.cpp: abre a tela dos últimos placares de uma competição e grava os parâmetros dela |
| `montar_ultimos_placares()` | 0x71b300 | cdecl | match_screens.cpp: monta a tela dos últimos placares: fundo, título, menu, painel e botões de voltar e avançar |
| `responder_ultimos_placares()` | 0x71b3d0 | cdecl | match_screens.cpp: a resposta da tela dos últimos placares a um clique (troca de competição, redesenho, menu) |

Mutações: `mutacoes/leva182.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 183ª leva — as telas da partida (3): a tela de detalhes da partida (oficial em 19/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `abrir_detalhes_da_partida()` | 0x71b4a0 | cdecl | match_screens.cpp: abre a tela de detalhes de uma partida e grava os parâmetros dela |
| `responder_detalhes_da_partida()` | 0x71bdc0 | cdecl | match_screens.cpp: a resposta da tela de detalhes a um clique (troca de aba, tática do clube, redesenho das abas, menu) |
| `fechar_detalhes_da_partida()` | 0x71c170 | cdecl | match_screens.cpp: fecha a tela de detalhes: destrói e libera os objetos guardados nos parâmetros |

Mutações: `mutacoes/leva183.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 184ª leva — a disciplina: a temporada nova, o dia e a vaga da tabela (oficial em 20/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `disciplina_temporada_nova()` | 0x556210 | thiscall | discipline.cpp: começa a temporada nova das regras de disciplina da competição (método 0 da tabela virtual) |
| `disciplina_do_dia()` | 0x556240 | thiscall | discipline.cpp: o dia de cada uma das 43 regras de disciplina (método 1 da tabela virtual) |
| `disciplina_gravar_vaga()` | 0x55cb20 | thiscall | discipline.cpp: grava o tipo e o valor de uma vaga da pessoa na tabela da disciplina |

Mutações: `mutacoes/leva184.py` (4, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 2 acusadas. `d_sem_regra` escapou: a competição com as regras de disciplina de índice 0 não começa temporada na carreira (liga fora das carregadas). `d_43` escapou: a 43ª regra de disciplina está vazia ou não muda nada nas ligas carregadas. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 185ª leva — a marca de contrato acertado e a vaga da oferta (oficial em 20/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `marcar_contrato_acertado()` | 0x8c5410 | thiscall | transfer_manager.cpp: liga a marca de contrato acertado (0x40) da pessoa no gerente de transferências |
| `desmarcar_contrato_acertado()` | 0x8c5430 | thiscall | transfer_manager.cpp: desliga a marca de contrato acertado da pessoa |
| `vaga_da_oferta()` | 0x8c61e0 | thiscall | transfer_manager.cpp: o endereço da vaga de uma oferta (fora da faixa, o começo da tabela) |

Mutações: `mutacoes/leva185.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 1 acusadas. `t_desmarca` escapou: quando a marca de contrato acertado é desligada, a pessoa nunca tem também a marca 0x80. `t_vaga` escapou: a carreira nunca pede a vaga de oferta igual à quantidade (fora da faixa). (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 186ª leva — as telas da partida (4): abrir a tela da partida e as respostas dos painéis dos últimos placares e das estatísticas do time (oficial em 20/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `abrir_tela_da_partida()` | 0x718aa0 | cdecl | match_screens.cpp: abre a tela da partida ao vivo, grava os parâmetros dela e marca o humano como espectador |
| `responder_ultimos_placares_da_partida()` | 0x725860 | cdecl | match_screens.cpp: o painel dos últimos placares da tela da partida: a troca de página a cada 5 s e os cliques (competição, páginas, parar) |
| `responder_estatisticas_do_time()` | 0x722dc0 | cdecl | match_screens.cpp: o painel das estatísticas do time: o redesenho com dados novos e os cliques que sobem e descem a linha |

Mutações: `mutacoes/leva186.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 187ª leva — as telas da partida (5): o painel dos últimos placares, a tela seguinte e a cor do título (oficial em 20/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `painel_dos_ultimos_placares()` | 0x723d20 | cdecl | match_screens.cpp: desenha o painel dos últimos placares: título, selo, grupos e partidas do dia, tabela e botões |
| `sem_tela_seguinte()` | 0x7febb0 | thiscall | scrman.cpp: o gerente de telas não tem tela seguinte para avançar |
| `cor_do_titulo()` | 0x839590 | cdecl | Setup.cpp: a cor do texto do título (duas cores pela marca) |

Mutações: `mutacoes/leva187.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 188ª leva — a lista de observados, a entrada do treino e a carga de treino (oficial em 20/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `lista_de_observados()` | 0x5d5ab0 | thiscall | game.cpp: o gerente da lista de observados guardado no gerente de transferências |
| `entrada_do_treino()` | 0x5d5ac0 | thiscall | game.cpp: a entrada de 6 bytes do jogador na tabela do treino |
| `gravar_carga_de_treino()` | 0x62c660 | thiscall | injury.cpp: grava a carga de treino do jogador no registro de condição |

Mutações: `mutacoes/leva188.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 3 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 189ª leva — a competição do amistoso, a data internacional e o registro de técnico (oficial em 20/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `e_competicao_do_amistoso()` | 0x5c2990 | thiscall | friendly.cpp: diz se a competição é a dos amistosos |
| `preencher_data_internacional()` | 0x5cb9b0 | thiscall | friendly.cpp: preenche uma data internacional: dia, mês, duração, ano, continente e competição |
| `limpar_registro_de_tecnico()` | 0x86fc20 | thiscall | staff_records.cpp: zera o registro de técnico e põe −1 nos ids do dono, da liga e dos clubes |

Mutações: `mutacoes/leva189.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 2 acusadas. `r_ids` escapou: o id da última passagem (+0x72) de um registro de técnico limpo nunca é lido na carreira antes de ser regravado. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 190ª leva — o contrato de empréstimo, as marcas de lance da lista de observados e o técnico substituto (oficial em 20/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `tirar_contrato_de_emprestimo()` | 0x4d0bd0 | thiscall | contract_manager.cpp: libera a vaga do contrato de empréstimo da pessoa, se houver |
| `desmarcar_lances_pelo_jogador()` | 0x84aeb0 | thiscall | shortlist_manager.cpp: tira a marca de lance das vagas da lista de observados do clube que miram o jogador |
| `tecnico_substituto()` | 0x600e90 | cdecl | human_manager.cpp: o auxiliar do clube ou, sem ele, o membro da comissão de maior reputação |

Mutações: `mutacoes/leva190.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 2 acusadas. `s_empate` escapou: o técnico substituto só é pedido para o clube humano, e na comissão dele nunca há empate de reputação. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 191ª leva — o clube do contrato acertado, o jogo pela seleção e a punição guardada (oficial em 20/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `clube_do_contrato_acertado()` | 0x8c5450 | thiscall | transfer_manager.cpp: o clube do contrato acertado da pessoa (desliga a marca quando o contrato sumiu) |
| `registrar_jogo_pela_selecao()` | 0x8ce920 | thiscall | transfer_manager.cpp: registra o jogo do jogador pela seleção nos bits dos últimos jogos |
| `punicao_guardada()` | 0x55cb60 | thiscall | discipline.cpp: a punição guardada do jogador numa vaga, ou nenhuma |

Mutações: `mutacoes/leva191.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 1 acusadas. `c_clube` escapou: na carreira o clube do contrato acertado nunca é pedido com a marca ligada e o contrato achado. `p_vaga` escapou: nenhum jogador tem duas punições guardadas em vagas diferentes quando a busca roda. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 192ª leva — a entrada da aposentadoria, as competições do amistoso e a vaga da notícia (oficial em 20/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `entrada_da_aposentadoria()` | 0x7a7ed0 | thiscall | player_regen.cpp: a entrada da pessoa na lista das aposentadorias (por lesão ou já anunciada) |
| `registrar_competicoes_do_amistoso()` | 0x5ca030 | thiscall | friendly.cpp: grava os objetos dos amistosos e dos torneios na tabela das competições |
| `vaga_da_noticia()` | 0x785830 | thiscall | news.cpp: o valor de uma das 50 vagas da notícia |

Mutações: `mutacoes/leva192.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 2 acusadas. `a_torneio` escapou: a vaga 0 dos torneios do amistoso nunca está ocupada na carreira. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 193ª leva — as cláusulas do contrato: percentual da próxima venda, bônus por jogo e bônus pela seleção (oficial em 20/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `tirar_percentual_da_proxima_venda()` | 0x8cb4f0 | thiscall | transfer_manager.cpp: tira o percentual da próxima venda do jogador (marca e registro da lista) |
| `tirar_bonus_por_jogo()` | 0x8cb590 | thiscall | transfer_manager.cpp: tira o bônus por jogo no time principal (marca e registro da lista) |
| `tirar_bonus_pela_selecao()` | 0x8cb630 | thiscall | transfer_manager.cpp: tira o bônus por jogo pela seleção (marca e registro da lista) |

Mutações: `mutacoes/leva193.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas. `l_venda` escapou: nenhuma cláusula de contrato (percentual da próxima venda, bônus por jogo, bônus pela seleção) é tirada na carreira até 2002-06. `l_jogo` escapou: nenhuma cláusula de contrato (percentual da próxima venda, bônus por jogo, bônus pela seleção) é tirada na carreira até 2002-06. `l_conta` escapou: nenhuma cláusula de contrato (percentual da próxima venda, bônus por jogo, bônus pela seleção) é tirada na carreira até 2002-06. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 194ª leva — as listas de reserva do gerador de jogadores e o prêmio da liga (oficial em 20/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `tirar_reserva_de_tecnico()` | 0x7a82b0 | thiscall | player_regen.cpp: tira da lista de reserva o registro de não jogador de um id |
| `tirar_jogador_do_banco()` | 0x7a8470 | thiscall | player_regen.cpp: tira do banco de jogadores livres o registro dos dados de jogador |
| `premio_da_liga()` | 0x682f70 | thiscall | league.cpp: o prêmio total da liga (uma cota a mais por posição acima do último) |

Mutações: `mutacoes/leva194.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 2 acusadas. `r_marca` escapou: o campo +8 da vaga que fica livre no fim da lista de reserva nunca é lido antes de ser regravado. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 195ª leva — a última nota do jogador e a vaga e o tipo da busca de jogadores (oficial em 20/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `ultima_nota_do_jogador()` | 0x7c1a60 | thiscall | player_stats.cpp: a nota da última partida do jogador (6,5 sem nota) |
| `vaga_da_busca()` | 0x7ae640 | thiscall | player_search.cpp: grava a vaga (0 a 9) da busca de jogadores |
| `tipo_da_lista_da_busca()` | 0x7ae6e0 | thiscall | player_search.cpp: grava o tipo de lista (1 a 4) da busca de jogadores |

Mutações: `mutacoes/leva195.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 3 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 196ª leva — o confronto direto da liga e os termos oferecidos na transferência (oficial em 20/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `ordem_do_confronto()` | 0x6834f0 | cdecl | league.cpp: a ordem da minitabela do confronto direto: pontos e os critérios de desempate da liga |
| `atualizar_confronto_direto()` | 0x6869a0 | thiscall | league.cpp: soma um jogo ao confronto direto (jogos, resultado, gols e pontos) |
| `marcar_termos_oferecidos()` | 0x8c1f80 | thiscall | transfer_manager.cpp: marca que a oferta de transferência teve os termos de contrato oferecidos ao jogador |

Mutações: `mutacoes/leva196.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 1 acusadas. `c_saldo` escapou: nenhuma liga da carreira desempata pelo confronto direto com o saldo (a minitabela não decide). `c_empate` escapou: os pontos por empate do confronto direto não decidem nenhuma classificação da carreira. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 197ª leva — o registro do técnico humano (hora das férias e escalação guardada) e o técnico da busca (oficial em 21/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `hora_das_ferias()` | 0x5fdc30 | thiscall | human_manager.cpp: a hora guardada quando o técnico humano saiu de férias |
| `escalacao_guardada()` | 0x5fed50 | thiscall | human_manager.cpp: o endereço da escalação guardada do técnico humano (16 pares) |
| `tecnico_da_busca()` | 0x7ae570 | thiscall | player_search.cpp: grava na busca de jogadores o id do técnico humano |

Mutações: `mutacoes/leva197.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 3 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 198ª leva — as telas da partida (6): a tela dos jogos do dia e o capitão padrão (oficial em 21/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `abrir_jogos_do_dia()` | 0x717cc0 | cdecl | match_screens.cpp: abre a tela dos jogos do dia numa vaga com o grupo de jogos (caixa de erro sem grupo) |
| `responder_jogos_do_dia()` | 0x718930 | cdecl | match_screens.cpp: a resposta da tela dos jogos do dia: outro grupo de jogos, o andamento da competição ou a barra de menu |
| `vaga_do_mais_bem_avaliado()` | 0x727e20 | cdecl | match_screens.cpp: a vaga dos onze com a maior nota da comissão (o capitão padrão), −1 sem ninguém |

Mutações: `mutacoes/leva198.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 199ª leva — as estatísticas da partida: a estatística de liga do jogador, a entrada do jogo e o balde do registro (oficial em 21/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `estatistica_da_liga_do_jogador()` | 0x7c47b0 | thiscall | player_stats.cpp: as partidas (campo 1) ou os gols (campo 2) de liga do jogador no clube, como número de ponto flutuante |
| `entrada_do_jogo_bate()` | 0x729500 | thiscall | match_screens.cpp: diz se uma entrada das estatísticas tem os dois clubes, em qualquer ordem, e as datas dadas |
| `balde_do_registro()` | 0x729bd0 | stdcall | match_screens.cpp: o balde (0 a 3) em que o registro de estatística do jogo é guardado: tipo da competição, seleção e marcas do jogo |

Mutações: `mutacoes/leva199.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 1 acusadas. `e_data` escapou: na carreira a segunda data da entrada do jogo nunca decide (quando a primeira bate, a segunda também bate). `b_tipo` escapou: o ramo sem a marca +0x40 do balde do registro não roda na carreira automática. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 200ª leva — as telas da partida (9): a cópia do bloco do time e da linha do jogador e o placar ao vivo (oficial em 21/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `atribuir_bloco_do_time()` | 0x71dd70 | thiscall | match_screens.cpp: copia o bloco do time da partida (0x625 bytes) campo a campo, com as 20 linhas de jogador |
| `atribuir_linha_do_jogador()` | 0x71df00 | thiscall | match_screens.cpp: copia uma linha de jogador do bloco do time (0x49 bytes), a data pela Date::operator= |
| `contar_placar_ao_vivo()` | 0x728e90 | cdecl | match_screens.cpp: conta os gols e os gols nos pênaltis de uma partida até o minuto atual da partida ao vivo |

Mutações: `mutacoes/leva200.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 201ª leva — as telas da partida (7): o Continuar da partida, abrir a escalação e a resposta da escalação (oficial em 21/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `responder_continuar_da_partida()` | 0x725ca0 | cdecl | match_screens.cpp: o Continuar da partida: o comando pendente da escalação (em rede, a vaga livre do clube ou a espera pelos outros), as partidas da rede e a escalação |
| `abrir_escalacao_da_partida()` | 0x725f70 | cdecl | match_screens.cpp: abre a tela da escalação da próxima partida do humano e grava o clube, a partida e se joga em casa |
| `responder_escalacao_da_partida()` | 0x727c30 | cdecl | match_screens.cpp: os botões da escalação: nomeia o capitão (o melhor titular) se faltar e deixa o comando pendente; os outros vão à resposta comum |

Mutações: `mutacoes/leva201.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 202ª leva — as telas da partida (8): os painéis das estatísticas do time, das notas dos jogadores e das estatísticas da partida (oficial em 21/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `painel_das_estatisticas_do_time()` | 0x7217a0 | cdecl | match_screens.cpp: desenha a tabela das estatísticas de um time (cabeçalho da página, setas, títulos das colunas) ou a mensagem de que não há estatísticas |
| `painel_das_notas_dos_jogadores()` | 0x722ed0 | cdecl | match_screens.cpp: desenha as notas dos jogadores dos dois times: camisa, nome (botão, com o capitão) e nota |
| `painel_das_estatisticas_da_partida()` | 0x71e040 | cdecl | match_screens.cpp: desenha as estatísticas da partida: 13 linhas rotuladas (chutes, escanteios, faltas, passes, desarmes, cabeçadas, cartões) |

Mutações: `mutacoes/leva202.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 203ª leva — as telas da partida (11): os valores das estatísticas da partida, das zonas de ação e das notas dos jogadores (oficial em 21/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `valores_das_estatisticas_da_partida()` | 0x71e750 | cdecl | match_screens.cpp: passa às células das estatísticas da partida os valores que mudaram desde a cópia do estado (parâmetro 0x1d) e atualiza a cópia |
| `valores_das_zonas_de_acao()` | 0x720320 | cdecl | match_screens.cpp: atualiza as zonas de ação: marcadores, posse, tempo de bola em jogo, barras das três zonas e da posse com as cores dos clubes; atualiza a cópia do estado |
| `valores_das_notas_dos_jogadores()` | 0x723350 | cdecl | match_screens.cpp: atualiza as linhas das notas dos jogadores: cartões, minuto de lesão/expulsão/substituição, nota, gols e cor do nome; atualiza a cópia do estado |

Mutações: `mutacoes/leva203.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 204ª leva — as telas da partida (10): os painéis das zonas de ação e do relatório da partida e a tela dos jogos do dia (oficial em 21/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `painel_das_zonas_de_acao()` | 0x71f500 | cdecl | match_screens.cpp: desenha as zonas de ação: o seletor de período, o campo dividido em três zonas com as barras das cores dos dois clubes, ou a mensagem de que não há zonas |
| `painel_do_relatorio_da_partida()` | 0x721000 | cdecl | match_screens.cpp: desenha o relatório da partida: os minutos e os textos dos lances (criando o relatório fora da partida), ou a mensagem de que não há relatório |
| `montar_jogos_do_dia()` | 0x717db0 | cdecl | match_screens.cpp: monta a tela dos jogos do dia: título, árvore das competições do dia, a tabela das rodadas e partidas e a rolagem até a partida do humano |

Mutações: `mutacoes/leva204.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 205ª leva — as telas da partida (12): as linhas das estatísticas do time, o redesenho dos últimos placares e a espera dos outros jogadores (oficial em 21/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `preencher_estatisticas_do_time()` | 0x721cd0 | cdecl | match_screens.cpp: escreve as 20 linhas de jogador das estatísticas do time (22 colunas pela tabela do parâmetro 0x1c), montando as células ou atualizando o que mudou |
| `atualizar_ultimos_placares()` | 0x724520 | cdecl | match_screens.cpp: reescreve as linhas dos últimos placares em páginas: fase, jogo, placar, período, autores dos eventos, agregado e o rodapé da página |
| `esperar_os_outros_jogadores()` | 0x727fb0 | cdecl | match_screens.cpp: no jogo em rede, espera os outros humanos da mesma partida ou fecha as telas, avisa e abre a tela da partida |

Mutações: `mutacoes/leva205.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 206ª leva — as estatísticas da partida: ler e gravar o registro no arquivo temporário e registrar a partida (oficial em 21/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `ler_registro_da_partida()` | 0x729600 | thiscall | match_stats.cpp: lê um registro de estatística da partida (0xd7a bytes) do match_stats_matches.tmp pelo índice |
| `registrar_partida()` | 0x7293f0 | thiscall | match_stats.cpp: preenche o registro de uma partida que terminou e o grava, salvo os jogos excluídos |
| `gravar_registro_da_partida()` | 0x729820 | thiscall | match_stats.cpp: grava os eventos e o registro na vaga do jogo (ou na vaga do cursor do balde, que anda em círculo) e atualiza o índice |

Mutações: `mutacoes/leva206.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 1 acusadas. `l_tamanho` escapou: ler o registro da partida (0x729600) só roda quando se abre o relatório de uma partida jogada; a carreira automática não abre telas. `r_excluido` escapou: na carreira o jogo nunca é o da competição [0x9cf684], então o "ou" decide na primeira metade e a marca +0x4c do registro nunca é lida. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 207ª leva — as telas da partida (15): a conferência da escalação e o índice das estatísticas da partida no save (oficial em 21/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `desenhar_conferencia_da_escalacao()` | 0x7260f0 | cdecl | match_screens.cpp: desenha a conferência da escalação: avisa (com os nomes ligados) os convocados, emprestados, indisponíveis, inelegíveis, suspensos, lesionados e sem permissão de trabalho, o limite do banco ou a falta de titulares; senão pede a confirmação (capitão, cobradores, reservas) com "No" e "Yes" |
| `carregar_estatisticas_da_partida()` | 0x72aa80 | thiscall | match_stats.cpp: lê do match_stats.dat do save o índice dos registros, os cursores dos quatro baldes e as contagens |
| `salvar_estatisticas_da_partida()` | 0x72adf0 | thiscall | match_stats.cpp: grava no match_stats.dat do save o índice dos registros, os cursores dos quatro baldes e as contagens |

Mutações: `mutacoes/leva207.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 208ª leva — as áreas da interface (1): organizar a grade, iniciar e desenhar uma área (com a trava das 30 linhas: o "Area.cpp 507" do relatório em 1080) (oficial em 21/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `organizar_area()` | 0x403350 | thiscall | Area.cpp: reparte a largura e a altura da área pelos pesos das colunas e linhas (até 30 de cada; mais dá a caixa Area.cpp 497/507) e liga a barra de rolagem |
| `iniciar_area()` | 0x402ac0 | thiscall | Area.cpp: inicia a área: retângulo limitado à tela, campos, barra de rolagem zerada e os pesos das colunas e linhas; as linhas à mostra travadas em 30 (o tamanho da grade), o resto vai para a barra de rolagem |
| `desenhar_area()` | 0x402e70 | thiscall | Area.cpp: desenha a área: o painel, as linhas da grade ou cada item à mostra na sua célula, e a barra de rolagem |

Mutações: `mutacoes/leva208.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Motor de partidas, 209ª leva — as telas da partida (14): a montagem e a resposta da tela da partida ao vivo e a montagem da tela de detalhes; as telas da partida (13): a barra da posse, o painel da visão geral da partida e a tabela da liga ao vivo (oficial em 21/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `montar_tela_da_partida()` | 0x718e90 | cdecl | match_screens.cpp: monta a tela da partida ao vivo: cabeçalhos dos times e placar, narração, relógio, controle, botões táticos, velocidades da narração, abas, o painel da aba, o estádio e o painel de narração |
| `responder_tela_da_partida()` | 0x71a240 | cdecl | match_screens.cpp: responde na tela da partida ao vivo: volume e velocidade da narração, o lance novo (relógio, narração, modos), assistir ou não, a tática de um time, a troca de aba e as respostas de cada aba |
| `montar_detalhes_da_partida()` | 0x71b630 | cdecl | match_screens.cpp: monta a tela de detalhes de uma partida guardada: lê o registro e a foto da partida nas estatísticas, os cabeçalhos dos times, as 7 abas e o painel da aba |
| `atualizar_barra_da_posse()` | 0x71d9e0 | cdecl | match_screens.cpp: redesenha a barra da posse da visão geral quando a posse muda (percentuais e a cor do clube com mais posse) e guarda o estado da partida na cópia do parâmetro 0x1d |
| `painel_da_visao_geral_da_partida()` | 0x71c270 | cdecl | match_screens.cpp: desenha a visão geral da partida: período, barra da posse e comentário, data, placar com intervalo/prorrogação/pênaltis, competição, autores dos gols e cartões, disputa de pênaltis, árbitro, público, tempo e partida abandonada |
| `tabela_da_liga_ao_vivo()` | 0x7280e0 | cdecl | match_screens.cpp: desenha a tabela da liga ao vivo (liga, topo, base) numa cópia da liga com os placares do dia aplicados (ou o aviso da data do sorteio): título, cabeçalhos, separadores, posição, seta de subida/descida, clube e colunas |

Mutações: `mutacoes/leva209.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 210ª leva — a história punível da imprensa, o tipo de lesão sorteado e a categoria da notícia; as convocações repetidas das seleções, a vaga do escalado e o comentarista ex-internacional; as regras da rodada no jogo da copa, a chave do jogo e o país da excursão (oficial em 22/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `historia_punivel_do_jogador()` | 0x7357c0 | thiscall | media.cpp: o tipo da história da imprensa sobre o jogador quando é de indisciplina, senão 0 |
| `escolher_tipo_de_lesao()` | 0x62d140 | thiscall | injury.cpp: sorteia o tipo de lesão da parte do corpo e da gravidade, mantendo a lesão atual ou preferindo a recorrente |
| `categoria_da_noticia()` | 0x785670 | thiscall | news.cpp: a categoria da notícia pelo milhar do tipo |
| `apagar_repetidos_das_convocacoes()` | 0x76c060 | stdcall | national_teams.cpp: apaga as convocações repetidas da seleção principal e da seleção B da nação |
| `vaga_do_escalado()` | 0x8972c0 | thiscall | tactics.cpp: a vaga (0..19) da escalação do clube em que o jogador está, ou 0xff |
| `escolher_comentarista()` | 0x72b810 | thiscall | media.cpp: sorteia um ex-internacional (37 anos ou mais, 50 jogos pela seleção) para comentar, pesando a reputação da nação e os jogos |
| `regras_da_rodada_no_jogo()` | 0x521d60 | thiscall | Cup.cpp: passa as regras da rodada para o jogo e, com decisão no empate, marca quem passa pelo nível dos dois lados |
| `chave_do_jogo_na_copa()` | 0x519c90 | thiscall | Cup.cpp: a chave da rodada em que está o mandante do jogo (-1 se nenhuma) |
| `pais_da_excursao()` | 0x5c4680 | thiscall | friendly.cpp: sorteia o país da excursão do clube, mais perto quanto menor a reputação |

Mutações: `mutacoes/leva210.py` (9, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 8 acusadas. `m_dobro` escapou: dobrar o peso (ou somar 1) só decide quando concorrem candidatos da nação do assunto e de outra nação; na carreira os de outra nação quase nunca passam do corte (reputação acima de 6500 + sorteio e ficha ≥ 6000), e entre os da mesma nação a ordem de 2a e a + 1 é a mesma. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 211ª leva — o fechamento das propostas de contrato da oferta, a fila da transferência livre e as negociações de troca do jogador; o jogador que sai da seleção, a faixa de horário do jogo e o teto da lista de observados; as ofertas pelo jogador, a reputação-alvo do clube e o jogo na notícia (oficial em 22/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `fechar_propostas_da_oferta()` | 0x8c6db0 | stdcall | transfer_manager.cpp: fecha (estado 4) as propostas de contrato ligadas à oferta: a do jogador-alvo e as dos jogadores da troca |
| `enfileirar_transferencia_livre()` | 0x8c16b0 | thiscall | transfer_manager.cpp: põe o jogador na fila da transferência livre (id, motivo e compensação), recusando o emprestado |
| `negociacoes_de_troca_do_jogador()` | 0x8c5fc0 | thiscall | transfer_manager.cpp: conta as negociações abertas do clube do jogador que o incluem na troca e, se pedido, lista os índices das ofertas |
| `tirar_jogador_da_selecao()` | 0x775ba0 | thiscall | national_teams.cpp: tira o jogador da reserva do nível dele na nação e da convocação, limpando os campos de seleção |
| `faixa_do_horario()` | 0x5af2f0 | thiscall | fix_man.cpp: a faixa de horário do jogo (1 fim de semana, 2 os outros) pela nação da competição e o dia |
| `teto_da_observacao()` | 0x84b230 | stdcall | shortlist_manager.cpp: o teto de preço da vaga na lista de observados: reputação do clube vezes um fator (ou -1) |
| `ofertas_pelo_jogador()` | 0x8c5cf0 | thiscall | transfer_manager.cpp: conta as ofertas de transferência pelo jogador (sem as encerradas, salvo pedido) e pode listar os índices delas |
| `reputacao_alvo_do_clube()` | 0x83bf40 | thiscall | shortlist_manager.cpp: a reputação que o clube procura nas contratações, ajustada pelo técnico e pelo presidente, entre o piso do clube e 10000 |
| `gravar_jogo_na_noticia()` | 0x4afeb0 | cdecl | comp_util.cpp: grava clubes, local, competição, placar e rodada do jogo em 18 dados seguidos da notícia e avança o cursor |

Mutações: `mutacoes/leva211.py` (9, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 8 acusadas. `s_faixa` escapou: na carreira o jogador que sai da seleção nunca está na faixa do nível dele com a nota do nível abaixo da da ficha, então o tamanho da faixa não diminui (o comparador cobre o registro da nação inteiro, onde fica o tamanho). (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e

### Simulação, 212ª leva — o registro da partida, a ordem dos eventos e o idioma que o jogador perde (oficial em 22/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `preencher_registro_da_partida()` | 0x729ff0 | thiscall | match_screens.cpp: preenche o registro de estatística da partida: clubes, datas, público, renda, clima, eventos ordenados, formação, capitão e os 20 jogadores de cada time |
| `ordem_dos_eventos()` | 0x72b160 | cdecl | match_screens.cpp: ordena dois eventos da partida pelo grupo do tipo, depois pelo minuto e pela ordem de chegada (comparador do qsort) |
| `tirar_idioma_do_jogador()` | 0x7c6bf0 | thiscall | player_stats.cpp: tira um idioma do jogador da tabela de idiomas (busca binária) e desce as entradas seguintes |

Mutações: `mutacoes/leva212.py` (3, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 2 acusadas. `i_ultima` escapou: na carreira a 0x7c6bf0 só é chamada pelos regens (idiomas 1..64 do jogador recém-criado, que ainda não tem nenhum): a busca nunca acha a entrada, então a última vaga nunca é apagada (o comparador cobre a tabela inteira). (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (5afd987e1a1a3ef3). Carreiras Rust ×2 e original até 2002-06-01: 2.361.839.396 sorteios, impressão b11bd5cb63f51d72, partidas 2461da5b0d296dba, banco 4d07a03c94c6fda5

### Simulação, 213ª leva — a contratação pelas regras da nação e as trocas que oferecem o jogador; as notícias não lidas, a marca gravada na notícia da caixa e o clube de técnico humano; as datas do sorteio e do jogo da rodada da copa e o clube que sai dos amistosos planejados (oficial em 22/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `pode_contratar_jogador()` | 0x8ced40 | thiscall | transfer_manager.cpp: se o clube pode contratar o jogador pelas regras da nação, com o motivo da recusa e a segunda chance do clube humano no limite de estrangeiros |
| `trocas_oferecendo_o_jogador()` | 0x8c5e40 | thiscall | transfer_manager.cpp: quantas ofertas abertas do clube oferecem o jogador na troca, com a lista dos índices se pedida |
| `noticias_nao_lidas()` | 0x786fe0 | thiscall | news.cpp: as notícias não lidas da caixa do técnico humano, no total ou de uma categoria |
| `gravar_marca_da_noticia()` | 0x786d30 | thiscall | news.cpp: grava um valor numa vaga da notícia da caixa do técnico humano e na cópia local dela |
| `clube_de_tecnico_humano()` | 0x7875e0 | stdcall | news.cpp: se um humano dirige o clube, sem contar as férias |
| `data_do_sorteio_da_rodada()` | 0x521e60 | cdecl | Cup.cpp: grava a data do sorteio da rodada da copa: o dia do ano e o ano contado do ano-base |
| `data_do_jogo_da_rodada()` | 0x521eb0 | cdecl | Cup.cpp: grava a data do jogo da rodada da copa, o horário e as marcas do mando |
| `tirar_clube_dos_amistosos()` | 0x5c64e0 | thiscall | friendly.cpp: tira o clube dos amistosos planejados pelo computador e das listas de amistosos |

Mutações: `mutacoes/leva213.py` (8, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 7 acusadas. `tr_recusa` escapou: na carreira as regras de transferência não recusam nenhuma contratação perguntada sem a segunda chance (a5 = 0), então o caminho "recusado, sem segunda pergunta" não roda (nem no modo comparação). (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (5afd987e1a1a3ef3). Carreiras Rust ×2 e original até 2002-06-01: 2.361.839.396 sorteios, impressão b11bd5cb63f51d72, partidas 2461da5b0d296dba, banco 4d07a03c94c6fda5

### Simulação, 214ª leva — a ordem das linhas da tabela, a grade de turnos da liga e a margem de gols do último dia; a partida guardada na notícia, o resultado da eliminatória para a notícia e o dia da rodada da copa; o mapa de casas da formação, a nota do clube para o jovem e a partida assistida pelo humano (oficial em 22/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `ordem_das_linhas_da_tabela()` | 0x683210 | cdecl | league.cpp: a comparação do qsort da tabela: pontos, os critérios de desempate da liga e o id do clube |
| `montar_grade_de_turnos()` | 0x6844b0 | stdcall | league.cpp: monta a grade em que cada clube enfrenta todos uma vez, com o mando equilibrado |
| `margem_do_ultimo_dia()` | 0x68a4d0 | thiscall | league.cpp: no último dia, a margem de gols que o clube precisa para o título, as vagas de cima ou fugir da queda, e a marca de jogo decisivo |
| `partida_das_vagas_da_noticia()` | 0x4b0010 | cdecl | comp_util.cpp: refaz a partida a partir de 18 vagas seguidas da notícia e liga os clubes, o estádio e a competição |
| `resultado_da_eliminatoria_na_noticia()` | 0x4b0890 | cdecl | comp_util.cpp: a categoria do resultado da eliminatória para o clube, pela margem final, o mando e a diferença de reputação |
| `dia_da_rodada_da_copa()` | 0x521f10 | cdecl | Cup.cpp: grava um dos oito dias da rodada da copa: o mando, o dia da semana, o horário e o código |
| `mapa_de_casas_da_formacao()` | 0x5b7160 | thiscall | formation.cpp: expande as posições de cada casa da formação nas tabelas de células do campo e no espelho delas |
| `nota_do_clube_para_o_jovem()` | 0x7ad220 | stdcall | player_regen.cpp: a nota sorteada de quanto o clube serve ao jovem: reputação contra habilidade, juniores, controle humano e nação |
| `partida_assistida_pelo_humano()` | 0x5fe290 | thiscall | human_manager.cpp: se a partida é a que o técnico humano escolheu assistir |

Mutações: `mutacoes/leva214.py` (9, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 8 acusadas. `lt_ultimo_dia` escapou: a margem só é pedida pela 0x68a3e0 (importância do jogo), que grava o byte num campo; no jogo real comparando (até 2002-01-01) ela não é chamada, e na carreira até 2002-06-01 o 0xfe no lugar de 0xff (sem margem) não muda nenhuma decisão. (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (5afd987e1a1a3ef3). Carreiras Rust ×2 e original até 2002-06-01: 2.361.839.396 sorteios, impressão b11bd5cb63f51d72, partidas 2461da5b0d296dba, banco 4d07a03c94c6fda5

### Motor de partidas, 215ª leva — o fim das telas da partida: a partida que o humano deixa e a ficha do jogo de um registro de estatística (oficial em 23/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `largar_partida_assistida()` | 0x6b6970 | cdecl | match_day.cpp: o humano da vez deixa a partida que assistia; sem outro humano assistindo, a partida anda até o fim, é registrada e sai da fila |
| `ficha_do_registro()` | 0x729e30 | thiscall | match_stats.cpp: acha na agenda o jogo de um registro de estatística (data, competição e clube) e copia a ficha |

Mutações: `mutacoes/leva215.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.368.478.169 sorteios, impressão b28542870027e432, partidas 959a02ada4a23639, banco ce037e0141221551

### Simulação, 216ª leva — se a liga tem rebaixamento, a data da rodada do modelo da liga e a importância do jogo da liga; as cópias do recordista, do registro de partida e do registro de sequência dos recordes do clube; a partida assistida por algum humano, o apelido dado pelo humano e a cópia da aposentadoria pendente (oficial em 23/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `liga_tem_rebaixamento()` | 0x6847b0 | thiscall | league.cpp: se a liga tem vagas de rebaixamento no fim da tabela |
| `data_da_rodada_da_liga()` | 0x68a160 | cdecl | league.cpp: grava num registro de rodada do modelo da liga a data com o ano relativo, o código e um valor |
| `importancia_do_jogo_da_liga()` | 0x68a3e0 | thiscall | league.cpp: a importância do jogo da liga para o clube: a base da liga, +1 no começo do campeonato ou com a margem do último dia |
| `copiar_recordista()` | 0x448490 | thiscall | club_records.cpp: copia o recordista: quatro campos de identificação e a data |
| `copiar_registro_de_partida()` | 0x448110 | thiscall | club_records.cpp: copia o registro de partida: a data, o adversário, o placar e os campos do jogo |
| `copiar_registro_de_sequencia()` | 0x448170 | thiscall | club_records.cpp: copia o registro de sequência: o número de jogos e as datas de início e fim |
| `humano_assiste_a_partida()` | 0x5fe420 | thiscall | human_manager.cpp: se algum dos técnicos humanos ativos escolheu assistir a partida |
| `humano_deu_apelido()` | 0x600740 | thiscall | human_manager.cpp: se o técnico humano (ou o da vez) deu apelido à pessoa, por busca binária na lista dele |
| `copiar_aposentadoria_pendente()` | 0x7a8780 | thiscall | player_regen.cpp: copia a aposentadoria pendente: a pessoa, o clube, as duas datas, o motivo e a marca de anunciada |

Mutações: `mutacoes/leva216.py` (9, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 8 acusadas. `ha_motivo` escapou: o motivo (+0x18) da aposentadoria pendente copiada não é lido de forma que decida até 2002-06-01, e no jogo real comparando (até 2002-01-01) a cópia não pega nenhuma aposentadoria com motivo que mude (as aposentadorias se resolvem no fim da temporada). (lacuna em `PENDENCIAS.md`) Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.368.478.169 sorteios, impressão b28542870027e432, partidas 959a02ada4a23639, banco ce037e0141221551

### Motor de partidas, 217ª leva — o fim das estatísticas da partida: o construtor (com a conferência do CD) e o destrutor (oficial em 23/10/2026)

| Rotina | Original | Convenção | Observação |
|---|---|---|---|
| `montar_estatisticas_da_partida()` | 0x729000 | thiscall | match_stats.cpp: confere o CD do CM01/02 e monta as estatísticas: carrega do save ou cria o índice, os baldes e o arquivo temporário zerado |
| `desmontar_estatisticas_da_partida()` | 0x7293c0 | thiscall | match_stats.cpp: libera o índice dos registros e zera o ponteiro e a quantidade |

Mutações: `mutacoes/leva217.py` (0, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): 0 acusadas.  Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.368.478.169 sorteios, impressão b28542870027e432, partidas 959a02ada4a23639, banco ce037e0141221551
