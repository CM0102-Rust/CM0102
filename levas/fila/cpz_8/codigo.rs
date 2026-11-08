// ---------- %%LEVA%%a leva: os jogos remarcados: os jogos vizinhos do clube movidos um dia, os jogos perto da data da seleção e a notícia do jogo remarcado (fix_man.cpp) ----------

unsafe fn cpz8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz8_u(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpz8_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpz8_sw(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i16) as i32
}

unsafe fn cpz8_sb(a: usize) -> i32 {
    *(a as *const i8) as i32
}

/// LLIST (0xe bytes: +0 conta i16, +2 cabeça, +6 cauda, +0xa cursor; nó: +0 dados, +6 próximo): o construtor
/// (0x68d020), o destrutor (0x68d040), o esvaziar (0x68d080), o cursor na cabeça e o próximo (0x68d1b0).
unsafe fn list_init(l: usize) {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    f(l);
}

unsafe fn list_destroy(l: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    f(l);
}

unsafe fn list_clear(l: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    f(l);
}

unsafe fn list_rewind(l: usize) {
    core::ptr::write_unaligned((l + 0xa) as *mut u32, cpz8_u(l + 2));
}

unsafe fn cpz8_proximo(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    f(l)
}

/// A competição do jogo é amistosa? (0x5c6710: thiscall em [0xae2c74], 1 argumento.)
unsafe fn fix_man_is_friendly_comp(comp: u32) -> bool {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5c6710usize);
    f(cpz8_d(0xae2c74), comp) != 0
}

/// As datas (Date, 8 bytes): construir por (dia, ano) (0x549ef0), menos e mais n dias (0x54c8e0, 0x54c5b0: thiscall
/// (data, saída, n), devolvem a saída) e a atribuição (0x417c10: thiscall(destino, temporária, origem)).
unsafe fn date_from_day_year_ptr(d: *mut u8, dia: u32, ano: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    f(d as usize, dia, ano)
}

unsafe fn date_sub_days(d: usize, saida: *mut u8, n: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    f(d, saida as usize, n)
}

unsafe fn fix_man_date_add_days(d: usize, saida: *mut u8, n: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    f(d, saida as usize, n)
}

unsafe fn date_assign_ptr(destino: *mut u8, origem: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut temporaria = [0u8; 8];
    f(destino as usize, temporaria.as_mut_ptr() as usize, origem);
}

/// Os jogos do clube entre duas datas (0x5acbe0, thiscall, 9 argumentos): a original empurra os dwords em +0 e +2 de
/// cada data (só a palavra baixa conta), o clube, a lista, comp −1, tipo −2 e lado 1.
unsafe fn find_club_fixtures_between(this: usize, a: *const u8, b: *const u8, clube: u32, lista: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x5acbe0usize);
    let (a, b) = (a as usize, b as usize);
    f(this, cpz8_u(a), cpz8_u(a + 2), cpz8_u(b), cpz8_u(b + 2), clube, lista, u32::MAX, 0xffff_fffe, 1)
}

/// Tirar o jogo do calendário (0x5abc30), gravar a data (0x5ab510), juntar (0x5aa680) e a notícia (0x5aac70).
unsafe fn fix_man_remove_fixture(this: usize, jogo: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5abc30usize);
    f(this, jogo)
}

unsafe fn fix_man_add_fixture(this: usize, jogo: usize, zerar: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
    f(this, jogo, zerar);
}

unsafe fn fix_man_queue_fixture_moved_news(this: usize, jogo: usize, causa: usize, velha: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x5aac70usize);
    f(this, jogo, causa, velha);
}

