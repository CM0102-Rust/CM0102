//! Reproducible heap for the batch simulation (only with CM_PARAR_EM).
//!
//! Windows changes the heap's address on every run and, with the low-fragmentation allocator, the
//! order of small blocks. The game breaks some ties by pointer order (on day
//! 2001-093 the line-up changed with an identical generator), so the same seed did not give the same
//! career. Here the exe's Heap* functions (IAT) switch to a fixed-address region with a
//! deterministic allocator: same sequence of requests => same addresses.
//!
//! The DLL memory the game sees (DirectDraw objects, surfaces and their pixels) goes to
//! a second fixed region (`visivel`): the game keeps pointers to it and the address in the process
//! heap varied (day 2001-271). The rest of the DLL goes to a third fixed region, its own (`DLL_REGION`):
//! far from the game's, so that comparisons, scoreboards and logging (which change with the mode) do not shift what the
//! game sees, and at a fixed address, because its pointers end up on the private stack (01/11/2026).
//!
//! Blocks: 16-byte header {capacity, requested size} + data aligned to 16.
//! Up to 4 KB: classes in steps of 16 bytes; above: classes per page (up to 256 MB). Each class has
//! a free list linked inside the blocks themselves (the last freed comes out first; nothing
//! is allocated for bookkeeping, which lets it serve as the global allocator); with no free block, the top advances.
//! Neighbouring free blocks are only merged when the region runs out (`alloc_merging`).
//!
//! [pt] Heap reproduzivel para a simulacao em lote (so com CM_PARAR_EM).
//!
//! O Windows muda a cada execucao o endereco da heap e, com o alocador de baixa fragmentacao, a
//! ordem dos blocos pequenos. O jogo desempata algumas decisoes pela ordem dos ponteiros (no dia
//! 2001-093 a escalacao mudava com o gerador identico), entao a mesma semente nao dava a mesma
//! carreira. Aqui as funcoes Heap* do exe (IAT) passam a usar uma regiao de endereco fixo com um
//! alocador deterministico: mesma sequencia de pedidos => mesmos enderecos.
//!
//! A memoria da DLL que o jogo enxerga (objetos DirectDraw, superficies e seus pixels) vai para
//! uma segunda regiao fixa (`visivel`): o jogo guarda ponteiros para ela e o endereco na heap do
//! processo variava (dia 2001-271). O resto da DLL vai para uma terceira regiao fixa, so dela (`REGIAO_DLL`):
//! longe da do jogo, para que comparacoes, placares e registro (que mudam com o modo) nao desloquem o que o
//! jogo ve, e em endereco fixo, porque os ponteiros dela ficam na pilha particular (01/11/2026).
//!
//! Blocos: cabecalho de 16 bytes {capacidade, tamanho pedido} + dados alinhados em 16.
//! Ate 4 KB: classes de 16 em 16 bytes; acima: classes por pagina (ate 256 MB). Cada classe tem
//! uma lista de livres encadeada dentro dos proprios blocos (o ultimo liberado sai primeiro; nada
//! e alocado para controle, o que permite servir de alocador global); sem livre, avanca o topo.
//! Os livres vizinhos so se juntam quando a regiao esgota (`alocar_juntando`).

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::Mutex;

use crate::registro::linha;

const HEADER: usize = 16;
const SMALL: usize = 4096;
const REGION: usize = 0x4000_0000; // 1 GB reserved and committed
const BASES: [usize; 1] = [0x1800_0000];

const HEAP_ZERO_MEMORY: u32 = 0x08;
const HEAP_REALLOC_IN_PLACE_ONLY: u32 = 0x10;

pub struct Allocator {
    start: usize,
    top: usize,
    end: usize,
    free: [usize; CLASSES], // head of each class's free list (0 = empty)
    peak: usize,
    /// (free, top) of the last `merge_free`, for the caller to log after releasing the lock (logging allocates, and the
    /// DLL's allocator cannot allocate with its own lock held: the Mutex is not reentrant)
    ///
    /// [pt] (livres, topo) do ultimo `juntar`, para quem chamou registrar depois de soltar a trava (registrar aloca, e o
    /// alocador da DLL nao pode alocar com a propria trava na mao: o Mutex nao e reentrante)
    merged: Option<(usize, usize)>,
}

