#[macro_export]
macro_rules! dowhile {
    ($($label:lifetime:)? $body:block $(,)? $cond:expr) => ({
        $($label:)? loop {
            $body
            if !$cond {
                break $($label)?;
            }
        }
    });
    ($($label:lifetime:)? $body:block let $pattern:pat = $value:expr $(=> $cond:expr)?) => ({
        $($label:)? loop {
            $body
            if let $pattern = $value {
                $(if !$cond { 
                    break; 
                })?   
            } else {
                break $($label)?;
            }
        }
    });
    ($($label:lifetime:)? $body:block let $pattern:pat = $value:expr $(=> $cond:expr)?) => ({
        $($label:)? loop {
            $body
            if let $pattern = $value {
                $(if !$cond { 
                    break; 
                })?   
            } else {
                break $($label)?;
            }
        }
    });
    // Alternative syntax
    ($cond:expr, $($label:lifetime:)? $body:block) => ({
        $($label:)? loop {
            $body
            if !$cond {
                break $($label)?;
            }
        }
    });
    (let $pattern:pat = $value:expr$ (=> $cond:expr)?, $($label:lifetime:)? $body:block) => ({
        $($label:)? loop {
            $body
            if let $pattern = $value {
                $(if !$cond {
                    break;
                })?   
            } else {
                break $($label)?;
            }
        }
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn nested_with_label() {
        let mut v = vec![];
        let mut x = 10;
        dowhile!(x < 6, 'main: {
            v.push(('x', x));
            if x > 6 {
                x = 0;
                continue;
            }
            let mut y = x;
            dowhile!({
                v.push(('y', y));
                if y > 3 {
                    break 'main;
                }
                y += 1;
            } y < x);
            x += 1;
        });
        assert_eq!(v, vec![
            ('x', 10), ('x', 0), ('y', 0), ('x', 1), ('y', 1), ('x', 2), 
            ('y', 2), ('x', 3), ('y', 3), ('x', 4), ('y', 4)
        ]);
    }

    #[test]
    fn while_true() {
        let mut x = 0;
        dowhile!({
            if x > 3 {
                break;
            }
            x += 1;
        }, true);
        assert_eq!(x, 4);
    }

    #[test]
    fn while_let() {
        let mut x = Some(10);
        let mut v = vec![];
        dowhile!({
            v.push(x);
            x = Some(x.unwrap_or(0) - 1);
        } let Some(8..) = x);
        let vv = vec![Some(10), Some(9), Some(8)];
        assert_eq!(v, vv);
    }
    
    #[test]
    fn while_let_val() {
        let mut x = Some(10);
        let mut v = vec![];
        dowhile!({
            v.push(x);
            x = Some(x.unwrap_or(0) - 1);
        } let Some(val) = x => val > 8);
        let vv = vec![Some(10), Some(9)];
        assert_eq!(v, vv);
    }
    
    #[test]
    fn while_let_alter() {
        let mut x = Some(10);
        let mut v = vec![];
        dowhile!(let Some(val) = x => val > 8, {
            v.push(x);
            x = Some(x.unwrap_or(0) - 1);
        });
        let vv = vec![Some(10), Some(9)];
        assert_eq!(v, vv);
    }
}
