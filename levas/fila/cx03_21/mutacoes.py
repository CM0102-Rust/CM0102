# Mutacoes (cx03_21), carreira com as ligas da Suecia ate 2002-08-01. Nenhuma abre caixa nem trava (as cabecas seguem 12
# para 12 vagas; as secoes e as rodadas seguem com o mesmo numero de clubes). O comparador de cada rotina (lado a lado)
# acusa; a reserva e as rodadas mudam a copa (a carreira).
# - as secoes pares com 1 cabeca e as impares com 2;
# - os inscritos proprios com a classe 1;
# - a final da chave principal sem a parte do fundo para o vencedor.
MUTACOES = [
    ('cx03_21_cabecas_trocadas',
     'if g % 2 == 0 { (0x1e, 2) } else { (0x1f, 1) };',
     'if g % 2 == 0 { (0x1e, 1) } else { (0x1f, 2) }; // mutation'),
    ('cx03_21_inscritos_com_classe_um',
     '        eng_cc_cup_write_entrant(this, k, read_usize(pool + k as usize * 4), 0);',
     '        eng_cc_cup_write_entrant(this, k, read_usize(pool + k as usize * 4), 1); // mutation'),
    ('cx03_21_final_sem_premio',
     '    cql12r((8, 4, 1, 3), (0x15, 4, 1, 2, 1, 4), 0x96, (0, 0, 0x83), 6, (2, 1, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),',
     '    cql12r((8, 4, 1, 3), (0x15, 4, 1, 2, 1, 4), 0x96, (0, 0, 0x83), 6, (2, 1, 0, 0), (0, 2, 7), (0, 0, 0)), // mutation'),
]