const CLASSES: usize = SMALL / 16 + 65536 + 1;

fn capacity(n: usize) -> usize {
    let n = n.max(1);
    if n <= SMALL { (n + 15) & !15 } else { (n + 4095) & !4095 }
}

/// Class of a capacity (None: too large to reuse).
///
/// [pt] Classe de uma capacidade (None: grande demais para reaproveitar).
fn class_of(cap: usize) -> Option<usize> {
    let c = if cap <= SMALL { cap / 16 } else { SMALL / 16 + cap / 4096 };
    (c < CLASSES).then_some(c)
}

impl Allocator {
    pub const fn novo(start: usize, size: usize) -> Self {
        Allocator { start, top: start, end: start + size, free: [0; CLASSES], peak: 0, merged: None }
    }

    /// Switches to the given region (without moving the structure, which is large for the stack).
    ///
    /// [pt] Passa a usar a regiao dada (sem mover a estrutura, que e grande para a pilha).
    fn activate(&mut self, start: usize, size: usize) {
        (self.start, self.top, self.end) = (start, start, start + size);
    }

    fn active(&self) -> bool {
        self.end != 0
    }

    pub unsafe fn alocar(&mut self, n: usize, zero: bool) -> usize {
        let cap = capacity(n);
        let free_head = class_of(cap).map_or(0, |c| self.free[c]);
        let p = match free_head {
            p if p != 0 => {
                self.free[class_of(cap).unwrap()] = *(p as *const usize);
                p
            }
            _ => {
                if self.end - self.top < HEADER + cap {
                    // only with the region exhausted (the careers that fit in it do not change): merges the free blocks and tries again
                    return self.alloc_merging(n, cap, zero);
                }
                let p = self.top + HEADER;
                self.top += HEADER + cap;
                if self.top - self.start > self.peak {
                    self.peak = self.top - self.start;
                }
                p
            }
        };
        *((p - 16) as *mut usize) = cap;
        *((p - 12) as *mut usize) = n;
        if zero {
            core::ptr::write_bytes(p as *mut u8, 0, n);
        }
        p
    }

    /// Region exhausted: merges neighbouring free blocks (those at the end go back to the top) and serves the request from the top or
    /// from the smallest free block that fits, splitting off the rest. Deterministic (by address order). The wide career
    /// (8 leagues) exhausted 1 GB in April 2002: the players' statistics list grows 100 at a time and the
    /// old copies were never reused (27/10/2026).
    /// simplification: scans all lists on each exhaustion; if it gets slow, merge only after N bytes freed.
    ///
    /// [pt] Regiao esgotada: junta os blocos livres vizinhos (os do fim voltam ao topo) e serve o pedido pelo topo ou
    /// pelo menor livre que caiba, partindo o resto. Deterministico (pela ordem dos enderecos). A carreira ampla
    /// (8 ligas) esgotava 1 GB em abril de 2002: a lista de estatisticas dos jogadores cresce de 100 em 100 e as
    /// copias antigas nunca serviam de novo (27/10/2026).
    /// simplification: varre todas as listas a cada esgotamento; se ficar lento, juntar so depois de N bytes liberados.
    unsafe fn alloc_merging(&mut self, n: usize, cap: usize, zero: bool) -> usize {
        let mut p = self.smallest_free(cap);
        if p == 0 {
            self.merge_free();
            if self.end - self.top >= HEADER + cap {
                p = self.top + HEADER;
                self.top += HEADER + cap;
                *((p - 16) as *mut usize) = cap;
            } else {
                p = self.smallest_free(cap);
            }
        }
        if p != 0 {
            *((p - 12) as *mut usize) = n;
            if zero {
                core::ptr::write_bytes(p as *mut u8, 0, n);
            }
        }
        p
    }

