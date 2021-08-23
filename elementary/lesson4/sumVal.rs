//求和
pub fn sumAll(vec: &Vec<u32>) -> Option<u32> {

    let mut sum: u32 =0;
    let mut v:u32 =0;
    for num in vec.iter() {
        let s=sum.checked_add(*num);
        match s {
            Some(s) => { sum=s}
            None => { return None}
        }
    }

    Some(sum)

}

//入口
pub fn run() {
    let vec = vec![1, 2, 3, 4, 5, u32::max_value()];
    let s = sumAll(&vec);

    println!("sum is {}", s.unwrap_or(0))
}