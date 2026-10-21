// ---------- 207a leva: as telas da partida (15): a conferência da escalação e o índice das estatísticas da partida no save (match_screens.cpp, match_stats.cpp) ----------

/// strcpy de C (a original copia com `rep movs`).
unsafe fn escalacao_copiar(d: *mut u8, s: *const u8) {
    let mut i = 0;
    loop {
        let c = *s.add(i);
        *d.add(i) = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
}

/// strcat de C.
unsafe fn escalacao_juntar(d: *mut u8, s: *const u8) {
    let mut n = 0;
    while *d.add(n) != 0 {
        n += 1;
    }
    escalacao_copiar(d.add(n), s);
}

/// O subtítulo da conferência ("Team Selection" ou "Please Confirm"): o texto traduzido em 0xde1f64 e
/// 0x803f80 no gerente.
unsafe fn titulo_da_escalacao(t: usize) {
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let subtitulo: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x803f80usize);
    texto(0xde1f64, t);
    subtitulo(GERENTE_DE_TELAS, 0xde1f64, 1);
}

/// Os nomes dos `n` jogadores de `lista` juntos em 0xde1f64 ("a", "a and b", "a, b, and c" pelos
/// textos traduzidos) e, para cada um, a ligação no Text `texto_obj` (0x4ae8a0, tipo 0x7dc na seleção,
/// senão 0x7d7).
unsafe fn nomes_da_escalacao(texto_obj: usize, lista: &[usize], n: i8, clube: usize) {
    let nome_do: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(0x53dba0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let ligacao: unsafe extern "thiscall" fn(usize, *const u8, u32, usize) -> i32 = core::mem::transmute(0x4ae8a0usize);
    let mut nome = [0u8; 0xc8];
    let mut juntos = [0u8; 0x7d0];
    let mut k: i8 = 0;
    while k < n {
        let p = lista[k as usize];
        nome_do(p, nome.as_mut_ptr(), 0xc8, 1);
        if k == 0 {
            escalacao_copiar(juntos.as_mut_ptr(), nome.as_ptr());
        } else {
            let modelo: usize = if k as i32 == n as i32 - 1 { if n == 2 { 0xa17b40 } else { 0xa17b18 } } else { 0xa17af4 };
            texto(juntos.as_mut_ptr() as usize, modelo, 0xde1f64usize, nome.as_ptr() as usize);
        }
        escalacao_copiar(0xde1f64 as *mut u8, juntos.as_ptr());
        let tipo = if selecao(clube) != 0 { 0x7dc } else { 0x7d7 };
        ligacao(texto_obj, nome.as_ptr(), tipo, p);
        k += 1;
    }
}

/// O aviso da conferência: a mensagem `msg` no Text (0x4ae660) numa área (0x5ed200) e o botão "Ok"
/// (0x55f990). `caixa_sobre`/`ok_sobre`: onde a original empurra a cor da caixa / a cor do texto do
/// botão com os 16 bits de cima da devolução da chamada anterior (varia por ramo).
unsafe fn aviso_da_escalacao(texto_obj: usize, msg: *const u8, caixa_sobre: bool, ok_sobre: bool) {
    let definir: unsafe extern "thiscall" fn(usize, *const u8) -> usize = core::mem::transmute(0x4ae660usize);
    let area: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(0x5ed200usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let linha: LinhaDoDisplay = core::mem::transmute(0x55f990usize);
    let r = definir(texto_obj, msg);
    let cor = if caixa_sobre { cor_sobre(r, 0xae3184) } else { cor_em(0xae3184) };
    area(0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, cor, texto_obj);
    let r = texto(0xde1f64, 0x990f7c) as usize;
    let cor_do_texto = if ok_sobre { cor_sobre(r, 0xae3184) } else { cor_em(0xae3184) };
    linha(display_do_humano(), 2, 0x64, 0x22b, 0x316, 0x24e, 0, 0, 0x30, cor_sobre(r, 0xae317e), cor_em(0xae317e), 0xc, 4, cor_do_texto, 0xde1f64, 0, 1, 0, usize::MAX);
}

/// { 0x7260f0 } (cdecl, match_screens.cpp, ~7 KB com os pedaços até 0x727c2e): desenha a conferência
/// da escalação do clube (parâmetro 0) para o jogo (parâmetro 1) e a equipe (parâmetro 2). Passa pelas
/// 20 vagas da tática (0x8968f0): conta titulares (< 11), reservas (< 11 + o banco [jogo+0x4d] & 0xf)
/// e os de fora; dos que estão no time, junta os convocados pela seleção (0x7757d0), os emprestados a
/// outro clube (0x4cf6d0/0x4cf5a0, fora da seleção), os inelegíveis com o mesmo motivo do primeiro
/// (0x8cf5e0), os suspensos (0x55d270), os lesionados e os indisponíveis (0x62c400/0x62ed70: lesão de
/// tipo 0x4d a 0x51 no idioma 0 conta como indisponível) e os sem permissão de trabalho (0x8c7560, fora
/// da competição [0x9cf684] e de amistoso). Depois o fundo do clube, o menu, as cores e o título com o
/// nome. O primeiro grupo não vazio, nessa ordem, vira o aviso "Team Selection" com os nomes (ligados)
/// e o botão "Ok"; senão o limite do banco do dia (0x8cf880); senão menos de 11 titulares; senão
/// "Please Confirm": o capitão escolhido (0x727e20) quando não há um válido, os cobradores de bola
/// parada fora dos 11, os reservas que faltam, os de fora do grupo, "Proceed to the match ?" e a lista
/// com "No" e "Yes".
unsafe extern "C" fn desenhar_conferencia_da_escalacao() {
    crate::comparacao::ramo("telas conferencia da escalacao");
    let criar: unsafe extern "thiscall" fn(*mut usize, usize) -> usize = core::mem::transmute(0x4ae590usize);
    let destruir: unsafe extern "thiscall" fn(*mut usize) = core::mem::transmute(0x4ae630usize);
    let definir: unsafe extern "thiscall" fn(usize, *const u8) -> usize = core::mem::transmute(0x4ae660usize);
    let ligacao: unsafe extern "thiscall" fn(usize, *const u8, u32, usize) -> i32 = core::mem::transmute(0x4ae8a0usize);
    let escolhido: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> usize = core::mem::transmute(0x8968f0usize);
    let convocado: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x7757d0usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let inelegivel: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x8cf5e0usize);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let estado_da_lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let sem_permissao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let fundo: unsafe extern "C" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x41c250usize);
    let menu: unsafe extern "C" fn(u32, u32) = core::mem::transmute(0x75db00usize);
    let cores: unsafe extern "C" fn(usize, u32, *mut u32, *mut u32) = core::mem::transmute(0x53b640usize);
    let nome_da_equipe: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x542100usize);
    let titulo: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x803f30usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let limite_do_banco: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, usize, u32, usize) -> u32 = core::mem::transmute(0x8cf880usize);
    let por_extenso: unsafe extern "C" fn(i32, *mut u8, u32, u32) -> i32 = core::mem::transmute(0x90c520usize);
    let vaga_do_capitao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u8 = core::mem::transmute(0x897590usize);
    let melhor_titular: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x727e20usize);
    let cobrador: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x897990usize);
    let vaga_do_jogador: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8 = core::mem::transmute(0x8972c0usize);
    let nome_do: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(0x53dba0usize);
    let area: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(0x5ed200usize);
    let lista: ListaDoDisplay = core::mem::transmute(0x55fba0usize);
    let celula: CelulaDaLista = core::mem::transmute(0x8040a0usize);
    let g = |a: usize| *(a as *const usize);
    const BUF: usize = 0xde1f64;
    const SEPARADOR: usize = 0x9ed918; // "\n\n"

    let mut obj = [0usize; 2];
    let t = obj.as_mut_ptr();
    let tu = t as usize;
    let mut motivo: u8 = 0;
    criar(t, 0);
    let colunas: [u8; 2] = [3, 1];
    let clube = parametro_da_tela(0);
    let jogo = parametro_da_tela(1);
    let equipe = parametro_da_tela(2);
    let banco = || (*((jogo + 0x4d) as *const u8) & 0xf) as i8;
    let competicao = || core::ptr::read_unaligned((jogo + 0x14) as *const usize);

    let (mut titulares, mut reservas, mut fora) = (0i8, 0i8, 0i8);
    let mut convocados = ([0usize; 20], 0i8);
    let mut emprestados = ([0usize; 20], 0i8);
    let mut indisponiveis = ([0usize; 20], 0i8);
    let mut inelegiveis = ([0usize; 20], 0i8);
    let mut suspensos = ([0usize; 20], 0i8);
    let mut lesionados = ([0usize; 20], 0i8);
    let mut sem_visto = ([0usize; 20], 0i8);
    let por = |g: &mut ([usize; 20], i8), p: usize| {
        g.0[g.1 as usize] = p;
        g.1 += 1;
    };
    for vaga in 0..0x14u8 {
        // ponytail: a original empurra a vaga como dword com os 3 bytes de cima sem valor (a variável é um
        // char); a chamada usa o char, aqui vai estendida com zero
        let p = escolhido(g(0xde1d08), clube, vaga as u32, equipe);
        if p == 0 {
            continue;
        }
        let v = vaga as i8;
        if v < 11 {
            titulares += 1;
        } else if v < banco() + 0xb {
            reservas += 1;
        } else {
            fora += 1;
        }
        if v >= banco() + 0xb {
            continue;
        }
        if convocado(g(0xb74318), p, 0xae2c90, clube, 0, 0) != 0 {
            por(&mut convocados, p);
        }
        if emprestimo(g(0xae1050), p) != 0
            && contrato(g(0xae1050), p) != 0
            && *((contrato(g(0xae1050), p) + 4) as *const i32) == *(clube as *const i32)
            && selecao(clube) == 0
        {
            por(&mut emprestados, p);
        }
        let m = inelegivel(g(0xde1f54), p, jogo);
        if m != 0 && (inelegiveis.1 == 0 || m == motivo) {
            motivo = m;
            por(&mut inelegiveis, p);
        }
        if suspenso(g(0xae28f0), p, competicao(), 1, 0) != 0 {
            por(&mut suspensos, p);
        }
        let tipo_ruim = |l: usize| l != 0 && (0x4d..=0x51).contains(&*(l as *const i8));
        if estado_da_lesao(g(0xb63d28), p) == 1 {
            let l = lesao(g(0xb63d28), p);
            if tipo_ruim(l) {
                if idioma() == 0 {
                    por(&mut indisponiveis, p);
                } else {
                    por(&mut lesionados, p);
                }
            }
        } else if estado_da_lesao(g(0xb63d28), p) == 2 {
            if idioma() != 0 {
                por(&mut lesionados, p);
            } else if tipo_ruim(lesao(g(0xb63d28), p)) {
                por(&mut indisponiveis, p);
            } else {
                por(&mut lesionados, p);
            }
        }
        if sem_permissao(g(0xde1f54), p, clube, 0) != 0
            && *(competicao() as *const i32) != *(0x9cf684 as *const i32)
            && amistoso(g(0xae2c74), competicao()) == 0
        {
            por(&mut sem_visto, p);
        }
    }

    fundo(clube, 0, 0);
    menu(4, 0);
    // 0x53b640 grava u16; o resto da variável da original é lixo da pilha, aqui 0
    let (mut frente, mut fundo_da_cor) = (0u32, 0u32);
    cores(clube, 1, &mut frente, &mut fundo_da_cor);
    nome_da_equipe(clube, equipe, BUF, 0x7d0, 3, 1);
    titulo(GERENTE_DE_TELAS, fundo_da_cor as usize, frente as usize, BUF);

    // ponytail: o motivo de inelegibilidade fora de 1 a 11 deixa a mensagem da original com o que
    // sobrou na pilha; aqui ela começa vazia
    let mut msg = [0u8; 0x2710];
    let m = msg.as_mut_ptr();
    let mu = m as usize;
    let grupo = |g: &([usize; 20], i8), plural: usize, singular: usize, traduzir: bool| {
        titulo_da_escalacao(0xa17b64);
        nomes_da_escalacao(tu, &g.0, g.1, clube);
        let modelo = if g.1 > 1 { plural } else { singular };
        if traduzir {
            texto(mu, modelo, BUF);
        } else {
            formatar(mu, modelo, BUF);
        }
    };
    if convocados.1 != 0 {
        grupo(&convocados, 0xa17a9c, 0xa17a4c, true);
        aviso_da_escalacao(tu, m, false, false);
    } else if emprestados.1 != 0 {
        titulo_da_escalacao(0xa17b64);
        nomes_da_escalacao(tu, &emprestados.0, emprestados.1, clube);
        if emprestados.1 > 1 {
            texto(mu, 0xa17a10, BUF);
        } else {
            let c = core::ptr::read_unaligned((emprestados.0[0] + 0x39) as *const usize);
            let genero = *((c + 0x52) as *const i8) as i32;
            texto(mu, 0xa179c8, genero, genero, BUF, c + 0x38);
        }
        aviso_da_escalacao(tu, m, false, true);
    } else if indisponiveis.1 != 0 {
        grupo(&indisponiveis, 0xa1799c, 0xa17970, false);
        aviso_da_escalacao(tu, m, false, false);
    } else if inelegiveis.1 != 0 {
        titulo_da_escalacao(0xa17b64);
        nomes_da_escalacao(tu, &inelegiveis.0, inelegiveis.1, clube);
        const PLURAL: [usize; 11] = [0xa1793c, 0xa17908, 0xa178c0, 0xa17878, 0xa1782c, 0xa177dc, 0xa1778c, 0xa1773c, 0xa176f4, 0xa17694, 0xa1763c];
        const SINGULAR: [usize; 11] = [0xa17610, 0xa175e0, 0xa175a0, 0xa1755c, 0xa17514, 0xa174cc, 0xa17484, 0xa1743c, 0xa173fc, 0xa173a8, 0xa17358];
        if (1..=11).contains(&motivo) {
            let modelo = if inelegiveis.1 > 1 { PLURAL[motivo as usize - 1] } else { SINGULAR[motivo as usize - 1] };
            texto(mu, modelo, BUF);
        }
        aviso_da_escalacao(tu, m, true, false);
    } else if suspensos.1 != 0 {
        grupo(&suspensos, 0xa17328, 0xa17300, true);
        aviso_da_escalacao(tu, m, false, false);
    } else if lesionados.1 != 0 {
        grupo(&lesionados, 0xa172b8, 0xa17278, true);
        aviso_da_escalacao(tu, m, false, false);
    } else if sem_visto.1 != 0 {
        grupo(&sem_visto, 0xa17224, 0xa171d4, true);
        aviso_da_escalacao(tu, m, false, false);
    } else if limite_do_banco(g(0xde1f54), clube, jogo, m, 0, 2, 0) != 0 {
        titulo_da_escalacao(0xa17b64);
        aviso_da_escalacao(tu, m, true, false);
    } else if titulares < 11 {
        titulo_da_escalacao(0xa17b64);
        let (mut maximo, mut quantos) = ([0u8; 0xc8], [0u8; 0xc8]);
        por_extenso((banco() + 0xb) as i32, maximo.as_mut_ptr(), 0xc8, 1);
        por_extenso(titulares as i32, quantos.as_mut_ptr(), 0xc8, 1);
        match titulares {
            0 => texto(mu, 0xa17150, maximo.as_ptr()),
            1 => texto(mu, 0xa170cc, maximo.as_ptr()),
            _ => texto(mu, 0xa17038, quantos.as_ptr(), maximo.as_ptr()),
        };
        aviso_da_escalacao(tu, m, false, true);
    } else {
        titulo_da_escalacao(0x9909b8);
        *m = 0;
        let acrescentar = |s: usize| {
            if *m != 0 {
                escalacao_juntar(m, SEPARADOR as *const u8);
            }
            escalacao_juntar(m, s as *const u8);
        };
        let mut nome = [0u8; 0xc8];
        let mut outro = [0u8; 0xc8];
        // o capitão: sem vaga de capitão (negativa) ou fora dos 11, o melhor titular (0x727e20)
        let c = vaga_do_capitao(g(0xde1d08), clube, equipe, 1) as i8;
        if c < 0 || vaga_do_capitao(g(0xde1d08), clube, equipe, 1) as i8 >= 0xb {
            let s = melhor_titular(clube, equipe) as i8;
            if s > -1 {
                // ponytail: como na vaga acima, os 3 bytes de cima do dword empurrado não têm valor
                let p = escolhido(g(0xde1d08), clube, s as u8 as u32, equipe);
                if p != 0 {
                    nome_do(p, nome.as_mut_ptr(), 0xc8, 1);
                    texto(BUF, 0xa16fec, nome.as_ptr());
                    acrescentar(BUF);
                    let tipo = if selecao(clube) != 0 { 0x7dc } else { 0x7d7 };
                    ligacao(tu, nome.as_ptr(), tipo, p);
                }
            }
        }
        // os cobradores de bola parada (5 papéis × 2 escolhas) que não estão nos 11, sem repetir; a escolha
        // vai no byte de baixo do dword da equipe (a original reaproveita a variável)
        let mut cobradores = ([0usize; 20], 0i8);
        for papel in 0..5usize {
            for escolha in 0..2usize {
                let p = cobrador(g(0xde1d08), clube, papel, (equipe & !0xff) | escolha, equipe);
                if p != 0 {
                    let s = vaga_do_jogador(g(0xde1d08), clube, p, equipe) as i8;
                    if !(0..=10).contains(&s) && !cobradores.0[..cobradores.1 as usize].contains(&p) {
                        por(&mut cobradores, p);
                    }
                }
            }
        }
        if cobradores.1 > 0 {
            nomes_da_escalacao(tu, &cobradores.0, cobradores.1, clube);
            let mut aviso = [0u8; 0x7c0];
            texto(aviso.as_mut_ptr() as usize, if cobradores.1 > 1 { 0xa16f8c } else { 0xa16f34 }, BUF);
            acrescentar(aviso.as_ptr() as usize);
        }
        if reservas < banco() {
            por_extenso(banco() as i32, nome.as_mut_ptr(), 0xc8, 1);
            por_extenso(reservas as i32, outro.as_mut_ptr(), 0xc8, 1);
            match reservas {
                0 => texto(BUF, 0xa16eec, nome.as_ptr()),
                1 => texto(BUF, 0xa16ea4, nome.as_ptr()),
                _ => texto(BUF, 0xa16e50, outro.as_ptr(), nome.as_ptr()),
            };
            acrescentar(BUF);
        }
        if fora > 0 {
            por_extenso(fora as i32, nome.as_mut_ptr(), 0xc8, 1);
            if fora == 1 {
                texto(BUF, 0xa16e14);
            } else {
                texto(BUF, 0xa16dcc, nome.as_ptr());
            }
            acrescentar(BUF);
        }
        texto(BUF, 0xa16db4);
        acrescentar(BUF);
        definir(tu, m);
        area(0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, cor_em(0xae3184), tu);
        let id = lista(display_do_humano(), 0x64, 0x22b, 0x316, 0x24e, 2, colunas.as_ptr() as usize, 1, 0, 1, 0, usize::MAX);
        let r = texto(BUF, 0x98e8cc) as usize;
        celula(GERENTE_DE_TELAS, id, 0, 0, 0x30, cor_sobre(r, 0xae317e), 0xc, 3, cor_em(0xae3184), BUF, 2, 0);
        texto(BUF, 0x98e8d0);
        celula(GERENTE_DE_TELAS, id, 1, 0, 0x30, cor_em(0xae317e), 0xc, 3, cor_em(0xae3184), BUF, 3, 0);
    }
    destruir(t);
}

