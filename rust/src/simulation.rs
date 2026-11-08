//! Batch simulation (see `docs/VERSAO_SEM_TELA.md` and `engenharia-reversa/simular.py`).
//!
//! `CM_PARAR_EM=AAAA-MM-DD`: on the first draw call after the game's current date
//! (`Data` at 0xae2c90) reaches the target, writes the fingerprint to `ddraw.log` and ends the
//! process. The stop point is fixed in the game's sequence, so two runs with the same
//! seed (`-seed`) and the same routines have to give the same fingerprint.
//!
//! Fingerprint: number of draws since the start + full generator state
//! (table pointer, offset, CRT rand() state). Any difference in
//! behaviour that changes the number or the order of the draws shows up here.
//!
//! [pt] Simulacao em lote (ver `docs/VERSAO_SEM_TELA.md` e `engenharia-reversa/simular.py`).
//!
//! `CM_PARAR_EM=AAAA-MM-DD`: na primeira chamada do sorteio depois de a data atual do jogo
//! (`Data` em 0xae2c90) alcancar o alvo, grava a impressao digital em `ddraw.log` e encerra o
//! processo. O ponto de parada e fixo na sequencia do jogo, entao duas execucoes com a mesma
//! semente (`-seed`) e as mesmas rotinas tem de dar a mesma impressao.
//!
//! Impressao digital: quantidade de sorteios desde o inicio + estado completo do gerador
//! (ponteiro da tabela, deslocamento, estado do rand() da CRT). Qualquer diferenca de
//! comportamento que mude o numero ou a ordem dos sorteios aparece aqui.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

use crate::registro::linha;

const CURRENT_DATE: *const [u16; 2] = 0xae2c90 as *const [u16; 2]; // (day of the year, year)

static DRAWS: AtomicU64 = AtomicU64::new(0);

fn date_from_env(name: &str) -> Option<(u16, u16)> {
    let n: Vec<u16> = name.encode_utf16().chain([0]).collect();
    let mut buf = [0u16; 32];
    let k = unsafe { crate::win::GetEnvironmentVariableW(n.as_ptr(), buf.as_mut_ptr(), buf.len() as u32) } as usize;
    let s = String::from_utf16_lossy(&buf[..k.min(32)]);
    let a = interpret(&s);
    if let Some((year, day)) = a {
        linha(&format!("simulacao: {} = {} (ano {}, dia {})", name, s, year, day));
    }
    a
}

/// Target (year, day of the year from 0) read from CM_PARAR_EM.
///
/// [pt] Alvo (ano, dia do ano a partir de 0) lido de CM_PARAR_EM.
fn target_date() -> Option<(u16, u16)> {
    static TARGET: OnceLock<Option<(u16, u16)>> = OnceLock::new();
    *TARGET.get_or_init(|| date_from_env("CM_PARAR_EM"))
}

/// `CM_RASTRO=AAAA-MM-DD`: every draw of that day goes to `rastro.log` (n, caller, generator
/// state), to find the first call where two runs diverge.
///
/// [pt] `CM_RASTRO=AAAA-MM-DD`: cada sorteio desse dia vai para `rastro.log` (n, quem chamou, estado do
/// gerador), para achar a primeira chamada em que duas execucoes divergem.
pub fn rastro() -> Option<(u16, u16)> {
    static TRACE_DATE: OnceLock<Option<(u16, u16)>> = OnceLock::new();
    *TRACE_DATE.get_or_init(|| date_from_env("CM_RASTRO"))
}

static TRACE_FILE: std::sync::Mutex<Option<std::io::BufWriter<std::fs::File>>> = std::sync::Mutex::new(None);

fn write_trace(text: &str) {
    use std::io::Write;
    let mut g = TRACE_FILE.lock().unwrap_or_else(|e| e.into_inner());
    if g.is_none() {
        let path = std::env::current_exe().map(|p| p.with_file_name("rastro.log")).unwrap_or_else(|_| "rastro.log".into());
        *g = std::fs::File::create(path).ok().map(std::io::BufWriter::new);
    }
    if let Some(f) = g.as_mut() {
        let _ = writeln!(f, "{}", text);
    }
}

fn close_trace() {
    use std::io::Write;
    if let Some(f) = TRACE_FILE.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
        let _ = f.flush();
    }
}

/// "AAAA-MM-DD" -> (year, day of the year from 0), with the game's leap-year rule.
///
/// [pt] "AAAA-MM-DD" -> (ano, dia do ano a partir de 0), com a regra de bissexto do jogo.
pub fn interpret(s: &str) -> Option<(u16, u16)> {
    let mut p = s.trim().split('-').map(|x| x.parse::<u32>().ok());
    let (a, m, d) = (p.next()??, p.next()??, p.next()??);
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) || a > 65535 {
        return None;
    }
    let cumulative = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let extra = (m > 2 && crate::date::is_leap_year(a as u16) == 1) as u32;
    Some((a as u16, (cumulative[m as usize - 1] + extra + d - 1) as u16))
}

