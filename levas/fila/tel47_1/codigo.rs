// ---------- batch %%LEVA%%: the TCP/IP layer, part 1: IPv4 addresses, the TcpIp object, Winsock, closing sockets and the loopback test (tcpip.cpp) ----------

/// The TcpIp object (tcpip.cpp, the base of the Network): offsets used by its methods. The connection table pointer
/// is `TCPIP_CONNECTIONS` (+0: u32 sockets, −1 free).
#[allow(dead_code)]
mod tcpip_object {
    /// i16: the length of the connection table.
    pub(crate) const CONNECTION_COUNT: usize = 0x4;
    /// i16: the local adapters found by `tcpip_scan_adapter_addresses`.
    pub(crate) const ADAPTER_COUNT: usize = 0x6;
    /// [20] × (sockaddr_in address, sockaddr_in broadcast): the local adapters.
    pub(crate) const ADAPTERS: usize = 0x8;
    pub(crate) const ADAPTER_SIZE: usize = 0x20;
    /// The broadcast sockaddr_in inside an adapter entry (the mask's address turned into the broadcast address).
    pub(crate) const ADAPTER_BROADCAST: usize = 0x10;
    /// u32[20]: the UDP broadcast sockets, one per adapter (−1 closed).
    pub(crate) const BROADCAST_SOCKETS: usize = 0x288;
    /// u32[20]: the TCP listen sockets, one per adapter (−1 closed).
    pub(crate) const LISTEN_SOCKETS: usize = 0x2d8;
    /// The WSADATA that WSAStartup fills.
    pub(crate) const WSA_DATA: usize = 0x328;
    /// u32 (unaligned): Winsock started by this object.
    pub(crate) const WINSOCK_STARTED: usize = 0x4b6;
    /// The length of the broadcast and listen socket arrays.
    pub(crate) const SOCKET_SLOTS: usize = 20;
}

/// The exe's Winsock import thunks (`jmp [IAT]`, wsock32, stdcall): tcpip.cpp reaches Winsock only through them.
const FN_WS_WSA_STARTUP: usize = 0x944c2e;
const FN_WS_WSA_CLEANUP: usize = 0x944c34;
const FN_WS_CLOSESOCKET: usize = 0x944c3a;
const FN_WS_ACCEPT: usize = 0x944c40;
const FN_WS_GETHOSTBYADDR: usize = 0x944c46;
const FN_WS_WSA_GET_LAST_ERROR: usize = 0x944c4c;
const FN_WS_SEND: usize = 0x944c52;
const FN_WS_RECV: usize = 0x944c58;
const FN_WS_SENDTO: usize = 0x944c5e;
const FN_WS_RECVFROM: usize = 0x944c6a;
const FN_WS_LISTEN: usize = 0x944c70;
const FN_WS_BIND: usize = 0x944c76;
const FN_WS_IOCTLSOCKET: usize = 0x944c7c;
const FN_WS_CONNECT: usize = 0x944c82;
const FN_WS_SETSOCKOPT: usize = 0x944c88;
const FN_WS_SOCKET: usize = 0x944c8e;

/// The TcpIp methods the copies call back through the exe (thiscall): the adapter scan (out, &count) and the loopback test.
const FN_TCPIP_SCAN_ADAPTER_ADDRESSES: usize = 0x8aee20;
const FN_TCPIP_TEST_LOOPBACK_CONNECTION: usize = 0x8af290;

/// "E:\dev\CM3\cm3 00-01\si\code\tcpip.cpp" (the error boxes of this file).
const TCPIP_CPP: usize = 0xa8c6d4;
/// "%d.%d.%d.%d".
const IPV4_FORMAT: usize = 0xa695e4;
/// "127.0.0.1".
const LOOPBACK_ADDRESS: usize = 0xa8c6fc;

