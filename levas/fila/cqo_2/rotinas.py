# O público de um jogo (stadium.cpp): Stadium::calculate_match_attendance (0x85cba0, 3,3 KB de x87; ~16,5 mil chamadas
# na carreira de 8 ligas até 2002-06-01: todo jogo que o motor prepara, 0x6b7f20, fora os da Copa do Mundo).
# O público entra na renda do jogo (as finanças dos clubes), no motor (+0x91d2..+0x91da e +0xf634) e nos lugares de
# estádio com menos de 1.500 (que ela sobe).
# Igual no cm0102-1080.exe (inteira).
# Convenções conferidas no ponto de chamada da original (lição da 258ª):
# - 0x85cba0 (pelo motor 0x6b7f20, 0x6b86ae): mov ecx, [0xdd7ef4]; push &saída (motor +0x91d2); push jogo; sem add esp
#   -> thiscall(this, jogo, saída), ret 8.
# - dentro: 0x5c6710 push comp; mov ecx, [0xae2c74] -> thiscall, ret 4 (Rust); 0x7e6f20 push comp (o ecx = [0xb74434]
#   só é testado) -> stdcall de 1, ret 4 (Rust); 0x90cfc0 push n; add esp, 4 -> cdecl (Rust); 0x85d8a0 push base (double,
#   2 dwords), cidade do clube, cidade do estádio, jogo; mov ecx, this -> thiscall, ret 0x14, double no st0 (Rust);
#   0x4a4850 push clube; mov ecx, [liga +8] -> thiscall, ret 4 (original: a linha do clube na tabela); 0x945640 _ftol
#   (ftol do motor.rs).
# - o ax da sequência (0x85d497: xor ax, ax; movzx di, cl; mov al, bl; sub eax, edi): os 16 bits baixos de bl − cl,
#   com sobra nos altos que ninguém lê (movsx eax, ax).
# Comparador: lado a lado (a cópia sem contar os sorteios) pelo estado do sorteador, pela saída (12 bytes), pelos lugares
# do estádio (+0x3c) e pela devolução. Sem jogo, só a original.
# Determinismo: as contas em f64 na ordem da original (x87 com precisão de 53 bits); só sorteios do jogo.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('publico_do_jogo', 0x85cba0, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('o público de um jogo',
        'the attendance of a match',
        [('publico_do_jogo', 'Stadium::calculate_match_attendance', '0x85cba0', 'stadium.cpp', 'thiscall, ret 8',
          'o público do jogo: um fator por lado (o número de clubes da competição, +2 nas copas de tipo 6/8/10, a diferença da rodada na liga entre −6 e 5; pela reputação/500 dos dois clubes: o maior atrai mais, ×1,1 contra quem passa de 17, ×1,25 contra quem passa de 12; dividido pelo número de clubes da liga do clube), com pisos por tipo de copa (finais e semifinais, +0x30/+0x32 = 150/130/160); o mandante leva a média de público × fator × distância (0x85d8a0, mínimo 0,15), o visitante (média + mínimo) × fator × distância × 0,5; na liga, bônus pela posição (+50/100/150 por clube na ponta, +25/50 na briga, −25/−10 embaixo) e pela sequência; clássico ×1,5 (mesma cidade ×1,35), fora do neutro ×1,05/×0,85; limitado pela lotação (o visitante entre 5% e 15%) e o estádio de menos de 1.500 lugares passa a reputação/500 × 1.000; total = os dois − sorteio(50)',
          'the match attendance: one factor per side (number of clubs in the competition, +2 for cup types 6/8/10, the league round difference between −6 and 5; by the reputation/500 of both clubs: the bigger one draws more, ×1.1 against sides above 17, ×1.25 against sides above 12; divided by the number of clubs in the club\'s league), with floors by cup type (finals and semi-finals, +0x30/+0x32 = 150/130/160); the home side gets the average gate × factor × distance (0x85d8a0, at least 0.15), the away side (average + minimum) × factor × distance × 0.5; in the league, a bonus by position (+50/100/150 per club at the top, +25/50 in the race, −25/−10 at the bottom) and by form; derby ×1.5 (same city ×1.35), off neutral ground ×1.05/×0.85; capped by the capacity (away fans between 5% and 15%) and a ground under 1,500 becomes reputation/500 × 1,000; total = both − random(50)')])
CAMPOS = [('partida +0x14 competição (+0 id, +0x69 número de clubes); +0x18 estádio (+0x38 cidade, +0x3c lugares; grava os lugares abaixo de 1.500); +0x1c/+0x20 mandante e visitante; +0x30/+0x32 [inferência: a fase da copa: 150 final, 130 semifinal, 160]; +0x34 a rodada; +0x3a tipo da competição; +0x3b; +0x3c neutro; lê', 'fixture +0x14 competition (+0 id, +0x69 number of clubs); +0x18 stadium (+0x38 city, +0x3c capacity; writes capacities below 1,500); +0x1c/+0x20 home and away; +0x30/+0x32 [inference: the cup stage: 150 final, 130 semi-final, 160]; +0x34 round; +0x3a competition type; +0x3b; +0x3c neutral; reads'),
          ('clube +0x57 liga (ponteiro); +0x69 estádio; +0x6d [inferência: público garantido]; +0x73 média de público, +0x77 mínimo, +0x7b máximo; +0x80 reputação; +0xb3/+0xb7/+0xbb os rivais; lê', 'club +0x57 league (pointer); +0x69 stadium; +0x6d [inference: guaranteed gate]; +0x73 average attendance, +0x77 minimum, +0x7b maximum; +0x80 reputation; +0xb3/+0xb7/+0xbb rivals; reads'),
          ('liga [0xadadfc][id] +8 a tabela, +0x3c a rodada, +0x43; linha da tabela (0x4a4850) +4 jogos, +0x19, +0x2d, +0x41, +0x55 [inferência: as faixas de posição], +0x72..+0x75 [inferência: a sequência]; lê', 'league [0xadadfc][id] +8 the table, +0x3c round, +0x43; table row (0x4a4850) +4 games, +0x19, +0x2d, +0x41, +0x55 [inference: position bands], +0x72..+0x75 [inference: the form]; reads')]
INTERESSA = ['CMRTE']
