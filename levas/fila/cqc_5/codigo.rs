// ---------- %%LEVA%%a leva: a copa: os jogos do confronto, a notícia do confronto pendente e os avisos das janelas de transferência (Cup.cpp, transfer_manager.cpp) ----------

unsafe fn cqc5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqc5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqc5_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqc5_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

/// O clube é dirigido por um técnico humano (0x7875e0: push clube; ecx = [0xb7431c]; sem add esp -> thiscall(notícias,
/// clube), ret 4).
unsafe fn cup_has_human_manager(clube: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(cqc5_d(0xb7431c), clube) != 0
}

/// A data da partida (+0x2a dia, +0x28 ano, pelos dwords que a original empurra: 0x549ef0 push ano; push dia; lea ecx,
/// local -> thiscall, ret 8) que não estiver depois de hoje ([0xae2c90]: ano sem sinal, dia com sinal) passa a amanhã
/// (0x54c5b0: push 1; push saída; mov ecx, 0xae2c90 -> thiscall(data, saída, n), ret 8); e a partida vai ao calendário
/// (0x5aa680: push 1; push &partida; ecx = [0xae2a58] -> thiscall, ret 8).
unsafe fn add_fixture_to_calendar(p: usize) {
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let juntar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
    let (mut t, mut saida) = ([0u8; 8], [0u8; 8]);
    let f = nova(t.as_mut_ptr() as usize, cqc5_d(p + 0x2a) as u32, cqc5_d(p + 0x28) as u32);
    let (hd, ha) = (cqc5_w(0xae2c90), cqc5_w(0xae2c92) as u16);
    let fa = cqc5_w(f + 2) as u16;
    let antes = ha < fa || (ha == fa && hd < cqc5_w(f));
    if !antes {
        let s = saida.as_mut_ptr() as usize;
        mais(0xae2c90, s, 1);
        cqc5_pw(p + 0x2a, cqc5_w(s) as u16);
        cqc5_pw(p + 0x28, cqc5_w(s + 2) as u16);
    }
    juntar(cqc5_d(0xae2a58), p, 1);
}

