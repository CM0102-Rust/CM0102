# Mutacoes (cqk_9): a MLS (temporada ate outubro de 2001, play-offs, Jogo das Estrelas), antes de 2002-06-01.
# - Os classificados para os play-offs nao ganham o registro de classificacao (o historico do tecnico). (Mudar so o
#   peso escapa: os pontos do tecnico nao chegam a impressao antes de 2002-06-01, visto na cqk_7.)
# - O jogo da temporada regular nao entra na tabela da segunda conferencia (a tabela dela e a classificacao mudam).
# - Os seis primeiros clubes fixos vao para a conferencia 0x414 no Jogo das Estrelas (os escolhidos mudam).
MUTACOES = [
    ('cqk9_sem_registro_classificado_mls', 'cqk9_classificou(clube, comp, 0xa0, 0x78, 0x28); // cqk9: o classificado da MLS',
     '// mutacao: sem o registro do classificado da MLS', 'cqk_9 mls classificado'),
    ('cqk9_sem_segunda_conferencia', 'f1(e1, jogo, estat); // cqk9: o jogo na tabela da segunda conferência',
     'let _ = (e1, f1); // mutacao: o jogo na tabela da segunda conferência', 'cqk_9 mls segunda conferencia'),
    ('cqk9_conferencia_trocada', 'return 0x413; // cqk9: a primeira conferência',
     'return 0x414; // mutacao: a primeira conferência', 'cqk_9 conferencia 0x413'),
]