/// { 0x72aa80 } (thiscall, ret 4, match_stats.cpp): lê do "match_stats.dat" do save (0x9326e0 no zip
/// `zip`) a quantidade em +4, o índice (12 bytes cada, alocado em [this]), os quatro cursores dos
/// baldes (+8), +0x1c e +0x18. Zip nulo: a original (caixa de erro). Sem o arquivo: erro 0x49e; falha
/// de leitura ou de alocação: erro 0x4a5/0x4ae/0x4b6/0x4be/0x4c6/0x4ce e fecha. Devolve 1 se leu.
unsafe extern "thiscall" fn carregar_estatisticas_da_partida(estat: usize, zip: usize) -> u32 {
    if zip == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(494)(estat, zip);
    }
    crate::comparacao::ramo("telas estatisticas carregar");
    let abrir: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
    let fechar: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let mut h = abrir(zip, 0xa17bd0, 0, 0x16, 4);
    if h == 0 {
        aviso_em(0xa17ba0, 0x49e);
        return 0;
    }
    let falha = |linha: u32, mut h: usize| {
        aviso_em(0xa17ba0, linha);
        fechar(&mut h);
        0
    };
    let n = (estat + 4) as *mut i32;
    if ler(n as usize, 4, 1, h) != 1 {
        return falha(0x4a5, h);
    }
    let tabela = alocar((*n).wrapping_mul(12) as u32);
    *(estat as *mut usize) = tabela;
    if tabela == 0 {
        return falha(0x4ae, h);
    }
    if ler(tabela, 0xc, *n as u32, h) != *n as u32 {
        return falha(0x4b6, h);
    }
    if ler(estat + 8, 4, 4, h) != 4 {
        return falha(0x4be, h);
    }
    if ler(estat + 0x1c, 4, 1, h) != 1 {
        return falha(0x4c6, h);
    }
    if ler(estat + 0x18, 4, 1, h) != 1 {
        return falha(0x4ce, h);
    }
    fechar(&mut h);
    1
}

