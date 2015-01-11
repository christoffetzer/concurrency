#![experimental]

use std::thread::Thread;

#[doc = r#"
This crate contains a few functions that demonstrate how to
program concurrent programs in Rust.

Our first function (a_spawn_thread) spawns two threads: 
the first thread prints 'hello' and the
2nd thread prints 'world'. 

```test_harness
use std::thread::Thread;

#[test]
pub fn a_spawn_thread() {
	let _t1 = Thread::spawn(|| { println!("Hello");}); // spawn thread
	let _t2 = Thread::spawn(|| { println!("World");}); // spawn thread
	assert!(true);
}
```
"#]

pub fn a_spawn_thread() {
	let _t1 = Thread::spawn(|| { println!("Hello");}); // spawn thread
	let _t2 = Thread::spawn(|| { println!("World");}); // spawn thread
	assert!(true);
}

#[doc = r#"
function b_channel creates a channel to transport integers 
and spawns one thread _t0 that 
receives integers from this channel and it generates two threads _t1 and _t2
that will send each one value across the channel.

```test_harness
use std::thread::Thread;

#[test]
pub fn b_channel() {
	let (tx,rx) = channel();
	let tx1 = tx.clone();
	
	let _t0 = Thread::spawn(move || { 
		let v1 : int = rx.recv();
		let v2 : int = rx.recv();
		println!("Value= {}", v1);
		println!("Value= {}", v2);
		assert!((v1 == 1 && v2 == 3) || (v1 == 3 && v2 == 1));
	});

	let _t1 = Thread::spawn(move || { tx.send(1i);});
	let _t2 = Thread::spawn(move || { tx1.send(3);});
}
```
"#]

pub fn b_channel() {
	let (tx,rx) = channel();
	let tx1 = tx.clone();
	
	let _t0 = Thread::spawn(move || { 
		let v1 : int = rx.recv();
		let v2 : int = rx.recv();
		println!("Value= {}", v1);
		println!("Value= {}", v2);
		assert!((v1 == 1 && v2 == 3) || (v1 == 3 && v2 == 1));
	});

	let _t1 = Thread::spawn(move || { tx.send(1i);});
	let _t2 = Thread::spawn(move || { tx1.send(3);});
}

#[doc = "
define messages for the actor implementation of the 'sadistic homework'.
"]

pub enum Msg<T : Send> {
  GetLeft(Sender<Option<T>>),  // dequeue value from the left of the list
  GetRight(Sender<Option<T>>), // dequeue value from the right of the list
  PutLeft(T),  	// enqueue a value on the left side of the list
  PutRight(T), 	// enqueue a value on the right side of the list
  Terminate, 	// terminate the thread
}

#[doc = r#"
An actor-based solution of the 'sadistic homework'. 

This is a blocking version: the actor only replies to a get request
if we can dequeue an element from the queue

```test_harness
use std::thread::Thread;
use concurrency::Msg;

#[test]
pub fn c_blocking_get() {
	 let (tx,rx) = channel();

	 let _t = Thread::spawn(move || {
		 let mut list : Vec<uint> = Vec::new();
		 let (mtx,mrx) = channel(); // delayed messages
		 loop { // do we have some Get requests to process?		
			let mut cont = true;
			while cont && list.len() > 0 {
				match mrx.try_recv() {
					Ok(Msg::GetLeft(tx))  => tx.send(list.remove(0)),
					Ok(Msg::GetRight(tx)) => tx.send(list.pop()),
					_ => cont = false,
				}
			}
			let m = rx.recv(); 
			match m {
				Msg::GetLeft(tx) => 
					if list.len() > 0 { tx.send(list.remove(0)) }
					else { mtx.send(Msg::GetLeft(tx)) },
				Msg::GetRight(tx)=> 
					if list.len() > 0 { tx.send(list.pop()) }
					else { mtx.send(Msg::GetRight(tx)) },
				Msg::PutLeft(v)  => list.insert(0u, v),
				Msg::PutRight(v) => list.push(v),			
				Msg::Terminate   => return,			
			 };
		 }
	});

	tx.send(Msg::PutLeft(10u));
	tx.send(Msg::PutRight(11u));

	let (rtx,rrx) = channel();
	tx.send(Msg::GetLeft(rtx.clone()));
	let v = rrx.recv();
	assert!(v == Some(10u));

	tx.send(Msg::GetLeft(rtx.clone()));
	let v = rrx.recv();
	assert!(v == Some(11u));

	tx.send(Msg::GetLeft(rtx.clone()));
	tx.send(Msg::PutRight(12u));

	let v = rrx.recv();
	assert!(v == Some(12u));

	tx.send(Msg::Terminate);
}
```
"#]