/// Called at the start of each draw (Rust version and comparison mode), with the generator state before the call.
/// Thread that made the first draw; draws from other threads make the simulation depend on
/// Windows scheduling and are logged.
///
/// [pt] Chamado no inicio de cada sorteio (versao Rust e modo comparacao), com o estado do gerador antes da chamada.
/// Thread que fez o primeiro sorteio; sorteios de outras threads tornam a simulacao dependente
/// do agendamento do Windows e sao registrados.
static MAIN_THREAD: AtomicU64 = AtomicU64::new(0);
static OTHER_THREADS: AtomicU64 = AtomicU64::new(0);

unsafe fn check_thread(origin: &str) {
    let t = crate::win::GetCurrentThreadId() as u64;
    let p = match MAIN_THREAD.compare_exchange(0, t, Ordering::Relaxed, Ordering::Relaxed) {
        Ok(_) => t,
        Err(p) => p,
    };
    if t != p {
        let n = OTHER_THREADS.fetch_add(1, Ordering::Relaxed);
        if n < 20 || (n + 1).is_power_of_two() {
            let [d, a] = *CURRENT_DATE;
            linha(&format!("simulacao: {} na thread {} (principal {}), data {}-{:03}, {} vezes", origin, t, p, a, d, n + 1));
        }
    }
}

/// FNV-1a of the people table (70-byte records at [0xae23c4], count at [0xae236c]).
///
/// [pt] FNV-1a da tabela de pessoas (registros de 70 bytes em [0xae23c4], quantidade em [0xae236c]).
unsafe fn hash_people() -> u64 {
    let (base, count) = (*(0xae23c4 as *const usize), *(0xae236c as *const i32));
    if base == 0 || count <= 0 {
        return 0;
    }
    core::slice::from_raw_parts(base as *const u8, count as usize * 70)
        .iter()
        .fold(0xcbf29ce484222325u64, |h, &b| (h ^ b as u64).wrapping_mul(0x100000001b3))
}

/// Daily memory map in jogo\memoria.log (with the trace on): one line per day with the hash
/// of each 4 KB block of .data and 64 KB block of the heap. Comparing runs, it separates the noise (blocks
/// that vary even between identical runs) from the first data that really diverges.
///
/// [pt] Mapa diario da memoria em jogo\memoria.log (com o rastro ligado): uma linha por dia com o hash
/// de cada bloco de 4 KB da .data e de 64 KB da heap. Comparando execucoes, separa o ruido (blocos
/// que variam ate entre execucoes iguais) do primeiro dado que diverge de verdade.
unsafe fn memory_map() {
    use std::io::Write;
    static FILE: std::sync::Mutex<Option<std::io::BufWriter<std::fs::File>>> = std::sync::Mutex::new(None);
    let mut g = FILE.lock().unwrap_or_else(|e| e.into_inner());
    if g.is_none() {
        let path = std::env::current_exe().map(|p| p.with_file_name("memoria.log")).unwrap_or_default();
        *g = std::fs::File::create(path).ok().map(std::io::BufWriter::new);
    }
    let Some(f) = g.as_mut() else { return };
    let [d, a] = *CURRENT_DATE;
    let mut line = format!("{}-{:03}", a, d);
    let mut b = 0x987000;
    while b < 0xde4000 {
        line.push_str(&format!(" d{:x}:{:x}", b, hash_range(b, b + 0x1000) as u32));
        b += 0x1000;
    }
    let (start, top) = crate::allocator::game_region();
    let mut b = start;
    while start != 0 && b + 0x10000 <= top {
        line.push_str(&format!(" h{:x}:{:x}", b, hash_range(b, b + 0x10000) as u32));
        b += 0x10000;
    }
    let _ = writeln!(f, "{}", line);
    let _ = f.flush();
}

/// Fast hash of a memory range (8-byte words).
///
/// [pt] Hash rapido de um trecho de memoria (palavras de 8 bytes).
unsafe fn hash_range(start: usize, end: usize) -> u64 {
    core::slice::from_raw_parts(start as *const u64, (end - start) / 8)
        .iter()
        .fold(0xcbf29ce484222325u64, |h, &w| (h ^ w).wrapping_mul(0x100000001b3).rotate_left(17))
}

/// Match fingerprint: the hash of each match engine when freed (the end-of-match state
/// that does not reach the draw, like the players' fatigue), chained in match order.
///
/// [pt] Impressao das partidas: o hash de cada motor de partida ao ser liberado (o estado de fim de jogo
/// que nao chega ao sorteio, como o desgaste dos jogadores), encadeado na ordem das partidas.
static MATCHES: AtomicU64 = AtomicU64::new(0);
static MATCH_COUNT: AtomicU64 = AtomicU64::new(0);