/// A data nova do jogo `f2` tem outro jogo? (0x5ad780: thiscall(jogo, &gravidade, 0) devolve uma LLIST alocada;
/// percorre e acha um item com bytes diferentes do jogo original `f1`; depois destrói e libera, 0x944c94.)
unsafe fn new_date_has_clash(this: usize, f2: *mut u8, f1: *const u8) -> bool {
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize = core::mem::transmute(0x5ad780usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let mut gravidade = 0i32;
    let l = achar(this, f2 as usize, &mut gravidade, 0);
    if l == 0 {
        return false;
    }
    list_rewind(l);
    let mut outro = false;
    loop {
        let q = cpz8_proximo(l);
        if q == 0 {
            break;
        }
        if core::slice::from_raw_parts(f1, 0x4f) != core::slice::from_raw_parts(q as *const u8, 0x4f) {
            outro = true;
            break;
        }
    }
    list_destroy(l);
    liberar(l);
    outro
}

/// O primeiro jogo da lista copiado para `f1` e `f2` (0x4f bytes).
unsafe fn copy_first_fixture(l: usize, f1: &mut [u8; 0x4f], f2: &mut [u8; 0x4f]) {
    list_rewind(l);
    let p = cpz8_proximo(l);
    core::ptr::copy_nonoverlapping(p as *const u8, f1.as_mut_ptr(), 0x4f);
    *f2 = *f1;
}

/// { 0x5ae9d0 } (thiscall, ret 4, fix_man.cpp, SEH da lista local e das datas): `FIXTURE_MANAGER::
/// move_nearby_club_fixtures(jogo)`, pela junção do jogo (0x5aa680). Jogo de competição amistosa (+0x14, 0x5c6710):
/// nada. Para os dois clubes (+0x1c, +0x20) de nação (+0x53) com a marca +0x11c: o primeiro jogo do clube dois dias
/// antes (0x5acbe0, lado 1), não amistoso e com +0x43 = 0xff, vai para um dia antes dele; o primeiro jogo dois dias
/// depois, não amistoso, vai para um dia depois. Só se a data nova não tiver outro jogo (0x5ad780): tira (0x5abc30),
/// junta (0x5aa680, 0) e dá a notícia do jogo remarcado (0x5aac70, com o jogo como causa).
unsafe extern "thiscall" fn move_nearby_club_fixtures(this: usize, jogo: usize) {
    let iniciar_data: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mut d = [0u8; 8];
    iniciar_data(d.as_mut_ptr() as usize);
    let mut l = [0u8; 0xe];
    let lp = l.as_mut_ptr() as usize;
    list_init(lp);
    if fix_man_is_friendly_comp(cpz8_u(jogo + 0x14)) {
        list_destroy(lp);
        return;
    }
    let mut f1 = [0u8; 0x4f];
    let mut f2 = [0u8; 0x4f];
    for k in 0..2 {
        let clube = cpz8_d(jogo + if k == 0 { 0x1c } else { 0x20 });
        let nacao = cpz8_d(clube + 0x53);
        if nacao == 0 || *((nacao + 0x11c) as *const u8) == 0 {
            continue;
        }
        // dois dias antes
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let r = date_from_day_year_ptr(t.as_mut_ptr(), cpz8_w(jogo + 0x2a) as u32, cpz8_w(jogo + 0x28) as u32);
        let r = date_sub_days(r, u.as_mut_ptr(), 2);
        date_assign_ptr(d.as_mut_ptr(), r);
        list_clear(lp);
        if find_club_fixtures_between(this, d.as_ptr(), d.as_ptr(), cpz8_u(clube), lp) as u16 != 0 {
            copy_first_fixture(lp, &mut f1, &mut f2);
            if !fix_man_is_friendly_comp(cpz8_u(f1.as_ptr() as usize + 0x14)) {
                crate::comparison::branch("cpz_8 jogo dois dias antes");
                if f2[0x43] == 0xff { // cpz8: o jogo de dois dias antes sem a marca
                    let mut v = [0u8; 8];
                    let r = date_sub_days(d.as_ptr() as usize, v.as_mut_ptr(), 1); // cpz8: um dia antes
                    date_assign_ptr(d.as_mut_ptr(), r);
                    f2[0x2a..0x2c].copy_from_slice(&d[0..2]);
                    f2[0x28..0x2a].copy_from_slice(&d[2..4]);
                    if !new_date_has_clash(this, f2.as_mut_ptr(), f1.as_ptr()) {
                        crate::comparison::branch("cpz_8 jogo antecipado");
                        fix_man_remove_fixture(this, f1.as_ptr() as usize);
                        fix_man_add_fixture(this, f2.as_mut_ptr() as usize, 0);
                        fix_man_queue_fixture_moved_news(this, f2.as_mut_ptr() as usize, jogo, 0);
                    }
                }
            }
        }
        // dois dias depois
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let r = date_from_day_year_ptr(t.as_mut_ptr(), cpz8_w(jogo + 0x2a) as u32, cpz8_w(jogo + 0x28) as u32);
        let r = fix_man_date_add_days(r, u.as_mut_ptr(), 2);
        date_assign_ptr(d.as_mut_ptr(), r);
        list_clear(lp);
        if find_club_fixtures_between(this, d.as_ptr(), d.as_ptr(), cpz8_u(clube), lp) as u16 == 0 {
            continue;
        }
        copy_first_fixture(lp, &mut f1, &mut f2);
        if fix_man_is_friendly_comp(cpz8_u(f1.as_ptr() as usize + 0x14)) {
            continue;
        }
        let mut v = [0u8; 8];
        let r = fix_man_date_add_days(d.as_ptr() as usize, v.as_mut_ptr(), 1); // cpz8: um dia depois
        date_assign_ptr(d.as_mut_ptr(), r);
        f2[0x2a..0x2c].copy_from_slice(&d[0..2]);
        f2[0x28..0x2a].copy_from_slice(&d[2..4]);
        if !new_date_has_clash(this, f2.as_mut_ptr(), f1.as_ptr()) {
            crate::comparison::branch("cpz_8 jogo adiado");
            fix_man_remove_fixture(this, f1.as_ptr() as usize);
            fix_man_add_fixture(this, f2.as_mut_ptr() as usize, 0);
            fix_man_queue_fixture_moved_news(this, f2.as_mut_ptr() as usize, jogo, 0);
        }
    }
    list_destroy(lp);
}

// ponytail: move jogos no calendário (tira, junta, aloca e libera listas, notícia): só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn compare_move_nearby_club_fixtures(this: usize, jogo: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, jogo)
}