    /// The smallest free block with capacity >= `cap` (the first of each list), split if there is room left; 0 if none.
    ///
    /// [pt] O menor livre de capacidade >= `cap` (o primeiro de cada lista), partido se sobrar; 0 se nenhum.
    unsafe fn smallest_free(&mut self, cap: usize) -> usize {
        let Some(c0) = class_of(cap) else { return 0 };
        let Some(c) = (c0..CLASSES).find(|&c| self.free[c] != 0 && *((self.free[c] - 16) as *const usize) >= cap) else {
            return 0;
        };
        let p = self.free[c];
        self.free[c] = *(p as *const usize);
        let total = *((p - 16) as *const usize);
        if total - cap >= HEADER + 16 {
            let rest = p + cap + HEADER;
            *((rest - 16) as *mut usize) = total - cap - HEADER;
            self.liberar(rest);
            *((p - 16) as *mut usize) = cap;
        }
        p
    }

    /// Merges neighbouring free blocks; those that reach the top go back to it. Does not go through the global allocator (the list goes
    /// straight to the Windows heap) nor logs: it also serves the DLL region, which runs with its own lock held.
    ///
    /// [pt] Junta os livres vizinhos; os que chegam ao topo voltam para ele. Nao passa pelo alocador global (a lista vai
    /// direto na heap do Windows) nem registra: serve tambem a regiao da DLL, que roda com a propria trava na mao.
    unsafe fn merge_free(&mut self) {
        let mut n = 0;
        for c in 0..CLASSES {
            let mut p = self.free[c];
            while p != 0 {
                n += 1;
                p = *(p as *const usize);
            }
        }
        let layout = Layout::array::<usize>(n.max(1)).unwrap();
        let buf = System.alloc(layout) as *mut usize;
        if buf.is_null() {
            return;
        }
        let free = core::slice::from_raw_parts_mut(buf, n);
        let mut i = 0;
        for c in 0..CLASSES {
            let mut p = self.free[c];
            while p != 0 {
                free[i] = p;
                i += 1;
                p = *(p as *const usize);
            }
            self.free[c] = 0;
        }
        free.sort_unstable();
        // neighbours merged in place: the first of each run gets the summed capacity
        let mut k = 0;
        for i in 0..n {
            let (p, c) = (free[i], *((free[i] - 16) as *const usize));
            if k > 0 {
                let q = free[k - 1];
                let cq = *((q - 16) as *const usize);
                if q + cq == p - HEADER {
                    *((q - 16) as *mut usize) = cq + HEADER + c;
                    continue;
                }
            }
            free[k] = p;
            k += 1;
        }
        if k > 0 && free[k - 1] + *((free[k - 1] - 16) as *const usize) == self.top {
            self.top = free[k - 1] - HEADER;
            k -= 1;
        }
        for i in (0..k).rev() {
            self.liberar(free[i]);
        }
        System.dealloc(buf as *mut u8, layout);
        self.merged = Some((k, self.top));
    }

    /// Logs the notice of the last `merge_free` (call after releasing the lock).
    ///
    /// [pt] Registra o aviso do ultimo `juntar` (chamar depois de soltar a trava).
    fn log_merged(notice: Option<(usize, usize)>, which: &str) {
        if let Some((k, top)) = notice {
            linha(&format!("allocator: region {} exhausted; {} free after merging, top at {:#x}", which, k, top));
        }
    }

    pub unsafe fn liberar(&mut self, p: usize) {
        if let Some(c) = class_of(*((p - 16) as *const usize)) {
            *(p as *mut usize) = self.free[c];
            self.free[c] = p;
        }
    }

    pub unsafe fn tamanho(&self, p: usize) -> usize {
        *((p - 12) as *const usize)
    }

