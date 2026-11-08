// ---------- %%LEVA%%a leva: a oferta de contrato acrescentada ou atualizada, a oferta de contrato tirada pela vaga e o menor salário pedido (contract_manager.cpp) ----------

unsafe fn cpw4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpw4_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// O contador de ofertas de contrato da pessoa (byte +8 do registro de 79 bytes em [0xae2d0c]).
unsafe fn contract_offer_counter(pessoa: usize) -> usize {
    cpw4_d(0xae2d0c).wrapping_add(cpw4_d(pessoa).wrapping_mul(0x4f)).wrapping_add(8)
}

/// A atribuição de data (0x417c10: thiscall(destino, temporária, origem)).
unsafe fn contract_manager_copy_date(destino: usize, origem: usize) {
    let copia: unsafe extern "thiscall" fn(usize, *mut [u32; 2], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u32; 2];
    copia(destino, &mut t, origem);
}

/// A oferta de contrato da pessoa (id em +0) pelo clube (id em +4) na lista `l` (nós: +0 o item, +6 o próximo), sem
/// mexer no cursor; nenhuma: 0.
unsafe fn find_contract_offer(l: usize, id: usize, clube: usize) -> usize {
    let mut no = cpw4_d(l + 2);
    let mut n = 0;
    while no != 0 && n < 100000 {
        let x = cpw4_d(no);
        if cpw4_d(x) == id && cpw4_d(x + 4) == clube {
            return x;
        }
        no = cpw4_d(no + 6);
        n += 1;
    }
    0
}

/// O ensaio do comparador: a cópia não cria nem acrescenta à lista (0x944cf1, 0x68d020, 0x68d0d0) e não chama o que vem
/// depois (0x8c1f80, 0x8c2060, 0x4cd230), que alocam, sorteiam e mexem nas ofertas.
static mut CPW4_ENSAIO: bool = false;

/// { 0x4cc490 } (thiscall, ret 0x1c, contract_manager.cpp, quadro SEH sem destrutor): `CONTRACT_MANAGER::
/// add_contract_offer(termos, vaga, valor, marca, a5, decidir, acertado)`, pela oferta de empréstimo (0x8bed60), pelas
/// ofertas guardadas (0x4cf4a0), pelas listas (0x84a020), pela negociação (0x4cda70) e pela tela. Calcula hoje + 14 (0x54c5b0,
/// descartado) e monta um registro local de 0x6f bytes (contrato 0x85e410, data 0x54a110). Com ofertas contadas (+8 do
/// registro de 79 bytes da pessoa em [0xae2d0c]) e lista (+0x34), a oferta do mesmo clube é atualizada: os termos (o
/// operator= 0x4cc810 embutido), +0x50 = 2 se `acertado` (senão 0), +0x51 = hoje, +0x59 a vaga, +0x5d o valor, +0x61 a
/// marca, +0x62 = 1 fora do contrato de tipo 6. Senão o registro local recebe o mesmo (+0x63 = −1, o resto fica como a
/// original o deixa) e entra na lista (criada se faltar; 0x68d0d0), e o contador sobe. Com vaga, a oferta (0x8c61e0) do
/// mesmo jogador marca os termos oferecidos (0x8c1f80), senão o jogador da troca (0x8c2060); com `decidir`, agenda a
/// decisão (0x4cd230).
unsafe extern "thiscall" fn add_contract_offer(this: usize, termos: usize, vaga: i32, valor: u32, marca: u32, a5: u32, decidir: u32, acertado: u32) {
    let _ = a5;
    let mais: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut quinzena = [0u32; 2];
    mais(0xae2c90, &mut quinzena, 0xe);
    // ponytail: os bytes +0x67..+0x6e do registro local a original não preenche (sobra de pilha); a cópia usa zero
    let mut registro = [0u8; 0x6f];
    let r = registro.as_mut_ptr() as usize;
    construir(r);
    data_padrao(r + 0x51);
    let id = cpw4_d(termos);
    let mut item = 0;
    if cpw4_b(contract_offer_counter(termos)) != 0 && cpw4_d(this + 0x34) != 0 {
        let clube = cpw4_d(termos + 4);
        let l = cpw4_d(this + 0x34);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, cpw4_d(l + 2));
        let mut x = proximo(cpw4_d(this + 0x34));
        while x != 0 {
            if cpw4_d(x) == id && cpw4_d(x + 4) == clube {
                item = x;
                break;
            }
            x = proximo(cpw4_d(this + 0x34));
        }
    }
    let tipo6 = cpw4_b(termos + 0x35) & 0x3f == 6;
    if item != 0 {
        crate::comparacao::ramo("cpw_4 oferta atualizada");
        // ponytail: a original embute o operator= (0x4cc810) aqui; a cópia chama a versão Rust dele, as mesmas gravações
        contract_manager_operator(item, termos);
        *((item + 0x50) as *mut u8) = if acertado != 0 { 2 } else { 0 };
        contract_manager_copy_date(item + 0x51, 0xae2c90);
        cpw4_pd(item + 0x59, vaga as u32);
        cpw4_pd(item + 0x5d, valor);
        *((item + 0x61) as *mut u8) = marca as u8;
        *((item + 0x62) as *mut u8) = !tipo6 as u8;
    } else {
        let operador: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cc810usize);
        operador(r, termos);
        *((r + 0x50) as *mut u8) = 0;
        contract_manager_copy_date(r + 0x51, 0xae2c90);
        cpw4_pd(r + 0x5d, valor);
        cpw4_pd(r + 0x59, vaga as u32);
        *((r + 0x61) as *mut u8) = marca as u8;
        cpw4_pd(r + 0x63, 0xffff_ffff);
        *((r + 0x62) as *mut u8) = !tipo6 as u8;
        if CPW4_ENSAIO {
            core::ptr::copy_nonoverlapping(r as *const u8, core::ptr::addr_of_mut!(CPW4_REGISTRO) as *mut u8, 0x6f);
        } else {
            if cpw4_d(this + 0x34) == 0 {
                let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
                let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
                let p = novo(0xe);
                let l = if p != 0 { lista_nova(p) } else { 0 };
                cpw4_pd(this + 0x34, l as u32);
            }
            let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
            acrescentar(cpw4_d(this + 0x34), r, 0x6f);
        }
        crate::comparacao::ramo("cpw_4 oferta nova");
        let c = contract_offer_counter(termos) as *mut u8;
        *c = (*c).wrapping_add(1); // cpw4: o contador de ofertas da pessoa
    }
    if CPW4_ENSAIO {
        return;
    }
    if vaga != -1 {
        let vaga_da: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x8c61e0usize);
        let tm = cpw4_d(0xde1f54);
        let v = vaga_da(tm, vaga);
        if cpw4_d(cpw4_d(v) + 0xc) == cpw4_d(termos) {
            crate::comparacao::ramo("cpw_4 termos ao jogador");
            let marcar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x8c1f80usize);
            marcar(cpw4_d(0xde1f54), vaga); // cpw4: os termos oferecidos ao jogador
        } else {
            let troca: unsafe extern "thiscall" fn(usize, i32, u32) = core::mem::transmute(0x8c2060usize);
            troca(cpw4_d(0xde1f54), vaga, cpw4_d(termos) as u32);
        }
    }
    if decidir != 0 {
        crate::comparacao::ramo("cpw_4 agendar decisao");
        let decisao: unsafe extern "thiscall" fn(usize, u32, i32) = core::mem::transmute(0x4cd230usize);
        decisao(this, cpw4_d(termos) as u32, vaga); // cpw4: a decisão agendada
    }
}