pub fn c_blocking_get() {
	 let (tx,rx) = channel();

	 let _t = Thread::spawn(move || {
		 let mut list : Vec<uint> = Vec::new();
		 let (mtx,mrx) = channel(); // delayed messages
		 loop { // first see if we can need to process some Get requests		
			let mut cont = true;
			while cont && list.len() > 0 {
				match mrx.try_recv() {
					Ok(Msg::GetLeft(tx))  => tx.send(list.remove(0)),
					Ok(Msg::GetRight(tx)) => tx.send(list.pop()),
					_ => cont = false,
				}
			}
			let m = rx.recv(); 
			match m {
				Msg::GetLeft(tx) => 
					if list.len() > 0 { tx.send(list.remove(0)) }
					else { mtx.send(Msg::GetLeft(tx)) },
				Msg::GetRight(tx)=> 
					if list.len() > 0 { tx.send(list.pop()) }
					else { mtx.send(Msg::GetRight(tx)) },
				Msg::PutLeft(v)  => list.insert(0u, v),
				Msg::PutRight(v) => list.push(v),			
				Msg::Terminate   => return,			
			 };
		 }
	});

	tx.send(Msg::PutLeft(10u));
	tx.send(Msg::PutRight(11u));

	let (rtx,rrx) = channel();
	tx.send(Msg::GetLeft(rtx.clone()));
	let v = rrx.recv();
	assert!(v == Some(10u));

	tx.send(Msg::GetLeft(rtx.clone()));
	let v = rrx.recv();
	assert!(v == Some(11u));

	tx.send(Msg::GetLeft(rtx.clone()));
	tx.send(Msg::PutRight(12u));

	let v = rrx.recv();
	assert!(v == Some(12u));

	tx.send(Msg::Terminate);
}

#[doc = r#"
actor solution of the 'sadistic homework'. 

This is a polling version: the actor replies with a 'None'
when attempting to read from an empty list.

```test_harness
use std::thread::Thread;
use concurrency::Msg;

#[test]
pub fn d_polling_version() {
	let (tx,rx) = channel();
	let _t = Thread::spawn(move || {
		 let mut list : Vec<uint> = Vec::new();
		 loop {
			let m = rx.recv(); 
			match m {
				Msg::GetLeft(tx) => tx.send(list.remove(0)),
				Msg::GetRight(tx)=> tx.send(list.pop()),
				Msg::PutLeft(v)  => list.insert(0u, v),
				Msg::PutRight(v) => list.push(v),			
				Msg::Terminate   => return,			
			 };
		 }
		});

	let (rtx,rrx) = channel();
	
	tx.send(Msg::GetLeft(rtx.clone()));

	let v = rrx.recv();
	assert!(v == None);
	
	tx.send(Msg::PutLeft(10u));
	tx.send(Msg::GetLeft(rtx.clone()));

	let v = rrx.recv();
	assert!(v == Some(10u));

	tx.send(Msg::PutRight(11u));
	tx.send(Msg::PutRight(12u));

	tx.send(Msg::GetRight(rtx.clone()));

	let v = rrx.recv();
	assert!(v == Some(12u));

	tx.send(Msg::Terminate);	
}
```
"#]

pub fn d_polling_version() {
	let (tx,rx) = channel();
	let _t = Thread::spawn(move || {
		 let mut list : Vec<uint> = Vec::new();
		 loop {
			let m = rx.recv(); 
			match m {
				Msg::GetLeft(tx) => tx.send(list.remove(0)),
				Msg::GetRight(tx)=> tx.send(list.pop()),
				Msg::PutLeft(v)  => list.insert(0u, v),
				Msg::PutRight(v) => list.push(v),			
				Msg::Terminate   => return,			
			 };
		 }
		});

	let (rtx,rrx) = channel();
	
	tx.send(Msg::GetLeft(rtx.clone()));

	let v = rrx.recv();
	assert!(v == None);
	
	tx.send(Msg::PutLeft(10u));
	tx.send(Msg::GetLeft(rtx.clone()));

	let v = rrx.recv();
	assert!(v == Some(10u));

	tx.send(Msg::PutRight(11u));
	tx.send(Msg::PutRight(12u));

	tx.send(Msg::GetRight(rtx.clone()));

	let v = rrx.recv();
	assert!(v == Some(12u));

	tx.send(Msg::Terminate);	
}