    pub unsafe fn realocar(&mut self, p: usize, n: usize, flags: u32) -> usize {
        let (cap, old_size) = (*((p - 16) as *const usize), self.tamanho(p));
        if capacity(n) <= cap {
            // fits in the same block
            if flags & HEAP_ZERO_MEMORY != 0 && n > old_size {
                core::ptr::write_bytes((p + old_size) as *mut u8, 0, n - old_size);
            }
            *((p - 12) as *mut usize) = n;
            return p;
        }
        if flags & HEAP_REALLOC_IN_PLACE_ONLY != 0 {
            return 0;
        }
        let q = self.alocar(n, flags & HEAP_ZERO_MEMORY != 0);
        if q != 0 {
            core::ptr::copy_nonoverlapping(p as *const u8, q as *mut u8, old_size.min(n));
            self.liberar(p);
        }
        q
    }

    pub fn contem(&self, p: usize) -> bool {
        p >= self.start && p < self.end
    }
}

static HEAP: Mutex<Allocator> = Mutex::new(Allocator::novo(0, 0));
static mut ORIGINALS: [usize; 6] = [0; 6]; // Create, Alloc, Free, ReAlloc, Size, Destroy
static mut HANDLE: usize = 0; // "handle" returned by HeapCreate (= start of the region)

fn with_heap<R>(f: impl FnOnce(&mut Allocator) -> R) -> R {
    let mut g = HEAP.lock().unwrap_or_else(|e| e.into_inner());
    let r = f(&mut g);
    let notice = g.merged.take();
    drop(g);
    Allocator::log_merged(notice, "do jogo");
    r
}

unsafe fn is_ours(h: usize) -> bool {
    h == HANDLE && HANDLE != 0
}

unsafe extern "system" fn heap_create(_flags: u32, _initial: usize, _maximum: usize) -> usize {
    HANDLE
}
unsafe extern "system" fn heap_alloc(h: usize, flags: u32, n: usize) -> usize {
    if !is_ours(h) {
        let f: unsafe extern "system" fn(usize, u32, usize) -> usize = core::mem::transmute(ORIGINALS[1]);
        return f(h, flags, n);
    }
    let p = with_heap(|a| a.alocar(n, flags & HEAP_ZERO_MEMORY != 0));
    if p == 0 {
        linha(&format!("allocator: region exhausted asking for {} bytes", n));
    }
    p
}
unsafe extern "system" fn heap_free(h: usize, flags: u32, p: usize) -> i32 {
    if !is_ours(h) {
        let f: unsafe extern "system" fn(usize, u32, usize) -> i32 = core::mem::transmute(ORIGINALS[2]);
        return f(h, flags, p);
    }
    if p != 0 {
        // the match engine (0xf63b bytes, 0xf640 with the CRT's rounding; players on the pitch included) goes into the match fingerprint
        if with_heap(|a| a.tamanho(p)) == 0xf640 {
            crate::simulation::match_finished(p);
        }
        with_heap(|a| a.liberar(p));
    }
    1
}
unsafe extern "system" fn heap_realloc(h: usize, flags: u32, p: usize, n: usize) -> usize {
    if !is_ours(h) {
        let f: unsafe extern "system" fn(usize, u32, usize, usize) -> usize = core::mem::transmute(ORIGINALS[3]);
        return f(h, flags, p, n);
    }
    with_heap(|a| a.realocar(p, n, flags))
}
unsafe extern "system" fn heap_size(h: usize, flags: u32, p: usize) -> usize {
    if !is_ours(h) {
        let f: unsafe extern "system" fn(usize, u32, usize) -> usize = core::mem::transmute(ORIGINALS[4]);
        return f(h, flags, p);
    }
    with_heap(|a| a.tamanho(p))
}
unsafe extern "system" fn heap_destroy(h: usize) -> i32 {
    if !is_ours(h) {
        let f: unsafe extern "system" fn(usize) -> i32 = core::mem::transmute(ORIGINALS[5]);
        return f(h);
    }
    1
}

/// Used range of the game region (start, top), for the simulation's daily hash.
///
/// [pt] RoleBitEffect usada da regiao do jogo (inicio, topo), para o hash diario da simulacao.
pub fn game_region() -> (usize, usize) {
    let a = HEAP.lock().unwrap_or_else(|e| e.into_inner());
    (a.start, a.top)
}

