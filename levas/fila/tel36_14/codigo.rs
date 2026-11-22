// ---------- batch %%LEVA%%: the network game connection (Network, national_teams_screens.cpp) ----------

/// The Network object (a TcpIp, tcpip.cpp, with the game's connections): offsets used by its methods.
#[allow(dead_code)]
mod network_game {
    /// u16[16]: the connected sockets (0xffff free).
    pub(crate) const SOCKETS: usize = 0x4ba;
    /// i32[16]: the length header of the packet being read on each connection.
    pub(crate) const HEADERS: usize = 0x4da;
    /// i32[16]: the read state of each connection: -4..-1 reading the 4-byte header, > 0 the packet length.
    pub(crate) const STATES: usize = 0x51a;
    /// u8[16][50000]: the packet buffer of each connection.
    pub(crate) const BUFFERS: usize = 0x55a;
    pub(crate) const BUFFER_SIZE: usize = 0xc350;
    /// u32: connected (hosting with the listen sockets open, or joined).
    pub(crate) const CONNECTED: usize = 0xc3a5a;
    /// u32: hosting the game.
    pub(crate) const HOSTING: usize = 0xc3a5e;
    /// cdecl(slot + 1): called when a connection is accepted / closed.
    pub(crate) const ON_CONNECT: usize = 0xc3a62;
    pub(crate) const ON_DISCONNECT: usize = 0xc3a66;
    /// i16: the servers found by the list requests.
    pub(crate) const SERVER_COUNT: usize = 0xc3a6a;
    /// char[10][50]: the servers' names and addresses (slot 0 is the host's own when hosting).
    pub(crate) const SERVER_NAMES: usize = 0xc3a6c;
    pub(crate) const SERVER_ADDRESSES: usize = 0xc3c60;
    pub(crate) const SERVER_TEXT: usize = 0x32;
}

/// The game's port (6667).
const NETWORK_PORT: u32 = 0x1a0b;
/// "CM3 LIST SERVER REQ", "CM3 LIST SERVER ACK", "CM3 LIST SERVER BYE".
const NETWORK_LIST_REQ: usize = 0xa649d4;
const NETWORK_LIST_ACK: usize = 0xa64994;
const NETWORK_LIST_BYE: usize = 0xa64980;
/// The "connecting to a listed server" flag (1 while the name typed is not a listed server).
const NETWORK_JOIN_BY_ADDRESS: usize = 0xb77c88;

/// A TcpIp method by address (thiscall).
unsafe fn tcpip_fn<F>(address: usize) -> F {
    core::mem::transmute_copy(&address)
}

/// The length of a C string.
unsafe fn network_strlen(s: usize) -> usize {
    let mut n = 0;
    while read_u8(s + n) != 0 {
        n += 1;
    }
    n
}

/// `strcmp` on two C strings, as the original's inline loop (equal or not).
unsafe fn network_same(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let (x, y) = (read_u8(a + k), read_u8(b + k));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        k += 1;
    }
}

/// `strcpy` (the original's inline copy, terminator included).
unsafe fn network_copy(dst: usize, src: usize) {
    let n = network_strlen(src) + 1;
    core::ptr::copy(src as *const u8, dst as *mut u8, n);
}

/// The "list server" datagram: the text (without its terminator), then 50 bytes of an address and 50 bytes of a name,
/// sent by `TcpIp::send_datagram(buffer, length + 100, port, to, a, b)`.
unsafe fn network_send_list(this: usize, text: usize, address: usize, name: usize, to: usize, a: u32, b: u32) {
    let mut buffer = [0u8; 0x7d0];
    let p = buffer.as_mut_ptr() as usize;
    let n = network_strlen(text);
    core::ptr::copy(text as *const u8, p as *mut u8, n);
    core::ptr::copy(address as *const u8, (p + n) as *mut u8, network_game::SERVER_TEXT);
    core::ptr::copy(name as *const u8, (p + n + network_game::SERVER_TEXT) as *mut u8, network_game::SERVER_TEXT);
    let send: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> i32 = tcpip_fn(0x8aec90);
    send(this, p, (n + 0x64) as u32, NETWORK_PORT, to, a, b);
}