/// Winsock values the TcpIp uses.
const INVALID_SOCKET: u32 = 0xffff_ffff;
const WINSOCK_VERSION_1_1: u32 = 0x101;
const AF_INET: u16 = 2;
const SOCK_STREAM: u32 = 1;
const SOCK_DGRAM: u32 = 2;
const IPPROTO_UDP: u32 = 0x11;
const SOL_SOCKET: u32 = 0xffff;
const SO_DEBUG: u32 = 1;
const SO_REUSEADDR: u32 = 4;
const SO_KEEPALIVE: u32 = 8;
const SO_BROADCAST: u32 = 0x20;
const SO_LINGER: u32 = 0x80;
const SO_SNDBUF: u32 = 0x1001;
const SO_RCVBUF: u32 = 0x1002;
const FIONBIO: u32 = 0x8004_667e;
const WSAEWOULDBLOCK: i32 = 0x2733;
/// The send and receive buffer size every stream socket gets.
const TCPIP_SOCKET_BUFFER: u32 = 15000;

unsafe fn ws_socket(family: u32, kind: u32, protocol: u32) -> u32 {
    let f: unsafe extern "system" fn(u32, u32, u32) -> u32 = core::mem::transmute(FN_WS_SOCKET);
    f(family, kind, protocol)
}

/// `setsockopt(s, SOL_SOCKET, name, value, 4)`: every option the TcpIp sets is a 4-byte socket-level one.
unsafe fn ws_setsockopt(s: u32, name: u32, value: usize) -> i32 {
    let f: unsafe extern "system" fn(u32, u32, u32, usize, i32) -> i32 = core::mem::transmute(FN_WS_SETSOCKOPT);
    f(s, SOL_SOCKET, name, value, 4)
}

unsafe fn ws_ioctlsocket(s: u32, command: u32, argument: *mut u32) -> i32 {
    let f: unsafe extern "system" fn(u32, u32, *mut u32) -> i32 = core::mem::transmute(FN_WS_IOCTLSOCKET);
    f(s, command, argument)
}

unsafe fn ws_bind(s: u32, address: usize, length: i32) -> i32 {
    let f: unsafe extern "system" fn(u32, usize, i32) -> i32 = core::mem::transmute(FN_WS_BIND);
    f(s, address, length)
}

unsafe fn ws_listen(s: u32, backlog: i32) -> i32 {
    let f: unsafe extern "system" fn(u32, i32) -> i32 = core::mem::transmute(FN_WS_LISTEN);
    f(s, backlog)
}

unsafe fn ws_connect(s: u32, address: usize, length: i32) -> i32 {
    let f: unsafe extern "system" fn(u32, usize, i32) -> i32 = core::mem::transmute(FN_WS_CONNECT);
    f(s, address, length)
}

unsafe fn ws_accept(s: u32) -> u32 {
    let f: unsafe extern "system" fn(u32, usize, usize) -> u32 = core::mem::transmute(FN_WS_ACCEPT);
    f(s, 0, 0)
}

unsafe fn ws_closesocket(s: u32) -> i32 {
    let f: unsafe extern "system" fn(u32) -> i32 = core::mem::transmute(FN_WS_CLOSESOCKET);
    f(s)
}

unsafe fn ws_send(s: u32, data: usize, length: i32) -> i32 {
    let f: unsafe extern "system" fn(u32, usize, i32, i32) -> i32 = core::mem::transmute(FN_WS_SEND);
    f(s, data, length, 0)
}

unsafe fn ws_recv(s: u32, buffer: usize, length: i32) -> i32 {
    let f: unsafe extern "system" fn(u32, usize, i32, i32) -> i32 = core::mem::transmute(FN_WS_RECV);
    f(s, buffer, length, 0)
}

unsafe fn ws_sendto(s: u32, data: usize, length: i32, to: usize, to_length: u32) -> i32 {
    let f: unsafe extern "system" fn(u32, usize, i32, i32, usize, u32) -> i32 = core::mem::transmute(FN_WS_SENDTO);
    f(s, data, length, 0, to, to_length)
}