/// { 0x5abb10 } (thiscall, ret 8, fix_man.cpp, SEH da lista local): `FIXTURE_MANAGER::
/// reschedule_club_fixtures_near_date(clube, data)`, pelas convocações das seleções (0x777940: clube com mais de dois
/// convocados). Os jogos do clube (o id em `*clube`) de um dia antes a um dia depois da data (0x5acbe0, lado 1); cada
/// um que sai do calendário (0x5abc30) ganha data nova (0x5ab510), volta (0x5aa680, 1) e dá a notícia com a data velha
/// (0x5aac70, sem causa).
unsafe extern "thiscall" fn reschedule_club_fixtures_near_date(this: usize, clube: usize, data: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5ab510usize);
    let mut l = [0u8; 0xe];
    let lp = l.as_mut_ptr() as usize;
    list_init(lp);
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    date_sub_days(data, a.as_mut_ptr(), 1);
    fix_man_date_add_days(data, b.as_mut_ptr(), 1);
    find_club_fixtures_between(this, a.as_ptr(), b.as_ptr(), cpz8_u(clube), lp);
    list_rewind(lp);
    loop {
        let p = cpz8_proximo(lp);
        if p == 0 {
            break;
        }
        let mut velha = [0u8; 8];
        date_from_day_year_ptr(velha.as_mut_ptr(), cpz8_w(p + 0x2a) as u32, cpz8_w(p + 0x28) as u32);
        if fix_man_remove_fixture(this, p) != 0 {
            crate::comparison::branch("cpz_8 jogo remarcado pela selecao");
            gravar(this, p);
            fix_man_add_fixture(this, p, 1);
            fix_man_queue_fixture_moved_news(this, p, 0, velha.as_ptr() as usize);
        }
    }
    list_destroy(lp);
}

