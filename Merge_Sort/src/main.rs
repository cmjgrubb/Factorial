use std::str::FromStr;

fn main() {
    println!("Please enter a series to be sorted separated by a space:");
    let mut user_input = String::new();

    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let mut array_to_sort = input_to_array(&user_input);

    println!("Your sorted series is:");
    println!("{:?}", merge_sort(&mut array_to_sort.clone()));
}

fn input_to_array<T>(input: &str) -> Vec<T>
where
    T: FromStr,
{
    input
        .split_whitespace()
        .map(|s| s.parse::<T>())
        .collect::<Result<Vec<T>, T::Err>>()
        .unwrap()
}

fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(arr, &left, &right);
}
