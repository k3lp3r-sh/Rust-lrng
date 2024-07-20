// fn maxi32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//
//     largest
// }
//
// fn maxChar(list: &[char]) -> &char {
//     let mut largest = &list[0];
//
//     for number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//
//     largest
// }
// fn main() {
//     let num_list = vec![1, 2, 3, 4, 5, 6, 7];
//
//     let largest = max(&num_list);
//
//     println!("the largest number is: {largest}")
//
//     let char_list = vec!['a', 'v', 'd'];
// }

// Alternative to above ^^

fn max<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let num_list = vec![1, 2, 3, 4, 5, 6, 7];

    let largest = max(&num_list);

    println!("the largest number is: {largest}");

    let char_list = vec!['a', 'v', 'd'];

    let largest = max(&char_list);

    println!("the largest char is: {largest}");
}
