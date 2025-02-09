fn main() {

    println!("|  h   |       x       | Approx. f'(x) |  Known f'(x)  |  Abs. Error   |");
    println!("|:----:|--------------:|--------------:|--------------:|--------------:|");

    let x: f64 = 1.0;
    let f_x = x.sin();
    let f_prime_x = x.cos();

    for i in 1..31 {
        let h =  2.0_f64.powi(-i); 
        let approx_f_prime_x = ((f_x - (x + h).sin()) / h).abs();
        let abs_error = (f_prime_x - approx_f_prime_x).abs();
        println!("|2^-{:02} |    {:.8} |    {:.8} |    {:.8} |    {:.8} |", i, x, approx_f_prime_x, f_prime_x, abs_error);
    }

}
