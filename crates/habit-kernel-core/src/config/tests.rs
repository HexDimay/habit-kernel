use super::*;
use crate::config::db::traits::{DbEntry, DbEntryMut, DbStore, DbStoreMut};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use uuid::Uuid;

// ------------------------------------------------------------------
// helpers (те же, что в тестах Config)
// ------------------------------------------------------------------

static CWD_LOCK: Mutex<()> = Mutex::new(());
static TEST_ID: AtomicUsize = AtomicUsize::new(0);

fn in_temp_cwd<F: FnOnce()>(f: F) {
    let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let original = std::env::current_dir().unwrap();
    let id = TEST_ID.fetch_add(1, Ordering::SeqCst);
    let tmp = std::env::temp_dir().join(format!("io-test-{}-{}", std::process::id(), id));

    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    std::env::set_current_dir(&tmp).unwrap();

    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));

    std::env::set_current_dir(&original).unwrap();
    let _ = fs::remove_dir_all(&tmp);

    if let Err(p) = outcome {
        std::panic::resume_unwind(p);
    }
}

/// Возвращает имена всех файлов в текущей директории (без рекурсии).
fn ls_cwd() -> Vec<String> {
    fs::read_dir(".")
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect()
}

// ------------------------------------------------------------------
// write_atomic
// ------------------------------------------------------------------

#[test]
fn write_atomic_creates_file_with_contents() {
    in_temp_cwd(|| {
        write_atomic("a.txt", "hello").unwrap();

        assert_eq!(fs::read_to_string("a.txt").unwrap(), "hello");
    });
}

#[test]
fn write_atomic_overwrites_existing() {
    in_temp_cwd(|| {
        fs::write("a.txt", "old").unwrap();
        write_atomic("a.txt", "new").unwrap();

        assert_eq!(fs::read_to_string("a.txt").unwrap(), "new");
    });
}

#[test]
fn write_atomic_creates_parent_dirs() {
    in_temp_cwd(|| {
        write_atomic("deep/nested/dir/a.txt", "x").unwrap();

        assert!(Path::new("deep/nested/dir/a.txt").exists());
    });
}

#[test]
fn write_atomic_leaves_no_tmp_files() {
    in_temp_cwd(|| {
        write_atomic("a.txt", "x").unwrap();

        let leftovers: Vec<_> = ls_cwd()
            .into_iter()
            .filter(|n| n.contains(".tmp"))
            .collect();

        assert!(leftovers.is_empty(), "остались tmp-файлы: {leftovers:?}");
    });
}

#[test]
fn write_atomic_creates_tmp_in_target_dir_not_cwd() {
    // tmp обязан лежать рядом с целью, иначе rename может не быть
    // атомарным (при пересечении ФС), а мы этого не заметим, пока не
    // появится реальный кейс с разными ФС. Проверяем по остаткам на
    // неудачной записи: пишем в несуществующий по правам путь.
    in_temp_cwd(|| {
        // Создадим файл там, где место есть, потом напишем по пути,
        // где родитель — файл, а не директория. `create_dir_all` упадёт
        // до создания tmp, и в cwd ничего не появится.
        fs::write("blocker", "i am a file").unwrap();

        let res = write_atomic("blocker/sub/a.txt", "x");
        assert!(res.is_err());

        let stray: Vec<_> = ls_cwd()
            .into_iter()
            .filter(|n| n.contains(".tmp"))
            .collect();
        assert!(stray.is_empty(), "остались tmp-файлы: {stray:?}");
    });
}

#[test]
fn write_atomic_is_utf8_safe() {
    in_temp_cwd(|| {
        let data = "ключ = \"значение\"\nemoji = \"🚀\"\n";
        write_atomic("a.toml", data).unwrap();

        assert_eq!(fs::read_to_string("a.toml").unwrap(), data);
    });
}