/// { 0x51cb90 } (thiscall, ret 0xc, Cup.cpp, quadro SEH sem destrutor no caminho normal: as datas e a notícia locais):
/// `Cup::create_tie_fixtures(rodada, confronto, adiar)`, da montagem da rodada (0x51c800) e da tela da competição.
/// Confronto −1: o primeiro da rodada com a marca +0x34 = 1 (adiar) ou 2 (criar); nenhum: 0. Adiar: marca 2. Criar:
/// marca 3; com time faltando (+0/+2 = −1), a notícia do confronto pendente (0x522950: push confronto; push rodada;
/// mov ecx, esi -> thiscall, ret 8). Senão a partida numa local de 0x4f bytes: a competição (+0x14) e o id (+0, −1
/// sem), −1 em +8, o ano da copa (+0x40) em +0x2c, +0x3a em +0x32, da rodada (0x68 bytes em +0xa3) o código +7 (+3
/// na de duas mãos, +0x21 = 2) em +0x30, +9 em +0x34, +0xb em +0x36, as regras +0xd em +0x38 (sem mão de volta e com
/// 0x800: 0x1000 se o nível (+4 do time) do mandante não é menor que o do visitante, senão 0x800), +0x17 em +0x3a, a
/// liga (+0x44) em +0x42, as marcas (0xff, 1, 0), o virtual +0x7c da copa (push &partida; mov ecx, esi), o bit 0x400
/// de +0x4d (sem mão de volta e +0x20 > 0), os clubes (+0x1c, +0x20) e ids (+0xc, +0x10; −1 sem clube), a data (dia
/// +3, ano +5 + o da copa), a fatia +6, a data da rodada seguinte (+0x6b/+0x6d; na última, 0), o estádio (0x85c260:
/// push +0x24 da copa; push &partida; push +0x13; mov ecx, [0xdd7ef4] -> thiscall, ret 0xc) e o calendário
/// (add_fixture_to_calendar). Com técnico humano num dos dois e sem o bit 0x100 em +0xd, a notícia 0x14 aos dois. Na de duas
/// mãos, com a copa ainda em [0xadadfc]: a volta +0x22 dias depois (0x549ef0, 0x54c5b0, 0x417c10), o código +4, a
/// fatia pelo dia da semana (0x54a3f0: sábado ou domingo 1, senão 2), mando invertido, o estádio e o calendário.
/// Devolve 1.
unsafe extern "thiscall" fn create_tie_fixtures(this: usize, rodada: u32, confronto: u32, adiar: u32) -> u32 {
    let id = cqc5_d(cqc5_d(this + 4));
    let ri = rodada as i16 as i32;
    let (mut k, mut adiar) = (confronto, adiar);
    if confronto as i16 == -1 {
        let n = cqc5_w(round_entry(this, ri) + 0x1a);
        let mut i: u32 = 0;
        loop {
            if (i as i16) >= n {
                return 0;
            }
            let m = cqc5_b(tie_of_round(this, ri, i as i16 as i32) + 0x34);
            if m == 1 {
                adiar = 1;
                break;
            }
            if m == 2 {
                adiar = 0;
                break;
            }
            i = i.wrapping_add(1);
        }
        k = i;
        if k as i16 == -1 {
            return 0;
        }
    }
    let t = tie_of_round(this, ri, k as i16 as i32);
    if adiar != 0 {
        *((t + 0x34) as *mut u8) = 2;
        return 1;
    }
    *((t + 0x34) as *mut u8) = 3;
    if cqc5_w(t) == -1 || cqc5_w(t + 2) == -1 {
        let pendente: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x522950usize);
        pendente(this, rodada, k);
        return 1;
    }
    let mut buf = [0u8; 0x50];
    let p = buf.as_mut_ptr() as usize;
    let comp = cqc5_d(this + 4);
    if comp != 0 {
        cqc5_pd(p + 0x14, comp);
        cqc5_pd(p, cqc5_d(comp));
    } else {
        cqc5_pd(p, usize::MAX);
        cqc5_pd(p + 0x14, 0);
    }
    cqc5_pw(p + 0x2c, cqc5_w(this + 0x40) as u16);
    cqc5_pw(p + 0x32, cqc5_w(this + 0x3a) as u16);
    let e = round_entry(this, ri);
    let mut codigo = cqc5_w(e + 7);
    if cqc5_b(e + 0x21) == 2 {
        codigo = codigo.wrapping_add(3);
    }
    cqc5_pw(p + 0x30, codigo as u16);
    cqc5_pw(p + 0x34, cqc5_w(e + 9) as u16);
    cqc5_pb(p + 0x3a, cqc5_b(e + 0x17));
    cqc5_pw(p + 0x36, cqc5_w(e + 0xb) as u16);
    // ponytail: a original faz and 0xf4ff na palavra +0x4d de um local sem valor (o +0x4d e os bits 2, 4..7 de
    // +0x4e ficam com a sobra de pilha) e não grava +4, +0x18, +0x2e, +0x2f, +0x3c antes do estádio: aqui, zero
    cqc5_pw(p + 0x38, cqc5_w(e + 0xd) as u16);
    for x in [0x43usize, 0x44, 0x4b, 0x4c, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a] {
        cqc5_pb(p + x, 0xff);
    }
    cqc5_pd(p + 8, usize::MAX);
    cqc5_pb(p + 0x42, cqc5_b(this + 0x44));
    cqc5_pb(p + 0x3d, 0);
    cqc5_pb(p + 0x3e, 0);
    cqc5_pb(p + 0x3b, 0xff);
    cqc5_pb(p + 0x40, 1);
    cqc5_pb(p + 0x41, 1);
    if (cqc5_b(e + 0x21) as i8) < 2 && cqc5_w(p + 0x38) as u16 & 0x800 != 0 {
        let nc = cqc5_b(cup_team_entry(this, cqc5_w(t)) + 4) as i8;
        let nf = cqc5_b(cup_team_entry(this, cqc5_w(t + 2)) + 4) as i8;
        cqc5_pw(p + 0x38, if nc >= nf { 0x1000 } else { 0x800 });
    }
    let virtual_7c: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(cqc5_d(cqc5_d(this) + 0x7c));
    virtual_7c(this, p);
    let e = round_entry(this, ri);
    if (cqc5_b(e + 0x21) as i8) < 2 && (cqc5_b(e + 0x20) as i8) > 0 {
        cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) | 4);
    } else {
        cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) & !4);
    }
    let t = tie_of_round(this, ri, k as i16 as i32);
    let (lc, lf) = (cqc5_w(t), cqc5_w(t + 2)); // cqc5: os lados da ida
    let casa = cqc5_d(cup_team_entry(this, lc));
    cqc5_pd(p + 0x1c, casa);
    cqc5_pd(p + 0xc, if casa != 0 { cqc5_d(casa) } else { usize::MAX });
    let fora = cqc5_d(cup_team_entry(this, lf));
    cqc5_pd(p + 0x20, fora);
    cqc5_pd(p + 0x10, if fora != 0 { cqc5_d(fora) } else { usize::MAX });
    let ano_copa = cqc5_w(this + 0x40);
    cqc5_pw(p + 0x28, (cqc5_b(e + 5) as i8 as i16).wrapping_add(ano_copa) as u16);
    cqc5_pw(p + 0x2a, cqc5_w(e + 3) as u16); // cqc5: o dia da ida
    cqc5_pb(p + 0x3f, cqc5_b(e + 6));
    if ri < cqc5_w(this + 0x3c) as i32 - 1 {
        cqc5_pw(p + 0x26, cqc5_w(e + 0x6b) as u16);
        cqc5_pw(p + 0x24, (cqc5_b(e + 0x6d) as i8 as i16).wrapping_add(cqc5_w(this + 0x40)) as u16);
    } else {
        cqc5_pw(p + 0x26, 0);
        cqc5_pw(p + 0x24, 0);
    }
    let estadio: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x85c260usize);
    estadio(cqc5_d(0xdd7ef4), cqc5_d(e + 0x13), p, cqc5_d(this + 0x24));
    crate::comparison::branch("cqc_5 jogo do confronto");
    add_fixture_to_calendar(p);
    let (casa, fora) = (cqc5_d(p + 0x1c), cqc5_d(p + 0x20));
    if (cup_has_human_manager(casa) || cup_has_human_manager(fora)) && cqc5_w(round_entry(this, ri) + 0xd) as u16 & 0x100 == 0 {
        let mut noticia = [0u32; 0x38];
        let n = noticia.as_mut_ptr() as usize;
        let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
        let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
        construir(n, 0x14, 0);
        gravar(n, 0, cqc5_d(cqc5_d(this + 4)) as u32);
        gravar(n, 1, cqc5_w(this + 0x3a) as i32 as u32);
        gravar(n, 2, cqc5_w(round_entry(this, ri) + 7) as i32 as u32);
        gravar(n, 5, (cqc5_b(round_entry(this, ri) + 0x21) == 2) as u32);
        gravar(n, 6, cqc5_d(p + 4) as u32);
        gravar(n, 7, cqc5_w(p + 0x2a) as u16 as u32);
        gravar(n, 8, cqc5_w(p + 0x28) as u16 as u32);
        if cup_has_human_manager(casa) {
            gravar(n, 3, cqc5_d(p + 0xc) as u32);
            gravar(n, 4, cqc5_d(p + 0x10) as u32);
        } else {
            gravar(n, 3, cqc5_d(p + 0x10) as u32);
            gravar(n, 4, cqc5_d(p + 0xc) as u32);
        }
        entregar(cqc5_d(0xb7431c), n, casa);
        entregar(cqc5_d(0xb7431c), n, fora);
    }
    let e = round_entry(this, ri);
    if cqc5_d(cqc5_d(0xadadfc) + id * 4) == 0 || cqc5_b(e + 0x21) != 2 {
        return 1;
    }
    crate::comparison::branch("cqc_5 jogo de volta");
    if (cqc5_b(e + 0x20) as i8) < 1 {
        cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) & !4);
    } else {
        cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) | 4);
    }
    cqc5_pw(p + 0x30, cqc5_w(e + 7).wrapping_add(4) as u16);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54a3f0usize);
    let (mut a, mut s, mut c, mut x) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let pa = a.as_mut_ptr() as usize;
    // ponytail: a original empurra o dia e o ano em registradores com sobra nos 16 bits altos; a 0x549ef0 só lê as
    // palavras
    nova(pa, cqc5_w(e + 3) as u16 as u32, (cqc5_b(e + 5) as i8 as i16).wrapping_add(cqc5_w(this + 0x40)) as u16 as u32);
    let r = mais(pa, s.as_mut_ptr() as usize, cqc5_b(e + 0x22) as u32);
    copia(pa, c.as_mut_ptr() as usize, r);
    cqc5_pw(p + 0x28, cqc5_w(pa + 2) as u16);
    cqc5_pw(p + 0x2a, cqc5_w(pa) as u16);
    let dia = nova(x.as_mut_ptr() as usize, cqc5_d(pa) as u32, cqc5_d(pa + 2) as u32);
    let sd = semana(dia) as i16;
    cqc5_pb(p + 0x3f, if sd == 5 || sd == 6 { 1 } else { 2 });
    let (c1, c2, i1, i2) = (cqc5_d(p + 0x1c), cqc5_d(p + 0x20), cqc5_d(p + 0xc), cqc5_d(p + 0x10)); // cqc5: os lados da volta
    cqc5_pb(p + 0x4e, cqc5_b(p + 0x4e) & 0xf4);
    cqc5_pd(p + 0x20, c1);
    cqc5_pd(p + 0xc, i2);
    cqc5_pd(p + 0x10, i1);
    cqc5_pd(p + 0x1c, c2);
    let e = round_entry(this, ri);
    estadio(cqc5_d(0xdd7ef4), cqc5_d(e + 0x13), p, cqc5_d(this + 0x24));
    add_fixture_to_calendar(p);
    1
}

