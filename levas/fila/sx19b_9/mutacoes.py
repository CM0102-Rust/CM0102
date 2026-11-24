# Mutacoes (sx19b_9), item de TELAS: a carreira automatica nao passa; servem para provar os comparadores numa sessao do
# Lucas jogando comparando (a tela de busca dos olheiros):
# - os filtros copiados com um byte a menos (o comparador dos filtros acusa quando o ultimo byte muda);
# - a carga que falha deixa o desfazer guardado (o comparador acusa ao carregar um arquivo que nao e de busca).
MUTACOES = [
    ('sx19b_9_filtro_curto',
     'core::ptr::copy(filters as *const u8, (s + 0x1fa) as *mut u8, 0x99); // sx19b_9: the filters (+0x1fa)',
     'core::ptr::copy(filters as *const u8, (s + 0x1fa) as *mut u8, 0x98); // mutation: one byte short'),
    ('sx19b_9_desfazer_fica',
     'write_u32(this + SESSION_HAS_UNDO, 0); // sx19b_9: the undo spent',
     '// mutation: the undo kept',
     'sx19b_9 load undone'),
]
