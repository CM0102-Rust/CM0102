use super::*;

/// Raises the reputations of an award winner: for a player, the world, home and current reputations rise by a fraction
/// of the award's reputations (up to 10000 and at least them); for a non-player, they rise up to the award floors and
/// the board confidence gains 1000.
///
/// `award_raise_winner_reputation(person, club, national_rep, world_rep, for_player)`, called for the
/// player-of-the-month slot (0x7671c0) and for the yearly and national side awards. A player (+0x61) in a player award:
/// world (+0xd) + 5% of the award's world reputation, up to 10000 and at least that value; home (+9): for the club of
/// his own nation (+0x53 = +0x1a of the person), + 10% of the national reputation and at least that value; otherwise +
/// 2.5% of the sum and at least the average; current (+0xb) + 10% of the national reputation, at least that value; puts
/// the player in the clubs' lists (0x842c00, with 1) if the list manager exists. A non-player (+0x69) in a non-player
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
pub(crate) unsafe extern "C" fn award_raise_winner_reputation(person: usize, clube: usize, nac: u32, mundo: u32, player: u32) {
    let nac = nac as u16 as i16;
    let mundo = mundo as u16 as i16;
    let p = read_usize(person + staff::PLAYER);
    if p != 0 && player != 0 {
        crate::comparison::branch("cpw_6 premio de jogador");
        add_to_word(p + player_sheet::WORLD_REPUTATION, match_pl_ftol(mundo as i32 as f64 * read_f64_at(0x967888))); // cpw6: o aumento mundial; 0x967888 = 0.05
        if read_i16(p + player_sheet::WORLD_REPUTATION) > 10000 {
            write_i16(p + player_sheet::WORLD_REPUTATION, 10000);
        }
        if read_i16(p + player_sheet::WORLD_REPUTATION) <= mundo { // cpw6: o piso mundial
            write_i16(p + player_sheet::WORLD_REPUTATION, mundo);
        }
        if clube != 0 && read_usize(clube + club::NATION) == read_usize(person + staff::NATION) { // cpw6: o clube da nação do jogador
            add_to_word(p + player_sheet::HOME_REPUTATION, match_pl_ftol(nac as i32 as f64 * read_f64_at(0x967880))); // 0x967880 = 0.1
            clamp_word(p + player_sheet::HOME_REPUTATION, nac);
        } else {
            add_to_word(p + player_sheet::HOME_REPUTATION, match_pl_ftol((nac as i32 + mundo as i32) as f64 * read_f64_at(0x967878))); // 0x967878 = 0.025
            if read_i16(p + player_sheet::HOME_REPUTATION) > 10000 {
                write_i16(p + player_sheet::HOME_REPUTATION, 10000);
            }
            let floor = nac as i32 / 2 + mundo as i32 / 2;
            if (read_i16(p + player_sheet::HOME_REPUTATION) as i32) <= floor {
                write_i16(p + player_sheet::HOME_REPUTATION, floor as i16);
            }
        }
        add_to_word(p + player_sheet::CURRENT_REPUTATION, match_pl_ftol(nac as i32 as f64 * read_f64_at(0x967880))); // 0x967880 = 0.1
        clamp_word(p + player_sheet::CURRENT_REPUTATION, nac);
        let lists = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
        if lists != 0 && !MONTH_AWARD_DRY_RUN {
            let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_ADD_PLAYER_TO_CLUB_SHORTLISTS);
            f(lists, person, 1);
        }
        return;
    }
    let np = read_usize(person + staff::NON_PLAYER);
    if np == 0 || player != 0 {
        return;
    }
    if read_i16(np + non_player::WORLD_REPUTATION) <= mundo {
        write_i16(np + non_player::WORLD_REPUTATION, mundo);
    }
    if clube != 0 && read_usize(clube + club::NATION) == read_usize(person + staff::NATION) {
        if read_i16(np + non_player::HOME_REPUTATION) <= nac {
            write_i16(np + non_player::HOME_REPUTATION, nac);
        }
    } else {
        let floor = nac as i32 / 2 + mundo as i32 / 2;
        if (read_i16(np + non_player::HOME_REPUTATION) as i32) <= floor {
            write_i16(np + non_player::HOME_REPUTATION, floor as i16);
        }
    }
    if read_i16(np + non_player::CURRENT_REPUTATION) <= nac {
        write_i16(np + non_player::CURRENT_REPUTATION, nac);
    }
    if clube == 0 {
        return;
    }
    let row = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(clube) as usize).wrapping_mul(73)).wrapping_add(6);
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
