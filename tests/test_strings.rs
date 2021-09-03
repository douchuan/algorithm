#![feature(is_sorted)]
#![allow(non_snake_case)]
use algo::strings::{Quick3String, Quick3Way, TrieST, LSD, MSD, TST};
use std::collections::HashMap;

const WORDS3: &'static str = include_str!("../res/strings/words3.txt");
const SHELLS: &'static str = include_str!("../res/strings/shells.txt");
const SHELLS_ST: &'static str = include_str!("../res/strings/shellsST.txt");

#[test]
fn LSD_radix_sort() {
    let i = WORDS3;
    let mut a = extract_words(i);
    let w = a[0].len();
    LSD::sort(&mut a, w);
    assert!(a.is_sorted());

    // license plate data
    let mut a = vec![
        "4PGC938", "2IYE230", "3CIO720", "1ICK750", "1OHV845", "4JZY524", "1ICK750", "3CIO720",
        "1OHV845", "1OHV845", "2RLA629", "2RLA629", "3ATW723",
    ];
    let w = a[0].len();
    LSD::sort(&mut a, w);
    assert!(a.is_sorted());
}

#[test]
fn LSD_radix_sort_i32() {
    let mut a: Vec<i32> = (0..10).rev().collect();
    LSD::sort_i32(&mut a);
    assert!(a.is_sorted());

    let mut a = vec![1, 2, 3, -1, -2, -3];
    LSD::sort_i32(&mut a);
    assert!(a.is_sorted());
}

#[test]
fn MSD_radix_sort() {
    // empty
    let mut data: Vec<&str> = vec![];
    MSD::sort(&mut data);
    assert!(data.is_sorted());

    // normal
    let i = SHELLS;
    let mut data = extract_words(i);
    MSD::sort(&mut data);
    assert!(data.is_sorted());
}

#[test]
fn quick3str() {
    // empty
    let mut data: Vec<&str> = vec![];
    Quick3String::sort(&mut data);
    assert!(data.is_sorted());

    // normal
    let i = SHELLS;
    let mut data = extract_words(i);
    Quick3String::sort(&mut data);
    assert!(data.is_sorted());
}

#[test]
fn quick3way() {
    // empty
    let mut data: Vec<&str> = vec![];
    Quick3Way::sort(&mut data);
    assert!(data.is_sorted());

    // normal
    let i = SHELLS;
    let mut data = extract_words(i);
    Quick3Way::sort(&mut data);
    assert!(data.is_sorted());
}

// also fine for sorted data
#[test]
fn sorted_data() {
    let mut a = vec![
        "4PGC938", "2IYE230", "3CIO720", "1ICK750", "1OHV845", "4JZY524", "1ICK750", "3CIO720",
        "1OHV845", "1OHV845", "2RLA629", "2RLA629", "3ATW723",
    ];
    a.sort();

    // lsd
    let w = a[0].len();
    LSD::sort(&mut a, w);
    assert!(a.is_sorted());

    // msd
    MSD::sort(&mut a);
    assert!(a.is_sorted());

    // three-way quick
    Quick3String::sort(&mut a);
    assert!(a.is_sorted());

    // lsd32
    let mut a: Vec<i32> = (0..10).collect();
    LSD::sort_i32(&mut a);
    assert!(a.is_sorted());
}

#[test]
fn trie_st() {
    let mut trie_st = TrieST::default();
    // test len & empty
    assert!(trie_st.is_empty());
    assert_eq!(0, trie_st.len());

    let i = SHELLS_ST;
    let mut hm = HashMap::new();
    let a = extract_words(i);
    for (i, &s) in a.iter().enumerate() {
        hm.insert(s, i);
        // test put
        trie_st.put(s, Some(i));
    }

    // test len & empty
    assert!(!trie_st.is_empty());
    assert_eq!(7, trie_st.len());

    for (&k, v) in hm.iter() {
        // test get & contains
        assert_eq!(trie_st.get(k), Some(v));
        assert!(trie_st.contains(k));
    }

    // test keys
    // TrieST keys contained in HashMap
    let keys = trie_st.keys();
    assert_eq!(hm.keys().len(), trie_st.len());
    for k in keys.iter() {
        assert!(hm.contains_key(k.as_str()));
    }
    // HashMap keys contained in TrieST
    let keys = hm.keys();
    for &k in keys {
        assert!(trie_st.contains(k));
    }

    // test keys_with_prefix
    let mut matches = trie_st.keys_with_prefix("shor");
    assert_eq!(Some("shore"), matches.dequeue().as_deref());

    // test keys_that_match
    let mut matches = trie_st.keys_that_match(".he.l.");
    assert_eq!(Some("shells"), matches.dequeue().as_deref());

    // test longest_prefix_of
    assert_eq!(Some("shells"), trie_st.longest_prefix_of("shellsort"));
    assert_eq!(None, trie_st.longest_prefix_of("quicksort"));

    // test delete
    assert!(trie_st.contains("shells"));
    trie_st.delete("shells");
    assert_eq!(6, trie_st.len());
    assert!(!trie_st.contains("shells"));

    // test put(xx, None) can delete too
    assert!(trie_st.contains("shore"));
    trie_st.put("shore", None);
    assert_eq!(5, trie_st.len());
    assert!(!trie_st.contains("shore"));
}

