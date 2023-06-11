
        use std::vec::Vec;

        use crate::write_into;

        // https://tools.ietf.org/html/rfc4648#section-10
        #[test]
        fn test_rfc4648_test_vectors_write_into() {
            // ```
            // BASE16("") = ""
            // ```
            let mut write = Vec::new();
            write_into("", &mut write).unwrap();
            assert_eq!(write, b"".to_vec());

            // ```
            // BASE16("f") = "66"
            // ```
            let mut write = Vec::new();
            write_into("f", &mut write).unwrap();
            assert_eq!(write, b"66".to_vec());

            // ```
            // BASE16("fo") = "666F"
            // ```
            let mut write = Vec::new();
            write_into("fo", &mut write).unwrap();
            assert_eq!(write, b"666f".to_vec());

            // ```
            // BASE16("foo") = "666F6F"
            // ```
            let mut write = Vec::new();
            write_into("foo", &mut write).unwrap();
            assert_eq!(write, b"666f6f".to_vec());

            // ```
            // BASE16("foob") = "666F6F62"
            // ```
            let mut write = Vec::new();
            write_into("foob", &mut write).unwrap();
            assert_eq!(write, b"666f6f62".to_vec());

            // ```
            // BASE16("fooba") = "666F6F6261"
            // ```
            let mut write = Vec::new();
            write_into("fooba", &mut write).unwrap();
            assert_eq!(write, b"666f6f6261".to_vec());

            // ```
            // BASE16("foobar") = "666F6F626172"
            // ```
            let mut write = Vec::new();
            write_into("foobar", &mut write).unwrap();
            assert_eq!(write, b"666f6f626172".to_vec());
        }

        #[test]
        fn test_write_into() {
            let data = b"Artichoke Ruby";
            let mut buf = Vec::new();
            write_into(data, &mut buf).unwrap();
            assert_eq!(buf, b"4172746963686f6b652052756279".to_vec());
        }
