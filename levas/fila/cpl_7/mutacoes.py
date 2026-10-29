# Mutacoes (com o ramo de cada uma). O interesse sem a conta subir (o comparando acusa pela conta; a carreira pelos
# boatos que a 0x754a30 deixa de sortear para o interesse perdido). O interesse repetido aceito (o comparando acusa pela
# conta; a carreira pelos interesses em dobro: mais sorteios de boato). O boato ja dado como saido (+0x10 = 1: o
# comparando acusa pelo registro; a carreira porque a 0x754a30 nao sorteia o boato desse interesse). Nenhuma nas
# comparacoes 0x757ea0 e 0x757ee0 (quentes: qsort e bsearch).
MUTACOES = [
    ('cpl_interesse_conta', 'cpl7_por(this + 0x14, n); // mais um interesse',
     'cpl7_por(this + 0x14, n.wrapping_sub(1)); // mutacao: a conta nao sobe',
     'cpl_7 interesse novo'),
    ('cpl_interesse_repetido', 'return; // o interesse já está',
     '// mutacao: o repetido entra',
     'cpl_7 interesse repetido'),
    ('cpl_interesse_boato', '*((item() + 0x10) as *mut u8) = 0; // o boato ainda não saiu',
     '*((item() + 0x10) as *mut u8) = 1; // mutacao: o boato ja saiu',
     'cpl_7 interesse novo'),
]
