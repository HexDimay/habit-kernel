use super::traits::*;
use super::*;
use std::path::PathBuf;
use uuid::Uuid;

fn cfg() -> DataBaseConfig<PathBuf> {
    DataBaseConfig::new(PathBuf::from("/data"))
}

// ------------------------------------------------------------------
// new / пустое состояние
// ------------------------------------------------------------------

#[test]
fn new_is_empty() {
    let c = cfg();

    assert!(c.is_empty());
    assert_eq!(c.len(), 0);
    assert_eq!(c.current_db_id(), None);
    assert!(c.current_db().is_none());
    assert!(c.databases().is_empty());
    assert_eq!(c.path_dir(), &PathBuf::from("/data"));
}

// ------------------------------------------------------------------
// add
// ------------------------------------------------------------------

#[test]
fn add_stores_record_and_returns_id() {
    let mut c = cfg();
    let id = c.add("main", PathBuf::from("/data/main.db")).unwrap();

    assert_eq!(c.len(), 1);
    let db = c.find_by_id(id).unwrap();
    assert_eq!(db.id(), id);
    assert_eq!(db.name(), "main");
    assert_eq!(db.path_file(), &PathBuf::from("/data/main.db"));
}

#[test]
fn add_generates_unique_ids() {
    let mut c = cfg();
    let a = c.add("a", PathBuf::from("/a")).unwrap();
    let b = c.add("b", PathBuf::from("/b")).unwrap();

    assert_ne!(a, b);
}

#[test]
fn add_rejects_duplicate_name() {
    let mut c = cfg();
    c.add("main", PathBuf::from("/a")).unwrap();

    let err = c.add("main", PathBuf::from("/b")).unwrap_err();
    assert!(matches!(err, DbConfigError::NameAlreadyExists(ref n) if n == "main"));
    assert_eq!(c.len(), 1, "запись всё же добавилась");
}

#[test]
fn add_does_not_change_current() {
    let mut c = cfg();
    let a = c.add("a", PathBuf::from("/a")).unwrap();
    c.select_by_id(a).unwrap();

    c.add("b", PathBuf::from("/b")).unwrap();

    assert_eq!(c.current_db_id(), Some(a), "добавление сбило выбор");
}

#[test]
fn add_with_default_path_uses_path_dir() {
    let mut c = cfg();
    let id = c.add_with_default_path("main").unwrap();

    assert_eq!(
        c.find_by_id(id).unwrap().path_file(),
        &PathBuf::from("/data/main.db"),
    );
}

#[test]
fn add_with_default_path_respects_updated_path_dir() {
    let mut c = cfg();
    c.set_path_dir(PathBuf::from("/other"));

    let id = c.add_with_default_path("main").unwrap();

    assert_eq!(
        c.find_by_id(id).unwrap().path_file(),
        &PathBuf::from("/other/main.db"),
    );
}

// ------------------------------------------------------------------
// set_path_dir
// ------------------------------------------------------------------

#[test]
fn set_path_dir_does_not_touch_existing_records() {
    let mut c = cfg();
    let id = c.add("main", PathBuf::from("/data/main.db")).unwrap();

    c.set_path_dir(PathBuf::from("/other"));

    assert_eq!(c.path_dir(), &PathBuf::from("/other"));
    assert_eq!(
        c.find_by_id(id).unwrap().path_file(),
        &PathBuf::from("/data/main.db"),
        "путь уже зарегистрированной БД переписан",
    );
}

// ------------------------------------------------------------------
// find
// ------------------------------------------------------------------

#[test]
fn find_by_id_returns_none_for_unknown() {
    let c = cfg();
    assert!(c.find_by_id(Uuid::new_v4()).is_none());
}

#[test]
fn find_by_name_returns_matching() {
    let mut c = cfg();
    c.add("a", PathBuf::from("/a")).unwrap();
    c.add("b", PathBuf::from("/b")).unwrap();

    assert_eq!(
        c.find_by_name("b").unwrap().path_file(),
        &PathBuf::from("/b")
    );
    assert!(c.find_by_name("missing").is_none());
}

#[test]
fn find_by_id_mut_allows_edit() {
    let mut c = cfg();
    let id = c.add("old", PathBuf::from("/a")).unwrap();

    c.find_by_id_mut(id).unwrap().set_name("new");
    c.find_by_id_mut(id)
        .unwrap()
        .set_path_file(PathBuf::from("/b"));

    let db = c.find_by_id(id).unwrap();
    assert_eq!(db.name(), "new");
    assert_eq!(db.path_file(), &PathBuf::from("/b"));
}

// ------------------------------------------------------------------
// remove
// ------------------------------------------------------------------

