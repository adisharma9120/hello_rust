/* fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let iter = v1.iter();

    for n in iter {
        println!("{}", n * 2);
    }
}




v1 = [1,2,3] banaya
iter = [&1, &2, &3] (iterator banaya jo references deta hai)
map(|x| x + 1) = har value me +1 add kar diya → [2,3,4]
for loop chalaya aur values print kar di
 */
 fn main()
 {
    let v1: Vec<i32> = vec![1,2,3,4,5,6,7];
    let iter = v1.iter();
        let iter2 = iter.filter(|x| *x % 2==0);

    for x in iter2{
           
        println!("{}", x);
    }
 }