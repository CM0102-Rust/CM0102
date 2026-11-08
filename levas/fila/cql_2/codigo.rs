// ---------- %%LEVA%%a leva: os números de camisa: dar número a quem chega e numerar o elenco de um clube (squad_manager.cpp) ----------

unsafe fn cql2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// A pessoa `p` está na tabela das pessoas com contrato (id < [0xae2364], com sinal).
unsafe fn is_contracted_staff(p: usize) -> bool {
    (cql2_d(p) as i32) < cql2_d(0xae2364) as i32
}

/// Os números preferidos (índice = número − 1) pela posição da ficha de jogador `f` (as notas de posição de 1 a 20,
/// "boa" = 15 ou mais): goleiro (+0xf) o 1; zagueiro ou líbero (+0x11, +0x10) o 2 se joga pela direita (+0x17), o 3 se
/// pela esquerda (+0x18), senão o 4, o 5 ou o 6; volante, meia ou meia-atacante (+0x12, +0x13, +0x14) o 6, o 7 ou o 8;
/// atacante (+0x15) o 8, o 9, o 10 ou o 11. Termina em 0xff. Nenhuma das posições boas: `None` (a original lê então a
/// lista de 4 bytes sem valor na pilha, ver `compare_allocate_squad_number`).
unsafe fn preferred_squad_numbers(f: usize) -> Option<[u8; 5]> {
    let b = |o: usize| *((f + o) as *const i8) >= 15;
    if b(0xf) {
        return Some([0, 0xff, 0xff, 0xff, 0xff]);
    }
    if b(0x11) || b(0x10) {
        if b(0x17) {
            return Some([1, 0xff, 0xff, 0xff, 0xff]);
        }
        if b(0x18) {
            return Some([2, 0xff, 0xff, 0xff, 0xff]);
        }
        return Some([3, 4, 5, 0xff, 0xff]);
    }
    if b(0x13) || b(0x12) || b(0x14) {
        return Some([5, 6, 7, 0xff, 0xff]);
    }
    if b(0x15) {
        return Some([7, 8, 9, 10, 0xff]);
    }
    None
}

/// { 0x85a470 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::allocate_squad_number`): dá um número de camisa
/// à pessoa `p` que chega ao clube `c`. Marca os números ocupados pelo elenco (0x541f90(c, lista, 0), cdecl de 3; a
/// lista ordenada pela idade com o qsort 0x945333 e a 0x540130; quem está fora da tabela das pessoas com contrato ou sem
/// ficha de jogador não conta) com o número de cada um (0x85a760, em módulo). Se `p` está na tabela e ainda não tem
/// número: o primeiro número livre entre os preferidos da posição (`preferred_squad_numbers`); senão o primeiro livre de 12 a
/// 50; e, se ainda sem número, o maior livre de 11 para baixo. O número vai positivo para o clube de técnico humano
/// (0x600a50(c, 0, 1, 0, 0), cdecl de 5) e negativo para os outros (0x85a850). Sem a pessoa, nada.
unsafe extern "thiscall" fn allocate_squad_number(this: usize, c: usize, p: usize) {
    if p == 0 {
        return;
    }
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let numero: unsafe extern "thiscall" fn(usize, usize, usize) -> i8 = core::mem::transmute(0x85a760usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x85a850usize);
    let mut usado = [0u8; 50];
    let mut lista = [0usize; 50];
    elenco(c, lista.as_mut_ptr(), 0);
    ordenar(lista.as_mut_ptr() as usize, 0x32, 4, 0x540130);
    for q in lista.iter_mut() {
        let x = *q;
        if x == 0 {
            continue;
        }
        if cql2_d(x + 0x61) == 0 {
            *q = 0;
            continue;
        }
        if is_contracted_staff(x) {
            let n = (numero(this, x, c) as i32).abs() as u8;
            if n != 0 {
                usado[(n as i8 as isize - 1) as usize] = 1;
            }
        }
    }
    if !is_contracted_staff(p) || numero(this, p, c) != 0 {
        return;
    }
    let pref = preferred_squad_numbers(cql2_d(p + 0x61) as usize).unwrap_or([0xff; 5]);
    // o número com o sinal do clube: positivo com técnico humano, negativo sem
    let sinal = |k: u8| -> u32 {
        if humano(c, 0, 1, 0, 0) != 0 {
            k.wrapping_add(1) as u32
        } else {
            0xffu8.wrapping_sub(k) as u32
        }
    };
    let mut b = 0usize;
    while pref[b] != 0xff {
        let k = pref[b];
        if usado[k as i8 as usize] == 0 {
            gravar(this, p, c, sinal(k));
            usado[k as usize] = 1;
            crate::comparison::branch("cql_2 numero da posicao");
            return; // cql2: o número da posição
        }
        b += 1;
        if b >= 5 {
            break;
        }
    }
    if let Some(k) = (11u8..50).find(|&k| usado[k as usize] == 0) {
        gravar(this, p, c, sinal(k));
        usado[k as usize] = 1;
    }
    if numero(this, p, c) != 0 {
        return;
    }
    if let Some(k) = (0u8..=10).rev().find(|&k| usado[k as usize] == 0) {
        gravar(this, p, c, sinal(k));
    }
}

