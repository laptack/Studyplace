use std::io;

fn main() {

    println!("Please input the number array:");

    //用户输入待排序数列，以‘ ’隔开
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("read_line error!");
        //.unwrap();

    //将输入的字符串转换为数组
    let mut nums:Vec<i32> = input.split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();

    //调用冒泡排序函数
    bubble_sort(&mut nums);

    //打印结果
    println!("The final result:{:?}",nums);
}


//基础要求的冒泡排序
fn bubble_sort(nums:&mut Vec<i32>) {
    let length = nums.len();
    for j in 1..length {
        for i in 1..length{
            if nums[i - 1] > nums[i] {
                nums.swap(i - 1,i);
            }
        }
    }
}