#[doc = r#"
actor solution of the 'sadistic homework' using select! instead
of a private channel to delay GetMessages arriving when the list
is empty.

We split the messages into two enums: GETs and PUTs
"#]

pub enum GETs<T : Send> {
  GetLeft(Sender<Option<T>>),
  GetRight(Sender<Option<T>>),
}

#[doc = "
We split the messages into two enums: GETs and PUTs
PUTs also contain the termination message.
"]

pub enum PUTs<T : Send> {
  PutLeft(T),
  PutRight(T),
  Terminate,
}

#[doc = r#"
actor solution of the 'sadistic homework' using select! instead
of a private channel to delay GetMessages arriving when the list
is empty.

```test_harness
use std::thread::Thread;
use concurrency::PUTs;
use concurrency::GETs;


#[test]
fn e_select_version() {
	let (ptx,prx) = channel();
	let (gtx,grx) = channel();

	let _t = Thread::spawn(move || {
		 let mut list : Vec<uint> = Vec::new();
		 loop {
			if list.len() > 0 {
				select!(   
					m = grx.recv() => {
						match m {
							GETs::GetLeft(tx) => tx.send(list.remove(0)),
							GETs::GetRight(tx)=> tx.send(list.pop()),
						};
					},
					m = prx.recv() => {
						match m {
							PUTs::PutLeft(v)  => list.insert(0u, v),
							PUTs::PutRight(v) => list.push(v),			
							PUTs::Terminate   => return,			
						};
					}
				)
			} else {
				let m = prx.recv();
				match m {
					PUTs::PutLeft(v)  => list.insert(0u, v),
					PUTs::PutRight(v) => list.push(v),			
					PUTs::Terminate   => return,			
				};
			}
		 }
		});

	let (rtx,rrx) = channel();
	gtx.send(GETs::GetLeft(rtx.clone()));
	gtx.send(GETs::GetRight(rtx.clone()));
	ptx.send(PUTs::PutLeft(10u));
	ptx.send(PUTs::PutRight(11u));

	let v = rrx.recv();
	assert!(v == Some(10u));
	let v = rrx.recv();
	assert!(v == Some(11u));

	ptx.send(PUTs::Terminate);
}
```
"#]

pub fn e_select_version() {
	let (ptx,prx) = channel();
	let (gtx,grx) = channel();

	let _t = Thread::spawn(move || {
		 let mut list : Vec<uint> = Vec::new();
		 loop {
			if list.len() > 0 { // select! does not (yet) support a guard expression...
				select!(   
					m = grx.recv() => {
						match m {
							GETs::GetLeft(tx) => tx.send(list.remove(0)),
							GETs::GetRight(tx)=> tx.send(list.pop()),
						};
					},
					m = prx.recv() => {
						match m {
							PUTs::PutLeft(v)  => list.insert(0u, v),
							PUTs::PutRight(v) => list.push(v),			
							PUTs::Terminate   => return,			
						};
					}
				)
			} else {
				let m = prx.recv();
				match m {
					PUTs::PutLeft(v)  => list.insert(0u, v),
					PUTs::PutRight(v) => list.push(v),			
					PUTs::Terminate   => return,			
				};
			}
		 }
		});

	let (rtx,rrx) = channel();
	gtx.send(GETs::GetLeft(rtx.clone()));
	gtx.send(GETs::GetRight(rtx.clone()));
	ptx.send(PUTs::PutLeft(10u));
	ptx.send(PUTs::PutRight(11u));

	let v = rrx.recv();
	assert!(v == Some(10u));
	let v = rrx.recv();
	assert!(v == Some(11u));

	ptx.send(PUTs::Terminate);
}