/// O registro que a cópia em ensaio acrescentaria à lista.
static mut CPW4_REGISTRO: [u8; 0x6f] = [0; 0x6f];

/// Comparador. Oferta já na lista (achada antes, sem mexer no cursor): a cópia em ensaio e a original a partir do mesmo
/// estado, comparando a oferta (0x63 bytes), o contador e a cabeça da lista (o cursor). Oferta nova (acrescentada, aloca):
/// a cópia em ensaio monta o registro; a original acrescenta, e o nó novo (o último do jogador pelo clube) tem de ter os
/// mesmos 0x63 primeiros bytes, com o contador + 1.
unsafe extern "thiscall" fn comparar_add_contract_offer(this: usize, termos: usize, vaga: i32, valor: u32, marca: u32, a5: u32, decidir: u32, acertado: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32)>(%%I0%%);
    let id = cpw4_d(termos);
    let clube = cpw4_d(termos + 4);
    let cont = contract_offer_counter(termos);
    let l = cpw4_d(this + 0x34);
    let item = if cpw4_b(cont) != 0 && l != 0 { find_contract_offer(l, id, clube) } else { 0 };
    let mut todas = vec![(cont, 1usize)];
    if item != 0 {
        todas.push((item, 0x63));
        todas.push((l, 0xe));
    }
    let n_cmp = todas.len();
    todas.extend(TRANSFER_MANAGER_DATE_STATICS);
    todas.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&todas);
    CPW4_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| add_contract_offer(this, termos, vaga, valor, marca, a5, decidir, acertado)));
    CPW4_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&todas[..n_cmp]);
    let registro = CPW4_REGISTRO;
    antes.escrever();
    o(this, termos, vaga, valor, marca, a5, decidir, acertado);
    let mo = crate::comparacao::Memoria::ler(&todas[..n_cmp]);
    let igual = if item != 0 {
        mo == mr
    } else {
        // o nó novo: o último do jogador pelo clube na lista (criada pela original se faltava)
        let l = cpw4_d(this + 0x34);
        let mut novo = 0;
        if l != 0 {
            let mut no = cpw4_d(l + 2);
            let mut n = 0;
            while no != 0 && n < 100000 {
                let x = cpw4_d(no);
                if cpw4_d(x) == id && cpw4_d(x + 4) == clube {
                    novo = x;
                }
                no = cpw4_d(no + 6);
                n += 1;
            }
        }
        mo == mr && novo != 0 && core::slice::from_raw_parts(novo as *const u8, 0x63) == &registro[..0x63]
    };
    PLACAR.registrar(igual, || format!("add_contract_offer(pessoa {:#x}, clube {:#x}, vaga {}, atualizada {}) original={:x?} rust={:x?}", id, clube, vaga, item != 0, mo, mr));
}

