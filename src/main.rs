use std::fmt::Display;
use std::ops::Deref;
use std::alloc::{alloc, dealloc, Layout};


///////////////////////// hash map ///////////////////////////////////////////////////////////////////////////////////////
#[macro_use]
extern crate std;
use std::collections::HashMap;
macro_rules! hash_map {
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}
/////////////////////////// rc ////////////////////////////////////////////////////////////////////////////////////////////
struct Myrc<T: Copy> {              // struct MyRc
    // data: *const T,
    data: T,                        // pointer to data
    time: *mut u8,                  // pointer to the counter
}
impl<T: Copy + Display> Myrc<T> {
    fn new(x: T) -> Myrc<T> {                           // when creating new rc
        let layout = Layout::new::<i32>();
        unsafe {
            let ptr = alloc(layout);        
            *(ptr as *mut u8) = 1;                      // set counter to 1
            Myrc { data: x, time: ptr as *mut u8 }
        }
    }
}

impl<T: Copy+Display> Deref for Myrc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &self.data                              // deref for the pointer to the data
        }
    }
}
impl<T: Copy+Display> Clone for Myrc<T> {           // when clone, increase the counter and copy the pointers
    fn clone(&self) -> Self {
        unsafe {
            *self.time.as_mut().unwrap() += 1;      // increase the counter
        }
        Myrc { data: self.data, time: self.time }   // copy the pointers
    }
}

impl<T: Copy> Drop for Myrc<T> {                    // when drop, decrease the counter, if reaches 0, dealloc for the pointers
    fn drop(&mut self) {
        unsafe {
            if *self.time.as_ref().unwrap() == 1 {                  // if to drop the last one
                let layout = Layout::new::<i32>();  
                dealloc(self.time, layout);                     // release the space
                println!("finally erased the last counter");            // print to show
            } else {
                *self.time.as_mut().unwrap() -= 1;                  // otherwise, decrease the counter
            }
        }
    }
}

/////////////////////////// stack ////////////////////////////////////////////////////////////////////////////////////////////

use std::cell::RefCell;

#[derive(Debug)]
struct SimpleStack<T> {
    stack: RefCell<Vec<T>>,
}
impl<T> SimpleStack<T> {
    fn new() -> SimpleStack<T> {
        SimpleStack { stack: RefCell::new(Vec::new()) }
    }

    fn push(&self, value: T) {
        self.stack.borrow_mut().push(value);
    }

    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}


fn main() {
    let map = hash_map! {
        "a" => 1,
        "b" => 2,
        "c" => 3
    };
    println!("Hash map:");
    println!("------------------------------");
    println!("{:?}", map);

    ////////////////////////////////////////////////////////////////
    println!("");
    println!("RC");
    println!("------------------------------");
    unsafe {
        let x = 30;
        let y = Myrc::new(x);                            // create the rc

        println!("data of y: {:?}, count of y: {:?}", *y, y.time.as_ref().unwrap());        // show the data

        {
            let z = y.clone();        
            println!("z created");
            println!("data of y: {:?}, count of y: {:?}", *y, y.time.as_ref().unwrap());        // show y, with increased count
            println!("data of z: {:?}, count of z: {:?}", *z, z.time.as_ref().unwrap());        // show z, with same data and increased count
        }
        println!("z erased");
        println!("data of y: {:?}, count of y: {:?}", *y, y.time.as_ref().unwrap());            // after z erased, counter will decrease     
        // then the erased info will be shown   
    }

    ///////////////////////////////////////////////////////////////
    let stack = SimpleStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("");
    println!("stack");
    println!("------------------------------");


    println!("pop 2: {:?} {:?}", stack.pop(), stack.pop());

    stack.push(4);
    println!("pop 3: {:?} {:?} {:?}", stack.pop(), stack.pop(), stack.pop());

}
