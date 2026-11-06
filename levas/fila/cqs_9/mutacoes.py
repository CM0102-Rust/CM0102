# Mutacoes (cqs_9), nos inscritos da Copa da UEFA de 2001/02 (montados em 2001-000; os jogos de agosto de 2001 a maio
# de 2002): completa so ate 117 (um clube a menos); quem completa entra com a classe 2 (outro pote no sorteio); a
# preliminar so com coeficiente 11 ou mais (outra conta em +0xb2: outra tabela de rodadas, e com +0xb2 impar nao ha a
# lista da Intertoto).
MUTACOES = [
    ('cqs9_completa_ate_117', 'if cqs9_conta(this, k) >= 0x76 { // cqs9: completa até 118', 'if cqs9_conta(this, k) >= 0x75 { // mutacao', 'cqs_9 clube completando a uefa'),
    ('cqs9_classe_2_de_quem_completa', '*((cqs9_d(this + 0xa7) + e + 4) as *mut u8) = 3; // cqs9: a classe de quem completa', '*((cqs9_d(this + 0xa7) + e + 4) as *mut u8) = 2; // mutacao', 'cqs_9 clube completando a uefa'),
    ('cqs9_preliminar_de_11', 'if *((c + 0x1df) as *const i8) >= 0xa { // cqs9: os da preliminar', 'if *((c + 0x1df) as *const i8) >= 0xb { // mutacao', 'cqs_9 inscritos da uefa'),
]
