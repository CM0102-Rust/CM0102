// ---------- batch %%LEVA%%: the TCP/IP layer, part 2: listen, connect, accept, send and receive on connections, the broadcast sockets and datagrams, and the adapter scan (tcpip.cpp) ----------

/// GetVersionExA's slot in the exe's import table (the scan calls it through the IAT).
const IAT_GET_VERSION_EX_A: usize = 0x9670c8;
/// The C runtime the scan uses (cdecl): `_spawnl(mode, command, args..., 0)`, `getenv`, `system`.
const FN_CRT_SPAWNL: usize = 0x9533ca;
const FN_CRT_GETENV: usize = 0x947480;
const FN_CRT_SYSTEM: usize = 0x9473ea;
/// The display (goldcup.cpp): whether the game is full screen (`[0xae319c]`), and `graphics_init_display(width,
/// height, 2, G_FONT_MODE, flag, title)` (cdecl; 0 = failed).
const FN_GRAPHICS_IS_FULLSCREEN: usize = 0x5e2ea0;
const FN_GRAPHICS_INIT_DISPLAY: usize = 0x5e2620;
/// The width and height pushed by the scan's two `graphics_init_display` calls, read from the running exe (the
/// cm0102-1080.exe has 1920 × 1080 there).
const ADAPTER_SCAN_WIDTH_1: usize = 0x8aeed9;
const ADAPTER_SCAN_HEIGHT_1: usize = 0x8aeed4;
const ADAPTER_SCAN_WIDTH_2: usize = 0x8aef01;
const ADAPTER_SCAN_HEIGHT_2: usize = 0x8aeefc;
/// "Championship Manager 01/02".
const GAME_TITLE: usize = 0x9cd328;
/// "winipcfg", " /batch", "~net.out", "ip.exe", " ", "windir", "\system32\ipconfig > ~net.out", "r".
const WINIPCFG: usize = 0xa8c738;
const WINIPCFG_BATCH: usize = 0xa8c744;
const NET_OUT_FILE: usize = 0xa8c74c;
const IP_EXE: usize = 0xa8c730;
const SPACE_TEXT: usize = 0x9f1308;
const WINDIR: usize = 0xa8c728;
const IPCONFIG_COMMAND: usize = 0xa8c708;
const READ_MODE: usize = 0x9899fc;
/// The FILE flag of the end of the file (`_IOEOF` in `_flag`, +0xc).
const FILE_FLAGS: usize = 0xc;
const FILE_EOF: u8 = 0x10;
/// The length of a report line read (`fgets`).
const ADAPTER_REPORT_LINE: u32 = 200;

/// How the adapter report is laid out: an address line is the one whose running colon count, modulo `period`, is
/// `residue`; the mask line is the next residue.
#[derive(Clone, Copy)]
struct AdapterReport {
    residue: i8,
    period: i8,
}

/// winipcfg (Windows 9x), ip.exe (NT 4): every 4th colon line from the 2nd.
const WINIPCFG_REPORT: AdapterReport = AdapterReport { residue: 2, period: 4 };
/// ipconfig (Windows 2000 and later): every 5th colon line from the 3rd.
const IPCONFIG_REPORT: AdapterReport = AdapterReport { residue: 3, period: 5 };

/// An adapter's sockaddr_in for `bind`/`sendto`: the 16 bytes at `entry` with family AF_INET and port `htons(port)`.
unsafe fn tcpip_adapter_sockaddr(entry: usize, port: u32) -> [u8; 16] {
    let mut a = [0u8; 16];
    core::ptr::copy(entry as *const u8, a.as_mut_ptr(), 16);
    a[..2].copy_from_slice(&AF_INET.to_le_bytes());
    a[2..4].copy_from_slice(&(port as u16).to_be_bytes());
    a
}

