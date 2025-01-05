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
