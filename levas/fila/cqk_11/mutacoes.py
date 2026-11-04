# Mutacoes (cqk_11), na Libertadores (os clubes brasileiros, liga jogavel): os 32 classificados (0x4c3f40, 2 chamadas
# ate 2002-06-01 com as 8 ligas) embaralhados com menos trocas mudam a contagem dos sorteios e os grupos; o mata-mata
# criado uma fase antes (0x4c3a80, 2 chamadas) muda quem joga em maio; o campeao na vaga errada do Mundial Interclubes
# (0x4c4080, so se a final cair na janela: o contador diz). Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqk11_embaralha_so_31',
     'embaralhar(lista, cqk11_w(this + 0xf2) as u16 as u32); // cqk11: os 32 da Libertadores embaralhados',
     'embaralhar(lista, 0x1f); // mutacao: embaralha so 31',
     'cqk_11 classificados da libertadores'),
    ('cqk11_campeao_na_vaga_1',
     'core::ptr::write_unaligned(t as *mut u32, clube as u32); // cqk11: o campeão na vaga sul-americana',
     'core::ptr::write_unaligned((t + 6) as *mut u32, clube as u32); // mutacao: o campeao na outra vaga',
     'cqk_11 campeao da libertadores'),
    ('cqk11_classificados_trocados_aos_pares',
     'core::ptr::write_unaligned((lista + k * 4) as *mut u32, c as u32); // cqk11: o clube na ordem do banco',
     'core::ptr::write_unaligned((lista + (k ^ 1) * 4) as *mut u32, c as u32); // mutacao: trocados aos pares',
     'cqk_11 classificados da libertadores'),
]
