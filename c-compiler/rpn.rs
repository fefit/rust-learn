fn main(){
  let v = rev_polish_notation("a + b * c + (d * e + f)*g");
  println!("v is {:?}", v);
}

fn get_priority(c: &char) -> u8{
  // 获取优先级
  match c{
      '+'|'-' => 1,
      '*'|'/' => 2,
      '(' => 0, // 遇到左括号
      _ => u8::MAX
  }
}

fn rev_polish_notation(expr: &str) -> Vec<char> {
  let count = expr.len();
  let mut stack = Vec::with_capacity(count);
  let mut output = Vec::with_capacity(count);
  for c in expr.chars(){
      match c{
        ' ' => continue, //遇到空字符，跳过
        ')' => {
            // 遇到右括号、从栈中找到左括号为止
          while let Some(op) = stack.pop(){
              if op == '('{
                  // 遇到左括号停止弹出
                  break;
              }else{
                  output.push(op);
              }
          }
        },
        '(' => {
          // 左括号直接入栈，只有右括号出现时它才能出栈
          stack.push(c);  
        },
        '+'|'-'|'*'|'/' => {
          // 针对+-*/运算，得从栈尾往前一直找到优先级比其低的运算符为止
          let cur_prio = get_priority(&c);          
          while let Some(op) = stack.last(){
              let prev_prio = get_priority(op);
              if prev_prio >= cur_prio{
                  output.push(stack.pop().unwrap());
              }else{
                  break;
              }
          }
          // 将当前符号压入栈中
          stack.push(c);
        },
        _ => {
          // 操作数，直接输出
          output.push(c);
        }
      };
  }
  // 将栈中剩余元素全部加入到输出队列中
  while let Some(op) = stack.pop(){
      output.push(op);
  }
  output
}