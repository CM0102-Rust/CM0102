# Mutacoes das copias e construtores dos recordes do clube. Os recordes do clube ficam fora da impressao da carreira
# (como os da 216a): PODEM ESCAPAR da carreira; o comparando acusa na primeira chamada. A dos construtores escreve a
# data padrao um byte adiante (pisa o recordista vizinho).
MUTACOES = [
    ('cpf_contagem', 'core::ptr::write_unaligned(this as *mut u16, core::ptr::read_unaligned(origem as *const u16)); // a contagem',
     'core::ptr::write_unaligned(this as *mut u16, core::ptr::read_unaligned(origem as *const u16).wrapping_add(1)); // mutacao: uma a mais'),
    ('cpf_torcedor_ano', 'core::ptr::write_unaligned((this + 0x18) as *mut u16, core::ptr::read_unaligned((origem + 0x18) as *const u16)); // o ano do torcedor',
     'core::ptr::write_unaligned((this + 0x18) as *mut u16, 0); // mutacao: o ano zerado'),
    ('cpf_ocorrencias_data', 'cpf2_data_padrao(this + 0x11); // a data das ocorrências',
     'cpf2_data_padrao(this + 0x10); // mutacao: a data um byte antes'),
    ('cpf_contagem_data', 'cpf2_data_padrao(this + 0x12); // a data da contagem',
     'cpf2_data_padrao(this + 0x11); // mutacao: a data um byte antes'),
]
