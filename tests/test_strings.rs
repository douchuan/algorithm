#![feature(is_sorted)]
#![allow(non_snake_case)]
use algo::strings::{brute_force, Quick3String, Quick3Way, TrieST, KMP, LSD, MSD, TST};
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
    let mut st = TrieST::default();
    // test len & empty
    assert!(st.is_empty());
    assert_eq!(0, st.len());

    let i = SHELLS_ST;
    let mut hm = HashMap::new();
    let a = extract_words(i);
    for (i, &s) in a.iter().enumerate() {
        hm.insert(s, i);
        // test put
        st.put(s, Some(i));
    }

    // test len & empty
    assert!(!st.is_empty());
    assert_eq!(7, st.len());

    for (&k, v) in hm.iter() {
        // test get & contains
        assert_eq!(st.get(k), Some(v));
        assert!(st.contains(k));
    }

    // test keys
    // TrieST keys contained in HashMap
    let keys = st.keys();
    assert_eq!(hm.keys().len(), st.len());
    for k in keys.iter() {
        assert!(hm.contains_key(k.as_str()));
    }
    // HashMap keys contained in TrieST
    let keys = hm.keys();
    for &k in keys {
        assert!(st.contains(k));
    }

    // test keys_with_prefix
    let mut matches = st.keys_with_prefix("shor");
    assert_eq!(1, matches.len());
    assert_eq!(Some("shore"), matches.dequeue().as_deref());

    // test keys_that_match
    let mut matches = st.keys_that_match(".he.l.");
    assert_eq!(Some("shells"), matches.dequeue().as_deref());

    // test longest_prefix_of
    assert_eq!(Some("shells"), st.longest_prefix_of("shellsort"));
    assert_eq!(Some("she"), st.longest_prefix_of("shell"));
    assert_eq!(None, st.longest_prefix_of("quicksort"));

    // test delete
    assert!(st.contains("shells"));
    st.delete("shells");
    assert_eq!(6, st.len());
    assert!(!st.contains("shells"));

    // test put(xx, None) can delete too
    assert!(st.contains("shore"));
    st.put("shore", None);
    assert_eq!(5, st.len());
    assert!(!st.contains("shore"));
}

#[test]
fn tst() {
    let mut st = TST::default();
    // test len & empty
    assert!(st.is_empty());
    assert_eq!(0, st.len());

    let i = SHELLS_ST;
    let mut hm = HashMap::new();
    let a = extract_words(i);
    for (i, &s) in a.iter().enumerate() {
        hm.insert(s, i);
        // test put
        st.put(s, Some(i));
    }

    // test len & empty
    assert!(!st.is_empty());
    assert_eq!(7, st.len());

    for (&k, v) in hm.iter() {
        // test get & contains
        assert_eq!(st.get(k), Some(v));
        assert!(st.contains(k));
    }

    // test keys
    // TST keys contained in HashMap
    let keys = st.keys();
    assert_eq!(hm.keys().len(), keys.len());
    for k in keys.iter() {
        assert!(hm.contains_key(k.as_str()));
    }
    // HashMap keys contained in TST
    let keys = hm.keys();
    for &k in keys {
        assert!(st.contains(k));
    }

    // test keys_with_prefix
    let mut matches = st.keys_with_prefix("shor");
    assert_eq!(1, matches.len());
    assert_eq!(Some("shore"), matches.dequeue().as_deref());

    // test keys_that_match
    let mut matches = st.keys_that_match(".he.l.");
    assert_eq!(Some("shells"), matches.dequeue().as_deref());

    // test longest_prefix_of
    assert_eq!(Some("shells"), st.longest_prefix_of("shellsort"));
    assert_eq!(Some("she"), st.longest_prefix_of("shell"));
    assert_eq!(None, st.longest_prefix_of("quicksort"));

    // test delete
    assert!(st.contains("shells"));
    st.put("shells", None);
    assert_eq!(6, st.len());
    assert!(!st.contains("shells"));
}

