// //                      Q1

// fn test<T:FnOnce()->u8>(y:T)
// {
//     println!("{}",y());
// }
// fn main()
// {
//     let num=9;
//     let c=|| num;
//     test(c);
// }


// //                      Q2
// fn calling<T:FnMut()>(mut y:T)
// {
//     y();
// }
// fn main()
// {
//     let mut string=String::from("Hello");
//     let closre=|| {string.push_str(" World")};
//     calling(closre);
//     println!("{:?}",string);
// }



// //                          Q3

// use std::thread;
// use std::time::Duration;

// fn main() {

//     let threads1 = thread::spawn(|| {
//         for i in (0..11).rev()
//         {
//             println!("Class starts in {} second",i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//         println!("WELCOME TO BHARIA AUDOTORIUM");
//         println!("Hello all Students");
//         println!("the class is start within 10 second please take your seats");
//         thread::sleep(Duration::from_millis(5));
//         threads1.join().unwrap();
//         println!("Class Starts Now")

    
// }