#[test]
fn trie_st_drop() {
    static mut DROPS: i32 = 0;
    struct Elem;
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                DROPS += 1;
            }
        }
    }

    let mut trie_st = TrieST::default();
    trie_st.put("aaa", Some(Elem));
    trie_st.put("bbb", Some(Elem));
    trie_st.put("ccc", Some(Elem));
    trie_st.put("ddd", Some(Elem));
    drop(trie_st);
    assert_eq!(unsafe { DROPS }, 4);

    // test overwrite "aaa"
    // reset DROPS
    unsafe {
        DROPS = 0;
    }
    let mut trie_st = TrieST::default();
    trie_st.put("aaa", Some(Elem));
    trie_st.put("bbb", Some(Elem));
    trie_st.put("ccc", Some(Elem));
    trie_st.put("ddd", Some(Elem));
    trie_st.put("aaa", Some(Elem)); // do overwrite
    drop(trie_st);
    assert_eq!(unsafe { DROPS }, 5);
}

#[test]
fn trie_st_drop_with_delete() {
    static mut DROPS: i32 = 0;
    struct Elem;
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                DROPS += 1;
            }
        }
    }

    let mut trie_st = TrieST::default();
    trie_st.put("aaa", Some(Elem));
    trie_st.put("bbb", Some(Elem));
    trie_st.put("ccc", Some(Elem));
    trie_st.put("ddd", Some(Elem));

    trie_st.delete("aaa");
    trie_st.delete("bbb");
    assert_eq!(unsafe { DROPS }, 2);

    drop(trie_st);
    assert_eq!(unsafe { DROPS }, 4);
}

#[test]
fn trie_st_drop_with_put() {
    static mut DROPS: i32 = 0;
    struct Elem;
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                DROPS += 1;
            }
        }
    }

    let mut trie_st = TrieST::default();
    trie_st.put("aaa", Some(Elem));
    trie_st.put("bbb", Some(Elem));
    trie_st.put("ccc", Some(Elem));
    trie_st.put("ddd", Some(Elem));

    trie_st.put("aaa", None);
    trie_st.put("bbb", None);
    assert_eq!(unsafe { DROPS }, 2);

    drop(trie_st);
    assert_eq!(unsafe { DROPS }, 4);
}

#[test]
fn tst() {
    let mut tst = TST::default();
    // test len & empty
    assert!(tst.is_empty());
    assert_eq!(0, tst.len());

    let i = SHELLS_ST;
    let mut hm = HashMap::new();
    let a = extract_words(i);
    for (i, &s) in a.iter().enumerate() {
        hm.insert(s, i);
        // test put
        tst.put(s, Some(i));
    }

    // test len & empty
    assert!(!tst.is_empty());
    assert_eq!(7, tst.len());

    for (&k, v) in hm.iter() {
        // test get & contains
        assert_eq!(tst.get(k), Some(v));
        assert!(tst.contains(k));
    }

    // test keys
    // TST keys contained in HashMap
    let keys = tst.keys();
    assert_eq!(hm.keys().len(), keys.len());
    for k in keys.iter() {
        assert!(hm.contains_key(k.as_str()));
    }
    // HashMap keys contained in TST
    let keys = hm.keys();
    for &k in keys {
        assert!(tst.contains(k));
    }

    // test delete
    assert!(tst.contains("shells"));
    tst.put("shells", None);
    assert_eq!(6, tst.len());
    assert!(!tst.contains("shells"));
}

#[test]
fn tst_drop() {
    static mut DROPS: i32 = 0;
    struct Elem;
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                DROPS += 1;
            }
        }
    }

    let mut tst = TST::default();
    tst.put("aaa", Some(Elem));
    tst.put("bbb", Some(Elem));
    tst.put("ccc", Some(Elem));
    tst.put("ddd", Some(Elem));
    // do drop
    drop(tst);
    assert_eq!(unsafe { DROPS }, 4);
}

#[test]
fn tst_drop_with_put_none() {
    static mut DROPS: i32 = 0;
    struct Elem;
    impl Drop for Elem {
        fn drop(&mut self) {
            unsafe {
                DROPS += 1;
            }
        }
    }

    // init
    let mut tst = TST::default();
    tst.put("aaa", Some(Elem));
    tst.put("bbb", Some(Elem));
    tst.put("ccc", Some(Elem));
    tst.put("ddd", Some(Elem));

    // do drops
    tst.put("aaa", None);
    assert_eq!(unsafe { DROPS }, 1);
    tst.put("bbb", None);
    assert_eq!(unsafe { DROPS }, 2);
    tst.put("ccc", None);
    assert_eq!(unsafe { DROPS }, 3);
    tst.put("ddd", None);
    assert_eq!(unsafe { DROPS }, 4);
}

fn extract_words(i: &str) -> Vec<&str> {
    i.split_whitespace().collect()
}
