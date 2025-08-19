fn main()
{
    stack_fn();
    heap_fn();
    updated_string();
}
fn stack_fn()
{
    
    let a = 30;
    let b = 32;
    let c = a+b;
    println!("Stack function: The sum of {} and {} is {}", a,b,c);
}
 fn heap_fn()
 {
    let s1 =String::from("hello");
    let s2 = String::from("world");
    let s3 = format!("{} {}",s1,s2);
    println!("Heap function : Combined string is {}",s3);
 }
 fn updated_string()
{
    let mut s = String::from("intail string");
    println!("before upadtes {}",s);
    println!("Capacity :{}, Length :{}, pointer: {:p }", s.capacity(),s.len(),s.as_ptr());
    
    // Apend some text to the string
    for _ in 0..100{
    s.push_str("and some additiona text");
     println!("Capacity :{}, Length :{}, pointer: {:p }", s.capacity(),s.len(),s.as_ptr());
    }
}  


/* 2. Tere output ka breakdown

Output ka ek-ek jump dekh:

First line:
Capacity :13, Length :13, pointer: 0x19a09ab7d50


Shuru me "intail string" dala → 13 chars.

Rust ne utna hi reserve kiya (cap = len = 13).

Pointer → heap ka address.

Jab push_str hua:
Capacity :36, Length :36, pointer: 0x19a09ab6020


Tu ne aur chars daale (total 36 chars ho gaye).

13 capacity chhoti padh gayi → Rust ne naya bada buffer allocate kiya (36).

Purane + naye chars copy hue → pointer badal gaya.

Next jump:
Capacity :72, Length :59, pointer: 0x19a09ab1330


Length abhi 59, lekin capacity Rust ne double kar di (36 → 72).

Reason: Rust hamesha thoda extra reserve karta future ke liye.
Matlab tu aur push kare toh baar-baar allocate na karna pade.

Phir:
Capacity :144, Length :82, pointer: 0x19a09ab0810


Length 82 ho gayi, lekin cap = 72 chhoti padh gayi.

Rust ne double karke 144 kar diya.

Naya block allocate hua, pointer badal gaya.

Ab dekho ye part:
Capacity :144, Length :105, pointer: 0x19a09ab0810
Capacity :144, Length :128, pointer: 0x19a09ab0810


Length badh rahi hai, but abhi 144 capacity me fit ho raha hai.

Isliye pointer same hai, koi reallocation nahi hua.

Jab 128 cross kiya:
Capacity :288, Length :151, pointer: 0x19a09ab9960


Ab 144 capacity kam padh gayi, toh Rust ne double karke 288 kar diya.

Naya buffer, naya pointer.

Ab dekh:
Capacity :288, Length :266, pointer: 0x19a09ab9960


Length abhi bhi 288 ke andar hai → no reallocation.

Pointer wahi ka wahi hai.

Jab 288 cross kiya:
Capacity :576, Length :289, pointer: 0x19a09ab9960


Rust phir se double kar diya (288 → 576).

Lekin interestingly, pointer same hi raha — Matlab allocator ne wahi block ko reallocate/extend kar diya instead of naya jagah dena (ye system allocator pe depend karta hai).

Fir same pattern:

576 → 1152 → 2304 → 4608 ... har bar capacity double hota hai.

Beech me jab tak len < cap hai, pointer same rehta hai.

Jaise hi len > cap hota hai, Rust dubara allocation karega.

🧠 3. Simple analogy (yaad rakhne layak 💯)

Soch ek dabba (heap memory) hai jisme tu apples (characters) daal raha hai:

len = dabbe me kitne apples daal diye.

cap = dabbe ki capacity kitni hai.

pointer = dabba kahan rakha hai (address).

Har bar jab dabba bhar gaya:

Rust naya bada dabba lata hai (aam taur pe double size).

Purane apples nikal ke naye dabbe me daal deta hai.

Kabhi-kabhi agar jagah mil gayi toh wahi dabba bada kar deta hai → pointer same rehta hai.

🧠 4. Tere output ka hidden pattern

Capacity mostly double hota hai: 13 → 36 → 72 → 144 → 288 → 576 → 1152 → 2304 → 4608...

Length ek-ek step me badh rahi hai, aur jab tak cap ke andar hai tab tak pointer same hai.

Jaise hi cap cross karta hai, Rust double karke naya allocate karta hai. */