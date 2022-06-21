trait ShapeMath {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Rectangle {
    a: f64,
    b: f64,
}

struct Circle {
    r: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl ShapeMath for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r.powi(2)
    }
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.r
    }
}

impl ShapeMath for Rectangle {
    fn area(&self) -> f64 {
        self.a * self.b
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.a + self.b)
    }
}

impl ShapeMath for Triangle {
    fn area(&self) -> f64 {
        let s = (self.a + self.b + self.c) / 2.0;
        s * (s - self.a) * (s - self.b) * (s - self.c)
    }
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}
// 1. 有一个指向某个函数的指针，如果将其解引用成一个列表，然后往列表中插入一个元素，请问会发生什么？（对比不同语言，看看这种操作是否允许，如果允许会发生什么）
// Java 类型转换异常，操作不允许
// 2. 要构造一个数据结构 Shape，可以是 Rectangle、 Circle 或是 Triangle，这三种结构见如下代码。请问 Shape 类型该用什么数据结构实现？怎么实现？
// 3. 对于上面的三种结构，如果我们要定义一个接口，可以计算周长和面积，怎么计算？

fn main() {
    let c = Circle { r: 1.0 };
    let r = Rectangle { a: 1.0, b: 2.0 };
    let t = Triangle {
        a: 1.0,
        b: 2.0,
        c: 3.0,
    };
    println!("{}", c.area());
    println!("{}", c.perimeter());
    println!("{}", r.area());
    println!("{}", r.perimeter());
    println!("{}", t.area());
    println!("{}", t.perimeter());
}
