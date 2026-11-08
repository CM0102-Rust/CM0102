# Mutacoes. O fator do treinador entra como (fator - 100) / 5 no peso do sorteio do progresso de cada categoria
# (0x8b1750): com 110 o peso sobe 2, o limit dos sorteios 0x90cfc0 muda e os atributos treinados mudam (o motor das
# partidas le os atributos). Tecnico do computador: todo clube sem humano, quente, sem contador. Sem tecnico: os clubes
# entre a saida e a chegada do tecnico (so pega se a carreira tiver clube sem tecnico com jogador treinando). Clube
# humano com treinadores de nota na categoria: a media sem o divisor (soma / 0,5) sobe o fator (so pega se o humano de
# ferias tiver comissao montada; senao o ramo nao roda e fica lacuna medida).
MUTACOES = [
    ('cpt8_computador_110', 'return (t as u32 & 0xffff_0000) | 100; // cpt8: técnico do computador',
     'return (t as u32 & 0xffff_0000) | 110; // mutacao: tecnico do computador 110'),
    ('cpt8_sem_tecnico_110', 'return 100; // cpt8: sem técnico',
     'return 110; // mutacao: sem tecnico 110',
     'cpt_8 sem tecnico'),
    ('cpt8_media_sem_divisor', 'soma = match_pl_ftol(soma as f64 / coach_average_divisor(n)) as i16; // cpt8: a média dos treinadores',
     'soma = match_pl_ftol(soma as f64 / 0.5) as i16; // mutacao: a media sem o divisor',
     'cpt_8 treinadores'),
]