/// (ano, dia) + n dias (o dia vai de 0 a 364, ou 365 no bissexto da 0x549ef0).
fn add_days_to_day_year((ano, dia): (i32, i32), n: i32) -> (i32, i32) {
    let (mut a, mut d) = (ano, dia + n);
    loop {
        let t = if crate::date::bissexto(a as u16) == 1 { 366 } else { 365 };
        if d < t {
            return (a, d);
        }
        d -= t;
        a += 1;
    }
}

/// Os jogos do clube (id) na competição de hoje até (ano, dia) (0x5acbe0: lista nula, tipo 0xfe e lado 0xff
/// quaisquer), para o comparador. A janela não passa de onde a 0x5aa680 pode pôr os jogos novos + 14 dias: a busca cria
/// o bloco do ano que faltar (0x5abf10).
unsafe fn club_fixtures_until(clube: u32, comp: u32, (ano, dia): (i32, i32)) -> u32 {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(0x5acbe0usize);
    achar(cqc5_d(0xae2a58), cqc5_w(0xae2c90) as u16 as u32, cqc5_w(0xae2c92) as u16 as u32, dia as u32, ano as u32, clube, 0,
        comp, 0xfe, 0xff) & 0xffff
}

/// Comparador da 0x51cb90: só a original (a partida vai ao calendário pela 0x5aa680, que aloca, e as notícias aos
/// técnicos). O plano, feito antes: o confronto escolhido (o −1 procura a marca 1 ou 2), a devolução, a marca (2 ao
/// adiar, 3 ao criar) e, criando com os dois times, os jogos do mandante na competição sobem 1 (2 na de duas mãos com a
/// copa ainda ativa).
unsafe extern "thiscall" fn comparar_create_tie_fixtures(this: usize, rodada: u32, confronto: u32, adiar: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(%%I0%%);
    let ri = rodada as i16 as i32;
    let id = cqc5_d(cqc5_d(this + 4)) as u32;
    let (mut k, mut ad) = (confronto as i16 as i32, adiar);
    if k == -1 {
        let n = cqc5_w(round_entry(this, ri) + 0x1a) as i32;
        k = -1;
        for i in 0..n.max(0) {
            let m = cqc5_b(tie_of_round(this, ri, i) + 0x34);
            if m == 1 || m == 2 {
                k = i;
                ad = (m == 1) as u32;
                break;
            }
        }
        if k == -1 {
            let r = o(this, rodada, confronto, adiar);
            SCOREBOARD.record(r == 0, || format!("create_tie_fixtures(rodada {}) sem confronto marcado: {}", ri, r));
            return r;
        }
    }
    let t = tie_of_round(this, ri, k);
    let completo = cqc5_w(t) != -1 && cqc5_w(t + 2) != -1;
    let e = round_entry(this, ri);
    let duas = cqc5_b(e + 0x21) == 2;
    let casa = if completo { cqc5_d(cup_team_entry(this, cqc5_w(t))) } else { 0 };
    let ic = if casa != 0 { cqc5_d(casa) as u32 } else { u32::MAX };
    let conferir = ad == 0 && completo && ic != u32::MAX;
    // o fim da janela: a data da rodada (+ os dias até a volta), amanhã ou a data da rodada seguinte (+0x6b/+0x6d), o
    // que vier depois, + 14 dias. Com conflito, a 0x5aa680 remarca o jogo (0x5ab510): copa (tipo 2) até a data da
    // rodada seguinte (sem ela, 2 semanas), os outros tipos até 2 semanas depois; com mais ligas acontece (a 1a rodada
    // da copa 101, 2001-225, foi a mais de 7 dias)
    let ida = (cqc5_b(e + 5) as i8 as i32 + cqc5_w(this + 0x40) as i32, cqc5_w(e + 3) as i32);
    let ultimo = if duas { add_days_to_day_year(ida, cqc5_b(e + 0x22) as i32) } else { ida };
    let amanha = add_days_to_day_year((cqc5_w(0xae2c92) as u16 as i32, cqc5_w(0xae2c90) as i32), 1);
    let seguinte = if ri < cqc5_w(this + 0x3c) as i32 - 1 {
        (cqc5_b(e + 0x6d) as i8 as i32 + cqc5_w(this + 0x40) as i32, cqc5_w(e + 0x6b) as i32)
    } else {
        ultimo
    };
    let fim = add_days_to_day_year(ultimo.max(amanha).max(seguinte), 14);
    let antes = if conferir { club_fixtures_until(ic, id, fim) } else { 0 };
    let r = o(this, rodada, confronto, adiar);
    let marca = cqc5_b(t + 0x34);
    let mut ok = r == 1 && marca == if ad != 0 { 2 } else { 3 };
    let mut esperado = 0;
    let mut depois = 0;
    if conferir && cqc5_d(cqc5_d(0xadadfc) + id as usize * 4) != 0 {
        esperado = 1 + duas as u32;
        depois = club_fixtures_until(ic, id, fim);
        ok = ok && depois == antes + esperado;
    }
    SCOREBOARD.record(ok, || format!("create_tie_fixtures(rodada {}, confronto {}, adiar {}) devolveu {} marca {} jogos {} -> {} (esperado +{})",
        ri, k, ad, r, marca, antes, depois, esperado));
    r
}

