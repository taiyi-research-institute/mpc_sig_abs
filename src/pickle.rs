use erreur::*;
use serde::{de::DeserializeOwned, Serialize};

pub trait Pickle
where
    Self: Serialize + DeserializeOwned + Sized,
{
    fn pickle(&self) -> Resultat<Vec<u8>> {
        let buf = serde_pickle::to_vec(self, Default::default()).catch_()?;
        Ok(buf)
    }
    fn unpickle(data: &[u8]) -> Resultat<Self> {
        let obj = serde_pickle::from_slice(data, Default::default()).catch_()?;
        Ok(obj)
    }
}

impl<T> Pickle for T where T: Serialize + DeserializeOwned + Sized {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Test {
        a: i32,
        b: String,
    }

    #[test]
    fn test_pickle() {
        let obj = Test {
            a: 42,
            b: "hello".to_string(),
        };
        let buf = obj.pickle().unwrap();
        let obj2 = Test::unpickle(&buf).unwrap();
        assert_eq!(obj, obj2);
    }
}