pub unsafe fn match_finished(engine: usize) {
    let h = hash_range(engine, engine + (0xf63b & !7));
    let v = MATCHES.load(Ordering::Relaxed);
    MATCHES.store(v.rotate_left(5) ^ h, Ordering::Relaxed);
    MATCH_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// The game's memory, chained day by day: the database blocks. The whole heap mixes in the
/// interface (image buffers, screens), which the game draws at moments that depend on speed and
/// varies between Rust and the original without changing the game (13/10/2026). In go the blocks pointed to by the
/// database pointer table (0xae2300..0xae2400: people, clubs, ...), the table at [[0xb67a44]]
/// and the GlobalAlloc blocks.
///
/// [pt] A memoria do jogo, encadeada dia a dia: os blocos do banco de dados. A heap inteira mistura a
/// interface (buffers de imagem, telas), que o jogo desenha em momentos que dependem da velocidade e
/// varia entre o Rust e o original sem mudar o jogo (13/10/2026). Entram os blocos apontados pela
/// tabela de ponteiros do banco (0xae2300..0xae2400: pessoas, clubes, ...), a tabela em [[0xb67a44]]
/// e os blocos do GlobalAlloc.
static MEMORY: AtomicU64 = AtomicU64::new(0);

pub unsafe fn game_blocks() -> Vec<(usize, usize)> {
    let mut v = crate::allocator::database_blocks();
    let mut pointers: Vec<usize> = (0xae2300..0xae2400).step_by(4).map(|g| *(g as *const usize)).collect();
    if *(0xb67a44 as *const usize) != 0 {
        pointers.push(**(0xb67a44 as *const *const usize));
    }
    for p in pointers {
        if let Some(n) = crate::allocator::bloco(p) {
            if !v.iter().any(|b| b.0 == p) {
                v.push((p, n));
            }
        }
    }
    v
}

unsafe fn hash_game_blocks() -> u64 {
    game_blocks().iter().fold(0u64, |h, &(p, n)| h.rotate_left(7) ^ hash_range(p, p + (n & !7)))
}

/// The hash chained so far, with the memory of this moment.
///
/// [pt] O hash encadeado ate agora, com a memoria deste momento.
unsafe fn memory_of_day() -> u64 {
    let v = MEMORY.load(Ordering::Relaxed).rotate_left(5) ^ hash_game_blocks();
    MEMORY.store(v, Ordering::Relaxed);
    v
}

/// With the trace on, the daily log gets the hash of all of the game's memory (fixed heap and the exe's .data,
/// 0x987000..0xde463c): finds the first day on which any data diverges, before it reaches the draw.
///
/// [pt] Com o rastro ligado, o diario ganha o hash de toda a memoria do jogo (heap fixa e .data do exe,
/// 0x987000..0xde463c): acha o primeiro dia em que qualquer dado diverge, antes de chegar ao sorteio.
unsafe fn hash_whole_game() -> String {
    if rastro().is_none() {
        return String::new();
    }
    memory_map();
    let (start, top) = crate::allocator::game_region();
    format!(" heap={:016x} dados={:016x}", if start == 0 { 0 } else { hash_range(start, top & !7) }, hash_range(0x987000, 0xde4638))
}

pub unsafe fn sorteio(requested: i32, pointer: u32, offset: i32, rand: u32) {
    let n = DRAWS.fetch_add(1, Ordering::Relaxed);
    let Some((year, day)) = target_date() else { return };
    check_thread("aleatorio");
    let [d, a] = *CURRENT_DATE;
    if let Some((ra, rd)) = rastro() {
        // First call of the trace day: memory snapshot (whole .data and hash per heap block).
        static SNAPSHOT_TAKEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if (a, d) == (ra, rd) && !SNAPSHOT_TAKEN.swap(true, Ordering::Relaxed) {
            let data_path = std::env::current_exe().map(|p| p.with_file_name("dados.bin")).unwrap_or_default();
            let _ = std::fs::write(data_path, core::slice::from_raw_parts(0x987000 as *const u8, 0xde4638 - 0x987000));
            let (start, top) = crate::allocator::game_region();
            if start != 0 {
                let heap = std::env::current_exe().map(|p| p.with_file_name("heap.bin")).unwrap_or_default();
                let _ = std::fs::write(heap, core::slice::from_raw_parts(start as *const u8, top - start));
            }
        }
        if (a, d) == (ra, rd) {
            write_trace(&format!("{} n={} chamador={:#x} ponteiro={:#x} deslocamento={:#x} rand={:#x}", n, requested, { crate::detours::RETURN_TO }, pointer, offset, rand));
        } else if (a, d) > (ra, rd) {
            close_trace();
        }
    }
    // Daily log: one line per game day, to find where two runs start to diverge.
    static LAST_DAY: AtomicU64 = AtomicU64::new(0);
    let today = (a as u64) << 16 | d as u64;
    if LAST_DAY.swap(today, Ordering::Relaxed) != today {
        let mem = memory_of_day();
        linha(&format!("dia {}-{:03} sorteios={} ponteiro={:#x} deslocamento={:#x} rand={:#x} pessoas={:016x} memoria={:016x} relogios={:?}{}", a, d, n, pointer, offset, rand, hash_people(), mem,
            CLOCKS.iter().map(|r| r.load(Ordering::Relaxed)).collect::<Vec<_>>(), hash_whole_game()));
    }
    if (a, d) < (year, day) || a < 1900 {
        return;
    }
    linha(&format!(
        "SIMULACAO parou em {}-{:03} sorteios={} ponteiro={:#x} deslocamento={:#x} rand={:#x}",
        a, d, n, pointer, offset, rand
    ));
    linha(&format!("partidas: {} motores, impressao {:016x}", MATCH_COUNT.load(Ordering::Relaxed), MATCHES.load(Ordering::Relaxed)));
    {
        let (start, top) = crate::allocator::game_region();
        // diagnosis: with jogo\heap_fim.txt (the output path inside), dumps the game's heap at the end
        // (a file, and not an environment variable, which would shift the heap and change the career)
        let marker = std::env::current_exe().map(|p| p.with_file_name("heap_fim.txt")).unwrap_or_default();
        if let Ok(file) = std::fs::read_to_string(marker).map(|t| t.trim().to_string()) {
            if start != 0 {
                let _ = std::fs::write(&file, core::slice::from_raw_parts(start as *const u8, top - start));
                linha(&format!("memoria: heap em {:#x}", start));
            }
        }
        let _ = start;
        let blocks = game_blocks();
        linha(&format!("memoria: banco {:016x} ({} blocos, {} KB)", memory_of_day(), blocks.len(), blocks.iter().map(|b| b.1).sum::<usize>() >> 10));
    }
    let (dll, live) = crate::allocator::dll_peak();
    linha(&format!("alocador: pico {} MB; regiao da DLL: pico {} KB, ate {} KB vivos", crate::allocator::pico() >> 20, dll >> 10, live >> 10));
    crate::detours::stub_summary();
    dump_coverage();
    close_trace();
    // Ends without running the game's finalizations (no saving of preferences).
    crate::win::TerminateProcess(crate::win::GetCurrentProcess(), 0);
}

// ---------- fixed clock ----------

/// Instant returned by the game's time() during the simulation (9/9/2001 01:46:40 UTC).
///
/// [pt] Instante devolvido pelo time() do jogo durante a simulacao (9/9/2001 01:46:40 UTC).
const FIXED_CLOCK: u32 = 1_000_000_000;

// 0x946ebb: push ebp; mov ebp, esp; sub esp, 0xcc
const PROLOGUE_TIME: crate::detours::Prologue =
    crate::detours::Prologue { address: 0x946ebb, bytes: &[0x55, 0x8b, 0xec, 0x81, 0xec, 0xcc, 0x00, 0x00, 0x00] };

/// CRT time() { 0x946ebb } in the simulation: always the same instant. The game reseeds rand()
/// with the clock (manager_manager.cpp, 0x696d4b), which would make the simulation irreproducible.
///
/// [pt] time() da CRT { 0x946ebb } na simulacao: sempre o mesmo instante. O jogo ressemeia o rand()
/// com o relogio (manager_manager.cpp, 0x696d4b), o que tornaria a simulacao irreproduzivel.
unsafe extern "C" fn fixed_time(out: *mut u32) -> u32 {
    if !out.is_null() {
        *out = FIXED_CLOCK;
    }
    FIXED_CLOCK
}

// 0x946a04: mov eax, dword ptr [0xad6c30]
const PROLOGUE_RAND: crate::detours::Prologue =
    crate::detours::Prologue { address: 0x946a04, bytes: &[0xa1, 0x30, 0x6c, 0xad, 0x00] };
static mut RAND_ORIGINAL: usize = 0;

/// CRT rand() observed in the simulation: only checks which thread it comes from and calls the original.
///
/// [pt] rand() da CRT observado na simulacao: so confere de que thread vem e chama o original.
unsafe extern "C" fn observed_rand() -> i32 {
    check_thread("rand");
    let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(RAND_ORIGINAL);
    f()
}

// 0x947299: push ebp; mov ebp, esp; push ecx; push ecx
const PROLOGUE_CLOCK: crate::detours::Prologue =
    crate::detours::Prologue { address: 0x947299, bytes: &[0x55, 0x8b, 0xec, 0x51, 0x51] };
/// Game functions that call clock() (via 0x90c510; return addresses checked with refs.py).
///
/// [pt] Funcoes do jogo que chamam clock() (via 0x90c510; enderecos de retorno conferidos com refs.py).
const CLOCK_CALLERS: [u32; 13] = [
    0x497050, 0x569610, 0x5cccd0, 0x68d520, 0x6b55c0, 0x6d7b00, 0x6d8c70, 0x71b1b0, 0x724b53, 0x725860,
    0x75db00, 0x7615d0, 0x81b890,
];
static CLOCKS: [AtomicU64; 14] = [const { AtomicU64::new(0) }; 14];

/// CRT clock() { 0x947299 } in the simulation: one clock per calling routine, which advances 1 ms on
/// each of its queries. A single clock (or the real one) makes the waits of match day (match_day.cpp,
/// "come back when clock() passes X") depend on how many times the menu bar and the screens
/// queried the clock, which varies with the Windows messages.
///
/// [pt] clock() da CRT { 0x947299 } na simulacao: um relogio por rotina chamadora, que avanca 1 ms a
/// cada consulta dela. Um relogio unico (ou o real) faz as esperas do dia de jogo (match_day.cpp,
/// "volta quando clock() passar de X") dependerem de quantas vezes a barra de menus e as telas
/// consultaram o relogio, o que varia com as mensagens do Windows.
unsafe extern "C" fn fixed_clock() -> i32 {
    let r = crate::detours::RETURN_TO;
    let i = CLOCK_CALLERS.iter().rposition(|&f| f <= r).unwrap_or(13);
    let v = CLOCKS[i].fetch_add(1, Ordering::Relaxed) as i32;
    let [d, a] = *CURRENT_DATE;
    if rastro() == Some((a, d)) {
        write_trace(&format!("clock de {:#x} (rotina {:#x}) = {}", r, CLOCK_CALLERS.get(i).copied().unwrap_or(0), v));
    }
    v
}

// 0x7a16d0 (total RAM, MB) and 0x7a1710 (free memory, MB): sub esp, 0x20; mov eax, 0x20
const PROLOGUE_RAM_TOTAL: crate::detours::Prologue =
    crate::detours::Prologue { address: 0x7a16d0, bytes: &[0x83, 0xec, 0x20, 0xb8, 0x20, 0x00, 0x00, 0x00] };
const PROLOGUE_RAM_FREE: crate::detours::Prologue =
    crate::detours::Prologue { address: 0x7a1710, bytes: &[0x83, 0xec, 0x20, 0xb8, 0x20, 0x00, 0x00, 0x00] };

// 0x7a1820 (os.cpp, free disk space in MB, saturated at 4096; 136 calls): sub esp, 0x2bc
const PROLOGUE_DISK: crate::detours::Prologue =
    crate::detours::Prologue { address: 0x7a1820, bytes: &[0x81, 0xec, 0xbc, 0x02, 0x00, 0x00] };

unsafe extern "C" fn fixed_disk(_drive: usize) -> i32 {
    4096
}

/// Drives seen by the 33 CD checks: only C:\ (pen drives and the like change nothing).
///
/// [pt] Unidades vistas pelas 33 checagens de CD: so C:\ (pen drives e afins nao mudam nada).
unsafe extern "system" fn fixed_drives(size: u32, buf: *mut u8) -> u32 {
    let s = b"C:\\\0\0";
    if (size as usize) < s.len() {
        return s.len() as u32;
    }
    core::ptr::copy_nonoverlapping(s.as_ptr(), buf, s.len());
    (s.len() - 1) as u32
}

/// Memory seen by the game in the simulation: fixed (the GlobalMemoryStatus ceiling), so it does not depend on the machine.
///
/// [pt] Memoria vista pelo jogo na simulacao: fixa (o teto do GlobalMemoryStatus), para nao depender da maquina.
unsafe extern "C" fn fixed_memory() -> i32 {
    2047
}

// ---------- qsort in the trace ----------

const PROLOGUE_QSORT: crate::detours::Prologue =
    crate::detours::Prologue { address: 0x945333, bytes: &[0x55, 0x8b, 0xec, 0x81, 0xec, 0xf8, 0x00, 0x00, 0x00] };
static mut QSORT_ORIGINAL: usize = 0;

/// Hash of the array with pointers into the people table replaced by the index (the table's address varies).
///
/// [pt] Hash do vetor com ponteiros para a tabela de pessoas trocados pelo indice (o endereco da tabela varia).
unsafe fn hash_array(base: *const u8, bytes: usize) -> u64 {
    let (people, count) = (*(0xae23c4 as *const u32), *(0xae236c as *const i32) as u32);
    let end = people.wrapping_add(count.wrapping_mul(70));
    let mut h = 0xcbf29ce484222325u64;
    for i in 0..bytes / 4 {
        let mut v = *(base as *const u32).add(i);
        if v >= people && v < end {
            v = 0x8000_0000 | (v - people);
        }
        h = (h ^ v as u64).wrapping_mul(0x100000001b3);
    }
    h
}

unsafe extern "C" fn traced_qsort(base: *mut u8, n: usize, width: usize, cmp: usize) {
    let f: unsafe extern "C" fn(*mut u8, usize, usize, usize) = core::mem::transmute(QSORT_ORIGINAL);
    let [d, a] = *CURRENT_DATE;
    let on_day = rastro() == Some((a, d));
    let before = if on_day { hash_array(base, n * width) } else { 0 };
    f(base, n, width, cmp);
    if on_day {
        write_trace(&format!("qsort cmp={:#x} n={} largura={} entrada={:016x} saida={:016x}", cmp, n, width, before, hash_array(base, n * width)));
    }
}

// ---------- coverage ----------

/// (address, file, counter) of each counted function (`CM_COBERTURA=1`, list in cobertura.txt).
///
/// [pt] (endereco, arquivo, contador) de cada funcao contada (`CM_COBERTURA=1`, lista em cobertura.txt).
static COVERAGE: std::sync::Mutex<Vec<(usize, String, &'static mut u32)>> = std::sync::Mutex::new(Vec::new());

/// Reads jogo\cobertura.txt (generated by engenharia-reversa\cobertura.py) and puts a counter on each function.
///
/// [pt] Le jogo\cobertura.txt (gerado por engenharia-reversa\cobertura.py) e poe um contador em cada funcao.
unsafe fn install_coverage() {
    let path = std::env::current_exe().map(|p| p.with_file_name("cobertura.txt")).unwrap_or_default();
    let Ok(text) = std::fs::read_to_string(&path) else {
        linha("cobertura: AVISO cobertura.txt nao encontrado (rode cobertura.py)");
        return;
    };
    let mut list = COVERAGE.lock().unwrap_or_else(|e| e.into_inner());
    for l in text.lines() {
        let mut c = l.split_whitespace();
        let (Some(end), Some(hex), file) = (c.next(), c.next(), c.next().unwrap_or("")) else { continue };
        let Ok(address) = usize::from_str_radix(end, 16) else { continue };
        let bytes: Vec<u8> = (0..hex.len() / 2).filter_map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).ok()).collect();
        let p = crate::detours::Prologue { address: address, bytes: Box::leak(bytes.into_boxed_slice()) };
        let counter: &'static mut u32 = Box::leak(Box::new(0));
        if crate::detours::count_calls(&format!("cobertura {:#x}", address), &p, counter) {
            list.push((address, file.to_string(), counter));
        }
    }
    linha(&format!("cobertura: {} funcoes com contador", list.len()));
}