unsafe fn ws_recvfrom(s: u32, buffer: usize, length: i32, from: usize, from_length: *mut i32) -> i32 {
    let f: unsafe extern "system" fn(u32, usize, i32, i32, usize, *mut i32) -> i32 = core::mem::transmute(FN_WS_RECVFROM);
    f(s, buffer, length, 0, from, from_length)
}

unsafe fn ws_wsa_startup(version: u32, data: usize) -> i32 {
    let f: unsafe extern "system" fn(u32, usize) -> i32 = core::mem::transmute(FN_WS_WSA_STARTUP);
    f(version, data)
}

unsafe fn ws_wsa_cleanup() -> i32 {
    let f: unsafe extern "system" fn() -> i32 = core::mem::transmute(FN_WS_WSA_CLEANUP);
    f()
}

unsafe fn ws_wsa_get_last_error() -> i32 {
    let f: unsafe extern "system" fn() -> i32 = core::mem::transmute(FN_WS_WSA_GET_LAST_ERROR);
    f()
}

unsafe fn ws_gethostbyaddr(address: usize, length: i32, family: u32) -> usize {
    let f: unsafe extern "system" fn(usize, i32, u32) -> usize = core::mem::transmute(FN_WS_GETHOSTBYADDR);
    f(address, length, family)
}

/// Fills a sockaddr_in from a dotted quad: `sscanf(text, "%d.%d.%d.%d")` into four ints whose low bytes become the
/// address, family AF_INET, port `htons(port)`; the last 8 bytes are left alone. Returns the sscanf count. The
/// original's four ints are uninitialised stack (in `parse_ipv4_address` the first one is the text's own argument
/// slot): when the text has fewer than four numbers the missing bytes are stack residue; the copy writes 0 there.
unsafe fn fill_ipv4_sockaddr(address: usize, text: usize, port: u32) -> i32 {
    let sscanf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let mut b = [0i32; 4];
    let fields = sscanf(text, IPV4_FORMAT, &mut b[0] as *mut i32, &mut b[1] as *mut i32, &mut b[2] as *mut i32, &mut b[3] as *mut i32);
    for k in 0..4 {
        write_u8(address + 4 + k, b[k] as u8); // tel47_1: the address bytes in order
    }
    write_u16(address, AF_INET);
    write_u16(address + 2, (port as u16).to_be());
    fields
}

/// Closes the socket held in a u32 slot when it is open and marks the slot −1.
unsafe fn tcpip_close_socket_at(at: usize) {
    let s = read_u32(at);
    if s != INVALID_SOCKET {
        ws_closesocket(s);
        write_u32(at, INVALID_SOCKET);
    }
}

/// The address of connection slot `slot` (a short) in the connection table.
unsafe fn tcpip_connection_at(this: usize, slot: i16) -> usize {
    (read_usize(this + TCPIP_CONNECTIONS) as isize + slot as isize * 4) as usize
}

/// `TcpIp::close_connection`'s body: with a connection table, the slot's socket closed and marked −1.
unsafe fn tcpip_close_slot(this: usize, slot: i16) {
    if read_usize(this + TCPIP_CONNECTIONS) != 0 {
        tcpip_close_socket_at(tcpip_connection_at(this, slot));
    }
}

/// The inline start of `open_listen_sockets`, `connect_to_host` and `open_broadcast_sockets`: Winsock already
/// started, true (no test); otherwise WSAStartup(1.1) into the object's WSADATA, the started flag, and the loopback
/// test (`TcpIp::test_loopback_connection`); false when either fails (the flag stays set after a failed test).
unsafe fn tcpip_ensure_winsock(this: usize) -> bool {
    if read_u32(this + tcpip_object::WINSOCK_STARTED) != 0 {
        return true;
    }
    if ws_wsa_startup(WINSOCK_VERSION_1_1, this + tcpip_object::WSA_DATA) != 0 {
        return false;
    }
    write_u32(this + tcpip_object::WINSOCK_STARTED, 1);
    let test: unsafe extern "thiscall" fn(usize) -> u32 = tcpip_fn(FN_TCPIP_TEST_LOOPBACK_CONNECTION);
    test(this) != 0
}

