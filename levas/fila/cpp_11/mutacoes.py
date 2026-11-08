# Mutacoes. O par proibido liberado (Iraque contra qualquer uma, Ira e EUA...: a 0x5cbdf0 marca o amistoso que a
# original recusa; o ramo diz se algum par proibido chegou a ser perguntado). As duas ordens invertidas (consistentes,
# o qsort termina): a 0x5cbb00 (515 chamadas) e a lista das nacoes 0x5ca3e0 percorrem as nacoes e os amistosos na
# ordem do qsort, entao os pares e as datas dos amistosos internacionais mudam. As ordens sao quentes: sem ramo.
MUTACOES = [
    ('cpp_par_proibido', 'return 1; // o par proibido',
     'return 0; // mutacao: o par proibido joga',
     'cpp_11 forbidden pair'),
    ('cpp_ordem_das_nacoes', 'rb - ra // a maior reputação primeiro',
     'ra - rb // mutacao: a menor primeiro'),
    ('cpp_ordem_dos_amistosos', 'r // a data mais cedo primeiro',
     '-r // mutacao: a mais tarde primeiro'),
]
