use tch::{kind, Tensor};

fn grad_example() {
    let mut x = Tensor::from(2.0f32)
        .to_device(tch::Device::Mps)
        .set_requires_grad(true);
    let y = &x * &x + &x + 36;
    println!("y {}", y.double_value(&[]));

    x.zero_grad();
    y.backward();

    let dy_over_dx = x.grad();
    println!("dy/dx {}", dy_over_dx.double_value(&[]))
}

fn main() {
    let t = Tensor::from_slice(&[3, 1, 4, 1, 5]);
    t.print(); // works on CPU tensors

    println!("t(cpu) {:?}", &t);
    println!("t device: {:?}", &t.device());
    let t = Tensor::randn([5, 4], kind::FLOAT_CPU).to_device(tch::Device::Mps);
    t.print();
    println!("t(mps) {:?}", &t);
    println!("t device: {:?}", &t.device());

    grad_example();
}
