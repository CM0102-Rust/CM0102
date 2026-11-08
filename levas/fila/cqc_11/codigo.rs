// ---------- %%LEVA%%a leva: os amistosos entre seleções nas datas internacionais e os convites de amistoso e de torneio (friendly.cpp) ----------

unsafe fn cqc11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc11_uw(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// A lista de nações `q` do Friendly (vetor de listas em +0x34f6; a primeira parte indexa pela id da nação, a segunda
/// pela posição na lista ordenada +0x34fa, como na original).
unsafe fn nation_friendly_list(this: usize, q: usize) -> usize {
    cqc11_d(cqc11_d(this + 0x34f6).wrapping_add(q.wrapping_mul(4)))
}

/// Os itens (os 4 bytes de cada) da lista, percorrendo os nós sem mexer no cursor (+2 o primeiro; nó: +0 o item, +6 o
/// seguinte).
unsafe fn list_item_ids(lista: usize) -> Vec<u32> {
    let mut v = Vec::new();
    if lista == 0 {
        return v;
    }
    let mut no = cqc11_d(lista + 2);
    while no != 0 {
        v.push(cqc11_d(cqc11_d(no)) as u32);
        no = cqc11_d(no + 6);
    }
    v
}

/// O ensaio do comparador da 0x5cbdf0: as nações usadas (o vetor local da original), as listas antes e o que a cópia
/// acrescentaria a cada uma (índice da lista, item).
#[derive(Clone)]
struct FriendlyComparerTrial {
    usadas: Vec<u8>,
    antes: std::rc::Rc<Vec<Vec<u32>>>,
    novos: Vec<Vec<u32>>,
}

/// A fita do ensaio: os sorteios que a original fez (n, resposta), o próximo e se um n não bateu. Também estão nela os
/// sorteios de dentro das rotinas que a original chama no amistoso (0x53b3d0, 0x5c6f00, 0x5c5f40), que o ensaio pula.
static mut FRIENDLY_TAPE: (Vec<(i32, i32)>, usize, bool) = (Vec::new(), 0, false);

/// O sorteio do ensaio: o próximo da fita, com o mesmo n.
unsafe extern "C" fn tape_random(n: i32) -> i32 {
    let f = &mut *core::ptr::addr_of_mut!(FRIENDLY_TAPE);
    match f.0.get(f.1) {
        Some(&(m, r)) if m == n && !f.2 => {
            f.1 += 1;
            r
        }
        _ => {
            f.2 = true;
            0
        }
    }
}

unsafe fn tape_fully_consumed() -> bool {
    let f = &*core::ptr::addr_of!(FRIENDLY_TAPE);
    !f.2 && f.1 == f.0.len()
}

impl FriendlyComparerTrial {
    fn usada(&self, id: usize) -> bool {
        self.usadas.get(id).is_some_and(|&b| b != 0)
    }
    fn usar(&mut self, id: usize) {
        if id >= self.usadas.len() {
            self.usadas.resize(id + 1, 0);
        }
        self.usadas[id] = 1;
    }
    fn por(&mut self, q: usize, item: u32) {
        if q >= self.novos.len() {
            self.novos.resize(q + 1, Vec::new());
        }
        self.novos[q].push(item);
    }
    fn tem(&self, q: usize, item: u32) -> bool {
        self.antes.get(q).is_some_and(|v| v.contains(&item)) || self.novos.get(q).is_some_and(|v| v.contains(&item))
    }
}

/// Um jogo da primeira parte: com nação nos dois clubes (+0x1c, +0x20; +0x53, a id em +0), as duas ficam usadas e cada
/// uma vai à lista (pela id) da outra: a de casa recebe a de fora, depois o contrário (0x68d0d0 com 4: thiscall na
/// lista). No ensaio, só o registro.
unsafe fn link_fixture_nations(this: usize, p: usize, usadas: usize, ens: Option<&mut FriendlyComparerTrial>) {
    let na = cqc11_d(cqc11_d(p + 0x1c) + 0x53);
    if na == 0 {
        return;
    }
    let a = cqc11_d(na) as u32;
    let nb = cqc11_d(cqc11_d(p + 0x20) + 0x53);
    if nb == 0 {
        return;
    }
    let b = cqc11_d(nb) as u32;
    match ens {
        Some(e) => {
            e.usar(a as usize);
            e.usar(b as usize);
            e.por(a as usize, b);
            e.por(b as usize, a);
        }
        None => {
            let por: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
            *((usadas.wrapping_add(a as usize)) as *mut u8) = 1;
            crate::comparison::branch("cqc_11 nacoes com jogo");
            *((usadas.wrapping_add(b as usize)) as *mut u8) = 1; // cqc11: as nações com jogo perto da data
            let (va, vb) = (a, b);
            por(nation_friendly_list(this, a as usize), &vb as *const u32 as usize, 4);
            por(nation_friendly_list(this, b as usize), &va as *const u32 as usize, 4);
        }
    }
}

/// A primeira parte da 0x5cbdf0: de 4 dias antes a 4 dias depois da data (+3 do registro de datas; 0x54c8e0, 0x54c5b0
/// e 0x417c10), nas 3 fatias e nos 2 grupos de cada dia (0x5ae6a0), cada jogo com nação nos dois clubes (+0x1c, +0x20;
/// +0x53, a id em +0) marca as duas nações como usadas e cada uma vai à lista da outra (pela id: a de casa recebe a de
/// fora, depois o contrário; 0x68d0d0 com 4). No ensaio, nada se grava fora das datas estáticas (o cursor das listas
/// do calendário não se mexe).
unsafe fn link_nations_near_date(this: usize, datas: usize, usadas: usize, mut ens: Option<&mut FriendlyComparerTrial>) {
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let lista_de: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x5ae6a0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let (mut x, mut y, mut t, mut u) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let (px, py, pt, pu) = (x.as_mut_ptr() as usize, y.as_mut_ptr() as usize, t.as_mut_ptr() as usize, u.as_mut_ptr() as usize);
    let data = datas + 3;
    copiar(px, pu, menos(data, pt, 4)); // a primeira data: 4 dias antes
    copiar(py, pu, mais(data, pt, 4)); // a última: 4 dias depois
    while cqc11_uw(py + 2) > cqc11_uw(px + 2) || (cqc11_uw(py + 2) == cqc11_uw(px + 2) && cqc11_w(py) >= cqc11_w(px)) {
        for fatia in 0u32..3 {
            for grupo in 0u32..2 {
                // ponytail: a original empurra os dois bytes com sobra de pilha em cima; a 0x5abf10 só lê o byte
                let l = lista_de(px, fatia, grupo);
                if let Some(e) = ens.as_deref_mut() {
                    let mut no = cqc11_d(l + 2);
                    while no != 0 {
                        link_fixture_nations(this, cqc11_d(no), usadas, Some(&mut *e));
                        no = cqc11_d(no + 6);
                    }
                } else {
                    core::ptr::write_unaligned((l + 0xa) as *mut usize, cqc11_d(l + 2));
                    let mut p = proximo(l);
                    while p != 0 {
                        link_fixture_nations(this, p, usadas, None);
                        p = proximo(l);
                    }
                }
            }
        }
        copiar(px, pu, mais(px, pt, 1)); // o dia seguinte
    }
}

/// A nação `n` (ponteiro) está numa das confederações da data: alguma das `quantas` entradas de 20 bytes a partir de
/// `datas` tem em +0xc −1 ou a id do continente da nação (+0x71, lido sem conferir o nulo, como na original). Sem
/// entradas: não.
unsafe fn nation_in_date_confederations(datas: usize, quantas: i32, n: usize) -> bool {
    let mut k = 0;
    while k < quantas {
        let c = cqc11_d(datas + 0xc + k as usize * 0x14) as i32;
        if c == -1 || cqc11_d(cqc11_d(n + 0x71)) as i32 == c {
            return true;
        }
        k += 1;
    }
    false
}

/// Na data só da África (+2 = 1), a nação africana (+0x71 = [0x9cfa08]) fica de fora.
unsafe fn is_african_excluded(datas: usize, n: usize) -> bool {
    *((datas + 2) as *const u8) == 1 && cqc11_d(n + 0x71) != 0 && cqc11_d(cqc11_d(n + 0x71)) == cqc11_d(0x9cfa08)
}

/// A segunda parte da 0x5cbdf0: cada nação `a` da lista ordenada (+0x34fa, conta +0x34fe relida a cada volta) ainda
/// livre, numa das confederações da data e não africana na data da África procura, entre as seguintes `b` livres, a
/// primeira que serve: sorteio de 500 (0x90cfc0, antes dos filtros de `b`), `b` na data e não africana, uma das duas
/// com +0x88 ≥ 5, o par não proibido (0x5cc3e0), mais um sorteio de 1000 se o continente (+0x71) é o mesmo, a soma
/// acima de 400 e `b` ainda não na lista da posição de `a` (+0x34f6, pela posição; o cursor volta ao começo). Achada:
/// o mando sorteado (0x90cfc0 de 2), as seleções principais (0x53b3d0, `b` antes) e o amistoso (0x5c6f00 com a data
/// +3); criado, a resposta forçada (0x5c5f40 com 1), as duas marcadas e cada uma na lista da posição da outra (`b` na
/// de `a`, depois `a` na de `b`; os 4 bytes do registro, a id). Criado ou não, a próxima `a`. No ensaio, o amistoso
/// não é criado: vale o que a original fez (a id de `b` apareceu na lista da posição de `a`), os sorteios vêm da fita
/// e, depois do mando, o resto continua (desde `a` seguinte) no primeiro sorteio de 500 da fita (ou no fim) com que
/// fecha: os de dentro das rotinas do amistoso ficam para trás. Começa na nação `i0`.
unsafe fn pair_nations_for_friendlies(this: usize, datas: usize, n: u32, usadas: usize, mut ens: Option<&mut FriendlyComparerTrial>, i0: i32) {
    let sortear: unsafe extern "C" fn(i32) -> i32 =
        if ens.is_some() { tape_random } else { core::mem::transmute(0x90cfc0usize) };
    let proibido: unsafe extern "thiscall" fn(usize, i32, i32) -> u32 = core::mem::transmute(0x5cc3e0usize);
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> usize = core::mem::transmute(0x5c6f00usize);
    let respond: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5c5f40usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let por: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    // os sorteios: push n; add esp, 4 -> cdecl; 0x53b3d0: push; add esp, 4 -> cdecl; 0x5cc3e0, 0x5c6f00, 0x5c5f40 e
    // 0x68d0d0: mov ecx, esi (ou a lista) -> thiscall
    let quantas = n as i16 as i32;
    let total = |this: usize| cqc11_d(this + 0x34fe) as i32;
    let nacao = |this: usize, k: i32| cqc11_d(cqc11_d(this + 0x34fa).wrapping_add(k as usize * 4));
    let mut i: i32 = i0;
    while i < total(this) {
        let a = nacao(this, i);
        let ida = cqc11_d(a);
        let livre_a = match ens.as_deref() {
            Some(e) => !e.usada(ida),
            None => *((usadas.wrapping_add(ida)) as *const u8) == 0,
        };
        if livre_a && quantas > 0 && nation_in_date_confederations(datas, quantas, a) && !is_african_excluded(datas, a) {
            let mut j = i + 1;
            while j < total(this) {
                if ens.is_some() && (*core::ptr::addr_of!(FRIENDLY_TAPE)).2 {
                    return; // a fita já não bate
                }
                let b = nacao(this, j);
                let idb = cqc11_d(b);
                let livre_b = match ens.as_deref() {
                    Some(e) => !e.usada(idb),
                    None => *((usadas.wrapping_add(idb)) as *const u8) == 0,
                };
                if livre_b {
                    let mut v = sortear(500);
                    if nation_in_date_confederations(datas, quantas, b) && !is_african_excluded(datas, b)
                        && (cqc11_d(a + 0x88) as i32 >= 5 || cqc11_d(b + 0x88) as i32 >= 5)
                        && proibido(this, ida as i32, idb as i32) == 0
                    {
                        if cqc11_d(a + 0x71) == cqc11_d(b + 0x71) {
                            v = v.wrapping_add(sortear(1000)); // cqc11: o mesmo continente
                        }
                        if v > 400 { // cqc11: a soma acima de 400
                            let ja = match ens.as_deref() {
                                Some(e) => e.tem(i as usize, idb as u32),
                                None => {
                                    let l = nation_friendly_list(this, i as usize);
                                    core::ptr::write_unaligned((l + 0xa) as *mut usize, cqc11_d(l + 2));
                                    let mut achou = false;
                                    let mut p = proximo(l);
                                    while p != 0 {
                                        if cqc11_d(p) == idb {
                                            achou = true;
                                            break;
                                        }
                                        p = proximo(nation_friendly_list(this, i as usize));
                                    }
                                    achou
                                }
                            };
                            if !ja {
                                crate::comparison::branch("cqc_11 amistoso entre selecoes");
                                let casa = sortear(2) as u32; // cqc11: o mando sorteado
                                match ens.as_deref_mut() {
                                    Some(e) => {
                                        if list_item_ids(nation_friendly_list(this, i as usize)).contains(&(idb as u32)) {
                                            e.usar(idb);
                                            e.usar(ida);
                                            e.por(i as usize, idb as u32);
                                            e.por(j as usize, ida as u32);
                                        }
                                        let f = core::ptr::addr_of_mut!(FRIENDLY_TAPE);
                                        if (*f).2 {
                                            return;
                                        }
                                        let base = e.clone();
                                        let pontos: Vec<usize> = {
                                            let v = &(*f).0;
                                            ((*f).1..=v.len()).filter(|&p| p == v.len() || v[p].0 == 500).collect()
                                        };
                                        for p in pontos {
                                            (*f).1 = p;
                                            *e = base.clone();
                                            pair_nations_for_friendlies(this, datas, n, 0, Some(&mut *e), i + 1);
                                            if tape_fully_consumed() {
                                                return;
                                            }
                                            (*f).2 = false;
                                        }
                                        (*f).2 = true; // nenhum ponto fecha
                                        return;
                                    }
                                    None => {
                                        let signed_byte_at = selecao(b);
                                        let sa = selecao(a);
                                        let m = acrescentar(this, sa, datas + 3, signed_byte_at, casa);
                                        if m != 0 {
                                            respond(this, m, 1);
                                            *((usadas.wrapping_add(idb)) as *mut u8) = 1;
                                            *((usadas.wrapping_add(ida)) as *mut u8) = 1; // cqc11: as duas usadas
                                            por(nation_friendly_list(this, i as usize), b, 4);
                                            por(nation_friendly_list(this, j as usize), a, 4);
                                        }
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
}

/// { 0x5cbdf0 } (thiscall, ret 8, friendly.cpp, quadro SEH só das datas locais, sem destrutor): `Friendly::
/// arrange_international_friendlies(datas, quantas)`, pelo dia dos amistosos internacionais (0x5cbb00, as entradas de
/// hoje + 12 semanas). O vetor das nações usadas ([0xae2350] bytes zerados, 0x945501; sem memória: a caixa, linha
/// 0x1738, e nada), a primeira parte (as nações com jogo de 4 dias antes a 4 depois) e a segunda (os pares), e o vetor
/// sai (0x9452ca).
unsafe extern "thiscall" fn arrange_international_friendlies(this: usize, datas: usize, n: u32) {
    let zerado: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    // calloc: push 1; push [0xae2350]; add esp, 8 -> cdecl (n, 1); free: push; add esp, 4 -> cdecl
    let usadas = zerado(cqc11_d(0xae2350) as u32, 1);
    if usadas == 0 {
        show_error_at(0x9cd0a0, 0x1738);
        return;
    }
    link_nations_near_date(this, datas, usadas, None);
    pair_nations_for_friendlies(this, datas, n, usadas, None, 0);
    liberar(usadas);
}

/// Comparador: a original cria amistosos (0x5c6f00, 0x5c5f40), acrescenta às listas (0x68d0d0) e sorteia: só ela roda,
/// com o plano em ensaio. A primeira parte do ensaio roda antes dela (depois, o calendário já teria os amistosos
/// novos); a segunda, depois dela, pela fita dos sorteios que ela fez (aleatorio::rastrear: n e resposta, os de dentro
/// das rotinas do amistoso também), decide cada amistoso pelo que a original fez. Confere a sequência dos sorteios (os
/// do ensaio são os da original, salvo os de dentro das rotinas do amistoso, e a fita acaba junto) e, lista a lista de
/// nações, que os itens novos da original são os que o ensaio acrescentaria. (Lado a lado pelo estado do sorteador não
/// servia: a 0x5c6f00 e a 0x5c5f40 sorteiam por dentro na original e o ensaio não as chama.)
unsafe extern "thiscall" fn comparar_arrange_international_friendlies(this: usize, datas: usize, n: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    let nacoes = cqc11_d(0xae2350) as i32;
    let k = nacoes.max(cqc11_d(this + 0x34fe) as i32).max(0) as usize;
    let antes: Vec<Vec<u32>> = (0..k).map(|q| list_item_ids(nation_friendly_list(this, q))).collect();
    let mut e = FriendlyComparerTrial { usadas: vec![0u8; nacoes.max(0) as usize], antes: std::rc::Rc::new(antes), novos: vec![Vec::new(); k] };
    link_nations_near_date(this, datas, 0, Some(&mut e));
    let ((), tape_lock) = crate::rng::trace(|| o(this, datas, n));
    let quantos = tape_lock.len();
    *core::ptr::addr_of_mut!(FRIENDLY_TAPE) = (tape_lock, 0, false);
    pair_nations_for_friendlies(this, datas, n, 0, Some(&mut e), 0);
    let igual = tape_fully_consumed();
    let (pos, falhou) = { let f = &*core::ptr::addr_of!(FRIENDLY_TAPE); (f.1, f.2) };
    *core::ptr::addr_of_mut!(FRIENDLY_TAPE) = (Vec::new(), 0, false);
    let mut dif = Vec::new();
    for q in 0..k.max(e.novos.len()) {
        let mut depois = if q < k { list_item_ids(nation_friendly_list(this, q)) } else { Vec::new() };
        let mut falta = false;
        for x in e.antes.get(q).map(|v| v.as_slice()).unwrap_or(&[]) {
            match depois.iter().position(|y| y == x) {
                Some(p) => {
                    depois.remove(p);
                }
                None => falta = true,
            }
        }
        let mut esperado = e.novos.get(q).cloned().unwrap_or_default();
        depois.sort_unstable();
        esperado.sort_unstable();
        if falta || depois != esperado {
            dif.push((q, depois, esperado));
        }
    }
    SCOREBOARD.record(igual && dif.is_empty(), || {
        format!("arrange_international_friendlies(datas {:x}, {}) tape_lock de {} sorteios, o ensaio parou em {} (n diferente: {}); listas (indice, original, ensaio) {:x?}",
            datas, n as i16, quantos, pos, falhou, dif.iter().take(4).collect::<Vec<_>>())
    });
}

/// { 0x5ca090 } (thiscall, ret 0x20, friendly.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `friendly_check_match_invitation(a, b, clube, dia, ano, data, texto, tamanho)`, pela notícia do convite (0x77bc80,
/// 0x782b70). Com texto e tamanho (palavra) abaixo de 100: a caixa da linha 0x1527 e 1 (a original). Sem a data: com
/// texto, os jogos do clube `a` (+0 id) no dia (dia, ano; 0x5acbe0 numa lista local) dizem se o amistoso foi marcado
/// (o primeiro jogo é de `a` contra `b`) ou cancelado (0x66f4e0 no texto); 0. Com a data: antes de hoje + 2 dias
/// (0x54c5b0), cancelado (com texto) e 0; o clube livre na data (0x5c6b70 com tipo 0): 1; senão, com texto, "{} ... are
/// unavailable" (o byte +0x52 duas vezes e o nome +0x38) e 0.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn friendly_check_match_invitation(this: usize, a: usize, b: usize, clube: usize, dia: u32, ano: u32,
    data: usize, texto: usize, tamanho: u32) -> u32 {
    if texto != 0 && (tamanho as i16) < 100 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, usize, usize, u32) -> u32>(%%I1%%)(
            this, a, b, clube, dia, ano, data, texto, tamanho); // a caixa
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    // 0x66f4e0: add esp, 8 / 0x14 depois -> cdecl (variádica)
    if data == 0 {
        if texto != 0 {
            let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
            let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
            let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
            let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
                core::mem::transmute(0x5acbe0usize);
            // 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24
            let mut l = [0u8; 0x10];
            let lista = l.as_mut_ptr() as usize;
            nova(lista);
            jogos(cqc11_d(0xae2a58), dia, ano, dia, ano, cqc11_d(a) as u32, lista, u32::MAX, 0xffff_fffe, u32::MAX);
            core::ptr::write_unaligned((lista + 0xa) as *mut usize, cqc11_d(lista + 2));
            let p = proximo(lista);
            let msg = if p != 0 && cqc11_d(p + 0x1c) == a && cqc11_d(p + 0x20) == b {
                0x9cd168 // "This match has been arranged."
            } else {
                0x9cd148 // "This match has been cancelled."
            };
            formatar(texto, msg);
            soltar(lista);
        }
        return 0;
    }
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let mut t = [0u8; 8];
    let r = mais(0xae2c90, t.as_mut_ptr() as usize, 2);
    let (tano, tdia) = (cqc11_uw(r + 2), cqc11_w(r));
    if tano > cqc11_uw(data + 2) || (tano == cqc11_uw(data + 2) && tdia > cqc11_w(data)) {
        if texto != 0 {
            formatar(texto, 0x9cd148usize);
        }
        return 0;
    }
    let livre: unsafe extern "stdcall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5c6b70usize);
    // 0x5c6b70: 3 pushes sem add esp -> stdcall (o mov ecx da original sobra)
    if livre(data, clube, 0) != 0 {
        return 1;
    }
    if texto != 0 {
        let k = *((clube + 0x52) as *const i8) as i32;
        formatar(texto, 0x9cd104usize, k, k, clube + 0x38); // "{}<%s - Club Name...>{} are unavailable for this match."
    }
    0
}

/// O texto da notícia (até o tamanho dado, no máximo 0x200 bytes), quando há.
fn news_text_regions(texto: usize, tamanho: u32) -> Vec<(usize, usize)> {
    if texto == 0 {
        Vec::new()
    } else {
        vec![(texto, ((tamanho as i16).max(0) as usize).min(0x200))]
    }
}

/// Comparador: lado a lado pelo texto e a devolução (as listas locais nascem e morrem nas duas); com a caixa, só a
/// original.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_friendly_check_match_invitation(this: usize, a: usize, b: usize, clube: usize, dia: u32,
    ano: u32, data: usize, texto: usize, tamanho: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, usize, usize, u32) -> u32>(%%I1%%);
    if texto != 0 && (tamanho as i16) < 100 {
        return o(this, a, b, clube, dia, ano, data, texto, tamanho);
    }
    let reg = news_text_regions(texto, tamanho);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, a, b, clube, dia, ano, data, texto, tamanho),
        || friendly_check_match_invitation(this, a, b, clube, dia, ano, data, texto, tamanho));
    SCOREBOARD.record(igual && r == n, || format!("friendly_check_match_invitation(a {:x}, b {:x}, clube {:x}, data {:x}) original={} rust={} {:x?} x {:x?}",
        a, b, clube, data, r, n, mo, mr));
    r
}

/// { 0x5ca290 } (thiscall, ret 0x10, friendly.cpp): `friendly_check_tournament_invitation(clube, torneio, texto,
/// tamanho)`, pela notícia do convite para torneio. Com texto e tamanho abaixo de 100: a caixa da linha 0x1575 e 1 (a
/// original). Sem torneio ou com a data dele (+4) antes de hoje + 2 dias: "This competition has been cancelled" (com
/// texto) e 0. O clube livre na data (0x5c6b70 com o tipo +0xc): 1; senão, com texto, "{} ... are unavailable for this
/// competition." e 0.
unsafe extern "thiscall" fn friendly_check_tournament_invitation(this: usize, clube: usize, torneio: usize, texto: usize, tamanho: u32) -> u32 {
    if texto != 0 && (tamanho as i16) < 100 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I2%%)(this, clube, torneio, texto, tamanho);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mut cancelado = torneio == 0;
    if !cancelado {
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let mut t = [0u8; 8];
        let r = mais(0xae2c90, t.as_mut_ptr() as usize, 2);
        let (tano, tdia) = (cqc11_uw(r + 2), cqc11_w(r));
        let d = torneio + 4;
        cancelado = tano > cqc11_uw(d + 2) || (tano == cqc11_uw(d + 2) && tdia > cqc11_w(d));
    }
    if cancelado {
        if texto != 0 {
            formatar(texto, 0x9cd1d4usize); // "This competition has been cancelled"
        }
        return 0;
    }
    let livre: unsafe extern "stdcall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5c6b70usize);
    // ponytail: a original empurra o tipo (byte +0xc) com o resto do ecx em cima; a 0x5c6b70 só lê o byte
    if livre(torneio + 4, clube, *((torneio + 0xc) as *const u8) as u32) != 0 {
        return 1;
    }
    if texto != 0 {
        let k = *((clube + 0x52) as *const i8) as i32;
        formatar(texto, 0x9cd188usize, k, k, clube + 0x38); // "... are unavailable for this competition."
    }
    0
}

/// Comparador: lado a lado pelo texto e a devolução; com a caixa, só a original.
unsafe extern "thiscall" fn comparar_friendly_check_tournament_invitation(this: usize, clube: usize, torneio: usize, texto: usize, tamanho: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(%%I2%%);
    if texto != 0 && (tamanho as i16) < 100 {
        return o(this, clube, torneio, texto, tamanho);
    }
    let reg = news_text_regions(texto, tamanho);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clube, torneio, texto, tamanho),
        || friendly_check_tournament_invitation(this, clube, torneio, texto, tamanho));
    SCOREBOARD.record(igual && r == n, || format!("friendly_check_tournament_invitation(clube {:x}, torneio {:x}) original={} rust={} {:x?} x {:x?}",
        clube, torneio, r, n, mo, mr));
    r
}
