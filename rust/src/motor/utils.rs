use super::*;

/// Builds the CM3_TEMP folder path in `[0xde279c]` and deletes its files matching the pattern, in groups of 100.
///
/// `utils_clear_temp_folder(pattern)`: builds the temporary folder in `[0xde279c]` (the `CM3_TEMP` variable via
/// 0x90bc50 + "\", if present, and the text of `[0xad9c64]`, copied with a limit by 0x90cf20) and deletes (`remove`
/// 0x94643a) its files that match the pattern, in groups of 100 names (the search is closed before deleting and redone
/// while the group fills up).
/// Original: `utils_clear_temp_folder` (0x90cd50, utils.cpp, cdecl).
///
/// [pt] { 0x90cd50 } (cdecl, utils.cpp): `utils_clear_temp_folder(padrao)`: monta em [0xde279c] a pasta
/// temporária (a variável `CM3_TEMP` pela 0x90bc50 + "\", se houver, e o texto de [0xad9c64], copiado
/// com limite pela 0x90cf20) e apaga (`remove` 0x94643a) os arquivos dela que casam com o padrão,
/// em grupos de 100 nomes (a busca é fechada antes de apagar e refeita enquanto o grupo enche).
pub(crate) unsafe extern "C" fn utils_clear_temp_folder(default: usize) {
    let join_path: unsafe extern "C" fn(*mut u8, usize, *const u8, usize, usize) = core::mem::transmute(FN_CRT_MAKEPATH);
    let first: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> i32 = core::mem::transmute(FN_FILE_FIND_FIRST);
    let next: unsafe extern "C" fn(*mut u8) -> i32 = core::mem::transmute(FN_FILE_FIND_NEXT);
    let encerrar: unsafe extern "C" fn(*mut u8) -> i32 = core::mem::transmute(FN_FILE_FIND_CLOSE);
    let erase: unsafe extern "C" fn(*mut u8) -> i32 = core::mem::transmute(FN_CRT_REMOVE);
    crate::comparison::branch("save limpar temporarios");
    let global = 0xde279c as *mut u8;
    *global = 0;
    let variavel = exe::utils_get_folder_for_key(0x987da0);
    let mut sufixo = [0u8; 0x100];
    exe::utils_copy_path_limited(0xad9c64, sufixo.as_mut_ptr());
    if variavel != 0 && *(variavel as *const u8) != 0 {
        zipdir_c_strcpy(global, variavel as *const u8);
        zipdir_c_strcat(global, 0x9c3730 as *const u8);
    }
    zipdir_c_strcat(global, sufixo.as_ptr());
    let mut folder = [0u8; 0x104];
    zipdir_c_strcpy(folder.as_mut_ptr(), global);
    let mut search = [0u8; 0x104];
    let mut found = [0u8; 0x114];
    let mut nomes = [0u8; 0x6400];
    loop {
        join_path(search.as_mut_ptr(), 0, folder.as_ptr(), default, 0);
        let mut n = 0usize;
        if first(search.as_mut_ptr(), found.as_mut_ptr(), 0) == 0 {
            loop {
                zipdir_c_strcpy(nomes.as_mut_ptr().add(0x100 * n), found.as_ptr().add(0x10));
                n += 1;
                if n == 100 || next(found.as_mut_ptr()) != 0 {
                    break;
                }
            }
        }
        encerrar(found.as_mut_ptr());
        for k in 0..n {
            join_path(search.as_mut_ptr(), 0, folder.as_ptr(), nomes.as_ptr().add(0x100 * k) as usize, 0);
            erase(search.as_mut_ptr());
        }
        if n != 100 {
            break;
        }
    }
}