#[test]
fn remove_by_id_returns_removed_and_shrinks() {
    let mut c = cfg();
    let id = c.add("main", PathBuf::from("/a")).unwrap();

    let removed = c.remove_by_id(id).unwrap();
    assert_eq!(removed.id(), id);
    assert_eq!(removed.name(), "main");
    assert!(c.is_empty());
    assert!(c.find_by_id(id).is_none());
}

#[test]
fn remove_by_name_returns_removed() {
    let mut c = cfg();
    c.add("main", PathBuf::from("/a")).unwrap();

    let removed = c.remove_by_name("main").unwrap();
    assert_eq!(removed.path_file(), &PathBuf::from("/a"));
    assert!(c.is_empty());
}

#[test]
fn remove_current_clears_current() {
    let mut c = cfg();
    let id = c.add("main", PathBuf::from("/a")).unwrap();
    c.select_by_id(id).unwrap();

    c.remove_by_id(id).unwrap();

    assert_eq!(c.current_db_id(), None);
    assert!(c.current_db().is_none());
}

#[test]
fn remove_other_keeps_current() {
    let mut c = cfg();
    let a = c.add("a", PathBuf::from("/a")).unwrap();
    let b = c.add("b", PathBuf::from("/b")).unwrap();
    c.select_by_id(a).unwrap();

    c.remove_by_id(b).unwrap();

    assert_eq!(c.current_db_id(), Some(a));
    assert_eq!(c.current_db().unwrap().name(), "a");
}

#[test]
fn remove_missing_id_errors() {
    let mut c = cfg();
    let err = c.remove_by_id(Uuid::new_v4()).unwrap_err();
    assert!(matches!(err, DbConfigError::IdNotFound(_)));
}

#[test]
fn remove_missing_name_errors() {
    let mut c = cfg();
    let err = c.remove_by_name("nope").unwrap_err();
    assert!(matches!(err, DbConfigError::NameNotFound(ref n) if n == "nope"));
}

#[test]
fn remove_frees_name_for_reuse() {
    let mut c = cfg();
    c.add("main", PathBuf::from("/a")).unwrap();
    c.remove_by_name("main").unwrap();

    let id = c.add("main", PathBuf::from("/b")).unwrap();
    assert_eq!(c.find_by_id(id).unwrap().path_file(), &PathBuf::from("/b"));
}

// ------------------------------------------------------------------
// select
// ------------------------------------------------------------------

#[test]
fn select_by_id_sets_current() {
    let mut c = cfg();
    let id = c.add("main", PathBuf::from("/a")).unwrap();

    c.select_by_id(id).unwrap();

    assert_eq!(c.current_db_id(), Some(id));
    assert_eq!(c.current_db().unwrap().name(), "main");
}

#[test]
fn select_by_name_sets_current() {
    let mut c = cfg();
    let id = c.add("main", PathBuf::from("/a")).unwrap();

    c.select_by_name("main").unwrap();

    assert_eq!(c.current_db_id(), Some(id));
}

#[test]
fn select_switches_between_databases() {
    let mut c = cfg();
    let a = c.add("a", PathBuf::from("/a")).unwrap();
    let b = c.add("b", PathBuf::from("/b")).unwrap();

    c.select_by_id(a).unwrap();
    assert_eq!(c.current_db_id(), Some(a));

    c.select_by_id(b).unwrap();
    assert_eq!(c.current_db_id(), Some(b));
}

#[test]
fn select_missing_id_errors_without_touching_state() {
    let mut c = cfg();
    let a = c.add("a", PathBuf::from("/a")).unwrap();
    c.select_by_id(a).unwrap();

    let err = c.select_by_id(Uuid::new_v4()).unwrap_err();
    assert!(matches!(err, DbConfigError::IdNotFound(_)));
    assert_eq!(c.current_db_id(), Some(a), "выбор сброшен при ошибке");
}

#[test]
fn select_missing_name_errors_without_touching_state() {
    let mut c = cfg();
    let a = c.add("a", PathBuf::from("/a")).unwrap();
    c.select_by_id(a).unwrap();

    let err = c.select_by_name("nope").unwrap_err();
    assert!(matches!(err, DbConfigError::NameNotFound(_)));
    assert_eq!(c.current_db_id(), Some(a));
}

#[test]
fn clear_current_keeps_records() {
    let mut c = cfg();
    let id = c.add("main", PathBuf::from("/a")).unwrap();
    c.select_by_id(id).unwrap();

    c.clear_current();

    assert!(c.current_db().is_none());
    assert_eq!(c.len(), 1, "clear_current не должен удалять запись");
}

#[test]
fn clear_current_is_idempotent() {
    let mut c = cfg();
    c.clear_current();
    c.clear_current();

    assert_eq!(c.current_db_id(), None);
}

// ------------------------------------------------------------------
// Инвариант: current_db всегда указывает на существующую запись
// ------------------------------------------------------------------