// ponytail: move jogos no calendário e dá a notícia (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_reschedule_club_fixtures_near_date(this: usize, clube: usize, data: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%)(this, clube, data)
}

/// O ensaio da notícia: a cópia monta a notícia nova aqui em vez de pô-la na fila (e não cria a fila).
static mut FIXTURE_MOVED_NEWS_DRY_RUN: bool = false;
static mut FIXTURE_MOVED_NEWS_BUILT: [u8; 0xe0] = [0; 0xe0];
static mut FIXTURE_MOVED_NEWS_WAS_BUILT: bool = false;

unsafe fn news_slot(n: usize, k: u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x785830usize);
    f(n, k)
}

/// A notícia já na fila para o mesmo jogo (dados 0, 1, 2, 0xe e 0xf: +0xc, +0x10, +0, +0x40 e +0x41 do jogo).
unsafe fn find_queued_fixture_news(l: usize, f: usize) -> usize {
    list_rewind(l);
    loop {
        let it = cpz8_proximo(l);
        if it == 0 {
            return 0;
        }
        if news_slot(it, 0) == cpz8_u(f + 0xc)
            && news_slot(it, 1) == cpz8_u(f + 0x10)
            && news_slot(it, 2) == cpz8_u(f)
            && news_slot(it, 0xe) as i32 == cpz8_sb(f + 0x40)
            && news_slot(it, 0xf) as i32 == cpz8_sb(f + 0x41)
        {
            return it;
        }
    }
}

/// { 0x5aac70 } (thiscall, ret 0xc, fix_man.cpp, SEH da notícia local): `FIXTURE_MANAGER::queue_fixture_moved_news(jogo,
/// causa, data velha)`, pelas remarcações (0x5aa680, 0x5abb10, 0x5ae380, 0x5ae9d0). A fila em +0x1128 (criada na
/// primeira vez: 0x944cf1 e 0x68d020): se já há a notícia do mesmo jogo, só troca a data nova (dados 5 e 6); senão a
/// notícia 1: o jogo (+0xc, +0x10, +0, +0x32, +0x30, a data nova +0x2a/+0x28, +0x40, +0x41) e a causa (os mesmos
/// campos dela nos dados 7 a 0xd) ou, sem causa, −1 e a data velha (dados 0xc e 0xd); entra na fila (0x68d0d0, 0xdf).
unsafe extern "thiscall" fn queue_fixture_moved_news(this: usize, f: usize, causa: usize, velha: usize) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    if cpz8_d(this + 0x1128) == 0 && !FIXTURE_MOVED_NEWS_DRY_RUN {
        let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
        let m = novo(0xe);
        if m != 0 {
            list_init(m);
        }
        *((this + 0x1128) as *mut usize) = m;
    }
    let l = cpz8_d(this + 0x1128);
    if l != 0 {
        let it = find_queued_fixture_news(l, f);
        if it != 0 {
            gravar(it, 5, cpz8_w(f + 0x2a) as u32);
            gravar(it, 6, cpz8_w(f + 0x28) as u32);
            return;
        }
    }
    crate::comparison::branch("cpz_8 noticia do jogo remarcado");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 1, 0);
    let campos = |nt: usize, k: i8, j: usize| {
        gravar(nt, k, cpz8_u(j + 0xc));
        gravar(nt, k + 1, cpz8_u(j + 0x10));
        gravar(nt, k + 2, cpz8_u(j));
        gravar(nt, k + 3, cpz8_sw(j + 0x32) as u32);
        gravar(nt, k + 4, cpz8_sw(j + 0x30) as u32);
        gravar(nt, k + 5, cpz8_w(j + 0x2a) as u32);
    };
    campos(nt, 0, f);
    gravar(nt, 6, cpz8_w(f + 0x28) as u32);
    gravar(nt, 0xe, cpz8_sb(f + 0x40) as u32);
    gravar(nt, 0xf, cpz8_sb(f + 0x41) as u32);
    if causa != 0 {
        campos(nt, 7, causa);
        gravar(nt, 0xd, cpz8_w(causa + 0x28) as u32);
    } else {
        gravar(nt, 7, u32::MAX);
        gravar(nt, 0xc, cpz8_sw(velha) as u32);
        gravar(nt, 0xd, cpz8_w(velha + 2) as u32);
    }
    if FIXTURE_MOVED_NEWS_DRY_RUN {
        core::ptr::copy_nonoverlapping(nt as *const u8, core::ptr::addr_of_mut!(FIXTURE_MOVED_NEWS_BUILT) as *mut u8, 0xe0);
        FIXTURE_MOVED_NEWS_WAS_BUILT = true;
        return;
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    copiar(cpz8_d(this + 0x1128), nt, 0xdf);
}

