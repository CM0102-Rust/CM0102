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
