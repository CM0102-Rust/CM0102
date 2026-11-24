# Database.cpp (leitor sx08, rodada 5, 23/11/2026): as preferencias de uma pessoa, irmas das ja portadas
# staff_add_favoured_staff 0x547a10 e staff_add_disliked_club 0x547e70 (mesmo nucleo database_add_to_preference_list).
# Convencoes: as duas cdecl(pessoa, outro, forcar), sem retorno usado; 0x90cfc0(3) o sorteio da vaga forcada.
# - 0x547af0: sai tambem quando outro == pessoa (antes do teste de nulo);
# - 0x547d90: o teste de saida e o clube GOSTAR da pessoa (club+0x9b/+0x9f/+0xa3), nao ela ja ser favorita.
# ATENCAO na integracao: 0x547af0 e 0x547d90 estao nas POS_PROLOGUES (match_day.rs, as portas do pos-jogo, desviadas
# so no modo de comparacao, antes das rotinas). Na comparacao o desvio da rotina seria recusado (bytes ja trocados).
# Fazer como na 0x867560: no instalar() do motor.rs, acrescentar as duas a condicao
#   `if comparando && (PROLOGOS[i].address == 0x6f5c30 || ... || PROLOGOS[i].address == 0x867560)`
# (`|| PROLOGOS[i].address == 0x547af0 || PROLOGOS[i].address == 0x547d90`). No modo Rust a rotina e desviada e as
# portas nao existem. Os comparadores ficam escritos (servem a um autoteste ou a uma porta que chame a comparacao).
# Tipo: simulacao (pos-jogo, lesoes, transferencias; a carreira ENG ate 2002-06-01).
SAVE = False
ROTINAS = [('staff_add_disliked_staff', 0x547af0, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('staff_add_favoured_club', 0x547d90, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c])]
DOCS = ('as preferências de uma pessoa: a antipatia por outra pessoa e o clube favorito',
        "a person's preferences: disliking another person and the favoured club",
        [('staff_add_disliked_staff', 'staff_add_disliked_staff', '0x547af0', 'Database.cpp', 'cdecl',
          'põe a outra pessoa entre as três de que a pessoa não gosta: nada se for ela mesma ou faltar alguém; já está, nada; se estava entre as três favoritas, deixa de estar e para; senão a primeira vaga livre; sem vaga, com "forçar", uma vaga sorteada entre as três',
          'puts the other person among the three the person dislikes: nothing if it is the person herself or someone is missing; already there, nothing; if among the three favoured, stops being one and stops; otherwise the first free slot; no free slot, with "force", a slot drawn among the three'),
         ('staff_add_favoured_club', 'staff_add_favoured_club', '0x547d90', 'Database.cpp', 'cdecl',
          'põe o clube entre os três favoritos da pessoa: se o clube já gosta da pessoa (entre os três favoritos do clube), nada; se o clube está entre os três de que a pessoa não gosta, deixa de estar e para; senão a primeira vaga livre (sem conferir se já é favorito, como no original); sem vaga, com "forçar", uma vaga sorteada entre as três',
          "puts the club among the person's three favoured clubs: if the club already favours the person (among the club's three favoured staff), nothing; if the club is among the three the person dislikes, it stops being one and stops; otherwise the first free slot (without checking whether it is already favoured, as in the original); no free slot, with \"force\", a slot drawn among the three")])
CAMPOS = []
INTERESSA = []
