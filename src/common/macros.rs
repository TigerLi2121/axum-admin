#[macro_export]
macro_rules! date_su {
    ($struct_ref:expr) => {
        let now = Some(Local::now().naive_local());
        $struct_ref.updated_at = now;
        if $struct_ref.id.is_none() {
            $struct_ref.created_at = now;
        }
    };
}

#[macro_export]
macro_rules! date_s {
    ($struct_ref:expr) => {
        $struct_ref.created_at = Some(Local::now().naive_local());
    };
}