/// { 0x72adf0 } (thiscall, ret 4, match_stats.cpp): grava no "match_stats.dat" do save (0x9326e0 em
/// escrita) a quantidade (+4), o índice [this], os quatro cursores (+8), +0x1c e +0x18. Zip nulo: a
/// original (caixa de erro); sem abrir, o jogo para (erro 0x4f1); falha de escrita: erro
/// 0x4f8/0x500/0x508/0x510/0x518 e fecha. Devolve 1 se gravou.
unsafe extern "thiscall" fn salvar_estatisticas_da_partida(estat: usize, zip: usize) -> u32 {
    if zip == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(495)(estat, zip);
    }
    crate::comparacao::ramo("telas estatisticas salvar");
    let abrir: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
    let fechar: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    let mut h = abrir(zip, 0xa17bd0, 1, 0x16, 4);
    if h == 0 {
        erro_em(0xa17ba0, 0x4f1);
    }
    let falha = |linha: u32, mut h: usize| {
        aviso_em(0xa17ba0, linha);
        fechar(&mut h);
        0
    };
    let n = (estat + 4) as *const i32;
    if escrever(n as usize, 4, 1, h) != 1 {
        return falha(0x4f8, h);
    }
    if escrever(*(estat as *const usize), 0xc, *n as u32, h) != *n as u32 {
        return falha(0x500, h);
    }
    if escrever(estat + 8, 4, 4, h) != 4 {
        return falha(0x508, h);
    }
    if escrever(estat + 0x1c, 4, 1, h) != 1 {
        return falha(0x510, h);
    }
    if escrever(estat + 0x18, 4, 1, h) != 1 {
        return falha(0x518, h);
    }
    fechar(&mut h);
    1
}

/// A conferência é comparada de verdade pela lista de exibição (o Text local é alocado e liberado
/// dentro de cada execução).
unsafe extern "C" fn comparar_desenhar_conferencia_da_escalacao() {
    let o = orig::<unsafe extern "C" fn()>(493);
    comparar_painel_pela_lista("desenhar_conferencia_da_escalacao", || o(), || desenhar_conferencia_da_escalacao());
}

// ponytail: carregar e salvar mexem no arquivo do save (e carregar aloca o índice): rodar duas vezes
// leria/gravaria duas vezes, então o comparador só chama a original; as cópias se provam no modo Rust.
unsafe extern "thiscall" fn comparar_carregar_estatisticas_da_partida(estat: usize, zip: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(494)(estat, zip)
}

unsafe extern "thiscall" fn comparar_salvar_estatisticas_da_partida(estat: usize, zip: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(495)(estat, zip)
}