/// The common tail of a stream socket's options: keep-alive and debug with `option` (whatever it holds), then
/// `option` = 15000 for the send and receive buffers. The caller's `option` keeps 15000 afterwards.
unsafe fn tcpip_tune_stream_socket(s: u32, option: &mut u32) {
    ws_setsockopt(s, SO_KEEPALIVE, option as *mut u32 as usize);
    ws_setsockopt(s, SO_DEBUG, option as *mut u32 as usize);
    *option = TCPIP_SOCKET_BUFFER;
    ws_setsockopt(s, SO_SNDBUF, option as *mut u32 as usize);
    ws_setsockopt(s, SO_RCVBUF, option as *mut u32 as usize);
}

/// Whether `n` u32 socket slots from `at` are all −1 (the comparers' plan).
unsafe fn tcpip_all_closed(at: usize, n: usize) -> bool {
    (0..n).all(|k| read_u32(at + 4 * k) == INVALID_SOCKET)
}

/// Rule: a dotted quad text becomes an IPv4 socket address (family AF_INET, port 0).
///
/// `fill_ipv4_sockaddr(out, text, 0)`; the last 8 bytes of `out` are not touched. Defect of the original: a text
/// with fewer than four numbers leaves stack residue in the missing address bytes (the first one is the low byte of
/// the text's own pointer); the copy writes 0.
/// Original: `parse_ipv4_address` (0x8ae0d0, tcpip.cpp, thiscall with `this` unused, ret 8).
pub(crate) unsafe extern "thiscall" fn tcpip_parse_ipv4_address(_this: usize, out: usize, text: usize) {
    covered!("tel47_1 address parsed");
    fill_ipv4_sockaddr(out, text, 0);
}

/// Rule: the four address bytes of an IPv4 socket address are written as a dotted quad text.
///
/// `sprintf(out, "%d.%d.%d.%d", the bytes at +4..+7)`.
/// Original: `format_ipv4_address` (0x8ae140, tcpip.cpp, thiscall with `this` unused, ret 8).
pub(crate) unsafe extern "thiscall" fn tcpip_format_ipv4_address(_this: usize, address: usize, out: usize) {
    covered!("tel47_1 address formatted");
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    sprintf(out, IPV4_FORMAT, read_u8(address + 4) as u32, read_u8(address + 5) as u32, read_u8(address + 6) as u32,
        read_u8(address + 7) as u32); // tel47_1: the four bytes
}

/// Rule: a TcpIp object starts with Winsock not started, every listen and broadcast socket closed and a connection
/// table of the given length with every slot free.
///
/// `tcpip_object::CONNECTION_COUNT` = `count` (a short), `WINSOCK_STARTED` = 0, the 20 broadcast and 20 listen
/// sockets −1, the table = `operator new(count × 4)` (`TCPIP_CONNECTIONS`), each slot −1. Returns `this`. Defect of the
/// original: when the allocation fails it shows the "tcpip.cpp 124" box (left button let go) and then still fills the
/// null table (a crash).
/// Original: `TcpIp::TcpIp` (0x8ae180, tcpip.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn tcpip_construct(this: usize, count: u32) -> usize {
    use tcpip_object as t;
    covered!("tel47_1 TcpIp built");
    write_i16(this + t::CONNECTION_COUNT, count as i16);
    write_u32(this + t::WINSOCK_STARTED, 0);
    for k in 0..t::SOCKET_SLOTS {
        write_u32(this + t::LISTEN_SOCKETS + 4 * k, INVALID_SOCKET);
        write_u32(this + t::BROADCAST_SOCKETS + 4 * k, INVALID_SOCKET);
    }
    let n = read_i16(this + t::CONNECTION_COUNT) as i32;
    let table = exe::crt_operator_new((n << 2) as u32);
    write_usize(this + TCPIP_CONNECTIONS, table);
    if table == 0 {
        tel26_error_box(TCPIP_CPP, 0x7c);
    }
    for k in 0..n.max(0) as usize {
        write_u32(table + 4 * k, INVALID_SOCKET);
    }
    this
}

