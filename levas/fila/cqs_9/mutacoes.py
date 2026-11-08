# Mutacoes (cqs_9), nos inscritos da Copa da UEFA de 2001/02 (montados em 2001-000; os jogos de agosto de 2001 a maio
# de 2002): a classe dos inscritos zerada (outros potes no sorteio); a lista sem a ordem pela classe; a preliminar so com
# coeficiente 11 ou mais (outra conta em +0xb2). As duas primeiras tentadas (completar ate 117 e a classe 2 de quem
# completa) escaparam em 06/11: com as 8 ligas a classificacao ja enche as 118 vagas e ninguem completa (contador 0). A da preliminar de 11 tambem escapa (nenhum
# clube com coeficiente exatamente 10: +0xb2 nao muda).
MUTACOES = [
    ('cqs9_classe_zero', '*((cqs9_d(this + 0xa7) + e + 4) as *mut u8) = *((c + 0x1df) as *const u8);', '*((cqs9_d(this + 0xa7) + e + 4) as *mut u8) = 0; // mutacao', 'cqs_9 uefa entrants'),
    ('cqs9_sem_ordem_pela_classe', 'ordenar(cqs9_d(this + 0xa7), k as i32 as u32, 6, 0x522190);', 'let _ = ordenar; // mutacao', 'cqs_9 uefa entrants'),
    ('cqs9_preliminar_de_11', 'if *((c + 0x1df) as *const i8) >= 0xa { // cqs9: os da preliminar', 'if *((c + 0x1df) as *const i8) >= 0xb { // mutacao', 'cqs_9 uefa entrants'),
]