/// Peak usage of the region (bytes), for the simulation's log.
///
/// [pt] Pico de uso da regiao (bytes), para o registro da simulacao.
pub fn pico() -> usize {
    HEAP.lock().unwrap_or_else(|e| e.into_inner()).peak
}

// ---------- the DLL's global allocator ----------

const VISIBLE_REGION: usize = 0x0800_0000; // 128 MB, just below the game region
const VISIBLE_BASES: [usize; 1] = [0x1000_0000];
static VISIBLE: Mutex<Allocator> = Mutex::new(Allocator::novo(0, 0));
static IN_VISIBLE: AtomicBool = AtomicBool::new(false);

/// The `visivel` pair for the hook of `cm0102-base` (the video layer's allocations): what is created in between
/// is allocated in the region visible to the game (when the simulation switched it on).
///
/// [pt] Aloca o que `f` criar na regiao visivel ao jogo (quando a simulacao a ligou).
/// The `visivel` pair for the hook of `cm0102-base` (the video layer's allocations).
pub fn enter_visible() -> bool {
    IN_VISIBLE.swap(true, Ordering::Relaxed)
}

pub fn leave_visible(before: bool) {
    IN_VISIBLE.store(before, Ordering::Relaxed);
}

#[allow(dead_code)]
pub fn visivel<R>(f: impl FnOnce() -> R) -> R {
    let before = IN_VISIBLE.swap(true, Ordering::Relaxed);
    let r = f();
    IN_VISIBLE.store(before, Ordering::Relaxed);
    r
}

// The rest of the DLL's memory (Vec, String, format!, Box: log, daily log, comparisons, tapes) also goes to a
// fixed region in the simulation, its own and of the thread that loaded the DLL (the game's). In the process heap, which changes
// address on every run, those pointers ended up on the private stack and an original routine that reads a variable without
// a value found them: the career changed between identical runs (item cqc_6, 01/11/2026). Like the game heap and the
// visible region, it writes nothing on the stack: it only fixes where the DLL's memory lives. Outside the simulation, nothing changes. What
// does not fit (region exhausted even after merging) goes to the Windows heap, with a WARNING.
// Comparison mode stays on the Windows heap: its snapshots and tapes reach 143 MB live up to 2002-01-01 (the Rust career,
// 96 KB), there is no free low space with room for that, and its criterion is divergence, not the fingerprint.
const DLL_REGION: usize = 0x0400_0000; // 64 MB; the career up to 2002-06-01 uses 145 KB
const DLL_BASES: [usize; 1] = [0x0b00_0000]; // up to 0x0f000000, below the private stack (0x0ff00000)
static DLL: Mutex<Allocator> = Mutex::new(Allocator::novo(0, 0));
static DLL_RANGE: [AtomicUsize; 2] = [AtomicUsize::new(0), AtomicUsize::new(0)]; // (start, end): dealloc checks without the lock
static DLL_THREAD: AtomicU32 = AtomicU32::new(0);
static DLL_EXHAUSTED: AtomicBool = AtomicBool::new(false);
static DLL_LIVE: [AtomicUsize; 2] = [AtomicUsize::new(0), AtomicUsize::new(0)]; // (bytes live now, maximum)

/// The DLL's global allocator: the process heap; in the simulation, the visible region inside `visivel(...)` and the DLL
/// region for the rest.
///
/// [pt] Alocador global da DLL: heap do processo; na simulacao, a regiao visivel dentro de `visivel(...)` e a regiao da
/// DLL no resto.
pub struct Global;

unsafe fn in_visible_region(l: Layout, zero: bool) -> Option<*mut u8> {
    if !IN_VISIBLE.load(Ordering::Relaxed) || l.align() > 16 {
        return None;
    }
    let mut a = VISIBLE.lock().unwrap_or_else(|e| e.into_inner());
    a.active().then(|| a.alocar(l.size(), zero) as *mut u8)
}

