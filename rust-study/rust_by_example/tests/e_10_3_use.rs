// 将 `deeply::nested::function` 路径绑定到 `other_function`。
use deeply::nested::function as other_function;
fn function (){
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function(){
            println!("called `deeply::nested::function()`")
        }
    }
}

#[test]
fn test_01() {
    // 更容易访问 `deeply::nested::function`
    other_function();

    println!("Enter block");
    {
        // 这和 `use deeply::nested::function as function` 等价。
        // 此 `function()` 将遮蔽外部的同名函数。
        use deeply::nested::function;
        function();

        println!("leave block")
    }

    function();

}