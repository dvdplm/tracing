#[cfg(not(feature = "parking_lot"))]
macro_rules! try_lock {
    ($lock:expr) => {
        try_lock!($lock, else return)
    };
    ($lock:expr, else $els:expr) => {
        if let Ok(l) = $lock {
            l
        } else if std::thread::panicking() {
            $els
        } else {
            panic!("lock poisoned")
        }
    };
}
#[cfg(feature = "parking_lot")]
macro_rules! try_lock {
    ($lock:expr) => {
        $lock
    };
    ($lock:expr, else $els:expr) => {
        $lock
    };
}

macro_rules! cfg_feature {
    ($name:literal, { $($item:item)* }) => {
        $(
            #[cfg(feature = $name)]
            #[cfg_attr(docsrs, doc(cfg(feature = $name)))]
            $item
        )*
    }
}