fn dump_coverage() {
    for (e, file, n) in COVERAGE.lock().unwrap_or_else(|e| e.into_inner()).iter() {
        linha(&format!("cobertura {:x} {} {}", e, file, n));
    }
}

// ---------- messages ----------

/// The game's PeekMessageA in the simulation: really empties the queue (the window keeps answering
/// Windows, including messages sent by other programs) and always answers "none".
/// Runs on the private stack (installed through the IAT behind `detours::isolar`): the Windows code
/// that handles the messages left garbage on the game's stack at moments that depend on the clock, and the
/// game reads uninitialized variables (26/12/2001, training: the career changed in 1 of every 4-6
/// runs with the generator identical up to the point of divergence).
///
/// [pt] PeekMessageA do jogo na simulacao: esvazia a fila de verdade (a janela segue respondendo ao
/// Windows, inclusive a mensagens enviadas por outros programas) e responde sempre "nenhuma".
/// Roda na pilha particular (instalada pela IAT atras de `detours::isolar`): o codigo do Windows
/// que trata as mensagens deixava lixo na pilha do jogo em momentos que dependem do relogio, e o
/// jogo le variaveis nao inicializadas (26/12/2001, treino: carreira mudava em 1 de cada 4-6
/// execucoes com o gerador identico ate o ponto da divergencia).
unsafe extern "system" fn isolated_peek(_msg: *mut [u32; 7], _hwnd: usize, _min: u32, _max: u32, _remove: u32) -> i32 {
    let mut m = [0u32; 7];
    while crate::win::PeekMessageA(&mut m, core::ptr::null_mut(), 0, 0, 1) != 0 {
        crate::win::TranslateMessage(&m);
        crate::win::DispatchMessageA(&m);
    }
    0
}