/// One adapter's listen socket into `at`: a stream socket (invalid: stays −1, false); `SO_REUSEADDR` with `option`,
/// non-blocking, `SO_LINGER` off (a failure closes it, false); `tcpip_tune_stream_socket`; bound to the adapter's
/// address at `port` and listening with the connection table length as the backlog (a failure closes it, false).
unsafe fn tcpip_open_listen_socket(this: usize, at: usize, adapter: usize, port: u32, option: &mut u32) -> bool {
    let s = ws_socket(AF_INET as u32, SOCK_STREAM, 0);
    write_u32(at, s);
    if s == INVALID_SOCKET {
        return false;
    }
    let mut non_blocking = 1u32;
    let linger = [0u16; 2];
    if ws_setsockopt(s, SO_REUSEADDR, option as *mut u32 as usize) == -1
        || ws_ioctlsocket(s, FIONBIO, &mut non_blocking) == -1
        || ws_setsockopt(s, SO_LINGER, linger.as_ptr() as usize) == -1
    {
        tcpip_close_socket_at(at);
        return false;
    }
    tcpip_tune_stream_socket(s, option);
    let address = tcpip_adapter_sockaddr(adapter, port);
    if ws_bind(s, address.as_ptr() as usize, 16) == -1 || ws_listen(s, read_i16(this + tcpip_object::CONNECTION_COUNT) as i32) == -1 {
        tcpip_close_socket_at(at);
        return false;
    }
    true
}

/// Rule: the game listens for connections on every local adapter at the given port; true when at least one listen
/// socket opens.
///
/// No connection table, or Winsock not started (`tcpip_ensure_winsock`): 0. The 20 listen sockets closed; the
/// adapters scanned into a local table of 20 (`TcpIp::scan_adapter_addresses`); each adapter k gets its listen
/// socket in slot k (`tcpip_open_listen_socket`; the option of 1 becomes 15000 after the first socket tuned and stays
/// so). Defect of the original: more than 20 adapters overflow the local table and the listen socket array.
/// Original: `TcpIp::open_listen_sockets` (0x8ae370, tcpip.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn tcpip_open_listen_sockets(this: usize, port: u32) -> u32 {
    if read_usize(this + TCPIP_CONNECTIONS) == 0 || !tcpip_ensure_winsock(this) {
        return 0;
    }
    covered!("tel47_2 listen sockets opened");
    for k in 0..tcpip_object::SOCKET_SLOTS {
        tcpip_close_socket_at(this + tcpip_object::LISTEN_SOCKETS + 4 * k);
    }
    let mut adapters = [0u8; 20 * tcpip_object::ADAPTER_SIZE];
    let mut count = 0i16;
    let scan: unsafe extern "thiscall" fn(usize, usize, *mut i16) = tcpip_fn(FN_TCPIP_SCAN_ADAPTER_ADDRESSES);
    scan(this, adapters.as_mut_ptr() as usize, &mut count);
    let mut option = 1u32;
    let mut opened = 0;
    let mut k = 0i16;
    while k < count {
        let at = this + tcpip_object::LISTEN_SOCKETS + 4 * k as usize;
        let adapter = adapters.as_ptr() as usize + k as usize * tcpip_object::ADAPTER_SIZE;
        if tcpip_open_listen_socket(this, at, adapter, port, &mut option) {
            opened = 1;
        }
        k += 1;
    }
    opened
}

/// Rule: a connection to a host takes the first free connection slot; returns the slot, or −1 when there is none or
/// the connection fails.
///
/// No connection table, Winsock not started (`tcpip_ensure_winsock`) or no free slot: −1. A stream socket into the
/// slot (invalid: −1). `SO_REUSEADDR` with an option of 1; a failure frees the slot, −1. `connect` (blocking) to the
/// address of `address` (+4) at `port`; a failure frees the slot, −1. Then non-blocking, `SO_LINGER` off and
/// `tcpip_tune_stream_socket`. Defects of the original: the `SO_REUSEADDR` failure frees the slot without closing the
/// socket; the `connect` failure calls `closesocket` on the slot number instead of the socket (the socket leaks); the
/// last 8 bytes of the sockaddr_in are stack residue (the copy sends zeros).
/// Original: `TcpIp::connect_to_host` (0x8ae640, tcpip.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn tcpip_connect_to_host(this: usize, port: u32, address: usize) -> u16 {
    if read_usize(this + TCPIP_CONNECTIONS) == 0 || !tcpip_ensure_winsock(this) {
        return 0xffff;
    }
    let count = read_i16(this + tcpip_object::CONNECTION_COUNT);
    let Some(slot) = (0..count).find(|&k| read_u32(tcpip_connection_at(this, k)) == INVALID_SOCKET) else {
        return 0xffff;
    };
    covered!("tel47_2 connect to a host");
    let at = tcpip_connection_at(this, slot);
    let s = ws_socket(AF_INET as u32, SOCK_STREAM, 0);
    write_u32(at, s);
    if s == INVALID_SOCKET {
        return 0xffff;
    }
    let mut option = 1u32;
    if ws_setsockopt(s, SO_REUSEADDR, &mut option as *mut u32 as usize) == -1 {
        write_u32(at, INVALID_SOCKET);
        return 0xffff;
    }
    let mut host = [0u8; 16];
    host[..2].copy_from_slice(&AF_INET.to_le_bytes());
    host[2..4].copy_from_slice(&(port as u16).to_be_bytes());
    host[4..8].copy_from_slice(&read_u32(address + 4).to_le_bytes());
    if ws_connect(s, host.as_ptr() as usize, 16) == -1 {
        ws_closesocket(slot as u32);
        write_u32(at, INVALID_SOCKET);
        return 0xffff;
    }
    let mut non_blocking = 1u32;
    let linger = [0u16; 2];
    ws_ioctlsocket(s, FIONBIO, &mut non_blocking);
    ws_setsockopt(s, SO_LINGER, linger.as_ptr() as usize);
    tcpip_tune_stream_socket(s, &mut option);
    slot as u16
}

