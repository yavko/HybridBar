use crate::config::SYSINFO;

/// Replaces `find` with `replace` if found.
fn replace_if_present(content: &mut String, find: &str, replace: &str, found_any: &mut bool) {
    if content.contains(find) {
        *content = content.replace(find, replace);
        *found_any = true;
    }
}

/// Checks if the `content` contains any of the built-in aliases, then replaces it with the real
/// value.
pub fn use_aliases(content: &str) -> String {
    // TODO: Clean this up.
    if SYSINFO.is_none() {
        log!("System Info isn't available for this system, therefore aliases have been disabled.");
        return execute!(content);
    }

    let info = SYSINFO
        .as_ref()
        .expect("[ERROR] Failed accessing SYSINFO, this should not happen!");
    let mut found_any = false;
    let mut content = content.to_owned();
    replace_if_present(&mut content, "%username%", &info.username, &mut found_any);
    replace_if_present(&mut content, "%hostname%", &info.hostname, &mut found_any);
    replace_if_present(&mut content, "%shell%", &info.shell, &mut found_any);
    replace_if_present(&mut content, "%kernel%", &info.kernel, &mut found_any);
    replace_if_present(&mut content, "%used_mem%", &info.used_mem, &mut found_any);
    replace_if_present(&mut content, "%distro_id%", &info.distro_id, &mut found_any);
    replace_if_present(&mut content, "%total_mem%", &info.total_mem, &mut found_any);
    replace_if_present(
        &mut content,
        "%cached_mem%",
        &info.cached_mem,
        &mut found_any,
    );
    replace_if_present(
        &mut content,
        "%available_mem%",
        &info.available_mem,
        &mut found_any,
    );
    replace_if_present(&mut content, "%distro%", &info.distro_name, &mut found_any);
    replace_if_present(
        &mut content,
        "%distro_build_id%",
        &info.distro_build_id,
        &mut found_any,
    );

    if !found_any {
        // Couldn't find any aliases present, run execute.
        return execute!(&content);
    }

    content
}
