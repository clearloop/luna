use spacejam::prototype::vm::Vm;

fn main() {
    let vm = Vm::default();
    let code = r#"
     (define (foo a)
       (* a 4))
     "#;

    &vm.input("foo", code).unwrap();
    let res = vm.exec("foo", vec![123.into()]);
    println!("{:?}", res);
}
