// ---------- batch %%LEVA%%: the network object's life and its packets: destroying a TcpIp and a Network, closing a player's connection, sending a packet and building and destroying a network buffer (national_teams_screens.cpp) ----------

/// The TcpIp methods the Network calls (tcpip.cpp, thiscall): close every socket and shut Winsock down, close one
/// connection slot (short), send on one connection slot (slot short, data, length) -> bytes sent or −2 when closed.
const FN_TCPIP_CLOSE_ALL_SOCKETS: usize = 0x8ae2a0;
const FN_TCPIP_CLOSE_CONNECTION: usize = 0x8ae340;
const FN_TCPIP_SEND_TO_CONNECTION: usize = 0x8ae8d0;
/// "E:\dev\CM3\cm3 00-01\si\code\network.CPP" (the error boxes of this file).
const NETWORK_CPP: usize = 0xa649a8;
/// The TcpIp's connection table pointer (+0, freed by the destructors) and the Network's connection slots as seen from
/// a 1-based player number (slot k is at `network_game::SOCKETS` + 2 × (k − 1)).
const TCPIP_CONNECTIONS: usize = 0x0;
const NETWORK_PLAYER_SLOTS: usize = 0x4b8;

/// The TcpIp base's destructor body: every socket closed (`TcpIp::close_all_sockets`) and the connection table freed
/// (`operator delete`, when set; the pointer is left as it was).
unsafe fn tcpip_close_and_free(this: usize) {
    let close: unsafe extern "thiscall" fn(usize) = tcpip_fn(FN_TCPIP_CLOSE_ALL_SOCKETS);
    close(this);
    let table = read_usize(this + TCPIP_CONNECTIONS);
    if table != 0 {
        exe::crt_operator_delete(table);
    }
}

/// Rule: a TcpIp object is destroyed by closing all its sockets and freeing its connection table.
///
/// `tcpip_close_and_free`. Only reached from the unwind handlers of `Network::Network` (an exception while the TcpIp
/// base is built).
/// Original: `TcpIp::~TcpIp` (0x77a870, national_teams_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn tcpip_destructor(this: usize) {
    covered!("tel40_2 TcpIp destroyed");
    tcpip_close_and_free(this);
}

/// Rule: a Network object is destroyed; when it hosts a game it first says goodbye to the list servers ("CM3 LIST SERVER
/// BYE" with its address), then all its sockets are closed and its connection table freed.
///
/// Hosting (`network_game::HOSTING`): the datagram "CM3 LIST SERVER BYE" (`NETWORK_LIST_BYE`, without its terminator)
/// followed by the 50 bytes of server address 0 (`network_game::SERVER_ADDRESSES`), sent by `TcpIp::send_datagram`
/// with the length of the text + 100, the game's port and no address (broadcast). The original's buffer is a stack
/// local, so the last 50 bytes sent are whatever its stack held; the copy sends zeros there. Then
/// `TcpIp::close_all_sockets` and the TcpIp base's destructor (`tcpip_close_and_free`: closing them again and freeing
/// the table).
/// Original: `Network::~Network` (0x77a890, national_teams_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn network_destructor(this: usize) {
    covered!("tel40_2 Network destroyed");
    if read_u32(this + network_game::HOSTING) != 0 {
        let mut buffer = [0u8; 0x7d0];
        let p = buffer.as_mut_ptr() as usize;
        let n = network_strlen(NETWORK_LIST_BYE);
        core::ptr::copy(NETWORK_LIST_BYE as *const u8, p as *mut u8, n);
        core::ptr::copy((this + network_game::SERVER_ADDRESSES) as *const u8, (p + n) as *mut u8, network_game::SERVER_TEXT);
        let send: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> i32 = tcpip_fn(FN_TCPIP_SEND_DATAGRAM);
        send(this, p, (n + 0x64) as u32, NETWORK_PORT, 0, 0, 0); // tel40_2: the goodbye datagram
    }
    let close: unsafe extern "thiscall" fn(usize) = tcpip_fn(FN_TCPIP_CLOSE_ALL_SOCKETS);
    close(this);
    tcpip_close_and_free(this);
}