/// Rule: a connection waiting on a listen socket is accepted into a free connection slot; returns the slot or −1.
///
/// No connection table: −1. For each open listen socket (20) whose `accept` gives a socket: the free slot is sought
/// from 0 while none is held yet; when the search stopped before the end of the table the socket goes to the held
/// slot, which is returned; otherwise the socket is closed and the next listen socket tried. Defects of the original:
/// a free slot that is the last of the table is found but the connection is closed; once found that way the slot stays
/// held, and the next accepted connection takes it with no search. None: −1.
/// Original: `TcpIp::accept_connection` (0x8ae840, tcpip.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn tcpip_accept_connection(this: usize) -> u16 {
    if read_usize(this + TCPIP_CONNECTIONS) == 0 {
        return 0xffff;
    }
    let mut free = -1i16;
    for k in 0..tcpip_object::SOCKET_SLOTS {
        let listen = read_u32(this + tcpip_object::LISTEN_SOCKETS + 4 * k);
        if listen == INVALID_SOCKET {
            continue;
        }
        let s = ws_accept(listen);
        if s == INVALID_SOCKET {
            continue;
        }
        let count = read_i16(this + tcpip_object::CONNECTION_COUNT);
        let mut j = 0i16;
        while j < count && free == -1 {
            if read_u32(tcpip_connection_at(this, j)) == INVALID_SOCKET {
                free = j;
            }
            j += 1;
        }
        if j < count { // tel47_2: a free slot found
            covered!("tel47_2 connection accepted");
            write_u32(tcpip_connection_at(this, free), s);
            return free as u16;
        }
        ws_closesocket(s);
    }
    0xffff
}

/// Rule: data goes out on one connection; the send is retried while the socket would block, any other error closes
/// the connection.
///
/// No connection table: −2. The slot free, no data or a length of 0 or less: −1. `send`: anything but −1 is returned
/// (a partial count too: the rest is not sent). −1 with `WSAEWOULDBLOCK`: sent again from the start; another error:
/// the slot closed (`tcpip_close_slot`), −2. (The original keeps a sent count for the retries, but only adds to it on
/// a path that never runs.)
/// Original: `TcpIp::send_to_connection` (0x8ae8d0, tcpip.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn tcpip_send_to_connection(this: usize, slot: u32, data: usize, length: i32) -> i32 {
    let slot = slot as i16;
    if read_usize(this + TCPIP_CONNECTIONS) == 0 {
        return -2;
    }
    if read_u32(tcpip_connection_at(this, slot)) == INVALID_SOCKET || data == 0 || length <= 0 {
        return -1;
    }
    covered!("tel47_2 data sent");
    loop {
        let r = ws_send(read_u32(tcpip_connection_at(this, slot)), data, length);
        if r != -1 {
            return r;
        }
        if ws_wsa_get_last_error() != WSAEWOULDBLOCK {
            tcpip_close_slot(this, slot);
            return -2;
        }
    }
}