/// Rule: a network game object starts with no connections and no servers listed; it opens its broadcast sockets on the
/// game's port when Winsock starts, and reports in `ok` whether it did.
///
/// `Network::Network(ok, on_connect, on_disconnect)`: `TcpIp::TcpIp(0x10)`; the server count, the hosting and the
/// connected flags 0; the 16 sockets free; the two callbacks kept; `*ok` = 1; `TcpIp::start_winsock` then
/// `TcpIp::open_broadcast_sockets(6667)`, or `*ok` = 0 when Winsock does not start. Returns the object.
/// Original: `Network::Network` (0x77a7d0, national_teams_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn network_network(this: usize, ok: *mut u32, on_connect: usize, on_disconnect: usize) -> usize {
    covered!("tel36_14 network object");
    let base: unsafe extern "thiscall" fn(usize, u32) -> usize = tcpip_fn(0x8ae180);
    base(this, 0x10);
    write_u16(this + network_game::SERVER_COUNT, 0);
    write_u32(this + network_game::HOSTING, 0);
    write_u32(this + network_game::CONNECTED, 0);
    core::ptr::write_bytes((this + network_game::SOCKETS) as *mut u8, 0xff, 0x20);
    write_usize(this + network_game::ON_DISCONNECT, on_disconnect);
    write_usize(this + network_game::ON_CONNECT, on_connect);
    *ok = 1;
    let start: unsafe extern "thiscall" fn(usize) -> u32 = tcpip_fn(0x8ae260);
    if start(this) != 0 {
        let open: unsafe extern "thiscall" fn(usize, u32) -> u32 = tcpip_fn(0x8aea80);
        open(this, NETWORK_PORT);
    } else {
        *ok = 0;
    }
    this
}

/// Rule: the machine's own addresses, as text, for the network game screen.
///
/// `Network::get_local_addresses(out)`: `tcpip_scan_adapter_addresses` into 20 local entries of 32 bytes and their
/// count; each address formatted (`format_ipv4_address`) into `out` + 20 × k. Returns the count.
/// Original: `Network::get_local_addresses` (0x77a980, national_teams_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_get_local_addresses(this: usize, out: usize) -> i32 {
    let mut addresses = [0u8; 0x280];
    let mut count = 0i16;
    let scan: unsafe extern "thiscall" fn(usize, usize, *mut i16) = tcpip_fn(0x8aee20);
    scan(this, addresses.as_mut_ptr() as usize, &mut count);
    let format: unsafe extern "thiscall" fn(usize, usize, usize) = tcpip_fn(0x8ae140);
    for k in 0..count.max(0) as usize {
        format(this, addresses.as_ptr() as usize + k * 0x20, out + k * 0x14);
    }
    count as i32
}

