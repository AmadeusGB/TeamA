use support::{StorageMap,Parameter};
use runtime_primitives::traits::Member;
use parity_codec::{Encode, Decode};
use system::ensure_signed;
use rstd::result;

#[cfg_attr(feature = "std",derive(Debug, PartialEq, Eq))]
#[derive(Encode, Decode)]
pub struct LinkedItem<Item> {
    pub prev: Option<Item>,
    pub next: Option<Item>,
}

pub struct LinkedList<Storage, Key, Item>(rstd::marker::PhantomData<(StorageMap))

impl<Storage, Key, Value> LinkedList<Storage, Key, Value> where
    Value: Parameter + Member + Copy,
    Key: Parameter,
    Storage: StorageMap<(Key, Option<Value>), LinkedItem<Value>, Query = Option<Item>>
{
    fn read_head(key: &Key) -> LinkedItem<Value> {
        Self::read(key, None)
    }    
    
    fn write_head(account: &Key, item: LinkedItem<Value>) {
        Self::write(account, None, item);
    }
}