/// Rule: data waiting on one connection is read; none waiting gives 0, an error or a disconnection closes it.
///
/// No connection table: −2. The slot free, no buffer or a length of 0 or less: −1. `recv`: −1 with
/// `WSAEWOULDBLOCK`: 0; −1 otherwise or 0 (closed by the other side): the slot closed, −2; else the count.
/// Original: `TcpIp::receive_from_connection` (0x8ae9c0, tcpip.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn tcpip_receive_from_connection(this: usize, slot: u32, buffer: usize, length: i32) -> i32 {
    let slot = slot as i16;
    if read_usize(this + TCPIP_CONNECTIONS) == 0 {
        return -2;
    }
    let s = read_u32(tcpip_connection_at(this, slot));
    if s == INVALID_SOCKET || buffer == 0 || length <= 0 {
        return -1;
    }
    let r = ws_recv(s, buffer, length);
    if r == -1 && ws_wsa_get_last_error() == WSAEWOULDBLOCK {
        return 0;
    }
    if r == -1 || r == 0 {
        covered!("tel47_2 connection lost");
        tcpip_close_slot(this, slot);
        return -2;
    }
    r
}

/// One adapter's broadcast socket into slot k: a UDP socket (invalid: stays −1, false); `SO_BROADCAST` with `option`
/// (a failure closes it, false); non-blocking, `SO_LINGER` on with 0 seconds and `SO_DEBUG` (unchecked); bound to the
/// adapter's address at `port`. Defect of the original: a failed `bind` closes the socket twice.
unsafe fn tcpip_open_broadcast_socket(this: usize, k: usize, port: u32, option: &mut u32) -> bool {
    let at = this + tcpip_object::BROADCAST_SOCKETS + 4 * k;
    let s = ws_socket(AF_INET as u32, SOCK_DGRAM, IPPROTO_UDP);
    write_u32(at, s);
    if s == INVALID_SOCKET {
        return false;
    }
    if ws_setsockopt(s, SO_BROADCAST, option as *mut u32 as usize) == -1 {
        tcpip_close_socket_at(at);
        return false;
    }
    let mut non_blocking = 1u32;
    let linger = [1u16, 0];
    ws_ioctlsocket(s, FIONBIO, &mut non_blocking);
    ws_setsockopt(s, SO_LINGER, linger.as_ptr() as usize);
    ws_setsockopt(s, SO_DEBUG, option as *mut u32 as usize);
    let address = tcpip_adapter_sockaddr(this + tcpip_object::ADAPTERS + k * tcpip_object::ADAPTER_SIZE, port);
    if ws_bind(s, address.as_ptr() as usize, 16) == -1 {
        ws_closesocket(s);
        tcpip_close_socket_at(at);
        return false;
    }
    true
}

/// Rule: the game opens a UDP broadcast socket on every local adapter at the given port; true when at least one binds.
///
/// Winsock not started (`tcpip_ensure_winsock`): 0. The 20 broadcast sockets closed; the adapters scanned into the
/// object (`tcpip_object::ADAPTERS`, `ADAPTER_COUNT`); each adapter k gets its socket in slot k
/// (`tcpip_open_broadcast_socket`, the option 1). Defect of the original: more than 20 adapters overflow the adapter
/// table into the socket arrays.
/// Original: `TcpIp::open_broadcast_sockets` (0x8aea80, tcpip.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn tcpip_open_broadcast_sockets(this: usize, port: u32) -> u32 {
    if !tcpip_ensure_winsock(this) {
        return 0;
    }
    covered!("tel47_2 broadcast sockets opened");
    for k in 0..tcpip_object::SOCKET_SLOTS {
        tcpip_close_socket_at(this + tcpip_object::BROADCAST_SOCKETS + 4 * k);
    }
    let scan: unsafe extern "thiscall" fn(usize, usize, *mut i16) = tcpip_fn(FN_TCPIP_SCAN_ADAPTER_ADDRESSES);
    scan(this, this + tcpip_object::ADAPTERS, (this + tcpip_object::ADAPTER_COUNT) as *mut i16);
    let mut option = 1u32;
    let mut opened = 0;
    let mut k = 0i16;
    while k < read_i16(this + tcpip_object::ADAPTER_COUNT) {
        if tcpip_open_broadcast_socket(this, k as usize, port, &mut option) {
            opened = 1;
        }
        k += 1;
    }
    opened
}

