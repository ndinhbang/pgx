use crate::{pg_sys, FromDatum, PgBox};

pub struct Internal<T>(pub PgBox<T>);

impl<T> FromDatum<Internal<T>> for Internal<T> {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<Internal<T>> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("Internal-type Datum flagged not null but its datum is zero")
        } else {
            Some(Internal::<T>(PgBox::<T>::from_pg(datum as *mut T)))
        }
    }
}