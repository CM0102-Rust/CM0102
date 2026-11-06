# O construtor da primeira divisão francesa (Agente B da frente das competições, 05-06/11/2026). Roda uma vez na
# criação da carreira de 8 ligas (docs/cobertura_competicoes.txt: 0x5bb160 1 vez):
# - 0x5bb160 FraFirst::FraFirst (988 bytes, quadro SEH só para os operator new, sem destrutor no caminho normal).
# Igual no cm0102-1080.exe (inteira).
# Convenções conferidas nos pontos de chamada da original:
# - quem chama: push registro; push ano; mov ecx, liga -> thiscall, ret 8 (o `ret 8` conferido);
# - 0x682200 (League::League): mov ecx, esi -> thiscall, ret 0;
# - 0x944cf1 (operator new): push n; add esp, 4 -> cdecl;
# - 0x48c580 (Comp::Comp): mov ecx, novo -> thiscall, ret 0;
# - 0x687b10 (League::load_from_temp_file): push 1; mov ecx, esi -> thiscall, ret 4;
# - 0x5bc410 (FraFirst::set_up_rules) e 0x5bc2b0 (FraFirst::create_table): mov ecx, esi -> thiscall, ret 0;
# - 0x6835c0 (League::create_fixtures): mov ecx, esi -> thiscall, ret 0;
# - 0x6827d0 (League::sort_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x49ee70 (Comp_Stats::Comp_Stats): push esi; mov ecx, novo -> thiscall, ret 4;
# - 0x68a850 (League::set_predicted_positions): mov ecx, esi -> thiscall, ret 0;
# - 0x5b91c0 (FraCfa::FraCfa) e 0x5bd600 (FraLower::FraLower): push ano (o ecx com só o cx de +0x40); mov ecx, novo
#   -> thiscall, ret 4;
# - a virtual 0 das duas ligas de fundo (o destrutor que solta): push 1; mov ecx, liga -> thiscall, ret 4;
# - a caixa: aviso_em.
# Comparador: aloca, sorteia (as ligas de fundo) e mexe nos clubes: só a original, conferindo a devolução, a vtable, o
# ano, o registro, a disciplina, os dois marcadores e, na liga nova, os campos do começo e as estatísticas.
# Determinismo: só campos do banco e globais do jogo; nada de pilha, heap ou endereço como valor (os ponteiros vão para
# campos, como na original).
# Provas (05-06/11/2026, cópia fora do projeto: o motor.rs da principal com a 277ª e a 278ª + os 9 itens cqo_4..6,
# cqp_4..6, cqq_4..6, 17 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO (motor 10^10, data 10^9, aleatorio 10^9 comparações, 0 divergências); o ramo da liga nova rodou;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base): as duas ACUSADAS: a divisão de baixo antes
#   da CFA divergiu em 2001-209; a outra disciplina em 2001-213. As primeiras (a competição de reserva trocada)
#   derrubavam o jogo no 1o dia (0xC0000005, as listas das ligas de fundo com clubes nulos): trocadas.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construir_primeira_francesa', 0x5bb160, [0x6a, 0xff, 0x68, 0x81, 0x99, 0x95, 0x00])]
DOCS = ('o construtor da primeira divisão francesa (os marcadores, os clubes da CFA e da divisão de baixo e as ligas de '
        'fundo)',
 'the French first division constructor (the placeholders, the CFA and lower division clubs and the background '
 'leagues)',
 [('construir_primeira_francesa', 'FraFirst::FraFirst', '0x5bb160', 'fra_first.cpp', 'thiscall, ret 8',
   'monta a primeira divisão francesa: disciplina 0xc; duas competições-marcador (a CFA e a divisão de baixo), sem '
   'jogos próprios; numa carreira nova, todo clube cujo campeonato no banco é a CFA passa à divisão genérica com a CFA '
   'como competição de reserva [inferência: os times B dos clubes profissionais], e todo clube da divisão de baixo '
   'passa à genérica sem reserva; depois as regras, a tabela, os jogos, a ordem, as estatísticas e as posições '
   'previstas; por fim monta a CFA (4 grupos de 18) e a divisão de baixo (2 grupos de 20), que deixam os seus jogos '
   'no calendário, e solta os dois objetos',
   'builds the French first division: discipline 0xc; two placeholder competitions (the CFA and the lower division), '
   'with no matches of their own; in a new career, every club whose league in the database is the CFA moves to the '
   'generic division with the CFA as reserve competition [inference: the B sides of the professional clubs], and every '
   'lower division club moves to the generic division with no reserve; then the rules, the table, the fixtures, the '
   'order, the statistics and the predicted positions; at last it builds the CFA (4 groups of 18) and the lower '
   'division (2 groups of 20), which leave their matches in the calendar, and frees both objects')])
CAMPOS = [('liga +0x40 ano, +4 registro, +0x50 disciplina, +0xee/+0xf2 os marcadores da CFA e da divisão de baixo, +0x44 '
           'fase, +0x30 fase atual, +0x2c/+0xc fases, +8 estatísticas; valor e ponteiro; grava',
           'league +0x40 year, +4 record, +0x50 discipline, +0xee/+0xf2 the CFA and lower division placeholders, +0x44 '
           'stage, +0x30 current stage, +0x2c/+0xc stages, +8 statistics; value and pointer; writes'),
          ('clube +0x57 competição, +0x60 competição de reserva; ponteiro; lê e grava',
           'club +0x57 competition, +0x60 reserve competition; pointer; reads and writes'),
          ('marcador (Comp, 0xa3 bytes): +4 registro, +0x42 1, +0x50 0xff; grava',
           'placeholder (Comp, 0xa3 bytes): +4 record, +0x42 1, +0x50 0xff; writes')]
INTERESSA = ['CMRTE']
