# Mutacoes (cqf_9), todas na 0x7a32e0 (a lesao pelo nome: 162 linhas INJURY do player_setup.cfg aplicadas na criacao do
# jogo da carreira automatica; o tipo decide a duracao e a gravidade da lesao do jogador desde o comeco).
# - O tipo de cada nome um a mais (todas as lesoes do arquivo trocadas pela seguinte).
# - Nenhum nome bate: todas viram 0x4b (a lesao desconhecida).
# - O ultimo (ARTHRITIS) e o desconhecido trocados.
MUTACOES = [
    ('cqf9_tipo_mais_1', 'return tipo as u32; // cqf9: o tipo da lesão pelo nome',
     'return tipo as u32 + 1; // mutacao: o tipo da lesão pelo nome', 'cqf_9 lesao pelo nome'),
    ('cqf9_nenhum_bate', 'if strncmp(nome, texto, n) == 0 {',
     'if strncmp(nome, texto, n) == 0 && false {', 'cqf_9 lesao pelo nome'),
    ('cqf9_artrite_trocada', 'if strncmp(nome, 0xa6b3a8, 8) == 0 { 0x4c } else { 0x4b }',
     'if strncmp(nome, 0xa6b3a8, 8) == 0 { 0x4b } else { 0x4c }', 'cqf_9 lesao pelo nome'),
]