unsafe fn in_dll_region(l: Layout, zero: bool) -> Option<*mut u8> {
    let t = DLL_THREAD.load(Ordering::Relaxed);
    if t == 0 || l.align() > 16 || crate::win::GetCurrentThreadId() != t {
        return None;
    }
    let mut a = DLL.lock().unwrap_or_else(|e| e.into_inner());
    let p = a.alocar(l.size(), zero);
    let notice = a.merged.take();
    drop(a);
    Allocator::log_merged(notice, "da DLL");
    if p == 0 && !DLL_EXHAUSTED.swap(true, Ordering::Relaxed) {
        linha(&format!("allocator: WARNING the DLL region is exhausted asking for {} bytes; what does not fit goes to the Windows heap", l.size()));
    }
    if p == 0 {
        return None;
    }
    let live = DLL_LIVE[0].fetch_add(l.size(), Ordering::Relaxed) + l.size();
    DLL_LIVE[1].fetch_max(live, Ordering::Relaxed);
    Some(p as *mut u8)
}

unsafe impl GlobalAlloc for Global {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        in_visible_region(l, false).or_else(|| in_dll_region(l, false)).unwrap_or_else(|| System.alloc(l))
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        in_visible_region(l, true).or_else(|| in_dll_region(l, true)).unwrap_or_else(|| System.alloc_zeroed(l))
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        let mut a = VISIBLE.lock().unwrap_or_else(|e| e.into_inner());
        if a.contem(p as usize) {
            a.liberar(p as usize);
            return;
        }
        drop(a);
        if (DLL_RANGE[0].load(Ordering::Relaxed)..DLL_RANGE[1].load(Ordering::Relaxed)).contains(&(p as usize)) {
            DLL.lock().unwrap_or_else(|e| e.into_inner()).liberar(p as usize);
            DLL_LIVE[0].fetch_sub(l.size(), Ordering::Relaxed);
        } else {
            System.dealloc(p, l)
        }
    }
}

/// Peak usage of the DLL region and maximum bytes live in it (0 without it), for the simulation's log. The peak exceeds
/// the live bytes because free blocks are only merged when the region runs out.
///
/// [pt] Pico de uso da regiao da DLL e maximo de bytes vivos nela (0 sem ela), para o registro da simulacao. O pico passa dos
/// vivos porque os livres so se juntam com a regiao esgotada.
pub fn dll_peak() -> (usize, usize) {
    (DLL.lock().unwrap_or_else(|e| e.into_inner()).peak, DLL_LIVE[1].load(Ordering::Relaxed))
}

unsafe fn reserve(bases: &[usize], size: usize) -> usize {
    for &b in bases {
        let p = crate::win::VirtualAlloc(b as *mut _, size, crate::win::MEM_COMMIT | crate::win::MEM_RESERVE, crate::win::PAGE_READWRITE);
        if !p.is_null() {
            if p as usize != bases[0] {
                linha(&format!("allocator: WARNING region at {:#x} and not at {:#x}", p as usize, bases[0]));
            }
            return p as usize;
        }
    }
    0
}

// ---------- the exe's VirtualAlloc ----------
//
// The VC6 CRT's small-block allocator (SBH) reserves 1 MB regions with VirtualAlloc(NULL)
// and Windows picks the address (day 2001-254: pointers at 0x045xxxxx changing between runs).
// Reservations without an address go to sequential addresses in a fixed range.

const VIRTUAL_RANGE: (usize, usize) = (0x5800_0000, 0x5f00_0000);
static NEXT_VIRTUAL: Mutex<usize> = Mutex::new(VIRTUAL_RANGE.0);
static mut VIRTUAL_ALLOC_ORIGINAL: usize = 0;

unsafe extern "system" fn virtual_alloc(end: usize, size: usize, kind: u32, prot: u32) -> usize {
    let f: unsafe extern "system" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(VIRTUAL_ALLOC_ORIGINAL);
    if end != 0 || kind & crate::win::MEM_RESERVE == 0 {
        return f(end, size, kind, prot);
    }
    let mut next = NEXT_VIRTUAL.lock().unwrap_or_else(|e| e.into_inner());
    while *next + size <= VIRTUAL_RANGE.1 {
        let candidate = *next;
        *next += (size + 0xffff) & !0xffff;
        let r = f(candidate, size, kind, prot);
        if r != 0 {
            return r;
        }
    }
    linha("allocator: WARNING the fixed VirtualAlloc band is exhausted; Windows address");
    f(0, size, kind, prot)
}