/// Lado a lado: o +0x3a dos dois contratos da pessoa (a única que a rotina numera) e a marca da caixa. Quando nenhuma
/// posição da ficha é boa, a original lê a lista de preferidos de 4 bytes sem valor na pilha (sobra de quem chamou):
/// a cópia usa a lista vazia (um valor que o jogo produz quando a sobra é 0xff) e o comparador deixa só a original.
unsafe extern "thiscall" fn compare_allocate_squad_number(this: usize, c: usize, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    if p == 0 || cql2_d(p + 0x61) == 0 || preferred_squad_numbers(cql2_d(p + 0x61) as usize).is_none() {
        return o(this, c, p);
    }
    let reg = squad_number_regions(&[p]);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, c, p), || allocate_squad_number(this, c, p));
    SCOREBOARD.record(igual, || format!("allocate_squad_number(clube {:x}, pessoa {:x}): original {:?} rust {:?}", c, p, mo.0, mr.0));
}

/// O +0x3a dos contratos (o do clube e o de empréstimo) das pessoas `ps` e a marca da caixa [0xb67a34].
unsafe fn squad_number_regions(ps: &[usize]) -> Vec<(usize, usize)> {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let mut reg = vec![(0xb67a34usize, 4usize)];
    for &p in ps {
        if p == 0 {
            continue;
        }
        for k in [contrato(cql2_d(0xae1050) as usize, p), emprestimo(cql2_d(0xae1050) as usize, p)] {
            if k != 0 && !reg.contains(&(k + 0x3a, 1)) {
                reg.push((k + 0x3a, 1));
            }
        }
    }
    reg
}

/// As vagas da escalação de referência da 0x859fe0 para a 0x6e2000: os 11 titulares e os 9 reservas, cada um com as
/// posições que servem (máscaras: 1 goleiro, 4/0x10/0x40 os setores, 0x80..0x800 as faixas [inferência]).
const SQUAD_MANAGER_REFERENCE_LINEUP_SLOTS: [u16; 20] = [
    0x1, 0x804, 0x84, 0x210, 0x204, 0x204, 0x810, 0x210, 0x240, 0x240, 0x90, // cql2: os titulares
    0x10, 0x1, 0x4, 0x40, 0x40, 0x10, 0x4, 0x40, 0x4, // os reservas
];

