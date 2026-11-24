# Mutacoes (sx07_2). O texto nao volta para a simulacao: acusadas pela comparacao (compare_text_writer compara o
# texto, os vinculos e o gerador), como as de texto aceitas em 12/11/2026. Em trechos que rodam em toda chamada ou no
# ramo mais comum (contrato aceito sem motivos, recusa com motivos):
# - o valor do contrato sem o valor de assinatura (muda o dinheiro e os limiares);
# - "em casa" nunca marcado (manchetes e recusas com o clube);
# - o acordo simples: renovacao e contrato novo trocados;
# - a recusa com motivos sem os motivos.
MUTACOES = [
    ('sx07_valor_sem_assinatura',
     'let mut value = (days / 7).wrapping_mul(wage).wrapping_add(signing); // sx07_2: the contract value',
     'let mut value = (days / 7).wrapping_mul(wage).wrapping_add(signing * 0); // mutation: no signing value',
     'sx07_2 offer response text'),
    ('sx07_nunca_em_casa',
     'read_u32(read_usize(hm + 0x39)) == slot(4); // sx07_2: at home',
     'read_u32(read_usize(hm + 0x39)) == slot(4) && false; // mutation: never at home',
     'sx07_2 offer response text'),
    ('sx07_acordo_trocado',
     'let tpl = if renewal { 0x9a8b64 } else { 0x9a895c }; // sx07_2: the plain agreement',
     'let tpl = if renewal { 0x9a895c } else { 0x9a8b64 }; // mutation: swapped',
     'sx07_2 offer response text'),
    ('sx07_recusa_sem_motivos',
     'fmt(NEWS_TEXT_BUFFER, 0x9a852c, gl, gl, n, long, j) // sx07_2: rejected with reasons',
     'fmt(NEWS_TEXT_BUFFER, 0x9a84bc, gl, gl, n, long) // mutation: without the reasons',
     'sx07_2 offer response text'),
]