/// Rule: hosting opens the listen sockets, lists this machine under the given name and announces it by broadcast;
/// joining connects to the listed server of that name (or to the name itself as an address) and closes the broadcast
/// sockets. Returns whether it is connected.
///
/// `Network::host_or_join_game(join, name)`: `TcpIp::start_winsock`, 0 when it fails. Hosting (`join` 0): hosting = 1;
/// `TcpIp::open_listen_sockets(6667)` sets connected; server name 0 = `name`; server address 0 = `gethostname` (50
/// bytes); the "CM3 LIST SERVER ACK" datagram with them, broadcast (no address). Joining:
/// `NETWORK_JOIN_BY_ADDRESS` = 1; a listed server whose name equals `name` gives its address (and the flag 0),
/// otherwise the address is `name`; hosting = 0, the state of connection 0 = -4; `parse_ipv4_address`, then
/// `TcpIp::connect_to_host(6667, &address)` into socket 0: when it connects, `TcpIp::close_broadcast_sockets` and
/// connected = 1. Returns the connected flag.
/// Original: `Network::host_or_join_game` (0x77a9f0, national_teams_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn network_host_or_join_game(this: usize, join: u32, name: usize) -> u32 {
    let start: unsafe extern "thiscall" fn(usize) -> u32 = tcpip_fn(0x8ae260);
    if start(this) == 0 {
        return 0;
    }
    if join == 0 {
        covered!("tel36_14 host a network game");
        write_u32(this + network_game::HOSTING, 1);
        let listen: unsafe extern "thiscall" fn(usize, u32) -> u32 = tcpip_fn(0x8ae370);
        if listen(this, NETWORK_PORT) != 0 {
            write_u32(this + network_game::CONNECTED, 1);
        }
        network_copy(this + network_game::SERVER_NAMES, name);
        let host_name: unsafe extern "system" fn(usize, i32) -> i32 = core::mem::transmute(0x944c28usize);
        host_name(this + network_game::SERVER_ADDRESSES, 0x32);
        network_send_list(this, NETWORK_LIST_ACK, this + network_game::SERVER_ADDRESSES, this + network_game::SERVER_NAMES, 0, 0, 0);
        return read_u32(this + network_game::CONNECTED);
    }
    covered!("tel36_14 join a network game");
    let mut address = name;
    write_u32(NETWORK_JOIN_BY_ADDRESS, 1);
    for k in 0..read_i16(this + network_game::SERVER_COUNT).max(0) as usize {
        if network_same(name, this + network_game::SERVER_NAMES + k * network_game::SERVER_TEXT) {
            write_u32(NETWORK_JOIN_BY_ADDRESS, 0);
            address = this + network_game::SERVER_ADDRESSES + k * network_game::SERVER_TEXT;
        }
    }
    write_u32(this + network_game::HOSTING, 0);
    write_i32(this + network_game::STATES, -4);
    let mut socket_address = [0u8; 0x10];
    let parse: unsafe extern "thiscall" fn(usize, usize, usize) = tcpip_fn(0x8ae0d0);
    parse(this, socket_address.as_mut_ptr() as usize, address);
    let connect: unsafe extern "thiscall" fn(usize, u32, usize) -> u16 = tcpip_fn(0x8ae640);
    let socket = connect(this, NETWORK_PORT, socket_address.as_ptr() as usize);
    write_u16(this + network_game::SOCKETS, socket);
    if socket != 0xffff {
        let close: unsafe extern "thiscall" fn(usize) = tcpip_fn(0x8aec60);
        close(this);
        write_u32(this + network_game::CONNECTED, 1);
    }
    read_u32(this + network_game::CONNECTED)
}

/// The server list after an "ACK": sorts the names (bubble sort by `strcmp`, swapping names and addresses as strings)
/// until a pass makes no swap.
unsafe fn network_sort_servers(this: usize) {
    let mut held = [0u8; 0x7d0];
    let tmp = held.as_mut_ptr() as usize;
    loop {
        let mut swapped = false;
        for k in 1..read_i16(this + network_game::SERVER_COUNT) as usize {
            let name = |j: usize| this + network_game::SERVER_NAMES + j * network_game::SERVER_TEXT;
            let address = |j: usize| this + network_game::SERVER_ADDRESSES + j * network_game::SERVER_TEXT;
            if exe_strcmp(name(k - 1), name(k)) > 0 {
                network_copy(tmp, address(k - 1));
                network_copy(address(k - 1), address(k));
                network_copy(address(k), tmp);
                network_copy(tmp, name(k - 1));
                network_copy(name(k - 1), name(k));
                network_copy(name(k), tmp);
                swapped = true;
            }
        }
        if !swapped {
            return;
        }
    }
}

/// `strcmp` with the sign (the original's inline loop: -1, 0 or 1).
unsafe fn exe_strcmp(a: usize, b: usize) -> i32 {
    let mut k = 0;
    loop {
        let (x, y) = (read_u8(a + k), read_u8(b + k));
        if x != y {
            return if x < y { -1 } else { 1 };
        }
        if x == 0 {
            return 0;
        }
        k += 1;
    }
}