/// Every Windows function called by the game runs on the private stack (`detours::isolar` on each
/// IAT entry, including the ones already swapped above). The Windows code left on the game's stack
/// values that change on each run (process heap addresses, handles) and the game reads uninitialized
/// variables: the career with the engine on changed in half the runs (26/12/2001,
/// training). This way the game's stack only sees what the game itself writes. Left out are the exception
/// and shutdown functions, which need the real stack.
///
/// [pt] Toda funcao do Windows chamada pelo jogo roda na pilha particular (`detours::isolar` em cada
/// entrada da IAT, inclusive as ja trocadas acima). O codigo do Windows deixava na pilha do jogo
/// valores que mudam a cada execucao (enderecos da heap do processo, alcas) e o jogo le variaveis
/// nao inicializadas: a carreira com o motor ligado mudava em metade das execucoes (26/12/2001,
/// treino). Assim a pilha do jogo so ve o que o proprio jogo escreve. Ficam de fora as funcoes de
/// excecao e de encerramento, que precisam da pilha real.
unsafe fn isolate_imports() {
    const EXCLUDED: [&str; 6] = ["RaiseException", "RtlUnwind", "ExitProcess", "TerminateProcess", "UnhandledExceptionFilter", "SetUnhandledExceptionFilter"];
    let n = crate::iat_hooks::for_each_import(|_dll, name, current| {
        if name.is_some_and(|n| EXCLUDED.contains(&n)) {
            None
        } else {
            Some(crate::detours::isolate_import(current))
        }
    });
    linha(&format!("simulacao: {} funcoes do Windows isoladas na pilha particular", n));
}

