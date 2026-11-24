# Mutacoes (cx03_22), carreira com as ligas da Suecia ate 2002-08-01. Nenhuma abre caixa nem muda o numero de clubes das
# fases (o que derrubaria as rodadas). Os comparadores so rodam a original: a carreira acusa.
# - as fases preliminares com a importancia 0x14 em vez do byte +0x51 da copa: a escalacao nos jogos das secoes muda;
# - a chave principal com a importancia 0: a escalacao nos jogos da chave muda.
MUTACOES = [
    ('cx03_22_preliminar_com_importancia_fixa',
     'let (created, table) = swe_cup_new_stage(this, stage, n as i32 as u32, teams, this.importance_bias() as u32);',
     'let (created, table) = swe_cup_new_stage(this, stage, n as i32 as u32, teams, 0x14); // mutation'),
    ('cx03_22_chave_principal_sem_importancia',
     '    let (_, table) = swe_cup_new_stage(this, 7, 0x40, teams, 0x14);',
     '    let (_, table) = swe_cup_new_stage(this, 7, 0x40, teams, 0); // mutation'),
]