#[test]
fn trie_st_drop() {
    use algo::common::drop::{self, Elem};

    let mut st = TrieST::default();
    drop::with(|ctx| {
        st.put("aaa", Some(Elem));
        st.put("bbb", Some(Elem));
        st.put("ccc", Some(Elem));
        st.put("ddd", Some(Elem));
        drop(st);
        assert_eq!(4, ctx.get());
    });

    // test overwrite "aaa"
    let mut st = TrieST::default();
    drop::with(|ctx| {
        st.put("aaa", Some(Elem));
        st.put("bbb", Some(Elem));
        st.put("ccc", Some(Elem));
        st.put("ddd", Some(Elem));
        assert_eq!(0, ctx.get());
        st.put("aaa", Some(Elem)); // do overwrite
        assert_eq!(1, ctx.get());
        drop(st);
        assert_eq!(5, ctx.get());
    });
}

#[test]
fn trie_st_drop_with_delete() {
    use algo::common::drop::{self, Elem};

    let mut st = TrieST::default();
    drop::with(|ctx| {
        st.put("aaa", Some(Elem));
        st.put("bbb", Some(Elem));
        st.put("ccc", Some(Elem));
        st.put("ddd", Some(Elem));

        st.delete("aaa");
        st.delete("bbb");
        assert_eq!(2, ctx.get());

        drop(st);
        assert_eq!(4, ctx.get());
    });
}

#[test]
fn trie_st_drop_with_put() {
    use algo::common::drop::{self, Elem};

    let mut st = TrieST::default();
    drop::with(|ctx| {
        st.put("aaa", Some(Elem));
        st.put("bbb", Some(Elem));
        st.put("ccc", Some(Elem));
        st.put("ddd", Some(Elem));

        st.put("aaa", None);
        st.put("bbb", None);
        assert_eq!(2, ctx.get());

        drop(st);
        assert_eq!(4, ctx.get());
    });
}

#[test]
fn tst_drop() {
    use algo::common::drop::{self, Elem};

    let mut st = TST::default();
    drop::with(|ctx| {
        st.put("aaa", Some(Elem));
        st.put("bbb", Some(Elem));
        st.put("ccc", Some(Elem));
        st.put("ddd", Some(Elem));
        // do drop
        drop(st);

        assert_eq!(4, ctx.get());
    });
}

#[test]
fn tst_drop_with_put_none() {
    use algo::common::drop::{self, Elem};

    // init
    let mut st = TST::default();
    drop::with(|ctx| {
        st.put("aaa", Some(Elem));
        st.put("bbb", Some(Elem));
        st.put("ccc", Some(Elem));

        // do drops
        st.put("aaa", None);
        assert_eq!(1, ctx.get());
        st.put("bbb", None);
        assert_eq!(2, ctx.get());
        st.put("ccc", None);
        assert_eq!(3, ctx.get());
    });
}

#[test]
fn brute_force_search() {
    let data = substr_data();
    for (pat, txt, pos) in data {
        assert_eq!(pos, brute_force::search1(pat, txt));
        assert_eq!(pos, brute_force::search2(pat, txt));
    }
}

#[test]
fn kmp() {
    let data = substr_data();
    for (pat, txt, pos) in data {
        let kmp = KMP::from(pat);
        assert_eq!(pos, kmp.search(txt));
    }
}

fn extract_words(i: &str) -> Vec<&str> {
    i.split_whitespace().collect()
}

fn substr_data() -> Vec<(&'static str, &'static str, Option<usize>)> {
    vec![
        (
            "abracadabra",
            "abacadabrabracabracadabrabrabracad",
            Some(14),
        ),
        ("rab", "abacadabrabracabracadabrabrabracad", Some(8)),
        ("bcara", "abacadabrabracabracadabrabrabracad", None),
        (
            "rabrabracad",
            "abacadabrabracabracadabrabrabracad",
            Some(23),
        ),
        ("abacad", "abacadabrabracabracadabrabrabracad", Some(0)),
    ]
}
