#[macro_export]
macro_rules! dowhile {
    ($body:block $cond:expr) => ({
        let mut r#dowhile_flag = true;
        while r#dowhile_flag || ($cond) {
            r#dowhile_flag = false;
            $body
        }
    });
    ($label:lifetime: $body:block $cond:expr) => ({
        let mut r#dowhile_flag = true;
        $label: while r#dowhile_flag || ($cond) {
            r#dowhile_flag = false;
            $body
        }
    });
    ($cond:expr, $body:block) => ({
        let mut r#dowhile_flag = true;
        while r#dowhile_flag || ($cond) {
            r#dowhile_flag = false;
            $body
        }
    });
    ($cond:expr, $label:lifetime: $body:block) => ({
        let mut r#dowhile_flag = true;
        $label: while r#dowhile_flag || ($cond) {
            r#dowhile_flag = false;
            $body
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
}
