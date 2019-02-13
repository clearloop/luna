// macros
//@bytes_macro
#[macro_export]
macro_rules! bytes {
    ($name: ident) => {
        impl $name {
            pub fn to_bytes(&self) -> Vec<u8> {
                serialize(&self).unwrap()
            }

            pub fn from_bytes(data: Vec<u8>) -> Self {
                deserialize(&data[..]).unwrap()
            }
        }
    }
}

//@deref_macro
#[macro_export]
macro_rules! deref {
    ($name: ident, $target: ident) => {
        impl Deref for $name {
            type Target = $target;
    
            fn deref(&self) -> &Self::Target { &self.0 }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
    }
}

//@partition
#[macro_export]
macro_rules! partition {
    ($name: ident, $partition: ident) => {
        #[derive(Serialize, Deserialize, Debug, Default)]
        pub struct $partition(pub Vec<$name>);

        impl $partition {
            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn push(&mut self, tx: $name) {
                self.0.push(tx)
            }
        }

        bytes!($partition);
    }
}