// ---------- exceptions ----------

/// Logs the process's exceptions to ddraw.log (code, address, registers, stack) and lets
/// handling go on as normal. In the simulation the game runs hidden and Windows does not always log the
/// crash; this way the address is always at hand. Only the first 8 (handled exceptions also pass here).
///
/// [pt] Registra no ddraw.log as excecoes do processo (codigo, endereco, registradores, pilha) e deixa
/// o tratamento seguir normal. Na simulacao o jogo roda oculto e o Windows nem sempre registra a
/// queda; assim o endereco fica sempre a mao. So as 8 primeiras (excecoes tratadas tambem passam aqui).
pub unsafe extern "system" fn record_exception(info: *const [usize; 2]) -> i32 {
    static SEEN: AtomicU64 = AtomicU64::new(0);
    if SEEN.fetch_add(1, Ordering::Relaxed) < 8 {
        let (reg, ctx) = ((*info)[0], (*info)[1]);
        let code = *(reg as *const u32);
        let address = *((reg + 12) as *const usize);
        // CONTEXT x86: edi 0x9c, esi 0xa0, ebx 0xa4, edx 0xa8, ecx 0xac, eax 0xb0, ebp 0xb4, eip 0xb8, esp 0xc4
        let r = |o: usize| *((ctx + o) as *const u32);
        let esp = r(0xc4) as usize;
        let stack: Vec<String> = (0..12).map(|i| format!("{:x}", *((esp + 4 * i) as *const u32))).collect();
        extern "C" {
            static __ImageBase: u8;
        }
        let dll = core::ptr::addr_of!(__ImageBase) as usize;
        let location = if address >= dll && address - dll < 0x0100_0000 { format!(" (ddraw.dll+{:#x})", address - dll) } else { String::new() };
        linha(&format!(
            "EXCECAO {:#x} em {:#x}{} (data {:?}) eax={:x} ebx={:x} ecx={:x} edx={:x} esi={:x} edi={:x} ebp={:x} esp={:x} pilha: {}",
            code, address, location, *CURRENT_DATE, r(0xb0), r(0xa4), r(0xac), r(0xa8), r(0xa0), r(0x9c), r(0xb4), esp, stack.join(" ")
        ));
    }
    0 // EXCEPTION_CONTINUE_SEARCH
}

