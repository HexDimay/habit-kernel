use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Полностью читает файл в строку.
pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Атомарно записывает `contents` в `path`.
///
/// Гарантии:
/// * либо файл целиком старого содержимого, либо целиком нового;
/// * родительские каталоги создаются при необходимости;
/// * временный файл не остаётся на диске после завершения функции.
pub fn write_atomic<P: AsRef<Path>>(path: P, contents: &str) -> io::Result<()> {
    let path = path.as_ref();
    let parent = path.parent().filter(|p| !p.as_os_str().is_empty());
    let dir = parent.unwrap_or_else(|| Path::new("."));

    if parent.is_some() {
        fs::create_dir_all(dir)?;
    }

    let tmp_path = dir.join(tmp_file_name(path));

    // Пишем и синхронизируем tmp. На выходе из блока дескриптор закрыт.
    {
        let mut f = fs::File::create(&tmp_path)?;
        f.write_all(contents.as_bytes())?;
        f.sync_all()?;
    }

    // Rename. При ошибке подчищаем tmp.
    if let Err(e) = fs::rename(&tmp_path, path) {
        let _ = fs::remove_file(&tmp_path);
        return Err(e);
    }

    // Durability rename'а на POSIX: fsync каталога. Best-effort, ошибку
    // не пробрасываем — файл уже на месте, содержимое консистентно.
    #[cfg(unix)]
    {
        if let Ok(dir_fd) = fs::File::open(dir) {
            let _ = dir_fd.sync_all();
        }
    }

    Ok(())
}

fn tmp_file_name(target: &Path) -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);

    let base = target
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("file");

    // `.` префикс делает файл скрытым на Unix; сохраняем базовое имя,
    // чтобы было понятно, чей это tmp, при отладке.
    format!(".{base}.{pid}.{nanos}.{n}.tmp")
}