/// { 0x4ccb30 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract_offer_for_transfer(pessoa,
/// vaga)`, pelas ofertas retiradas e desfeitas (0x8bf680, 0x8bfcb0, 0x8c4950, 0x8c4e10, 0x8d0f70, 0x8df9b0): na lista das
/// ofertas de contrato (+0x34; cursor recomeçado) a da pessoa com a vaga (+0x59). Achada: com +0x63 marcado, a original
/// 0x7a81a0 de [0xb7434c] com +0x63, +0x67 e +0x6b; acertada (+0x50 = 2), desliga a marca de contrato acertado (0x8c5430);
/// tira da lista (0x68d1f0) e o contador de ofertas da pessoa (+8 do registro de 79 bytes) desce.
unsafe extern "thiscall" fn remove_contract_offer_for_transfer(this: usize, pessoa: usize, vaga: i32) {
    let l = cpw4_d(this + 0x34);
    if l == 0 {
        return;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpw4_d(l + 2));
    let mut x = proximo(cpw4_d(this + 0x34));
    while x != 0 {
        if cpw4_d(x) == cpw4_d(pessoa) && cpw4_d(x + 0x59) as i32 == vaga {
            break;
        }
        x = proximo(cpw4_d(this + 0x34));
    }
    if x == 0 {
        return;
    }
    crate::comparacao::ramo("cpw_4 oferta tirada pela vaga");
    if cpw4_d(x + 0x63) as i32 != -1 {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(0x7a81a0usize);
        f(cpw4_d(0xb7434c), cpw4_d(x + 0x63) as u32, cpw4_d(x + 0x67) as u32, cpw4_d(x + 0x6b) as u32);
    }
    if cpw4_b(x + 0x50) == 2 {
        let desmarcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c5430usize);
        desmarcar(cpw4_d(0xde1f54), pessoa);
    }
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    tirar(cpw4_d(this + 0x34), x);
    let c = contract_offer_counter(pessoa) as *mut u8;
    *c = (*c).wrapping_sub(1);
}

/// Comparador: a original tira o item da lista (libera): só ela, conferindo o plano feito antes: com a oferta da pessoa
/// pela vaga na lista, o contador desce 1; sem ela, fica.
unsafe extern "thiscall" fn comparar_remove_contract_offer_for_transfer(this: usize, pessoa: usize, vaga: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(%%I1%%);
    let l = cpw4_d(this + 0x34);
    let mut achou = false;
    if l != 0 {
        let mut no = cpw4_d(l + 2);
        let mut n = 0;
        while no != 0 && n < 100000 {
            let x = cpw4_d(no);
            if cpw4_d(x) == cpw4_d(pessoa) && cpw4_d(x + 0x59) as i32 == vaga {
                achou = true;
                break;
            }
            no = cpw4_d(no + 6);
            n += 1;
        }
    }
    let antes = cpw4_b(contract_offer_counter(pessoa));
    o(this, pessoa, vaga);
    let depois = cpw4_b(contract_offer_counter(pessoa));
    let esperado = if achou { antes.wrapping_sub(1) } else { antes };
    PLACAR.registrar(depois == esperado, || format!("remove_contract_offer_for_transfer(pessoa {:#x}, vaga {}) achou {} contador {} -> {}", cpw4_d(pessoa), vaga, achou, antes, depois));
}

/// { 0x4ce450 } (stdcall, ret 8, contract_manager.cpp, quadro SEH sem destrutor): o menor salário que a pessoa pede ao
/// clube: num contrato local (0x85e410), o desejado (0x85fc50 com −1, a situação 1 e depois 2, 0xb), no máximo 10 milhões.
/// Pela tela do contrato.
unsafe extern "stdcall" fn contract_manager_lowest_wage_demand(pessoa: usize, clube: usize) -> i32 {
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x85fc50usize);
    let mut local = [0u8; 0x50];
    let c = local.as_mut_ptr() as usize;
    construir(c);
    let mut menor = 0x989680;
    let a = desejado(c, pessoa, clube, -1, 1, 0xb, 0, 0);
    if a < menor {
        menor = a;
    }
    let b = desejado(c, pessoa, clube, -1, 2, 0xb, 0, 0);
    if b < menor { b } else { menor }
}

/// Comparador: a original e a cópia a partir do mesmo estado (o sorteio e o último contrato pedido [0xae1080]); a
/// devolução e a memória.
unsafe extern "stdcall" fn comparar_contract_manager_lowest_wage_demand(pessoa: usize, clube: usize) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> i32>(%%I2%%);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(pessoa, clube), || crate::aleatorio::sem_contar(|| contract_manager_lowest_wage_demand(pessoa, clube)));
    PLACAR.registrar(igual && r == n, || format!("contract_manager_lowest_wage_demand(pessoa {:#x}) original={} {:x?} rust={} {:x?}", cpw4_d(pessoa), r, mo, n, mr));
    r
}