const PROLOGUE_RESTORE_HUMANS: crate::detours::Prologue =
    crate::detours::Prologue { address: 0x8387e0, bytes: &[0x51, 0x53, 0x55, 0x33, 0xed] };
static mut RESTORE_ORIGINAL: usize = 0;

/// CM_FERIAS=1 (save proof, load step): the game does not allow saving with the manager on holiday, and a save
/// loaded with -load waits for Continue. On entry to 0x8387e0 (Setup.cpp, the humans restored; only
/// -load and the network go through it, after load_game_data) each active human (+0x5f = 1, among the last 16
/// people) goes on holiday like auto_setup's ON_HOLIDAY=YES (0x827cc9: set_on_holiday 0x5fd2f0 with 1), and the
/// holiday stamp (+0x9a65, setter 0x5fdd30) goes back to 0, as after the reset 0x5fb810 of the automatic
/// career: with the save's real-clock stamp, the simulation's fixed clock would never pass it + 2000 and the
/// sidebar (0x7615d0) would not move on its own. Same in both modes (it is not a ported routine).
///
/// [pt] CM_FERIAS=1 (prova do save, etapa de carregar): o jogo nao deixa gravar com o tecnico de ferias, e um save
/// carregado com -load espera o Continuar. Na entrada da 0x8387e0 (Setup.cpp, os humanos restaurados; so o
/// -load e a rede passam por ela, depois do load_game_data) cada humano ativo (+0x5f = 1, entre as 16 ultimas
/// pessoas) sai de ferias como o ON_HOLIDAY=YES do auto_setup (0x827cc9: set_on_holiday 0x5fd2f0 com 1), e o
/// carimbo das ferias (+0x9a65, setter 0x5fdd30) volta a 0, como depois do reset 0x5fb810 da carreira
/// automatica: com o carimbo do relogio real do save, o clock fixo da simulacao nunca passaria dele + 2000 e a
/// barra lateral (0x7615d0) nao andaria sozinha. Igual nos dois modos (nao e rotina portada).
unsafe extern "C" fn restore_humans_on_holiday() -> i32 {
    let hm = *(0xb63c98 as *const usize);
    if hm != 0 {
        let set_on_holiday: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x5fd2f0usize);
        let set_holiday_stamp: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x5fdd30usize);
        let (base, count) = (*(0xae23bc as *const usize), *(0xae2364 as *const i32) as usize);
        for i in 0..16usize.min(count) {
            let t = base + (count - 1 - i) * 0x6e;
            if *((t + 0x5f) as *const u8) == 1 {
                set_on_holiday(hm, 1, t);
                set_holiday_stamp(hm, 0, t);
                crate::registro::linha(&format!("CM_FERIAS: humano {:#x} de ferias depois do -load", t));
            }
        }
    }
    let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(RESTORE_ORIGINAL);
    f()
}