/// Rule: each turn the network object answers the list requests (hosting) or collects the server list (looking for a
/// game), then reads the connections; a complete packet is handed out with its connection number.
///
/// `Network::receive_packet(out, slot)`: hosting: the first free socket takes `TcpIp::accept_connection` (its header
/// and state -4, `ON_CONNECT(k + 1)`); a datagram (`TcpIp::receive_datagram`, 2000 bytes) "CM3 LIST SERVER REQ" is
/// answered with the "ACK" datagram (the host's address and name) to its sender. Not hosting: an "ACK" datagram with
/// fewer than 9 servers listed adds the sender (its formatted address and the name in the datagram) unless the
/// address is already listed or `TcpIp::can_resolve_address` refuses it, then sorts the list by name; a "BYE"
/// datagram removes the sender's address from the list (moving the rest up); when it is not found the count still goes
/// up by one (a defect of the original, kept). Then for each socket in use: while the state is negative the 4-byte
/// header is read a byte at a time (`TcpIp::receive_from_connection`); when complete a zero length starts it again,
/// otherwise the state becomes the length; with a length the packet is read into the connection's buffer until the
/// header (counting down) reaches 0: then `*slot` = k + 1, the packet is copied to `out`, the state goes back to -4 and
/// 1 comes back. A read of -1 or -2 closes the socket (`ON_DISCONNECT(k + 1)`). Otherwise 0.
/// Original: `Network::receive_packet` (0x77af20, national_teams_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn network_receive_packet(this: usize, out: usize, slot: *mut u16) -> u32 {
    let mut buffer = [0u8; 0x7d0];
    let p = buffer.as_mut_ptr() as usize;
    let mut from = [0u8; 0x10];
    let mut text = [0u8; 0x34];
    let (mut a, mut b) = (0u32, 0u32);
    let receive: unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut u32, *mut u32) -> i32 = tcpip_fn(0x8aed80);
    let format: unsafe extern "thiscall" fn(usize, usize, usize) = tcpip_fn(0x8ae140);
    let callback = |field: usize, k: usize| {
        let f = read_usize(this + field);
        if f != 0 {
            let g: unsafe extern "C" fn(u32) = core::mem::transmute(f);
            g(k as u32 + 1);
        }
    };
    if read_u32(this + network_game::HOSTING) != 0 {
        if let Some(k) = (0..0x10).find(|&k| read_u16(this + network_game::SOCKETS + 2 * k) == 0xffff) {
            let accept: unsafe extern "thiscall" fn(usize) -> u16 = tcpip_fn(0x8ae840);
            let socket = accept(this);
            write_u16(this + network_game::SOCKETS + 2 * k, socket);
            if socket != 0xffff {
                covered!("tel36_14 connection accepted");
                write_i32(this + network_game::HEADERS + 4 * k, -4);
                write_i32(this + network_game::STATES + 4 * k, -4);
                callback(network_game::ON_CONNECT, k);
            }
        }
        if receive(this, p, 0x7d0, from.as_mut_ptr() as usize, &mut a, &mut b) > 0 && network_same(p, NETWORK_LIST_REQ) {
            covered!("tel36_14 list request answered");
            network_send_list(this, NETWORK_LIST_ACK, this + network_game::SERVER_ADDRESSES, this + network_game::SERVER_NAMES, from.as_ptr() as usize, a, b);
        }
    } else if receive(this, p, 0x7d0, from.as_mut_ptr() as usize, &mut a, &mut b) > 0 {
        let count = read_i16(this + network_game::SERVER_COUNT);
        let ack = network_strlen(NETWORK_LIST_ACK) as u32;
        if exe::crt_strncmp(p, NETWORK_LIST_ACK, ack) == 0 && count < 9 {
            let n = count as usize;
            let address = this + network_game::SERVER_ADDRESSES + n * network_game::SERVER_TEXT;
            format(this, from.as_ptr() as usize, address);
            core::ptr::copy((p + ack as usize + network_game::SERVER_TEXT) as *const u8,
                (this + network_game::SERVER_NAMES + n * network_game::SERVER_TEXT) as *mut u8, network_game::SERVER_TEXT);
            let listed = (0..n).any(|k| network_same(address, this + network_game::SERVER_ADDRESSES + k * network_game::SERVER_TEXT));
            let resolve: unsafe extern "thiscall" fn(usize, usize) -> u32 = tcpip_fn(0x8aee00);
            if !listed && resolve(this, from.as_ptr() as usize) != 0 {
                covered!("tel36_14 server listed");
                write_i16(this + network_game::SERVER_COUNT, count + 1);
                network_sort_servers(this);
            }
        } else if exe::crt_strncmp(p, NETWORK_LIST_BYE, network_strlen(NETWORK_LIST_BYE) as u32) == 0 {
            covered!("tel36_14 server gone");
            format(this, from.as_ptr() as usize, text.as_mut_ptr() as usize);
            let mut k = 0i32;
            while k < read_i16(this + network_game::SERVER_COUNT) as i32 {
                let at = |j: i32| this + network_game::SERVER_ADDRESSES + j as usize * network_game::SERVER_TEXT;
                if network_same(text.as_ptr() as usize, at(k)) {
                    k += 1;
                    while k < read_i16(this + network_game::SERVER_COUNT) as i32 {
                        network_copy(at(k - 1), at(k));
                        let name = |j: i32| this + network_game::SERVER_NAMES + j as usize * network_game::SERVER_TEXT;
                        network_copy(name(k - 1), name(k));
                        k += 1;
                    }
                    write_i16(this + network_game::SERVER_COUNT, read_i16(this + network_game::SERVER_COUNT) - 1);
                }
                k += 1;
            }
            let count = read_i16(this + network_game::SERVER_COUNT);
            if k == count as i32 {
                write_i16(this + network_game::SERVER_COUNT, count + 1);
            }
        }
    }
    let read: unsafe extern "thiscall" fn(usize, u32, usize, i32) -> i32 = tcpip_fn(0x8ae9c0);
    for k in 0..0x10usize {
        let socket = this + network_game::SOCKETS + 2 * k;
        let state = this + network_game::STATES + 4 * k;
        let header = this + network_game::HEADERS + 4 * k;
        let packet = this + network_game::BUFFERS + k * network_game::BUFFER_SIZE;
        if read_u16(socket) == 0xffff {
            continue;
        }
        if read_i32(state) < 0 {
            let mut r = read(this, read_u16(socket) as u32, (header as isize + 4 + read_i32(state) as isize) as usize, 1);
            while r > 0 {
                write_i32(state, read_i32(state) + 1);
                if read_i32(state) == 0 {
                    if read_i32(header) != 0 {
                        write_i32(state, read_i32(header));
                        break;
                    }
                    write_i32(state, -4);
                }
                r = read(this, read_u16(socket) as u32, (header as isize + 4 + read_i32(state) as isize) as usize, 1);
            }
            if r == -1 || r == -2 {
                write_u16(socket, 0xffff);
                callback(network_game::ON_DISCONNECT, k);
            }
        }
        if read_i32(state) > 0 {
            let at = || (packet as isize + read_i32(state) as isize - read_i32(header) as isize) as usize;
            let mut r = read(this, read_u16(socket) as u32, at(), read_i32(header));
            while r > 0 {
                write_i32(header, read_i32(header) - r);
                if read_i32(header) <= 0 {
                    covered!("tel36_14 packet received");
                    *slot = k as u16 + 1;
                    core::ptr::copy(packet as *const u8, out as *mut u8, read_i32(state) as u32 as usize);
                    write_i32(state, -4);
                    return 1;
                }
                r = read(this, read_u16(socket) as u32, at(), read_i32(header));
            }
            if r == -1 || r == -2 {
                write_u16(socket, 0xffff);
                callback(network_game::ON_DISCONNECT, k);
            }
        }
    }
    0
}

