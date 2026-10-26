// ---------- %%LEVA%%a leva: o adjetivo de nacionalidade pelo idioma e pela forma (index.cpp) ----------

// As tabelas da nation_get_nationality_adjective, geradas do exe por `python engenharia-reversa/adjetivos_nacao.py`
// (rust/src/adjetivos_nacao.rs).
include!("adjetivos_nacao.rs");

/// { 0x6181d0 } (cdecl, index.cpp; 0x6181d0..0x62a67a, o mapa de funções corta em 26.014 bytes):
/// `nation_get_nationality_adjective(nacao, texto, tamanho, genero)`: o adjetivo da nação em `texto`. Zera o texto
/// antes de tudo; nação nula, texto nulo ou tamanho (curto com sinal) < 50: 0. Gênero (byte) 0: o nome da nação
/// (+0x57). Nos idiomas 9, 4, 5, 10 e 6 (0x66f4d0) a forma vem da 0x545a60(gênero, idioma) e as cadeias da tabela dão o
/// texto: em cada cadeia a primeira chave igual ao id (+0) copia a fonte da forma (0..4, as outras pela 6ª coluna; 0
/// não copia, 1 é o nome) e passa à cadeia seguinte; no fim, com o texto vazio, o nome (nos idiomas 4 e 5 não: fica
/// vazio). Nos outros idiomas (o inglês, 0), o nome. Devolve 1. Sem SEH, sem locais, sem sorteio.
unsafe extern "C" fn adjetivo_da_nacao(nacao: usize, texto: usize, tamanho: u32, genero: u32) -> u32 {
    core::ptr::write_volatile(texto as *mut u8, 0); // antes dos testes, como a original (texto nulo: a mesma falha)
    if nacao == 0 || texto == 0 || (tamanho as u16 as i16) < 50 {
        return 0;
    }
    let nome = nacao + 0x57; // o nome da nação
    if genero as u8 != 0 {
        let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
        let i = idioma();
        if let Some(&(_, nome_se_vazio, cadeias)) = ADJETIVOS_DA_NACAO.iter().find(|x| x.0 == i) {
            let forma: unsafe extern "C" fn(u32, i32) -> u32 = core::mem::transmute(0x545a60usize);
            let f = forma(genero, i) as u8 as usize;
            let id = core::ptr::read_unaligned(nacao as *const i32);
            for cadeia in cadeias {
                if let Some((_, fontes)) = cadeia.iter().find(|e| *(e.0 as *const i32) == id) {
                    match fontes[f.min(5)] { // as formas 5..255 pela última coluna
                        0 => {}
                        1 => dbu1_copiar(texto, nome),
                        t => dbu1_copiar(texto, t as usize),
                    }
                }
            }
            if !nome_se_vazio || *(texto as *const u8) != 0 {
                return 1;
            }
        }
    }
    dbu1_copiar(texto, nome);
    1
}

unsafe extern "C" fn comparar_adjetivo_da_nacao(nacao: usize, texto: usize, tamanho: u32, genero: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(%%I0%%);
    if texto == 0 {
        return o(nacao, texto, tamanho, genero);
    }
    // o texto produzido (até o tamanho dado, de 1 a 128 bytes; abaixo de 50 só o zero do começo)
    let n = ((tamanho as u16 as i16).max(1) as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, n)], || o(nacao, texto, tamanho, genero),
        || adjetivo_da_nacao(nacao, texto, tamanho, genero));
    PLACAR.registrar(igual && r == x, || format!("adjetivo_da_nacao({:x}, tamanho {}, genero {}) original={} rust={} {:x?} x {:x?}",
        nacao, tamanho as u16 as i16, genero as u8, r, x, mo, mr));
    r
}
