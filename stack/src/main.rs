use stack::stack;

fn main() {
    let mut s = stack::stack::Stack::new();
    s.push(1);
    s.push(2);
    s.push(4);

    // 使用 unwrap 取出 option 的 some() 值 data ,  为 none 抛错
    // println!("pop {:?}, size {}",s.peek().unwrap(), s.size());

    println!("top {:?}", s.peek().unwrap());
    println!(" size {}", s.size());

    println!("pop {:?}, size {}", s.pop().unwrap(), s.size());
    println!("is_empty:{}, stack:{:?}", s.is_empty(), s);

    let sa = "(2+3){func}[abc]";
    let sb = "(2+3)*(3-1";
    let res1 = par_checker(sa);
    let res2 = par_checker(sb);
    println!("sa balanced:{res1}, sb balanced:{res2}");
}

/* ==================括号算法=============================================== */

pub fn par_checker(par: &str) -> bool {
    // 解析字符串
    let mut chars: Vec<char> = Vec::new();

    for c in par.chars() {
        chars.push(c);
    }

    let mut balance = true;
    let mut stack = stack::stack::Stack::new();

    let mut index = 0;
    while index < chars.len() && balance {
        let c = chars[index];

        // 3种情况：左括号、右括号、非括号
        if '(' == c || '{' == c || '[' == c {
            stack.push(c);
        } else if ')' == c || ']' == c || '}' == c {
            if stack.is_empty() {
                balance = false
            } else {
                let cc = stack.pop().unwrap();
                balance = par_match(cc, c)
            }
        }
        index += 1;
    }

    balance && stack.is_empty()
}

fn par_match(first: char, second: char) -> bool {
    "({[".find(first) == ")}]".find(second)
}

/* ================================================================= */
fn base_converter(mut dec_num: u32, base: u32) -> String {
    // digits 对应各种余数的字符形式 ， 尤其是 10 - 15
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let mut rem_stack = stack::stack::Stack::new();
    // 余数入栈
    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }
    // 余数出栈并取对应字符来拼接成字符串
    let mut base_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap() as usize;
        base_str += &digits[rem].to_string();
    }
    base_str
}