/// Rule: Winsock is started once (version 1.1) and TCP/IP is reported working when a loopback connection works.
///
/// Not started: WSAStartup(1.1, `WSA_DATA`); an error returns 0, otherwise `WINSOCK_STARTED` = 1. Then, started or
/// not, `TcpIp::test_loopback_connection` decides (1 or 0).
/// Original: `TcpIp::start_winsock` (0x8ae260, tcpip.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn tcpip_start_winsock(this: usize) -> u32 {
    covered!("tel47_1 Winsock started");
    if read_u32(this + tcpip_object::WINSOCK_STARTED) == 0 {
        if ws_wsa_startup(WINSOCK_VERSION_1_1, this + tcpip_object::WSA_DATA) != 0 {
            return 0;
        }
        write_u32(this + tcpip_object::WINSOCK_STARTED, 1);
    }
    let test: unsafe extern "thiscall" fn(usize) -> u32 = tcpip_fn(FN_TCPIP_TEST_LOOPBACK_CONNECTION);
    (test(this) != 0) as u32
}

/// Rule: every socket of the object is closed and Winsock shut down when this object started it.
///
/// Each connection slot (`tcpip_close_slot`, the table length re-read each turn), the 20 listen sockets, the 20
/// broadcast sockets (open ones closed and marked −1); `WINSOCK_STARTED`: WSACleanup and the flag 0.
/// Original: `TcpIp::close_all_sockets` (0x8ae2a0, tcpip.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn tcpip_close_all_sockets(this: usize) {
    use tcpip_object as t;
    covered!("tel47_1 every socket closed");
    let mut k = 0i16;
    while k < read_i16(this + t::CONNECTION_COUNT) {
        tcpip_close_slot(this, k);
        k += 1;
    }
    for k in 0..t::SOCKET_SLOTS {
        tcpip_close_socket_at(this + t::LISTEN_SOCKETS + 4 * k);
    }
    for k in 0..t::SOCKET_SLOTS {
        tcpip_close_socket_at(this + t::BROADCAST_SOCKETS + 4 * k);
    }
    if read_u32(this + t::WINSOCK_STARTED) != 0 {
        ws_wsa_cleanup();
        write_u32(this + t::WINSOCK_STARTED, 0);
    }
}

/// Rule: one connection slot's socket is closed and the slot freed.
///
/// `tcpip_close_slot(slot)` (a short; nothing without a connection table or with the slot already free).
/// Original: `TcpIp::close_connection` (0x8ae340, tcpip.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn tcpip_close_connection(this: usize, slot: u32) {
    covered!("tel47_1 connection closed");
    tcpip_close_slot(this, slot as i16);
}

/// Rule: the twenty UDP broadcast sockets are closed.
///
/// Each open one of `tcpip_object::BROADCAST_SOCKETS` closed and marked −1.
/// Original: `TcpIp::close_broadcast_sockets` (0x8aec60, tcpip.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn tcpip_close_broadcast_sockets(this: usize) {
    covered!("tel47_1 broadcast sockets closed");
    for k in 0..tcpip_object::SOCKET_SLOTS {
        tcpip_close_socket_at(this + tcpip_object::BROADCAST_SOCKETS + 4 * k);
    }
}

/// Rule: an address is accepted only when a reverse name look-up finds its host.
///
/// `gethostbyaddr(address + 4, 4, AF_INET)` not null: 1, else 0.
/// Original: `TcpIp::can_resolve_address` (0x8aee00, tcpip.cpp, thiscall with `this` unused, ret 4).
pub(crate) unsafe extern "thiscall" fn tcpip_can_resolve_address(_this: usize, address: usize) -> u32 {
    covered!("tel47_1 address resolved");
    (ws_gethostbyaddr(address + 4, 4, AF_INET as u32) != 0) as u32
}

