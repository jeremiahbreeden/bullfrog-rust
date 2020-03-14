#[cfg(test)]
mod bounded_vectors {

   use bullfrog::bounded_vectors::*;

   const CAPACITY : usize = 5;

   #[test]
   fn initialization(){

      let test_length = CAPACITY - 1 as usize;
      let test_value = 42;

      let _v : Vector<u32,CAPACITY> 
         = Vector::new(||{ return test_value; },test_length);
      assert!(_v.length() == test_length);
      for index in 0 .. test_length {
         assert!(_v.element_at(index) == Some(test_value));
      }

      let _v : Vector<u32,CAPACITY> = Vector::new_empty(||{ return test_value});
      assert!(_v.length() == 0);

      let _v : Vector<u32,CAPACITY> = Vector::new_full(||{ return test_value});
      assert!(_v.length() == CAPACITY);
      for index in 0 .. CAPACITY {
         assert!(_v.element_at(index) == Some(test_value));
      }

      let _v : Vector<u32,CAPACITY> = Vector::default();
      assert!(_v.length() == 0);

      let test_array = [1,2,3,4,5];
      let _v : Vector<u32,CAPACITY> = test_array.into();
      assert!(_v.length() == CAPACITY);
      for index in 0 .. CAPACITY {
         assert!(_v.element_at(index) == Some(test_array[index]));
      }

      let start_index = 3;
      let _v : Vector<u32,CAPACITY> = Vector::from(&test_array[start_index..]);
      assert!(_v.length() == (CAPACITY - start_index));
      for index in start_index .. CAPACITY {
         assert!(_v.element_at(index-start_index) == Some(test_array[index]));
      }

      let _v : Vector<u32,CAPACITY> = test_value.into();
      assert!(_v.length() == CAPACITY);
      for index in 0 .. CAPACITY {
         assert!(_v.element_at(index) == Some(test_value));
      }

   }

   #[test]
   #[should_panic]
   fn new_panic(){

      let test_length = CAPACITY + 1 as usize;

      let _v : Vector<u32,5> = Vector::new(||{ return 42},test_length);
   }

}