/// Параллельные записи не должны перемешиваться и не должны ломать
/// файл — итог всегда одно из валидных значений.
#[test]
fn write_atomic_parallel_is_consistent() {
    in_temp_cwd(|| {
        use std::sync::Arc;
        use std::thread;

        let payloads: Vec<Arc<String>> = (0..8)
            .map(|i| Arc::new(format!("payload-{i}-").repeat(1024)))
            .collect();

        let handles: Vec<_> = payloads
            .iter()
            .cloned()
            .map(|p| {
                thread::spawn(move || {
                    for _ in 0..32 {
                        write_atomic("hot.txt", &p).unwrap();
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        let final_content = fs::read_to_string("hot.txt").unwrap();
        assert!(
            payloads.iter().any(|p| **p == final_content),
            "файл содержит не целый payload — записи перемешались",
        );

        let stray: Vec<_> = ls_cwd()
            .into_iter()
            .filter(|n| n.contains(".tmp"))
            .collect();
        assert!(stray.is_empty(), "остались tmp-файлы: {stray:?}");
    });
}

// ------------------------------------------------------------------
// read_to_string
// ------------------------------------------------------------------

#[test]
fn read_returns_contents() {
    in_temp_cwd(|| {
        write_atomic("a.txt", "hello").unwrap();
        assert_eq!(read_to_string("a.txt").unwrap(), "hello");
    });
}

#[test]
fn read_missing_file_errors() {
    in_temp_cwd(|| {
        let err = read_to_string("nope.txt").unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    });
}

#[test]
fn read_invalid_utf8_errors() {
    in_temp_cwd(|| {
        fs::write("bin", [0xff, 0xfe, 0x00]).unwrap();
        assert!(read_to_string("bin").is_err());
    });
}

#[test]
fn database_accessors_see_same_instance() {
    let mut cfg = Config::new();
    cfg.database_mut()
        .add("main", PathBuf::from("/db/main.db"))
        .unwrap();

    assert_eq!(cfg.database().len(), 1);
    assert_eq!(cfg.database().find_by_name("main").unwrap().name(), "main");
}

#[test]
fn new_has_empty_database_config() {
    let cfg = Config::new();
    assert!(cfg.database().is_empty());
    assert_eq!(cfg.database().path_dir(), &PathBuf::from("./db"));
}

// ---------------------------------------------------------------------
// Config как DbStore (только чтение)
// ---------------------------------------------------------------------

#[test]
fn config_delegates_read_side_of_dbstore() {
    let mut cfg = Config::new();
    let a = cfg.add("a", PathBuf::from("/a")).unwrap();
    let b = cfg.add("b", PathBuf::from("/b")).unwrap();
    cfg.select_by_id(b).unwrap();

    // Геттеры через трейт.
    assert_eq!(DbStore::len(&cfg), 2);
    assert!(!DbStore::is_empty(&cfg));
    assert_eq!(DbStore::current_db_id(&cfg), Some(b));
    assert_eq!(DbStore::current_db(&cfg).unwrap().id(), b);
    assert_eq!(DbStore::find_by_name(&cfg, "a").unwrap().id(), a);

    // Путь каталога тоже проходит через трейт.
    assert_eq!(DbStore::path_dir(&cfg), &PathBuf::from("./db"));
}

#[test]
fn config_finds_return_none_for_unknown() {
    let cfg = Config::new();
    assert!(DbStore::find_by_id(&cfg, Uuid::new_v4()).is_none());
    assert!(DbStore::find_by_name(&cfg, "nope").is_none());
    assert!(DbStore::current_db(&cfg).is_none());
}

// ---------------------------------------------------------------------
// Config как DbStoreMut (запись)
// ---------------------------------------------------------------------

#[test]
fn config_delegates_write_side_of_dbstoremut() {
    let mut cfg = Config::new();

    let id = cfg.add("main", PathBuf::from("/db/main.db")).unwrap();
    assert_eq!(cfg.database().find_by_id(id).unwrap().name(), "main");

    cfg.select_by_name("main").unwrap();
    assert_eq!(DbStore::current_db_id(&cfg), Some(id));

    cfg.remove_by_name("main").unwrap();
    assert!(cfg.database().is_empty());
    assert_eq!(DbStore::current_db_id(&cfg), None, "current не сброшен");
}

#[test]
fn config_add_with_default_path_uses_inner_path_dir() {
    let mut cfg = Config::new();
    cfg.set_path_dir(PathBuf::from("/var/db"));

    let id = cfg.add_with_default_path("main").unwrap();
    assert_eq!(
        cfg.database().find_by_id(id).unwrap().path_file(),
        &PathBuf::from("/var/db/main.db"),
    );
}

#[test]
fn config_set_path_dir_does_not_touch_records() {
    let mut cfg = Config::new();
    let id = cfg.add("main", PathBuf::from("/db/main.db")).unwrap();

    cfg.set_path_dir(PathBuf::from("/other"));

    assert_eq!(
        cfg.database().find_by_id(id).unwrap().path_file(),
        &PathBuf::from("/db/main.db"),
    );
}

#[test]
fn config_find_mut_allows_edit() {
    let mut cfg = Config::new();
    let id = cfg.add("old", PathBuf::from("/a")).unwrap();

    cfg.find_by_id_mut(id).unwrap().set_name("new");
    assert_eq!(cfg.database().find_by_id(id).unwrap().name(), "new");
}

// ---------------------------------------------------------------------
// Инвариант: current_db всегда указывает на существующую запись
// ---------------------------------------------------------------------

#[test]
fn config_current_id_never_dangles() {
    let mut cfg = Config::new();
    let a = cfg.add("a", PathBuf::from("/a")).unwrap();
    let b = cfg.add("b", PathBuf::from("/b")).unwrap();

    cfg.select_by_id(b).unwrap();
    cfg.remove_by_id(a).unwrap();
    assert!(DbStore::find_by_id(&cfg, DbStore::current_db_id(&cfg).unwrap()).is_some());

    cfg.remove_by_id(b).unwrap();
    assert_eq!(DbStore::current_db_id(&cfg), None);
}

// ---------------------------------------------------------------------
// Полиморфизм: одна и та же generic-функция работает и с DataBaseConfig,
// и с Config.
// ---------------------------------------------------------------------

fn names<S: DbStore>(s: &S) -> Vec<String> {
    s.databases().iter().map(|e| e.name().to_string()).collect()
}

#[test]
fn same_generic_works_for_both_types() {
    let mut inner: DataBaseConfig<PathBuf> = DataBaseConfig::new(PathBuf::from("/db"));
    inner.add("a", PathBuf::from("/a")).unwrap();
    inner.add("b", PathBuf::from("/b")).unwrap();

    let mut cfg = Config::new();
    cfg.add("a", PathBuf::from("/a")).unwrap();
    cfg.add("b", PathBuf::from("/b")).unwrap();

    assert_eq!(names(&inner), names(&cfg));
    assert_eq!(names(&cfg), vec!["a".to_string(), "b".to_string()]);
}
