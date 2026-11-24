# Os construtores da competicao e da copa e as regras de substituicao da partida (comp.cpp, Cup.cpp; leitor cx01,
# rodada 5, 24/11/2026). Carreira ENG (sem LIGAS): os construtores rodam na criacao do jogo (toda liga e toda copa);
# Comp::set_fixture_substitutes (slot 0x7c, o mesmo na League e na Cup) em toda partida criada.
# - 0x48c580 Comp::Comp thiscall() ret 0 -> this: so grava campos (chamado pelo Cup::Cup e pelos construtores da League,
#   da ReserveComp, da sub-21 e da B: call direto com o ecx do objeto).
# - 0x518640 Cup::Cup thiscall() ret 0 -> this: Comp::Comp (call 0x48c580, mov ecx ja no esi) e os campos da copa.
#   Prologo push esi; mov esi, ecx; call rel32: RELATIVAS (0x518640, 3) no rust/src/detours.rs.
# - 0x48f2d0 Comp::set_fixture_substitutes thiscall(partida) ret 4: a etapa (+0x44 != 0xff) com competicao-mae na tabela
#   0xadadfc chama o slot 0x7c dela (push partida; mov ecx -> thiscall, ret 4); senao os dois nibbles da palavra +0x4d.
#   Prologo cmp byte [ecx+0x44], 0xff; je rel8: RELATIVAS (0x48f2d0, 4).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: os dois construtores lado a lado sobre o objeto (0x53 e 0xad bytes) e o retorno; a palavra +0x4d da
# partida lado a lado.
SAVE = False
ROTINAS = [('new_comp', 0x48c580, [0x8b, 0xc1, 0x83, 0xca, 0xff]),
           ('new_cup', 0x518640, [0x56, 0x8b, 0xf1, 0xe8, 0x38, 0x3f, 0xf7, 0xff]),
           ('comp_set_fixture_substitutes', 0x48f2d0, [0x80, 0x79, 0x44, 0xff, 0x74, 0x24])]
DOCS = ('os construtores da competição e da copa e as regras de substituição da partida',
        'the competition and cup constructors and the fixture\'s substitute rules',
        [('new_comp', 'Comp::Comp', '0x48c580', 'comp.cpp', 'thiscall, ret 0',
          'monta a base de toda competição: sem estatísticas, etapas nem inscritos, sem divisão acima ou abaixo nem estádio fixo (−1), fase atual −1, contagens zeradas, jogada por clubes, ativa, 5 reservas e 3 substituições, regra disciplinar 0xff e importância 20',
          'builds the base of every competition: no statistics, stages or entrants, no division above or below nor fixed stadium (−1), current stage −1, counts cleared, played by club sides, active, 5 substitutes named and 3 substitutions, discipline rule 0xff and importance 20'),
         ('new_cup', 'Cup::Cup', '0x518640', 'Cup.cpp', 'thiscall, ret 0',
          'monta a base de toda copa: a base da competição, a rodada em montagem e a próxima a sortear zeradas e o tipo copa',
          'builds the base of every cup: the competition base, the round being set up and the next to draw cleared, and the cup type'),
         ('comp_set_fixture_substitutes', 'Comp::set_fixture_substitutes', '0x48f2d0', 'comp.cpp', 'thiscall, ret 4',
          'grava na partida as regras de banco da competição: uma etapa com competição-mãe pergunta à mãe; senão o número de reservas no nibble baixo e o de substituições no seguinte',
          'writes the competition\'s bench rules into the fixture: a stage with a parent competition asks the parent; otherwise the substitutes named in the low nibble and the substitutions allowed in the next')])
CAMPOS = [('competição +0x10, +0x18 ponteiros possuídos; +0x34 palavra; zerados pelo construtor', 'competition +0x10, +0x18 owned pointers; +0x34 word; cleared by the constructor')]
INTERESSA = []
