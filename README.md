A Fast Linked List!

```
use linked_list::FastLinkedList; 

fn main() { 
    let mut linked = FastLinkedList::new();
    let one = linked.push(1);
    let tow = linked.push(2);
    let three = linked.push(3);

    assert_eq!(linked.get(one),Some(&1));
    assert_eq!(linked.get_mut(tow),Some(&mut 2));

    linked.move_front(one).unwrap();
    assert_eq!(linked.remove(one), Some(1));
    linked.move_front(tow).unwrap();
    assert_eq!(linked.remove(tow), Some(2));
    linked.move_front(three).unwrap();
    assert_eq!(linked.remove(three), Some(3));

    linked.push(1);
    linked.push(2);
    linked.push(3);

    assert_eq!(linked.remove_last(), Some(1));
    assert_eq!(linked.remove_last(), Some(2));
    assert_eq!(linked.remove_last(), Some(3));

    for i in 1..10i32 {
        linked.push(i);
    }

    for (_, item) in linked.iter_mut() {
        *item += 1;
    }

    for (key, value) in linked.iter() {
        println!("key:{},value:{}",key,value);
    }

    linked.clear();
 }
```