/// Rule: a datagram goes to one address through one broadcast socket, or, with no address, to the broadcast address
/// of every adapter.
///
/// No data or a length of 0 or less: 0. With `to`: `sendto` on broadcast socket `index` (a short) to `to`
/// (`to_length`), its result. Without: each open broadcast socket k sends to adapter k's broadcast address
/// (`tcpip_object::ADAPTER_BROADCAST`) at `port`; the results (−1 included) are added and returned as a short.
/// Original: `TcpIp::send_datagram` (0x8aec90, tcpip.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn tcpip_send_datagram(this: usize, data: usize, length: i32, port: u32, to: usize, to_length: u32, index: u32) -> i32 {
    use tcpip_object as t;
    if data == 0 || length <= 0 {
        return 0;
    }
    if to != 0 {
        covered!("tel47_2 datagram to one address");
        let s = read_u32((this as isize + t::BROADCAST_SOCKETS as isize + 4 * index as i16 as isize) as usize);
        return ws_sendto(s, data, length, to, to_length);
    }
    covered!("tel47_2 datagram broadcast");
    let mut sent = 0i32;
    for k in 0..t::SOCKET_SLOTS {
        let s = read_u32(this + t::BROADCAST_SOCKETS + 4 * k);
        if s == INVALID_SOCKET {
            continue;
        }
        let address = tcpip_adapter_sockaddr(this + t::ADAPTERS + k * t::ADAPTER_SIZE + t::ADAPTER_BROADCAST, port);
        sent = sent.wrapping_add(ws_sendto(s, data, length, address.as_ptr() as usize, 16)); // tel47_2: each adapter's broadcast
    }
    sent as i16 as i32
}

/// Rule: the first datagram waiting on the broadcast sockets is read, with its sender and the socket's index.
///
/// No buffer, a length of 0 or less or no sender buffer: 0. Each open broadcast socket k in turn: `recvfrom` into
/// `buffer`, `from` and `from_length`; `*index` = k (written whatever the result); a result whose low word is
/// positive is returned (as that short). None: 0.
/// Original: `TcpIp::receive_datagram` (0x8aed80, tcpip.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn tcpip_receive_datagram(this: usize, buffer: usize, length: i32, from: usize, from_length: *mut i32, index: *mut u16) -> i32 {
    if buffer == 0 || length <= 0 || from == 0 {
        return 0;
    }
    for k in 0..tcpip_object::SOCKET_SLOTS {
        let s = read_u32(this + tcpip_object::BROADCAST_SOCKETS + 4 * k);
        if s == INVALID_SOCKET {
            continue;
        }
        let r = ws_recvfrom(s, buffer, length, from, from_length) as i16;
        *index = k as u16;
        if r > 0 {
            covered!("tel47_2 datagram received");
            return r as i32;
        }
    }
    0
}

/// The scan's way back from full screen before ipconfig runs: full screen, `graphics_init_display` with the size of
/// the exe's first call and flag 0, then flag 1 with the second's; both failing: the "tcpip.cpp 999" box, the fatal
/// cleanup and `exit(-1)`.
unsafe fn tcpip_leave_fullscreen() {
    let fullscreen: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_GRAPHICS_IS_FULLSCREEN);
    if fullscreen() == 0 {
        return;
    }
    let init: unsafe extern "C" fn(u32, u32, u32, u32, u32, usize) -> u32 = core::mem::transmute(FN_GRAPHICS_INIT_DISPLAY);
    let size = |width: usize, height: usize| ((width as *const u32).read_unaligned(), (height as *const u32).read_unaligned());
    let (w, h) = size(ADAPTER_SCAN_WIDTH_1, ADAPTER_SCAN_HEIGHT_1);
    if init(w, h, 2, read_u32(G_FONT_MODE), 0, GAME_TITLE) != 0 {
        return;
    }
    let (w, h) = size(ADAPTER_SCAN_WIDTH_2, ADAPTER_SCAN_HEIGHT_2);
    if init(w, h, 2, read_u32(G_FONT_MODE), 1, GAME_TITLE) != 0 {
        return;
    }
    tel26_error_box(TCPIP_CPP, 0x3e7);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(-1);
}

