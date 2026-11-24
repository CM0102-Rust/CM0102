# Mutacoes (sx20_14), item de TELAS: a carreira automatica nao toca som; servem para provar o comparador numa sessao
# do Lucas jogando comparando, com o som ligado. O que mexe no DirectSound so roda a original no comparador (as
# mutacoes dessas rotinas se ouvem no jogo Rust, nao no comparador); as consultas comparam a resposta:
# - find_free_slot tambem devolve vaga ocupada (o comparador acusa quando um som esta tocando);
# - o lado direito com a panoramica da esquerda (ouve-se no modo Rust).
MUTACOES = [
    ('sx20_14_vaga_ocupada',
     'if buffer != 0 && !sound_buffer_busy(buffer) { // sx20_14: an idle buffer',
     'if buffer != 0 { // mutation: a busy buffer too'),
    ('sx20_14_lado_trocado',
     '1 => 5000, // sx20_14: the right pan',
     '1 => -5000, // mutation: the left pan'),
]
