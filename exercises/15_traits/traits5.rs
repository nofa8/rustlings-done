trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
// Alternative 1 - trait parameters:
// fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
// Alternative 2 - generic parameters with trait bounds:
// fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
// Alternative 3 - also generic parameters with trait bounds:
fn some_func<T>(item: T) -> bool
    where T: SomeTrait + OtherTrait 
{
    item.some_function() && item.other_function()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}
