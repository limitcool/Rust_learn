/* loop有个用途是尝试一个操作直到成功为止。若操作返回一个值则可能需要将其传递给代码的其余部分：
将该值放在break之后,它就会被loop表达式返回 */
fn main() {
    let mut counter = 0;

    let result = loop{
        counter +=1;
        if counter ==10{
            break counter * 2;
        }
    };
    println!("result={:?}",result);
    assert_eq!(result,20);
    println!("assert_eq={:?}",assert_eq!(result,20))
}