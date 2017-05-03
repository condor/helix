use sys::{VALUE, Qnil};
use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl<T> UncheckedValue<Option<T>> for VALUE where VALUE: UncheckedValue<T> {
    fn to_checked(self) -> CheckResult<Option<T>> {
        if unsafe { self == Qnil } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            match UncheckedValue::<T>::to_checked(self) {
                Ok(_) => Ok(unsafe { CheckedValue::new(self) }),
                Err(e) => Err(e)
            }
        }
    }
}

impl<T> ToRust<Option<T>> for CheckedValue<Option<T>> where CheckedValue<T>: ToRust<T> {
    fn to_rust(self) -> Option<T> {
        if unsafe { self.inner == Qnil } {
            None
        } else {
            let checked: CheckedValue<T> = unsafe { CheckedValue::new(self.inner) };
            Some(checked.to_rust())
        }
    }
}

impl<T> ToRuby for Option<T> where T: ToRuby {
    fn to_ruby(&self) -> VALUE {
        match *self {
            Some(ref value) => value.to_ruby(),
            None => unsafe { Qnil }
        }
    }
}