// ---------- the exe's GlobalAlloc ----------
//
// Database.cpp allocates the whole database with GlobalAlloc (0x529cdf) and the game keeps pointers
// into it (day 2001-254). The exe only imports GlobalAlloc/GlobalFree (no GlobalLock), that
// is, always GMEM_FIXED: the handle is the pointer itself.

const GMEM_ZEROINIT: u32 = 0x40;

/// The live GlobalAlloc blocks (the game's database), for the memory fingerprint.
///
/// [pt] Os blocos vivos do GlobalAlloc (o banco de dados do jogo), para a impressao da memoria.
static DATABASE_BLOCKS: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());

/// The size of the block that starts at `p` in the fixed region, if `p` really looks like the start of a block
/// (consistent header: a class capacity, not smaller than that of the requested size; realloc keeps
/// the capacity when shrinking).
///
/// [pt] O tamanho do bloco que comeca em `p` na regiao fixa, se `p` parece mesmo o inicio de um bloco
/// (cabecalho coerente: capacidade de classe, nao menor que a do tamanho pedido; o realocar mantem
/// a capacidade ao encolher).
pub unsafe fn bloco(p: usize) -> Option<usize> {
    let (start, top) = game_region();
    if p < start + HEADER || p >= top || p & 15 != 0 {
        return None;
    }
    let (cap, n) = (*((p - 16) as *const usize), *((p - 12) as *const usize));
    (n > 0 && cap >= capacity(n) && capacity(cap) == cap && p + cap <= top).then_some(n)
}

