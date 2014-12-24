use std::thread::Thread;
use std::comm::{channel, Sender, Receiver};

fn plus_one(sender: &Sender<int>, receiver: &Receiver<int>) {
    let mut value: int;
    loop {
        value = receiver.recv();

        if value == -1 {
           // FIXME: probably just want an exit here, so look it up
           panic!("I'm done")
        }

        sender.send(value + 1);
        if value == 0 { break; }
    }
}

fn main () {
    let (from_parent_sender, from_parent_receiver) = channel();
    let (from_child_sender, from_child_receiver) = channel();

    let guard = Thread::spawn(move || {
        plus_one(&from_child_sender, &from_parent_receiver)
    });

    from_parent_sender.send(22);
    from_parent_sender.send(23);
    from_parent_sender.send(24);
    from_parent_sender.send(25);
    from_parent_sender.send(-1); // stop

    for _ in range(0u, 4) {
        let answer = from_child_receiver.recv();
        println!("{}", answer.to_string());
    }

    let result = guard.join();

    if (result != 2) {
        // FIXME: compiler says I must pay attention to this value 
    }

}
