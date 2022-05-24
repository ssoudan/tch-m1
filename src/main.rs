use tch::{kind, Tensor};

fn grad_example() {
    let mut x = Tensor::from(2.0).set_requires_grad(true);
    let y = &x * &x + &x + 36;
    println!("{}", y.double_value(&[]));
    x.zero_grad();
    y.backward();
    let dy_over_dx = x.grad();
    println!("{}", dy_over_dx.double_value(&[]));
}

fn main() {
    tch::maybe_init_cuda();
    let t = Tensor::of_slice(&[3, 1, 4, 1, 5]);
    // t.print(); // For some reason t.print() segfaults
    println!("{:?}", &t);
    let t = Tensor::randn(&[5, 4], kind::FLOAT_CPU);
    println!("{:?}", &t);
    grad_example();
    println!("Cuda available: {}", tch::Cuda::is_available());
    println!("Cudnn available: {}", tch::Cuda::cudnn_is_available());
}