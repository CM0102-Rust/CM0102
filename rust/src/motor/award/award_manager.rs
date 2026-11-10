use super::*;

/// Raises the reputations of an award winner: for a player, the world, home and current reputations rise by a fraction
/// of the award's reputations (up to 10000 and at least them); for a non-player, they rise up to the award floors and
/// the board confidence gains 1000.
///
/// `award_raise_winner_reputation(person, club, national_rep, world_rep, for_player)`, called for the
/// player-of-the-month slot (`set_winner`) and for the yearly and national side awards. A player (+0x61) in a player award:
/// world (+0xd) + 5% of the award's world reputation, up to 10000 and at least that value; home (+9): for the club of
/// his own nation (+0x53 = +0x1a of the person), + 10% of the national reputation and at least that value; otherwise +
/// 2.5% of the sum and at least the average; current (+0xb) + 10% of the national reputation, at least that value; puts
/// the player in the clubs' lists (`add_player_to_club_shortlists`, with 1) if the list manager exists. A non-player (+0x69) in a non-player
/// award: the three rise to the floors (+0xc world, +8 by the club's nation, +0xa national) and, with a club, the board
/// confidence (+6 of the 73-byte row) + 1000, between 6500 and 10000. Only the low words of the reputations count.
/// Original: `award_raise_winner_reputation` (0x412f00, award_manager.cpp, cdecl).
///
/// [pt] { 0x412f00 } (cdecl, award_manager.cpp): `award_raise_winner_reputation(pessoa, clube, rep_nacional, rep_mundial,
/// de_jogador)`, pela vaga do vencedor do mês (0x7671c0) e pelos prêmios do ano e das seleções. Jogador (+0x61) num
/// prêmio de jogador: mundial (+0xd) + 5% da mundial do prêmio, até 10000 e pelo menos ela; nacional (+9): do clube da
/// nação dele (+0x53 = +0x1a da pessoa), + 10% da nacional e pelo menos ela; senão + 2,5% da soma e pelo menos a média;
/// atual (+0xb) + 10% da nacional, pelo menos ela; põe o jogador nas listas dos clubes (0x842c00, com 1) se houver o
/// gerente das listas. Não jogador (+0x69) num prêmio que não é de jogador: as três sobem até os pisos (+0xc mundial,
/// +8 pela nação do clube, +0xa nacional) e, com clube, a confiança da diretoria (+6 da linha de 73 bytes) + 1000, entre
/// 6500 e 10000. Só as palavras baixas das reputações valem.
pub(crate) unsafe extern "C" fn award_raise_winner_reputation(person: usize, club: usize, nation: u32, world: u32, player: u32) {
    let person = Staff(person);
    let club = Club(club);
    let nation = nation as u16 as i16;
    let world = world as u16 as i16;
    let p = PlayerSheet(person.player());
    if !p.is_null() && player != 0 {
        crate::comparison::branch("cpw_6 player award");
        add_to_word(p.addr() + player_sheet::WORLD_REPUTATION, ftol(world as i32 as f64 * read_f64(rdata::F64_0_05))); // cpw6: the worldwide increase;
        if p.world_reputation() > 10000 {
            p.set_world_reputation(10000);
        }
        if p.world_reputation() <= world { // cpw6: o piso mundial
            p.set_world_reputation(world);
        }
        if !club.is_null() && club.nation() == person.nation() { // cpw6: the club of the player’s nation
            add_to_word(p.addr() + player_sheet::HOME_REPUTATION, ftol(nation as i32 as f64 * read_f64(rdata::F64_0_1)));
            clamp_word(p.addr() + player_sheet::HOME_REPUTATION, nation);
        } else {
            add_to_word(p.addr() + player_sheet::HOME_REPUTATION, ftol((nation as i32 + world as i32) as f64 * read_f64(rdata::F64_0_025)));
            if p.home_reputation() > 10000 {
                p.set_home_reputation(10000);
            }
            let floor = nation as i32 / 2 + world as i32 / 2;
            if (p.home_reputation() as i32) <= floor {
                p.set_home_reputation(floor as i16);
            }
        }
        add_to_word(p.addr() + player_sheet::CURRENT_REPUTATION, ftol(nation as i32 as f64 * read_f64(rdata::F64_0_1)));
        clamp_word(p.addr() + player_sheet::CURRENT_REPUTATION, nation);
        let lists = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
        if lists != 0 && !MONTH_AWARD_DRY_RUN {
            exe::shortlist_manager_add_player_to_club_shortlists(lists, person, 1);
        }
        return;
    }
    let np = person.non_player();
    if np == 0 || player != 0 {
        return;
    }
    if read_i16(np + non_player::WORLD_REPUTATION) <= world {
        write_i16(np + non_player::WORLD_REPUTATION, world);
    }
    if !club.is_null() && club.nation() == person.nation() {
        if read_i16(np + non_player::HOME_REPUTATION) <= nation {
            write_i16(np + non_player::HOME_REPUTATION, nation);
        }
    } else {
        let floor = nation as i32 / 2 + world as i32 / 2;
        if (read_i16(np + non_player::HOME_REPUTATION) as i32) <= floor {
            write_i16(np + non_player::HOME_REPUTATION, floor as i16);
        }
    }
    if read_i16(np + non_player::CURRENT_REPUTATION) <= nation {
        write_i16(np + non_player::CURRENT_REPUTATION, nation);
    }
    if club.is_null() {
        return;
    }
    let row = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(club.addr()) as usize).wrapping_mul(73)).wrapping_add(6);
    let v = read_i16(row).wrapping_add(1000);
    let v = if v < 1 {
        1
    } else if v > 10000 {
        10000
    } else {
        v
    };
    write_i16(row, if v < 6500 { 6500 } else { v });
}