/// Rule: TCP/IP works when a socket listening on 127.0.0.1, port 6667, accepts a connection from a second socket.
///
/// Winsock not started: 0. The server socket (stream): `SO_REUSEADDR` with an option of 1, non-blocking (`FIONBIO`),
/// `SO_LINGER` off; any failure closes it and returns 0; then `tcpip_tune_stream_socket`. Bound to 127.0.0.1:6667
/// (`fill_ipv4_sockaddr` of "127.0.0.1" and `NETWORK_PORT`) and listening with the connection table length as the
/// backlog (a failure closes it, 0). The client socket (stream; invalid: the server closed, 0): `SO_REUSEADDR` with
/// the option (15000 now), then `connect` to the same address. Both closed; 1 when the client connected, else 0.
/// Original: `TcpIp::test_loopback_connection` (0x8af290, tcpip.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn tcpip_test_loopback_connection(this: usize) -> u32 {
    if read_u32(this + tcpip_object::WINSOCK_STARTED) == 0 {
        return 0;
    }
    let server = ws_socket(AF_INET as u32, SOCK_STREAM, 0);
    if server == INVALID_SOCKET {
        return 0;
    }
    covered!("tel47_1 loopback test");
    let mut option = 1u32;
    let mut non_blocking = 1u32;
    let linger = [0u16; 2];
    if ws_setsockopt(server, SO_REUSEADDR, &mut option as *mut u32 as usize) == -1
        || ws_ioctlsocket(server, FIONBIO, &mut non_blocking) == -1
        || ws_setsockopt(server, SO_LINGER, linger.as_ptr() as usize) == -1
    {
        ws_closesocket(server);
        return 0;
    }
    tcpip_tune_stream_socket(server, &mut option);
    let mut address = [0u8; 16];
    let a = address.as_mut_ptr() as usize;
    fill_ipv4_sockaddr(a, LOOPBACK_ADDRESS, NETWORK_PORT);
    if ws_bind(server, a, 16) == -1 || ws_listen(server, read_i16(this + tcpip_object::CONNECTION_COUNT) as i32) == -1 {
        ws_closesocket(server);
        return 0;
    }
    let client = ws_socket(AF_INET as u32, SOCK_STREAM, 0);
    if client == INVALID_SOCKET {
        ws_closesocket(server);
        return 0;
    }
    let connected = ws_setsockopt(client, SO_REUSEADDR, &mut option as *mut u32 as usize) != -1 && {
        fill_ipv4_sockaddr(a, LOOPBACK_ADDRESS, NETWORK_PORT);
        ws_connect(client, a, 16) != -1 // tel47_1: the loopback connection made
    };
    ws_closesocket(server);
    ws_closesocket(client);
    connected as u32
}

/// Comparer for `tcpip_parse_ipv4_address`: side by side. The copy fills a local copy of `out`; family, port and the
/// address bytes sscanf read must match (the others are the original's stack residue).
pub(crate) unsafe extern "thiscall" fn compare_tcpip_parse_ipv4_address(this: usize, out: usize, text: usize) {
    let mut copy = [0u8; 16];
    core::ptr::copy(out as *const u8, copy.as_mut_ptr(), 16);
    let fields = fill_ipv4_sockaddr(copy.as_mut_ptr() as usize, text, 0);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, out, text);
    let n = 4 + fields.clamp(0, 4) as usize;
    let original = core::slice::from_raw_parts(out as *const u8, 16);
    SCOREBOARD.record(original[..n] == copy[..n], || format!("parse_ipv4_address({:?}): original {:02x?}, copy {:02x?}",
        String::from_utf8_lossy(&c_string_bytes(text)), original, copy));
}