/// { 0x522950 } (thiscall, ret 8, Cup.cpp, quadro SEH sem destrutor no caminho normal: a notícia local):
/// `Cup::post_draw_news(rodada, confronto)`, só da 0x51cb90. Com os dois times definidos, ou rodada (0x68 bytes em
/// +0xa3) com menos de 2 confrontos (+0x1a): nada. Cada lado sem time (+0/+2 = −1) vem do confronto de origem (rodada
/// +0xe/+0x10, confronto +0xa/+0xc; sem origem ou com time faltando lá: nada): os dois clubes possíveis; com time, o
/// clube dele (+0xa7, 6 bytes). A notícia 0x26: 0 = o id da copa, 1 = +0x3a, 2 = o código da rodada (+7), 3/4 = o
/// mandante e −1 ou os dois possíveis, 5/6 = o visitante e −1 ou os dois possíveis; aos técnicos de cada clube dela
/// (0x7861e0: push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8; os possíveis só se não nulos).
unsafe extern "thiscall" fn post_draw_news(this: usize, rodada: u32, confronto: u32) {
    let ri = rodada as i16 as i32;
    let t = tie_of_round(this, ri, confronto as i16 as i32);
    let (mut a1, mut a2, mut b1, mut b2, mut casa) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let h = cqc5_w(t);
    if h != -1 && cqc5_w(t + 2) != -1 {
        return;
    }
    if cqc5_w(round_entry(this, ri) + 0x1a) < 2 {
        return;
    }
    if h == -1 {
        let r2 = cqc5_w(t + 0xe);
        if r2 == -1 {
            return;
        }
        let o = tie_of_round(this, r2 as i32, cqc5_w(t + 0xa) as i32);
        let (x, y) = (cqc5_w(o), cqc5_w(o + 2));
        if x == -1 || y == -1 {
            return;
        }
        a1 = cqc5_d(cup_team_entry(this, x));
        a2 = cqc5_d(cup_team_entry(this, y));
    } else {
        casa = cqc5_d(cup_team_entry(this, h));
    }
    let fora;
    let v = cqc5_w(t + 2);
    if v == -1 {
        let r2 = cqc5_w(t + 0x10);
        if r2 == -1 {
            return;
        }
        let o = tie_of_round(this, r2 as i32, cqc5_w(t + 0xc) as i32);
        let (x, y) = (cqc5_w(o), cqc5_w(o + 2));
        if x == -1 || y == -1 {
            return;
        }
        b1 = cqc5_d(cup_team_entry(this, x));
        b2 = cqc5_d(cup_team_entry(this, y));
        fora = 0;
    } else {
        fora = cqc5_d(cup_team_entry(this, v));
    }
    crate::comparison::branch("cqc_5 confronto pendente");
    let mut noticia = [0u32; 0x38];
    let n = noticia.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    construir(n, 0x26, 0);
    gravar(n, 0, cqc5_d(cqc5_d(this + 4)) as u32);
    gravar(n, 1, cqc5_w(this + 0x3a) as i32 as u32);
    gravar(n, 2, cqc5_w(round_entry(this, ri) + 7) as i32 as u32);
    if casa != 0 {
        gravar(n, 3, cqc5_d(casa) as u32);
        gravar(n, 4, u32::MAX);
    } else {
        gravar(n, 3, cqc5_d(a1) as u32);
        gravar(n, 4, cqc5_d(a2) as u32);
    }
    if fora != 0 {
        gravar(n, 5, cqc5_d(fora) as u32);
        gravar(n, 6, u32::MAX);
    } else {
        gravar(n, 5, cqc5_d(b1) as u32);
        gravar(n, 6, cqc5_d(b2) as u32);
    }
    let news = cqc5_d(0xb7431c);
    if casa != 0 {
        entregar(news, n, casa);
    } else {
        if a1 != 0 {
            entregar(news, n, a1);
        }
        if a2 != 0 {
            entregar(news, n, a2);
        }
    }
    if fora != 0 {
        entregar(news, n, fora);
    } else {
        if b1 != 0 {
            entregar(news, n, b1);
        }
        if b2 != 0 {
            entregar(news, n, b2);
        }
    }
}