/// Writes the adapter report "~net.out" by the Windows version (`GetVersionExA`, through the IAT): Windows 9x runs
/// `winipcfg /batch ~net.out`, NT 4 or older `ip.exe` (waiting for both), later ones leave full screen
/// (`tcpip_leave_fullscreen`) and run `%windir%\system32\ipconfig > ~net.out` (`system`). Returns the report's layout
/// (ipconfig's only for the last; winipcfg's also when the version is not known, and then nothing runs).
unsafe fn tcpip_write_adapter_report() -> AdapterReport {
    let mut version = [0u32; 0x25];
    version[0] = 0x94; // dwOSVersionInfoSize
    let get_version: unsafe extern "system" fn(*mut u32) -> i32 = core::mem::transmute(read_usize(IAT_GET_VERSION_EX_A));
    if get_version(version.as_mut_ptr()) == 0 {
        return WINIPCFG_REPORT;
    }
    let spawn: unsafe extern "C" fn(u32, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPAWNL);
    let (major, platform) = (version[1], version[4]);
    if platform == 1 {
        spawn(0, WINIPCFG, WINIPCFG_BATCH, NET_OUT_FILE, 0usize);
        return WINIPCFG_REPORT;
    }
    if major <= 4 {
        spawn(0, IP_EXE, SPACE_TEXT, 0usize);
        return WINIPCFG_REPORT;
    }
    tcpip_leave_fullscreen();
    let getenv: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_GETENV);
    let system: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_SYSTEM);
    let mut command = [0u8; 0x104];
    let c = command.as_mut_ptr() as usize;
    network_copy(c, getenv(WINDIR));
    network_copy(c + network_strlen(c), IPCONFIG_COMMAND);
    system(c);
    IPCONFIG_REPORT
}

/// Where a report line's value starts: after its first colon (the colon itself when it is the first character),
/// past the spaces.
unsafe fn adapter_report_value(line: usize) -> usize {
    let mut k = 0;
    while read_u8(line + k) != b':' && read_u8(line + k) != 0 {
        k += 1;
    }
    if k != 0 {
        k += 1;
    }
    while read_u8(line + k) == b' ' {
        k += 1;
    }
    line + k
}

/// One report line. Every colon in it adds to the running `colons`; a line with none is skipped. With the count
/// modulo the period at the address residue: the value parsed into adapter `*count` (`fill_ipv4_sockaddr`); a
/// non-zero address other than "127.0.0.1" is kept (`*count` + 1, `*awaiting_mask`), otherwise not. At the next
/// residue, with an address kept and waiting: the value parsed into that adapter's second sockaddr, whose address
/// becomes the broadcast address (address AND mask) OR NOT mask; the wait is not cleared (a later mask line of the
/// same adapter parses again).
unsafe fn scan_adapter_line(line: usize, out: usize, count: *mut i16, colons: &mut i32, awaiting_mask: &mut bool, report: AdapterReport) {
    let mut has_colon = false;
    let mut k = 0;
    while read_u8(line + k) != 0 {
        if read_u8(line + k) == b':' {
            *colons += 1;
            has_colon = true;
        }
        k += 1;
    }
    if !has_colon {
        return;
    }
    let phase = *colons as i16 as i32 % report.period as i32;
    let n = *count;
    if phase == report.residue as i32 {
        let value = adapter_report_value(line);
        let entry = out + n as usize * tcpip_object::ADAPTER_SIZE;
        fill_ipv4_sockaddr(entry, value, 0);
        if read_u32(entry + 4) != 0 && !network_same(value, LOOPBACK_ADDRESS) {
            covered!("tel47_2 adapter address");
            *count = n + 1;
            *awaiting_mask = true;
        } else {
            *awaiting_mask = false;
        }
    } else if phase == report.residue as i32 + 1 && n > 0 && *awaiting_mask {
        covered!("tel47_2 adapter mask");
        let entry = out + (n - 1) as usize * tcpip_object::ADAPTER_SIZE;
        let mask = entry + tcpip_object::ADAPTER_BROADCAST;
        fill_ipv4_sockaddr(mask, adapter_report_value(line), 0);
        let (address, bits) = (read_u32(entry + 4), read_u32(mask + 4));
        write_u32(mask + 4, (address & bits) | !bits); // tel47_2: the broadcast address
    }
}

