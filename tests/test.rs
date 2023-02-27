use linked_list::FastLinkedList;


#[test]
fn test_push_remove() {
    let mut linked = FastLinkedList::new();
    let one = linked.push("1".to_string());
    let tow = linked.push("2".to_string());
    let three = linked.push("3".to_string());

    assert_eq!(linked.remove(one), Some("1".to_string()));
    assert_eq!(linked.remove(tow), Some("2".to_string()));
    assert_eq!(linked.remove(three), Some("3".to_string()));

    let one = linked.push("1".to_string());
    let tow = linked.push("2".to_string());
    let three = linked.push("3".to_string());

    assert_eq!(linked.remove(three), Some("3".to_string()));
    assert_eq!(linked.remove(tow), Some("2".to_string()));
    assert_eq!(linked.remove(one), Some("1".to_string()));

    let one = linked.push("1".to_string());
    let tow = linked.push("2".to_string());
    let three = linked.push("3".to_string());

    assert_eq!(linked.remove(tow), Some("2".to_string()));
    assert_eq!(linked.remove(three), Some("3".to_string()));
    assert_eq!(linked.remove(one), Some("1".to_string()));

    let one = linked.push("1".to_string());
    let tow = linked.push("2".to_string());
    let three = linked.push("3".to_string());

    assert_eq!(linked.remove(one), Some("1".to_string()));
    assert_eq!(linked.remove(three), Some("3".to_string()));
    assert_eq!(linked.remove(tow), Some("2".to_string()));
}

#[test]
fn test_move_front() {
    let mut linked: FastLinkedList<usize> = FastLinkedList::new();

    let one = linked.push(1);
    linked.move_front(one).unwrap();
    assert_eq!(linked.remove(one), Some(1));

    let one = linked.push(1);
    let tow = linked.push(2);
    let three = linked.push(3);

    linked.move_front(one).unwrap();
    assert_eq!(linked.remove(one), Some(1));
    linked.move_front(tow).unwrap();
    assert_eq!(linked.remove(tow), Some(2));
    linked.move_front(three).unwrap();
    assert_eq!(linked.remove(three), Some(3));

    for i in 1..10000 {
        linked.push(i);
    }

    let one = linked.push(1);
    let tow = linked.push(2);
    let three = linked.push(3);

    assert_eq!(linked.get(one), Some(&1));
    assert_eq!(linked.get_mut(tow), Some(&mut 2));

    linked.move_front(three).unwrap();
    assert_eq!(linked.remove(three), Some(3));
    linked.shrink_to_fit();
    linked.move_front(tow).unwrap();
    assert_eq!(linked.remove(tow), Some(2));
    linked.move_front(one).unwrap();
    assert_eq!(linked.remove(one), Some(1));

    let one = linked.push(1);
    let tow = linked.push(2);
    let three = linked.push(3);

    linked.move_front(one).unwrap();
    assert_eq!(linked.remove(tow), Some(2));
    linked.move_front(three).unwrap();
    assert_eq!(linked.remove(one), Some(1));
    linked.move_front(three).unwrap();
    assert_eq!(linked.remove(three), Some(3));
}

#[test]
fn linked_linked_remove_last() {
    let mut linked: FastLinkedList<usize> = FastLinkedList::new();
    assert_eq!(linked.remove_last(), None);
    linked.push(1);
    assert_eq!(linked.len(), 1);
    assert_eq!(linked.remove_last(), Some(1));

    linked.push(1);
    linked.push(2);
    linked.push(3);

    assert_eq!(linked.remove_last(), Some(1));
    assert_eq!(linked.remove_last(), Some(2));
    assert_eq!(linked.remove_last(), Some(3));
    assert_eq!(linked.remove_last(), None);

    let one = linked.push(1);
    linked.push(2);
    let three = linked.push(3);
    linked.move_front(one).unwrap();
    assert_eq!(linked.remove_last(), Some(2));
    linked.move_front(three).unwrap();
    assert_eq!(linked.remove_last(), Some(1));
    assert_eq!(linked.remove_last(), Some(3));

    linked.push(1);
    linked.push(2);
    linked.push(3);
    linked.clear();
    assert_eq!(linked.remove_last(), None);
    linked.shrink_to_fit();
}

#[test]
fn test_iter() {
    let mut linked = FastLinkedList::new();
    for i in 1..10i32 {
        linked.push(i);
    }
    println!("{}", linked.len());

    for (key, value) in linked.iter() {
        println!("key:{key},value:{value}")
    }

    for (key, value) in linked.iter().rev() {
        println!("rev key:{key},value:{value}")
    }

    let mut iter = linked.iter();
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

    linked.remove_last();
    let mut iter = linked.iter();
    assert_eq!(iter.len(), 8);
    iter.next();
    assert_eq!(iter.len(), 7);
    iter.next_back();
    assert_eq!(iter.len(), 6);
}

#[test]
fn test_iter_mut() {
    let mut linked = FastLinkedList::new();
    for i in 1..10i32 {
        linked.push(i);
    }

    for (_, item) in linked.iter_mut() {
        *item += 1;
    }

    for (key, value) in linked.iter() {
        println!("key:{key},value:{value}")
    }

    for key in 0..9usize {
        let p = key as i32 + 2;
        assert_eq!(linked.get(key), Some(&p))
    }

    for (key, value) in linked.iter_mut().rev() {
        println!("rev key:{key},value:{value}")
    }

    let mut iter = linked.iter_mut();
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

    linked.remove_last();
    let mut iter = linked.iter_mut();
    assert_eq!(iter.len(), 8);
    iter.next();
    assert_eq!(iter.len(), 7);
    iter.next_back();
    assert_eq!(iter.len(), 6);

    linked.clear();
}

#[test]
fn test_into_iter() {
    let mut linked = FastLinkedList::new();
    for i in 1..10i32 {
        linked.push(i);
    }

    for value in linked.into_iter() {
        println!("value:{value}")
    }

    let mut linked = FastLinkedList::new();
    for i in 1..10i32 {
        linked.push(i);
    }

    let ref_linked = &linked;
    for (key, value) in ref_linked.into_iter() {
        println!("key:{key},value:{value}")
    }

    let ref_mut_linked = &mut linked;
    for (key, value) in ref_mut_linked.into_iter() {
        println!("mut key:{key},value:{value}")
    }
}