/// Comparer for `tcpip_format_ipv4_address`: side by side, the copy into a local text.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_format_ipv4_address(this: usize, address: usize, out: usize) {
    let mut copy = [0u8; 32];
    tcpip_format_ipv4_address(this, address, copy.as_mut_ptr() as usize);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%)(this, address, out);
    let (o, c) = (c_string_bytes(out), c_string_bytes(copy.as_ptr() as usize));
    SCOREBOARD.record(o == c, || format!("format_ipv4_address({:02x?}): original {:?}, copy {:?}",
        core::slice::from_raw_parts((address + 4) as *const u8, 4), String::from_utf8_lossy(&o), String::from_utf8_lossy(&c)));
}

// simplification: the table is allocated once. Original only; the plan: the length, Winsock not started, the 40
// sockets and every table slot −1.
/// Comparer for `tcpip_construct`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_construct(this: usize, count: u32) -> usize {
    use tcpip_object as t;
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, count);
    let n = read_i16(this + t::CONNECTION_COUNT);
    let table = read_usize(this + TCPIP_CONNECTIONS);
    SCOREBOARD.record(r == this && n == count as i16 && read_u32(this + t::WINSOCK_STARTED) == 0
        && tcpip_all_closed(this + t::BROADCAST_SOCKETS, 2 * t::SOCKET_SLOTS) && table != 0 && tcpip_all_closed(table, n.max(0) as usize),
        || format!("TcpIp::TcpIp({}): returned {:#x}, length {}, started {}, table {:#x}", count as i16, r, n,
            read_u32(this + t::WINSOCK_STARTED), table));
    r
}

// simplification: WSAStartup and the loopback sockets do not run twice. Original only; proven in Rust mode (a network game).
/// Comparer for `tcpip_start_winsock`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_start_winsock(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I3%%)(this)
}

// simplification: the sockets close once. Original only; the plan: every slot and socket −1, Winsock not started.
/// Comparer for `tcpip_close_all_sockets`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_close_all_sockets(this: usize) {
    use tcpip_object as t;
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this);
    let table = read_usize(this + TCPIP_CONNECTIONS);
    let n = read_i16(this + t::CONNECTION_COUNT).max(0) as usize;
    SCOREBOARD.record((table == 0 || tcpip_all_closed(table, n)) && tcpip_all_closed(this + t::BROADCAST_SOCKETS, 2 * t::SOCKET_SLOTS)
        && read_u32(this + t::WINSOCK_STARTED) == 0,
        || format!("TcpIp::close_all_sockets: started {}, table {:#x}", read_u32(this + t::WINSOCK_STARTED), table));
}

// simplification: the socket closes once. Original only; the plan: the slot −1.
/// Comparer for `tcpip_close_connection`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_close_connection(this: usize, slot: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I5%%)(this, slot);
    let table = read_usize(this + TCPIP_CONNECTIONS);
    SCOREBOARD.record(table == 0 || read_u32(tcpip_connection_at(this, slot as i16)) == INVALID_SOCKET,
        || format!("TcpIp::close_connection({}): slot left {:#x}", slot as i16, read_u32(tcpip_connection_at(this, slot as i16))));
}

// simplification: the sockets close once. Original only; the plan: the 20 broadcast sockets −1.
/// Comparer for `tcpip_close_broadcast_sockets`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_close_broadcast_sockets(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this);
    SCOREBOARD.record(tcpip_all_closed(this + tcpip_object::BROADCAST_SOCKETS, tcpip_object::SOCKET_SLOTS),
        || format!("TcpIp::close_broadcast_sockets: {:x?}", core::slice::from_raw_parts((this + tcpip_object::BROADCAST_SOCKETS) as *const u32, 20)));
}

// simplification: a name look-up goes to the network. Original only; proven in Rust mode (a network game).
/// Comparer for `tcpip_can_resolve_address`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_can_resolve_address(this: usize, address: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I7%%)(this, address)
}

// simplification: the loopback sockets do not run twice. Original only; proven in Rust mode (a network game).
/// Comparer for `tcpip_test_loopback_connection`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_test_loopback_connection(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I8%%)(this)
}
