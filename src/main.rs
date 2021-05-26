fn main() {
    use dtree::OsState;
    let mut s = OsState::new();
    s.mkdir("a").unwrap();
    s.chdir(&["a"]).unwrap();
    s.mkdir("b").unwrap();
    s.chdir(&["b"]).unwrap();
    s.mkdir("c").unwrap();
    s.chdir(&[]).unwrap();
    assert_eq!(&s.paths().unwrap(), &["/a/b/c/"]);
    
    use dtree::DTree;
    let mut da = DTree::new();
    da.mkdir("a").unwrap();
    da.mkdir("z").unwrap();
    da.with_subdir_mut(&["a"], |da| da.mkdir("b").unwrap()).unwrap();
    da.with_subdir_mut(&["a"], |da| da.mkdir("c").unwrap()).unwrap();
    da.with_subdir_mut(&["a","c"], |da| da.mkdir("d").unwrap()).unwrap();
    let mut paths = da.paths();
    paths.sort();
    println!("{:?}", paths);

    let mut db = DTree::new();
    db.mkdir("test").unwrap();
    assert_eq!(&db.paths(), &["/test/"]);

    let mut dd = DTree::new();
    dd.mkdir("a").unwrap();
    dd.with_subdir_mut(&["a"], |dd| dd.mkdir("b").unwrap()).unwrap();
    assert_eq!(&dd.paths(), &["/a/b/"]);
}