

fn main() {
    let mut oak: [i32; 7]=[1,13,15,7,11,20,33];
    let mut bay: [i32; 7]=[99,1,13,15,7,20,33];
    bubble_sort(&mut oak);
    bubble_sort(&mut bay);
    println!("the sort result is {:#?}",oak);
    println!("the sort result is {:#?}",bay);
}
fn bubble_sort(stephen: &mut [i32]) {
    let mut swapped = true;
    while swapped {
    swapped = false;
    for i in 1..stephen.len() {
    if stephen[i - 1] > stephen[i] {
    stephen.swap(i - 1, i);
    swapped = true;
    }
    }
    }
    }
    