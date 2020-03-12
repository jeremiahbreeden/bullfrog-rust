#[cfg(test)]
mod safe_array_init {

   use bullfrog::safe_array_init::*;

   #[test]
   fn initialization(){
      let _v : [u32; 10] = from_closure(||{return 10});
      let _v = from_default::<usize,5>();
   }
}