/// O comparador da notícia, em ensaio. Já na fila: lado a lado no cabeçalho da fila e na notícia achada (a data nova).
/// Nova: a cópia monta a notícia sem pô-la na fila e a original roda; a fila tem de crescer um e o último item (a cauda,
/// +6, nó +0) ser a notícia da cópia nos 0xde bytes que o construtor (0x77bbe0) grava; o último dos 0xdf copiados é
/// sobra de pilha na original (a cópia leva zero).
unsafe extern "thiscall" fn compare_queue_fixture_moved_news(this: usize, f: usize, causa: usize, velha: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I2%%);
    let l = cpz8_d(this + 0x1128);
    let alvo = if l != 0 { find_queued_fixture_news(l, f) } else { 0 };
    if alvo != 0 {
        let regioes = [(this + 0x1128, 4), (l, 0xe), (alvo, 0xdf)];
        let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regioes, || o(this, f, causa, velha), || queue_fixture_moved_news(this, f, causa, velha));
        SCOREBOARD.record(igual, || format!("queue_fixture_moved_news(jogo {:#x}, na fila) original={:x?} rust={:x?}", f, mo, mr));
        return;
    }
    let mut regioes = vec![(this + 0x1128, 4)];
    if l != 0 {
        regioes.push((l, 0xe));
    }
    let antes = crate::comparison::MemorySnapshot::read(&regioes);
    let conta = if l != 0 { cpz8_sw(l) } else { 0 };
    FIXTURE_MOVED_NEWS_DRY_RUN = true;
    FIXTURE_MOVED_NEWS_WAS_BUILT = false;
    crate::rng::pause_trace(|| queue_fixture_moved_news(this, f, causa, velha));
    FIXTURE_MOVED_NEWS_DRY_RUN = false;
    antes.write_back();
    o(this, f, causa, velha);
    let l2 = cpz8_d(this + 0x1128);
    let montada = FIXTURE_MOVED_NEWS_WAS_BUILT;
    let igual = montada
        && l2 != 0
        && cpz8_sw(l2) == conta + 1
        && cpz8_d(cpz8_d(l2 + 6)) != 0
        && core::slice::from_raw_parts(cpz8_d(cpz8_d(l2 + 6)) as *const u8, 0xde)
            == core::slice::from_raw_parts(core::ptr::addr_of!(FIXTURE_MOVED_NEWS_BUILT) as *const u8, 0xde);
    SCOREBOARD.record(igual, || format!("queue_fixture_moved_news(jogo {:#x}, nova) montada={} conta {} -> {}", f, montada, conta, if l2 != 0 { cpz8_sw(l2) } else { -1 }));
}
