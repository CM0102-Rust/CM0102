// ---------- %%LEVA%%a leva: o texto de um dado da notícia, a notícia do clube em apuro financeiro e se os olheiros do clube cobrem a nação ----------

unsafe fn cpk6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpk6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x785a20 } (thiscall, ret 0xc, news.cpp): `NewsItem::get_slot_string(dado, destino, n)`: copia o texto guardado a
/// partir do dado `dado` (+5 + 4 × dado) para `destino` como o strncpy (0x9466d0: até o zero, completando com zeros
/// até `n`) e zera o último byte (`destino[n − 1]`). Devolve 1. Dado fora de 0..49, destino nulo ou `n` < 1: a
/// original (caixa "Error" e 0).
unsafe extern "thiscall" fn get_slot_string(noticia: usize, dado: u32, destino: *mut u8, n: i32) -> u32 {
    let k = dado as u8 as i8;
    if !(0..0x32).contains(&k) || destino.is_null() || n < 1 {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32>(%%I0%%)(noticia, dado, destino, n); // caixa: só a original
    }
    crate::comparacao::ramo("cpk_6 texto do dado");
    let origem = (noticia + 5 + 4 * k as usize) as *const u8; // o dado
    let mut fim = false;
    for i in 0..n as usize {
        let c = if fim { 0 } else { *origem.add(i) };
        fim |= c == 0;
        *destino.add(i) = c;
    }
    *destino.add(n as usize - 1) = 0;
    1
}

/// { 0x74ae60 } (thiscall, ret 4, media.cpp, quadro SEH sem destrutor): `Media::post_financial_trouble_news(clube)`:
/// com o clube, a nação (+0x53), a ficha (+0x69, com +0x38) e a divisão (+0x57), monta a notícia 0x1790 numa local
/// (0x77bbe0 com 0) com os dados 0 = id do clube, 1 = reputação (+0x80) e 2 = sorteio(2) (0x90cfc0); clube de
/// reputação 9000 ou mais numa divisão com +0x69 acima de 12: para todos os técnicos (0x786780); senão para os da
/// nação (0x7863f0). A notícia local tem 0xe0 bytes (a entrega copia só até +0xde).
unsafe extern "thiscall" fn post_financial_trouble_news(_this: usize, clube: usize) {
    if clube == 0 || cpk6_d(clube + 0x53) == 0 {
        return;
    }
    let ficha = cpk6_d(clube + 0x69);
    if ficha == 0 || cpk6_d(ficha + 0x38) == 0 || cpk6_d(clube + 0x57) == 0 {
        return;
    }
    crate::comparacao::ramo("cpk_6 apuro financeiro");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    construir(n, 0x1790, 0);
    gravar(n, 0, cpk6_d(clube) as u32);
    gravar(n, 1, cpk6_w(clube + 0x80) as i32 as u32);
    let v = sorteio(2); // o sorteio da notícia
    gravar(n, 2, v as u32);
    let noticias = cpk6_d(0xb7431c);
    let divisao = cpk6_d(clube + 0x57);
    if cpk6_w(clube + 0x80) >= 9000 && divisao != 0 && cpk6_w(divisao + 0x69) > 12 {
        let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
        todos(noticias, n);
    } else {
        let pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
        pais(noticias, n, cpk6_d(clube + 0x53));
    }
}

/// O clube que a 0x7f9190 usa: o `clube`, ou o do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]] + 0x39).
unsafe fn cpk6_clube(clube: usize) -> usize {
    if clube != 0 {
        return clube;
    }
    cpk6_d(cpk6_d(0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize as usize) + 0x39)
}

/// { 0x7f9190 } (thiscall, ret 8, scout_manager.cpp; `this` não é lido): `SCOUT_MANAGER::can_club_scout_nation(nacao,
/// clube)`: o clube (ou o do humano da vez) não cobre a nação só se tem as finanças (0x5a0590 em [0xae2a38]) com saldo
/// de 64 bits abaixo de 1.000.000, reputação (+0x80) abaixo de 5000 e a região (+0x76) da nação dele (+0x53) diferente
/// da região da `nacao`: 0. Senão 1. Sem clube nenhum: a original (caixa "Error" e 0).
unsafe extern "thiscall" fn can_club_scout_nation(this: usize, nacao: usize, clube: usize) -> u32 {
    let c = cpk6_clube(clube);
    if c == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%)(this, nacao, clube); // caixa: só a original
    }
    let get_club_finance: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let f = get_club_finance(cpk6_d(0xae2a38), c);
    if f != 0 && core::ptr::read_unaligned(f as *const i64) < 1_000_000 && cpk6_w(c + 0x80) < 5000 {
        if *((cpk6_d(c + 0x53) + 0x76) as *const u8) != *((nacao + 0x76) as *const u8) { // fora da região
            crate::comparacao::ramo("cpk_6 olheiro fora da regiao");
            return 0;
        }
    }
    1
}

/// Comparador: lado a lado pelos `n` bytes do destino.
unsafe extern "thiscall" fn comparar_get_slot_string(noticia: usize, dado: u32, destino: *mut u8, n: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32>(%%I0%%);
    let k = dado as u8 as i8;
    if !(0..0x32).contains(&k) || destino.is_null() || n < 1 {
        return o(noticia, dado, destino, n);
    }
    let reg = [(destino as usize, n as usize)];
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(noticia, dado, destino, n), || get_slot_string(noticia, dado, destino, n));
    PLACAR.registrar(igual && r == m, || format!("get_slot_string({}, {}) {:x?} x {:x?}", k, n, mo, mr));
    r
}

/// Comparador: a notícia sorteia (0x90cfc0) e vai para as caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn comparar_post_financial_trouble_news(this: usize, clube: usize) {
    crate::comparacao::ramo("post_financial_trouble_news");
    orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, clube)
}

/// Comparador: pelo valor (a 0x5a0590 só procura).
unsafe extern "thiscall" fn comparar_can_club_scout_nation(this: usize, nacao: usize, clube: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%)(this, nacao, clube);
    if cpk6_clube(clube) == 0 {
        return r;
    }
    let n = can_club_scout_nation(this, nacao, clube);
    PLACAR.registrar(r == n, || format!("can_club_scout_nation({:x}, {:x}) original={} rust={}", nacao, clube, r, n));
    r
}