/// Rule: the connection of one of the sixteen players (numbered 1 to 16) is closed; any other number does nothing.
///
/// For `player` 1..16 (a short): `TcpIp::close_connection` with the connection slot of that player
/// (`NETWORK_PLAYER_SLOTS` + 2 × player, a word); the slot is not marked free here.
/// Original: `Network::release_slot_socket` (0x77ac00, national_teams_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_release_slot_socket(this: usize, player: u32) {
    let k = player as i16;
    if k <= 0 || k > 0x10 {
        return;
    }
    covered!("tel40_2 close a player's connection");
    let close: unsafe extern "thiscall" fn(usize, u32) = tcpip_fn(FN_TCPIP_CLOSE_CONNECTION);
    close(this, read_u16(this + NETWORK_PLAYER_SLOTS + 2 * k as usize) as u32);
}

/// Sends one packet on a connection slot: its length as 4 single-byte sends (little-endian), then the data; true
/// unless the data send returned −2 (the connection closed).
unsafe fn network_send_on(this: usize, slot: u16, data: usize, length: u32) -> bool {
    let send: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> i32 = tcpip_fn(FN_TCPIP_SEND_TO_CONNECTION);
    let header = length.to_le_bytes();
    for k in 0..4 {
        send(this, slot as u32, header.as_ptr() as usize + k, 1);
    }
    send(this, slot as u32, data, length) != -2 // tel40_2: only a closed connection fails
}

/// Rule: a packet goes to one player's connection, or to every connection for player 0 when hosting, preceded by its
/// 4-byte length; a connection found closed is freed (and the leave callback told, when hosting). Returns whether the
/// (last) send worked.
///
/// A length of 0 or less: the box `network.CPP 211` and 0. A length of 50000 or more: 0. Hosting
/// (`network_game::HOSTING`): player 0 (the low word) walks the 16 connection slots (`network_game::SOCKETS`); each one
/// in use (not 0xffff) gets `network_send_on`; a closed one is marked free (0xffff) and the leave callback
/// (`network_game::ON_DISCONNECT`, when set) gets the slot + 1. Returns the result of the last slot in use (0 with
/// none). Another player: his slot (`NETWORK_PLAYER_SLOTS` + 2 × player); free, 0; otherwise `network_send_on`, closed
/// -> freed and the callback gets `player`. A client: slot 0 the same way, freed without the callback.
/// Original: `Network::send_packet` (0x77ac30, national_teams_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn network_send_packet(this: usize, data: usize, length: i32, player: u32) -> u32 {
    use network_game as g;
    if length <= 0 {
        tel26_error_box(NETWORK_CPP, 0xd3);
        return 0;
    }
    if length >= 0xc350 {
        return 0;
    }
    covered!("tel40_2 send a packet");
    let callback = |arg: u32| {
        let f = read_usize(this + g::ON_DISCONNECT);
        if f != 0 {
            let f: unsafe extern "C" fn(u32) = core::mem::transmute(f);
            f(arg);
        }
    };
    if read_u32(this + g::HOSTING) == 0 {
        let slot = read_u16(this + g::SOCKETS);
        if slot == 0xffff {
            return 0;
        }
        let ok = network_send_on(this, slot, data, length as u32);
        if !ok {
            write_u16(this + g::SOCKETS, 0xffff);
        }
        return ok as u32;
    }
    if player as u16 != 0 {
        let at = (this as isize + NETWORK_PLAYER_SLOTS as isize + player as i16 as isize * 2) as usize;
        let slot = read_u16(at);
        if slot == 0xffff {
            return 0;
        }
        let ok = network_send_on(this, slot, data, length as u32);
        if !ok {
            write_u16(at, 0xffff);
            callback(player);
        }
        return ok as u32;
    }
    covered!("tel40_2 send a packet to everyone");
    let mut ok = false;
    for k in 0..0x10usize {
        let slot = read_u16(this + g::SOCKETS + 2 * k);
        if slot == 0xffff {
            continue;
        }
        ok = network_send_on(this, slot, data, length as u32);
        if !ok {
            write_u16(this + g::SOCKETS + 2 * k, 0xffff); // tel40_2: the closed connection freed
            callback(k as u32 + 1);
        }
    }
    ok as u32
}

