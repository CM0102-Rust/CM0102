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
