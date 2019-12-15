//                      Q1

// use std::io;
// fn main()
// {
//     let i=true;
    
//     while i==true
//     {
//         println!("Enter your values:");
//         let mut input=String::new();
//         io::stdin().read_line(&mut input).expect("Failed to read line");  
//         // let val: String = input.trim().parse().unwrap();
//         let mut v = Vec::new();
//         for i in input.split_whitespace()
//         {
//             v.push(i)
//         }
//         let int1:f32=v[0].parse().unwrap();
//         let int2:f32=v[2].parse().unwrap();
//         if int1==0.0
//         {
//             break;
//         }

//         match v[1]
//         {
//             "+"=>
//             {
//                 println!("{}",add(int1,int2));
//                 continue;
//             },
//             "-"=>
//             {
//                 println!("{}",sub(int1,int2));
//                 continue;
//             },
//             "*"=>
//             {
//                 println!("{}",mul(int1,int2));
//                 continue;
//             },
//             "/"=>
//             {
//                 println!("{}",div(int1,int2));
//                 continue;
//             },
//             "^"=>
//             {
//                 println!("{}",pow(int1,int2));
//                 continue;
//             },
//             _=>
//             {
//                 println!("please input correct operation");
//                 continue;
//             },
//         }
//     }
//     println!("Bye");
// }

// fn add(int1:f32,int2:f32)->f32
// {
//     let sum =int1+int2;
//     sum
// }
// fn sub(int1:f32,int2:f32)->f32
// {
//     let diff =int1-int2;
//     diff
// }
// fn mul(int1:f32,int2:f32)->f32
// {
//     let mul =int1*int2;
//     mul
// }
// fn div(int1:f32,int2:f32)->f32
// {
//     let div =int1/int2;
//     div
// }
// fn pow(int1:f32,int2:f32)->f32
// {
//     let iter=int2+1.0;
//     let mut i=1.0;
//     let mut powers=1.0;
//     while i!=iter
//     {
//        powers =powers*int1;
//        i+=1.0;
//        continue;
//        if i==iter
//        {
//            break;
//        }
//     }
//    powers
// }










//                  Q2
// fn main() {
//     let x =||{println!("hello world")};
//     x();
// }



// fn main() {
//     let x =|num:u32|->u32{ 
//     let y=num+1;
//     y
//     };
//     println!("The function returns: {}",x(3));
// }





// fn main() {
//     let  mut c = 1;
//     let x =|mut a:u32|->u32
//     {
//         a=a+c;
//         a
//     };
//     c=x(c);
//     println!("The new value of c is: {}",c);
// }




// fn main() 
// {
//     let x=||{};
//     funct(x);

// }

// fn funct(a:impl Fn())
// {
//     a();
// }




// fn main()
// {
//     let x=|mut a:u32|->u32
//     {
//         a=a+1;
//         a
//     };
//     funct(x);
// }
// fn funct(a:impl Fn(u32)->u32)
// fn funct(a:T)
//     where T:Fn(u32)->u32 
// {
//     println!("{}",a(10));
// }






//                      Q4

// struct student1
// {
//     nm:String,
//     Educations:bool,
//     bilinguals:bool,
// }
// struct student2
// {
//     nm:String,
//     Educations:bool,
//     bilinguals:bool,
// }
// pub trait quality
// {
//     fn edu(&self)->bool;
//     fn bil(&self)->bool;
// }

// impl quality for student1
// {
//     fn edu(&self)->bool
//     {
//         if self.Educations==true && self.bilinguals==true
//         {
//             true
//         }
//         else
//         {
//             false
//         }
//     }
//     fn bil(&self)->bool
//     {
//         if self.Educations==true && self.bilinguals==true
//         {
//             true
//         }
//         else
//         {
//             false
//         }
//     }
// }
// impl quality for student2
// {
//     fn edu(&self)->bool
//     {
//         if self.Educations==true && self.bilinguals==true
//         {
//             true
//         }
//         else
//         {
//             false
//         }
//     }
//     fn bil(&self)->bool
//     {
//         if self.Educations==true && self.bilinguals==true
//         {
//             true
//         }
//         else
//         {
//             false
//         }
//     }
// }
// fn adopt(stud1:impl quality,stud2:impl quality )
// {
//     if stud1.edu()==true && stud1.bil()==true && stud2.edu()==true && stud2.bil()==true
//     {
//         println!("yes I Mr. Asim happily brings them to my new home sweet home");
//     }
//     else
//     {
//         println!("Sorry I, Mr Asim cannot bring them to home:");
//     }
// }
// fn main()
// {
//     let s1=student1
//     {
//         nm:String::from("Hassan"),
//         Educations:true,
//         bilinguals:true,
//     };
//     let s2=student2
//     {
//         nm:String::from("Hassan"),
//         Educations:true,
//         bilinguals:true,
//     };
//     adopt(s1,s2);
// }





                            // Q5

// Closures don’t require you to annotate the types of the parameters or the return value like 
// fn functions do

// Closures are usually short and relevant only within a narrow context rather than in any arbitrary
// scenario

// Making programmers annotate the types in these small, anonymous functions would be annoying and largely
// redundant with the information the compiler already has available

// closures can capture values from the scope in which they’re defined.