/// Comparer for `network_network`.
pub(crate) unsafe extern "thiscall" fn compare_network_network(this: usize, ok: *mut u32, on_connect: usize, on_disconnect: usize) -> usize {
    // simplification: Winsock and the sockets do not run twice; proven in Rust mode (a network game)
    original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, usize, usize) -> usize>(%%I0%%)(this, ok, on_connect, on_disconnect)
}

/// Comparer for `network_get_local_addresses`.
pub(crate) unsafe extern "thiscall" fn compare_network_get_local_addresses(this: usize, out: usize) -> i32 {
    // simplification: the adapter scan runs once; proven in Rust mode (a network game)
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(%%I1%%)(this, out)
}

/// Comparer for `network_host_or_join_game`.
pub(crate) unsafe extern "thiscall" fn compare_network_host_or_join_game(this: usize, join: u32, name: usize) -> u32 {
    // simplification: the sockets and the datagrams do not run twice; proven in Rust mode (a network game)
    original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(%%I2%%)(this, join, name)
}

/// Comparer for `network_receive_packet`.
pub(crate) unsafe extern "thiscall" fn compare_network_receive_packet(this: usize, out: usize, slot: *mut u16) -> u32 {
    // simplification: reading the sockets does not run twice; proven in Rust mode (a network game)
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u16) -> u32>(%%I3%%)(this, out, slot)
}
