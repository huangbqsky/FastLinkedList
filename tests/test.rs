use linked_list::FastLinkedList;


#[test]
fn test_push_remove() {
    let mut lur = FastLinkedList::new();
    let one = lur.push("1".to_string());
    let tow = lur.push("2".to_string());
    let three = lur.push("3".to_string());

    assert_eq!(lur.remove(one), Some("1".to_string()));
    assert_eq!(lur.remove(tow), Some("2".to_string()));
    assert_eq!(lur.remove(three), Some("3".to_string()));

    let one = lur.push("1".to_string());
    let tow = lur.push("2".to_string());
    let three = lur.push("3".to_string());

    assert_eq!(lur.remove(three), Some("3".to_string()));
    assert_eq!(lur.remove(tow), Some("2".to_string()));
    assert_eq!(lur.remove(one), Some("1".to_string()));

    let one = lur.push("1".to_string());
    let tow = lur.push("2".to_string());
    let three = lur.push("3".to_string());

    assert_eq!(lur.remove(tow), Some("2".to_string()));
    assert_eq!(lur.remove(three), Some("3".to_string()));
    assert_eq!(lur.remove(one), Some("1".to_string()));

    let one = lur.push("1".to_string());
    let tow = lur.push("2".to_string());
    let three = lur.push("3".to_string());

    assert_eq!(lur.remove(one), Some("1".to_string()));
    assert_eq!(lur.remove(three), Some("3".to_string()));
    assert_eq!(lur.remove(tow), Some("2".to_string()));
}

#[test]
fn test_move_front() {
    let mut lur: FastLinkedList<usize> = FastLinkedList::new();

    let one = lur.push(1);
    lur.move_front(one).unwrap();
    assert_eq!(lur.remove(one), Some(1));

    let one = lur.push(1);
    let tow = lur.push(2);
    let three = lur.push(3);

    lur.move_front(one).unwrap();
    assert_eq!(lur.remove(one), Some(1));
    lur.move_front(tow).unwrap();
    assert_eq!(lur.remove(tow), Some(2));
    lur.move_front(three).unwrap();
    assert_eq!(lur.remove(three), Some(3));

    for i in 1..10000 {
        lur.push(i);
    }

    let one = lur.push(1);
    let tow = lur.push(2);
    let three = lur.push(3);

    assert_eq!(lur.get(one), Some(&1));
    assert_eq!(lur.get_mut(tow), Some(&mut 2));

    lur.move_front(three).unwrap();
    assert_eq!(lur.remove(three), Some(3));
    lur.shrink_to_fit();
    lur.move_front(tow).unwrap();
    assert_eq!(lur.remove(tow), Some(2));
    lur.move_front(one).unwrap();
    assert_eq!(lur.remove(one), Some(1));

    let one = lur.push(1);
    let tow = lur.push(2);
    let three = lur.push(3);

    lur.move_front(one).unwrap();
    assert_eq!(lur.remove(tow), Some(2));
    lur.move_front(three).unwrap();
    assert_eq!(lur.remove(one), Some(1));
    lur.move_front(three).unwrap();
    assert_eq!(lur.remove(three), Some(3));
}

#[test]
fn lur_linked_remove_last() {
    let mut lur: FastLinkedList<usize> = FastLinkedList::new();
    assert_eq!(lur.remove_last(), None);
    lur.push(1);
    assert_eq!(lur.len(), 1);
    assert_eq!(lur.remove_last(), Some(1));

    lur.push(1);
    lur.push(2);
    lur.push(3);

    assert_eq!(lur.remove_last(), Some(1));
    assert_eq!(lur.remove_last(), Some(2));
    assert_eq!(lur.remove_last(), Some(3));
    assert_eq!(lur.remove_last(), None);

    let one = lur.push(1);
    lur.push(2);
    let three = lur.push(3);
    lur.move_front(one).unwrap();
    assert_eq!(lur.remove_last(), Some(2));
    lur.move_front(three).unwrap();
    assert_eq!(lur.remove_last(), Some(1));
    assert_eq!(lur.remove_last(), Some(3));

    lur.push(1);
    lur.push(2);
    lur.push(3);
    lur.clear();
    assert_eq!(lur.remove_last(), None);
    lur.shrink_to_fit();
}

#[test]
fn test_iter() {
    let mut lur = FastLinkedList::new();
    for i in 1..10i32 {
        lur.push(i);
    }
    println!("{}", lur.len());

    for (key, value) in lur.iter() {
        println!("key:{key},value:{value}")
    }

    for (key, value) in lur.iter().rev() {
        println!("rev key:{key},value:{value}")
    }

    let mut iter = lur.iter();
    for i in 0..9 {
        if i % 2 == 0 {
            if let Some((key, value)) = iter.next() {
                println!("key:{key},value:{value}")
            } else {
                panic!("1")
            }
        } else {
            if let Some((key, value)) = iter.next_back() {
                println!("back key:{key},value:{value}")
            } else {
                panic!("2")
            }
        }
    }

    lur.remove_last();
    let mut iter = lur.iter();
    assert_eq!(iter.len(), 8);
    iter.next();
    assert_eq!(iter.len(), 7);
    iter.next_back();
    assert_eq!(iter.len(), 6);
}

#[test]
fn test_iter_mut() {
    let mut lur = FastLinkedList::new();
    for i in 1..10i32 {
        lur.push(i);
    }

    for (_, item) in lur.iter_mut() {
        *item += 1;
    }

    for (key, value) in lur.iter() {
        println!("key:{key},value:{value}")
    }

    for key in 0..9usize {
        let p = key as i32 + 2;
        assert_eq!(lur.get(key), Some(&p))
    }

    for (key, value) in lur.iter_mut().rev() {
        println!("rev key:{key},value:{value}")
    }

    let mut iter = lur.iter_mut();
    for i in 0..9 {
        if i % 2 == 0 {
            if let Some((key, value)) = iter.next() {
                println!("key:{key},value:{value}")
            } else {
                panic!("1")
            }
        } else {
            if let Some((key, value)) = iter.next_back() {
                println!("back key:{key},value:{value}")
            } else {
                panic!("2")
            }
        }
    }

    lur.remove_last();
    let mut iter = lur.iter_mut();
    assert_eq!(iter.len(), 8);
    iter.next();
    assert_eq!(iter.len(), 7);
    iter.next_back();
    assert_eq!(iter.len(), 6);

    lur.clear();
}

#[test]
fn test_into_iter() {
    let mut lur = FastLinkedList::new();
    for i in 1..10i32 {
        lur.push(i);
    }

    for value in lur.into_iter() {
        println!("value:{value}")
    }

    let mut lur = FastLinkedList::new();
    for i in 1..10i32 {
        lur.push(i);
    }

    let ref_lur = &lur;
    for (key, value) in ref_lur.into_iter() {
        println!("key:{key},value:{value}")
    }

    let ref_mut_lur = &mut lur;
    for (key, value) in ref_mut_lur.into_iter() {
        println!("mut key:{key},value:{value}")
    }
}