pub fn database_blocks() -> Vec<(usize, usize)> {
    DATABASE_BLOCKS.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

unsafe extern "system" fn global_alloc(flags: u32, n: usize) -> usize {
    let p = with_heap(|a| a.alocar(n, flags & GMEM_ZEROINIT != 0));
    if p != 0 {
        DATABASE_BLOCKS.lock().unwrap_or_else(|e| e.into_inner()).push((p, n));
    }
    p
}
unsafe extern "system" fn global_free(p: usize) -> usize {
    if p != 0 {
        DATABASE_BLOCKS.lock().unwrap_or_else(|e| e.into_inner()).retain(|b| b.0 != p);
        with_heap(|a| a.liberar(p));
    }
    0
}

/// Reserves the fixed region and swaps the exe's Heap* functions. Called when the DLL loads, before the game's CRT.
///
/// [pt] Reserva a regiao fixa e troca as Heap* do exe. Chamado no carregamento da DLL, antes da CRT do jogo.
pub unsafe fn instalar() {
    let start = reserve(&BASES, REGION);
    if start == 0 {
        linha("allocator: WARNING no free fixed region; Windows heap (simulation not reproducible)");
        return;
    }
    let vis = reserve(&VISIBLE_BASES, VISIBLE_REGION);
    if vis != 0 {
        VISIBLE.lock().unwrap_or_else(|e| e.into_inner()).activate(vis, VISIBLE_REGION);
        linha(&format!("alocador: memoria visivel da DLL em regiao fixa {:#x}..{:#x}", vis, vis + VISIBLE_REGION));
    }
    let compare = crate::win::ambiente("CM_COMPARAR");
    let dll = if compare { 0 } else { reserve(&DLL_BASES, DLL_REGION) };
    if compare {
        linha("allocator: comparison mode, the rest of the DLL memory stays on the Windows heap");
    } else if dll != 0 {
        DLL.lock().unwrap_or_else(|e| e.into_inner()).activate(dll, DLL_REGION);
        DLL_RANGE[0].store(dll, Ordering::Relaxed);
        DLL_RANGE[1].store(dll + DLL_REGION, Ordering::Relaxed);
        DLL_THREAD.store(crate::win::GetCurrentThreadId(), Ordering::Relaxed);
        linha(&format!("allocator: rest of the DLL memory in fixed region {:#x}..{:#x}", dll, dll + DLL_REGION));
    } else {
        linha("allocator: WARNING the DLL region is taken; the rest of the DLL memory stays on the Windows heap");
    }
    HEAP.lock().unwrap_or_else(|e| e.into_inner()).activate(start, REGION);
    HANDLE = start;
    let swaps: [(&str, usize); 6] = [
        ("HeapCreate", heap_create as *const () as usize),
        ("HeapAlloc", heap_alloc as *const () as usize),
        ("HeapFree", heap_free as *const () as usize),
        ("HeapReAlloc", heap_realloc as *const () as usize),
        ("HeapSize", heap_size as *const () as usize),
        ("HeapDestroy", heap_destroy as *const () as usize),
    ];
    crate::iat_hooks::replace_import("KERNEL32.dll", "GlobalAlloc", global_alloc as *const () as usize);
    crate::iat_hooks::replace_import("KERNEL32.dll", "GlobalFree", global_free as *const () as usize);
    match crate::iat_hooks::replace_import("KERNEL32.dll", "VirtualAlloc", virtual_alloc as *const () as usize) {
        Some(a) => VIRTUAL_ALLOC_ORIGINAL = a,
        None => linha("allocator: WARNING VirtualAlloc not replaced"),
    }
    for (i, (name, new_fn)) in swaps.iter().enumerate() {
        match crate::iat_hooks::replace_import("KERNEL32.dll", name, *new_fn) {
            Some(a) => ORIGINALS[i] = a,
            None => linha(&format!("allocator: {} not replaced", name)),
        }
    }
    linha(&format!("alocador: regiao fixa {:#x}..{:#x}", start, start + REGION));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reproducible_and_reuses() {
        let mut mem = vec![0u8; 1 << 20];
        let base = (mem.as_mut_ptr() as usize + 15) & !15;
        let run = |base: usize| unsafe {
            let mut a = Allocator::novo(base, (1 << 20) - 16);
            let x = a.alocar(10, false);
            let y = a.alocar(5000, true);
            assert_eq!((x % 16, a.tamanho(x), a.tamanho(y)), (0, 10, 5000));
            a.liberar(x);
            let z = a.alocar(16, false); // same class as x: reused
            assert_eq!(z, x);
            *(z as *mut u8) = 7;
            let w = a.realocar(z, 12, 0); // fits: same block
            assert_eq!(w, z);
            let v = a.realocar(w, 100, HEAP_ZERO_MEMORY); // does not fit: copies
            assert_ne!(v, w);
            assert_eq!(*(v as *const u8), 7);
            assert_eq!(*((v + 99) as *const u8), 0);
            assert_eq!(a.realocar(v, 5000, HEAP_REALLOC_IN_PLACE_ONLY), 0);
            assert_eq!(a.alocar(1 << 21, false), 0); // exhausted
            vec![x - base, y - base, z - base, v - base]
        };
        assert_eq!(run(base), run(base));
    }

    #[test]
    fn exhausted_merges_free_blocks() {
        let mut mem = vec![0u8; 1 << 20];
        let base = (mem.as_mut_ptr() as usize + 15) & !15;
        unsafe {
            let mut a = Allocator::novo(base, (1 << 20) - 16);
            let fixed = a.alocar(100, false); // at the start, never freed
            // the list that grows 3200 bytes at a time (like the players' statistics one): without merging, it runs out
            let mut p = a.alocar(3200, false);
            *(p as *mut u32) = 0x1234;
            for k in 2..300 {
                p = a.realocar(p, 3200 * k, 0);
                assert_ne!(p, 0, "esgotou em {}", k);
                assert_eq!(*(p as *const u32), 0x1234);
                if 3200 * (k + 1) > (1 << 18) { // without merging, it runs out near k = 25
                    break;
                }
            }
            assert_eq!(a.tamanho(fixed), 100);
        }
    }
}