/// Rule: the machine's IPv4 adapters (address and broadcast address, loopback left out) are read from the Windows
/// adapter report.
///
/// `*count` = 0; `tcpip_write_adapter_report`; "~net.out" opened for reading (none: done); until the end of the file:
/// a line of up to 199 characters (`fgets`), its last character cut (the line feed), `scan_adapter_line`; closed.
/// Defects of the original: the line read at the end of the file fails and the previous line is scanned again (one
/// more character cut); an empty report scans the uninitialised line buffer once (the copy's starts empty); more than
/// 20 adapters overflow the caller's table (no bound); the colon count runs over every line, so a report with extra
/// colon lines (IPv6, newer ipconfig layouts) puts the residues on the wrong lines.
/// Original: `tcpip_scan_adapter_addresses` (0x8aee20, tcpip.cpp, thiscall with `this` unused, ret 8).
pub(crate) unsafe extern "thiscall" fn tcpip_scan_adapter_addresses(_this: usize, out: usize, count: *mut i16) {
    covered!("tel47_2 adapters scanned");
    *count = 0;
    let report = tcpip_write_adapter_report();
    let file = exe::crt_fopen(NET_OUT_FILE, READ_MODE);
    if file == 0 {
        return;
    }
    let mut buffer = [0u8; 1 + ADAPTER_REPORT_LINE as usize]; // [0]: the byte before the line, cut when a line is empty
    let line = buffer.as_mut_ptr() as usize + 1;
    let mut colons = 0i32;
    let mut awaiting_mask = false;
    while read_u8(file + FILE_FLAGS) & FILE_EOF == 0 {
        exe::at_94700c(line, ADAPTER_REPORT_LINE, file);
        write_u8(line + network_strlen(line) - 1, 0);
        scan_adapter_line(line, out, count, &mut colons, &mut awaiting_mask, report);
    }
    exe::crt_fclose(file);
}

// simplification: sockets do not open twice. Original only; proven in Rust mode (a network game).
/// Comparer for `tcpip_open_listen_sockets`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_open_listen_sockets(this: usize, port: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%)(this, port)
}

// simplification: a connection is made once. Original only; proven in Rust mode (a network game).
/// Comparer for `tcpip_connect_to_host`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_connect_to_host(this: usize, port: u32, address: usize) -> u16 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u16>(%%I1%%)(this, port, address)
}

// simplification: a connection is accepted once. Original only; proven in Rust mode (a network game).
/// Comparer for `tcpip_accept_connection`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_accept_connection(this: usize) -> u16 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u16>(%%I2%%)(this)
}

// simplification: data is sent once. Original only; proven in Rust mode (a network game).
/// Comparer for `tcpip_send_to_connection`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_send_to_connection(this: usize, slot: u32, data: usize, length: i32) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, i32) -> i32>(%%I3%%)(this, slot, data, length)
}

// simplification: data is read once. Original only; proven in Rust mode (a network game).
/// Comparer for `tcpip_receive_from_connection`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_receive_from_connection(this: usize, slot: u32, buffer: usize, length: i32) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, i32) -> i32>(%%I4%%)(this, slot, buffer, length)
}

// simplification: sockets do not open twice. Original only; proven in Rust mode (a network game).
/// Comparer for `tcpip_open_broadcast_sockets`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_open_broadcast_sockets(this: usize, port: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I5%%)(this, port)
}

// simplification: a datagram is sent once. Original only; proven in Rust mode (a network game).
/// Comparer for `tcpip_send_datagram`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_send_datagram(this: usize, data: usize, length: i32, port: u32, to: usize, to_length: u32, index: u32) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, usize, u32, u32) -> i32>(%%I6%%)(this, data, length, port, to, to_length, index)
}

// simplification: a datagram is read once. Original only; proven in Rust mode (a network game).
/// Comparer for `tcpip_receive_datagram`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_receive_datagram(this: usize, buffer: usize, length: i32, from: usize, from_length: *mut i32, index: *mut u16) -> i32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, usize, *mut i32, *mut u16) -> i32>(%%I7%%)(this, buffer, length, from, from_length, index)
}

// simplification: the scan runs a process and writes a file. Original only; proven in Rust mode (a network game).
/// Comparer for `tcpip_scan_adapter_addresses`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_scan_adapter_addresses(this: usize, out: usize, count: *mut i16) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i16)>(%%I8%%)(this, out, count)
}
