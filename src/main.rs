use core::f64;

fn differential<F: Fn(f64) -> f64>(funtion: F) -> impl Fn(f64) -> f64 {
    // 이 함수는 무려 인자로 들어온 함수의 도함수(클로저)를 뱉어냄
    //                       f(x + dx) - f(x - dx)
    //                lim    _____________________
    //              dx -> 0           2dx
    move |x: f64| {
        (funtion(x + 2.3283064365386962890625e-10 /* dx = 2^-32 */)
            - funtion(x - 2.32830643653869628906255e-10 /* dx = 2^-32 */))
            * 2147483648.0
    } /* 1/2dx = 2^31 / 2 */
}

// 이 함수는 무료로 적분해줍니다. -> 사다리꼴 적분
fn _integral<F>(funtion: F, to: f64, from: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let len: f64 = to - from;
    let mut result: f64 = 0.0;

    let dx: f64 = len * 9.5367431640625e-7;

    for k in 0..1048576 {
        result +=
            0.5 * dx * { funtion(from + dx * k as f64) + funtion(from + dx * (k + 1) as f64) };
    }
    result
}

// 얘도 같은 알고리즘으로 짰는데 Xn이랑 X(n+1)을 미리 계산된 값을 복사하는, 쪼오끔 효율적인 방법을 써서
// 요놈이 쪼오끔 더 빠름. 어썸하다.
fn integral<F>(funtion: F, to: f64, from: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    //사다리꼴 수치적분
    //sigma dx * (f(Xn) + f(X(n+1))) / 2

    if from == to {
        return 0.0;
    }

    let len: f64 = to - from;
    let mut result: f64 = 0.0;

    let dx: f64 = len * 9.5367431640625e-7;
    let mut x_n = from;
    let mut x_n1 = x_n + dx;
    let mut Fx: f64 = funtion(from);
    let mut Fx_1: f64;
    for _ in 0..1048576 {
        Fx_1 = funtion(x_n1);
        result += 0.5 * dx * { Fx + Fx_1 };
        // dx도 나중에 곱하곤 싶은데 그러면 오버플로우 날 것 같음ㅇㅇ
        x_n = x_n1;
        x_n1 = x_n + dx;
        //생각해보니 함수값도 계속 두번씩 계산할 필요 없잖아 -> 7.808s
        Fx = Fx_1;
    }
    result
}

fn ln_ocha() {
    // ln(1 + x) 와 그 근사치(라고 주장하는) xln2 + a 에서 (0 < x < 1) a가 어느정도 되야 평균적으로 오차가 제일 줄어느나?

    fn error1(x: f64) -> f64 {
        integral(
            |t: f64| ((t + 1.0).ln() - f64::ln(2.0) * t - x).powi(2),
            1.0,
            0.0,
        )
    }

    fn error2(x: f64) -> f64 {
        integral(
            |t: f64| ((t + 1.0).ln() - f64::ln(2.0) * t - x).abs(),
            1.0,
            0.0,
        )
    }

    let mut x = 0.00006103515625 * 500.0;

    let mut b1: f64 = 10.0;
    let mut b2: f64 = 10.0;
    while x < 0.044 {
        let e1 = error1(x);
        let e2 = error2(x);
        println!("{} : {} :: {}", x, e1, e2);
        x += 0.00006103515625;

        if e1 > b1 {
            println!("\n");
            if e2 > b2 {
                break;
            }
        }
        b1 = e1;
        b2 = e2;
    }

    let a: f64 = 0.0397207708399179641258481821872648521132502015403828811810200142;
    println!("answer {} : {} :: {}", a, error1(a), error2(x));
}
// 수치적으로 구한 a값            제곱해서 최소 오차값
// 0.039794921875       : 0.0003169858183111574     :: 0.015317646025614255
//                                                      abs:절댓값으로 최소 오차값
// 0.0439453125         : 0.0003348270721732604     :: 0.014948338140564114
// -> 왜 다른 것?????????????? 진짜 이해가 안된다. 내가 알고리즘을 잘 못짰나?