/// Comparador da 0x522950: só a original (a notícia vai aos técnicos e não tem outro efeito).
unsafe extern "thiscall" fn comparar_post_draw_news(this: usize, rodada: u32, confronto: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I1%%)(this, rodada, confronto) // ponytail: só a notícia
}

/// { 0x8c6bc0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `TRANSFER_MANAGER::update_transfer_window_news(hoje)`, do dia das transferências. Os meses anterior, atual e
/// seguinte (0x54cfb0: mov ecx, hoje -> thiscall, ret 0; o anterior de janeiro é dezembro do ano −1, o seguinte de
/// dezembro é janeiro do ano +1), hoje + 1 semana (0x54c770: push 1; push saída; mov ecx, hoje -> thiscall, ret 8) e
/// ontem (0x54c8e0: idem). Para cada janela do vetor [this+0x8a4] (conta +0x8a8, relida): no primeiro dos três meses
/// igual ao dela (+4), a data dela naquele ano (0x40a7d0: push ano; push saída; mov ecx, janela -> thiscall, ret 8;
/// copiada pela 0x417c10). Hoje: o virtual +0x2c das regras da nação dela ([this+0x8ac + 4·(+0)]: push o dword do ano
/// da data; push o código 1, ou 3 se a janela fecha (+5), com os bytes altos do mesmo dword; push janela -> thiscall,
/// ret 0xc); ontem e janela que abre: o virtual com 0 e as ofertas em espera da nação das regras (+0xa; 0x8c4e10: push
/// nação; mov ecx, esi -> thiscall, ret 4); daqui a uma semana e janela que abre: o virtual com 2.
unsafe extern "thiscall" fn update_transfer_window_news(this: usize, hoje: usize) {
    let mes: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mais_semanas: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c770usize);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let data_da_janela: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x40a7d0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let espera: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c4e10usize);
    let mut anos = [0i16; 3];
    let mut d = [0u8; 8];
    let pd = d.as_mut_ptr() as usize;
    padrao(pd);
    let mut meses = [0u8; 3];
    meses[0] = mes(hoje).wrapping_sub(1);
    if (meses[0] as i8) < 0 {
        meses[0] = 0xb;
        anos[0] = -1;
    }
    meses[1] = mes(hoje);
    meses[2] = mes(hoje).wrapping_add(1);
    if (meses[2] as i8) > 0xb {
        meses[2] = 0;
        anos[2] = 1;
    }
    let (mut semana, mut ontem) = ([0u8; 8], [0u8; 8]);
    let (ps, po) = (semana.as_mut_ptr() as usize, ontem.as_mut_ptr() as usize);
    mais_semanas(hoje, ps, 1);
    menos(hoje, po, 1);
    let mut v = cqc5_d(this + 0x8a4);
    let mut i: i32 = 0;
    while i < cqc5_d(this + 0x8a8) as i32 {
        for j in 0..3 {
            let w = cqc5_d(v);
            if cqc5_b(w + 4) != meses[j] {
                continue;
            }
            let ano = (anos[j] as u16).wrapping_add(cqc5_w(hoje + 2) as u16);
            let (mut t, mut c) = ([0u8; 8], [0u8; 8]);
            let r = data_da_janela(w, t.as_mut_ptr() as usize, ano as u32);
            copia(pd, c.as_mut_ptr() as usize, r);
            let (hd, ha) = (cqc5_w(hoje), cqc5_w(hoje + 2));
            let (dd, da, dw) = (cqc5_w(pd), cqc5_w(pd + 2), cqc5_d(pd + 2) as u32);
            let regras = |w: usize| cqc5_d(this + 0x8ac + (cqc5_b(w) as i8 as isize * 4) as usize);
            let aviso = |w: usize, codigo: u32| {
                let rg = regras(w);
                let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(cqc5_d(cqc5_d(rg) + 0x2c));
                f(rg, w, codigo, dw);
            };
            if ha == da && hd == dd {
                let codigo = if cqc5_b(w + 5) != 0 { 3 } else { 1 };
                aviso(w, (dw & 0xffff_ff00) | codigo); // cqc5: o aviso do dia da janela
                break;
            }
            if cqc5_w(po + 2) == da && cqc5_w(po) == dd && cqc5_b(w + 5) == 0 {
                aviso(w, 0);
                let nacao = cqc5_d(regras(cqc5_d(v)) + 0xa);
                if nacao != 0 {
                    crate::comparison::branch("cqc_5 janela aberta");
                    espera(this, nacao); // cqc5: as ofertas em espera da janela que abriu
                }
                break;
            }
            if cqc5_w(ps + 2) == da && cqc5_w(ps) == dd && cqc5_b(w + 5) == 0 {
                aviso(w, 2);
                break;
            }
        }
        i += 1;
        v += 4;
    }
}

/// Comparador da 0x8c6bc0: só a original (ela só grava nas locais; os avisos das regras alocam notícias e as ofertas
/// em espera mudam de status e são reagendadas).
unsafe extern "thiscall" fn comparar_update_transfer_window_news(this: usize, hoje: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, hoje) // ponytail: nada próprio a conferir
}