/// Rule: a network buffer starts empty with the given capacity and its own storage; with no memory the game stops.
///
/// Position 0, capacity = `capacity`, the storage `operator new(capacity)`; null: the box `network.CPP 562`,
/// `game_fatal_cleanup` (0x5cc7d0), the empty routine 0x5a8f60 and `exit(-1)`. Returns the buffer.
/// Original: `NetworkBuffer::NetworkBuffer` (0x77b630, national_teams_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_construct(this: usize, capacity: u32) -> usize {
    write_u32(this + network_buffer::POS, 0);
    write_u32(this + network_buffer::CAPACITY, capacity);
    let data = exe::crt_operator_new(capacity);
    write_usize(this + network_buffer::DATA, data);
    if data == 0 {
        tel26_error_box(NETWORK_CPP, 0x232);
        let cleanup: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
        cleanup();
        let noop: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
        noop();
        let exit: unsafe extern "C" fn(i32) = core::mem::transmute(FN_CRT_EXIT);
        exit(-1);
    }
    this
}

/// Rule: a network buffer is destroyed by freeing its storage, when it has one, and clearing the pointer.
/// Original: `NetworkBuffer::~NetworkBuffer` (0x77b6e0, national_teams_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn network_buffer_destruct(this: usize) {
    let data = read_usize(this + network_buffer::DATA);
    if data != 0 {
        exe::crt_operator_delete(data);
    }
    write_usize(this + network_buffer::DATA, 0); // tel40_2: the pointer cleared
}

// simplification: closing sockets and freeing memory do not run twice. Original only.
/// Comparer for `tcpip_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

// simplification: the datagram, the sockets and the table do not run twice. Original only.
/// Comparer for `network_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_network_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

// simplification: closing a socket does not run twice. Original only.
/// Comparer for `network_release_slot_socket`.
pub(crate) unsafe extern "thiscall" fn compare_network_release_slot_socket(this: usize, player: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I2%%)(this, player)
}

// simplification: a packet is not sent twice. Original only; the plan: hosting and sending to everyone, a failed send
// leaves its slot free.
/// Comparer for `network_send_packet`.
pub(crate) unsafe extern "thiscall" fn compare_network_send_packet(this: usize, data: usize, length: i32, player: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(%%I3%%)(this, data, length, player)
}

// simplification: the storage is allocated once (and a failure exits). Original only; the plan: position 0, the
// capacity kept, the storage set, the buffer returned.
/// Comparer for `network_buffer_construct`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_construct(this: usize, capacity: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, capacity);
    SCOREBOARD.record(r == this && read_u32(this + network_buffer::POS) == 0 && read_u32(this + network_buffer::CAPACITY) == capacity
        && read_usize(this + network_buffer::DATA) != 0, || format!("NetworkBuffer::NetworkBuffer({}): returned {:#x}, buffer {:x?}", capacity, r,
        core::slice::from_raw_parts(this as *const u8, 12)));
    r
}

// simplification: the storage is freed once. Original only; the plan: the pointer cleared.
/// Comparer for `network_buffer_destruct`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_destruct(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%)(this);
    SCOREBOARD.record(read_usize(this + network_buffer::DATA) == 0, || format!("NetworkBuffer::~NetworkBuffer: storage {:#x} left", read_usize(this + network_buffer::DATA)));
}