//                                      제곱해서 오차구하는걸로
// 해석적으로 구한 값 0.039720770839917964 : 0.00031698031993515646    :: 0.014946936581737465
// 절댓값으로하는건 도저히 내 지식과 지능으로 못 풀겠음 (사실 나눠지는 부분이 t(x) 인 함수로 나와서 머리 깨짐. 실제로 풀 수 있는지도 의문이다.)

fn which_is_faster_integral() {
    // 적분 알고리즘에서 어떤게 더 빠른지 측정하는 함수
    // 변수와 미리 계산된 값을 잘 활용하자.

    /*  내 컴 :     MacBook Pro
                13-inch, M2, 2022
                Chip    :   Apple M2
                Memory  :   8 GB (...)
    */
    let start = std::time::Instant::now();
    let mut x = 0.0;
    let f = |x: f64| x.ln();
    while x < 50.0 {
        let _ = _integral(f, 1.0, 1.0 + x);
        x += 0.125;
    }
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    println!("{:?}", duration); //11.829394334s

    let start = std::time::Instant::now();
    let mut x = 0.0;
    let f = |x: f64| x.ln();
    while x < 50.0 {
        let _ = integral(f, 1.0, 1.0 + x);
        x += 0.125;
    }
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    println!("{:?}", duration); //9.725161042s

    // 1 - 9.725161042/11.829394334 = 0.1778817437805772279870977205010976346407423769968306138977681323
    // 대충 17.8% 정도 차이 나네... 이렇게 계산하는게 맞나...?

    // 11.829394334/9.725161042 = 1.2163700202919477783183428336692421838457819133767718229215519863
    // 21% 빠르다고. 이렇게 계산하는게 맞나..?
}

fn simpsons_rule<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    if n % 2 != 0 {
        panic!("n must be even");
    }

    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);

    for i in 1..n {
        let x = a + i as f64 * h;
        if i % 2 == 0 {
            sum += 2.0 * f(x);
        } else {
            sum += 4.0 * f(x);
        }
    }

    sum * h / 3.0
}

fn d_i() {
    let mibun = differential(|x| x.ln());
    let mibun_dashi_integral = |x| simpsons_rule(&mibun, 1.0, x, usize::pow(2, 12));

    let mut x = 1.0;

    while x <= 20.0 {
        println!(
            "x : {}\n  ln x : {}\nsdln x : {} \n",
            x,
            f64::ln(x),
            mibun_dashi_integral(x)
        );
        x += 0.25;
    }
}

fn facto(mut n: usize) -> f64 {
    let mut ret: f64 = 1.0;
    loop {
        if n == 0 {
            break ret;
        }
        ret *= n as f64;
        n -= 1;
    }
}

fn exp(x: f64) -> f64 {
    let mut n = 1;
    let mut sum: f64 = 1.0;
    loop {
        sum += x * (1.0 / facto(n));
        n += 1;
        if n > 16 {
            break;
        }
    }
    return sum;
}

#[inline]
fn invfac(n: usize) -> f64 {
    1.0 / facto(n)
}

fn ex(x: f64, n: usize) -> f64 {
    if n > 16 {
        return invfac(n);
    }

    return invfac(n) + x * ex(x, n + 1);
}

fn exponantial() {
    let e = f64::exp(1.0);
    let e1 = exp(1.0);
    let e2 = ex(1.0, 0);

    println!("{}\n{}\n{}", e, e1, e2);

    let e3 = differential(f64::exp)(1.0);
    let e4 = differential(|x| ex(x, 0))(1.0);

    println!("{}\n{}", e3, e4);
}

fn main() {
    let e = integral(|x| 1.0 / x, f64::consts::E + 0.0000000000289764, 1.0);
    println!("{e}");
}