/// Called when the DLL loads: only acts in simulation mode (CM_PARAR_EM set).
///
/// [pt] Chamado no carregamento da DLL: so age em modo simulacao (CM_PARAR_EM definido).
pub unsafe fn install() {
    if target_date().is_some() {
        // Every routine detoured from here on runs on the private stack and records its caller.
        crate::detours::ISOLATE.store(true, Ordering::Relaxed);
        crate::win::AddVectoredExceptionHandler(1, record_exception);
        crate::iat_hooks::replace_import("USER32.dll", "PeekMessageA", isolated_peek as *const () as usize);
        // garbage detector (simular.py --detector): jogo\veneno.txt with the poison in hex. A file, and not an environment
        // variable (it would shift the game's heap); read before the DLL region, and the line has the same length with and
        // without it, so the DLL's memory stays equal to that of the normal careers.
        let file = std::env::current_exe().map(|p| p.with_file_name("veneno.txt")).unwrap_or_default();
        if let Some(v) = std::fs::read_to_string(file).ok().and_then(|t| u32::from_str_radix(t.trim(), 16).ok()) {
            crate::detours::POISON.store(v, Ordering::Relaxed);
        }
        linha(&format!("simulacao: veneno {:08x}", crate::detours::POISON.load(Ordering::Relaxed)));
        crate::allocator::instalar();
        if crate::win::ambiente("CM_FERIAS") {
            if let Some(t) = crate::detours::hook("ferias depois do -load (CM_FERIAS)", &PROLOGUE_RESTORE_HUMANS,
                                                     restore_humans_on_holiday as *const () as usize) {
                RESTORE_ORIGINAL = t;
            }
        }
        if crate::win::ambiente("CM_COBERTURA") {
            install_coverage();
        }
        if rastro().is_some() {
            if let Some(t) = crate::detours::hook("qsort (rastro)", &PROLOGUE_QSORT, traced_qsort as *const () as usize) {
                QSORT_ORIGINAL = t;
            }
        }
        crate::detours::hook("time (relogio fixo da simulacao)", &PROLOGUE_TIME, fixed_time as *const () as usize);
        crate::detours::hook("clock (relogio da simulacao)", &PROLOGUE_CLOCK, fixed_clock as *const () as usize);
        crate::detours::hook("disco livre (fixo na simulacao)", &PROLOGUE_DISK, fixed_disk as *const () as usize);
        crate::iat_hooks::replace_import("KERNEL32.dll", "GetLogicalDriveStringsA", fixed_drives as *const () as usize);
        crate::detours::hook("RAM total (fixa na simulacao)", &PROLOGUE_RAM_TOTAL, fixed_memory as *const () as usize);
        crate::detours::hook("RAM livre (fixa na simulacao)", &PROLOGUE_RAM_FREE, fixed_memory as *const () as usize);
        if let Some(t) = crate::detours::hook("rand (observado na simulacao)", &PROLOGUE_RAND, observed_rand as *const () as usize) {
            RAND_ORIGINAL = t;
        }
        isolate_imports();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates() {
        assert_eq!(interpret("2001-01-01"), Some((2001, 0)));
        assert_eq!(interpret("2001-08-09"), Some((2001, 220))); // the date seen in the game's memory
        assert_eq!(interpret("2004-03-01"), Some((2004, 60)));
        assert_eq!(interpret("2001-13-01"), None);
        assert_eq!(interpret("lixo"), None);
    }
}
