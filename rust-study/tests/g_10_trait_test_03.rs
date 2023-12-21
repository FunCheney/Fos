/// trait Object
/// 形式上，就是在 trait 名前加 dyn 关键字修饰，在这个例子里就是 dyn TraitA。
/// dyn TraitName 本身就是一种类型，它和 TraitName 这个 trait 相关，但是它们不同，dyn TraitName 是一个独立的类型。

struct Atype;
struct Btype;
struct Ctype;

trait TraitA {}

impl TraitA for Atype {}
impl TraitA for Btype {}
impl TraitA for Ctype {}

fn doit(i: u32) -> Box<dyn TraitA> {
    if i == 0 {
        let a = Atype;
        Box::new(a)
    } else if i == 1 {
        let b = Btype;
        Box::new(b)
    } else {
        let c = Ctype;
        Box::new(c)
    }
}
/// 这里我们引入了一个新的东西 Box。Box 的作用是可以保证获得里面值的所有权，必要的时候会进行内存的复制，
/// 比如把栈上的值复制到堆中去。一旦值到了堆中，就很容易掌握到它的所有权。
/// 具体到这个示例中，因为 a、b、c 都是函数中的局部变量，这里如果返回引用 &dyn TraitA 的话是万万不能的，因为违反了所有权规则。
/// 而 Box 就能满足这里的要求。后续我们在智能指针一讲中会继续讲解 Box。