#[test]
fn current_id_never_dangles_after_full_lifecycle() {
    let mut c = cfg();

    let _ = c.add("a", PathBuf::from("/a")).unwrap();
    let b = c.add("b", PathBuf::from("/b")).unwrap();
    let d = c.add("d", PathBuf::from("/d")).unwrap();

    c.select_by_name("b").unwrap();
    assert!(c.find_by_id(c.current_db_id().unwrap()).is_some());

    c.remove_by_name("a").unwrap();
    assert!(c.find_by_id(c.current_db_id().unwrap()).is_some());

    c.select_by_id(d).unwrap();
    c.remove_by_id(b).unwrap();
    assert!(c.find_by_id(c.current_db_id().unwrap()).is_some());

    c.remove_by_id(d).unwrap();
    assert_eq!(c.current_db_id(), None);
}

fn count_all<S: DbStore>(s: &S) -> usize {
    s.len()
}

fn first_name<S: DbStore>(s: &S) -> Option<&str> {
    s.databases().first().map(|e| e.name())
}

#[test]
fn concrete_type_works_through_abstraction() {
    let mut c: DataBaseConfig<PathBuf> = DataBaseConfig::new(PathBuf::from("/data"));
    c.add("a", PathBuf::from("/a")).unwrap();
    c.add("b", PathBuf::from("/b")).unwrap();

    c.select_by_name("b").unwrap();

    assert_eq!(count_all(&c), 2);
    assert_eq!(first_name(&c), Some("a"));
    assert_eq!(c.current_db().unwrap().name(), "b");
}

#[test]
fn abstraction_preserves_rename() {
    let mut c: DataBaseConfig<PathBuf> = DataBaseConfig::new(PathBuf::from("/data"));
    let id = c.add("old", PathBuf::from("/a")).unwrap();

    let entry = <DataBaseConfig<PathBuf> as DbStoreMut>::find_by_id_mut(&mut c, id).unwrap();
    entry.set_name("new");

    assert_eq!(c.find_by_id(id).unwrap().name(), "new");
}

// ---------------------------------------------------------------------
// 2. Подставной тип — доказывает, что код на trait-bound не привязан
//    к DataBaseConfig/CurrentDataBaseConfig.
// ---------------------------------------------------------------------

#[derive(Debug)]
struct MockEntry {
    id: Uuid,
    name: String,
    path: PathBuf,
}

impl DbEntry for MockEntry {
    type Path = PathBuf;
    fn id(&self) -> Uuid {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn path_file(&self) -> &PathBuf {
        &self.path
    }
}

impl DbEntryMut for MockEntry {
    fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    fn set_path_file(&mut self, path: PathBuf) {
        self.path = path;
    }
}

#[derive(Debug)]
struct MockStore {
    path_dir: PathBuf,
    entries: Vec<MockEntry>,
    current: Option<Uuid>,
}

impl DbStore for MockStore {
    type Path = PathBuf;
    type Entry = MockEntry;

    fn path_dir(&self) -> &PathBuf {
        &self.path_dir
    }

    fn databases(&self) -> &[MockEntry] {
        &self.entries
    }

    fn current_db_id(&self) -> Option<Uuid> {
        self.current
    }
}

#[test]
fn generic_code_runs_over_mock_store() {
    let id = Uuid::new_v4();
    let mock = MockStore {
        path_dir: PathBuf::from("/mock"),
        entries: vec![MockEntry {
            id,
            name: "m".into(),
            path: PathBuf::from("/m"),
        }],
        current: Some(id),
    };

    assert_eq!(count_all(&mock), 1);
    assert_eq!(first_name(&mock), Some("m"));
    assert_eq!(mock.current_db().unwrap().id(), id);
}

// ---------------------------------------------------------------------
// 3. Provided-методы ведут себя одинаково для обоих типов.
// ---------------------------------------------------------------------

#[test]
fn provided_finders_agree_across_impls() {
    fn assert_finds<S: DbStore>(s: &S, id: Uuid, name: &str) {
        assert_eq!(s.find_by_id(id).map(|e| e.name()), Some(name));
        assert_eq!(s.find_by_name(name).map(|e| e.id()), Some(id));
        assert!(s.find_by_name("nope").is_none());
    }

    let mut c: DataBaseConfig<PathBuf> = DataBaseConfig::new(PathBuf::from("/data"));
    let id = c.add("x", PathBuf::from("/x")).unwrap();
    assert_finds(&c, id, "x");

    let mock = MockStore {
        path_dir: PathBuf::from("/m"),
        entries: vec![MockEntry {
            id,
            name: "x".into(),
            path: PathBuf::from("/x"),
        }],
        current: None,
    };
    assert_finds(&mock, id, "x");
}