/// { 0x859fe0 } (thiscall, ret 8, squad_manager.cpp, `SQUAD_MANAGER::assign_club_squad_numbers`): numera o elenco do
/// clube `c`. Junta os jogadores do elenco (+0xd7, 50 vagas; só os da tabela das pessoas com contrato) e os do clube
/// ligado (0x540a50(c, 0, 1), cdecl de 3; até 50 no total); escala os 11 titulares e os 9 reservas de referência
/// (0x6e2000, cdecl de 15, com `SQUAD_MANAGER_REFERENCE_LINEUP_SLOTS` e os modos 0x5840) e põe os outros depois, nas vagas livres; ordena os 30
/// últimos pela idade (qsort 0x945333 com a 0x540130) e junta a lista. Depois, na ordem: do 22º em diante, quem não tem
/// contrato com o clube (0x4cf5f0) ou tem contrato do tipo 6 ou 7 (+0x4f, nibble alto) [inferência: juvenil e amador]
/// fica sem número; quem tem número (0x85a760) o guarda se ele está livre e, sem `manter`, se é de titular (até 11)
/// entre os 11 primeiros ou de reserva (12 em diante) depois deles (senão fica sem número); com `manter`, guarda o número
/// com o sinal trocado; quem não tem número ganha o da sua ordem (1º o 1...) se livre. Os 40 primeiros sem número que
/// têm contrato com o clube (fora os tipos 6 e 7) ganham um pela 0x85a470. No fim, cada número guardado é regravado:
/// positivo (o número) com técnico humano (0x600a50(c, 0, 1, 0, 0)), senão −1, −2... na ordem dos números.
unsafe extern "thiscall" fn assign_club_squad_numbers(this: usize, c: usize, manter: u32) {
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let pick: AssignPositionsFn = core::mem::transmute(0x6e2000usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let do_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let numero: unsafe extern "thiscall" fn(usize, usize, usize) -> i8 = core::mem::transmute(0x85a760usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x85a850usize);
    let dar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x85a470usize);
    let mut guardado = [0usize; 50]; // por número − 1
    let mut ordem = [0usize; 50];
    let mut todos = [0usize; 50];
    let mut n = 0usize;
    for k in 0..50 {
        let x = cql2_d(c + 0xd7 + 4 * k) as usize;
        if x != 0 && is_contracted_staff(x) {
            todos[n] = x;
            n += 1;
        }
    }
    let l = parceiro(c, 0, 1);
    if l != 0 {
        for k in 0..50 {
            if n >= 50 {
                break;
            }
            let x = cql2_d(l + 0xd7 + 4 * k) as usize;
            if x != 0 && is_contracted_staff(x) {
                todos[n] = x;
                n += 1;
            }
        }
    }
    let vagas = SQUAD_MANAGER_REFERENCE_LINEUP_SLOTS;
    crate::comparison::branch("cql_2 escala");
    pick(todos.as_ptr() as usize, n as u32, vagas.as_ptr() as usize, 0xb, 9, ordem.as_mut_ptr() as usize, 0x5840, c, 0, 0, 0, 0, 0, 0, 0);
    let mut livre = 0usize;
    for &x in todos[..n].iter() {
        if ordem[..20].contains(&x) {
            continue;
        }
        while livre < 50 && ordem[livre] != 0 {
            livre += 1;
        }
        if livre < 50 {
            ordem[livre] = x;
        }
    }
    ordenar(ordem[20..].as_mut_ptr() as usize, 0x1e, 4, 0x540130);
    let mut a = 0usize;
    for b in 0..50 {
        let x = ordem[b];
        if x == 0 || b <= a {
            continue;
        }
        while a < b && ordem[a] != 0 {
            a += 1;
        }
        if a < b {
            ordem[a] = x;
            a += 1;
            ordem[b] = 0;
        }
    }
    // `vez` = −1, −2... (char), a ordem de cada um; `k` = 1, 2... a mesma ordem em positivo
    for i in 0..50usize {
        let vez = (-1 - i as i32) as i8;
        let x = ordem[i];
        if x == 0 {
            continue;
        }
        if vez < -21 {
            let ct = do_clube(cql2_d(0xae1050) as usize, x, c);
            let t = if ct != 0 { *((ct + 0x4f) as *const u8) & 0xf0 } else { 0 };
            if ct == 0 || t == 0x60 || t == 0x70 {
                gravar(this, x, c, 0);
                continue;
            }
        }
        let nb = (numero(this, x, c) as i32).abs() as u8 as i8;
        if nb == 0 {
            if guardado[i] == 0 {
                guardado[i] = x;
                gravar(this, x, c, vez as u8 as u32);
            }
            continue;
        }
        let vaga = (nb as isize - 1) as usize;
        if guardado[vaga] != 0 {
            gravar(this, x, c, 0);
            continue;
        }
        if manter != 0 {
            gravar(this, x, c, nb.wrapping_neg() as u8 as u32);
        } else if (nb < 12 && vez < -11) || (nb >= 12 && vez > -12) {
            crate::comparison::branch("cql_2 perde o numero");
            gravar(this, x, c, 0); // cql2: o número fora do lugar
            continue;
        }
        if nb > 0 {
            guardado[vaga] = x;
        }
    }
    for b in 0..40usize {
        let x = ordem[b];
        if x == 0 {
            continue;
        }
        let ct = do_clube(cql2_d(0xae1050) as usize, x, c);
        if ct == 0 {
            continue;
        }
        let t = *((ct + 0x4f) as *const u8) & 0xf0;
        if t == 0x60 || t == 0x70 {
            continue;
        }
        if numero(this, x, c) == 0 {
            dar(this, c, x);
        }
    }
    if humano(c, 0, 1, 0, 0) != 0 {
        for (k, &x) in guardado.iter().enumerate() {
            if x != 0 {
                gravar(this, x, c, k as u32 + 1);
            }
        }
    } else {
        let mut v = -1i8;
        for &x in guardado.iter() {
            if x != 0 {
                crate::comparison::branch("cql_2 regrava negativo");
                gravar(this, x, c, v as u8 as u32); // cql2: regrava negativo
            }
            v = v.wrapping_sub(1);
        }
    }
}

/// Lado a lado: o +0x3a dos contratos de quem está no elenco do clube e no do clube ligado (as únicas pessoas que a
/// rotina e a 0x85a470 numeram) e a marca da caixa.
unsafe extern "thiscall" fn compare_assign_club_squad_numbers(this: usize, c: usize, manter: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%);
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut ps: Vec<usize> = (0..50).map(|k| cql2_d(c + 0xd7 + 4 * k) as usize).collect();
    let l = parceiro(c, 0, 1);
    if l != 0 {
        ps.extend((0..50).map(|k| cql2_d(l + 0xd7 + 4 * k) as usize));
    }
    let reg = squad_number_regions(&ps);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, c, manter), || assign_club_squad_numbers(this, c, manter));
    SCOREBOARD.record(igual, || {
        let d: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).take(12).map(|(a, b)| format!("{:x}: {:?}/{:?}", a.0, a.1, b.1)).collect();
        format!("assign_club_squad_numbers(clube {:x}, manter {}): {}", c, manter, d.join(", "))
    });
}
