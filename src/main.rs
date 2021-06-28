fn muestra_tercer_elemento() {
    let mut v = vec![100, 32, 57];
    v.push(50);
    for i in &mut v {
	*i += 50;
        println!("{}", i);
    }
}

fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    muestra_tercer_elemento();
}
