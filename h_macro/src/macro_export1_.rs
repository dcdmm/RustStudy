#[macro_export]
macro_rules! use0_ {
    ($e0:expr, $e1:expr) => {
        $e0 + $e1
    };
}

#[macro_export]
macro_rules! use1_ {
    ($e0:expr, $e1:expr) => {
        $e0 - $e1
    };
}

#[macro_export]
macro_rules! use2_ {
    ($e0:expr, $e1:expr) => {
        $e0 * $e1
    };
}

#[macro_export]
macro_rules! use3_ {
    ($e0:expr, $e1:expr) => {
        $e0 / $e1
    };
}
